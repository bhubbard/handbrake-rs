use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct CropBounds {
    pub top: u32,
    pub bottom: u32,
    pub left: u32,
    pub right: u32,
}

impl CropBounds {
    pub fn new(top: u32, bottom: u32, left: u32, right: u32) -> Self {
        Self { top, bottom, left, right }
    }

    pub fn parse(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() == 4 {
            let top = parts[0].parse().ok()?;
            let bottom = parts[1].parse().ok()?;
            let left = parts[2].parse().ok()?;
            let right = parts[3].parse().ok()?;
            Some(Self { top, bottom, left, right })
        } else {
            None
        }
    }

    pub fn is_zero(&self) -> bool {
        self.top == 0 && self.bottom == 0 && self.left == 0 && self.right == 0
    }

    /// Calculate cropped dimensions, enforcing minimum dimension bounds
    pub fn apply(&self, width: u32, height: u32) -> (u32, u32) {
        let new_w = width.saturating_sub(self.left + self.right).max(16);
        let new_h = height.saturating_sub(self.top + self.bottom).max(16);
        (new_w, new_h)
    }

    /// Black bar detection algorithm on a luma (Y) plane buffer
    pub fn detect_black_bars(width: u32, height: u32, y_plane: &[u8], threshold: u8) -> Self {
        if y_plane.len() < (width * height) as usize {
            return Self::default();
        }

        let w = width as usize;
        let h = height as usize;

        // 1. Detect top black lines
        let mut top = 0;
        for row in 0..h / 2 {
            let row_slice = &y_plane[row * w..(row + 1) * w];
            let is_black = row_slice.iter().all(|&val| val <= threshold);
            if is_black {
                top += 1;
            } else {
                break;
            }
        }

        // 2. Detect bottom black lines
        let mut bottom = 0;
        for row in (h / 2..h).rev() {
            let row_slice = &y_plane[row * w..(row + 1) * w];
            let is_black = row_slice.iter().all(|&val| val <= threshold);
            if is_black {
                bottom += 1;
            } else {
                break;
            }
        }

        // 3. Detect left black columns
        let mut left = 0;
        for col in 0..w / 2 {
            let is_black = (top..h - bottom).all(|row| y_plane[row * w + col] <= threshold);
            if is_black {
                left += 1;
            } else {
                break;
            }
        }

        // 4. Detect right black columns
        let mut right = 0;
        for col in (w / 2..w).rev() {
            let is_black = (top..h - bottom).all(|row| y_plane[row * w + col] <= threshold);
            if is_black {
                right += 1;
            } else {
                break;
            }
        }

        // Align crop values to even multiples for YUV420 chroma subsampling
        Self {
            top: (top / 2 * 2) as u32,
            bottom: (bottom / 2 * 2) as u32,
            left: (left / 2 * 2) as u32,
            right: (right / 2 * 2) as u32,
        }
    }
}
