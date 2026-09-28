//! Analytical and Geometric Accuracy Verification Tests for handbrake-rs
//!
//! Validates:
//! 1. Macroblock modulus dimension alignment invariance (mod 2, 4, 8, 16).
//! 2. Display Aspect Ratio (DAR) and Pixel Aspect Ratio (PAR) exact geometry (NTSC, PAL, 1080p anamorphic reduction).
//! 3. Synthetic luma black-bar (letterbox) edge boundary detection accuracy.
//! 4. Spatial crop dimension subtraction and boundary saturation conservation.

use handbrake_rs::filter::crop::CropBounds;
use handbrake_rs::filter::scale::{AspectRatio, ScaleFilter};

#[test]
fn test_modulus_macroblock_alignment_invariance() {
    let test_resolutions = [
        (1920, 1080),
        (1919, 1079),
        (1280, 720),
        (723, 481),
        (854, 480),
        (640, 360),
    ];

    for &modulus in &[2, 4, 8, 16] {
        for &(w, h) in &test_resolutions {
            let (target_w, target_h) = ScaleFilter::calculate_target_size(
                w,
                h,
                Some(1280),
                Some(720),
                true,
                modulus,
            );

            assert_eq!(
                target_w % modulus,
                0,
                "Width {} not aligned to mod {} (input {}x{})",
                target_w,
                modulus,
                w,
                h
            );
            assert_eq!(
                target_h % modulus,
                0,
                "Height {} not aligned to mod {} (input {}x{})",
                target_h,
                modulus,
                w,
                h
            );
        }
    }
}

#[test]
fn test_aspect_ratio_dar_par_exact_geometry() {
    // 1. Standard 1080p Square Pixel: 1920x1080, PAR 1:1 => DAR 16:9
    let par_square = AspectRatio::new(1, 1);
    let dar_1080p = ScaleFilter::compute_dar(1920, 1080, par_square);
    assert_eq!(dar_1080p.num, 16);
    assert_eq!(dar_1080p.den, 9);

    // 2. NTSC Anamorphic Widescreen DVD: 720x480, PAR 32:27 => DAR 16:9
    // (720 * 32) / (480 * 27) = 23040 / 12960 = 16 / 9
    let par_ntsc_wide = AspectRatio::new(32, 27);
    let dar_ntsc = ScaleFilter::compute_dar(720, 480, par_ntsc_wide);
    assert_eq!(dar_ntsc.num, 16);
    assert_eq!(dar_ntsc.den, 9);

    // 3. PAL Anamorphic Widescreen DVD: 720x576, PAR 64:45 => DAR 16:9
    // (720 * 64) / (576 * 45) = 46080 / 25920 = 16 / 9
    let par_pal_wide = AspectRatio::new(64, 45);
    let dar_pal = ScaleFilter::compute_dar(720, 576, par_pal_wide);
    assert_eq!(dar_pal.num, 16);
    assert_eq!(dar_pal.den, 9);

    // 4. Classic 4:3 SD: 640x480, PAR 1:1 => DAR 4:3
    let dar_sd = ScaleFilter::compute_dar(640, 480, par_square);
    assert_eq!(dar_sd.num, 4);
    assert_eq!(dar_sd.den, 3);
}

#[test]
fn test_luma_black_bar_detection_exactness() {
    let width = 160;
    let height = 120;
    let top_bar = 20;
    let bot_bar = 24; // Even multiple for YUV420 chroma alignment
    let left_bar = 16;
    let right_bar = 14;

    let mut y_plane = vec![16u8; (width * height) as usize]; // Black = 16

    // Fill active video area with luma = 180
    for y in top_bar..(height - bot_bar) {
        for x in left_bar..(width - right_bar) {
            y_plane[(y * width + x) as usize] = 180;
        }
    }

    let detected = CropBounds::detect_black_bars(width, height, &y_plane, 16);
    assert_eq!(
        detected.top, top_bar,
        "Top black bar detection mismatch: got {}, expected {}",
        detected.top, top_bar
    );
    assert_eq!(
        detected.bottom, bot_bar,
        "Bottom black bar detection mismatch: got {}, expected {}",
        detected.bottom, bot_bar
    );
    assert_eq!(
        detected.left, left_bar,
        "Left black bar detection mismatch: got {}, expected {}",
        detected.left, left_bar
    );
    assert_eq!(
        detected.right, right_bar,
        "Right black bar detection mismatch: got {}, expected {}",
        detected.right, right_bar
    );
}

#[test]
fn test_crop_dimension_subtraction_conservation() {
    let bounds = CropBounds::new(10, 20, 30, 40);
    let (out_w, out_h) = bounds.apply(1920, 1080);
    assert_eq!(out_w, 1920 - (30 + 40));
    assert_eq!(out_h, 1080 - (10 + 20));

    // Boundary saturation protection: minimum dimension clamp at 16
    let extreme_bounds = CropBounds::new(500, 600, 900, 1100);
    let (clamped_w, clamped_h) = extreme_bounds.apply(1920, 1080);
    assert_eq!(clamped_w, 16);
    assert_eq!(clamped_h, 16);
}
