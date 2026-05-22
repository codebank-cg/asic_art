use image::Rgb;

const CHARS: [char; 5] = [' ', '░', '▒', '▓', '█'];

pub fn map_pixel(pixel: Rgb<u8>) -> char {
    let [r, g, b] = pixel.0;
    let brightness = 0.2126 * r as f32 + 0.7152 * g as f32 + 0.0722 * b as f32;
    let index = ((brightness / 255.0) * (CHARS.len() - 1) as f32).round() as usize;
    CHARS[index.min(CHARS.len() - 1)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn black_maps_to_space() {
        assert_eq!(map_pixel(Rgb([0, 0, 0])), ' ');
    }

    #[test]
    fn white_maps_to_full_block() {
        assert_eq!(map_pixel(Rgb([255, 255, 255])), '█');
    }

    #[test]
    fn midpoint_gray_maps_to_medium_shade() {
        let ch = map_pixel(Rgb([128, 128, 128]));
        assert!(ch == '▒' || ch == '▓', "expected medium shade, got {ch}");
    }

    #[test]
    fn dark_gray_maps_to_light_shade() {
        let ch = map_pixel(Rgb([64, 64, 64]));
        assert!(ch == '░' || ch == '▒', "expected light shade, got {ch}");
    }
}
