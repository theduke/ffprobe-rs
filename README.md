# ffprobe-rs

Simple wrapper for the [ffprobe](https://ffmpeg.org/ffprobe.html) CLI utility,
which is part of the ffmpeg tool suite.

This crate allows retrieving typed information about media files (images and videos)
by invoking `ffprobe` with JSON output options and deserializing the data
into convenient Rust types.
