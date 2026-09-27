use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscodeProgress {
    pub job_id: String,
    pub percent: f32,
    pub current_frame: u64,
    pub total_frames: u64,
    pub fps: f32,
    pub eta_seconds: u64,
    pub average_bitrate_kbps: f32,
    pub is_finished: bool,
}

impl TranscodeProgress {
    pub fn new(job_id: String, total_frames: u64) -> Self {
        Self {
            job_id,
            percent: 0.0,
            current_frame: 0,
            total_frames: total_frames.max(1),
            fps: 0.0,
            eta_seconds: 0,
            average_bitrate_kbps: 0.0,
            is_finished: false,
        }
    }

    pub fn update(&mut self, current_frame: u64, elapsed_secs: f32) {
        self.current_frame = current_frame;
        self.percent = ((current_frame as f32 / self.total_frames as f32) * 100.0).clamp(0.0, 100.0);

        if elapsed_secs > 0.05 {
            self.fps = current_frame as f32 / elapsed_secs;
            let remaining_frames = self.total_frames.saturating_sub(current_frame);
            if self.fps > 0.1 {
                self.eta_seconds = (remaining_frames as f32 / self.fps).round() as u64;
            }
        }

        if current_frame >= self.total_frames {
            self.is_finished = true;
            self.percent = 100.0;
            self.eta_seconds = 0;
        }
    }
}
