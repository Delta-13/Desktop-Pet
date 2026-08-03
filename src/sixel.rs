//! Compact Sixel encoder adapted from OpenAI Codex (Apache-2.0).

use anyhow::Context;
use anyhow::Result;
use anyhow::bail;

const ST: &[u8] = b"\x1b\\";
const BAND_HEIGHT: u32 = 6;
const PALETTE_SIZE: usize = 256;
const ALPHA_THRESHOLD: u8 = 128;

pub fn encode_rgba(rgba: &[u8], width: u32, height: u32) -> Result<Vec<u8>> {
    if width == 0 || height == 0 {
        bail!("sixel image dimensions must be non-zero");
    }
    let expected = usize::try_from(u64::from(width) * u64::from(height) * 4)?;
    if rgba.len() != expected {
        bail!(
            "sixel RGBA buffer has {} bytes, expected {expected}",
            rgba.len()
        );
    }

    let palette = Palette::from_rgba(rgba);
    let mut output = format!("\x1bP9;1;0q\"1;1;{width};{height}").into_bytes();
    palette.write_definitions(&mut output);
    for band_top in (0..height).step_by(BAND_HEIGHT as usize) {
        let colors = active_colors(rgba, width, height, band_top, &palette)?;
        for (position, color) in colors.iter().enumerate() {
            output.extend_from_slice(format!("#{color}").as_bytes());
            let mut run = None;
            let mut length = 0;
            for x in 0..width {
                push_run(
                    &mut run,
                    &mut length,
                    &mut output,
                    sixel_column(rgba, width, height, band_top, x, *color)?,
                );
            }
            flush_run(&mut run, &mut length, &mut output);
            if position + 1 < colors.len() {
                output.push(b'$');
            }
        }
        if band_top + BAND_HEIGHT < height {
            output.extend_from_slice(if colors.is_empty() { b"-" } else { b"$-" });
        }
    }
    output.extend_from_slice(ST);
    Ok(output)
}

fn active_colors(
    rgba: &[u8],
    width: u32,
    height: u32,
    band_top: u32,
    palette: &Palette,
) -> Result<Vec<u8>> {
    let mut active = [false; PALETTE_SIZE];
    for y in band_top..height.min(band_top + BAND_HEIGHT) {
        for x in 0..width {
            if let Some(color) = color_at(rgba, width, x, y)? {
                active[usize::from(color)] = true;
            }
        }
    }
    Ok(palette
        .indices()
        .filter(|color| active[usize::from(*color)])
        .collect())
}

fn sixel_column(
    rgba: &[u8],
    width: u32,
    height: u32,
    band_top: u32,
    x: u32,
    color: u8,
) -> Result<u8> {
    let mut mask = 0;
    for bit in 0..BAND_HEIGHT {
        let y = band_top + bit;
        if y < height && color_at(rgba, width, x, y)? == Some(color) {
            mask |= 1 << bit;
        }
    }
    Ok(b'?' + mask)
}

fn color_at(rgba: &[u8], width: u32, x: u32, y: u32) -> Result<Option<u8>> {
    let offset = usize::try_from((u64::from(y) * u64::from(width) + u64::from(x)) * 4)
        .context("sixel pixel index overflow")?;
    if rgba[offset + 3] < ALPHA_THRESHOLD {
        return Ok(None);
    }
    Ok(Some(rgb332(
        rgba[offset],
        rgba[offset + 1],
        rgba[offset + 2],
    )))
}

fn rgb332(red: u8, green: u8, blue: u8) -> u8 {
    (red >> 5) << 5 | (green >> 5) << 2 | (blue >> 6)
}

fn rgb332_color(index: u8) -> (u8, u8, u8) {
    let scale =
        |value: u8, max: u8| u8::try_from(u16::from(value) * 255 / u16::from(max)).unwrap_or(255);
    (
        scale(index >> 5, 7),
        scale((index >> 2) & 7, 7),
        scale(index & 3, 3),
    )
}

fn push_run(run: &mut Option<u8>, length: &mut usize, output: &mut Vec<u8>, byte: u8) {
    match *run {
        Some(current) if current == byte => *length += 1,
        _ => {
            flush_run(run, length, output);
            *run = Some(byte);
            *length = 1;
        }
    }
}

fn flush_run(run: &mut Option<u8>, length: &mut usize, output: &mut Vec<u8>) {
    let Some(byte) = run.take() else {
        return;
    };
    if *length > 3 {
        output.extend_from_slice(format!("!{}", *length).as_bytes());
        output.push(byte);
    } else {
        output.extend(std::iter::repeat_n(byte, *length));
    }
    *length = 0;
}

struct Palette {
    used: [bool; PALETTE_SIZE],
}

impl Palette {
    fn from_rgba(rgba: &[u8]) -> Self {
        let mut used = [false; PALETTE_SIZE];
        for pixel in rgba
            .chunks_exact(4)
            .filter(|pixel| pixel[3] >= ALPHA_THRESHOLD)
        {
            used[usize::from(rgb332(pixel[0], pixel[1], pixel[2]))] = true;
        }
        Self { used }
    }

    fn indices(&self) -> impl Iterator<Item = u8> + '_ {
        (0..=u8::MAX).filter(|index| self.used[usize::from(*index)])
    }

    fn write_definitions(&self, output: &mut Vec<u8>) {
        for index in self.indices() {
            let (red, green, blue) = rgb332_color(index);
            let percent = |value| u16::from(value) * 100 / 255;
            output.extend_from_slice(
                format!(
                    "#{index};2;{};{};{}",
                    percent(red),
                    percent(green),
                    percent(blue)
                )
                .as_bytes(),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encodes_a_transparent_background() {
        let sixel = String::from_utf8(encode_rgba(&[255, 0, 0, 255], 1, 1).unwrap()).unwrap();
        assert_eq!(sixel, "\x1bP9;1;0q\"1;1;1;1#224;2;100;0;0#224@\x1b\\");
    }
}
