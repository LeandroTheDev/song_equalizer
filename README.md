# 🎵 Song Equalizer

**Song Equalizer** is a Rust-based command-line tool that uses **FFmpeg** to normalize the volume of all audio files in a selected directory. Perfect for fixing songs that are too loud or too quiet by bringing them all to a consistent, standard loudness.

> Compatible with **Windows** and **Linux**.

---

## ✨ Features

- 📂 Automatically scans a directory for audio files
- 📊 Normalizes volume to a target loudness level
- 🎧 Uses loudness analysis via FFmpeg (EBU R128)
- ⚙️ Supports custom equalization settings:
  - `Integrated loudness target (I)` – Sets desired average volume (LUFS)
  - `True peak limit (TP)` – Sets max allowed peak (dBTP)
  - `Loudness range (LRA)` – Controls the dynamic range
  - `Audio quality (q:a)` – Sets the output audio quality

---

## 🛠️ Build Requirements

- **Rust** (latest stable version recommended)
- **FFmpeg** installed and available in your system's PATH (`ffmpeg` and `ffprobe`)

---

## 📦 Building
```bash
git clone https://github.com/your-username/song-equalizer.git
cd song-equalizer
cargo build --release
```

## 🚀 Usage
- Download the [windows binary](https://github.com/LeandroTheDev/song_equalizer/releases/download/1.0/song.equalizer.windows.zip) or [linux binary](https://github.com/LeandroTheDev/song_equalizer/releases/download/1.0/song.equalizer.linux.zip)
- If you are using windows you will also need to install [microsoft vcredist](https://aka.ms/vs/17/release/vc_redist.x64.exe)
- Run song_equalizer
- Copy the directory that you want to equalize the songs
- Proceed to the next steps
- Result output will be stored in the same directory folder named ``result``
