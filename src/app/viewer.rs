use eframe::egui;

pub struct ImageViewer {
    texture: Option<egui::TextureHandle>,
}

impl ImageViewer {
    pub fn new(texture: Option<egui::TextureHandle>) -> Self {
        Self { texture }
    }

    fn calculate_display_size(
        &self,
        available_size: egui::Vec2,
        texture_size: egui::Vec2,
    ) -> egui::Vec2 {
        let available_ratio = available_size.x / available_size.y;
        let image_ratio = texture_size.x / texture_size.y;

        if image_ratio > available_ratio {
            // Image is wider than available space
            egui::vec2(available_size.x, available_size.x / image_ratio)
        } else {
            // Image is taller than available space
            egui::vec2(available_size.y * image_ratio, available_size.y)
        }
    }

    fn render_image(&self, ui: &mut egui::Ui, texture: &egui::TextureHandle) {
        let available_size = ui.available_size();
        let texture_size = texture.size_vec2();
        let display_size = self.calculate_display_size(available_size, texture_size);
        let pos = ui.available_rect_before_wrap().center() - display_size * 0.5;

        let image_widget = egui::Image::new(texture)
            .fit_to_exact_size(display_size)
            .maintain_aspect_ratio(true);

        ui.put(egui::Rect::from_min_size(pos, display_size), image_widget);
    }
}

impl eframe::App for ImageViewer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(texture) = &self.texture {
                self.render_image(ui, texture);
            } else {
                let _ = ui.heading("Failed to load image");
            }
        });
    }
}
