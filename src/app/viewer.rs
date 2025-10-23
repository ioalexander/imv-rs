use eframe::egui;
use std::path::PathBuf;

use crate::app::{image_state::ImageState, navigation::Navigation};

pub struct ImageViewer {
    image_state: ImageState,
    navigation: Navigation,
    pending_navigation: Option<isize>,
}

impl ImageViewer {
    pub fn new_from_path(path: &str, ctx: &egui::Context) -> Self {
        log::info!("Creating ImageViewer for path: {}", path);
        let current_path = PathBuf::from(path);

        let supported_extensions = ["jpg", "jpeg", "png", "gif", "webp", "bmp", "ico", "tiff"];

        let start_path = if current_path.is_file() {
            current_path.clone()
        } else if current_path.is_dir() {
            let mut first_image: Option<PathBuf> = None;
            if let Ok(entries) = std::fs::read_dir(&current_path) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                            if supported_extensions.contains(&ext.to_lowercase().as_str()) {
                                first_image = Some(path);
                                break;
                            }
                        }
                    }
                }
            }
            first_image.unwrap_or(current_path.clone())
        } else {
            current_path.clone()
        };

        let navigation = Navigation::from_path(&start_path);

        let mut image_state = ImageState::new(
            navigation
                .current_path()
                .cloned()
                .unwrap_or_else(|| start_path.clone()),
        );

        if let Some(p) = navigation.current_path() {
            image_state.load_from_path(p, ctx);
        }

        Self {
            image_state,
            navigation,
            pending_navigation: None,
        }
    }

    fn process_pending_navigation(&mut self, ctx: &egui::Context) {
        if self.pending_navigation.is_some() {
            if let Some(p) = self.navigation.current_path() {
                self.image_state.load_from_path(p, ctx);
            }
            self.pending_navigation = None;
        }
    }

    fn navigate_previous(&mut self) {
        if self.navigation.is_empty() {
            return;
        }
        self.navigation.navigate_previous();
        self.pending_navigation = Some(-1);
    }

    fn navigate_next(&mut self) {
        if self.navigation.is_empty() {
            return;
        }
        self.navigation.navigate_next();
        self.pending_navigation = Some(1);
    }
}

impl eframe::App for ImageViewer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.process_pending_navigation(ctx);

        ctx.input(|i| {
            if i.key_pressed(egui::Key::ArrowLeft) || i.key_pressed(egui::Key::H) {
                self.navigate_previous();
            }
            if i.key_pressed(egui::Key::ArrowRight) || i.key_pressed(egui::Key::L) {
                self.navigate_next();
            }
        });

        // gif
        if !self.image_state.gif_frames.is_empty() {
            if self.image_state.advance_frame_if_needed() {
                ctx.request_repaint();
            } else {
                let remaining = self
                    .image_state
                    .frame_delay
                    .saturating_sub(self.image_state.last_frame_update.elapsed());
                if remaining == std::time::Duration::ZERO {
                    ctx.request_repaint();
                } else {
                    ctx.request_repaint_after(remaining);
                }
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            if !self.image_state.gif_frames.is_empty() {
                let texture = &self.image_state.gif_frames[self.image_state.current_frame];
                self.image_state.render_image(ui, texture);
            } else if let Some(texture) = &self.image_state.texture {
                self.image_state.render_image(ui, texture);
            } else {
                let _ = ui.heading("Failed to load image");
            }
        });
    }
}
