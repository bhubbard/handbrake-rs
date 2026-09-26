# TODO: `handbrake-rs` 🏎️🦀

A high-performance, modular, memory-safe Rust port and headless transcoding engine inspired by [HandBrake/HandBrake](https://github.com/HandBrake/HandBrake) (`libhb`).

---

## 🎯 Mission & Goals

- **Modern Transcoding Pipeline**: Memory-safe Rust reimplementation of `libhb`'s pipeline (Demuxer -> Frame Queue -> Filter Graph -> Encoder -> Muxer).
- **Official Preset Compatibility**: Full JSON parser and validator for standard HandBrake presets (`Fast 1080p30`, `Production Standard`, `Apple 4K HEVC`, `Discord Tiny`, etc.).
- **Hardware Acceleration**: Zero-copy hardware pipelines for Apple Silicon (`VideoToolbox`), NVIDIA (`NVENC`/`NVDEC`), and Intel (`QSV`/`VAAPI`).
- **Comprehensive Video Filters**: Safe Rust implementations of essential transcoding filters: Decomb, Yadif deinterlace, HQDN3D/NLMeans denoise, Chroma smooth, and HDR10 to SDR tone mapping.

---

## 🏗️ Crates Architecture Plan

- [ ] `handbrake-core`: Core media graph pipeline (`Job`, `Title`, `Track`, `FrameQueue`, `ProgressReporter`).
- [ ] `handbrake-presets`: Parser, serializer, and validator for HandBrake JSON preset definitions.
- [ ] `handbrake-filter`: Audio and video filter graphs (decomb, deinterlace, crop, scale, rotate, colorspace/LUT).
- [ ] `handbrake-codec`: Abstraction layer over software (`rav1e`, `x264`, `x265`, `svt-av1`) and hardware encoders (`VideoToolbox`, `NVENC`).
- [ ] `handbrake-container`: MP4, MKV, and WebM muxing and chapter indexing.
- [ ] `handbrake-cli`: Headless CLI compatible with `HandBrakeCLI` arguments.

---

## 📋 Implementation Checklist

### Phase 1: Core Transcoding Graph
- [ ] Define asynchronous media pipeline architecture:
  ```rust
  pub struct TranscodeJob {
      pub source: PathBuf,
      pub destination: PathBuf,
      pub title_index: usize,
      pub video_settings: VideoSettings,
      pub audio_tracks: Vec<AudioTrackSettings>,
      pub subtitle_tracks: Vec<SubtitleTrackSettings>,
      pub filters: FilterChain,
  }
  ```
- [ ] Build bounded, back-pressure-aware `FrameQueue` with zero-copy buffer pools.
- [ ] Title and stream scanner: parse video resolution, frame rate, color primaries, audio tracks, and chapter markers.

### Phase 2: Preset Engine
- [ ] Reverse-engineer & validate HandBrake's JSON preset schema.
- [ ] Embed official default presets (`General`, `Web`, `Devices`, `Matroska`, `Production`).
- [ ] CLI flag `--preset` with fuzzy-matching and override parameters (`--encoder`, `--quality`, `--rate`).

### Phase 3: Video Filter Graph
- [ ] **Crop & Scale**: Auto-crop black bar detection and Lanczos/Bicubic resamplers.
- [ ] **Decomb / Deinterlace**: Port HandBrake’s motion-adaptive decomb filter and Yadif.
- [ ] **Denoise**: NLMeans and HQDN3D spatial/temporal denoiser implementations.
- [ ] **Color Management**: BT.709, BT.2020, and HDR10 static metadata passthrough / tone mapping.
- [ ] **Rotation & Flipping**: Efficient 90/180/270-degree transposition.

### Phase 4: Encoders & Hardware Acceleration
- [ ] Software encoders:
  - [ ] H.264 (`x264`) and H.265 (`x265`) via safe Rust bindings.
  - [ ] AV1 via `svt-av1` and `rav1e`.
- [ ] Hardware acceleration:
  - [ ] macOS: `VideoToolbox` H.264, HEVC, and ProRes hardware encoders.
  - [ ] Linux/Windows: `NVENC` (NVIDIA) and `VAAPI` / `QSV` (Intel).
- [ ] Two-pass rate control and Constant Quality (`CRF` / `CQ`) modes.

### Phase 5: Audio & Subtitles
- [ ] Audio transcoding: AAC, Opus, FLAC, AC-3, E-AC-3, and MP3.
- [ ] Multi-channel downmixing (5.1/7.1 to stereo) and dynamic range compression.
- [ ] Subtitle handling:
  - [ ] Passthrough (SRT, SSA/ASS in MKV, Tx3g in MP4).
  - [ ] Burn-in / hard-sub filter for bitmap subtitles (PGS, VOBSUB).

### Phase 6: CLI & Batch Processing
- [ ] Implement `HandBrakeCLI` argument parity (`-i`, `-o`, `-e`, `-q`, `-B`, `-R`, `--all-subtitles`, `--markers`).
- [ ] Batch directory watcher and transcode queue manager.
- [ ] Detailed ETA, FPS, bitrate, and frame progress reporting via terminal or JSON event stream.

### Phase 7: Benchmarks & Parity Tests
- [ ] Transcode speed and VMAF / SSIM quality benchmark comparing `handbrake-rs` vs official C `HandBrakeCLI`.
- [ ] Memory leak and thread-scaling test suite across 16+ core machines.
