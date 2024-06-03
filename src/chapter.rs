use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "__internal_deny_unknown_fields", serde(deny_unknown_fields))]
/// Chapter parsed
pub struct Chapter {
    /// This is an identifier for the chapter. It's a unique number that distinguishes this chapter from others.
    pub id: i64,
    /// This represents the time base of the chapter, which is a rational number. Time base is used to convert time stamps into seconds. It's usually formatted as a fraction, like "1/1000", meaning each unit in the time stamps is 1/1000 of a second.
    pub time_base: String,
    /// This is the start time of the chapter, in units of time_base. To get the start time in seconds, you'd multiply this value by the time base.
    pub start: i64,
    /// This is the start time of the chapter in a human-readable format s
    pub start_time: String,
    /// This is the end time of the chapter, in units of time_base. Similar to start, this can be converted to seconds.
    pub end: i64,
    /// This is the end time of the chapter in a human-readable format s
    pub end_time: String,
    /// This holds additional metadata tags associated with the chapter, such as its title.
    pub tags: ChapterTags,
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
/// Tags for chapter
pub struct ChapterTags {
    /// This is the title of the chapter. Titles can provide descriptive names for chapters, such as "Introduction" or "Chapter 1: Getting Started".
    pub title: String,
}
