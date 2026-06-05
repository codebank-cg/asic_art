use image::RgbImage;
use crate::mapper::map_pixel;

/// Render image into ANSI-colored Unicode art, appending to `out`.
/// Row endings: `\n` (still-image path, not in raw mode).
pub fn render(img: &RgbImage, out: &mut Vec<u8>, grayscale: bool) {
    render_impl(img, out, b"\n", grayscale);
}

/// Same as `render` but uses `\r\n` row endings required for terminal raw mode (video playback).
pub fn render_crlf(img: &RgbImage, out: &mut Vec<u8>, grayscale: bool) {
    render_impl(img, out, b"\r\n", grayscale);
}

fn render_impl(img: &RgbImage, out: &mut Vec<u8>, row_ending: &[u8], grayscale: bool) {
    let (width, height) = img.dimensions();
    for y in 0..height {
        for x in 0..width {
            let pixel = *img.get_pixel(x, y);
            let [r, g, b] = pixel.0;
            let ch = map_pixel(pixel);
            let (cr, cg, cb) = if grayscale {
                let luma = (0.2126 * r as f32 + 0.7152 * g as f32 + 0.0722 * b as f32)
                    .round() as u8;
                (luma, luma, luma)
            } else {
                (r, g, b)
            };
            out.extend_from_slice(b"\x1b[38;2;");
            write_decimal(out, cr);
            out.push(b';');
            write_decimal(out, cg);
            out.push(b';');
            write_decimal(out, cb);
            out.push(b'm');
            let mut tmp = [0u8; 4];
            out.extend_from_slice(ch.encode_utf8(&mut tmp).as_bytes());
            out.extend_from_slice(b"\x1b[0m");
        }
        out.extend_from_slice(row_ending);
    }
}

fn write_decimal(out: &mut Vec<u8>, n: u8) {
    if n >= 100 {
        out.push(b'0' + n / 100);
        out.push(b'0' + (n / 10) % 10);
        out.push(b'0' + n % 10);
    } else if n >= 10 {
        out.push(b'0' + n / 10);
        out.push(b'0' + n % 10);
    } else {
        out.push(b'0' + n);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{Rgb, RgbImage};

    fn render_to_string(img: &RgbImage) -> String {
        let mut buf = Vec::new();
        render(img, &mut buf, false);
        String::from_utf8(buf).unwrap()
    }

    #[test]
    fn red_pixel_has_correct_ansi_prefix() {
        let mut img = RgbImage::new(1, 1);
        img.put_pixel(0, 0, Rgb([255, 0, 0]));
        let out = render_to_string(&img);
        assert!(out.contains("\x1b[38;2;255;0;0m"), "missing red ANSI code");
    }

    #[test]
    fn each_char_is_reset() {
        let mut img = RgbImage::new(1, 1);
        img.put_pixel(0, 0, Rgb([128, 128, 128]));
        let out = render_to_string(&img);
        assert!(out.contains("\x1b[0m"), "missing reset code");
    }

    #[test]
    fn output_has_one_row_per_pixel_row() {
        let img = RgbImage::new(4, 3);
        let mut buf = Vec::new();
        render(&img, &mut buf, false);
        let out = String::from_utf8(buf).unwrap();
        assert_eq!(out.matches('\n').count(), 3);
    }

    #[test]
    fn render_crlf_uses_crlf_endings() {
        let img = RgbImage::new(2, 2);
        let mut buf = Vec::new();
        render_crlf(&img, &mut buf, false);
        let out = String::from_utf8(buf).unwrap();
        assert_eq!(out.matches("\r\n").count(), 2);
        assert!(!out.contains("\n\n"), "bare LF should not appear");
    }

    #[test]
    fn grayscale_uses_equal_rgb_components() {
        let mut img = RgbImage::new(1, 1);
        img.put_pixel(0, 0, Rgb([200, 100, 50]));
        let mut buf = Vec::new();
        render(&img, &mut buf, true);
        let out = String::from_utf8(buf).unwrap();
        // Expect equal R;G;B components (some gray value repeated three times).
        let luma = (0.2126 * 200f32 + 0.7152 * 100f32 + 0.0722 * 50f32).round() as u8;
        let expected = format!("\x1b[38;2;{luma};{luma};{luma}m");
        assert!(out.contains(&expected), "expected gray ANSI code {expected} in {out:?}");
    }
}
