use image::RgbImage;
use crate::mapper::map_pixel;

pub fn render(img: &RgbImage) -> Vec<String> {
    let (width, height) = img.dimensions();
    (0..height)
        .map(|y| {
            (0..width)
                .map(|x| {
                    let pixel = *img.get_pixel(x, y);
                    let [r, g, b] = pixel.0;
                    let ch = map_pixel(pixel);
                    format!("\x1b[38;2;{r};{g};{b}m{ch}\x1b[0m")
                })
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{Rgb, RgbImage};

    #[test]
    fn red_pixel_has_correct_ansi_prefix() {
        let mut img = RgbImage::new(1, 1);
        img.put_pixel(0, 0, Rgb([255, 0, 0]));
        let rows = render(&img);
        assert_eq!(rows.len(), 1);
        assert!(rows[0].contains("\x1b[38;2;255;0;0m"), "missing red ANSI code");
    }

    #[test]
    fn each_char_is_reset() {
        let mut img = RgbImage::new(1, 1);
        img.put_pixel(0, 0, Rgb([128, 128, 128]));
        let rows = render(&img);
        assert!(rows[0].ends_with("\x1b[0m"), "missing reset code");
    }

    #[test]
    fn output_has_one_row_per_pixel_row() {
        let img = RgbImage::new(4, 3);
        let rows = render(&img);
        assert_eq!(rows.len(), 3);
    }
}
