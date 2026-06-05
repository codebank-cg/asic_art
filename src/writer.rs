use std::fs;
use std::io::{self, Write};
use std::path::Path;

use image::RgbImage;

use crate::renderer::render;

pub fn write_stdout(img: &RgbImage, grayscale: bool) {
    let mut buf = Vec::with_capacity(img.width() as usize * img.height() as usize * 24);
    render(img, &mut buf, grayscale);
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    handle.write_all(&buf).unwrap();
}

pub fn write_file(img: &RgbImage, path: &Path, grayscale: bool) -> Result<(), String> {
    let mut buf = Vec::new();
    render(img, &mut buf, grayscale);
    let raw = String::from_utf8_lossy(&buf);
    let plain = strip_ansi(&raw);
    fs::write(path, plain).map_err(|e| format!("Failed to write '{}': {}", path.display(), e))
}

fn strip_ansi(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '\x1b' {
            if chars.peek() == Some(&'[') {
                chars.next();
                for c in chars.by_ref() {
                    if c.is_ascii_alphabetic() {
                        break;
                    }
                }
            }
        } else {
            out.push(ch);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::strip_ansi;

    #[test]
    fn strips_color_codes() {
        let input = "\x1b[38;2;255;0;0m█\x1b[0m";
        let out = strip_ansi(input);
        assert_eq!(out, "█");
        assert!(!out.contains('\x1b'));
    }

    #[test]
    fn strips_multiple_codes_per_row() {
        let input = "\x1b[38;2;0;255;0m░\x1b[0m\x1b[38;2;0;0;255m▒\x1b[0m";
        let out = strip_ansi(input);
        assert_eq!(out, "░▒");
    }

    #[test]
    fn passthrough_plain_text() {
        let input = "hello world";
        assert_eq!(strip_ansi(input), "hello world");
    }
}
