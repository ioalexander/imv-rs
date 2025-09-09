use image::GenericImageView;
use minifb::{Key, Window, WindowOptions};
use resize::Pixel::RGBA8;
use resize::Type::Lanczos3;
use resize::px::RGBA;

fn main() {
    // Create a resizable window
    let mut window = Window::new(
        "Image Viewer",
        800,
        600,
        WindowOptions {
            resize: true,
            ..Default::default()
        },
    )
    .expect("Failed to create window");

    // Load the image
    let img = image::open("example.png").expect("Failed to open image");
    let (img_w, img_h) = img.dimensions();
    let img = img.to_rgba8();
    let img_pixels = img.as_raw();

    // Track last window size to redraw only when necessary
    let mut last_size = (0, 0);
    let mut buffer: Vec<u32> = Vec::new();

    loop {
        if !window.is_open() || window.is_key_down(Key::Escape) {
            break;
        }

        let (width, height) = window.get_size();
        let width = width as u32;
        let height = height as u32;

        if last_size != (width, height) {
            // Resize occurred → redraw
            buffer = vec![0; (width * height) as usize];
            buffer.fill(0x000000ff); // black background

            // Compute scale to "contain"
            let scale_x = width as f32 / img_w as f32;
            let scale_y = height as f32 / img_h as f32;
            let scale = scale_x.min(scale_y);

            let draw_w = (img_w as f32 * scale).round() as usize;
            let draw_h = (img_h as f32 * scale).round() as usize;

            let offset_x = ((width as usize - draw_w) / 2) as usize;
            let offset_y = ((height as usize - draw_h) / 2) as usize;

            // Destination buffer for resized image (flat RGBA u8)
            let mut resized_pixels = vec![0u8; draw_w * draw_h * 4];

            // Create resizer instance (unwrap Result)
            let mut resizer = resize::new(
                img_w as usize,
                img_h as usize,
                draw_w,
                draw_h,
                RGBA8,
                Lanczos3,
            )
            .expect("Failed to create resizer");
            // Convert the image buffer (Vec<u8>) to slice of RGBA pixels
            let src_pixels: &[RGBA<u8>] = unsafe {
                std::slice::from_raw_parts(
                    img_pixels.as_ptr() as *const RGBA<u8>,
                    img_pixels.len() / 4,
                )
            };
            let mut dst_pixels: &mut [RGBA<u8>] = unsafe {
                std::slice::from_raw_parts_mut(
                    resized_pixels.as_mut_ptr() as *mut RGBA<u8>,
                    resized_pixels.len() / 4,
                )
            };

            // Now resize
            resizer.resize(src_pixels, dst_pixels);

            // Copy resized pixels into framebuffer (ARGB for minifb)
            for y in 0..draw_h {
                for x in 0..draw_w {
                    let src_idx = (y * draw_w + x) * 4;
                    let dst_idx = ((y + offset_y) * width as usize + (x + offset_x)) as usize;

                    let r = resized_pixels[src_idx + 0] as u32;
                    let g = resized_pixels[src_idx + 1] as u32;
                    let b = resized_pixels[src_idx + 2] as u32;
                    let a = resized_pixels[src_idx + 3] as u32;

                    buffer[dst_idx] = (a << 24) | (r << 16) | (g << 8) | b;
                }
            }

            last_size = (width, height);
        }

        window
            .update_with_buffer(&buffer, width as usize, height as usize)
            .unwrap();
    }
}
