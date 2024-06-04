use std::fmt::Display;

#[derive(Debug)]
#[non_exhaustive]
pub enum FfProbeError {
    Io(std::io::Error),
    Status(std::process::Output),
    Deserialize(serde_json::Error),
}

impl Display for FfProbeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FfProbeError::Io(e) => e.fmt(f),
            FfProbeError::Status(o) => {
                write!(
                    f,
                    "ffprobe exited with status code {}: {}",
                    o.status,
                    String::from_utf8_lossy(&o.stdout)
                )
            }
            FfProbeError::Deserialize(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for FfProbeError {}
