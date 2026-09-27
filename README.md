# handbrake-rs 🏎️🦀

[![Crates.io](https://img.shields.io/badge/crates.io-v0.0.1-orange.svg)](https://crates.io/crates/handbrake-rs)
[![Documentation](https://img.shields.io/badge/docs-GitHub_Pages-blue.svg)](http://code.brandonhubbard.com/handbrake-rs/)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)
[![HandBrake Compatible](https://img.shields.io/badge/HandBrake-libhb_Compatible-red.svg)](https://github.com/HandBrake/HandBrake)

High-performance, memory-safe video transcoding engine and CLI inspired by [HandBrake](https://github.com/HandBrake/HandBrake) (`libhb`) in pure Rust.

Eliminates C/C++ memory vulnerabilities while delivering official HandBrake preset compatibility, intelligent black-bar auto-cropping, motion-adaptive decomb filtering, and multi-file batch queue execution.

---

## 🚀 Features

- **Official Preset Database**:
  - `General`: `Fast 1080p30`, `Fast 720p30`, `HQ 1080p30 Surround`, `Super HQ 1080p30`, `Fast 4K HEVC`
  - `Web`: `Discord Nitro 1080p`, `Discord Small 720p`, `YouTube 1080p60`
  - `Devices`: `Apple 1080p30 Surround`, `Apple 4K HEVC`
  - `Matroska`: `H.265 MKV 1080p`, `AV1 MKV 1080p`
  - `Production`: `Production Standard`, `Production Max ProRes`
- **Intelligent Video Filters**:
  - Auto-crop black bar detector on luma planes with YUV420 chroma alignment
  - Resolution scaling (Lanczos, Bicubic, Bilinear) with DAR & PAR geometry
  - Motion-adaptive comb / interlacing artifact detection
  - Lossless 90/180/270 degree rotation
- **Multi-File Batch Queue**: Sequential job queue runner with real-time FPS, ETA, bitrate, and progress telemetry.
- **`HandBrakeCLI` Compatible**: CLI flags matching upstream HandBrake (`-i`, `-o`, `-Z`, `--preset-list`, `-e`, `-q`, `-w`, `-l`, `--crop`, `--decomb`).
- **Interactive Documentation**: Explore presets, test letterbox auto-cropping, and simulate batch queue execution at [code.brandonhubbard.com/handbrake-rs](http://code.brandonhubbard.com/handbrake-rs/).

---

## 📦 Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
handbrake-rs = "0.0.1"
```

Or install the standalone binary:

```bash
cargo install handbrake-rs
```

---

## 🛠️ CLI Usage

```bash
# List all available presets
HandBrakeCLI --preset-list

# Transcode with official preset
HandBrakeCLI -i input.mov -o output.mp4 -Z "Fast 1080p30"

# Custom quality and cropping overrides
HandBrakeCLI -i gameplay.mkv -o discord.mp4 \
  -Z "Discord Nitro 1080p" \
  -q 20.0 \
  --crop 140:140:0:0 \
  --decomb
```

---

## 💻 Rust SDK Example

```rust
use handbrake_rs::{TranscodeEngine, TranscodeJob, Preset};
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    // 1. Select official preset
    let preset = Preset::find("Fast 1080p30")?;

    // 2. Configure job
    let job = TranscodeJob::from_preset(
        PathBuf::from("raw_footage.mkv"),
        PathBuf::from("export.mp4"),
        &preset,
    );

    // 3. Execute with progress monitoring
    TranscodeEngine::run(&job, |progress| {
        println!("{:.1}% | {:.1} fps | ETA: {}s | {:.1} kbps",
            progress.percent,
            progress.fps,
            progress.eta_seconds,
            progress.average_bitrate_kbps,
        );
    })?;

    Ok(())
}
```

---

## 🧪 Testing

Run all unit and integration tests:

```bash
cargo test
```

---

## 📄 License

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))
