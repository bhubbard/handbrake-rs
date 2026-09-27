use thiserror::Error;

#[derive(Error, Debug)]
pub enum HandBrakeError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Preset error: {0}")]
    PresetNotFound(String),

    #[error("Invalid track or stream: {0}")]
    InvalidTrack(String),

    #[error("Transcoding error: {0}")]
    Transcode(String),

    #[error("Filter error: {0}")]
    Filter(String),

    #[error("Unsupported codec: {0}")]
    UnsupportedCodec(String),
}

pub type Result<T> = std::result::Result<T, HandBrakeError>;
