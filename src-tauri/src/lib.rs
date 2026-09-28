use std::sync::{Arc, Mutex};
use tauri::{AppHandle, State};
use tauri_plugin_dialog::DialogExt;
use handbrake_rs::preset::Preset;
use handbrake_rs::pipeline::{JobQueue, TranscodeEngine, TranscodeJob, TranscodeProgress};

pub struct AppState {
    pub queue: Arc<Mutex<JobQueue>>,
    pub active_progress: Arc<Mutex<Option<TranscodeProgress>>>,
}

#[tauri::command]
fn get_presets() -> Vec<Preset> {
    Preset::all_presets()
}

#[tauri::command]
fn get_preset_by_name(name: String) -> Option<Preset> {
    Preset::all_presets().into_iter().find(|p| p.name == name)
}

#[tauri::command]
fn pick_source_file(app: AppHandle) -> Option<String> {
    app.dialog()
        .file()
        .add_filter("Video Files", &["mp4", "mkv", "mov", "avi", "m4v", "webm", "ts", "flv"])
        .add_filter("All Files", &["*"])
        .blocking_pick_file()
        .map(|path| path.to_string())
}

#[tauri::command]
fn pick_destination_file(app: AppHandle, default_name: Option<String>) -> Option<String> {
    let mut builder = app.dialog().file();
    if let Some(ref name) = default_name {
        builder = builder.set_file_name(name);
    }
    builder
        .add_filter("MP4 Video", &["mp4", "m4v"])
        .add_filter("Matroska Video", &["mkv"])
        .add_filter("WebM Video", &["webm"])
        .blocking_save_file()
        .map(|path| path.to_string())
}

#[tauri::command]
fn add_to_queue(state: State<'_, AppState>, job: TranscodeJob) -> Result<String, String> {
    let id = job.id.clone();
    let mut queue = state.queue.lock().map_err(|e| e.to_string())?;
    queue.push(job);
    Ok(id)
}

#[tauri::command]
fn get_queue(state: State<'_, AppState>) -> Result<Vec<TranscodeJob>, String> {
    let queue = state.queue.lock().map_err(|e| e.to_string())?;
    Ok(queue.jobs.iter().cloned().collect())
}

#[tauri::command]
fn clear_queue(state: State<'_, AppState>) -> Result<(), String> {
    let mut queue = state.queue.lock().map_err(|e| e.to_string())?;
    queue.jobs.clear();
    let mut prog = state.active_progress.lock().map_err(|e| e.to_string())?;
    *prog = None;
    Ok(())
}

#[tauri::command]
fn get_progress(state: State<'_, AppState>) -> Result<Option<TranscodeProgress>, String> {
    let prog = state.active_progress.lock().map_err(|e| e.to_string())?;
    Ok(prog.clone())
}

#[tauri::command]
async fn start_encode(state: State<'_, AppState>) -> Result<(), String> {
    let mut current_job: Option<TranscodeJob> = None;
    {
        let mut queue = state.queue.lock().map_err(|e| e.to_string())?;
        if let Some(job) = queue.jobs.pop_front() {
            current_job = Some(job);
        }
    }

    if let Some(job) = current_job {
        let progress_state = state.active_progress.clone();
        
        // Run transcode in worker thread
        tokio::task::spawn_blocking(move || {
            let res = TranscodeEngine::run(&job, |prog| {
                if let Ok(mut lock) = progress_state.lock() {
                    *lock = Some(prog.clone());
                }
            });
            if let Err(e) = res {
                eprintln!("Encoding error: {:?}", e);
            }
        }).await.map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            queue: Arc::new(Mutex::new(JobQueue::new())),
            active_progress: Arc::new(Mutex::new(None)),
        })
        .invoke_handler(tauri::generate_handler![
            get_presets,
            get_preset_by_name,
            pick_source_file,
            pick_destination_file,
            add_to_queue,
            get_queue,
            clear_queue,
            get_progress,
            start_encode
        ])
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
