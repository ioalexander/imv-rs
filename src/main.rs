use eframe::egui;
use std::env;
use std::path::Path;

fn main() -> Result<(), eframe::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <image-path>", args[0]);
        std::process::exit(1);
    }

    let image_path = &args[1];

    if !Path::new(image_path).exists() {
        eprintln!("Error: File '{}' does not exist", image_path);
        std::process::exit(1);
    }

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Simple Image Viewer",
        options,
        Box::new(|cc| {
            let texture = load_image(image_path, &cc.egui_ctx);
            Ok(Box::new(ImageViewer::new(texture)))
        }),
    )
}

struct ImageViewer {
    texture: Option<egui::TextureHandle>,
}

impl ImageViewer {
    fn new(texture: Option<egui::TextureHandle>) -> Self {
        Self { texture }
    }
}

impl eframe::App for ImageViewer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(texture) = &self.texture {
                // Get the available size of the central panel
                let available_size = ui.available_size();

                // Calculate the aspect ratio and display size
                let texture_size = texture.size_vec2();
                let available_ratio = available_size.x / available_size.y;
                let image_ratio = texture_size.x / texture_size.y;

                let display_size = if image_ratio > available_ratio {
                    // Image is wider than available space
                    egui::vec2(available_size.x, available_size.x / image_ratio)
                } else {
                    // Image is taller than available space
                    egui::vec2(available_size.y * image_ratio, available_size.y)
                };

                // Center the image
                let pos = ui.available_rect_before_wrap().center() - display_size * 0.5;

                // Create an Image widget with the texture and set its size
                let image_widget = egui::Image::new(texture)
                    .fit_to_exact_size(display_size)
                    .maintain_aspect_ratio(true);

                // Show the image at the calculated position and size
                ui.put(egui::Rect::from_min_size(pos, display_size), image_widget);
            } else {
                ui.heading("Failed to load image");
            }
        });
    }
}

fn load_image(path: &str, ctx: &egui::Context) -> Option<egui::TextureHandle> {
    let image_data = std::fs::read(path).ok()?;
    let image = image::load_from_memory(&image_data).ok()?;
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    let image_data = egui::ColorImage::from_rgba_unmultiplied(
        [image.width() as usize, image.height() as usize],
        pixels.as_slice(),
    );

    Some(ctx.load_texture(path, image_data, egui::TextureOptions::LINEAR))
}
