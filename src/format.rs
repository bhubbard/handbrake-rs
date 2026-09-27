use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ContainerFormat {
    Mp4,
    Mkv,
    Webm,
}

impl ContainerFormat {
    pub fn extension(&self) -> &'static str {
        match self {
            ContainerFormat::Mp4 => "mp4",
            ContainerFormat::Mkv => "mkv",
            ContainerFormat::Webm => "webm",
        }
    }

    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "mp4" | "m4v" => Some(ContainerFormat::Mp4),
            "mkv" => Some(ContainerFormat::Mkv),
            "webm" => Some(ContainerFormat::Webm),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VideoEncoder {
    X264,
    X265,
    SvtAv1,
    Vp9,
    ProRes,
    VtH264,
    VtH265,
}

impl VideoEncoder {
    pub fn as_str(&self) -> &'static str {
        match self {
            VideoEncoder::X264 => "x264",
            VideoEncoder::X265 => "x265",
            VideoEncoder::SvtAv1 => "svt_av1",
            VideoEncoder::Vp9 => "vp9",
            VideoEncoder::ProRes => "prores",
            VideoEncoder::VtH264 => "vt_h264",
            VideoEncoder::VtH265 => "vt_h265",
        }
    }

    pub fn from_str_loose(s: &str) -> Option<Self> {
        match s.to_lowercase().replace('-', "_").as_str() {
            "x264" | "h264" | "avc" => Some(VideoEncoder::X264),
            "x265" | "h265" | "hevc" => Some(VideoEncoder::X265),
            "svt_av1" | "av1" => Some(VideoEncoder::SvtAv1),
            "vp9" => Some(VideoEncoder::Vp9),
            "prores" => Some(VideoEncoder::ProRes),
            "vt_h264" | "videotoolbox_h264" => Some(VideoEncoder::VtH264),
            "vt_h265" | "videotoolbox_hevc" => Some(VideoEncoder::VtH265),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AudioEncoder {
    Aac,
    Opus,
    Flac,
    Mp3,
    Ac3,
    Copy,
}

impl AudioEncoder {
    pub fn as_str(&self) -> &'static str {
        match self {
            AudioEncoder::Aac => "aac",
            AudioEncoder::Opus => "opus",
            AudioEncoder::Flac => "flac",
            AudioEncoder::Mp3 => "mp3",
            AudioEncoder::Ac3 => "ac3",
            AudioEncoder::Copy => "copy",
        }
    }

    pub fn from_str_loose(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "aac" | "ca_aac" => Some(AudioEncoder::Aac),
            "opus" => Some(AudioEncoder::Opus),
            "flac" | "flac16" | "flac24" => Some(AudioEncoder::Flac),
            "mp3" | "lame" => Some(AudioEncoder::Mp3),
            "ac3" | "eac3" => Some(AudioEncoder::Ac3),
            "copy" | "passthrough" => Some(AudioEncoder::Copy),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RateControl {
    ConstantQuality { rf: f32 },
    AverageBitrate { kbps: u32, two_pass: bool },
}

impl Default for RateControl {
    fn default() -> Self {
        RateControl::ConstantQuality { rf: 22.0 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PixelFormat {
    Yuv420p,
    Yuv420p10,
    Yuv422p,
    Yuv444p,
    Rgb24,
}

impl fmt::Display for PixelFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PixelFormat::Yuv420p => write!(f, "yuv420p"),
            PixelFormat::Yuv420p10 => write!(f, "yuv420p10le"),
            PixelFormat::Yuv422p => write!(f, "yuv422p"),
            PixelFormat::Yuv444p => write!(f, "yuv444p"),
            PixelFormat::Rgb24 => write!(f, "rgb24"),
        }
    }
}
