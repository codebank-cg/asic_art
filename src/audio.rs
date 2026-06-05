use std::io::Read;
use std::path::Path;
use std::process::{Child, ChildStdout, Command, Stdio};
use std::time::Duration;

use rodio::source::Source;
use rodio::{OutputStream, OutputStreamHandle, Sink};

pub struct AudioHandle {
    _stream: OutputStream,
    _handle: OutputStreamHandle,
    _sink: Sink,
    process: Child,
}

impl AudioHandle {
    /// Spawns ffmpeg to decode audio from `path` and plays it via the default output device.
    /// Returns `None` silently on any failure (no ffmpeg in PATH, no audio track, no device, …).
    pub fn start(path: &Path) -> Option<Self> {
        Self::start_at(path, 0.0)
    }

    /// Like [`start`], but begins playback at `start_secs` seconds into the file.
    pub fn start_at(path: &Path, start_secs: f64) -> Option<Self> {
        let path_str = path.to_string_lossy();
        let mut cmd = Command::new("ffmpeg");
        if start_secs > 0.01 {
            cmd.args(["-ss", &format!("{start_secs:.3}")]);
        }
        cmd.args(["-i", &path_str]);
        cmd.args([
            "-f", "s16le", "-ar", "44100", "-ac", "2",
            "-acodec", "pcm_s16le", "pipe:1", "-loglevel", "quiet",
        ]);
        let mut child = cmd
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .ok()?;

        let stdout = child.stdout.take()?;
        let (stream, handle) = OutputStream::try_default().ok()?;
        let sink = Sink::try_new(&handle).ok()?;
        let source = PcmPipeSource::new(stdout);
        sink.append(source.convert_samples::<f32>());
        sink.play();

        Some(Self { _stream: stream, _handle: handle, _sink: sink, process: child })
    }
}

impl Drop for AudioHandle {
    fn drop(&mut self) {
        let _ = self.process.kill();
        let _ = self.process.wait();
    }
}

// ---------------------------------------------------------------------------
// rodio Source backed by raw s16le PCM piped from ffmpeg
// ---------------------------------------------------------------------------

struct PcmPipeSource {
    reader: std::io::BufReader<ChildStdout>,
    buffer: Vec<i16>,
    pos: usize,
}

impl PcmPipeSource {
    fn new(stdout: ChildStdout) -> Self {
        Self {
            reader: std::io::BufReader::with_capacity(65536, stdout),
            buffer: Vec::with_capacity(4096),
            pos: 0,
        }
    }

    fn refill(&mut self) -> bool {
        let mut bytes = [0u8; 8192];
        match self.reader.read(&mut bytes) {
            Ok(0) | Err(_) => false,
            Ok(n) => {
                self.buffer.clear();
                self.pos = 0;
                for chunk in bytes[..(n & !1)].chunks_exact(2) {
                    self.buffer.push(i16::from_le_bytes([chunk[0], chunk[1]]));
                }
                !self.buffer.is_empty()
            }
        }
    }
}

impl Iterator for PcmPipeSource {
    type Item = i16;

    fn next(&mut self) -> Option<i16> {
        if self.pos >= self.buffer.len() && !self.refill() {
            return None;
        }
        let s = self.buffer[self.pos];
        self.pos += 1;
        Some(s)
    }
}

impl Source for PcmPipeSource {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }
    fn channels(&self) -> u16 {
        2
    }
    fn sample_rate(&self) -> u32 {
        44100
    }
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}
