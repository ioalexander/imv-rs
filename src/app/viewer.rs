use eframe::egui;
use image::AnimationDecoder;
use std::io::BufReader;
use std::time::{Duration, Instant};

pub struct ImageViewer {
    // static images
    texture: Option<egui::TextureHandle>,
    // gifs
    gif_frames: Vec<egui::TextureHandle>,
    current_frame: usize,
    last_frame_update: Instant,
    frame_delay: Duration,
}

impl ImageViewer {
    pub fn new(texture: Option<egui::TextureHandle>) -> Self {
        Self {
            texture,
            gif_frames: Vec::new(),
            current_frame: 0,
            last_frame_update: Instant::now(),
            frame_delay: Duration::from_millis(100),
        }
    }

    pub fn load_animated_gif(path: &str, ctx: &egui::Context) -> Option<Self> {
        let file = match std::fs::File::open(path) {
            Ok(file) => file,
            Err(_) => return None,
        };

        let buf_reader = BufReader::new(file);
        let decoder = match image::codecs::gif::GifDecoder::new(buf_reader) {
            Ok(decoder) => decoder,
            Err(_) => return None,
        };

        let frames = decoder.into_frames();

        let mut textures = Vec::new();
        let mut frame_delay = Duration::from_millis(100);

        for frame_result in frames {
            let frame = match frame_result {
                Ok(frame) => frame,
                Err(_) => continue,
            };

            if textures.is_empty() {
                frame_delay = Duration::from_millis(frame.delay().numer_denom_ms().0 as u64);
            }

            let buffer = frame.into_buffer();
            let image_data = egui::ColorImage::from_rgba_unmultiplied(
                [buffer.width() as usize, buffer.height() as usize],
                buffer.as_flat_samples().as_slice(),
            );

            let texture = ctx.load_texture(path, image_data, egui::TextureOptions::LINEAR);
            textures.push(texture);
        }

        if textures.is_empty() {
            return None;
        }

        Some(Self {
            texture: None,
            gif_frames: textures,
            current_frame: 0,
            last_frame_update: Instant::now(),
            frame_delay,
        })
    }

    fn calculate_display_size(
        &self,
        available_size: egui::Vec2,
        texture_size: egui::Vec2,
    ) -> egui::Vec2 {
        let available_ratio = available_size.x / available_size.y;
        let image_ratio = texture_size.x / texture_size.y;

        if image_ratio > available_ratio {
            egui::vec2(available_size.x, available_size.x / image_ratio)
        } else {
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
        // gif logic
        if !self.gif_frames.is_empty() && self.last_frame_update.elapsed() >= self.frame_delay {
            self.current_frame = (self.current_frame + 1) % self.gif_frames.len();
            self.last_frame_update = Instant::now();
            ctx.request_repaint();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            if !self.gif_frames.is_empty() {
                let texture = &self.gif_frames[self.current_frame];
                self.render_image(ui, texture);
            } else if let Some(texture) = &self.texture {
                self.render_image(ui, texture);
            } else {
                let _ = ui.heading("Failed to load image");
            }
        });
    }
}
