use std::{error::Error, fs, path};

use file::get_video_files;
use video::get_video_info;

mod args;
mod color;
mod file;
mod video;

fn main() -> Result<(), Box<dyn Error>> {
    let args = args::parse_args();

    let input_dir_path = fs::canonicalize(args.input_dir)?;
    let video_paths = get_video_files(&input_dir_path)?;

    let output_dir_path = path::absolute(args.output_dir)?;
    fs::create_dir_all(&output_dir_path)?;
    for video_path in video_paths {
        let video_info = get_video_info(&video_path)?;
        println!("{:?}", video_info);
    }

    Ok(())
}
