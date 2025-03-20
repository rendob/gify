use std::{error::Error, path::Path, process::Command};

use serde::Deserialize;

#[derive(Debug)]
pub struct VideoInfo {
    pub width: u32,
    pub height: u32,
    pub frame_count: u32,
}

#[derive(Debug, Clone, Deserialize)]
struct Stream {
    width: Option<u32>,
    height: Option<u32>,
    nb_frames: String,
}

#[derive(Debug, Deserialize)]
struct Output {
    streams: Vec<Stream>,
}

pub fn get_video_info(video_path: &Path) -> Result<VideoInfo, Box<dyn Error>> {
    let result = Command::new("ffprobe")
        .args([
            "-show_entries",
            "stream=width,height,nb_frames",
            "-of",
            "json",
            video_path.to_str().unwrap(),
        ])
        .output()?;
    let stdout = String::from_utf8(result.stdout)?;

    let output: Output = serde_json::from_str(&stdout)?;
    let Stream {
        width,
        height,
        nb_frames,
    } = &output.streams[0];

    Ok(VideoInfo {
        width: width.unwrap(),
        height: height.unwrap(),
        frame_count: u32::from_str_radix(nb_frames, 10)?,
    })
}
