asic_art
========

A fast command-line tool written in Rust that renders image files as colored
Unicode ASCII art directly in your terminal.


USAGE
-----

    asic_art <image> [OPTIONS]
    asic_art <image> --output result.txt
    asic_art <image> --width 120
    asic_art <image> --scale 0.5
    asic_art <image> --width 80 --height 40


OPTIONS
-------

    --output <file>    Write plain-text output (ANSI codes stripped) to a file
    --width <N>        Set output width in columns (default: terminal width)
    --height <N>       Set output height in rows (default: aspect-ratio corrected)
    --scale <F>        Scale factor on the resolved width, e.g. 0.5 = half, 2.0 = double
    --help             Print help


EXAMPLES
--------

    # Auto-fit to terminal width
    asic_art photo.jpg

    # Fix output to 120 columns wide
    asic_art photo.jpg --width 120

    # Half the terminal width
    asic_art photo.jpg --scale 0.5

    # Fixed 80x24 canvas
    asic_art photo.jpg --width 80 --height 24

    # Scale up and save to file
    asic_art logo.png --scale 2.0 --output logo.txt

    # Explicit dimensions saved to file
    asic_art banner.webp --width 160 --height 50 --output banner.txt


SUPPORTED FORMATS
-----------------

    JPEG, PNG, BMP, TIFF, WebP


FEATURES
--------

  - Unicode block characters (░ ▒ ▓ █) for smooth brightness shading
  - ANSI 24-bit color — each character matches the original pixel color
  - Auto-fits to your terminal width (default behavior)
  - --width: pin output to an exact column count
  - --height: pin output to an exact row count
  - --scale: multiply the resolved width by a factor (e.g. 0.5, 2.0)
  - Corrects for monospace font aspect ratio (no vertical stretching)
  - Save plain-text output to file with --output (ANSI codes stripped)


SIZING RULES
------------

  Width resolution (in priority order):
    1. --width, if given
    2. Auto-detected terminal width
    3. Fallback: 80 columns

  --scale is applied on top of the resolved width.

  Height resolution (in priority order):
    1. --height, if given
    2. Computed from aspect ratio: (image_h / image_w) * width * 0.5


BUILDING
--------

    cargo build --release
    ./target/release/asic_art <image>


REQUIREMENTS
------------

  - Rust 1.74 or later (stable)
  - A terminal with ANSI 24-bit color support
    (most modern terminals: iTerm2, Alacritty, Windows Terminal, GNOME Terminal)


PROJECT STRUCTURE
-----------------

  src/
    main.rs       Entry point, wires all modules together
    cli.rs        CLI argument parsing (clap) — --width, --height, --scale, --output
    loader.rs     Image decoding (JPEG/PNG/BMP/TIFF/WebP)
    resizer.rs    Scale image to target dimensions with ResizeOptions
    mapper.rs     Map pixel brightness to Unicode block character
    renderer.rs   Wrap characters in ANSI 24-bit color escape codes
    writer.rs     Write to stdout or plain-text file


LICENSE
-------

    MIT
