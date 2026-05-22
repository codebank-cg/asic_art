use image::{DynamicImage, RgbImage};
use std::path::Path;

pub fn load(path: &Path) -> Result<RgbImage, String> {
    let img = image::open(path).map_err(|e| format!("Failed to open image '{}': {}", path.display(), e))?;
    Ok(to_rgb(img))
}

fn to_rgb(img: DynamicImage) -> RgbImage {
    match img {
        DynamicImage::ImageRgba8(rgba) => {
            let (w, h) = rgba.dimensions();
            let mut rgb = RgbImage::new(w, h);
            for (x, y, pixel) in rgba.enumerate_pixels() {
                let a = pixel[3] as f32 / 255.0;
                let r = (pixel[0] as f32 * a) as u8;
                let g = (pixel[1] as f32 * a) as u8;
                let b = (pixel[2] as f32 * a) as u8;
                rgb.put_pixel(x, y, image::Rgb([r, g, b]));
            }
            rgb
        }
        other => other.to_rgb8(),
    }
}
