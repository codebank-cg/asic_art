# Plan: ASCII Art Image Renderer

> Source PRD: PRD.md

## Architectural decisions

- **Binary name**: `asic_art`
- **Rust edition**: 2021, MSRV 1.74
- **Pipeline shape**: `cli::parse` → `loader::load` → `resizer::resize` → `renderer::render` → `writer::write`
- **Key models**: `RgbImage` (from `image` crate) flows through every stage; `renderer` produces `Vec<String>` (one ANSI-colored string per row)
- **Character scale** (dark → light): `' '` `░` `▒` `▓` `█` (U+2591–U+2593, U+2588)
- **Brightness formula**: perceptual luminance `0.2126R + 0.7152G + 0.0722B`
- **Aspect correction**: fixed factor of 0.5 (char height ≈ 2× char width)
- **Terminal width fallback**: 80 columns when detection fails
- **ANSI color format**: `\x1b[38;2;{R};{G};{B}m{char}\x1b[0m` per character
- **Key crates**: `image`, `clap` (derive API), `terminal_size`

---

## Phase 1: Working pipeline — image → grayscale chars → stdout

**User stories**: 1, 2, 3, 4, 6, 7, 14, 15, 19, 20

### What to build

Wire all six modules together into a fully working CLI. The user runs `asic_art <image>` and sees the image rendered as Unicode block characters in the terminal, auto-fitted to terminal width with aspect-ratio correction. Color is not applied yet — characters carry brightness only. Errors (missing file, unsupported format) print a human-readable message to stderr and exit non-zero. `--help` is provided by clap automatically.

### Acceptance criteria

- [ ] `asic_art photo.jpg` prints Unicode block chars to stdout without error
- [ ] Output width matches the detected terminal column count
- [ ] Output height is approximately `image_height / image_width * terminal_width * 0.5`
- [ ] Running against a non-existent path exits non-zero with a readable error message
- [ ] `asic_art --help` prints usage and argument descriptions
- [ ] `cargo build --release` produces a single binary with no runtime dependencies

---

## Phase 2: Full-color ANSI rendering

**User stories**: 5

### What to build

Extend the renderer so every character is wrapped in an ANSI 24-bit foreground color escape matching its source pixel's RGB value, followed by a reset. The grayscale brightness mapping from Phase 1 is unchanged — color is layered on top. The terminal now displays a vibrant, color-accurate representation of the original image.

### Acceptance criteria

- [ ] Output characters are wrapped in `\x1b[38;2;R;G;Bm…\x1b[0m` with correct RGB values
- [ ] A red pixel maps to a character with red foreground color
- [ ] A white pixel maps to `█` with white foreground
- [ ] A black pixel maps to `' '` (space) regardless of color
- [ ] No color bleed between characters when a terminal truncates a line

---

## Phase 3: PNG transparency and multi-format validation

**User stories**: 8, 9, 10, 11

### What to build

Add alpha compositing in the loader: transparent PNG pixels are blended against a black background before the image is converted to `RgbImage`. Confirm that BMP, TIFF, and WebP files all load and render correctly through the existing pipeline end-to-end.

### Acceptance criteria

- [ ] A fully transparent PNG pixel renders as `' '` (black background composite)
- [ ] A 50% transparent red pixel renders with a darkened red, not as pure red
- [ ] `asic_art image.bmp` renders without error
- [ ] `asic_art image.tiff` renders without error
- [ ] `asic_art image.webp` renders without error
- [ ] An unsupported format (e.g. `.svg`) exits non-zero with a clear error

---

## Phase 4: File output (`--output`)

**User stories**: 12, 13

### What to build

Add an `--output <path>` flag. When provided, the writer strips all ANSI escape sequences from the rendered rows and writes plain UTF-8 text to the specified file instead of stdout. When the flag is omitted, behavior is unchanged (colored stdout).

### Acceptance criteria

- [ ] `asic_art photo.jpg --output out.txt` creates `out.txt`
- [ ] `out.txt` contains no `\x1b[` escape sequences
- [ ] `out.txt` contains the same Unicode block characters as the stdout output
- [ ] A write error (e.g. unwritable path) exits non-zero with a readable error message
- [ ] Omitting `--output` still prints colored output to stdout

---

## Phase 5: Test suite

**User stories**: 16, 17, 18

### What to build

Add unit tests for the four pure/isolated modules and one integration test. No test should touch the real terminal or require a specific screen size — pass terminal width as a parameter in tests. Bundle a small deterministic test image (8×8 gradient PNG) in `tests/fixtures/`.

### Acceptance criteria

- [ ] `mapper` unit tests: black pixel → `' '`, white pixel → `'█'`, known midpoint grays → correct intermediate chars
- [ ] `resizer` unit tests: given a 100×50 image and width=80, output dimensions match expected values after aspect correction
- [ ] `renderer` unit tests: a 1×1 red `RgbImage` produces a string containing `\x1b[38;2;255;0;0m` and `\x1b[0m`
- [ ] `writer` unit tests: input strings with ANSI codes produce stripped output containing no `\x1b[` sequences
- [ ] Integration test: load `tests/fixtures/gradient.png` at width=40, assert row count and per-row character count are correct
- [ ] `cargo test` passes with no failures
