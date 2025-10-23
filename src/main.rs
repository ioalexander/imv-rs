use std::sync::Once;

static LOG_INIT: Once = Once::new();

mod app;
mod image;

use eframe::egui;
use std::env;
use std::path::Path;

fn main() -> Result<(), eframe::Error> {
    let args: Vec<String> = env::args().collect();

    let (debug_mode, image_path) = if args.len() == 2 {
        (false, args[1].as_str())
    } else if args.len() == 3 && args[1] == "--debug" {
        (true, args[2].as_str())
    } else if args.len() == 1 {
        (false, ".")
    } else {
        eprintln!("Usage: {} [--debug] <image-path>", args[0]);
        std::process::exit(1);
    };

    LOG_INIT.call_once(|| {
        if debug_mode {
            env_logger::Builder::from_default_env()
                .filter_level(log::LevelFilter::Debug)
                .init();
            log::debug!("Debug mode enabled");
        } else {
            env_logger::Builder::from_default_env()
                .filter_level(log::LevelFilter::Warn)
                .init();
        }
    });

    if !Path::new(image_path).exists() {
        log::error!("File '{}' does not exist", image_path);
        eprintln!("Error: File '{}' does not exist", image_path);
        std::process::exit(1);
    }

    log::info!("Starting image viewer with path: {}", image_path);

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
