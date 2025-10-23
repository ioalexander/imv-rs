use eframe::egui;
use image::AnimationDecoder;
use natord::compare;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

pub struct ImageViewer {
    // current image state
    texture: Option<egui::TextureHandle>,
    gif_frames: Vec<egui::TextureHandle>,
    current_frame: usize,
    last_frame_update: Instant,
    frame_delay: Duration,

    // file navigation state
    current_file_path: PathBuf,
    file_list: Vec<PathBuf>,
    current_index: usize,

    // navigation state
    pending_navigation: Option<isize>,
}

impl ImageViewer {
    pub fn new_from_path(path: &str, ctx: &egui::Context) -> Self {
        let current_path = PathBuf::from(path);

        let (file_list, current_index) = Self::build_file_list(&current_path);

        let mut viewer = Self {
            texture: None,
            gif_frames: Vec::new(),
            current_frame: 0,
            last_frame_update: Instant::now(),
            frame_delay: Duration::from_millis(100),
            current_file_path: current_path.clone(),
            file_list,
            current_index,
            pending_navigation: None,
        };

        viewer.load_current_image(ctx);
        viewer
    }

    fn build_file_list(current_path: &Path) -> (Vec<PathBuf>, usize) {
        let dir = current_path.parent().unwrap_or(Path::new("."));
        let mut file_list = Vec::new();

        let supported_extensions = ["jpg", "jpeg", "png", "gif", "webp", "bmp", "ico", "tiff"];

        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                        if supported_extensions.contains(&ext.to_lowercase().as_str()) {
                            file_list.push(path);
                        }
                    }
                }
            }
        }

        file_list.sort_by(|a, b| {
            compare(
                &a.file_name().unwrap().to_string_lossy(),
                &b.file_name().unwrap().to_string_lossy(),
            )
        });

        let current_index = file_list
            .iter()
            .position(|p| p == current_path)
            .unwrap_or(0);

        (file_list, current_index)
    }

    fn load_current_image(&mut self, ctx: &egui::Context) {
        if self.file_list.is_empty() {
            return;
        }

        self.current_file_path = self.file_list[self.current_index].clone();

        // reset animation state
        self.gif_frames.clear();
        self.texture = None;
        self.current_frame = 0;
        self.last_frame_update = Instant::now();

        let path_str = self.current_file_path.to_str().unwrap();

        // load as gif first
        if let Some((frames, delay)) = Self::load_animated_gif(path_str, ctx) {
            self.gif_frames = frames;
            self.frame_delay = delay;
        } else {
            self.texture = crate::image::loader::load_image(path_str, ctx);
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

    fn navigate_previous(&mut self) {
        if self.file_list.is_empty() {
            return;
        }

        self.current_index = if self.current_index == 0 {
            self.file_list.len() - 1
        } else {
            self.current_index - 1
        };

        self.pending_navigation = Some(-1);
    }

    fn navigate_next(&mut self) {
        if self.file_list.is_empty() {
            return;
        }

        self.current_index = if self.current_index == self.file_list.len() - 1 {
            0
        } else {
            self.current_index + 1
        };

        self.pending_navigation = Some(1);
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
        // handle pending navigation (deferred to avoid deadlocks)
        if self.pending_navigation.is_some() {
            self.load_current_image(ctx);
            self.pending_navigation = None;
        }

        // keyboard navigation
        ctx.input(|i| {
            if i.key_pressed(egui::Key::ArrowLeft) || i.key_pressed(egui::Key::H) {
                self.navigate_previous();
            }
            if i.key_pressed(egui::Key::ArrowRight) || i.key_pressed(egui::Key::L) {
                self.navigate_next();
            }
        });

        // animated gif logic
        if !self.gif_frames.is_empty() {
            if self.last_frame_update.elapsed() >= self.frame_delay {
                self.current_frame = (self.current_frame + 1) % self.gif_frames.len();
                self.last_frame_update = Instant::now();
            }
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
