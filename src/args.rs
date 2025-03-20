use clap::{Error, Parser, builder::ValueParser, error::ErrorKind};

use crate::color::Color;

fn parse_color_code(color_code: &str) -> Result<Color, Error> {
    Color::from(color_code).map_err(|_| Error::new(ErrorKind::ValueValidation))
}

/// convert video to gif with progress bar.
#[derive(Parser, Debug)]
pub struct Args {
    /// path of the directory that contains videos
    pub input_dir: String,

    /// path of output directory
    pub output_dir: String,

    /// frame rate of output gif
    #[arg(short = 'r', long, default_value_t = 10)]
    pub frame_rate: u16,

    /// progress bar color. "red", "green", "blue", or arbitrary color code in rgba
    #[arg(short = 'c', long, default_value="blue", value_parser=ValueParser::new(parse_color_code))]
    pub progress_bar_color: Color,

    /// ratio of progress bar height to image height
    #[arg(short, long, default_value_t = 0.01)]
    pub progress_bar_height_ratio: f64,
}

pub fn parse_args() -> Args {
    Args::parse()
}
