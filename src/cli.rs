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
}

pub fn parse() -> Args {
    Args::parse()
}
