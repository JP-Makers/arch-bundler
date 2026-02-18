# 🎵🎬 FFmpeg Media Tool - Interactive Bash Script

A colorful, terminal-based tool to **extract**, **add**, or **modify** audio & subtitles in video files using `ffmpeg`. Built with an eye-catching interface and menu-driven interaction.

---

## ✨ Features

- 🎧 **Extract Audio** (Choose stream & bitrate)
- ➕ **Add Audio** (Merge new audio tracks)
- ⭐ **Set Default Audio** (Make a selected stream default)
- ❌ **Remove Unwanted Audio** (Drop specific audio tracks)
- 📝 **Extract Subtitles** (.srt, .vtt, etc.)
- ➕ **Add Subtitles** (Embed external subtitle files)
- ⭐ **Set Default Subtitle**
- 🔁 **Batch Convert MKV ➡️ MP4**

## ⚙️ Requirements

- `bash`
- `ffmpeg`
- Terminal with color support (for full experience)

Install `ffmpeg`:
```bash
sudo apt install ffmpeg    # Debian/Ubuntu
sudo pacman -S ffmpeg      # Arch Linux
```

## 📦 Usage
```
chmod +x avedit
./avedit <input_video>
```
Example: ./avedit video.mkv music.aac

## 🧾 Menu Options

When you run the script, a visually styled menu will appear with the following choices:
```
1. Extract Audio         → Extract a specific audio stream and save as MP3/AAC/etc.
2. Add Audio             → Add an external audio file (e.g., background music)
3. Default Audio         → Set a selected audio stream as default
4. Unwanted Audio Remove → Remove specific audio streams
5. Extract Subtitle      → Extract subtitles from video (SRT/VTT/etc.)
6. Add Subtitle          → Add external subtitles to the video
7. Default Subtitle      → Set a specific subtitle track as default
8. MKV to MP4            → Convert all MKV files in the folder to MP4
```
## 🗃️ Output File Prompt

### Output File Name
Just the name without extension (e.g., new_audio)

### Output File Type
Provide the desired file extension (e.g., mp3, aac, mp4, srt)

```
Enter the Output File Name : new_audio
Enter the Output File Type : mp3
```

The script then constructs the final file name like:
```new_audio.mp3```
