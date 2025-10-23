use eframe::egui;
use image::AnimationDecoder;
use std::io::BufReader;
use std::path::PathBuf;
use std::time::{Duration, Instant};

pub struct ImageState {
    pub texture: Option<egui::TextureHandle>,
    pub gif_frames: Vec<egui::TextureHandle>,
    pub current_frame: usize,
    pub last_frame_update: Instant,
    pub frame_delay: Duration,
    pub current_file_path: PathBuf,
}

impl ImageState {
    pub fn new(path: PathBuf) -> Self {
        Self {
            texture: None,
            gif_frames: Vec::new(),
            current_frame: 0,
            last_frame_update: Instant::now(),
            frame_delay: Duration::from_millis(100),
            current_file_path: path,
        }
    }

    pub fn load_from_path(&mut self, path: &PathBuf, ctx: &egui::Context) {
        self.current_file_path = path.clone();

        self.gif_frames.clear();
        self.texture = None;
        self.current_frame = 0;
        self.last_frame_update = Instant::now();

        let path_str = self
            .current_file_path
            .to_str()
            .unwrap_or_else(|| "<invalid path>");

        if let Some((frames, delay)) = Self::load_animated_gif(path_str, ctx) {
            log::debug!(
                "Loaded animated GIF with {} frames, delay: {:?}",
                frames.len(),
                delay
            );
            self.gif_frames = frames;
            self.frame_delay = delay;
        } else {
            self.texture = crate::image::loader::load_image(path_str, ctx);
            if self.texture.is_some() {
                log::debug!("Static image loaded: {}", path_str);
            } else {
                log::error!("Failed to load image: {}", path_str);
            }
        }
    }

    fn load_animated_gif(
        path: &str,
        ctx: &egui::Context,
    ) -> Option<(Vec<egui::TextureHandle>, Duration)> {
        let file = std::fs::File::open(path).ok()?;
        let buf_reader = BufReader::new(file);
        let decoder = image::codecs::gif::GifDecoder::new(buf_reader).ok()?;
        let frames = decoder.into_frames();
        let mut textures = Vec::new();
        let mut frame_delay = Duration::from_millis(100);

        for frame_result in frames {
            let frame = frame_result.ok()?;
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
            None
        } else {
            Some((textures, frame_delay))
        }
    }

    pub fn advance_frame_if_needed(&mut self) -> bool {
        if self.gif_frames.is_empty() {
            return false;
        }
        if self.last_frame_update.elapsed() >= self.frame_delay {
            self.current_frame = (self.current_frame + 1) % self.gif_frames.len();
            self.last_frame_update = Instant::now();
            true
        } else {
            false
        }
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

    pub fn render_image(&self, ui: &mut egui::Ui, texture: &egui::TextureHandle) {
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
