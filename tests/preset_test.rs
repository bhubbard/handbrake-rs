use handbrake_rs::format::{AudioEncoder, ContainerFormat, VideoEncoder};
use handbrake_rs::preset::{Preset, PresetCategory};

#[test]
fn test_all_presets_validity() {
    let presets = Preset::all_presets();
    assert!(presets.len() >= 12);

    for p in &presets {
        assert!(!p.name.is_empty());
        assert!(p.quality_rf >= 5.0 && p.quality_rf <= 40.0);
        if let Some(w) = p.max_width {
            assert!(w >= 640);
        }
        if let Some(h) = p.max_height {
            assert!(h >= 360);
        }
    }
}

#[test]
fn test_find_preset_exact_and_fuzzy() -> anyhow::Result<()> {
    // Exact
    let fast_1080 = Preset::find("Fast 1080p30")?;
    assert_eq!(fast_1080.category, PresetCategory::General);
    assert_eq!(fast_1080.container, ContainerFormat::Mp4);
    assert_eq!(fast_1080.video_encoder, VideoEncoder::X264);

    // Fuzzy
    let discord = Preset::find("discord nitro")?;
    assert_eq!(discord.category, PresetCategory::Web);
    assert_eq!(discord.audio_encoder, AudioEncoder::Opus);

    // Unknown
    assert!(Preset::find("non_existent_preset_999").is_err());
    Ok(())
}
