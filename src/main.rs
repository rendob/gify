use std::{error::Error, fs, path};

use file::get_video_files;
use gif::make_gif;

mod args;
mod color;
mod file;
mod gif;
mod video;

fn main() -> Result<(), Box<dyn Error>> {
    let args = args::parse_args();

    let input_dir_path = fs::canonicalize(args.input_dir)?;
    let video_paths = get_video_files(&input_dir_path)?;

    let output_dir_path = path::absolute(args.output_dir)?;
    fs::create_dir_all(&output_dir_path)?;
    for video_path in video_paths {
        make_gif(
            &video_path,
            &output_dir_path,
            args.frame_rate,
            &args.progress_bar_color,
            args.progress_bar_height_ratio,
        )?;
    }

    Ok(())
}
