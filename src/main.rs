mod app;
mod image;

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
            // Try to load as GIF first, fall back to static image
            if let Some(gif_viewer) =
                app::viewer::ImageViewer::load_animated_gif(image_path, &cc.egui_ctx)
            {
                Ok(Box::new(gif_viewer))
            } else {
                let texture = image::loader::load_image(image_path, &cc.egui_ctx);
                Ok(Box::new(app::viewer::ImageViewer::new(texture)))
            }
        }),
    )
}
