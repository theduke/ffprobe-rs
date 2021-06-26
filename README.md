# ffprobe-rs


[![crates.io](https://img.shields.io/crates/v/ffprobe?label=latest)](https://crates.io/crates/ffprobe)
[![Documentation](https://docs.rs/ffprobe/badge.svg?version)](https://docs.rs/ffprobe)

Simple wrapper for the [ffprobe](https://ffmpeg.org/ffprobe.html) CLI utility,
which is part of the ffmpeg tool suite.

This crate allows retrieving typed information about media files (images and videos)
by invoking `ffprobe` with JSON output options and deserializing the data
into convenient Rust types.

## Example

```rust
fn main() {
    match ffprobe::ffprobe("path/to/video.mp4") {
        Ok(info) => {
	    dbg!(info);
        },
	Err(err) => {
	    eprintln!("Could not analyze file with ffprobe: {:?}", err);
	}
    }
}
```
