# gify

video-to-gif converter

```sh
% gify -h
convert video to gif with progress bar

Usage: gify [OPTIONS] <INPUT_DIR> <OUTPUT_DIR>

Arguments:
  <INPUT_DIR>   path of the directory that contains videos
  <OUTPUT_DIR>  path of output directory

Options:
  -r, --frame-rate <FRAME_RATE>
          frame rate of output gif [default: 10]
  -c, --progress-bar-color <PROGRESS_BAR_COLOR>
          progress bar color. "red", "green", "blue", or arbitrary color code in rgba [default: blue]
  -p, --progress-bar-height-ratio <PROGRESS_BAR_HEIGHT_RATIO>
          ratio of progress bar height to image height [default: 0.01]
  -h, --help
          Print help
```

## Prerequisite

- `ffmpeg` (and `ffprobe`)

## Installation

- Install `gify` to `~/.cargo/bin/`

```sh
cargo install --git https://github.com/rendob/gify
```
