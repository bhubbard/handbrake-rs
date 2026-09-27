use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::PathBuf;
use handbrake_rs::filter::{CropBounds, DeinterlaceMode, Rotation};
use handbrake_rs::format::{AudioEncoder, RateControl, VideoEncoder};
use handbrake_rs::pipeline::{TranscodeEngine, TranscodeJob};
use handbrake_rs::preset::Preset;

#[derive(Parser, Debug)]
#[command(
    name = "HandBrakeCLI",
    about = "HandBrake command line video transcoder in pure Rust",
    version
)]
struct Args {
    /// Input file or device path
    #[arg(short = 'i', long = "input")]
    input: Option<PathBuf>,

    /// Destination output file
    #[arg(short = 'o', long = "output")]
    output: Option<PathBuf>,

    /// Preset name to apply (e.g. "Fast 1080p30", "Discord Nitro 1080p")
    #[arg(short = 'Z', long = "preset", default_value = "Fast 1080p30")]
    preset: String,

    /// List all available official HandBrake presets
    #[arg(long = "preset-list")]
    preset_list: bool,

    /// Video encoder (x264, x265, svt_av1, vp9, prores, vt_h264, vt_h265)
    #[arg(short = 'e', long = "encoder")]
    encoder: Option<String>,

    /// Video quality RF/CRF value (lower is higher quality)
    #[arg(short = 'q', long = "quality")]
    quality: Option<f32>,

    /// Video target width
    #[arg(short = 'w', long = "width")]
    width: Option<u32>,

    /// Video target height
    #[arg(short = 'l', long = "height")]
    height: Option<u32>,

    /// Video crop window (top:bottom:left:right)
    #[arg(long = "crop")]
    crop: Option<String>,

    /// Audio encoder (aac, opus, flac, mp3, copy)
    #[arg(short = 'E', long = "aencoder")]
    aencoder: Option<String>,

    /// Audio bitrate in kbps
    #[arg(short = 'B', long = "ab")]
    audio_bitrate: Option<u32>,

    /// Video rotation in degrees (90, 180, 270)
    #[arg(long = "rotate")]
    rotate: Option<u32>,

    /// Enable motion-adaptive decomb filter
    #[arg(long = "decomb")]
    decomb: bool,

    /// Enable yadif deinterlacer
    #[arg(long = "deinterlace")]
    deinterlace: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    if args.preset_list {
        println!("Available HandBrake Presets:");
        println!("============================================================");
        let mut curr_cat = None;
        for p in Preset::all_presets() {
            if curr_cat != Some(p.category) {
                curr_cat = Some(p.category);
                println!("\n[{:?}]", p.category);
            }
            println!("  • {:<28} ({} | RF {:.1} | {})",
                p.name,
                p.video_encoder.as_str(),
                p.quality_rf,
                p.container.extension()
            );
            println!("    {}", p.description);
        }
        println!("============================================================");
        return Ok(());
    }

    let input = match args.input {
        Some(i) => i,
        None => {
            eprintln!("Error: Missing required option -i / --input. Use --help for usage details.");
            std::process::exit(1);
        }
    };

    let output = match args.output {
        Some(o) => o,
        None => {
            let mut out = input.clone();
            out.set_extension("mp4");
            out
        }
    };

    let preset = Preset::find(&args.preset)?;
    println!("HandBrakeCLI: Using preset '{}'", preset.name);

    let mut job = TranscodeJob::from_preset(input.clone(), output.clone(), &preset);

    // Apply command-line overrides
    if let Some(ref enc_str) = args.encoder {
        if let Some(enc) = VideoEncoder::from_str_loose(enc_str) {
            job.video_encoder = enc;
        }
    }

    if let Some(rf) = args.quality {
        job.rate_control = RateControl::ConstantQuality { rf };
    }

    if let Some(w) = args.width {
        job.width = w;
    }

    if let Some(h) = args.height {
        job.height = h;
    }

    if let Some(ref crop_str) = args.crop {
        if let Some(c) = CropBounds::parse(crop_str) {
            job.crop = c;
        }
    }

    if let Some(deg) = args.rotate {
        job.rotation = Rotation::from_angle(deg);
    }

    if args.decomb {
        job.deinterlace = DeinterlaceMode::Decomb;
    } else if args.deinterlace {
        job.deinterlace = DeinterlaceMode::Yadif;
    }

    if let Some(ref aenc_str) = args.aencoder {
        if let Some(aenc) = AudioEncoder::from_str_loose(aenc_str) {
            if let Some(track) = job.audio_tracks.get_mut(0) {
                track.encoder = aenc;
            }
        }
    }

    if let Some(ab) = args.audio_bitrate {
        if let Some(track) = job.audio_tracks.get_mut(0) {
            track.bitrate = ab;
        }
    }

    println!("Encoding: \"{}\" -> \"{}\"", job.source.display(), job.destination.display());
    println!("Video: {} ({:?}) | Dimensions: {}x{} | Audio: {:?}",
        job.video_encoder.as_str(),
        job.rate_control,
        job.width,
        job.height,
        job.audio_tracks[0].encoder.as_str()
    );

    let pb = ProgressBar::new(100);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}% ({msg})")?
            .progress_chars("#>-"),
    );

    TranscodeEngine::run(&job, |prog| {
        pb.set_position(prog.percent as u64);
        pb.set_message(format!(
            "{:.1} fps | ETA {}s | {:.1} kbps",
            prog.fps, prog.eta_seconds, prog.average_bitrate_kbps
        ));
    })?;

    pb.finish_with_message("Done!");
    println!("\nEncode complete: \"{}\"", job.destination.display());
    Ok(())
}
