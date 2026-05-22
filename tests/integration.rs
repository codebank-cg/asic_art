use std::path::Path;

#[test]
fn gradient_png_renders_correct_dimensions() {
    let fixture = Path::new("tests/fixtures/gradient.png");
    assert!(fixture.exists(), "missing test fixture");

    // Load via the public pipeline
    let img = asic_art_lib::loader::load(fixture).expect("load failed");
    let resized = asic_art_lib::resizer::resize_to_width(&img, 40);
    let rows = asic_art_lib::renderer::render(&resized);

    assert_eq!(rows.len() as u32, resized.height(), "row count mismatch");
    // Each row should have one ANSI sequence per column
    let expected_chars_per_row = 40usize;
    // Each cell is "\x1b[38;2;R;G;Bm{ch}\x1b[0m" — count reset sequences as proxy for char count
    let resets = rows[0].matches("\x1b[0m").count();
    assert_eq!(resets, expected_chars_per_row, "column count mismatch");
}
