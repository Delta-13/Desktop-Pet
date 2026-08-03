#![forbid(unsafe_code)]

mod sixel;

use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::io;
use std::io::Cursor;
use std::io::IsTerminal;
use std::io::Write;
use std::path::Component;
use std::path::Path;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use std::time::Instant;

use anyhow::Context;
use anyhow::Result;
use anyhow::bail;
use base64::Engine as _;
use base64::engine::general_purpose;
use clap::Parser;
use clap::ValueEnum;
use image::DynamicImage;
use image::ImageFormat;
use image::RgbaImage;
use serde::Deserialize;

const FRAME_WIDTH: u32 = 192;
const FRAME_HEIGHT: u32 = 208;
const FRAME_COLUMNS: u32 = 8;
const V1_ROWS: u32 = 9;
const V2_ROWS: u32 = 11;
const PET_IMAGE_ID: u32 = 0xC0DE;
const KITTY_CHUNK_SIZE: usize = 4096;
const ESC: &str = "\x1b";
const ST: &str = "\x1b\\";

#[derive(Debug, Parser)]
#[command(
    version,
    about = "Play a Codex-compatible pet in the current terminal."
)]
struct Cli {
    /// Directory containing pet.json, or the path to pet.json itself.
    pet: PathBuf,

    /// Animation name from the pet manifest or the Codex default animation set.
    #[arg(long, default_value = "idle")]
    state: String,

    /// Terminal image protocol. Auto detects a supported terminal.
    #[arg(long, value_enum, default_value_t = ProtocolSelection::Auto)]
    protocol: ProtocolSelection,

    /// Rendered sprite height in pixels.
    #[arg(long, default_value_t = 75, value_parser = clap::value_parser!(u16).range(1..=1_000))]
    height: u16,

    /// Stop after this many seconds instead of running until interrupted.
    #[arg(long)]
    duration: Option<f64>,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum ProtocolSelection {
    Auto,
    Kitty,
    Sixel,
}

#[derive(Clone, Copy, Debug)]
enum ImageProtocol {
    Kitty,
    KittyFile,
    Sixel,
}

#[derive(Debug, Deserialize)]
struct Manifest {
    #[serde(rename = "spritesheetPath")]
    spritesheet_path: Option<String>,
    frame: Option<FrameSpec>,
    #[serde(default)]
    animations: BTreeMap<String, AnimationSpec>,
}

#[derive(Clone, Copy, Debug, Deserialize)]
struct FrameSpec {
    width: u32,
    height: u32,
    columns: u32,
    rows: u32,
}

#[derive(Debug, Deserialize)]
struct AnimationSpec {
    #[serde(default)]
    frames: Vec<usize>,
    fps: Option<f64>,
    #[serde(rename = "loop")]
    loop_animation: Option<bool>,
}

#[derive(Clone, Debug)]
struct AnimationFrame {
    sprite_index: usize,
    duration: Duration,
}

#[derive(Clone, Debug)]
struct Animation {
    frames: Vec<AnimationFrame>,
    loop_start: Option<usize>,
}

#[derive(Debug)]
struct Pet {
    image: RgbaImage,
    frame: FrameSpec,
    animations: BTreeMap<String, Animation>,
}

impl Pet {
    fn load(path: &Path) -> Result<Self> {
        let manifest_path = if path.is_dir() {
            path.join("pet.json")
        } else {
            path.to_path_buf()
        };
        let pet_dir = manifest_path
            .parent()
            .context("pet manifest has no parent directory")?
            .canonicalize()
            .with_context(|| format!("resolve {}", manifest_path.display()))?;
        let manifest_path = pet_dir.join(
            manifest_path
                .file_name()
                .context("pet manifest has no file name")?,
        );
        let manifest: Manifest = serde_json::from_str(
            &fs::read_to_string(&manifest_path)
                .with_context(|| format!("read {}", manifest_path.display()))?,
        )
        .with_context(|| format!("parse {}", manifest_path.display()))?;

        let spritesheet_path = manifest
            .spritesheet_path
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .unwrap_or("spritesheet.webp");
        let spritesheet_path = resolve_child_path(&pet_dir, spritesheet_path)?;
        let image = image::open(&spritesheet_path)
            .with_context(|| format!("read {}", spritesheet_path.display()))?
            .to_rgba8();
        let frame = manifest.frame.unwrap_or(FrameSpec {
            width: FRAME_WIDTH,
            height: FRAME_HEIGHT,
            columns: FRAME_COLUMNS,
            rows: image.height() / FRAME_HEIGHT,
        });
        validate_frame_spec(frame, image.width(), image.height())?;
        let frame_count = usize::try_from(frame.columns * frame.rows)?;
        let animations = load_animations(manifest.animations, frame.rows, frame_count)?;

        Ok(Self {
            image,
            frame,
            animations,
        })
    }

    fn frame_png(&self, index: usize) -> Result<Vec<u8>> {
        let frame = self.frame_image(index)?;
        let mut encoded = Vec::new();
        DynamicImage::ImageRgba8(frame)
            .write_to(&mut Cursor::new(&mut encoded), ImageFormat::Png)
            .context("encode PNG frame")?;
        Ok(encoded)
    }

    fn frame_image(&self, index: usize) -> Result<RgbaImage> {
        let frame_count = usize::try_from(self.frame.columns * self.frame.rows)?;
        if index >= frame_count {
            bail!("animation frame {index} exceeds {frame_count} available frames");
        }
        let index = u32::try_from(index)?;
        let x = index % self.frame.columns * self.frame.width;
        let y = index / self.frame.columns * self.frame.height;
        Ok(
            image::imageops::crop_imm(&self.image, x, y, self.frame.width, self.frame.height)
                .to_image(),
        )
    }
}

impl Animation {
    fn current_frame(&self, elapsed: Duration) -> (&AnimationFrame, Option<Duration>) {
        let elapsed = elapsed.as_nanos();
        let total = self.total_duration().as_nanos();
        let effective_elapsed = self.loop_start.and_then(|start| {
            let prefix = self.frames[..start]
                .iter()
                .map(|frame| frame.duration.as_nanos())
                .sum::<u128>();
            let loop_duration = self.frames[start..]
                .iter()
                .map(|frame| frame.duration.as_nanos())
                .sum::<u128>();
            (elapsed >= total && loop_duration > 0)
                .then_some(prefix + elapsed.saturating_sub(prefix) % loop_duration)
        });
        let mut remaining = effective_elapsed.unwrap_or(elapsed);
        for frame in &self.frames {
            let duration = frame.duration.as_nanos().max(1);
            if remaining < duration {
                return (frame, Some(duration_from_nanos(duration - remaining)));
            }
            remaining = remaining.saturating_sub(duration);
        }
        (
            self.frames.last().expect("validated non-empty animation"),
            None,
        )
    }

    fn total_duration(&self) -> Duration {
        self.frames.iter().map(|frame| frame.duration).sum()
    }
}

struct Renderer {
    protocol: ImageProtocol,
    columns: u16,
    rows: u16,
    png_frames: Vec<Vec<u8>>,
    sixel_frames: Vec<Vec<u8>>,
    frame_files: Option<tempfile::TempDir>,
    stdout: io::Stdout,
}

impl Renderer {
    fn new(pet: &Pet, protocol: ImageProtocol, height: u16) -> Result<Self> {
        let rows = (f64::from(height) / 15.0).round().max(1.0) as u16;
        let aspect = f64::from(pet.frame.height) / f64::from(pet.frame.width) * 0.52;
        let columns = (f64::from(rows) / aspect).round().max(1.0) as u16;
        let frame_count = usize::try_from(pet.frame.columns * pet.frame.rows)?;
        let png_frames = (0..frame_count)
            .map(|index| pet.frame_png(index))
            .collect::<Result<Vec<_>>>()?;
        let sixel_frames = if matches!(protocol, ImageProtocol::Sixel) {
            (0..frame_count)
                .map(|index| {
                    let frame = pet.frame_image(index)?;
                    let width = ((u64::from(frame.width()) * u64::from(height))
                        / u64::from(frame.height()))
                    .max(1) as u32;
                    let resized = image::imageops::resize(
                        &frame,
                        width,
                        u32::from(height),
                        image::imageops::FilterType::Lanczos3,
                    );
                    sixel::encode_rgba(&resized.into_raw(), width, u32::from(height))
                })
                .collect::<Result<Vec<_>>>()?
        } else {
            Vec::new()
        };
        let frame_files = if matches!(protocol, ImageProtocol::KittyFile) {
            let dir = tempfile::tempdir().context("create temporary iTerm2 frame directory")?;
            for (index, png) in png_frames.iter().enumerate() {
                fs::write(dir.path().join(format!("frame_{index:03}.png")), png)?;
            }
            Some(dir)
        } else {
            None
        };

        Ok(Self {
            protocol,
            columns,
            rows,
            png_frames,
            sixel_frames,
            frame_files,
            stdout: io::stdout(),
        })
    }

    fn render(&mut self, index: usize) -> Result<()> {
        write!(self.stdout, "{ESC}7")?;
        match self.protocol {
            ImageProtocol::Kitty => {
                write!(self.stdout, "{}", kitty_delete())?;
                write!(
                    self.stdout,
                    "{}",
                    kitty_inline(&self.png_frames[index], self.columns, self.rows)?
                )?;
            }
            ImageProtocol::KittyFile => {
                write!(self.stdout, "{}", kitty_delete())?;
                let path = self
                    .frame_files
                    .as_ref()
                    .expect("iTerm2 renderer owns its temporary frames")
                    .path()
                    .join(format!("frame_{index:03}.png"));
                write!(
                    self.stdout,
                    "{}",
                    kitty_file(&path, self.columns, self.rows)?
                )?;
            }
            ImageProtocol::Sixel => {
                clear_sixel_area(&mut self.stdout, self.rows)?;
                write!(self.stdout, "{ESC}8{ESC}7")?;
                self.stdout.write_all(&self.sixel_frames[index])?;
            }
        }
        write!(self.stdout, "{ESC}8")?;
        self.stdout.flush()?;
        Ok(())
    }

    fn clear(&mut self) -> Result<()> {
        write!(self.stdout, "{ESC}7")?;
        match self.protocol {
            ImageProtocol::Kitty | ImageProtocol::KittyFile => {
                write!(self.stdout, "{}", kitty_delete())?
            }
            ImageProtocol::Sixel => clear_sixel_area(&mut self.stdout, self.rows)?,
        }
        write!(self.stdout, "{ESC}8")?;
        self.stdout.flush()?;
        Ok(())
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    if !io::stdout().is_terminal() {
        bail!("terminal-sprite-pet must write to an interactive terminal");
    }
    if let Some(seconds) = cli.duration
        && (!seconds.is_finite() || seconds <= 0.0)
    {
        bail!("--duration must be a positive, finite number of seconds");
    }

    let pet = Pet::load(&cli.pet)?;
    let state = pet.animations.get(&cli.state).with_context(|| {
        let names = pet
            .animations
            .keys()
            .map(String::as_str)
            .collect::<Vec<_>>()
            .join(", ");
        format!("unknown --state {}; available: {names}", cli.state)
    })?;
    let protocol = resolve_protocol(cli.protocol)?;
    let mut renderer = Renderer::new(&pet, protocol, cli.height)?;
    let started = Instant::now();
    let limit = cli.duration.map(Duration::from_secs_f64);

    loop {
        let elapsed = started.elapsed();
        if limit.is_some_and(|limit| elapsed >= limit) {
            break;
        }
        let (frame, next_delay) = state.current_frame(elapsed);
        renderer.render(frame.sprite_index)?;
        let delay = next_delay.unwrap_or(Duration::from_millis(250));
        thread::sleep(limit.map_or(delay, |limit| delay.min(limit.saturating_sub(elapsed))));
    }
    renderer.clear()
}

fn resolve_child_path(pet_dir: &Path, value: &str) -> Result<PathBuf> {
    let path = Path::new(value);
    if path.is_absolute()
        || path
            .components()
            .any(|part| matches!(part, Component::ParentDir | Component::Prefix(_)))
    {
        bail!("spritesheetPath must stay inside {}", pet_dir.display());
    }
    let path = pet_dir.join(path).canonicalize()?;
    if !path.starts_with(pet_dir) {
        bail!("spritesheetPath must stay inside {}", pet_dir.display());
    }
    Ok(path)
}

fn validate_frame_spec(frame: FrameSpec, width: u32, height: u32) -> Result<()> {
    if frame.width != FRAME_WIDTH || frame.height != FRAME_HEIGHT || frame.columns != FRAME_COLUMNS
    {
        bail!("Codex-compatible pets use 192x208 frames in 8 columns");
    }
    if !matches!(frame.rows, V1_ROWS | V2_ROWS)
        || width != FRAME_WIDTH * FRAME_COLUMNS
        || height != FRAME_HEIGHT * frame.rows
    {
        bail!("spritesheet must be 1536x1872 (v1) or 1536x2288 (v2)");
    }
    Ok(())
}

fn load_animations(
    specs: BTreeMap<String, AnimationSpec>,
    rows: u32,
    frame_count: usize,
) -> Result<BTreeMap<String, Animation>> {
    let mut animations = default_animations(rows);
    for (name, spec) in specs {
        if spec.frames.is_empty() {
            bail!("animation {name} must include at least one frame");
        }
        if spec.frames.iter().any(|index| *index >= frame_count) {
            bail!("animation {name} references a frame outside the spritesheet");
        }
        let fps = spec.fps.unwrap_or(8.0);
        if !fps.is_finite() || !(0.0..=60.0).contains(&fps) || fps == 0.0 {
            bail!("animation {name} fps must be finite and between 0 and 60");
        }
        animations.insert(
            name,
            Animation {
                frames: spec
                    .frames
                    .into_iter()
                    .map(|sprite_index| AnimationFrame {
                        sprite_index,
                        duration: Duration::from_secs_f64(1.0 / fps),
                    })
                    .collect(),
                loop_start: spec.loop_animation.unwrap_or(true).then_some(0),
            },
        );
    }
    Ok(animations)
}

fn default_animations(rows: u32) -> BTreeMap<String, Animation> {
    let mut animations = BTreeMap::from([
        ("idle".to_string(), idle_animation()),
        ("running-right".to_string(), state_animation(1, 8, 120, 220)),
        ("running-left".to_string(), state_animation(2, 8, 120, 220)),
        ("waving".to_string(), state_animation(3, 4, 140, 280)),
        ("jumping".to_string(), state_animation(4, 5, 140, 280)),
        ("failed".to_string(), state_animation(5, 8, 140, 240)),
        ("waiting".to_string(), state_animation(6, 6, 150, 260)),
        ("running".to_string(), state_animation(7, 6, 120, 220)),
        ("review".to_string(), state_animation(8, 6, 150, 280)),
    ]);
    if rows == V2_ROWS {
        for (name, index) in [
            ("look-up", 72),
            ("look-up-right", 74),
            ("look-right", 76),
            ("look-down-right", 78),
            ("look-down", 80),
            ("look-down-left", 82),
            ("look-left", 84),
            ("look-up-left", 86),
        ] {
            animations.insert(name.to_string(), single_frame_animation(index));
        }
    }
    animations
}

fn idle_animation() -> Animation {
    Animation {
        frames: [(0, 1680), (1, 660), (2, 660), (3, 840), (4, 840), (5, 1920)]
            .into_iter()
            .map(|(sprite_index, duration)| AnimationFrame {
                sprite_index,
                duration: Duration::from_millis(duration),
            })
            .collect(),
        loop_start: Some(0),
    }
}

fn state_animation(row: usize, count: usize, duration: u64, final_duration: u64) -> Animation {
    let primary = (0..count)
        .map(|column| AnimationFrame {
            sprite_index: row * FRAME_COLUMNS as usize + column,
            duration: Duration::from_millis(if column + 1 == count {
                final_duration
            } else {
                duration
            }),
        })
        .collect::<Vec<_>>();
    let loop_start = primary.len() * 3;
    let frames = primary
        .iter()
        .chain(&primary)
        .chain(&primary)
        .cloned()
        .chain(idle_animation().frames)
        .collect();
    Animation {
        frames,
        loop_start: Some(loop_start),
    }
}

fn single_frame_animation(sprite_index: usize) -> Animation {
    Animation {
        frames: vec![AnimationFrame {
            sprite_index,
            duration: Duration::from_secs(1),
        }],
        loop_start: Some(0),
    }
}

fn resolve_protocol(selection: ProtocolSelection) -> Result<ImageProtocol> {
    match selection {
        ProtocolSelection::Kitty => Ok(ImageProtocol::Kitty),
        ProtocolSelection::Sixel => Ok(ImageProtocol::Sixel),
        ProtocolSelection::Auto => detect_protocol(),
    }
}

fn detect_protocol() -> Result<ImageProtocol> {
    if env::var_os("TMUX").is_some() || env::var_os("TMUX_PANE").is_some() {
        bail!("auto mode disables pets in tmux; use a terminal outside tmux");
    }
    if ["ZELLIJ", "ZELLIJ_SESSION_NAME", "ZELLIJ_VERSION"]
        .iter()
        .any(|name| env::var_os(name).is_some())
    {
        bail!("auto mode disables pets in Zellij; use a terminal outside Zellij");
    }
    if env::var_os("KITTY_WINDOW_ID").is_some()
        || env::var_os("WEZTERM_EXECUTABLE").is_some()
        || env::var_os("WEZTERM_VERSION").is_some()
    {
        return Ok(ImageProtocol::Kitty);
    }

    let term_program = env::var("TERM_PROGRAM").unwrap_or_default();
    let term = env::var("TERM").unwrap_or_default();
    if term_program.to_ascii_lowercase().contains("iterm") {
        let version = env::var("TERM_PROGRAM_VERSION").ok();
        if version_at_least(version.as_deref(), (3, 6, 0)) {
            return Ok(ImageProtocol::KittyFile);
        }
        bail!("iTerm2 3.6 or newer is required for terminal pets");
    }
    if [term_program.as_str(), term.as_str()]
        .iter()
        .any(|value| contains_any(value, &["kitty", "ghostty", "wezterm"]))
    {
        return Ok(ImageProtocol::Kitty);
    }
    if env::var_os("WT_SESSION").is_some() || contains_any(&term, &["sixel", "mlterm", "foot"]) {
        return Ok(ImageProtocol::Sixel);
    }
    bail!(
        "no supported image protocol detected; use Windows Terminal, iTerm2 3.6+, Kitty, Ghostty, WezTerm, or --protocol kitty|sixel"
    )
}

fn contains_any(value: &str, needles: &[&str]) -> bool {
    let value = value.to_ascii_lowercase();
    needles.iter().any(|needle| value.contains(needle))
}

fn version_at_least(version: Option<&str>, minimum: (u64, u64, u64)) -> bool {
    let Some(version) = version else {
        return false;
    };
    let mut parts = version.split('.').map(str::parse::<u64>);
    let parsed = match (parts.next(), parts.next(), parts.next(), parts.next()) {
        (Some(Ok(major)), Some(Ok(minor)), patch, None) => {
            (major, minor, patch.and_then(|part| part.ok()).unwrap_or(0))
        }
        (Some(Ok(major)), None, None, None) => (major, 0, 0),
        _ => return false,
    };
    parsed >= minimum
}

fn kitty_delete() -> String {
    wrap_tmux(&format!("{ESC}_Ga=d,d=I,i={PET_IMAGE_ID},q=2;{ST}"))
}

fn kitty_inline(png: &[u8], columns: u16, rows: u16) -> Result<String> {
    let payload = general_purpose::STANDARD.encode(png);
    let mut output = String::new();
    for (index, chunk) in payload.as_bytes().chunks(KITTY_CHUNK_SIZE).enumerate() {
        let chunk = std::str::from_utf8(chunk)?;
        let more = u8::from((index + 1) * KITTY_CHUNK_SIZE < payload.len());
        if index == 0 {
            output.push_str(&format!("{ESC}_Ga=T,t=d,f=100,c={columns},r={rows},q=2,i={PET_IMAGE_ID},m={more};{chunk}{ST}"));
        } else {
            output.push_str(&format!("{ESC}_Gm={more};{chunk}{ST}"));
        }
    }
    Ok(wrap_tmux(&output))
}

fn kitty_file(path: &Path, columns: u16, rows: u16) -> Result<String> {
    let payload =
        general_purpose::STANDARD.encode(path.canonicalize()?.to_string_lossy().as_bytes());
    Ok(wrap_tmux(&format!(
        "{ESC}_Ga=T,t=f,f=100,c={columns},r={rows},q=2,i={PET_IMAGE_ID};{payload}{ST}"
    )))
}

fn wrap_tmux(command: &str) -> String {
    if env::var_os("TMUX").is_none() {
        return command.to_string();
    }
    format!("{ESC}Ptmux;{}{ST}", command.replace(ESC, "\x1b\x1b"))
}

fn clear_sixel_area(out: &mut impl Write, rows: u16) -> io::Result<()> {
    for row in 0..rows {
        write!(out, "{ESC}[2K")?;
        if row + 1 < rows {
            write!(out, "{ESC}[1B\r")?;
        }
    }
    Ok(())
}

fn duration_from_nanos(nanos: u128) -> Duration {
    Duration::from_nanos(nanos.min(u128::from(u64::MAX)) as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn v2_default_look_states_use_extended_rows() {
        let animations = default_animations(V2_ROWS);
        assert_eq!(
            animations["look-left"].frames[0].sprite_index,
            ((V1_ROWS + 1) * FRAME_COLUMNS + 4) as usize
        );
        assert!(!default_animations(V1_ROWS).contains_key("look-left"));
    }

    #[test]
    fn loads_a_codex_v2_pet_package() {
        let dir = tempfile::tempdir().unwrap();
        let image = RgbaImage::new(FRAME_WIDTH * FRAME_COLUMNS, FRAME_HEIGHT * V2_ROWS);
        image.save(dir.path().join("spritesheet.png")).unwrap();
        fs::write(
            dir.path().join("pet.json"),
            r#"{"spriteVersionNumber":2,"spritesheetPath":"spritesheet.png"}"#,
        )
        .unwrap();

        let pet = Pet::load(dir.path()).unwrap();

        assert_eq!(pet.frame.rows, V2_ROWS);
        assert!(pet.animations.contains_key("look-up"));
    }
}
