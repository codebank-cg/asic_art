# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
# Build
cargo build
cargo build --release

# Run (dev)
cargo run -- <image> [OPTIONS]

# Run (release)
./target/release/asic_art <image> [OPTIONS]

# Test all
cargo test

# Test a single module
cargo test --lib mapper
cargo test --lib resizer
cargo test --lib renderer
cargo test --lib writer

# Run a single test by name
cargo test black_maps_to_space

# Lint
cargo clippy

# Format
cargo fmt
```

## Architecture

The crate is split into a binary (`src/main.rs`) and a library (`asic_art_lib` in `src/lib.rs`). `main.rs` imports from the library via `asic_art_lib::*`; `cli.rs` is private to the binary only.

Pipeline (linear, no shared state):

```
cli::parse() → Args
loader::load(path) → RgbImage        # decodes to RGB8; composites RGBA against black
resizer::resize(img, opts) → RgbImage # Lanczos3; width/scale/height logic in ResizeOptions
renderer::render(img) → Vec<String>   # one String per row; ANSI 24-bit color per pixel
writer::write_stdout / write_file     # file path strips ANSI with a hand-rolled parser
```

### Key design choices

- **`mapper::map_pixel`** is a pure function — brightness via perceptual luminance (`0.2126R + 0.7152G + 0.0722B`), mapped to `[' ', '░', '▒', '▓', '█']`.
- **`resizer::ResizeOptions`**: width resolution order is `--width` → terminal auto-detect → 80. `--scale` multiplies the resolved width. `--height` overrides aspect-ratio correction (`height = (orig_h/orig_w) * width * 0.5`).
- **`writer::strip_ansi`** is hand-rolled (no regex crate) — walks chars looking for `ESC [` sequences and discards through the final ASCII alpha.
- MSRV is **1.74** (driven by the `image` crate).

## Skill routing

When the user's request matches an available skill, invoke it via the Skill tool. When in doubt, invoke the skill.

Key routing rules:
- Product ideas/brainstorming → invoke /office-hours
- Strategy/scope → invoke /plan-ceo-review
- Architecture → invoke /plan-eng-review
- Design system/plan review → invoke /design-consultation or /plan-design-review
- Full review pipeline → invoke /autoplan
- Bugs/errors → invoke /investigate
- QA/testing site behavior → invoke /qa or /qa-only
- Code review/diff check → invoke /review
- Visual polish → invoke /design-review
- Ship/deploy/PR → invoke /ship or /land-and-deploy
- Save progress → invoke /context-save
- Resume context → invoke /context-restore
- Author a backlog-ready spec/issue → invoke /spec
