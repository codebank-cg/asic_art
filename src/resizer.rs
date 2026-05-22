use image::{imageops::FilterType, RgbImage};
use terminal_size::{terminal_size, Width};

const ASPECT_CORRECTION: f32 = 0.5;
const FALLBACK_WIDTH: u32 = 80;

pub struct ResizeOptions {
    /// Explicit output width in columns; overrides terminal auto-detect.
    pub width: Option<u32>,
    /// Explicit output height in rows; overrides aspect-ratio correction.
    pub height: Option<u32>,
    /// Multiply the resolved width by this factor before resizing.
    pub scale: Option<f32>,
}

pub fn resize(img: &RgbImage, opts: &ResizeOptions) -> RgbImage {
    let base_width = opts.width.unwrap_or_else(|| {
        terminal_size()
            .map(|(Width(w), _)| w as u32)
            .unwrap_or(FALLBACK_WIDTH)
    });
    let final_width = ((base_width as f32) * opts.scale.unwrap_or(1.0))
        .round()
        .max(1.0) as u32;

    let (orig_w, orig_h) = img.dimensions();
    let final_height = opts.height.unwrap_or_else(|| {
        ((orig_h as f32 / orig_w as f32) * final_width as f32 * ASPECT_CORRECTION)
            .round()
            .max(1.0) as u32
    });

    image::imageops::resize(img, final_width, final_height, FilterType::Lanczos3)
}

/// Convenience wrapper used by tests and the integration suite.
pub fn resize_to_width(img: &RgbImage, term_width: u32) -> RgbImage {
    resize(img, &ResizeOptions { width: Some(term_width), height: None, scale: None })
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::RgbImage;

    fn blank(w: u32, h: u32) -> RgbImage {
        RgbImage::new(w, h)
    }

    #[test]
    fn resizes_to_terminal_width() {
        let out = resize_to_width(&blank(100, 50), 80);
        assert_eq!(out.width(), 80);
    }

    #[test]
    fn applies_aspect_correction() {
        // 100x50 at width=80 → height = (50/100) * 80 * 0.5 = 20
        let out = resize_to_width(&blank(100, 50), 80);
        assert_eq!(out.height(), 20);
    }

    #[test]
    fn height_never_zero() {
        let out = resize_to_width(&blank(1000, 1), 80);
        assert!(out.height() >= 1);
    }

    #[test]
    fn explicit_width_overrides_terminal() {
        let out = resize(&blank(200, 100), &ResizeOptions { width: Some(60), height: None, scale: None });
        assert_eq!(out.width(), 60);
    }

    #[test]
    fn explicit_height_overrides_aspect_correction() {
        let out = resize(&blank(100, 50), &ResizeOptions { width: Some(80), height: Some(10), scale: None });
        assert_eq!(out.height(), 10);
    }

    #[test]
    fn scale_halves_width() {
        let out = resize(&blank(100, 100), &ResizeOptions { width: Some(80), height: None, scale: Some(0.5) });
        assert_eq!(out.width(), 40);
    }

    #[test]
    fn scale_doubles_width() {
        let out = resize(&blank(100, 100), &ResizeOptions { width: Some(40), height: None, scale: Some(2.0) });
        assert_eq!(out.width(), 80);
    }

    #[test]
    fn scale_and_explicit_height_together() {
        // width=40 * scale=2.0 = 80; height pinned to 15
        let out = resize(&blank(100, 100), &ResizeOptions { width: Some(40), height: Some(15), scale: Some(2.0) });
        assert_eq!(out.width(), 80);
        assert_eq!(out.height(), 15);
    }

    #[test]
    fn scale_below_one_clamps_to_minimum_one() {
        // Very small scale should not produce zero dimensions
        let out = resize(&blank(10, 10), &ResizeOptions { width: Some(1), height: None, scale: Some(0.01) });
        assert!(out.width() >= 1);
        assert!(out.height() >= 1);
    }
}
