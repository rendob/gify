use std::{
    error::Error,
    fs::{self, File},
    io::BufWriter,
    path::{Path, PathBuf},
    process::Command,
};

use crate::{
    color::Color,
    video::{VideoInfo, get_video_info},
};

fn create_bar_image(
    video_info: &VideoInfo,
    output_dir_path: &Path,
    color: &Color,
    height_ratio: f64,
) -> Result<PathBuf, Box<dyn Error>> {
    let tmp_bar_image_path = output_dir_path.join("tmp_bar.png");
    let image_width = video_info.width;
    let image_height = (f64::from(video_info.height) * height_ratio).ceil() as u32;

    let image_file = File::create(&tmp_bar_image_path)?;
    let w = &mut BufWriter::new(image_file);

    let mut encoder = png::Encoder::new(w, image_width, image_height);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header()?;
    let pixels = color
        .rgba()
        .repeat((image_width * image_height).try_into()?);
    writer.write_image_data(&pixels)?;

    Ok(tmp_bar_image_path)
}

pub fn make_gif(
    video_path: &Path,
    output_dir_path: &Path,
    frame_rate: u16,
    progress_bar_color: &Color,
    progress_bar_height_ratio: f64,
) -> Result<(), Box<dyn Error>> {
    let gif_path = {
        let mut path = output_dir_path.join(video_path.file_name().unwrap());
        path.set_extension("gif");
        path
    };
    if gif_path.exists() {
        return Ok(());
    }

    let video_info = get_video_info(video_path)?;
    let tmp_bar_image_path = create_bar_image(
        &video_info,
        output_dir_path,
        progress_bar_color,
        progress_bar_height_ratio,
    )?;

    Command::new("ffmpeg")
        .args([
            "-i",
            video_path.to_str().unwrap(),
            "-i",
            tmp_bar_image_path.to_str().unwrap(),
            "-filter_complex",
            &format!(
                "[1][0]scale=rw:ih[overlay];[0][overlay]overlay=x=-W*(1-n/{}):y=0",
                video_info.frame_count,
            ),
            "-r",
            &frame_rate.to_string(),
            gif_path.to_str().unwrap(),
        ])
        .spawn()?
        .wait()?;

    fs::remove_file(&tmp_bar_image_path)?;

    Ok(())
}
