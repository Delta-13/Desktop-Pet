use std::path::PathBuf;
use std::time::Duration;
use std::time::Instant;

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

pub fn run() -> Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1060.0, 720.0])
            .with_min_inner_size([820.0, 560.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Terminal Sprite Pet",
        options,
        Box::new(|creation_context| Ok(Box::new(PetApp::new(creation_context)))),
    )
    .map_err(|error| anyhow::anyhow!(error.to_string()))
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
    sprite_offset: egui::Vec2,
    texture: Option<TextureHandle>,
    feedback: Option<String>,
}

impl PetApp {
    fn new(creation_context: &eframe::CreationContext<'_>) -> Self {
        configure_theme(&creation_context.egui_ctx);
        Self {
            loaded: None,
            path_input: String::new(),
            selected_state: "idle".to_string(),
            animation_started: Instant::now(),
            sprite_offset: egui::Vec2::ZERO,
            texture: None,
            feedback: None,
        }
    }

    fn choose_pet(&mut self, ctx: &egui::Context) {
        if let Some(path) = rfd::FileDialog::new()
            .set_title("Choose a Codex pet folder")
            .pick_folder()
        {
            self.path_input = path.display().to_string();
            self.load_pet(path, ctx);
        }
    }

    fn load_path_input(&mut self, ctx: &egui::Context) {
        let path = PathBuf::from(self.path_input.trim());
        if path.as_os_str().is_empty() {
            self.feedback = Some("Enter a folder that contains pet.json first.".to_string());
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
                self.sprite_offset = egui::Vec2::ZERO;
                self.texture = None;
                self.feedback = Some(
                    "Pet loaded. Pick a state to preview it, then drag it around the canvas."
                        .to_string(),
                );
                self.refresh_preview(ctx);
            }
            Err(error) => {
                self.feedback = Some(format!("Could not load pet: {error:#}"));
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
        ctx.request_repaint_after(next_delay.unwrap_or(Duration::from_millis(250)));
    }

    fn select_state(&mut self, state: String, ctx: &egui::Context) {
        self.selected_state = state;
        self.animation_started = Instant::now();
        self.refresh_preview(ctx);
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
}

impl eframe::App for PetApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.refresh_preview(ctx);

        egui::TopBottomPanel::top("topbar")
            .frame(
                Frame::new()
                    .fill(CANVAS)
                    .inner_margin(Margin::symmetric(26, 16)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("✦").size(24.0).color(ACCENT));
                    ui.vertical(|ui| {
                        ui.label(RichText::new("terminal / sprite pet").size(18.0).strong());
                        ui.label(
                            RichText::new("A focused control room for Codex-compatible pets")
                                .small()
                                .color(TEXT_MUTED),
                        );
                    });
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let ready = self.loaded.is_some();
                        status_badge(
                            ui,
                            if ready {
                                "PREVIEW READY"
                            } else {
                                "AWAITING PET"
                            },
                            ready,
                        );
                    });
                });
            });

        egui::CentralPanel::default()
            .frame(Frame::new().fill(CANVAS).inner_margin(Margin::same(26)))
            .show(ctx, |ui| {
                ui.columns(2, |columns| {
                    columns[0].set_min_width(374.0);
                    setup_panel(self, &mut columns[0], ctx);
                    preview_panel(self, &mut columns[1], ctx);
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
                    ui.label(RichText::new("LOCAL ONLY").small().strong().color(SUCCESS));
                    ui.label(
                        RichText::new("No account, sync, or pet data leaves this device.")
                            .small()
                            .color(TEXT_MUTED),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(
                            RichText::new("Codex-compatible · desktop preview")
                                .small()
                                .color(TEXT_MUTED),
                        );
                    });
                });
            });
    }
}

fn setup_panel(app: &mut PetApp, ui: &mut egui::Ui, ctx: &egui::Context) {
    Frame::new()
        .fill(SURFACE)
        .stroke(Stroke::new(1.0_f32, SURFACE_RAISED))
        .corner_radius(12.0)
        .inner_margin(Margin::same(20))
        .show(ui, |ui| {
            section_label(ui, "01  /  PET PACKAGE");
            ui.add_space(8.0);
            ui.label(RichText::new("Pick a local pet").size(22.0).strong());
            ui.label(
                RichText::new("Load any folder containing pet.json and its spritesheet.")
                    .color(TEXT_MUTED),
            );
            ui.add_space(16.0);

            let button = egui::Button::new(RichText::new("Choose pet folder").strong())
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
                    egui::TextEdit::singleline(&mut app.path_input)
                        .hint_text("Paste a pet folder path"),
                );
                if ui.button("Load").clicked() {
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
            section_label(ui, "02  /  ANIMATION STATE");
            ui.add_space(8.0);
            ui.label(RichText::new("Set the mood").size(22.0).strong());
            ui.label(
                RichText::new("The preview uses the animation map from this pet package.")
                    .color(TEXT_MUTED),
            );
            ui.add_space(14.0);

            let states = app.state_names();
            if states.is_empty() {
                ui.label(
                    RichText::new("Load a pet to unlock its states.")
                        .italics()
                        .color(TEXT_MUTED),
                );
            } else {
                ui.horizontal_wrapped(|ui| {
                    for state in states {
                        let active = state == app.selected_state;
                        let button = egui::Button::new(&state)
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
    Frame::new()
        .fill(SURFACE)
        .stroke(Stroke::new(1.0_f32, SURFACE_RAISED))
        .corner_radius(12.0)
        .inner_margin(Margin::same(20))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    section_label(ui, "LIVE PREVIEW");
                    ui.label(RichText::new("Your pet, at rest").size(22.0).strong());
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        RichText::new(if app.loaded.is_some() {
                            &app.selected_state
                        } else {
                            "idle"
                        })
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
                let centered = rect.center() + egui::vec2(0.0, -6.0) + app.sprite_offset;
                let image_rect = egui::Rect::from_center_size(centered, image_size);
                let drag_response = ui.interact(
                    image_rect,
                    ui.id().with("pet-drag-handle"),
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
                    "DRAG PET TO REPOSITION",
                    egui::FontId::monospace(11.0),
                    TEXT_MUTED,
                );
            } else {
                painter.text(
                    rect.center() - egui::vec2(0.0, 8.0),
                    egui::Align2::CENTER_CENTER,
                    "Pick a pet package\nto bring this space to life",
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
                        RichText::new("TERMINAL COMMAND")
                            .small()
                            .strong()
                            .color(TEXT_MUTED),
                    );
                    let command = app
                        .terminal_command()
                        .unwrap_or_else(|| "Load a pet to generate a command".to_string());
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
                        .add_enabled(app.loaded.is_some(), egui::Button::new("Copy"))
                        .clicked()
                        && let Some(command) = app.terminal_command()
                    {
                        ctx.copy_text(command);
                        app.feedback =
                            Some("Terminal command copied. It has not been launched.".to_string());
                    }
                });
            });
        });
}

fn configure_theme(ctx: &egui::Context) {
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
}
