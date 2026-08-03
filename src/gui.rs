use std::fs;
use std::path::PathBuf;
use std::time::Duration;
use std::time::Instant;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use anyhow::Context;
use anyhow::Result;
use eframe::egui;
use eframe::egui::Color32;
use eframe::egui::ColorImage;
use eframe::egui::Frame;
use eframe::egui::Margin;
use eframe::egui::RichText;
use eframe::egui::Stroke;
use eframe::egui::TextureHandle;
use eframe::egui::TextureOptions;

use crate::Pet;

const CANVAS: Color32 = Color32::from_rgb(12, 14, 18);
const SURFACE: Color32 = Color32::from_rgb(20, 23, 29);
const SURFACE_RAISED: Color32 = Color32::from_rgb(28, 32, 40);
const TEXT_MUTED: Color32 = Color32::from_rgb(156, 165, 181);
const ACCENT: Color32 = Color32::from_rgb(139, 132, 255);
const ACCENT_SOFT: Color32 = Color32::from_rgb(47, 47, 79);
const SUCCESS: Color32 = Color32::from_rgb(107, 218, 169);
const MANAGER_SIZE: [f32; 2] = [1060.0, 720.0];
const DESKTOP_PET_SIZE: [f32; 2] = [360.0, 440.0];
const REST_AFTER_IDLE: Duration = Duration::from_secs(10);
const RANDOM_WALK_DELAY: Duration = Duration::from_secs(7);
const RANDOM_WALK_DURATION: Duration = Duration::from_secs(2);
const DRAG_START_DISTANCE_POINTS: f32 = 5.0;
const DRAG_SPEED_DISTANCE_POINTS: f32 = 4.0;
const DRAG_SPEED_THRESHOLD_POINTS_PER_SECOND: f32 = 120.0;
const HORIZONTAL_DEAD_ZONE_POINTS: f32 = 3.0;
const EFFECTIVE_MOTION_POINTS: f32 = 1.0;
const CLICK_MAX_DURATION: Duration = Duration::from_millis(250);
const DIRECTION_SWITCH_DELAY: Duration = Duration::from_millis(30);
const DRAG_STILL_DELAY: Duration = Duration::from_millis(80);
const HOVER_CENTER_DEAD_ZONE_POINTS: f32 = 25.0;
const HOVER_DIRECTION_HYSTERESIS_RADIANS: f32 = 8.0_f32.to_radians();

pub fn run(initial_pet: Option<PathBuf>, start_desktop: bool) -> Result<()> {
    let viewport = egui::ViewportBuilder::default()
        .with_inner_size(if start_desktop {
            DESKTOP_PET_SIZE
        } else {
            MANAGER_SIZE
        })
        .with_min_inner_size(if start_desktop {
            DESKTOP_PET_SIZE
        } else {
            [820.0, 560.0]
        })
        .with_resizable(!start_desktop)
        .with_decorations(!start_desktop)
        .with_transparent(true)
        .with_window_level(if start_desktop {
            egui::WindowLevel::AlwaysOnTop
        } else {
            egui::WindowLevel::Normal
        });
    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };
    eframe::run_native(
        "Desktop Pet",
        options,
        Box::new(move |creation_context| {
            Ok(Box::new(PetApp::new(
                creation_context,
                initial_pet,
                start_desktop,
            )))
        }),
    )
    .map_err(|error| anyhow::anyhow!(error.to_string()))
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum Language {
    #[default]
    English,
    Chinese,
    Japanese,
}

impl Language {
    fn copy(self) -> UiCopy {
        match self {
            Self::English => UiCopy {
                subtitle: "A playful desktop companion for Codex-compatible pets",
                pet_ready: "PET READY",
                awaiting_pet: "AWAITING PET",
                open_desktop: "Desktop pet mode",
                controller: "Controller",
                desktop_settings: "03  /  DESKTOP PET",
                desktop_settings_title: "Desktop companion settings",
                pet_size: "Pet size",
                random_walk: "Let the pet wander",
                random_walk_hint: "When idle, it takes short walks across your desktop.",
                right_click_hint: "Right-click the desktop pet to return to these settings.",
                pet_package: "01  /  PET PACKAGE",
                pick_local: "Pick a local pet",
                load_help: "Load any folder containing pet.json and its spritesheet.",
                choose_folder: "Choose pet folder",
                path_hint: "Paste a pet folder path",
                load: "Load",
                animation_state: "02  /  ANIMATION STATE",
                set_mood: "Set the mood",
                mood_help: "Choose an animation or use a desktop activity below.",
                unlock_states: "Load a pet to unlock its states.",
                live_preview: "LIVE PREVIEW",
                at_rest: "Your pet, at rest",
                drag_preview: "DRAG PET TO REPOSITION",
                empty_preview: "Pick a pet package\nto bring this space to life",
                terminal_command: "TERMINAL COMMAND",
                command_unavailable: "Load a pet to generate a command",
                copy: "Copy",
                local_only: "LOCAL ONLY",
                privacy: "No account, sync, or pet data leaves this device.",
                controller_hint: "Codex-compatible · desktop companion",
                feedback_loaded: "Pet ready. Choose an activity, then drag it around the preview.",
                empty_path: "Enter a folder that contains pet.json first.",
                copied: "Terminal command copied. It has not been launched.",
                activity_unavailable: "This pet does not provide that activity.",
            },
            Self::Chinese => UiCopy {
                subtitle: "为兼容 Codex 的宠物打造的桌面互动伙伴",
                pet_ready: "宠物已就绪",
                awaiting_pet: "等待载入宠物",
                open_desktop: "进入桌宠模式",
                controller: "控制台",
                desktop_settings: "03  /  桌面宠物",
                desktop_settings_title: "桌宠设置",
                pet_size: "宠物大小",
                random_walk: "让宠物随机走动",
                random_walk_hint: "空闲时，它会在桌面上进行短距离散步。",
                right_click_hint: "右键点击桌宠即可返回这些设置。",
                pet_package: "01  /  宠物包",
                pick_local: "选择本地宠物",
                load_help: "载入包含 pet.json 和精灵图的任意文件夹。",
                choose_folder: "选择宠物文件夹",
                path_hint: "粘贴宠物文件夹路径",
                load: "载入",
                animation_state: "02  /  动画状态",
                set_mood: "设定心情",
                mood_help: "选择动画，或使用下方的桌宠互动动作。",
                unlock_states: "载入宠物后即可使用其状态。",
                live_preview: "实时预览",
                at_rest: "你的宠物，正在休息",
                drag_preview: "拖动宠物以重新定位",
                empty_preview: "选择一个宠物包\n让这里充满活力",
                terminal_command: "终端命令",
                command_unavailable: "载入宠物后生成命令",
                copy: "复制",
                local_only: "仅本地运行",
                privacy: "不会上传账户、同步数据或宠物数据。",
                controller_hint: "兼容 Codex · 桌面伙伴",
                feedback_loaded: "宠物已就绪。选择动作，然后在预览中拖动它。",
                empty_path: "请先输入包含 pet.json 的文件夹。",
                copied: "终端命令已复制；程序不会启动终端或进程。",
                activity_unavailable: "此宠物没有提供该互动动作。",
            },
            Self::Japanese => UiCopy {
                subtitle: "Codex 互換ペットのための、遊べるデスクトップ相棒",
                pet_ready: "ペット準備完了",
                awaiting_pet: "ペットを待機中",
                open_desktop: "デスクトップペットにする",
                controller: "コントローラー",
                desktop_settings: "03  /  デスクトップペット",
                desktop_settings_title: "デスクトップ相棒の設定",
                pet_size: "ペットのサイズ",
                random_walk: "ペットをランダムに散歩させる",
                random_walk_hint: "何もしない間、デスクトップを少しだけ散歩します。",
                right_click_hint: "デスクトップペットを右クリックすると、この設定に戻れます。",
                pet_package: "01  /  ペットパッケージ",
                pick_local: "ローカルのペットを選ぶ",
                load_help: "pet.json とスプライトシートを含むフォルダーを読み込みます。",
                choose_folder: "ペットフォルダーを選ぶ",
                path_hint: "ペットフォルダーのパスを貼り付け",
                load: "読み込む",
                animation_state: "02  /  アニメーション状態",
                set_mood: "気分を選ぶ",
                mood_help: "アニメーションを選ぶか、下のデスクトップ操作を使います。",
                unlock_states: "ペットを読み込むと状態を利用できます。",
                live_preview: "ライブプレビュー",
                at_rest: "あなたのペットはおやすみ中",
                drag_preview: "ペットをドラッグして位置を変える",
                empty_preview: "ペットパッケージを選んで\nこの場所に命を吹き込みましょう",
                terminal_command: "ターミナルコマンド",
                command_unavailable: "ペットを読み込むとコマンドを生成します",
                copy: "コピー",
                local_only: "ローカルのみ",
                privacy: "アカウント、同期、ペットデータは端末の外へ送信されません。",
                controller_hint: "Codex 互換 · デスクトップ相棒",
                feedback_loaded: "ペットの準備ができました。動きを選び、プレビューでドラッグできます。",
                empty_path: "先に pet.json を含むフォルダーを入力してください。",
                copied: "ターミナルコマンドをコピーしました。実行はしていません。",
                activity_unavailable: "このペットにはその動きがありません。",
            },
        }
    }
}

struct UiCopy {
    subtitle: &'static str,
    pet_ready: &'static str,
    awaiting_pet: &'static str,
    open_desktop: &'static str,
    controller: &'static str,
    desktop_settings: &'static str,
    desktop_settings_title: &'static str,
    pet_size: &'static str,
    random_walk: &'static str,
    random_walk_hint: &'static str,
    right_click_hint: &'static str,
    pet_package: &'static str,
    pick_local: &'static str,
    load_help: &'static str,
    choose_folder: &'static str,
    path_hint: &'static str,
    load: &'static str,
    animation_state: &'static str,
    set_mood: &'static str,
    mood_help: &'static str,
    unlock_states: &'static str,
    live_preview: &'static str,
    at_rest: &'static str,
    drag_preview: &'static str,
    empty_preview: &'static str,
    terminal_command: &'static str,
    command_unavailable: &'static str,
    copy: &'static str,
    local_only: &'static str,
    privacy: &'static str,
    controller_hint: &'static str,
    feedback_loaded: &'static str,
    empty_path: &'static str,
    copied: &'static str,
    activity_unavailable: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DisplayMode {
    Controller,
    DesktopPet,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum HorizontalDirection {
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum LookDirection {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl LookDirection {
    fn animation_name(self) -> &'static str {
        match self {
            Self::Up => "look-up",
            Self::UpRight => "look-up-right",
            Self::Right => "look-right",
            Self::DownRight => "look-down-right",
            Self::Down => "look-down",
            Self::DownLeft => "look-down-left",
            Self::Left => "look-left",
            Self::UpLeft => "look-up-left",
        }
    }

    fn center_angle(self) -> f32 {
        match self {
            Self::Right => 0.0,
            Self::DownRight => std::f32::consts::FRAC_PI_4,
            Self::Down => std::f32::consts::FRAC_PI_2,
            Self::DownLeft => 3.0 * std::f32::consts::FRAC_PI_4,
            Self::Left => std::f32::consts::PI,
            Self::UpLeft => -3.0 * std::f32::consts::FRAC_PI_4,
            Self::Up => -std::f32::consts::FRAC_PI_2,
            Self::UpRight => -std::f32::consts::FRAC_PI_4,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ClickReaction {
    Play,
    Wave,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum PetState {
    #[default]
    Idle,
    Rest,
    Look(LookDirection),
    ClickReaction(ClickReaction),
    Dragging(HorizontalDirection),
    AutoWalking(HorizontalDirection),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PointerCompletion {
    Click,
    DragEnd,
    Cancel,
}

#[derive(Clone, Debug)]
struct PointerSession {
    /// All pointer-session positions are physical screen pixels. Conversion from egui points
    /// happens once, when raw input enters the interaction state machine.
    press_position_screen: egui::Pos2,
    previous_position_screen: egui::Pos2,
    current_position_screen: egui::Pos2,
    press_time: Duration,
    accumulated_distance: f32,
    drag_started: bool,
    filtered_velocity_x: f32,
    last_horizontal_direction: HorizontalDirection,
    press_window_position_screen: egui::Pos2,
    grab_offset_screen: egui::Vec2,
    last_sample_time: Duration,
    last_effective_motion_time: Duration,
    pending_direction: Option<HorizontalDirection>,
    pending_direction_since: Duration,
    pending_direction_samples: u8,
}

impl PointerSession {
    fn new(
        pointer_screen: egui::Pos2,
        window_screen: egui::Pos2,
        press_time: Duration,
        last_horizontal_direction: HorizontalDirection,
    ) -> Self {
        Self {
            press_position_screen: pointer_screen,
            previous_position_screen: pointer_screen,
            current_position_screen: pointer_screen,
            press_time,
            accumulated_distance: 0.0,
            drag_started: false,
            filtered_velocity_x: 0.0,
            last_horizontal_direction,
            press_window_position_screen: window_screen,
            grab_offset_screen: pointer_screen - window_screen,
            last_sample_time: press_time,
            last_effective_motion_time: press_time,
            pending_direction: None,
            pending_direction_since: press_time,
            pending_direction_samples: 0,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct PointerMotion {
    target_window_position_screen: egui::Pos2,
    direction: HorizontalDirection,
    direction_changed: bool,
    drag_started: bool,
    effective_motion: bool,
}

#[derive(Clone, Copy, Debug)]
struct DesktopGeometry {
    pixels_per_point: f32,
    inner_position_screen: egui::Pos2,
    outer_rect_screen: egui::Rect,
    hit_rect_screen: egui::Rect,
    monitor_bounds_screen: Option<egui::Rect>,
}

struct LoadedPet {
    pet: Pet,
    path: PathBuf,
}

struct PetApp {
    loaded: Option<LoadedPet>,
    path_input: String,
    selected_state: String,
    animation_started: Instant,
    activity_started: Instant,
    sprite_offset: egui::Vec2,
    texture: Option<TextureHandle>,
    feedback: Option<String>,
    language: Language,
    mode: DisplayMode,
    pet_state: PetState,
    facing_direction: HorizontalDirection,
    pet_scale: f32,
    random_walking: bool,
    last_interaction: Instant,
    next_random_walk_at: Instant,
    random_walk_until: Option<Instant>,
    reaction_until: Option<Instant>,
    pointer_session: Option<PointerSession>,
    pointer_over_pet: bool,
    last_hover_position_screen: Option<egui::Pos2>,
    hover_direction: Option<LookDirection>,
    last_reaction: Option<ClickReaction>,
    repeated_reaction_count: u8,
    clock_started: Instant,
}

impl PetApp {
    fn new(
        creation_context: &eframe::CreationContext<'_>,
        initial_pet: Option<PathBuf>,
        start_desktop: bool,
    ) -> Self {
        configure_theme(&creation_context.egui_ctx);
        let now = Instant::now();
        let mut app = Self {
            loaded: None,
            path_input: String::new(),
            selected_state: "idle".to_string(),
            animation_started: now,
            activity_started: now,
            sprite_offset: egui::Vec2::ZERO,
            texture: None,
            feedback: None,
            language: Language::default(),
            mode: if start_desktop {
                DisplayMode::DesktopPet
            } else {
                DisplayMode::Controller
            },
            pet_state: PetState::Idle,
            facing_direction: HorizontalDirection::Right,
            pet_scale: 1.0,
            random_walking: false,
            last_interaction: now,
            next_random_walk_at: now + RANDOM_WALK_DELAY,
            random_walk_until: None,
            reaction_until: None,
            pointer_session: None,
            pointer_over_pet: false,
            last_hover_position_screen: None,
            hover_direction: None,
            last_reaction: None,
            repeated_reaction_count: 0,
            clock_started: now,
        };
        if let Some(path) = initial_pet {
            app.path_input = path.display().to_string();
            app.load_pet(path, &creation_context.egui_ctx);
        }
        app
    }

    fn copy(&self) -> UiCopy {
        self.language.copy()
    }

    fn choose_pet(&mut self, ctx: &egui::Context) {
        let title = match self.language {
            Language::English => "Choose a Codex pet folder",
            Language::Chinese => "选择 Codex 宠物文件夹",
            Language::Japanese => "Codex ペットフォルダーを選択",
        };
        if let Some(path) = rfd::FileDialog::new().set_title(title).pick_folder() {
            self.path_input = path.display().to_string();
            self.load_pet(path, ctx);
        }
    }

    fn load_path_input(&mut self, ctx: &egui::Context) {
        let path = PathBuf::from(self.path_input.trim());
        if path.as_os_str().is_empty() {
            self.feedback = Some(self.copy().empty_path.to_string());
            return;
        }
        self.load_pet(path, ctx);
    }

    fn load_pet(&mut self, path: PathBuf, ctx: &egui::Context) {
        match Pet::load(&path).with_context(|| format!("load pet from {}", path.display())) {
            Ok(pet) => {
                self.selected_state = pet
                    .animations
                    .contains_key("idle")
                    .then_some("idle".to_string())
                    .or_else(|| pet.animations.keys().next().cloned())
                    .unwrap_or_default();
                self.loaded = Some(LoadedPet { pet, path });
                self.animation_started = Instant::now();
                self.activity_started = Instant::now();
                self.pet_state = PetState::Idle;
                self.facing_direction = HorizontalDirection::Right;
                self.pointer_session = None;
                self.reaction_until = None;
                self.record_interaction();
                self.sprite_offset = egui::Vec2::ZERO;
                self.texture = None;
                self.feedback = Some(self.copy().feedback_loaded.to_string());
                self.refresh_preview(ctx);
            }
            Err(error) => {
                let prefix = match self.language {
                    Language::English => "Could not load pet",
                    Language::Chinese => "无法载入宠物",
                    Language::Japanese => "ペットを読み込めませんでした",
                };
                self.feedback = Some(format!("{prefix}: {error:#}"));
            }
        }
    }

    fn refresh_preview(&mut self, ctx: &egui::Context) {
        let Some(loaded) = self.loaded.as_ref() else {
            return;
        };
        let Some(animation) = loaded.pet.animations.get(&self.selected_state) else {
            return;
        };
        let (frame, next_delay) = animation.current_frame(self.animation_started.elapsed());
        let Ok(image) = loaded.pet.frame_image(frame.sprite_index) else {
            return;
        };
        let size = [image.width() as usize, image.height() as usize];
        let image = ColorImage::from_rgba_unmultiplied(size, image.as_raw());
        if let Some(texture) = self.texture.as_mut() {
            texture.set(image, TextureOptions::NEAREST);
        } else {
            self.texture = Some(ctx.load_texture("pet-preview", image, TextureOptions::NEAREST));
        }
        let repaint = if matches!(
            self.pet_state,
            PetState::ClickReaction(_) | PetState::Dragging(_) | PetState::AutoWalking(_)
        ) {
            Duration::from_millis(16)
        } else {
            next_delay.unwrap_or(Duration::from_millis(250))
        };
        ctx.request_repaint_after(repaint);
    }

    fn select_state(&mut self, state: String, ctx: &egui::Context) {
        self.selected_state = state;
        self.animation_started = Instant::now();
        self.activity_started = Instant::now();
        self.pet_state = PetState::Idle;
        self.reaction_until = None;
        self.record_interaction();
        self.refresh_preview(ctx);
    }

    fn trigger_tap(&mut self, ctx: &egui::Context) {
        let (reaction, repeated_reaction_count) = choose_click_reaction(
            random_seed(),
            self.last_reaction,
            self.repeated_reaction_count,
        );
        if self.set_pet_state(PetState::ClickReaction(reaction), ctx) {
            self.last_reaction = Some(reaction);
            self.repeated_reaction_count = repeated_reaction_count;
            let duration = self.current_reaction_duration();
            self.reaction_until = Some(Instant::now() + duration);
            self.record_interaction();
        }
    }

    fn set_pet_state(&mut self, requested: PetState, ctx: &egui::Context) -> bool {
        let (actual, candidates): (PetState, &[&str]) = match requested {
            PetState::Idle => (PetState::Idle, &["idle"]),
            PetState::Rest => (PetState::Rest, &["waiting", "idle"]),
            PetState::Look(direction) => {
                if self.has_animation(direction.animation_name()) {
                    (PetState::Look(direction), look_state_candidates(direction))
                } else {
                    (PetState::Idle, &["idle"])
                }
            }
            PetState::ClickReaction(ClickReaction::Play) => {
                (requested, &["jumping", "review", "waving"])
            }
            PetState::ClickReaction(ClickReaction::Wave) => {
                (requested, &["waving", "review", "jumping"])
            }
            PetState::Dragging(direction) | PetState::AutoWalking(direction) => {
                (requested, walk_state_candidates(direction))
            }
        };
        let next = self.loaded.as_ref().and_then(|loaded| {
            candidates
                .iter()
                .find(|name| loaded.pet.animations.contains_key(**name))
                .map(|name| (*name).to_string())
        });
        let Some(next) = next else {
            if self.loaded.is_some() {
                self.feedback = Some(self.copy().activity_unavailable.to_string());
            }
            return false;
        };

        let changed = self.pet_state != actual || self.selected_state != next;
        self.pet_state = actual;
        if let PetState::Dragging(direction) | PetState::AutoWalking(direction) = actual {
            self.facing_direction = direction;
        }
        if !matches!(actual, PetState::ClickReaction(_)) {
            self.reaction_until = None;
        }
        if changed {
            self.selected_state = next;
            self.animation_started = Instant::now();
            self.activity_started = Instant::now();
            self.refresh_preview(ctx);
        }
        true
    }

    fn has_animation(&self, name: &str) -> bool {
        self.loaded
            .as_ref()
            .is_some_and(|loaded| loaded.pet.animations.contains_key(name))
    }

    fn current_reaction_duration(&self) -> Duration {
        let Some(animation) = self
            .loaded
            .as_ref()
            .and_then(|loaded| loaded.pet.animations.get(&self.selected_state))
        else {
            return Duration::from_millis(900);
        };
        let end = animation.loop_start.unwrap_or(animation.frames.len());
        let duration: Duration = animation.frames[..end]
            .iter()
            .map(|frame| frame.duration)
            .sum();
        if duration.is_zero() {
            animation
                .total_duration()
                .clamp(Duration::from_millis(450), Duration::from_secs(3))
        } else {
            duration.clamp(Duration::from_millis(450), Duration::from_secs(3))
        }
    }

    fn record_interaction(&mut self) {
        let now = Instant::now();
        self.last_interaction = now;
        self.next_random_walk_at = now + RANDOM_WALK_DELAY;
        self.random_walk_until = None;
    }

    fn terminal_command(&self) -> Option<String> {
        let loaded = self.loaded.as_ref()?;
        Some(format!(
            "terminal-sprite-pet \"{}\" --state {}",
            loaded.path.display(),
            self.selected_state
        ))
    }

    fn state_names(&self) -> Vec<String> {
        self.loaded
            .as_ref()
            .map(|loaded| loaded.pet.animations.keys().cloned().collect())
            .unwrap_or_default()
    }

    fn activity_motion(&self) -> egui::Vec2 {
        let seconds = self.activity_started.elapsed().as_secs_f32();
        match self.pet_state {
            PetState::ClickReaction(ClickReaction::Play) => {
                egui::vec2(0.0, -(seconds * 7.0).sin().max(0.0) * 18.0)
            }
            _ => egui::Vec2::ZERO,
        }
    }

    fn desktop_image_size(&self) -> egui::Vec2 {
        scaled_pet_size(egui::vec2(300.0, 325.0), self.pet_scale)
    }

    fn desktop_window_size(&self) -> egui::Vec2 {
        (self.desktop_image_size() + egui::vec2(42.0, 44.0)).max(egui::vec2(220.0, 240.0))
    }

    fn update_desktop_behavior(&mut self, ctx: &egui::Context) {
        if self.mode != DisplayMode::DesktopPet || self.loaded.is_none() {
            return;
        }

        let now = Instant::now();
        if let Some((drag_started, last_motion, direction)) =
            self.pointer_session.as_ref().map(|session| {
                (
                    session.drag_started,
                    session.last_effective_motion_time,
                    session.last_horizontal_direction,
                )
            })
        {
            if drag_started {
                let running = drag_animation_is_active(self.clock_elapsed(), last_motion);
                if !running && matches!(self.pet_state, PetState::Dragging(_)) {
                    self.set_standing_facing(direction, ctx);
                } else if running {
                    ctx.request_repaint_after(Duration::from_millis(16));
                }
            }
            return;
        }

        if let Some(until) = self.reaction_until {
            if now < until {
                ctx.request_repaint_after(Duration::from_millis(16));
                return;
            }
            self.reaction_until = None;
            let _ = self.set_pet_state(PetState::Idle, ctx);
        }

        if self.pointer_over_pet {
            return;
        }

        if auto_walk_allowed(
            self.random_walking,
            self.pointer_session.is_some(),
            self.pointer_over_pet,
            self.pet_state,
        ) && self.random_walk_until.is_none()
            && now >= self.next_random_walk_at
            && self.pet_state == PetState::Idle
        {
            let direction = if random_seed() & 1 == 0 {
                HorizontalDirection::Left
            } else {
                HorizontalDirection::Right
            };
            if self.set_pet_state(PetState::AutoWalking(direction), ctx) {
                self.random_walk_until = Some(now + RANDOM_WALK_DURATION);
                self.next_random_walk_at =
                    now + RANDOM_WALK_DELAY + Duration::from_millis(1_000 + random_seed() % 2_000);
            }
        }

        if let Some(until) = self.random_walk_until {
            if now < until {
                self.move_window_for_random_walk(ctx);
                ctx.request_repaint_after(Duration::from_millis(16));
            } else {
                self.random_walk_until = None;
                let _ = self.set_pet_state(PetState::Idle, ctx);
            }
        } else if self.last_interaction.elapsed() >= REST_AFTER_IDLE
            && self.pet_state == PetState::Idle
        {
            let _ = self.set_pet_state(PetState::Rest, ctx);
        }
    }

    fn move_window_for_random_walk(&mut self, ctx: &egui::Context) {
        let Some(geometry) = desktop_geometry(ctx, egui::Rect::NOTHING) else {
            return;
        };
        let Some(bounds) = geometry.monitor_bounds_screen else {
            return;
        };
        let direction = match self.pet_state {
            PetState::AutoWalking(direction) => direction,
            _ => return,
        };
        let step = match direction {
            HorizontalDirection::Left => -1.5 * geometry.pixels_per_point,
            HorizontalDirection::Right => 1.5 * geometry.pixels_per_point,
        };
        let desired = geometry.outer_rect_screen.min + egui::vec2(step, 0.0);
        let clamped =
            clamp_window_position_screen(desired, geometry.outer_rect_screen.size(), bounds);
        if (clamped.x - desired.x).abs() > f32::EPSILON {
            let reversed = match direction {
                HorizontalDirection::Left => HorizontalDirection::Right,
                HorizontalDirection::Right => HorizontalDirection::Left,
            };
            let _ = self.set_pet_state(PetState::AutoWalking(reversed), ctx);
        }
        send_outer_position_screen(ctx, clamped, geometry.pixels_per_point);
    }

    fn set_standing_facing(&mut self, direction: HorizontalDirection, ctx: &egui::Context) {
        let look = match direction {
            HorizontalDirection::Left => LookDirection::Left,
            HorizontalDirection::Right => LookDirection::Right,
        };
        let requested = if self.has_animation(look.animation_name()) {
            PetState::Look(look)
        } else {
            PetState::Idle
        };
        let _ = self.set_pet_state(requested, ctx);
    }

    fn clock_elapsed(&self) -> Duration {
        self.clock_started.elapsed()
    }

    fn process_desktop_pointer(&mut self, ctx: &egui::Context, image_rect: egui::Rect) {
        let Some(geometry) = desktop_geometry(ctx, image_rect) else {
            return;
        };
        let (events, primary_down, focused, hover_position) = ctx.input(|input| {
            (
                input.raw.events.clone(),
                input.pointer.primary_down(),
                input.viewport().focused,
                input.pointer.hover_pos(),
            )
        });
        let now = self.clock_elapsed();

        let right_pressed = events.iter().find_map(|event| match event {
            egui::Event::PointerButton {
                pos,
                button: egui::PointerButton::Secondary,
                pressed: true,
                ..
            } => Some(screen_position(*pos, geometry)),
            _ => None,
        });
        let right_hits_pet =
            right_pressed.is_some_and(|position| geometry.hit_rect_screen.contains(position));
        if should_open_settings(
            right_pressed.is_some(),
            self.pointer_session.is_some(),
            right_hits_pet,
        ) {
            self.cancel_pointer_interaction(ctx, true);
            self.set_display_mode(DisplayMode::Controller, ctx);
            return;
        }

        let cancelled = events.iter().any(|event| {
            matches!(
                event,
                egui::Event::PointerGone | egui::Event::WindowFocused(false)
            )
        }) || (focused == Some(false) && self.pointer_session.is_some());
        if cancelled {
            self.cancel_pointer_interaction(ctx, true);
            self.pointer_over_pet = false;
            return;
        } else {
            let mut completed_primary = false;
            for event in events {
                match event {
                    egui::Event::PointerButton {
                        pos,
                        button: egui::PointerButton::Primary,
                        pressed: true,
                        ..
                    } => {
                        let pointer_screen = screen_position(pos, geometry);
                        if geometry.hit_rect_screen.contains(pointer_screen) {
                            self.begin_pointer_session(pointer_screen, geometry, now, ctx);
                        }
                    }
                    egui::Event::PointerMoved(pos) => {
                        let pointer_screen = screen_position(pos, geometry);
                        self.move_pointer_session(pointer_screen, now, geometry, ctx);
                    }
                    egui::Event::PointerButton {
                        pos,
                        button: egui::PointerButton::Primary,
                        pressed: false,
                        ..
                    } => {
                        let pointer_screen = screen_position(pos, geometry);
                        completed_primary |= self.pointer_session.is_some();
                        self.complete_pointer_session(pointer_screen, now, geometry, ctx);
                    }
                    _ => {}
                }
            }
            if completed_primary {
                self.pointer_over_pet = false;
                return;
            }
        }

        if self.pointer_session.is_some() && !primary_down {
            self.cancel_pointer_interaction(ctx, true);
        }
        self.update_hover_state(hover_position, geometry, ctx);
    }

    fn begin_pointer_session(
        &mut self,
        pointer_screen: egui::Pos2,
        geometry: DesktopGeometry,
        now: Duration,
        ctx: &egui::Context,
    ) {
        self.cancel_pointer_interaction(ctx, false);
        self.pointer_session = Some(PointerSession::new(
            pointer_screen,
            geometry.outer_rect_screen.min,
            now,
            self.facing_direction,
        ));
        self.pointer_over_pet = true;
        self.hover_direction = None;
        self.record_interaction();
        let _ = self.set_pet_state(PetState::Idle, ctx);
    }

    fn move_pointer_session(
        &mut self,
        pointer_screen: egui::Pos2,
        now: Duration,
        geometry: DesktopGeometry,
        ctx: &egui::Context,
    ) {
        let Some(session) = self.pointer_session.take() else {
            return;
        };
        let (session, motion) =
            advance_pointer_session(session, pointer_screen, now, geometry.pixels_per_point);
        if motion.effective_motion {
            self.record_interaction();
        }
        if motion.drag_started {
            send_outer_position_screen(
                ctx,
                motion.target_window_position_screen,
                geometry.pixels_per_point,
            );
            if motion.effective_motion
                && (motion.direction_changed || !matches!(self.pet_state, PetState::Dragging(_)))
            {
                let _ = self.set_pet_state(PetState::Dragging(motion.direction), ctx);
            }
        }
        self.pointer_session = Some(session);
    }

    fn complete_pointer_session(
        &mut self,
        pointer_screen: egui::Pos2,
        now: Duration,
        geometry: DesktopGeometry,
        ctx: &egui::Context,
    ) {
        let Some(session) = self.pointer_session.take() else {
            return;
        };
        let (session, motion) =
            advance_pointer_session(session, pointer_screen, now, geometry.pixels_per_point);
        if motion.drag_started {
            send_outer_position_screen(
                ctx,
                motion.target_window_position_screen,
                geometry.pixels_per_point,
            );
        }
        let completion = classify_pointer_completion(&session, now, geometry.pixels_per_point);
        self.clear_pointer_tracking();
        self.record_interaction();
        match completion {
            PointerCompletion::Click => self.trigger_tap(ctx),
            PointerCompletion::DragEnd | PointerCompletion::Cancel => {
                let _ = self.set_pet_state(PetState::Idle, ctx);
            }
        }
    }

    fn cancel_pointer_interaction(&mut self, ctx: &egui::Context, record: bool) {
        let had_session = self.pointer_session.take().is_some();
        self.clear_pointer_tracking();
        if had_session {
            let _ = self.set_pet_state(PetState::Idle, ctx);
            if record {
                self.record_interaction();
            }
        }
    }

    fn clear_pointer_tracking(&mut self) {
        self.pointer_session = None;
        self.hover_direction = None;
        self.last_hover_position_screen = None;
    }

    fn update_hover_state(
        &mut self,
        hover_position: Option<egui::Pos2>,
        geometry: DesktopGeometry,
        ctx: &egui::Context,
    ) {
        if self.pointer_session.is_some() {
            self.pointer_over_pet = true;
            return;
        }
        let hover_screen = hover_position.map(|position| screen_position(position, geometry));
        let over_pet =
            hover_screen.is_some_and(|position| geometry.hit_rect_screen.contains(position));
        self.pointer_over_pet = over_pet;
        if !over_pet {
            self.last_hover_position_screen = None;
            self.hover_direction = None;
            if matches!(self.pet_state, PetState::Look(_)) {
                let _ = self.set_pet_state(PetState::Idle, ctx);
            }
            return;
        }
        let Some(hover_screen) = hover_screen else {
            return;
        };
        let significant_motion = self.last_hover_position_screen.is_none_or(|previous| {
            previous.distance(hover_screen) >= EFFECTIVE_MOTION_POINTS * geometry.pixels_per_point
        });
        self.last_hover_position_screen = Some(hover_screen);
        if significant_motion {
            self.record_interaction();
        }
        if matches!(self.pet_state, PetState::ClickReaction(_)) {
            return;
        }
        let direction = quantize_look_direction(
            hover_screen - geometry.hit_rect_screen.center(),
            HOVER_CENTER_DEAD_ZONE_POINTS * geometry.pixels_per_point,
            self.hover_direction,
        );
        self.hover_direction = direction;
        let requested = direction.map_or(PetState::Idle, PetState::Look);
        let _ = self.set_pet_state(requested, ctx);
    }

    fn set_display_mode(&mut self, mode: DisplayMode, ctx: &egui::Context) {
        if self.mode == mode {
            return;
        }
        self.cancel_pointer_interaction(ctx, false);
        self.pointer_over_pet = false;
        self.random_walk_until = None;
        self.reaction_until = None;
        let _ = self.set_pet_state(PetState::Idle, ctx);
        self.record_interaction();
        self.mode = mode;
        match mode {
            DisplayMode::Controller => {
                ctx.send_viewport_cmd(egui::ViewportCommand::Decorations(true));
                ctx.send_viewport_cmd(egui::ViewportCommand::Resizable(true));
                ctx.send_viewport_cmd(egui::ViewportCommand::WindowLevel(
                    egui::WindowLevel::Normal,
                ));
                ctx.send_viewport_cmd(egui::ViewportCommand::MinInnerSize(egui::vec2(
                    820.0, 560.0,
                )));
                ctx.send_viewport_cmd(egui::ViewportCommand::MaxInnerSize(egui::Vec2::INFINITY));
                ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(
                    MANAGER_SIZE[0],
                    MANAGER_SIZE[1],
                )));
            }
            DisplayMode::DesktopPet => {
                let desktop_size = self.desktop_window_size();
                ctx.send_viewport_cmd(egui::ViewportCommand::Decorations(false));
                ctx.send_viewport_cmd(egui::ViewportCommand::Resizable(false));
                ctx.send_viewport_cmd(egui::ViewportCommand::WindowLevel(
                    egui::WindowLevel::AlwaysOnTop,
                ));
                ctx.send_viewport_cmd(egui::ViewportCommand::MinInnerSize(egui::vec2(
                    desktop_size.x,
                    desktop_size.y,
                )));
                ctx.send_viewport_cmd(egui::ViewportCommand::MaxInnerSize(egui::vec2(
                    desktop_size.x,
                    desktop_size.y,
                )));
                ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(
                    desktop_size.x,
                    desktop_size.y,
                )));
            }
        }
    }
}

impl eframe::App for PetApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.mode == DisplayMode::DesktopPet {
            let image_rect = egui::Rect::from_center_size(
                ctx.screen_rect().center() + self.activity_motion(),
                self.desktop_image_size(),
            );
            self.process_desktop_pointer(ctx, image_rect);
            if self.mode == DisplayMode::DesktopPet {
                self.update_desktop_behavior(ctx);
            }
        }
        self.refresh_preview(ctx);
        match self.mode {
            DisplayMode::Controller => controller_view(self, ctx),
            DisplayMode::DesktopPet => desktop_pet_view(self, ctx),
        }
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.pointer_session = None;
        self.random_walk_until = None;
        self.reaction_until = None;
        self.pet_state = PetState::Idle;
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        match self.mode {
            DisplayMode::Controller => CANVAS.to_normalized_gamma_f32(),
            DisplayMode::DesktopPet => Color32::TRANSPARENT.to_normalized_gamma_f32(),
        }
    }
}

fn controller_view(app: &mut PetApp, ctx: &egui::Context) {
    let copy = app.copy();
    egui::TopBottomPanel::top("topbar")
        .frame(
            Frame::new()
                .fill(CANVAS)
                .inner_margin(Margin::symmetric(26, 16)),
        )
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new("PET").small().strong().color(ACCENT));
                ui.vertical(|ui| {
                    ui.label(
                        RichText::new(format!("terminal / sprite pet · {}", copy.controller))
                            .size(18.0)
                            .strong(),
                    );
                    ui.label(RichText::new(copy.subtitle).small().color(TEXT_MUTED));
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let ready = app.loaded.is_some();
                    status_badge(
                        ui,
                        if ready {
                            copy.pet_ready
                        } else {
                            copy.awaiting_pet
                        },
                        ready,
                    );
                    language_picker(app, ui);
                });
            });
        });

    egui::CentralPanel::default()
        .frame(Frame::new().fill(CANVAS).inner_margin(Margin::same(26)))
        .show(ctx, |ui| {
            ui.columns(2, |columns| {
                columns[0].set_min_width(374.0);
                egui::ScrollArea::vertical().show(&mut columns[0], |ui| {
                    setup_panel(app, ui, ctx);
                });
                preview_panel(app, &mut columns[1], ctx);
            });
        });

    egui::TopBottomPanel::bottom("footer")
        .frame(
            Frame::new()
                .fill(CANVAS)
                .inner_margin(Margin::symmetric(26, 14)),
        )
        .show(ctx, |ui| {
            ui.separator();
            ui.add_space(5.0);
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new(copy.local_only)
                        .small()
                        .strong()
                        .color(SUCCESS),
                );
                ui.label(RichText::new(copy.privacy).small().color(TEXT_MUTED));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        RichText::new(copy.controller_hint)
                            .small()
                            .color(TEXT_MUTED),
                    );
                });
            });
        });
}

fn setup_panel(app: &mut PetApp, ui: &mut egui::Ui, ctx: &egui::Context) {
    let copy = app.copy();
    Frame::new()
        .fill(SURFACE)
        .stroke(Stroke::new(1.0_f32, SURFACE_RAISED))
        .corner_radius(12.0)
        .inner_margin(Margin::same(20))
        .show(ui, |ui| {
            section_label(ui, copy.pet_package);
            ui.add_space(8.0);
            ui.label(RichText::new(copy.pick_local).size(22.0).strong());
            ui.label(RichText::new(copy.load_help).color(TEXT_MUTED));
            ui.add_space(16.0);

            let button = egui::Button::new(RichText::new(copy.choose_folder).strong())
                .fill(ACCENT)
                .stroke(Stroke::NONE)
                .min_size(egui::vec2(176.0, 38.0));
            if ui.add(button).clicked() {
                app.choose_pet(ctx);
            }
            ui.add_space(12.0);
            ui.horizontal(|ui| {
                ui.add_sized(
                    [252.0, 34.0],
                    egui::TextEdit::singleline(&mut app.path_input).hint_text(copy.path_hint),
                );
                if ui.button(copy.load).clicked() {
                    app.load_path_input(ctx);
                }
            });
        });

    ui.add_space(14.0);
    Frame::new()
        .fill(SURFACE)
        .stroke(Stroke::new(1.0_f32, SURFACE_RAISED))
        .corner_radius(12.0)
        .inner_margin(Margin::same(20))
        .show(ui, |ui| {
            section_label(ui, copy.animation_state);
            ui.add_space(8.0);
            ui.label(RichText::new(copy.set_mood).size(22.0).strong());
            ui.label(RichText::new(copy.mood_help).color(TEXT_MUTED));
            ui.add_space(14.0);

            let states = app.state_names();
            if states.is_empty() {
                ui.label(
                    RichText::new(copy.unlock_states)
                        .italics()
                        .color(TEXT_MUTED),
                );
            } else {
                ui.horizontal_wrapped(|ui| {
                    for state in states {
                        let active = state == app.selected_state;
                        let button = egui::Button::new(localized_state_name(app.language, &state))
                            .fill(if active { ACCENT_SOFT } else { SURFACE_RAISED })
                            .stroke(Stroke::new(
                                1.0_f32,
                                if active { ACCENT } else { SURFACE_RAISED },
                            ))
                            .corner_radius(7.0);
                        if ui.add(button).clicked() {
                            app.select_state(state, ctx);
                        }
                    }
                });
            }
        });

    ui.add_space(14.0);
    Frame::new()
        .fill(SURFACE)
        .stroke(Stroke::new(1.0_f32, SURFACE_RAISED))
        .corner_radius(12.0)
        .inner_margin(Margin::same(20))
        .show(ui, |ui| {
            section_label(ui, copy.desktop_settings);
            ui.add_space(8.0);
            ui.label(
                RichText::new(copy.desktop_settings_title)
                    .size(22.0)
                    .strong(),
            );
            ui.add_space(8.0);
            ui.add(
                egui::Slider::new(&mut app.pet_scale, 0.60..=1.60)
                    .text(copy.pet_size)
                    .suffix("x"),
            );
            ui.checkbox(&mut app.random_walking, copy.random_walk);
            ui.label(
                RichText::new(copy.random_walk_hint)
                    .small()
                    .color(TEXT_MUTED),
            );
            ui.label(
                RichText::new(copy.right_click_hint)
                    .small()
                    .color(TEXT_MUTED),
            );
            ui.add_space(10.0);
            if ui
                .add_enabled(
                    app.loaded.is_some(),
                    egui::Button::new(RichText::new(copy.open_desktop).strong()).fill(ACCENT_SOFT),
                )
                .clicked()
            {
                app.set_display_mode(DisplayMode::DesktopPet, ctx);
            }
        });

    ui.add_space(14.0);
    if let Some(message) = &app.feedback {
        Frame::new()
            .fill(Color32::from_rgb(24, 29, 37))
            .corner_radius(10.0)
            .inner_margin(Margin::same(14))
            .show(ui, |ui| {
                ui.label(RichText::new(message).small().color(TEXT_MUTED));
            });
    }
}

fn preview_panel(app: &mut PetApp, ui: &mut egui::Ui, ctx: &egui::Context) {
    let copy = app.copy();
    Frame::new()
        .fill(SURFACE)
        .stroke(Stroke::new(1.0_f32, SURFACE_RAISED))
        .corner_radius(12.0)
        .inner_margin(Margin::same(20))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    section_label(ui, copy.live_preview);
                    ui.label(RichText::new(copy.at_rest).size(22.0).strong());
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        RichText::new(localized_state_name(app.language, &app.selected_state))
                            .monospace()
                            .color(ACCENT),
                    );
                });
            });
            ui.add_space(18.0);

            let desired_height = 390.0;
            let (rect, _) = ui.allocate_exact_size(
                egui::vec2(ui.available_width(), desired_height),
                egui::Sense::hover(),
            );
            let painter = ui.painter_at(rect);
            painter.rect_filled(rect, 10.0, Color32::from_rgb(10, 12, 16));
            let inset = rect.shrink(16.0);
            painter.rect_stroke(
                inset,
                8.0,
                Stroke::new(1.0_f32, Color32::from_rgb(40, 45, 57)),
                egui::StrokeKind::Inside,
            );
            painter.line_segment(
                [
                    egui::pos2(inset.left() + 20.0, inset.bottom() - 30.0),
                    egui::pos2(inset.right() - 20.0, inset.bottom() - 30.0),
                ],
                Stroke::new(1.0_f32, ACCENT_SOFT),
            );

            if let Some(texture) = &app.texture {
                let image_size = egui::vec2(300.0, 325.0);
                let centered = rect.center()
                    + egui::vec2(0.0, -6.0)
                    + app.sprite_offset
                    + app.activity_motion();
                let image_rect = egui::Rect::from_center_size(centered, image_size);
                let drag_response = ui.interact(
                    image_rect,
                    ui.id().with("pet-preview-drag-handle"),
                    egui::Sense::drag(),
                );
                if drag_response.dragged() {
                    app.sprite_offset = clamp_sprite_offset(
                        app.sprite_offset + drag_response.drag_delta(),
                        inset,
                        image_size,
                    );
                }
                if drag_response.hovered() || drag_response.dragged() {
                    ui.ctx().set_cursor_icon(if drag_response.dragged() {
                        egui::CursorIcon::Grabbing
                    } else {
                        egui::CursorIcon::Grab
                    });
                }
                painter.image(
                    texture.id(),
                    image_rect,
                    egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)),
                    Color32::WHITE,
                );
                painter.text(
                    egui::pos2(inset.left() + 14.0, inset.top() + 14.0),
                    egui::Align2::LEFT_TOP,
                    copy.drag_preview,
                    egui::FontId::monospace(11.0),
                    TEXT_MUTED,
                );
            } else {
                painter.text(
                    rect.center() - egui::vec2(0.0, 8.0),
                    egui::Align2::CENTER_CENTER,
                    copy.empty_preview,
                    egui::FontId::proportional(18.0),
                    TEXT_MUTED,
                );
            }

            ui.add_space(18.0);
            ui.separator();
            ui.add_space(12.0);
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label(
                        RichText::new(copy.terminal_command)
                            .small()
                            .strong()
                            .color(TEXT_MUTED),
                    );
                    let command = app
                        .terminal_command()
                        .unwrap_or_else(|| copy.command_unavailable.to_string());
                    ui.label(RichText::new(&command).monospace().small().color(
                        if app.loaded.is_some() {
                            Color32::WHITE
                        } else {
                            TEXT_MUTED
                        },
                    ));
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui
                        .add_enabled(app.loaded.is_some(), egui::Button::new(copy.copy))
                        .clicked()
                        && let Some(command) = app.terminal_command()
                    {
                        ctx.copy_text(command);
                        app.feedback = Some(copy.copied.to_string());
                    }
                });
            });
        });
}

fn desktop_pet_view(app: &mut PetApp, ctx: &egui::Context) {
    egui::CentralPanel::default()
        .frame(Frame::NONE)
        .show(ctx, |ui| {
            let rect = ui.max_rect();
            if let Some(texture_id) = app.texture.as_ref().map(TextureHandle::id) {
                let image_rect = egui::Rect::from_center_size(
                    rect.center() + app.activity_motion(),
                    app.desktop_image_size(),
                );
                let response = ui.interact(
                    image_rect,
                    ui.id().with("desktop-pet-window-drag-handle"),
                    egui::Sense::hover(),
                );
                if response.hovered() || app.pointer_session.is_some() {
                    ui.ctx().set_cursor_icon(if app.pointer_session.is_some() {
                        egui::CursorIcon::Grabbing
                    } else {
                        egui::CursorIcon::Grab
                    });
                }
                ui.painter().image(
                    texture_id,
                    image_rect,
                    egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)),
                    Color32::WHITE,
                );
            }
        });
}

fn language_picker(app: &mut PetApp, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        for (language, label) in [
            (Language::English, "EN"),
            (Language::Chinese, "中"),
            (Language::Japanese, "日"),
        ] {
            if ui
                .selectable_label(app.language == language, RichText::new(label).small())
                .clicked()
            {
                app.language = language;
            }
        }
    });
}

fn walk_state_candidates(direction: HorizontalDirection) -> &'static [&'static str] {
    match direction {
        HorizontalDirection::Left => &["running-left", "running"],
        HorizontalDirection::Right => &["running-right", "running"],
    }
}

fn look_state_candidates(direction: LookDirection) -> &'static [&'static str] {
    match direction {
        LookDirection::Up => &["look-up"],
        LookDirection::UpRight => &["look-up-right"],
        LookDirection::Right => &["look-right"],
        LookDirection::DownRight => &["look-down-right"],
        LookDirection::Down => &["look-down"],
        LookDirection::DownLeft => &["look-down-left"],
        LookDirection::Left => &["look-left"],
        LookDirection::UpLeft => &["look-up-left"],
    }
}

fn desktop_geometry(ctx: &egui::Context, hit_rect: egui::Rect) -> Option<DesktopGeometry> {
    let pixels_per_point = ctx.pixels_per_point();
    let (inner_rect, outer_rect, monitor_size) = ctx.input(|input| {
        (
            input.viewport().inner_rect,
            input.viewport().outer_rect,
            input.viewport().monitor_size,
        )
    });
    let inner_rect = inner_rect?;
    let outer_rect = outer_rect?;
    let inner_position_screen = point_to_screen_pixels(inner_rect.min, pixels_per_point);
    let outer_rect_screen = rect_to_screen_pixels(outer_rect, pixels_per_point);
    let hit_rect_screen = if hit_rect == egui::Rect::NOTHING {
        egui::Rect::NOTHING
    } else {
        egui::Rect::from_min_max(
            inner_position_screen + hit_rect.min.to_vec2() * pixels_per_point,
            inner_position_screen + hit_rect.max.to_vec2() * pixels_per_point,
        )
    };
    let monitor_bounds_screen = monitor_size.and_then(|size| {
        let bounds = egui::Rect::from_min_size(egui::Pos2::ZERO, size * pixels_per_point);
        // eframe 0.31 exposes the size of current_monitor(), but not its global origin/work area.
        // Only use the bound when the reported global window coordinates prove it is the primary
        // monitor. On a negative or offset monitor, skipping the clamp is safer than teleporting.
        bounds
            .contains(outer_rect_screen.center())
            .then_some(bounds)
    });
    Some(DesktopGeometry {
        pixels_per_point,
        inner_position_screen,
        outer_rect_screen,
        hit_rect_screen,
        monitor_bounds_screen,
    })
}

fn point_to_screen_pixels(point: egui::Pos2, pixels_per_point: f32) -> egui::Pos2 {
    egui::pos2(point.x * pixels_per_point, point.y * pixels_per_point)
}

fn rect_to_screen_pixels(rect: egui::Rect, pixels_per_point: f32) -> egui::Rect {
    egui::Rect::from_min_max(
        point_to_screen_pixels(rect.min, pixels_per_point),
        point_to_screen_pixels(rect.max, pixels_per_point),
    )
}

fn screen_position(local_position: egui::Pos2, geometry: DesktopGeometry) -> egui::Pos2 {
    geometry.inner_position_screen + local_position.to_vec2() * geometry.pixels_per_point
}

fn send_outer_position_screen(
    ctx: &egui::Context,
    position_screen: egui::Pos2,
    pixels_per_point: f32,
) {
    ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(egui::pos2(
        position_screen.x / pixels_per_point,
        position_screen.y / pixels_per_point,
    )));
}

fn advance_pointer_session(
    mut session: PointerSession,
    current_position_screen: egui::Pos2,
    now: Duration,
    pixels_per_point: f32,
) -> (PointerSession, PointerMotion) {
    let delta_screen = current_position_screen - session.previous_position_screen;
    session.current_position_screen = current_position_screen;
    session.accumulated_distance += delta_screen.length();

    let elapsed = now.saturating_sub(session.last_sample_time);
    let elapsed_seconds = elapsed.as_secs_f32().max(0.001);
    let raw_velocity_x = delta_screen.x / elapsed_seconds;
    session.filtered_velocity_x = if session.last_sample_time == session.press_time {
        raw_velocity_x
    } else {
        0.65 * raw_velocity_x + 0.35 * session.filtered_velocity_x
    };
    session.last_sample_time = now;

    let effective_motion = delta_screen.length() >= EFFECTIVE_MOTION_POINTS * pixels_per_point;
    if effective_motion {
        session.last_effective_motion_time = now;
    }

    let press_distance = session
        .press_position_screen
        .distance(current_position_screen);
    let crossed_distance = press_distance >= DRAG_START_DISTANCE_POINTS * pixels_per_point;
    let crossed_speed = session.accumulated_distance
        >= DRAG_SPEED_DISTANCE_POINTS * pixels_per_point
        && session.filtered_velocity_x.abs()
            >= DRAG_SPEED_THRESHOLD_POINTS_PER_SECOND * pixels_per_point;
    session.drag_started |= crossed_distance || crossed_speed;

    let old_direction = session.last_horizontal_direction;
    if session.drag_started {
        let dead_zone = HORIZONTAL_DEAD_ZONE_POINTS * pixels_per_point;
        if let Some(candidate) = horizontal_direction_candidate(delta_screen, dead_zone) {
            if candidate == session.last_horizontal_direction {
                session.pending_direction = None;
                session.pending_direction_samples = 0;
            } else {
                let strong_reversal = delta_screen.x.abs() >= 2.0 * dead_zone;
                if session.pending_direction == Some(candidate) {
                    session.pending_direction_samples =
                        session.pending_direction_samples.saturating_add(1);
                } else {
                    session.pending_direction = Some(candidate);
                    session.pending_direction_since = now;
                    session.pending_direction_samples = 1;
                }
                if strong_reversal
                    || session.pending_direction_samples >= 2
                    || now.saturating_sub(session.pending_direction_since) >= DIRECTION_SWITCH_DELAY
                {
                    session.last_horizontal_direction = candidate;
                    session.pending_direction = None;
                    session.pending_direction_samples = 0;
                }
            }
        } else {
            session.pending_direction = None;
            session.pending_direction_samples = 0;
        }
    }

    // The next sample must always be compared with this sample, never the press position.
    session.previous_position_screen = current_position_screen;
    let target_window_position_screen = current_position_screen - session.grab_offset_screen;
    debug_assert!(
        target_window_position_screen.distance(
            session.press_window_position_screen
                + (current_position_screen - session.press_position_screen)
        ) < 0.01
    );
    let motion = PointerMotion {
        target_window_position_screen,
        direction: session.last_horizontal_direction,
        direction_changed: old_direction != session.last_horizontal_direction,
        drag_started: session.drag_started,
        effective_motion,
    };
    (session, motion)
}

fn horizontal_direction_candidate(
    delta_screen: egui::Vec2,
    dead_zone: f32,
) -> Option<HorizontalDirection> {
    (delta_screen.x.abs() > dead_zone && delta_screen.x.abs() >= delta_screen.y.abs()).then_some(
        if delta_screen.x < 0.0 {
            HorizontalDirection::Left
        } else {
            HorizontalDirection::Right
        },
    )
}

fn classify_pointer_completion(
    session: &PointerSession,
    release_time: Duration,
    pixels_per_point: f32,
) -> PointerCompletion {
    if session.drag_started {
        return PointerCompletion::DragEnd;
    }
    let short_press = release_time.saturating_sub(session.press_time) <= CLICK_MAX_DURATION;
    let small_movement = session
        .press_position_screen
        .distance(session.current_position_screen)
        < DRAG_START_DISTANCE_POINTS * pixels_per_point;
    if short_press && small_movement {
        PointerCompletion::Click
    } else {
        PointerCompletion::Cancel
    }
}

fn clamp_window_position_screen(
    desired_position: egui::Pos2,
    window_size: egui::Vec2,
    work_area: egui::Rect,
) -> egui::Pos2 {
    let max_x = (work_area.right() - window_size.x).max(work_area.left());
    let max_y = (work_area.bottom() - window_size.y).max(work_area.top());
    egui::pos2(
        desired_position.x.clamp(work_area.left(), max_x),
        desired_position.y.clamp(work_area.top(), max_y),
    )
}

fn quantize_look_direction(
    offset_screen: egui::Vec2,
    center_dead_zone: f32,
    previous: Option<LookDirection>,
) -> Option<LookDirection> {
    if offset_screen.length() <= center_dead_zone {
        return None;
    }
    let angle = offset_screen.y.atan2(offset_screen.x);
    if previous.is_some_and(|direction| {
        angular_distance(angle, direction.center_angle())
            <= std::f32::consts::FRAC_PI_8 + HOVER_DIRECTION_HYSTERESIS_RADIANS
    }) {
        return previous;
    }
    let sector = (angle / std::f32::consts::FRAC_PI_4).round() as i32;
    Some(match sector.rem_euclid(8) {
        0 => LookDirection::Right,
        1 => LookDirection::DownRight,
        2 => LookDirection::Down,
        3 => LookDirection::DownLeft,
        4 => LookDirection::Left,
        5 => LookDirection::UpLeft,
        6 => LookDirection::Up,
        _ => LookDirection::UpRight,
    })
}

fn angular_distance(a: f32, b: f32) -> f32 {
    let tau = 2.0 * std::f32::consts::PI;
    ((a - b + std::f32::consts::PI).rem_euclid(tau) - std::f32::consts::PI).abs()
}

fn choose_click_reaction(
    seed: u64,
    previous: Option<ClickReaction>,
    repeated_count: u8,
) -> (ClickReaction, u8) {
    let random_choice = if seed & 1 == 0 {
        ClickReaction::Play
    } else {
        ClickReaction::Wave
    };
    let choice = if previous == Some(random_choice) && repeated_count >= 2 {
        match random_choice {
            ClickReaction::Play => ClickReaction::Wave,
            ClickReaction::Wave => ClickReaction::Play,
        }
    } else {
        random_choice
    };
    let next_count = if previous == Some(choice) {
        repeated_count.saturating_add(1)
    } else {
        1
    };
    (choice, next_count)
}

fn should_open_settings(
    right_pressed: bool,
    pointer_session_active: bool,
    pointer_hits_pet: bool,
) -> bool {
    right_pressed && (pointer_session_active || pointer_hits_pet)
}

fn auto_walk_allowed(
    enabled: bool,
    pointer_session_active: bool,
    hovered: bool,
    state: PetState,
) -> bool {
    enabled
        && !pointer_session_active
        && !hovered
        && matches!(state, PetState::Idle | PetState::AutoWalking(_))
}

fn drag_animation_is_active(now: Duration, last_effective_motion: Duration) -> bool {
    now.saturating_sub(last_effective_motion) < DRAG_STILL_DELAY
}

fn random_seed() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|elapsed| elapsed.as_nanos() as u64)
        .unwrap_or_default()
}

fn localized_state_name(language: Language, state: &str) -> String {
    let label = match (language, state) {
        (Language::Chinese, "idle") => "待机",
        (Language::Chinese, "jumping") => "跳跃",
        (Language::Chinese, "waving") => "挥手",
        (Language::Chinese, "running") => "奔跑",
        (Language::Chinese, "running-left") => "向左跑",
        (Language::Chinese, "running-right") => "向右跑",
        (Language::Chinese, "waiting") => "等待",
        (Language::Chinese, "review") => "查看",
        (Language::Chinese, "failed") => "失败",
        (Language::Japanese, "idle") => "待機",
        (Language::Japanese, "jumping") => "ジャンプ",
        (Language::Japanese, "waving") => "手を振る",
        (Language::Japanese, "running") => "走る",
        (Language::Japanese, "running-left") => "左へ走る",
        (Language::Japanese, "running-right") => "右へ走る",
        (Language::Japanese, "waiting") => "待つ",
        (Language::Japanese, "review") => "確認",
        (Language::Japanese, "failed") => "失敗",
        _ => state,
    };
    label.to_string()
}

fn configure_theme(ctx: &egui::Context) {
    configure_cjk_font_fallback(ctx);

    let mut visuals = egui::Visuals::dark();
    visuals.panel_fill = CANVAS;
    visuals.window_fill = SURFACE;
    visuals.extreme_bg_color = CANVAS;
    visuals.faint_bg_color = SURFACE_RAISED;
    visuals.widgets.noninteractive.bg_fill = SURFACE;
    visuals.widgets.inactive.bg_fill = SURFACE_RAISED;
    visuals.widgets.hovered.bg_fill = Color32::from_rgb(53, 52, 83);
    visuals.widgets.active.bg_fill = ACCENT_SOFT;
    visuals.selection.bg_fill = ACCENT_SOFT;
    visuals.selection.stroke = Stroke::new(1.0_f32, ACCENT);
    ctx.set_visuals(visuals);

    let mut style = (*ctx.style()).clone();
    style.spacing.item_spacing = egui::vec2(8.0, 8.0);
    style.spacing.button_padding = egui::vec2(10.0, 7.0);
    ctx.set_style(style);
}

fn configure_cjk_font_fallback(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    let mut fallback_names = Vec::new();

    for (name, path) in [
        ("noto-sans-sc", r"C:\Windows\Fonts\NotoSansSC-VF.ttf"),
        ("noto-sans-jp", r"C:\Windows\Fonts\NotoSansJP-VF.ttf"),
    ] {
        if let Ok(bytes) = fs::read(path) {
            fonts
                .font_data
                .insert(name.to_string(), egui::FontData::from_owned(bytes).into());
            fallback_names.push(name.to_string());
        }
    }

    for family in [egui::FontFamily::Proportional, egui::FontFamily::Monospace] {
        if let Some(chain) = fonts.families.get_mut(&family) {
            chain.extend(fallback_names.iter().cloned());
        }
    }
    ctx.set_fonts(fonts);
}

fn section_label(ui: &mut egui::Ui, text: &str) {
    ui.label(RichText::new(text).small().strong().color(ACCENT));
}

fn status_badge(ui: &mut egui::Ui, text: &str, ready: bool) {
    let color = if ready { SUCCESS } else { TEXT_MUTED };
    Frame::new()
        .fill(Color32::from_rgba_unmultiplied(
            color.r(),
            color.g(),
            color.b(),
            26,
        ))
        .stroke(Stroke::new(
            1.0_f32,
            Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), 86),
        ))
        .corner_radius(999.0)
        .inner_margin(Margin::symmetric(9, 5))
        .show(ui, |ui| {
            ui.label(RichText::new(text).small().strong().color(color));
        });
}

fn clamp_sprite_offset(
    offset: egui::Vec2,
    bounds: egui::Rect,
    image_size: egui::Vec2,
) -> egui::Vec2 {
    let horizontal_limit = ((bounds.width() - image_size.x) / 2.0).max(0.0);
    let vertical_limit = ((bounds.height() - image_size.y) / 2.0).max(0.0);
    egui::vec2(
        offset.x.clamp(-horizontal_limit, horizontal_limit),
        offset.y.clamp(-vertical_limit, vertical_limit),
    )
}

fn scaled_pet_size(base_size: egui::Vec2, scale: f32) -> egui::Vec2 {
    base_size * scale
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pointer_session(direction: HorizontalDirection) -> PointerSession {
        PointerSession::new(
            egui::pos2(100.0, 100.0),
            egui::pos2(40.0, 30.0),
            Duration::ZERO,
            direction,
        )
    }

    fn advance(
        session: PointerSession,
        x: f32,
        y: f32,
        millis: u64,
    ) -> (PointerSession, PointerMotion) {
        advance_pointer_session(
            session,
            egui::pos2(x, y),
            Duration::from_millis(millis),
            1.0,
        )
    }

    #[test]
    fn keeps_a_dragged_pet_inside_the_preview_bounds() {
        let bounds = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(400.0, 400.0));
        let offset =
            clamp_sprite_offset(egui::vec2(250.0, -250.0), bounds, egui::vec2(300.0, 325.0));

        assert_eq!(offset, egui::vec2(50.0, -37.5));
    }

    #[test]
    fn ships_all_three_ui_languages() {
        assert_eq!(Language::English.copy().open_desktop, "Desktop pet mode");
        assert_eq!(Language::Chinese.copy().open_desktop, "进入桌宠模式");
        assert_eq!(
            Language::Japanese.copy().open_desktop,
            "デスクトップペットにする"
        );
    }

    #[test]
    fn drag_directions_choose_matching_walk_states() {
        assert_eq!(
            walk_state_candidates(HorizontalDirection::Left),
            ["running-left", "running"]
        );
        assert_eq!(
            walk_state_candidates(HorizontalDirection::Right),
            ["running-right", "running"]
        );
    }

    #[test]
    fn one_drag_reverses_from_right_to_left_immediately() {
        let (session, motion) = advance(
            pointer_session(HorizontalDirection::Right),
            112.0,
            100.0,
            16,
        );
        assert_eq!(motion.direction, HorizontalDirection::Right);

        let (_, reversed) = advance(session, 102.0, 100.0, 32);
        assert_eq!(reversed.direction, HorizontalDirection::Left);
        assert!(reversed.direction_changed);
    }

    #[test]
    fn one_drag_reverses_from_left_to_right_immediately() {
        let (session, motion) =
            advance(pointer_session(HorizontalDirection::Left), 88.0, 100.0, 16);
        assert_eq!(motion.direction, HorizontalDirection::Left);

        let (_, reversed) = advance(session, 98.0, 100.0, 32);
        assert_eq!(reversed.direction, HorizontalDirection::Right);
        assert!(reversed.direction_changed);
    }

    #[test]
    fn weak_reversal_is_confirmed_on_the_second_sample() {
        let (session, _) = advance(
            pointer_session(HorizontalDirection::Right),
            112.0,
            100.0,
            16,
        );
        let (session, first) = advance(session, 108.0, 100.0, 32);
        assert_eq!(first.direction, HorizontalDirection::Right);

        let (_, second) = advance(session, 104.0, 100.0, 48);
        assert_eq!(second.direction, HorizontalDirection::Left);
    }

    #[test]
    fn micro_jitter_does_not_flip_direction() {
        let (session, _) = advance(
            pointer_session(HorizontalDirection::Right),
            112.0,
            100.0,
            16,
        );
        let (session, first) = advance(session, 110.0, 100.5, 32);
        let (_, second) = advance(session, 112.0, 99.5, 48);

        assert_eq!(first.direction, HorizontalDirection::Right);
        assert_eq!(second.direction, HorizontalDirection::Right);
    }

    #[test]
    fn mostly_vertical_drag_keeps_the_last_direction() {
        let (session, _) = advance(pointer_session(HorizontalDirection::Left), 94.0, 100.0, 16);
        let (_, motion) = advance(session, 96.0, 116.0, 32);

        assert_eq!(motion.direction, HorizontalDirection::Left);
    }

    #[test]
    fn small_short_movement_is_a_click() {
        let (session, _) = advance(
            pointer_session(HorizontalDirection::Right),
            103.0,
            102.0,
            100,
        );

        assert_eq!(
            classify_pointer_completion(&session, Duration::from_millis(120), 1.0,),
            PointerCompletion::Click
        );
    }

    #[test]
    fn crossing_drag_threshold_never_becomes_a_click() {
        let (session, motion) = advance(
            pointer_session(HorizontalDirection::Right),
            106.0,
            100.0,
            100,
        );
        assert!(motion.drag_started);
        assert_eq!(
            classify_pointer_completion(&session, Duration::from_millis(120), 1.0,),
            PointerCompletion::DragEnd
        );
    }

    #[test]
    fn right_click_has_priority_over_an_active_drag() {
        assert!(should_open_settings(true, true, false));
        assert!(should_open_settings(true, false, true));
        assert!(!should_open_settings(false, true, true));
    }

    #[test]
    fn user_interaction_disables_auto_walk_immediately() {
        assert!(auto_walk_allowed(true, false, false, PetState::Idle));
        assert!(!auto_walk_allowed(
            true,
            true,
            false,
            PetState::AutoWalking(HorizontalDirection::Right)
        ));
        assert!(!auto_walk_allowed(true, false, true, PetState::Idle));
    }

    #[test]
    fn dpi_and_negative_monitor_bounds_are_calculated_in_screen_pixels() {
        assert_eq!(
            point_to_screen_pixels(egui::pos2(-640.0, 120.0), 1.5),
            egui::pos2(-960.0, 180.0)
        );
        let work_area =
            egui::Rect::from_min_size(egui::pos2(-1920.0, -120.0), egui::vec2(1920.0, 1080.0));
        assert_eq!(
            clamp_window_position_screen(
                egui::pos2(-2_100.0, 900.0),
                egui::vec2(360.0, 440.0),
                work_area,
            ),
            egui::pos2(-1920.0, 520.0)
        );
    }

    #[test]
    fn scaled_pet_size_updates_the_hit_region_dimensions() {
        let base = egui::vec2(300.0, 325.0);
        let small = scaled_pet_size(base, 0.6);
        let large = scaled_pet_size(base, 1.6);
        assert!((small.x - 180.0).abs() < 0.001 && (small.y - 195.0).abs() < 0.001);
        assert!((large.x - 480.0).abs() < 0.001 && (large.y - 520.0).abs() < 0.001);
    }

    #[test]
    fn drag_target_preserves_the_original_grab_offset() {
        let session = pointer_session(HorizontalDirection::Right);
        let (_, motion) = advance(session, 130.0, 145.0, 16);

        assert_eq!(motion.target_window_position_screen, egui::pos2(70.0, 75.0));
    }

    #[test]
    fn click_reaction_never_repeats_more_than_twice() {
        let (choice, count) = choose_click_reaction(0, Some(ClickReaction::Play), 2);
        assert_eq!(choice, ClickReaction::Wave);
        assert_eq!(count, 1);
    }

    #[test]
    fn drag_animation_stops_after_eighty_milliseconds_without_motion() {
        let last_motion = Duration::from_millis(20);
        assert!(drag_animation_is_active(
            Duration::from_millis(99),
            last_motion
        ));
        assert!(!drag_animation_is_active(
            Duration::from_millis(100),
            last_motion
        ));
    }
}
