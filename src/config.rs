use crate::{error::FfProbeError, ffprobe::FfProbe, ffprobe_config};

/// ffprobe configuration.
///
/// Use [`Config::new`] for constructing a new config.
#[derive(Clone, Debug)]
pub struct Config {
    pub(crate) count_frames: bool,
    pub(crate) ffprobe_bin: std::path::PathBuf,
}

impl Config {
    /// Construct a new Config.
    pub fn new() -> Config {
        Config {
            count_frames: false,
            ffprobe_bin: "ffprobe".into(),
        }
    }

    /// Enable the -count_frames setting.
    /// Will fully decode the file and count the frames.
    /// Frame count will be available in [`Stream::nb_read_frames`].
    pub fn count_frames(mut self, count_frames: bool) -> Self {
        self.count_frames = count_frames;
        self
    }

    /// Specify which binary name (e.g. `"ffprobe-6"`) or path (e.g. `"/opt/bin/ffprobe"`) to use
    /// for executing `ffprobe`.
    pub fn ffprobe_bin(mut self, ffprobe_bin: impl AsRef<std::path::Path>) -> Self {
        self.ffprobe_bin = ffprobe_bin.as_ref().to_path_buf();
        self
    }

    /// Run ffprobe with the config produced by this builder.
    pub fn run(self, path: impl AsRef<std::path::Path>) -> Result<FfProbe, FfProbeError> {
        ffprobe_config(self, path)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
