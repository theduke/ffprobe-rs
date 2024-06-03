use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::streams::StreamTags;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Tags specific for subtitles

pub struct SubtititleTags {
    #[serde(flatten)]
    pub tags: StreamTags,
    pub filename: Option<String>,
    pub mimetype: Option<String>,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub bit_rate: Option<String>,
    #[serde(rename = "SOURCE_ID")]
    pub source_id: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "__internal_deny_unknown_fields", serde(deny_unknown_fields))]
/// Stream of type subtitle
pub struct SubtitleStream {
    /// Bit rate of the video stream.
    /// The bit_rate represents the number of bits that are processed per unit of time in the video stream. It is a measure of the video stream's data rate, indicating how much data is encoded for each second of video.
    pub bit_rate: Option<String>,
    /// width of video
    pub width: Option<i64>,
    /// height of video
    pub height: Option<i64>,
    /// Long name of the codec used for the video stream.
    pub codec_long_name: String,
    /// Short name of the codec used for the video stream.
    /// Example: h264
    pub codec_name: String,
    /// Duration of the video stream in timestamp units.
    pub duration_ts: Option<u64>,
    /// Metadata tags associated with the video stream.
    pub tags: Option<SubtititleTags>,
    #[cfg(feature = "__internal_deny_unknown_fields")]
    codec_type: Option<serde_json::Value>,
    #[cfg(feature = "__internal_deny_unknown_fields")]
    start_time: Option<serde_json::Value>,
    #[cfg(feature = "__internal_deny_unknown_fields")]
    duration: Option<serde_json::Value>,
}
