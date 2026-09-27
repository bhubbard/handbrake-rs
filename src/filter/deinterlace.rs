use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DeinterlaceMode {
    #[default]
    Off,
    Decomb,
    Yadif,
    Bwdif,
}

pub struct DeinterlaceFilter;

impl DeinterlaceFilter {
    /// Detect interlacing / combing artifacts in a progressive frame
    pub fn is_combed(width: u32, height: u32, y_plane: &[u8], threshold: i32) -> bool {
        if y_plane.len() < (width * height) as usize || height < 3 {
            return false;
        }

        let w = width as usize;
        let h = height as usize;
        let mut combed_pixels = 0;

        for y in 1..h - 1 {
            let row_prev = (y - 1) * w;
            let row_curr = y * w;
            let row_next = (y + 1) * w;

            for x in 0..w {
                let p = y_plane[row_prev + x] as i32;
                let c = y_plane[row_curr + x] as i32;
                let n = y_plane[row_next + x] as i32;

                // Combing condition: current line departs sharply from average of prev and next
                let diff1 = c - p;
                let diff2 = c - n;
                if diff1 * diff2 > threshold * threshold {
                    combed_pixels += 1;
                }
            }
        }

        // Frame is combed if more than 0.5% of pixels show combing
        combed_pixels > (w * h) / 200
    }
}
