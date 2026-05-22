use std::fs;
use std::io::{self, Write};
use std::path::Path;

pub fn write_stdout(rows: &[String]) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    for row in rows {
        handle.write_all(row.as_bytes()).unwrap();
        handle.write_all(b"\n").unwrap();
    }
}

pub fn write_file(rows: &[String], path: &Path) -> Result<(), String> {
    let plain: String = rows
        .iter()
        .map(|row| strip_ansi(row))
        .collect::<Vec<_>>()
        .join("\n");
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
