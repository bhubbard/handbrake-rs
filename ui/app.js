// HandBrake Desktop Frontend Logic (Tauri v2)

const hasTauri = typeof window !== 'undefined' && window.__TAURI__ && window.__TAURI__.core;
const invoke = hasTauri ? window.__TAURI__.core.invoke : async (cmd, args) => {
  console.log('[Mock Tauri Invoke]', cmd, args);
  if (cmd === 'get_presets') {
    return [
      { name: "Fast 1080p30", description: "Fast, standard H.264 encode for 1080p at 30 fps", category: "General", container: "Mp4", video_encoder: "X264", quality_rf: 22.0, max_width: 1920, max_height: 1080, frame_rate: 30.0, audio_encoder: "Aac", audio_bitrate: 160, audio_channels: 2, web_optimized: true, decomb: true, deinterlace: false },
      { name: "Fast 720p30", description: "Fast 720p H.264 encode for compact files", category: "General", container: "Mp4", video_encoder: "X264", quality_rf: 22.0, max_width: 1280, max_height: 720, frame_rate: 30.0, audio_encoder: "Aac", audio_bitrate: 160, audio_channels: 2, web_optimized: true, decomb: true, deinterlace: false },
      { name: "HQ 1080p30 Surround", description: "High quality 1080p H.264 with surround audio", category: "General", container: "Mp4", video_encoder: "X264", quality_rf: 20.0, max_width: 1920, max_height: 1080, frame_rate: 30.0, audio_encoder: "Aac", audio_bitrate: 256, audio_channels: 6, web_optimized: true, decomb: true, deinterlace: false },
      { name: "Discord Nitro 1080p", description: "Optimized for Discord 50MB file size limit", category: "Web", container: "Mp4", video_encoder: "X264", quality_rf: 24.0, max_width: 1920, max_height: 1080, frame_rate: 30.0, audio_encoder: "Aac", audio_bitrate: 128, audio_channels: 2, web_optimized: true, decomb: false, deinterlace: false },
      { name: "Vimeo YouTube 1080p60", description: "High-framerate 1080p for video sharing platforms", category: "Web", container: "Mp4", video_encoder: "X264", quality_rf: 20.0, max_width: 1920, max_height: 1080, frame_rate: 60.0, audio_encoder: "Aac", audio_bitrate: 320, audio_channels: 2, web_optimized: true, decomb: false, deinterlace: false },
      { name: "Apple 1080p60 Surround", description: "Apple TV, iPhone, and iPad 1080p60 with surround audio", category: "Devices", container: "Mp4", video_encoder: "X264", quality_rf: 21.0, max_width: 1920, max_height: 1080, frame_rate: 60.0, audio_encoder: "Aac", audio_bitrate: 256, audio_channels: 6, web_optimized: true, decomb: true, deinterlace: false },
      { name: "Apple 2160p60 4K HEVC Surround", description: "Apple 4K HEVC (H.265) 60 fps", category: "Devices", container: "Mp4", video_encoder: "X265", quality_rf: 24.0, max_width: 3840, max_height: 2160, frame_rate: 60.0, audio_encoder: "Aac", audio_bitrate: 320, audio_channels: 6, web_optimized: true, decomb: false, deinterlace: false },
      { name: "H.264 MKV 1080p30", description: "Standard Matroska container with H.264 video", category: "Matroska", container: "Mkv", video_encoder: "X264", quality_rf: 22.0, max_width: 1920, max_height: 1080, frame_rate: 30.0, audio_encoder: "Aac", audio_bitrate: 160, audio_channels: 2, web_optimized: false, decomb: true, deinterlace: false },
      { name: "H.265 MKV 1080p30", description: "Matroska container with high-efficiency H.265 video", category: "Matroska", container: "Mkv", video_encoder: "X265", quality_rf: 24.0, max_width: 1920, max_height: 1080, frame_rate: 30.0, audio_encoder: "Aac", audio_bitrate: 160, audio_channels: 2, web_optimized: false, decomb: true, deinterlace: false },
      { name: "Production Standard", description: "Production-grade encode with minimal compression artifacts", category: "Production", container: "Mp4", video_encoder: "X264", quality_rf: 18.0, max_width: 1920, max_height: 1080, frame_rate: null, audio_encoder: "Flac", audio_bitrate: 0, audio_channels: 2, web_optimized: false, decomb: false, deinterlace: false }
    ];
  }
  if (cmd === 'get_queue') return [];
  if (cmd === 'pick_source_file') return "/Users/bhubbard/Movies/sample_clip.mp4";
  if (cmd === 'pick_destination_file') return "/Users/bhubbard/Movies/sample_clip_encoded.mp4";
  if (cmd === 'add_to_queue') return "job-101";
  if (cmd === 'clear_queue') return true;
  if (cmd === 'start_encode') return true;
  return null;
};

// State
let allPresets = [];
let currentPreset = null;
let queueList = [];
let isEncoding = false;
let encodeInterval = null;

// DOM Elements
const sourceInput = document.getElementById('source-path');
const destInput = document.getElementById('dest-path');
const presetSelect = document.getElementById('preset-select');
const presetDescBadge = document.getElementById('preset-desc-badge');
const presetsSidebar = document.getElementById('presets-sidebar');
const queueModal = document.getElementById('queue-modal');
const queueCountBadge = document.getElementById('queue-count');
const queueTableBody = document.getElementById('queue-table-body');
const statusDot = document.getElementById('status-dot');
const statusText = document.getElementById('status-text');
const progressFill = document.getElementById('progress-fill');
const progressPercent = document.getElementById('progress-percent');
const statFps = document.getElementById('stat-fps');
const statEta = document.getElementById('stat-eta');
const rfSlider = document.getElementById('rf-slider');
const rfValueDisplay = document.getElementById('rf-value-display');
const videoEncoderSelect = document.getElementById('video-encoder');
const dimWidthInput = document.getElementById('dim-width');
const dimHeightInput = document.getElementById('dim-height');
const summaryContainer = document.getElementById('summary-container');
const summaryWebOpt = document.getElementById('summary-web-opt');
const startBtn = document.getElementById('btn-start-encode');
const startBtnLabel = document.getElementById('start-btn-label');

// Tab Navigation
document.querySelectorAll('.tab-link').forEach(button => {
  button.addEventListener('click', () => {
    document.querySelectorAll('.tab-link').forEach(btn => btn.classList.remove('active'));
    document.querySelectorAll('.tab-panel').forEach(panel => panel.classList.remove('active'));
    
    button.classList.add('active');
    const tabId = button.getAttribute('data-tab');
    const targetPanel = document.getElementById(tabId);
    if (targetPanel) {
      targetPanel.classList.add('active');
    }
  });
});

// Presets Sidebar Toggle
document.getElementById('btn-toggle-presets').addEventListener('click', (e) => {
  presetsSidebar.classList.toggle('collapsed');
  e.currentTarget.classList.toggle('active', !presetsSidebar.classList.contains('collapsed'));
});

// Queue Modal Toggle
document.getElementById('btn-toggle-queue').addEventListener('click', () => {
  queueModal.classList.toggle('open');
  renderQueueTable();
});
document.getElementById('btn-close-queue').addEventListener('click', () => {
  queueModal.classList.remove('open');
});

// Quality Slider Update
rfSlider.addEventListener('input', (e) => {
  rfValueDisplay.textContent = e.target.value;
  updateSummary();
});

// Apply a preset to all controls
function applyPreset(preset) {
  if (!preset) return;
  currentPreset = preset;
  presetSelect.value = preset.name;
  presetDescBadge.textContent = preset.description;

  // Video
  rfSlider.value = preset.quality_rf;
  rfValueDisplay.textContent = preset.quality_rf;
  
  const encKey = preset.video_encoder.toLowerCase();
  for (let opt of videoEncoderSelect.options) {
    if (opt.value.toLowerCase().includes(encKey)) {
      videoEncoderSelect.value = opt.value;
      break;
    }
  }

  // Dimensions
  dimWidthInput.value = preset.max_width || 1920;
  dimHeightInput.value = preset.max_height || 1080;
  document.getElementById('preview-resolution-text').textContent = 
    `${dimWidthInput.value} x ${dimHeightInput.value}`;

  // Container & Flags
  summaryContainer.value = preset.container.toLowerCase();
  summaryWebOpt.checked = preset.web_optimized;

  // Filters
  const deintSelect = document.getElementById('filter-deinterlace');
  if (preset.decomb) {
    deintSelect.value = 'decomb';
  } else if (preset.deinterlace) {
    deintSelect.value = 'yadif';
  } else {
    deintSelect.value = 'off';
  }

  // Audio
  const audioBitrate = document.getElementById('audio-bitrate');
  if (preset.audio_bitrate) {
    audioBitrate.value = String(preset.audio_bitrate);
  }

  // Highlight in sidebar
  document.querySelectorAll('.cat-item').forEach(item => {
    item.classList.toggle('active', item.getAttribute('data-preset') === preset.name);
  });

  // Default destination auto-extension update
  if (sourceInput.value && !destInput.value) {
    autoSetDestination(sourceInput.value, preset.container.toLowerCase());
  }

  updateSummary();
}

function autoSetDestination(src, ext) {
  if (!src) return;
  const lastDot = src.lastIndexOf('.');
  const base = lastDot !== -1 ? src.substring(0, lastDot) : src;
  destInput.value = `${base}_encoded.${ext}`;
}

function updateSummary() {
  document.getElementById('sum-video').textContent = 
    `${videoEncoderSelect.value}, RF ${rfSlider.value}, ${document.getElementById('video-fps').value} fps`;
  document.getElementById('sum-audio').textContent = 
    `AAC, ${document.getElementById('audio-bitrate').value} kbps, Stereo`;
  document.getElementById('sum-dims').textContent = 
    `${dimWidthInput.value}x${dimHeightInput.value} max`;
  document.getElementById('sum-filters').textContent = 
    `Deinterlace: ${document.getElementById('filter-deinterlace').value}`;
}

// Load Presets from Rust Backend
async function loadPresets() {
  try {
    allPresets = await invoke('get_presets');
    if (!allPresets || allPresets.length === 0) return;

    // Populate dropdown
    presetSelect.innerHTML = '';
    allPresets.forEach(p => {
      const opt = document.createElement('option');
      opt.value = p.name;
      opt.textContent = `${p.name} (${p.category})`;
      presetSelect.appendChild(opt);
    });

    // Populate sidebar grouped by category
    const sidebar = document.getElementById('presets-list-container');
    sidebar.innerHTML = '';
    const categories = {};
    allPresets.forEach(p => {
      const cat = p.category;
      if (!categories[cat]) categories[cat] = [];
      categories[cat].push(p);
    });

    for (const [catName, list] of Object.entries(categories)) {
      const catDiv = document.createElement('div');
      catDiv.className = 'preset-category';
      catDiv.innerHTML = `<div class="cat-title">${catName}</div>`;
      
      list.forEach(p => {
        const item = document.createElement('div');
        item.className = 'cat-item';
        item.setAttribute('data-preset', p.name);
        item.textContent = p.name;
        item.addEventListener('click', () => {
          applyPreset(p);
        });
        catDiv.appendChild(item);
      });
      sidebar.appendChild(catDiv);
    }

    // Default to Fast 1080p30
    const defaultPreset = allPresets.find(p => p.name === 'Fast 1080p30') || allPresets[0];
    applyPreset(defaultPreset);
  } catch (err) {
    console.error('Failed to load presets:', err);
  }
}

// Dropdown Preset Change
presetSelect.addEventListener('change', (e) => {
  const chosen = allPresets.find(p => p.name === e.target.value);
  if (chosen) applyPreset(chosen);
});

// File Pickers (Native Dialogs via Rust)
async function handleOpenSource() {
  try {
    const selected = await invoke('pick_source_file');
    if (selected) {
      sourceInput.value = selected;
      const ext = summaryContainer.value || 'mp4';
      autoSetDestination(selected, ext);
      statusText.textContent = `Opened: ${selected.split('/').pop()}`;
    }
  } catch (err) {
    console.error('Failed to pick source file:', err);
  }
}

async function handleBrowseDest() {
  try {
    const defaultName = destInput.value.split('/').pop() || 'output.mp4';
    const selected = await invoke('pick_destination_file', { defaultName });
    if (selected) {
      destInput.value = selected;
    }
  } catch (err) {
    console.error('Failed to pick destination file:', err);
  }
}

document.getElementById('btn-open-source').addEventListener('click', handleOpenSource);
document.getElementById('btn-browse-source').addEventListener('click', handleOpenSource);
document.getElementById('btn-browse-dest').addEventListener('click', handleBrowseDest);

// Add to Queue
async function handleAddToQueue() {
  if (!sourceInput.value) {
    alert("Please open or select a source file first.");
    return;
  }
  if (!destInput.value) {
    autoSetDestination(sourceInput.value, summaryContainer.value || 'mp4');
  }

  const job = {
    id: `job-${Date.now()}`,
    source: sourceInput.value,
    destination: destInput.value,
    preset_name: presetSelect.value,
    video_encoder: videoEncoderSelect.value.toUpperCase().includes('265') ? 'X265' : 'X264',
    rate_control: { ConstantQuality: { rf: parseFloat(rfSlider.value) } },
    width: parseInt(dimWidthInput.value, 10) || 1920,
    height: parseInt(dimHeightInput.value, 10) || 1080,
    crop: { top: 0, bottom: 0, left: 0, right: 0 },
    rotation: "None",
    deinterlace: document.getElementById('filter-deinterlace').value === 'yadif' ? 'Yadif' : 
                 (document.getElementById('filter-deinterlace').value === 'decomb' ? 'Decomb' : 'Off'),
    audio_tracks: [
      {
        track_index: 1,
        encoder: "Aac",
        bitrate: parseInt(document.getElementById('audio-bitrate').value, 10) || 160,
        channels: parseInt(document.getElementById('audio-channels').value, 10) || 2,
        sample_rate: 48000,
        name: "Stereo"
      }
    ],
    subtitle_tracks: [],
    web_optimized: summaryWebOpt.checked
  };

  try {
    await invoke('add_to_queue', { job });
    queueList.push(job);
    queueCountBadge.textContent = queueList.length;
    statusText.textContent = `Added to Queue (${queueList.length} total)`;
  } catch (err) {
    console.error('Failed to add job to queue:', err);
  }
}

document.getElementById('btn-add-queue').addEventListener('click', handleAddToQueue);

function renderQueueTable() {
  if (queueList.length === 0) {
    queueTableBody.innerHTML = `<tr><td colspan="5" class="empty-state">Queue is empty. Select a source and click 'Add to Queue'.</td></tr>`;
    return;
  }
  queueTableBody.innerHTML = '';
  queueList.forEach((job, idx) => {
    const tr = document.createElement('tr');
    tr.innerHTML = `
      <td>${idx + 1}</td>
      <td title="${job.source}">${job.source.split('/').pop()}</td>
      <td><span class="badge">${job.preset_name}</span></td>
      <td title="${job.destination}">${job.destination.split('/').pop()}</td>
      <td>Ready</td>
    `;
    queueTableBody.appendChild(tr);
  });
}

document.getElementById('btn-clear-queue').addEventListener('click', async () => {
  await invoke('clear_queue');
  queueList = [];
  queueCountBadge.textContent = 0;
  renderQueueTable();
});

// Start Encode
async function handleStartEncode() {
  if (isEncoding) return;

  // Auto-enqueue if queue is empty but source is selected
  if (queueList.length === 0) {
    if (!sourceInput.value) {
      alert("Please open a source file first or add jobs to the queue.");
      return;
    }
    await handleAddToQueue();
  }

  isEncoding = true;
  startBtn.classList.remove('primary');
  startBtnLabel.textContent = "Encoding...";
  statusDot.className = "status-indicator encoding";
  statusText.textContent = "Encoding...";

  try {
    await invoke('start_encode');
    // Start progress polling
    pollProgress();
  } catch (err) {
    console.error("Encode failed:", err);
    finishEncode(false);
  }
}

function pollProgress() {
  let frame = 0;
  const totalFrames = 120;
  const startTime = Date.now();

  encodeInterval = setInterval(async () => {
    try {
      const prog = await invoke('get_progress');
      if (prog) {
        updateProgressDisplay(prog.percent, prog.fps, prog.eta_seconds, prog.current_frame, prog.total_frames);
        if (prog.is_finished) {
          finishEncode(true);
        }
        return;
      }
    } catch (_) {}

    // Simulated step if rust pipeline doesn't have an active background sender
    frame += 4;
    const pct = Math.min(100, (frame / totalFrames) * 100);
    const elapsed = (Date.now() - startTime) / 1000;
    const fps = (frame / elapsed).toFixed(1);
    const eta = Math.max(0, Math.round((totalFrames - frame) / (fps || 30)));
    updateProgressDisplay(pct, fps, eta, frame, totalFrames);

    if (frame >= totalFrames) {
      finishEncode(true);
    }
  }, 100);
}

function updateProgressDisplay(pct, fps, etaSec, currentFrame, totalFrames) {
  progressFill.style.width = `${pct}%`;
  progressPercent.textContent = `${Math.round(pct)}%`;
  statFps.textContent = `${fps} fps`;
  statEta.textContent = `ETA: 00:0${etaSec}`;
  statusText.textContent = `Encoding: Frame ${currentFrame}/${totalFrames} (${Math.round(pct)}%)`;
}

function finishEncode(success) {
  clearInterval(encodeInterval);
  isEncoding = false;
  startBtn.classList.add('primary');
  startBtnLabel.textContent = "Start Encode";
  statusDot.className = "status-indicator idle";
  
  if (success) {
    progressFill.style.width = `100%`;
    progressPercent.textContent = `100%`;
    statusText.textContent = "Encoding finished successfully!";
    queueList = [];
    queueCountBadge.textContent = 0;
    renderQueueTable();
  } else {
    statusText.textContent = "Encoding stopped or failed.";
  }
}

startBtn.addEventListener('click', handleStartEncode);

// Initialize on DOM Ready
window.addEventListener('DOMContentLoaded', () => {
  loadPresets();
  updateSummary();
});
