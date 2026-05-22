asic_art
========

A fast command-line tool written in Rust that renders image files as colored
Unicode ASCII art directly in your terminal.


USAGE
-----

    asic_art <image>
    asic_art <image> --output result.txt


EXAMPLES
--------

    asic_art photo.jpg
    asic_art logo.png --output logo.txt
    asic_art banner.webp


SUPPORTED FORMATS
-----------------

    JPEG, PNG, BMP, TIFF, WebP


FEATURES
--------

  - Unicode block characters (░ ▒ ▓ █) for smooth brightness shading
  - ANSI 24-bit color — each character matches the original pixel color
  - Auto-fits to your terminal width
  - Corrects for monospace font aspect ratio (no vertical stretching)
  - Save plain-text output to file with --output (ANSI codes stripped)


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
    cli.rs        CLI argument parsing (clap)
    loader.rs     Image decoding (JPEG/PNG/BMP/TIFF/WebP)
    resizer.rs    Scale image to terminal dimensions with aspect correction
    mapper.rs     Map pixel brightness to Unicode block character
    renderer.rs   Wrap characters in ANSI 24-bit color escape codes
    writer.rs     Write to stdout or plain-text file


LICENSE
-------

    MIT
