use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "__internal_deny_unknown_fields", serde(deny_unknown_fields))]
pub struct Disposition {
    pub attached_pic: i64,
    pub captions: i64,
    pub clean_effects: i64,
    pub comment: i64,
    pub default: i64,
    pub dependent: i64,
    pub descriptions: i64,
    pub dub: i64,
    pub forced: i64,
    pub hearing_impaired: i64,
    pub karaoke: i64,
    pub lyrics: i64,
    pub metadata: i64,
    pub non_diegetic: i64,
    pub original: i64,
    pub still_image: i64,
    pub timed_thumbnails: i64,
    pub visual_impaired: i64,
}
