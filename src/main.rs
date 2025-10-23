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
    let path = Path::new(image_path);

    if !path.exists() {
        eprintln!("Error: File '{}' does not exist", image_path);
        std::process::exit(1);
    }

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "imvrs - Image Viewer",
        options,
        Box::new(|cc| {
            let viewer = app::viewer::ImageViewer::new_from_path(image_path, &cc.egui_ctx);
            Ok(Box::new(viewer))
        }),
    )
}
