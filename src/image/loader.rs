use eframe::egui;

pub fn load_image(path: &str, ctx: &egui::Context) -> Option<egui::TextureHandle> {
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
