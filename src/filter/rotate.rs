use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Rotation {
    #[default]
    None,
    Rotate90,
    Rotate180,
    Rotate270,
    FlipHorizontal,
    FlipVertical,
}

impl Rotation {
    pub fn from_angle(degrees: u32) -> Self {
        match degrees % 360 {
            90 => Rotation::Rotate90,
            180 => Rotation::Rotate180,
            270 => Rotation::Rotate270,
            _ => Rotation::None,
        }
    }

    /// Swaps width and height for 90 and 270 degree rotations
    pub fn output_dimensions(&self, width: u32, height: u32) -> (u32, u32) {
        match self {
            Rotation::Rotate90 | Rotation::Rotate270 => (height, width),
            _ => (width, height),
        }
    }
}
