use std::path::Path;
use asic_art_lib::{loader, renderer, resizer};
use resizer::ResizeOptions;

fn load_gradient() -> image::RgbImage {
    let fixture = Path::new("tests/fixtures/gradient.png");
    assert!(fixture.exists(), "missing test fixture: tests/fixtures/gradient.png");
    loader::load(fixture).expect("load failed")
}

fn render_to_lines(img: &image::RgbImage) -> Vec<String> {
    let mut buf = Vec::new();
    renderer::render(img, &mut buf, false);
    let s = String::from_utf8(buf).unwrap();
    s.lines().map(|l| l.to_owned()).collect()
}

fn count_resets(row: &str) -> usize {
    row.matches("\x1b[0m").count()
}

// ── baseline ────────────────────────────────────────────────────────────────

#[test]
fn gradient_png_renders_correct_dimensions() {
    let img = load_gradient();
    let resized = resizer::resize_to_width(&img, 40);
    let rows = render_to_lines(&resized);

    assert_eq!(rows.len() as u32, resized.height(), "row count mismatch");
    assert_eq!(count_resets(&rows[0]), 40, "column count mismatch");
}

// ── explicit width ───────────────────────────────────────────────────────────

#[test]
fn explicit_width_controls_output_columns() {
    let img = load_gradient();
    let resized = resizer::resize(&img, &ResizeOptions { width: Some(20), height: None, scale: None });
    let rows = render_to_lines(&resized);
    assert_eq!(count_resets(&rows[0]), 20);
}

#[test]
fn explicit_width_larger_than_image_upscales() {
    let img = load_gradient(); // 8x8 source
    let resized = resizer::resize(&img, &ResizeOptions { width: Some(32), height: None, scale: None });
    let rows = render_to_lines(&resized);
    assert_eq!(count_resets(&rows[0]), 32);
}

// ── explicit height ──────────────────────────────────────────────────────────

#[test]
fn explicit_height_controls_output_rows() {
    let img = load_gradient();
    let resized = resizer::resize(&img, &ResizeOptions { width: Some(40), height: Some(5), scale: None });
    let rows = render_to_lines(&resized);
    assert_eq!(rows.len(), 5);
}

#[test]
fn explicit_width_and_height_together() {
    let img = load_gradient();
    let resized = resizer::resize(&img, &ResizeOptions { width: Some(30), height: Some(8), scale: None });
    let rows = render_to_lines(&resized);
    assert_eq!(count_resets(&rows[0]), 30);
    assert_eq!(rows.len(), 8);
}

// ── scale factor ─────────────────────────────────────────────────────────────

#[test]
fn scale_half_produces_half_columns() {
    let img = load_gradient();
    let resized = resizer::resize(&img, &ResizeOptions { width: Some(40), height: None, scale: Some(0.5) });
    let rows = render_to_lines(&resized);
    assert_eq!(count_resets(&rows[0]), 20);
}

#[test]
fn scale_double_produces_double_columns() {
    let img = load_gradient();
    let resized = resizer::resize(&img, &ResizeOptions { width: Some(20), height: None, scale: Some(2.0) });
    let rows = render_to_lines(&resized);
    assert_eq!(count_resets(&rows[0]), 40);
}

#[test]
fn scale_with_pinned_height() {
    let img = load_gradient();
    let resized = resizer::resize(&img, &ResizeOptions { width: Some(20), height: Some(6), scale: Some(2.0) });
    let rows = render_to_lines(&resized);
    assert_eq!(count_resets(&rows[0]), 40); // 20 * 2.0
    assert_eq!(rows.len(), 6);              // pinned
}
