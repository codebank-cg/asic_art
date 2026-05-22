use image::{imageops::FilterType, RgbImage};
use terminal_size::{terminal_size, Width};

const ASPECT_CORRECTION: f32 = 0.5;
const FALLBACK_WIDTH: u32 = 80;

pub fn resize(img: &RgbImage) -> RgbImage {
    let term_width = terminal_size()
        .map(|(Width(w), _)| w as u32)
        .unwrap_or(FALLBACK_WIDTH);
    resize_to_width(img, term_width)
}

pub fn resize_to_width(img: &RgbImage, term_width: u32) -> RgbImage {
    let (orig_w, orig_h) = img.dimensions();
    let target_w = term_width;
    let target_h = ((orig_h as f32 / orig_w as f32) * target_w as f32 * ASPECT_CORRECTION).max(1.0) as u32;
    image::imageops::resize(img, target_w, target_h, FilterType::Lanczos3)
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::RgbImage;

    #[test]
    fn resizes_to_terminal_width() {
        let img = RgbImage::new(100, 50);
        let out = resize_to_width(&img, 80);
        assert_eq!(out.width(), 80);
    }

    #[test]
    fn applies_aspect_correction() {
        // 100x50 image at width=80: height = (50/100) * 80 * 0.5 = 20
        let img = RgbImage::new(100, 50);
        let out = resize_to_width(&img, 80);
        assert_eq!(out.height(), 20);
    }

    #[test]
    fn height_never_zero() {
        let img = RgbImage::new(1000, 1);
        let out = resize_to_width(&img, 80);
        assert!(out.height() >= 1);
    }
}
