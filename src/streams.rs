use std::time::Duration;

use chrono::{DateTime, FixedOffset, NaiveDateTime, NaiveTime, Timelike};
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;

use crate::{
    attachment_stream::AttachmentStream, audio_stream::AudioStream, data_stream::DataStream,
    disposition::Disposition, ratio::Ratio, subtitle_stream::SubtitleStream,
    video_stream::VideoStream,
};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
/// Stream parsed
pub struct Stream {
    /// The identifier of the stream, if available.
    pub id: Option<String>,
    // The index of the stream.
    pub index: u64,
    /// Disposition flags indicating various properties of the stream.
    pub disposition: Disposition,
    /// The average frame rate of the stream.
    pub avg_frame_rate: Ratio,
    /// The codec_tag field represents a numeric identifier associated with the codec used in the stream. It is often an integer value assigned to a specific codec format, allowing programs to quickly identify the codec type without needing to parse through codec-specific headers extensively. These tags are usually defined by standards organizations or codec developers.
    /// For example, in the context of video codecs, a codec tag might represent the codec used for encoding the video stream, such as H.264 (codec tag 0x21) or MPEG-4 Visual (codec tag 0x20).
    #[serde(deserialize_with = "string_to_bytes")]
    pub codec_tag: Vec<u8>,
    /// The time base of the stream. eg. 1/1000
    pub time_base: Ratio,
    /// The start presentation timestamp (PTS) of the stream.
    /// ptr * timebase = start in seconds
    pub start_pts: i64,
    #[serde(default)]
    /// A list of side data associated with the stream.
    pub side_data_list: Vec<SideData>,
    /// The size of the extra data associated with the stream, if available.
    pub extradata_size: Option<i64>,
    /// The real frame rate of the stream.
    pub r_frame_rate: Ratio,
    /// The total number of frames in the stream, if available.
    #[serde(deserialize_with = "option_string_to_int", default)]
    pub nb_frames: Option<i64>,
    /// Number of frames seen by the decoder.
    /// Requires full decoding and is only available if the 'count_frames'
    /// setting was enabled.
    #[serde(deserialize_with = "option_string_to_int", default)]
    pub nb_read_frames: Option<i64>,
    #[cfg(feature = "__internal_deny_unknown_fields")]
    codec_tag_string: Value,
    #[serde(flatten)]
    pub stream: StreamKinds,
}

impl Stream {
    pub fn start_time(&self) -> f64 {
        (self.start_pts * self.time_base.numerator() as i64) as f64
            / (self.time_base.denominator() as f64)
    }
    pub fn duration(&self) -> Option<Duration> {
        match &self.stream {
            StreamKinds::Audio(v) => v.duration_ts,
            StreamKinds::Video(v) => v.duration_ts,
            StreamKinds::Subtitle(sub) => sub.duration_ts,
            StreamKinds::Attachment(attach) => Some(attach.duration_ts),
            StreamKinds::Data(v) => Some(v.duration_ts),
        }
        .map(|d| {
            Duration::from_millis(
                ((d * self.time_base.numerator()) as f64 / self.time_base.denominator() as f64
                    * 1000.) as u64,
            )
        })
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
/// codec_type as enum
pub enum StreamKinds {
    Audio(AudioStream),
    Video(VideoStream),
    Subtitle(SubtitleStream),
    Attachment(AttachmentStream),
    Data(DataStream),
}

impl Serialize for StreamKinds {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            StreamKinds::Audio(ref __field0) => Serialize::serialize(__field0, serializer),
            StreamKinds::Video(ref __field0) => Serialize::serialize(__field0, serializer),
            StreamKinds::Subtitle(ref __field0) => Serialize::serialize(__field0, serializer),
            StreamKinds::Attachment(ref __field0) => Serialize::serialize(__field0, serializer),
            StreamKinds::Data(ref __field0) => Serialize::serialize(__field0, serializer),
        }
    }
}

#[derive(Deserialize, Serialize)]
struct StreamKindInfo {
    codec_type: String,
}

impl<'de> Deserialize<'de> for StreamKinds {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let __content = <serde::__private::de::Content as Deserialize>::deserialize(deserializer)?;
        let deserializer =
            serde::__private::de::ContentRefDeserializer::<D::Error>::new(&__content);
        let mut err = None;
        if let Ok(kind) = StreamKindInfo::deserialize(deserializer) {
            match kind.codec_type.as_str() {
                "audio" => {
                    match Result::map(
                        <AudioStream as Deserialize>::deserialize(deserializer),
                        StreamKinds::Audio,
                    ) {
                        Ok(__ok) => return Ok(__ok),
                        Err(e) => err = Some(e.to_string()),
                    }
                }
                "video" => {
                    match Result::map(
                        <VideoStream as Deserialize>::deserialize(deserializer),
                        StreamKinds::Video,
                    ) {
                        Ok(__ok) => return Ok(__ok),
                        Err(e) => err = Some(e.to_string()),
                    }
                }
                "attachment" => {
                    match Result::map(
                        <AttachmentStream as Deserialize>::deserialize(deserializer),
                        StreamKinds::Attachment,
                    ) {
                        Ok(__ok) => return Ok(__ok),
                        Err(e) => err = Some(e.to_string()),
                    }
                }
                "data" => {
                    match Result::map(
                        <DataStream as Deserialize>::deserialize(deserializer),
                        StreamKinds::Data,
                    ) {
                        Ok(__ok) => return Ok(__ok),
                        Err(e) => err = Some(e.to_string()),
                    }
                }
                "subtitle" => {
                    match Result::map(
                        <SubtitleStream as Deserialize>::deserialize(deserializer),
                        StreamKinds::Subtitle,
                    ) {
                        Ok(__ok) => return Ok(__ok),
                        Err(e) => err = Some(e.to_string()),
                    };
                }
                _ => {}
            }
        }
        let msg = Value::deserialize(deserializer);

        let msg = match err {
            Some(v) => match msg {
                Ok(vv) => format!("StreamKinds: {} {:#?}", v, vv),
                Err(_) => format!("StreamKinds: {}", v),
            },
            None => match msg {
                Ok(v) => format!(
                    "data did not match any variant of untagged enum StreamKinds: {:#?}",
                    v
                ),
                Err(_) => "data did not match any variant of untagged enum StreamKinds".to_string(),
            },
        };

        Err(de::Error::custom(msg))
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "__internal_deny_unknown_fields", serde(deny_unknown_fields))]
pub struct SideData {
    side_data_type: Option<String>,
    service_type: Option<i64>,
    dv_version_major: Option<i64>,
    dv_version_minor: Option<i64>,
    dv_profile: Option<i64>,
    dv_level: Option<i64>,
    rpu_present_flag: Option<i64>,
    el_present_flag: Option<i64>,
    bl_present_flag: Option<i64>,
    dv_bl_signal_compatibility_id: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Stream tags for video, audio, subtitle
pub struct StreamTags {
    #[serde(rename = "BPS")]
    #[serde(deserialize_with = "option_string_to_int", default)]
    pub bps: Option<i64>,
    #[serde(rename = "DURATION")]
    #[serde(deserialize_with = "option_chronostring_to_duration", default)]
    pub duration: Option<Duration>,
    #[serde(rename = "NUMBER_OF_BYTES")]
    #[serde(deserialize_with = "option_string_to_int", default)]
    pub number_of_bytes: Option<i64>,
    #[serde(rename = "NUMBER_OF_FRAMES")]
    #[serde(deserialize_with = "option_string_to_int", default)]
    pub number_of_frames: Option<i64>,
    #[serde(rename = "_STATISTICS_TAGS")]
    pub statistics_tags: Option<String>,
    #[serde(rename = "_STATISTICS_WRITING_APP")]
    pub statistics_writing_app: Option<String>,
    #[serde(rename = "_STATISTICS_WRITING_DATE_UTC")]
    #[serde(deserialize_with = "option_string_to_naivedatetime", default)]
    pub statistics_writing_date_utc: Option<NaiveDateTime>,
    #[serde(alias = "HANDLER_NAME")]
    pub handler_name: Option<String>,
    #[serde(deserialize_with = "option_string_to_datetime", default)]
    pub creation_time: Option<DateTime<FixedOffset>>,
    #[serde(alias = "ENCODER")]
    pub encoder: Option<String>,
    #[serde(alias = "VENDOR_ID")]
    pub vendor_id: Option<String>,
    pub title: Option<String>,
    pub language: Option<String>,
}

pub fn option_string_to_int<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(s) => s.parse::<i64>().map(Some).map_err(serde::de::Error::custom),
        None => Ok(None),
    }
}

pub fn string_to_int<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.parse::<i64>().map_err(serde::de::Error::custom)
}

pub fn string_to_bytes<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let hex_str = String::deserialize(deserializer)?;

    let trimmed_str = if let Some(stripped) = hex_str.strip_prefix("0x") {
        stripped
    } else {
        &hex_str
    };

    if trimmed_str.len() % 2 != 0 {
        return Err(serde::de::Error::custom(
            "Hex string must have an even number of digits",
        ));
    }

    trimmed_str
        .as_bytes()
        .chunks(2)
        .map(|chunk| {
            let hex_digit = std::str::from_utf8(chunk).map_err(|_| "Invalid UTF-8 sequence")?;

            u8::from_str_radix(hex_digit, 16).map_err(|_| "Conversion error")
        })
        .collect::<Result<Vec<_>, _>>()
        .map_err(serde::de::Error::custom)
}

pub fn option_string_to_bool<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(s) => s
            .parse::<bool>()
            .map(Some)
            .map_err(serde::de::Error::custom),
        None => Ok(None),
    }
}

pub fn option_chronostring_to_duration<'de, D>(
    deserializer: D,
) -> Result<Option<Duration>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(s) => NaiveTime::parse_from_str(&s, "%H:%M:%S%.f")
            .map(|time| {
                let seconds = time.hour() * 3600 + time.minute() * 60 + time.second();
                Duration::new(seconds as u64, time.nanosecond())
            })
            .map(Some)
            .map_err(serde::de::Error::custom),
        None => Ok(None),
    }
}

pub fn option_string_to_naivedatetime<'de, D>(
    deserializer: D,
) -> Result<Option<NaiveDateTime>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(s) => NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S")
            .map(Some)
            .map_err(serde::de::Error::custom),
        None => Ok(None),
    }
}

pub fn option_string_to_datetime<'de, D>(
    deserializer: D,
) -> Result<Option<DateTime<FixedOffset>>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(s) => DateTime::parse_from_rfc3339(&s)
            .map(Some)
            .map_err(serde::de::Error::custom),
        None => Ok(None),
    }
}
