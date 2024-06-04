use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::streams::{option_string_to_int, string_to_int, StreamTags};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Tags specific for audio
pub struct AudioTags {
    #[serde(flatten)]
    pub tags: StreamTags,
    #[serde(rename = "ENCODER_OPTIONS")]
    pub encoder_options: Option<String>,
    #[serde(rename = "SOURCE_ID")]
    pub source_id: Option<String>,
    #[serde(rename = "COMMENT")]
    pub comment: Option<String>,
    #[serde(deserialize_with = "option_string_to_int", default)]
    pub track: Option<i64>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "__internal_deny_unknown_fields", serde(deny_unknown_fields))]
/// Stream of type audio
pub struct AudioStream {
    /// The number of bits per sample in the audio stream.
    pub bits_per_sample: i64,
    /// The layout of audio channels.
    /// eg. stereo
    pub channel_layout: Option<String>,
    /// number of channels
    /// eg. 2
    pub channels: i64,
    /// The initial padding in the audio stream.
    /// Padding in audio streams refers to extra bits or bytes added to the beginning of audio data to align it with specific boundaries or to provide some additional space for processing.
    pub initial_padding: i64,
    /// The sample format of the audio stream.
    /// Smple format defines how each audio sample is represented in binary form. It describes the encoding method and the number of bits used to represent each sample.
    ///
    /// Common sample formats include:
    /// s16: Signed 16-bit integer
    /// s32: Signed 32-bit integer
    /// flt: Floating point
    /// dbl: Double precision floating point
    /// todo: enum
    pub sample_fmt: String,
    /// The sample rate of the audio stream.
    /// eg. 44100 Hz
    #[serde(deserialize_with = "string_to_int")]
    pub sample_rate: i64,
    /// Bit rate of the video stream.
    /// The bit_rate represents the number of bits that are processed per unit of time in the video stream. It is a measure of the video stream's data rate, indicating how much data is encoded for each second of video.
    #[serde(deserialize_with = "option_string_to_int", default)]
    pub bit_rate: Option<i64>,
    /// Long name of the codec used for the video stream.
    pub codec_long_name: String,
    /// Short name of the codec used for the video stream.
    /// Example: h264
    pub codec_name: String,
    /// Duration of the video stream in timestamp units.
    pub duration_ts: Option<u64>,
    /// Profile of the codec used for the video stream (e.g., Main, High).
    // todo: enum
    pub profile: Option<String>,
    ///  This specifies the number of bits used to represent each component of the pixel. For example, in an 8-bit raw sample, each color component (e.g., red, green, and blue in an RGB format) is represented by 8 bits, allowing 256 different levels per component.
    #[serde(deserialize_with = "option_string_to_int", default)]
    pub bits_per_raw_sample: Option<i64>,
    /// Metadata tags associated with the video stream.
    pub tags: Option<AudioTags>,
    #[cfg(feature = "__internal_deny_unknown_fields")]
    codec_type: Option<serde_json::Value>,
    #[cfg(feature = "__internal_deny_unknown_fields")]
    start_time: Option<serde_json::Value>,
    #[cfg(feature = "__internal_deny_unknown_fields")]
    duration: Option<serde_json::Value>,
}
