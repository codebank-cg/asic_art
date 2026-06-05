asic_art
========

A fast command-line tool written in Rust that renders images, video files, and
webcam streams as colored Unicode ASCII art directly in your terminal.


USAGE
-----

    asic_art <image> [OPTIONS]
    asic_art --video <file> [OPTIONS]
    asic_art --cam [<index>] [OPTIONS]
    asic_art <image> --output result.txt
    asic_art <image> --width 120
    asic_art <image> --scale 0.5
    asic_art <image> --width 80 --height 40
    asic_art <image> --bw
    asic_art --video video.mp4 --width 100
    asic_art --cam --fps 30


OPTIONS
-------

    <image>              Path to the input image (JPEG, PNG, BMP, TIFF, WebP)

    --video <file>       Play a video file (MP4, MKV, WebM, MOV, AVI, ...)
    --cam [<index>]      Stream from webcam (device index, default 0)

    --output, -o <file>  Write plain-text output (ANSI codes stripped) to a file
                           (not usable with --video or --cam)
    --width <N>          Set output width in columns (≥ 1, default: terminal width)
    --height <N>         Set output height in rows (≥ 1, default: aspect-ratio corrected)
    --scale <F>          Scale factor on the resolved width (> 0), e.g. 0.5, 2.0
    --fps <F>            Webcam capture frame rate (default: 15, does not affect video files)
    --bw                 Render in grayscale (black-and-white) using luminance only
    --help               Print help

NOTES
-----

  --video and --cam are mutually exclusive with each other, with a positional
  <image> argument, and with --output. Only one input mode at a time.


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

    # Grayscale output
    asic_art photo.jpg --bw

    # Scale up and save to file
    asic_art logo.png --scale 2.0 --output logo.txt

    # Explicit dimensions saved to file
    asic_art banner.webp --width 160 --height 50 --output banner.txt

    # Play a video file fullscreen in the terminal
    asic_art --video movie.mp4

    # Stream from the default webcam
    asic_art --cam

    # Stream from webcam device 1 at 30 fps
    asic_art --cam 1 --fps 30 --width 100


VIDEO / WEBCAM CONTROLS
-----------------------

    Left/Right arrows    Seek backward/forward 10 seconds (video files only)
    q                    Quit playback
    Ctrl-C               Quit playback


SUPPORTED FORMATS
-----------------

    Still images: JPEG, PNG, BMP, TIFF, WebP
    Video files:  MP4, MKV, WebM, MOV, AVI, ... (anything ffmpeg can decode)
    Webcam:       Any V4L2 / AVFoundation device (via nokhwa)


FEATURES
--------

  - Unicode block characters (░ ▒ ▓ █) for smooth brightness shading
  - ANSI 24-bit color — each character matches the original pixel color
  - Auto-fits to your terminal width (default behavior)
  - --width: pin output to an exact column count
  - --height: pin output to an exact row count
  - --scale: multiply the resolved width by a factor (e.g. 0.5, 2.0)
  - --bw: grayscale mode using perceptual luminance only
  - --video: real-time video playback as ASCII art with synchronized audio
  - --cam: live webcam streaming as ASCII art
  - --fps: configure webcam frame rate
  - Audio playback during video (via ffmpeg pipe + rodio)
  - Keyboard seek (Left/Right arrows) and quit (q) during video playback
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
    ./target/release/asic_art --video movie.mp4


REQUIREMENTS
------------

  - Rust 1.74 or later (stable)
  - ffmpeg on PATH (required for audio playback during video)
  - A terminal with ANSI 24-bit color support
    (most modern terminals: iTerm2, Alacritty, Windows Terminal, GNOME Terminal)


PROJECT STRUCTURE
-----------------

  src/
    main.rs       Entry point, selects still-image / video / webcam path
    cli.rs        CLI argument parsing (clap) — all flags
    lib.rs        Re-exports all public library modules
    loader.rs     Image decoding (JPEG/PNG/BMP/TIFF/WebP) with RGBA compositing
    resizer.rs    Scale image to target dimensions with ResizeOptions
    mapper.rs     Map pixel brightness to Unicode block character
    renderer.rs   Wrap characters in ANSI 24-bit color escape codes; grayscale mode
    writer.rs     Write to stdout or plain-text file
    source.rs     FrameSource trait + VideoFileSource + WebcamSource implementations
    player.rs     Video/webcam playback loop (raw terminal, frame timing, seek, quit)
    audio.rs      Synchronized audio playback via ffmpeg pipe + rodio


LICENSE
-------

    MIT
