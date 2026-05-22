# PRD: ASCII Art Image Renderer (asic_art)

## Problem Statement

Developers and terminal users have no simple, fast command-line tool to render an image file as colored Unicode ASCII art directly in their terminal. Existing tools are often written in Python (slow startup), produce only grayscale output, or require complex installation. There is no idiomatic Rust CLI that accepts an image path, auto-fits to the terminal, and renders a faithful color representation using Unicode block characters and ANSI 24-bit color.

## Solution

A single-binary Rust CLI tool (`asic_art`) that reads an image file, resizes it to fit the current terminal width, maps each pixel to a Unicode block character (░▒▓█) weighted by brightness, applies ANSI 24-bit foreground color matching the original pixel, and prints the result to stdout. Optionally the output can be saved to a plain-text file (ANSI codes stripped).

## User Stories

1. As a Rust developer, I want to install a single binary, so that I don't need a Python runtime or complex dependencies.
2. As a terminal user, I want to pass an image file path as a positional argument, so that the tool is intuitive to use.
3. As a terminal user, I want the output to automatically fit my terminal width, so that the image is not truncated or too small.
4. As a terminal user, I want the output to correct for monospace font aspect ratio, so that the image is not stretched vertically.
5. As a terminal user, I want the characters to be colored with ANSI 24-bit color matching the original image, so that the output is visually faithful to the source image.
6. As a terminal user, I want Unicode block characters (░▒▓█) used for brightness mapping, so that shading is smooth and precise.
7. As a terminal user, I want to open JPEG images, so that I can render common photos.
8. As a terminal user, I want to open PNG images, so that I can render logos and diagrams with transparency.
9. As a terminal user, I want to open BMP images, so that I can render uncompressed bitmap files.
10. As a terminal user, I want to open TIFF images, so that I can render scanned documents and high-quality images.
11. As a terminal user, I want to open WebP images, so that I can render modern web image formats.
12. As a terminal user, I want to save the ASCII art to a file with `--output`, so that I can share or embed the output in documentation.
13. As a terminal user, I want the saved file to have ANSI codes stripped, so that the text file is readable in any editor.
14. As a terminal user, I want a clear error message when the input file does not exist, so that I can quickly fix the path.
21. As a terminal user, I want to set the output width with `--width <N>`, so that I can render images at a specific column count regardless of my terminal size.
22. As a terminal user, I want to set the output height with `--height <N>`, so that I can fit the art into a fixed canvas without relying on aspect-ratio correction.
23. As a terminal user, I want to scale the output with `--scale <F>`, so that I can quickly halve or double the size without calculating exact dimensions.
24. As a terminal user, I want `--scale` to multiply the resolved width (whether from `--width` or auto-detected), so that scaling behaves consistently in both cases.
25. As a terminal user, I want `--width` and `--scale` to be usable together with `--height`, so that I have full control over both dimensions independently.
15. As a terminal user, I want a clear error message when the image format is unsupported, so that I understand what formats are accepted.
16. As a developer, I want the image loading step to be isolated from the rendering step, so that I can test each component independently.
17. As a developer, I want the character mapper to be a pure function of brightness, so that it can be unit tested without terminal or file I/O.
18. As a developer, I want the color renderer to produce deterministic ANSI escape strings, so that output can be snapshot-tested.
19. As a CI user, I want the binary to exit with a non-zero code on error, so that pipelines fail loudly.
20. As a terminal user, I want `--help` to show usage and all flags, so that I can discover options without reading a README.

## Implementation Decisions

### Modules

**1. CLI (`cli` module)**
- Built with `clap` (derive API)
- Positional argument: `<input>` — path to the image file
- Optional flag: `--output <path>` — write plain-text (ANSI-stripped) output to a file
- Optional flag: `--width <N>` (u32 ≥ 1) — explicit output width in columns
- Optional flag: `--height <N>` (u32 ≥ 1) — explicit output height in rows
- Optional flag: `--scale <F>` (f32 > 0) — scale factor on resolved width
- Prints `--help` with descriptions for all arguments

**2. Image Loader (`loader` module)**
- Uses the `image` crate to open and decode the input file
- Supports: JPEG, PNG, BMP, TIFF, WebP (all handled by the `image` crate's format auto-detection)
- Transparent PNG pixels: alpha-composite against a black background before processing
- Returns an `RgbImage` (all pixels as 8-bit RGB)

**3. Resizer / Sampler (`resizer` module)**
- Accepts a `ResizeOptions` struct: `width: Option<u32>`, `height: Option<u32>`, `scale: Option<f32>`
- Width resolution order: explicit `--width` → terminal auto-detect → fallback 80
- `--scale` is applied on top of the resolved width: `final_width = base_width * scale`
- Height resolution order: explicit `--height` → aspect-ratio formula
- Aspect formula: `(image_height / image_width) * final_width * ASPECT_CORRECTION`
- `ASPECT_CORRECTION` = 0.5 (monospace chars are roughly 2× taller than wide)
- Both `final_width` and `final_height` are clamped to a minimum of 1
- Uses `image::imageops::resize` with `FilterType::Lanczos3` for quality downscaling

**4. Character Mapper (`mapper` module)**
- Maps a single `Rgb<u8>` pixel to a Unicode block character
- Brightness = `(0.2126 * R + 0.7152 * G + 0.0722 * B)` (perceptual luminance)
- Character scale (dark→light): `' '`, `'░'` (U+2591), `'▒'` (U+2592), `'▓'` (U+2593), `'█'` (U+2588)
- Pure function: `fn map_pixel(pixel: Rgb<u8>) -> char`

**5. Color Renderer (`renderer` module)**
- Iterates over each resized pixel row-by-row
- For each pixel, wraps the mapped character in ANSI 24-bit foreground color:
  `\x1b[38;2;{R};{G};{B}m{char}\x1b[0m`
- Builds a `String` per row, joined by newlines
- Returns `Vec<String>` (one String per row)

**6. Output Writer (`writer` module)**
- Stdout path: writes colored rows directly via `println!`
- File path (`--output`): strips all ANSI escape sequences with a regex (`\x1b\[[0-9;]*m`) before writing to file

### Architecture

```
main
 └─ cli::parse()           → Args
 └─ loader::load(path)     → RgbImage
 └─ resizer::resize(img)   → RgbImage (terminal-fit dimensions)
 └─ renderer::render(img)  → Vec<String> (ANSI colored rows)
 └─ writer::write(rows, output_path)
```

### Key Crate Dependencies

| Crate | Purpose |
|-------|---------|
| `image` | Image decoding and resizing |
| `clap` | CLI argument parsing |
| `terminal_size` | Detect terminal width at runtime |

## Testing Decisions

### What makes a good test

Tests should only verify **observable behavior** — what goes in and what comes out — never internal implementation details (private fields, intermediate buffers, call order). Tests should be deterministic: no filesystem side effects unless explicitly testing the writer.

### Modules with unit tests

- **`mapper` module**: Pure function tests. Given an `Rgb<u8>` pixel with a known brightness, assert the returned character is the expected Unicode block char. Cover boundary values: pure black → `' '`, pure white → `'█'`, midpoint grays → intermediate chars.

- **`resizer` module**: Given an image of known dimensions and a specified terminal width, assert the output image dimensions match the expected values after aspect-ratio correction. No real terminal needed — pass width as a parameter.

- **`renderer` module**: Given a 1×1 `RgbImage` with a known RGB value, assert the output string contains the correct ANSI escape prefix and suffix. Snapshot test for a small 2×2 image with known pixels.

- **`writer` module**: Given a `Vec<String>` with ANSI escape codes, assert the stripped output contains no `\x1b[` sequences.

### Integration test

- Load a small bundled test PNG (e.g., 8×8 gradient), run the full pipeline with a fixed terminal width, assert the output has the correct number of rows and each row has the correct character count.

## Out of Scope

- Animated GIF playback (multi-frame)
- Video input
- Interactive terminal UI (scrolling, zoom)
- Sixel or Kitty graphics protocol output
- Windows console (ANSI color codes only — Windows Terminal supports them, legacy cmd.exe does not)
- Font aspect ratio detection (fixed 0.5 correction factor)
- `--invert` flag (not requested)
- Image preprocessing (brightness, contrast, saturation adjustments)

## Further Notes

- The binary name should be `asic_art` matching the project directory name.
- Rust edition: 2021.
- Target MSRV (minimum supported Rust version): 1.74 (stable, matches `image` crate requirements).
- The `image` crate handles format auto-detection by magic bytes, not file extension, making it robust to misnamed files.
- ANSI reset (`\x1b[0m`) after every character prevents color bleed if the terminal truncates a line mid-render.
