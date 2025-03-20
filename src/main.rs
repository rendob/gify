use std::{error::Error, fs};

use file::get_video_files;

mod args;
mod color;
mod file;

fn main() -> Result<(), Box<dyn Error>> {
    let args = args::parse_args();

    let input_dir_path = fs::canonicalize(args.input_dir)?;
    let video_paths = get_video_files(&input_dir_path)?;

    println!("{:?}", video_paths);

    Ok(())
}
