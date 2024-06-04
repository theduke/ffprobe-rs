use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Tags for the type data
pub struct DataTags {
    creation_time: Option<String>,
    language: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "__internal_deny_unknown_fields", serde(deny_unknown_fields))]
/// Stream of type data
pub struct DataStream {
    /// Duration of the video stream in timestamp units.
    pub duration_ts: u64,
    /// Metadata tags associated with the video stream.
    pub tags: DataTags,
    /// Long name of the codec used for the video stream. eg. binary data
    pub codec_long_name: Option<String>,
    /// Short name of the codec used for the video stream.
    /// Example: bin_data
    pub codec_name: Option<String>,
    #[cfg(feature = "__internal_deny_unknown_fields")]
    codec_type: serde_json::Value,
    #[cfg(feature = "__internal_deny_unknown_fields")]
    start_time: Option<serde_json::Value>,
    #[cfg(feature = "__internal_deny_unknown_fields")]
    duration: Option<serde_json::Value>,
}
