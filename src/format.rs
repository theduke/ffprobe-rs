use std::time::Duration;

use crate::{
    option_string_to_duration,
    streams::{option_string_to_int, string_to_int},
};

#[derive(Default, Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
/// Parsed Format
pub struct Format {
    /// Filename
    pub filename: String,
    /// Number of streams
    pub nb_streams: u64,
    /// Number of programs
    pub nb_programs: u64,
    // Number of stream groups
    pub nb_stream_groups: u64,
    /// eg. matroska,webm
    pub format_name: String,
    /// eg. Matroska / WebM
    pub format_long_name: String,
    #[serde(deserialize_with = "option_string_to_duration", default)]
    pub start_time: Option<Duration>,
    /// Length in seconds
    #[serde(deserialize_with = "option_string_to_duration", default)]
    pub duration: Option<Duration>,
    // FIXME: wrap with Option<_> on next semver breaking release.
    #[serde(default)]
    /// Size in bytes
    #[serde(deserialize_with = "string_to_int")]
    pub size: i64,
    #[serde(deserialize_with = "option_string_to_int", default)]
    pub bit_rate: Option<i64>,
    ///value from 0-100
    pub probe_score: u64,
    /// File Metadata
    pub tags: Option<FormatTags>,
}

#[derive(Default, Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
/// Metadata Tags of format
pub struct FormatTags {
    #[serde(rename = "WMFSDKNeeded")]
    pub wmfsdkneeded: Option<String>,
    #[serde(rename = "DeviceConformanceTemplate")]
    pub device_conformance_template: Option<String>,
    #[serde(rename = "WMFSDKVersion")]
    pub wmfsdkversion: Option<String>,
    #[serde(rename = "IsVBR")]
    pub is_vbr: Option<String>,
    #[serde(alias = "MAJOR_BRAND")]
    pub major_brand: Option<String>,
    #[serde(alias = "MINOR_VERSION")]
    pub minor_version: Option<String>,
    #[serde(alias = "COMPATIBLE_BRANDS")]
    pub compatible_brands: Option<String>,
    #[serde(alias = "ENCODER")]
    pub encoder: Option<String>,
    #[serde(alias = "MOVIE/ENCODER")]
    pub encoder_: Option<String>,
    #[serde(alias = "ARTIST")]
    pub artist: Option<String>,
    #[serde(alias = "COMMENT")]
    pub comment: Option<String>,
    #[serde(rename = "SUBJECT")]
    pub subject: Option<String>,
    #[serde(rename = "PRODUCT")]
    pub product: Option<String>,
    #[serde(rename = "IRTD")]
    pub irtd: Option<String>,
    pub title: Option<String>,
    #[serde(rename = "COPYRIGHT")]
    pub copyright: Option<String>,
    #[serde(rename = "SOFTWARE")]
    pub software: Option<String>,
    #[serde(rename = "LANGUAGE")]
    pub language: Option<String>,
    pub track: Option<String>,
    #[serde(rename = "TDTG")]
    pub tdtg: Option<String>,
    #[serde(alias = "ENCODED_BY")]
    pub encoded_by: Option<String>,
    pub date: Option<String>,
    #[serde(rename = "TLEN")]
    pub tlen: Option<String>,
    #[serde(rename = "Encoded by")]
    pub encoded_by_: Option<String>,
    #[serde(rename = "DESCRIPTION")]
    pub description: Option<String>,
    #[serde(rename = "Source")]
    pub source: Option<String>,
    #[serde(rename = "IMDB")]
    pub imdb: Option<String>,
    #[serde(alias = "CREATION_TIME")]
    pub creation_time: Option<String>,
    #[serde(rename = "TMDB")]
    pub tmdb: Option<String>,
    pub genre: Option<String>,
    pub album: Option<String>,
    #[serde(flatten)]
    sony_xdcam: SonyXDCAM,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Default, Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
struct SonyXDCAM {
    pub operational_pattern_ul: Option<String>,
    pub uid: Option<String>,
    pub generation_uid: Option<String>,
    pub company_name: Option<String>,
    pub product_name: Option<String>,
    pub product_version: Option<String>,
    pub product_uid: Option<String>,
    pub modification_date: Option<String>,
    pub material_package_umid: Option<String>,
    pub timecode: Option<String>,
}
