use tempfile::tempdir;
use handbrake_rs::pipeline::{JobQueue, TranscodeEngine, TranscodeJob};
use handbrake_rs::preset::Preset;

#[test]
fn test_single_job_transcode() -> anyhow::Result<()> {
    let tmp = tempdir()?;
    let src = tmp.path().join("sample_input.mkv");
    let dst = tmp.path().join("output.mp4");

    // Create dummy source file
    std::fs::write(&src, b"FAKE_VIDEO_SOURCE_STREAM_DATA")?;

    let preset = Preset::find("Fast 720p30")?;
    let job = TranscodeJob::from_preset(src, dst.clone(), &preset);

    let mut last_percent = 0.0;
    TranscodeEngine::run(&job, |prog| {
        last_percent = prog.percent;
    })?;

    assert_eq!(last_percent, 100.0);
    assert!(dst.exists());

    let content = std::fs::read_to_string(&dst)?;
    assert!(content.contains("HB_CONTAINER_V0.1"));
    assert!(content.contains("[FRAME:00120"));

    Ok(())
}

#[test]
fn test_batch_job_queue() -> anyhow::Result<()> {
    let tmp = tempdir()?;
    let preset = Preset::find("Discord Small 720p")?;

    let mut queue = JobQueue::new();
    for i in 1..=3 {
        let src = tmp.path().join(format!("input_{}.mp4", i));
        let dst = tmp.path().join(format!("output_{}.mp4", i));
        std::fs::write(&src, b"DATA")?;
        queue.push(TranscodeJob::from_preset(src, dst, &preset));
    }

    assert_eq!(queue.len(), 3);

    let mut jobs_processed = 0;
    let completed = queue.process_all(|idx, total, _job, prog| {
        if prog.is_finished {
            jobs_processed += 1;
            assert_eq!(total, 3);
            assert!(idx <= 3);
        }
    })?;

    assert_eq!(completed, 3);
    assert_eq!(jobs_processed, 3);
    assert!(queue.is_empty());

    Ok(())
}
