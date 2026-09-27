use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::filter::crop::CropBounds;
use crate::filter::deinterlace::DeinterlaceMode;
use crate::filter::rotate::Rotation;
use crate::format::{AudioEncoder, RateControl, VideoEncoder};
use crate::preset::Preset;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioTrackConfig {
    pub track_index: usize,
    pub encoder: AudioEncoder,
    pub bitrate: u32,
    pub channels: u32,
    pub sample_rate: u32,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubtitleTrackConfig {
    pub track_index: usize,
    pub burn_in: bool,
    pub default: bool,
    pub language: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscodeJob {
    pub id: String,
    pub source: PathBuf,
    pub destination: PathBuf,
    pub preset_name: String,
    pub video_encoder: VideoEncoder,
    pub rate_control: RateControl,
    pub width: u32,
    pub height: u32,
    pub crop: CropBounds,
    pub rotation: Rotation,
    pub deinterlace: DeinterlaceMode,
    pub audio_tracks: Vec<AudioTrackConfig>,
    pub subtitle_tracks: Vec<SubtitleTrackConfig>,
    pub web_optimized: bool,
}

impl TranscodeJob {
    pub fn from_preset(source: PathBuf, destination: PathBuf, preset: &Preset) -> Self {
        let (width, height) = (
            preset.max_width.unwrap_or(1920),
            preset.max_height.unwrap_or(1080),
        );

        let audio_track = AudioTrackConfig {
            track_index: 1,
            encoder: preset.audio_encoder,
            bitrate: preset.audio_bitrate,
            channels: preset.audio_channels,
            sample_rate: 48000,
            name: Some("Stereo".to_string()),
        };

        let deinterlace = if preset.decomb {
            DeinterlaceMode::Decomb
        } else if preset.deinterlace {
            DeinterlaceMode::Yadif
        } else {
            DeinterlaceMode::Off
        };

        Self {
            id: Uuid::new_v4().to_string(),
            source,
            destination,
            preset_name: preset.name.clone(),
            video_encoder: preset.video_encoder,
            rate_control: RateControl::ConstantQuality { rf: preset.quality_rf },
            width,
            height,
            crop: CropBounds::default(),
            rotation: Rotation::None,
            deinterlace,
            audio_tracks: vec![audio_track],
            subtitle_tracks: Vec::new(),
            web_optimized: preset.web_optimized,
        }
    }
}
