use eframe::egui;
use std::path::Path;
use std::time::Instant;

pub fn load_image(path: &str, ctx: &egui::Context) -> Option<egui::TextureHandle> {
    let total_start = Instant::now();
    log::debug!("Loading image: {}", path);

    let result = load_with_image_crate(path, ctx);

    if result.is_some() {
        log::info!("TOTAL load time for {}: {:?}", path, total_start.elapsed());
    }

    result
}

fn load_with_image_crate(path: &str, ctx: &egui::Context) -> Option<egui::TextureHandle> {
    let open_start = Instant::now();

    let image = match image::open(Path::new(path)) {
        Ok(img) => img,
        Err(e) => {
            log::error!("Failed to open image {}: {}", path, e);
            return None;
        }
    };

    log::debug!("image::open elapsed: {:?}", open_start.elapsed());
    log::debug!("Image dimensions: {}x{}", image.width(), image.height());

    let process_start = Instant::now();
    let image_buffer = image.to_rgba8();
    let image_data = egui::ColorImage::from_rgba_unmultiplied(
        [image.width() as usize, image.height() as usize],
        image_buffer.as_flat_samples().as_slice(),
    );
    log::debug!("Image processing: {:?}", process_start.elapsed());

    let texture_start = Instant::now();
    let texture = ctx.load_texture(path, image_data, egui::TextureOptions::LINEAR);
    log::debug!("Texture creation: {:?}", texture_start.elapsed());

    Some(texture)
}
