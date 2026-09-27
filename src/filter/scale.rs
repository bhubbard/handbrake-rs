use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ScaleAlgorithm {
    #[default]
    Lanczos,
    Bicubic,
    Bilinear,
    Spline,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct AspectRatio {
    pub num: u32,
    pub den: u32,
}

impl AspectRatio {
    pub fn new(num: u32, den: u32) -> Self {
        Self { num, den }
    }

    pub fn to_f32(&self) -> f32 {
        if self.den == 0 {
            1.0
        } else {
            self.num as f32 / self.den as f32
        }
    }
}

pub struct ScaleFilter;

impl ScaleFilter {
    /// Calculate output dimensions given original dimensions, max width/height, and modulus constraint
    pub fn calculate_target_size(
        orig_width: u32,
        orig_height: u32,
        max_width: Option<u32>,
        max_height: Option<u32>,
        keep_aspect_ratio: bool,
        modulus: u32,
    ) -> (u32, u32) {
        let mod_val = modulus.max(2);
        let orig_aspect = orig_width as f32 / orig_height.max(1) as f32;

        let (mut target_w, mut target_h) = (orig_width, orig_height);

        match (max_width, max_height) {
            (Some(mw), Some(mh)) => {
                if keep_aspect_ratio {
                    let w_scale = mw as f32 / orig_width as f32;
                    let h_scale = mh as f32 / orig_height as f32;
                    let scale = w_scale.min(h_scale).min(1.0); // Never upscale by default

                    target_w = (orig_width as f32 * scale).round() as u32;
                    target_h = (orig_height as f32 * scale).round() as u32;
                } else {
                    target_w = mw;
                    target_h = mh;
                }
            }
            (Some(mw), None) => {
                target_w = mw.min(orig_width);
                if keep_aspect_ratio {
                    target_h = (target_w as f32 / orig_aspect).round() as u32;
                }
            }
            (None, Some(mh)) => {
                target_h = mh.min(orig_height);
                if keep_aspect_ratio {
                    target_w = (target_h as f32 * orig_aspect).round() as u32;
                }
            }
            (None, None) => {}
        }

        // Align to modulus constraint
        target_w = ((target_w + mod_val / 2) / mod_val) * mod_val;
        target_h = ((target_h + mod_val / 2) / mod_val) * mod_val;

        (target_w.max(mod_val), target_h.max(mod_val))
    }

    /// Compute Display Aspect Ratio (DAR) from dimensions and Pixel Aspect Ratio (PAR)
    pub fn compute_dar(width: u32, height: u32, par: AspectRatio) -> AspectRatio {
        let dar_num = width * par.num;
        let dar_den = height * par.den;
        let gcd_val = gcd(dar_num, dar_den);
        AspectRatio::new(dar_num / gcd_val.max(1), dar_den / gcd_val.max(1))
    }
}

fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}
