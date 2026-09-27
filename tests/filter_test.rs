use handbrake_rs::filter::{
    AspectRatio, CropBounds, DeinterlaceFilter, Rotation, ScaleFilter,
};

#[test]
fn test_crop_bounds_parse_and_apply() {
    let crop = CropBounds::parse("10:12:8:6").expect("Valid crop string");
    assert_eq!(crop.top, 10);
    assert_eq!(crop.bottom, 12);
    assert_eq!(crop.left, 8);
    assert_eq!(crop.right, 6);

    let (new_w, new_h) = crop.apply(1920, 1080);
    assert_eq!(new_w, 1920 - 14);
    assert_eq!(new_h, 1080 - 22);
}

#[test]
fn test_black_bar_detection() {
    let width = 100u32;
    let height = 60u32;
    let mut frame = vec![128u8; (width * height) as usize];

    // Add 10 lines of black bars at top and bottom
    for row in 0..10 {
        for col in 0..width as usize {
            frame[row * width as usize + col] = 0;
            frame[(height as usize - 1 - row) * width as usize + col] = 0;
        }
    }

    let detected = CropBounds::detect_black_bars(width, height, &frame, 16);
    assert_eq!(detected.top, 10);
    assert_eq!(detected.bottom, 10);
    assert_eq!(detected.left, 0);
    assert_eq!(detected.right, 0);
}

#[test]
fn test_scale_calculations() {
    // 4K to 1080p maintaining 16:9
    let (tw, th) = ScaleFilter::calculate_target_size(
        3840,
        2160,
        Some(1920),
        Some(1080),
        true,
        2,
    );
    assert_eq!(tw, 1920);
    assert_eq!(th, 1080);

    // DAR computation
    let dar = ScaleFilter::compute_dar(1920, 1080, AspectRatio::new(1, 1));
    assert_eq!(dar.num, 16);
    assert_eq!(dar.den, 9);
}

#[test]
fn test_rotation_dimensions() {
    let rot = Rotation::Rotate90;
    let (rw, rh) = rot.output_dimensions(1920, 1080);
    assert_eq!(rw, 1080);
    assert_eq!(rh, 1920);
}

#[test]
fn test_deinterlace_combed_detection() {
    let width = 64u32;
    let height = 32u32;

    // Progressive frame (smooth vertical gradient)
    let progressive: Vec<u8> = (0..height)
        .flat_map(|y| vec![(y * 4) as u8; width as usize])
        .collect();
    assert!(!DeinterlaceFilter::is_combed(width, height, &progressive, 20));

    // Interlaced frame with alternating comb lines
    let mut combed = vec![0u8; (width * height) as usize];
    for y in 0..height as usize {
        let val = if y % 2 == 0 { 240 } else { 10 };
        for x in 0..width as usize {
            combed[y * width as usize + x] = val;
        }
    }
    assert!(DeinterlaceFilter::is_combed(width, height, &combed, 20));
}
