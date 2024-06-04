#[cfg(feature = "chapters")]
use crate::Chapter;
#[cfg(feature = "format")]
use crate::Format;
#[cfg(feature = "streams")]
use crate::Stream;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "__internal_deny_unknown_fields", serde(deny_unknown_fields))]
/// FfProbe parsed
pub struct FfProbe {
    #[cfg(feature = "streams")]
    /// Streams of file
    pub streams: Vec<Stream>,
    #[cfg(feature = "chapters")]
    /// Chapters of file
    pub chapters: Vec<Chapter>,
    #[cfg(feature = "format")]
    /// Format of file
    pub format: Format,
}
