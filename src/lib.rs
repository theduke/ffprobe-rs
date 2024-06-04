//! Simple wrapper for the [ffprobe](https://ffmpeg.org/ffprobe.html) CLI utility,
//! which is part of the ffmpeg tool suite.
//!
//! This crate allows retrieving typed information about media files (images and videos)
//! by invoking `ffprobe` with JSON output options and deserializing the data
//! into convenient Rust types.
//!
//!
//!
//! ```rust
//! match ffprobe::ffprobe("path/to/video.mp4") {
//!    Ok(info) => {
//!        dbg!(info);
//!    },
//!    Err(err) => {
//!        eprintln!("Could not analyze file with ffprobe: {:?}", err);
//!     },
//! }
//!
//! ```

//! ## Features
//! - streams
//! - format
//! - chapters
//! - async
//!

use std::path::Path;
use std::process::Command;
use std::time::Duration;

use error::FfProbeError;
#[cfg(feature = "streams")]
mod attachment_stream;
#[cfg(feature = "streams")]
mod audio_stream;
#[cfg(feature = "chapters")]
mod chapter;
mod config;
#[cfg(feature = "streams")]
mod data_stream;
#[cfg(feature = "streams")]
mod disposition;
pub mod error;
mod ffprobe;
#[cfg(feature = "format")]
mod format;
mod ratio;
#[cfg(feature = "streams")]
mod streams;
#[cfg(feature = "streams")]
mod subtitle_stream;
#[cfg(feature = "streams")]
mod video_stream;

#[cfg(feature = "streams")]
pub use attachment_stream::AttachmentStream;
#[cfg(feature = "streams")]
pub use attachment_stream::AttachmentTags;
#[cfg(feature = "streams")]
pub use audio_stream::AudioStream;
#[cfg(feature = "streams")]
pub use audio_stream::AudioTags;
#[cfg(feature = "chapters")]
pub use chapter::Chapter;
#[cfg(feature = "chapters")]
pub use chapter::ChapterTags;
pub use config::Config;
#[cfg(feature = "streams")]
pub use data_stream::DataStream;
#[cfg(feature = "streams")]
pub use data_stream::DataTags;
#[cfg(feature = "streams")]
pub use disposition::Disposition;
pub use ffprobe::FfProbe;
#[cfg(feature = "format")]
pub use format::Format;
#[cfg(feature = "format")]
pub use format::FormatTags;
pub use ratio::Ratio;
use serde::Deserialize;
use serde::Deserializer;
#[cfg(feature = "streams")]
pub use streams::SideData;
#[cfg(feature = "streams")]
pub use streams::Stream;
#[cfg(feature = "streams")]
pub use streams::StreamKinds;
#[cfg(feature = "streams")]
pub use streams::StreamTags;
#[cfg(feature = "streams")]
pub use subtitle_stream::SubtititleTags;
#[cfg(feature = "streams")]
pub use subtitle_stream::SubtitleStream;
#[cfg(feature = "streams")]
pub use video_stream::VideoStream;
#[cfg(feature = "streams")]
pub use video_stream::VideoTags;
/// Execute ffprobe with default settings and return the extracted data.
///
/// See [`ffprobe_config`] if you need to customize settings.
pub fn ffprobe(path: impl AsRef<Path>) -> Result<FfProbe, FfProbeError> {
    ffprobe_config(Config::new(), path)
}

/// Run ffprobe with a custom config.
/// See [`ConfigBuilder`] for more details.
pub fn ffprobe_config(config: Config, path: impl AsRef<Path>) -> Result<FfProbe, FfProbeError> {
    let path = path.as_ref();
    let mut cmd = Command::new(config.ffprobe_bin);
    // Default args.
    cmd.args(["-v", "quiet", "-print_format", "json"]);
    #[cfg(feature = "chapters")]
    cmd.arg("-show_chapters");
    #[cfg(feature = "format")]
    cmd.arg("-show_format");
    #[cfg(feature = "streams")]
    cmd.arg("-show_streams");

    if config.count_frames {
        cmd.arg("-count_frames");
    }

    cmd.arg(path);

    let out = cmd.output().map_err(FfProbeError::Io)?;

    if !out.status.success() {
        return Err(FfProbeError::Status(out));
    }

    serde_json::from_slice::<FfProbe>(&out.stdout).map_err(FfProbeError::Deserialize)
}

#[cfg(feature = "async")]
pub async fn ffprobe_async(path: impl AsRef<std::path::Path>) -> Result<FfProbe, FfProbeError> {
    ffprobe_async_config(Config::new(), path).await
}

#[cfg(feature = "async")]
pub async fn ffprobe_async_config(
    config: Config,
    path: impl AsRef<Path>,
) -> Result<FfProbe, FfProbeError> {
    let mut cmd = tokio::process::Command::new("ffprobe");
    let path = path.as_ref();
    cmd.args(["-v", "quiet", "-print_format", "json"]);
    #[cfg(feature = "chapters")]
    cmd.arg("-show_chapters");
    #[cfg(feature = "format")]
    cmd.arg("-show_format");
    #[cfg(feature = "streams")]
    cmd.arg("-show_streams");

    if config.count_frames {
        cmd.arg("-count_frames");
    }

    cmd.arg(path);

    let out = cmd.output().await.map_err(FfProbeError::Io)?;

    if !out.status.success() {
        return Err(FfProbeError::Status(out));
    }

    serde_json::from_slice::<FfProbe>(&out.stdout).map_err(FfProbeError::Deserialize)
}

pub fn option_string_to_duration<'de, D>(deserializer: D) -> Result<Option<Duration>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(s) => s
            .parse::<f64>()
            .map(Duration::from_secs_f64)
            .map(Some)
            .map_err(serde::de::Error::custom),
        None => Ok(None),
    }
}
