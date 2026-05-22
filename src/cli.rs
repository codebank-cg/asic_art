use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "asic_art", about = "Render an image as Unicode ASCII art in your terminal")]
pub struct Args {
    /// Path to the input image (JPEG, PNG, BMP, TIFF, WebP)
    pub input: PathBuf,

    /// Write plain-text output (ANSI codes stripped) to this file
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
}

fn parse_positive_f32(s: &str) -> Result<f32, String> {
    let v: f32 = s.parse().map_err(|_| format!("'{s}' is not a valid number"))?;
    if v > 0.0 {
        Ok(v)
    } else {
        Err("scale must be greater than 0".to_string())
    }
}

pub fn parse() -> Args {
    Args::parse()
}
