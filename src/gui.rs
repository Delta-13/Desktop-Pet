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
enum Activity {
    Rest,
    Play,
    Wave,
    Walk,
    Custom,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum WalkDirection {
    Left,
    Right,
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
    activity: Activity,
    walk_direction: WalkDirection,
    pet_scale: f32,
    random_walking: bool,
    last_interaction: Instant,
    next_random_walk_at: Instant,
    random_walk_until: Option<Instant>,
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
            activity: Activity::Rest,
            walk_direction: WalkDirection::Right,
            pet_scale: 1.0,
            random_walking: false,
            last_interaction: now,
            next_random_walk_at: now + RANDOM_WALK_DELAY,
            random_walk_until: None,
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
                self.activity = Activity::Rest;
                self.walk_direction = WalkDirection::Right;
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
        let repaint = if matches!(self.activity, Activity::Play | Activity::Walk) {
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
        self.activity = Activity::Custom;
        self.record_interaction();
        self.refresh_preview(ctx);
    }

    fn start_activity(&mut self, activity: Activity, ctx: &egui::Context) {
        if self.apply_activity(activity, ctx) {
            self.record_interaction();
        }
    }

    fn apply_activity(&mut self, activity: Activity, ctx: &egui::Context) -> bool {
        let candidates = match activity {
            Activity::Rest => &["idle", "waiting"][..],
            Activity::Play => &["jumping", "waving", "review"][..],
            Activity::Wave => &["waving", "review"][..],
            Activity::Walk => &["running", "running-right", "running-left"][..],
            Activity::Custom => &[][..],
        };
        let next = self.loaded.as_ref().and_then(|loaded| {
            candidates
                .iter()
                .find(|name| loaded.pet.animations.contains_key(**name))
                .map(|name| (*name).to_string())
        });
        if let Some(next) = next {
            self.selected_state = next;
            self.activity = activity;
            self.animation_started = Instant::now();
            self.activity_started = Instant::now();
            self.refresh_preview(ctx);
            true
        } else if self.loaded.is_some() {
            self.feedback = Some(self.copy().activity_unavailable.to_string());
            false
        } else {
            false
        }
    }

    fn trigger_tap(&mut self, ctx: &egui::Context) {
        let activity = if random_seed() & 1 == 0 {
            Activity::Play
        } else {
            Activity::Wave
        };
        self.start_activity(activity, ctx);
    }

    fn walk_with_drag(&mut self, drag_delta: egui::Vec2, ctx: &egui::Context) {
        if drag_delta.x.abs() < 2.0 || drag_delta.x.abs() < drag_delta.y.abs() {
            self.record_interaction();
            return;
        }
        let direction = if drag_delta.x.is_sign_negative() {
            WalkDirection::Left
        } else {
            WalkDirection::Right
        };
        self.start_walk(direction, ctx);
        self.record_interaction();
    }

    fn start_walk(&mut self, direction: WalkDirection, ctx: &egui::Context) {
        let candidates = walk_state_candidates(direction);
        let next = self.loaded.as_ref().and_then(|loaded| {
            candidates
                .iter()
                .find(|name| loaded.pet.animations.contains_key(**name))
                .map(|name| (*name).to_string())
        });
        if let Some(next) = next {
            let changed = self.activity != Activity::Walk
                || self.walk_direction != direction
                || self.selected_state != next;
            self.activity = Activity::Walk;
            self.walk_direction = direction;
            if changed {
                self.selected_state = next;
                self.animation_started = Instant::now();
                self.activity_started = Instant::now();
            }
            self.refresh_preview(ctx);
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
        match self.activity {
            Activity::Play => egui::vec2(0.0, -(seconds * 7.0).sin().max(0.0) * 18.0),
            _ => egui::Vec2::ZERO,
        }
    }

    fn desktop_image_size(&self) -> egui::Vec2 {
        egui::vec2(300.0, 325.0) * self.pet_scale
    }

    fn desktop_window_size(&self) -> egui::Vec2 {
        (self.desktop_image_size() + egui::vec2(42.0, 44.0)).max(egui::vec2(220.0, 240.0))
    }

    fn update_desktop_behavior(&mut self, ctx: &egui::Context) {
        if self.mode != DisplayMode::DesktopPet || self.loaded.is_none() {
            return;
        }

        let now = Instant::now();
        if self.random_walking
            && self.random_walk_until.is_none()
            && now >= self.next_random_walk_at
        {
            let direction = if random_seed() & 1 == 0 {
                WalkDirection::Left
            } else {
                WalkDirection::Right
            };
            self.start_walk(direction, ctx);
            self.random_walk_until = Some(now + RANDOM_WALK_DURATION);
            self.next_random_walk_at =
                now + RANDOM_WALK_DELAY + Duration::from_millis(1_000 + random_seed() % 2_000);
        }

        if let Some(until) = self.random_walk_until {
            if now < until {
                self.move_window_for_random_walk(ctx);
                ctx.request_repaint_after(Duration::from_millis(16));
            } else {
                self.random_walk_until = None;
                let _ = self.apply_activity(Activity::Rest, ctx);
            }
        } else if self.last_interaction.elapsed() >= REST_AFTER_IDLE
            && self.activity != Activity::Rest
        {
            let _ = self.apply_activity(Activity::Rest, ctx);
        }
    }

    fn move_window_for_random_walk(&self, ctx: &egui::Context) {
        let (outer_rect, monitor_size) =
            ctx.input(|input| (input.viewport().outer_rect, input.viewport().monitor_size));
        let (Some(outer_rect), Some(monitor_size)) = (outer_rect, monitor_size) else {
            return;
        };
        let step = match self.walk_direction {
            WalkDirection::Left => -1.5,
            WalkDirection::Right => 1.5,
        };
        let max_x = (monitor_size.x - outer_rect.width()).max(0.0);
        let x = (outer_rect.left() + step).clamp(0.0, max_x);
        ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(egui::pos2(
            x,
            outer_rect.top(),
        )));
    }

    fn set_display_mode(&mut self, mode: DisplayMode, ctx: &egui::Context) {
        if self.mode == mode {
            return;
        }
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
            self.update_desktop_behavior(ctx);
        }
        self.refresh_preview(ctx);
        match self.mode {
            DisplayMode::Controller => controller_view(self, ctx),
            DisplayMode::DesktopPet => desktop_pet_view(self, ctx),
        }
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
                    egui::Sense::click_and_drag(),
                );
                if response.secondary_clicked() {
                    app.set_display_mode(DisplayMode::Controller, ctx);
                } else if response.drag_started() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
                }
                if response.dragged() {
                    app.walk_with_drag(response.drag_delta(), ctx);
                } else if response.clicked() {
                    app.trigger_tap(ctx);
                }
                if response.hovered() || response.dragged() {
                    ui.ctx().set_cursor_icon(if response.dragged() {
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

fn walk_state_candidates(direction: WalkDirection) -> &'static [&'static str] {
    match direction {
        WalkDirection::Left => &["running-left", "running"],
        WalkDirection::Right => &["running-right", "running"],
    }
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

#[cfg(test)]
mod tests {
    use super::*;

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
            walk_state_candidates(WalkDirection::Left),
            ["running-left", "running"]
        );
        assert_eq!(
            walk_state_candidates(WalkDirection::Right),
            ["running-right", "running"]
        );
    }
}
