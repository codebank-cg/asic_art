use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "asic_art", about = "Render an image as Unicode ASCII art in your terminal")]
pub struct Args {
    /// Path to the input image (JPEG, PNG, BMP, TIFF, WebP)
    #[arg(required_unless_present_any = ["video", "cam"])]
    pub input: Option<PathBuf>,

    /// Play a video file (MP4, MKV, WebM, MOV, AVI, …)
    #[arg(long, conflicts_with_all = ["input", "cam", "output"])]
    pub video: Option<PathBuf>,

    /// Stream from webcam (device index, default 0)
    #[arg(long, conflicts_with_all = ["input", "video", "output"])]
    pub cam: Option<Option<u32>>,

    /// Write plain-text output (ANSI codes stripped) to this file — not usable with --video or --cam
    #[arg(long, short)]
    pub output: Option<PathBuf>,

    /// Output width in columns (overrides auto-detected terminal width)
    #[arg(long, value_parser = clap::value_parser!(u32).range(1..))]
    pub width: Option<u32>,

    /// Output height in rows (overrides aspect-ratio-corrected height)
    #[arg(long, value_parser = clap::value_parser!(u32).range(1..))]
    pub height: Option<u32>,

    /// Scale factor applied to the resolved width, e.g. 0.5 = half, 2.0 = double
    #[arg(long, value_parser = parse_positive_f32)]
    pub scale: Option<f32>,

    /// Webcam capture fps (default 15); does not affect video file playback speed
    #[arg(long, value_parser = parse_positive_f32, default_value = "15")]
    pub fps: f32,

    /// Render in grayscale (black-and-white) using luminance levels only
    #[arg(long)]
    pub bw: bool,
}

fn parse_positive_f32(s: &str) -> Result<f32, String> {
    let v: f32 = s.parse().map_err(|_| format!("'{s}' is not a valid number"))?;
    if v > 0.0 {
        Ok(v)
    } else {
        Err("value must be greater than 0".to_string())
    }
}

pub fn parse() -> Args {
    Args::parse()
}
