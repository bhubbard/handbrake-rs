const PRESETS = [
  {
    name: "Fast 1080p30",
    category: "General",
    encoder: "x264",
    rf: 22.0,
    container: "MP4",
    res: "1920x1080",
    audio: "AAC 160k",
    desc: "Fast, standard H.264 encode for 1080p at 30 fps"
  },
  {
    name: "Fast 720p30",
    category: "General",
    encoder: "x264",
    rf: 22.0,
    container: "MP4",
    res: "1280x720",
    audio: "AAC 128k",
    desc: "Fast 720p H.264 encode for compact files"
  },
  {
    name: "HQ 1080p30 Surround",
    category: "General",
    encoder: "x264",
    rf: 20.0,
    container: "MP4",
    res: "1920x1080",
    audio: "AAC 5.1 (320k)",
    desc: "High quality H.264 with 5.1 surround sound pass-through"
  },
  {
    name: "Fast 4K HEVC",
    category: "General",
    encoder: "x265",
    rf: 24.0,
    container: "MP4",
    res: "3840x2160",
    audio: "AAC 192k",
    desc: "Ultra HD 4K H.265 encode for high dynamic range content"
  },
  {
    name: "Discord Nitro 1080p",
    category: "Web",
    encoder: "x264",
    rf: 23.0,
    container: "MP4",
    res: "1920x1080",
    audio: "Opus 128k",
    desc: "Optimized for 50MB/100MB Discord upload limits at 1080p"
  },
  {
    name: "Discord Small 720p",
    category: "Web",
    encoder: "x264",
    rf: 25.0,
    container: "MP4",
    res: "1280x720",
    audio: "Opus 96k",
    desc: "Strictly tuned for free 25MB Discord file limits"
  },
  {
    name: "YouTube 1080p60",
    category: "Web",
    encoder: "x264",
    rf: 21.0,
    container: "MP4",
    res: "1920x1080",
    audio: "AAC 192k",
    desc: "Fluid 60 fps encode ideal for game capture and screen recordings"
  },
  {
    name: "Apple 1080p30 Surround",
    category: "Devices",
    encoder: "vt_h264",
    rf: 22.0,
    container: "MP4",
    res: "1920x1080",
    audio: "AAC 5.1 (160k)",
    desc: "Compatible with Apple TV, iPad, iPhone, and Mac"
  },
  {
    name: "Apple 4K HEVC",
    category: "Devices",
    encoder: "vt_h265",
    rf: 23.0,
    container: "MP4",
    res: "3840x2160",
    audio: "AAC 5.1 (256k)",
    desc: "Hardware-accelerated Apple Silicon VideoToolbox HEVC 4K"
  },
  {
    name: "H.265 MKV 1080p",
    category: "Matroska",
    encoder: "x265",
    rf: 22.0,
    container: "MKV",
    res: "1920x1080",
    audio: "Opus 160k",
    desc: "Matroska container with high efficiency H.265 video"
  },
  {
    name: "AV1 MKV 1080p",
    category: "Matroska",
    encoder: "svt_av1",
    rf: 26.0,
    container: "MKV",
    res: "1920x1080",
    audio: "Opus 128k",
    desc: "Next-generation royalty-free AV1 video in Matroska"
  },
  {
    name: "Production Max ProRes",
    category: "Production",
    encoder: "prores",
    rf: 10.0,
    container: "MKV",
    res: "Source Resolution",
    audio: "PCM Copy",
    desc: "Apple ProRes 422 HQ visually lossless video for editing suites"
  }
];

document.addEventListener("DOMContentLoaded", () => {
  initPresets();
  initCropStudio();
  initCliBuilder();
  initQueueSimulator();
});

function initPresets() {
  const container = document.getElementById("presets-grid");
  const buttons = document.querySelectorAll(".cat-btn");

  function render(cat) {
    container.innerHTML = "";
    const filtered = cat === "all" ? PRESETS : PRESETS.filter(p => p.category === cat);

    filtered.forEach(p => {
      const card = document.createElement("div");
      card.className = "preset-card";
      card.innerHTML = `
        <div>
          <div class="preset-header">
            <span class="preset-name">${p.name}</span>
            <span class="badge-codec">${p.encoder}</span>
          </div>
          <p class="preset-desc">${p.desc}</p>
        </div>
        <div class="preset-specs">
          <span>RF ${p.rf.toFixed(1)}</span>
          <span>•</span>
          <span>${p.res}</span>
          <span>•</span>
          <span>${p.container}</span>
          <span>•</span>
          <span>${p.audio}</span>
        </div>
      `;
      container.appendChild(card);
    });
  }

  buttons.forEach(btn => {
    btn.addEventListener("click", () => {
      buttons.forEach(b => b.classList.remove("active"));
      btn.classList.add("active");
      render(btn.dataset.cat);
    });
  });

  render("all");
}

function initCropStudio() {
  const topInput = document.getElementById("crop-top");
  const botInput = document.getElementById("crop-bot");
  const leftInput = document.getElementById("crop-left");
  const rightInput = document.getElementById("crop-right");

  const topVal = document.getElementById("top-val");
  const botVal = document.getElementById("bot-val");
  const leftVal = document.getElementById("left-val");
  const rightVal = document.getElementById("right-val");

  const barTop = document.getElementById("bar-top");
  const barBot = document.getElementById("bar-bot");
  const barLeft = document.getElementById("bar-left");
  const barRight = document.getElementById("bar-right");

  const resDisplay = document.getElementById("res-display");
  const geomSummary = document.getElementById("geom-summary");
  const autoBtn = document.getElementById("autocrop-btn");

  function update() {
    const t = parseInt(topInput.value);
    const b = parseInt(botInput.value);
    const l = parseInt(leftInput.value);
    const r = parseInt(rightInput.value);

    topVal.textContent = t;
    botVal.textContent = b;
    leftVal.textContent = l;
    rightVal.textContent = r;

    // Visual scale: map 140px crop to visual 40px
    barTop.style.height = `${(t / 140) * 45}px`;
    barBot.style.height = `${(b / 140) * 45}px`;
    barLeft.style.width = `${(l / 140) * 55}px`;
    barRight.style.width = `${(r / 140) * 55}px`;

    const activeW = 1920 - (l + r);
    const activeH = 1080 - (t + b);
    const aspect = (activeW / Math.max(1, activeH)).toFixed(2);

    resDisplay.textContent = `${activeW} x ${activeH} (${aspect}:1)`;
    geomSummary.innerHTML = `Cropped Surface: <strong>${activeW}x${activeH}</strong> | DAR: <strong>${aspect}:1</strong> | Modulus: <strong>2 (Aligned)</strong>`;
  }

  [topInput, botInput, leftInput, rightInput].forEach(inp => {
    inp.addEventListener("input", update);
  });

  autoBtn.addEventListener("click", () => {
    // Simulate detecting 2.39:1 Cinemascope bars on 16:9 1080p
    topInput.value = 138;
    botInput.value = 138;
    leftInput.value = 0;
    rightInput.value = 0;
    update();
  });

  update();
}

function initCliBuilder() {
  const inpFile = document.getElementById("cli-input");
  const outFile = document.getElementById("cli-output");
  const presetSel = document.getElementById("cli-preset");
  const rfSlider = document.getElementById("cli-rf");
  const rfVal = document.getElementById("cli-rf-val");
  const decombChk = document.getElementById("cli-decomb");
  const webChk = document.getElementById("cli-web");
  const cmdOutput = document.getElementById("cli-cmd-output");
  const copyBtn = document.getElementById("copy-cli-btn");

  rfSlider.addEventListener("input", e => rfVal.textContent = parseFloat(e.target.value).toFixed(1));

  function generate() {
    let cmd = `HandBrakeCLI -i "${inpFile.value}" -o "${outFile.value}" -Z "${presetSel.value}" -q ${parseFloat(rfSlider.value).toFixed(1)}`;
    if (decombChk.checked) cmd += " --decomb";
    if (webChk.checked) cmd += " --web-optimized";
    cmdOutput.textContent = cmd;
  }

  [inpFile, outFile, presetSel, rfSlider, decombChk, webChk].forEach(el => {
    el.addEventListener("input", generate);
    el.addEventListener("change", generate);
  });

  copyBtn.addEventListener("click", () => {
    navigator.clipboard.writeText(cmdOutput.textContent);
    copyBtn.textContent = "✅ Copied to Clipboard!";
    setTimeout(() => copyBtn.textContent = "📋 Copy Command", 2000);
  });

  generate();
}

function initQueueSimulator() {
  const container = document.getElementById("queue-jobs-container");
  const startBtn = document.getElementById("start-queue-btn");
  const statusBadge = document.getElementById("queue-status-badge");

  const sampleJobs = [
    { title: "Job 1: Summer_Holiday_4K.mov", preset: "Fast 1080p30", out: "Holiday_1080p.mp4", duration: 120 },
    { title: "Job 2: Gameplay_Stream_Raw.mkv", preset: "Discord Nitro 1080p", out: "Gameplay_Discord.mp4", duration: 80 },
    { title: "Job 3: Master_Interview_ProRes.mov", preset: "Apple 4K HEVC", out: "Interview_HEVC.mp4", duration: 150 }
  ];

  function renderJobs() {
    container.innerHTML = "";
    sampleJobs.forEach((job, idx) => {
      const el = document.createElement("div");
      el.className = "job-item";
      el.id = `sim-job-${idx}`;
      el.innerHTML = `
        <div class="job-meta">
          <span>${job.title} <span style="color: #94a3b8; font-weight: 400;">(${job.preset} ➔ ${job.out})</span></span>
          <span id="job-tele-${idx}" class="job-telemetry">Queued</span>
        </div>
        <div class="progress-bar-track">
          <div id="job-fill-${idx}" class="progress-bar-fill"></div>
        </div>
      `;
      container.appendChild(el);
    });
  }

  renderJobs();

  startBtn.addEventListener("click", () => {
    startBtn.disabled = true;
    statusBadge.textContent = "Processing Queue";
    statusBadge.className = "status-badge running";

    let currentJob = 0;

    function runNextJob() {
      if (currentJob >= sampleJobs.length) {
        statusBadge.textContent = "All Jobs Finished";
        statusBadge.className = "status-badge idle";
        startBtn.disabled = false;
        startBtn.textContent = "↺ Run Queue Again";
        return;
      }

      const fill = document.getElementById(`job-fill-${currentJob}`);
      const tele = document.getElementById(`job-tele-${currentJob}`);
      let pct = 0;

      const timer = setInterval(() => {
        pct += 3.5;
        if (pct <= 100) {
          fill.style.width = `${pct}%`;
          const fps = (110 + Math.random() * 25).toFixed(1);
          const eta = Math.max(0, Math.round((100 - pct) * 0.08));
          tele.textContent = `${pct.toFixed(0)}% | ${fps} fps | ETA: ${eta}s | 2,450 kbps`;
        } else {
          clearInterval(timer);
          fill.style.width = "100%";
          tele.textContent = "Complete (120 fps avg)";
          currentJob++;
          runNextJob();
        }
      }, 40);
    }

    runNextJob();
  });
}
