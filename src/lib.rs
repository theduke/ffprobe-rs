//! Simple wrapper for the [ffprobe](https://ffmpeg.org/ffprobe.html) CLI utility,
//! which is part of the ffmpeg tool suite.
//!
//! This crate allows retrieving typed information about media files (images and videos)
//! by invoking `ffprobe` with JSON output options and deserializing the data
//! into convenient Rust types.

pub fn ffprobe(path: impl AsRef<std::path::Path>) -> Result<FfProbe, FfProbeError> {
    let path = path.as_ref();

    let out = std::process::Command::new("ffprobe")
        .args(&[
            "-v",
            "quiet",
            "-show_format",
            "-show_streams",
            "-print_format",
            "json",
        ])
        .arg(path)
        .output()
        .map_err(|err| FfProbeError::new(format!("Could not execute ffprobe: {}", err)))?;

    if !out.status.success() {
        let stderr = String::from_utf8_lossy(&out.stderr);

        Err(FfProbeError::new(format!(
            "ffprobe with with status code {}: {}",
            out.status, stderr
        )))?;
    }

    serde_json::from_slice::<FfProbe>(&out.stdout)
        .map_err(|err| FfProbeError::new(format!("Could not deserialize ffprobe output: {}", err)))
}

#[derive(Debug)]
pub struct FfProbeError {
    message: String,
}

impl FfProbeError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for FfProbeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for FfProbeError {}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(
    feature = "__internal_deny_unknown_fields",
    serde(deny_unknown_fields)
)]
pub struct FfProbe {
    pub streams: Vec<Stream>,
    pub format: Format,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(
    feature = "__internal_deny_unknown_fields",
    serde(deny_unknown_fields)
)]
pub struct Stream {
    pub index: i64,
    pub codec_name: String,
    pub sample_aspect_ratio: Option<String>,
    pub display_aspect_ratio: Option<String>,
    pub color_range: Option<String>,
    pub color_space: Option<String>,
    pub bits_per_raw_sample: Option<String>,
    pub channel_layout: Option<String>,
    pub max_bit_rate: Option<String>,
    pub nb_frames: Option<String>,
    pub codec_long_name: String,
    pub codec_type: String,
    pub codec_time_base: String,
    pub codec_tag_string: String,
    pub codec_tag: String,
    pub sample_fmt: Option<String>,
    pub sample_rate: Option<String>,
    pub channels: Option<i64>,
    pub bits_per_sample: Option<i64>,
    pub r_frame_rate: String,
    pub avg_frame_rate: String,
    pub time_base: String,
    pub start_pts: i64,
    pub start_time: String,
    pub duration_ts: Option<i64>,
    pub duration: Option<String>,
    pub bit_rate: Option<String>,
    pub disposition: Disposition,
    pub tags: Option<StreamTags>,
    pub profile: Option<String>,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub coded_width: Option<i64>,
    pub coded_height: Option<i64>,
    pub closed_captions: Option<i64>,
    pub has_b_frames: Option<i64>,
    pub pix_fmt: Option<String>,
    pub level: Option<i64>,
    pub chroma_location: Option<String>,
    pub refs: Option<i64>,
    pub is_avc: Option<String>,
    pub nal_length: Option<String>,
    pub nal_length_size: Option<String>,
    pub field_order: Option<String>,
    pub id: Option<String>,
    #[serde(default)]
    pub side_data_list: Vec<SideData>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(
    feature = "__internal_deny_unknown_fields",
    serde(deny_unknown_fields)
)]
pub struct SideData {
    pub side_data_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(
    feature = "__internal_deny_unknown_fields",
    serde(deny_unknown_fields)
)]
pub struct Disposition {
    pub default: i64,
    pub dub: i64,
    pub original: i64,
    pub comment: i64,
    pub lyrics: i64,
    pub karaoke: i64,
    pub forced: i64,
    pub hearing_impaired: i64,
    pub visual_impaired: i64,
    pub clean_effects: i64,
    pub attached_pic: i64,
    pub timed_thumbnails: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(
    feature = "__internal_deny_unknown_fields",
    serde(deny_unknown_fields)
)]
pub struct StreamTags {
    pub language: Option<String>,
    pub creation_time: Option<String>,
    pub handler_name: Option<String>,
    pub encoder: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(
    feature = "__internal_deny_unknown_fields",
    serde(deny_unknown_fields)
)]
pub struct Format {
    pub filename: String,
    pub nb_streams: i64,
    pub nb_programs: i64,
    pub format_name: String,
    pub format_long_name: String,
    pub start_time: String,
    pub duration: String,
    pub size: String,
    pub bit_rate: String,
    pub probe_score: i64,
    pub tags: Option<FormatTags>,
}

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(
    feature = "__internal_deny_unknown_fields",
    serde(deny_unknown_fields)
)]
pub struct FormatTags {
    #[serde(rename = "WMFSDKNeeded")]
    pub wmfsdkneeded: Option<String>,
    #[serde(rename = "DeviceConformanceTemplate")]
    pub device_conformance_template: Option<String>,
    #[serde(rename = "WMFSDKVersion")]
    pub wmfsdkversion: Option<String>,
    #[serde(rename = "IsVBR")]
    pub is_vbr: Option<String>,
    pub major_brand: Option<String>,
    pub minor_version: Option<String>,
    pub compatible_brands: Option<String>,
    pub creation_time: Option<String>,
    pub encoder: Option<String>,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub date: Option<String>,
    pub album_artist: Option<String>,
    pub copyright: Option<String>,
    pub genre: Option<String>,
    pub track: Option<String>,
    pub disc: Option<String>,
}
