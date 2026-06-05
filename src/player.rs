use std::io::{self, Write};
use std::mem;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use image::RgbImage;
use scopeguard::defer;

use crate::audio::AudioHandle;
use crate::renderer::render_crlf;
use crate::resizer::{resize, ResizeOptions};
use crate::source::FrameSource;

pub fn play(
    source: &mut dyn FrameSource,
    opts: &ResizeOptions,
    audio_path: Option<&Path>,
    grayscale: bool,
) -> Result<(), String> {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    if let Err(e) = ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }) {
        // Tolerate duplicate registration (e.g. multiple calls in the same process during tests).
        if !matches!(e, ctrlc::Error::MultipleHandlers) {
            return Err(format!("Failed to set Ctrl-C handler: {e}"));
        }
    }

    // Held only for Drop side effect (kills ffmpeg on exit/seek). mem::replace drops old handle.
    let mut audio: Option<AudioHandle> = audio_path.and_then(AudioHandle::start);
    let _ = &audio;

    let _ = enable_raw_mode();

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    // Hide cursor; restore it on any exit path.
    handle.write_all(b"\x1b[?25l").unwrap();
    defer! {
        let mut h = io::stdout();
        let _ = h.write_all(b"\x1b[?25h\x1b[0m\n");
        let _ = h.flush();
        let _ = disable_raw_mode();
    }

    let mut raw_buf = RgbImage::new(1, 1);
    let mut out_buf: Vec<u8> = Vec::with_capacity(1 << 20);

    while running.load(Ordering::SeqCst) {
        let t0 = Instant::now();

        let frame_delay = match source.next_frame(&mut raw_buf) {
            Some(d) => d,
            None => break,
        };

        if !running.load(Ordering::SeqCst) {
            break;
        }

        let resized = resize(&raw_buf, opts);

        out_buf.clear();
        out_buf.extend_from_slice(b"\x1b[H"); // cursor home — overwrite in place, no flash
        render_crlf(&resized, &mut out_buf, grayscale);

        handle.write_all(&out_buf).unwrap();
        handle.flush().unwrap();

        let elapsed = t0.elapsed();
        if let Some(sleep_dur) = frame_delay.checked_sub(elapsed) {
            std::thread::sleep(sleep_dur);
        }

        // Non-blocking key event check — handle seek and quit.
        while event::poll(Duration::from_millis(0)).unwrap_or(false) {
            if let Ok(Event::Key(key)) = event::read() {
                match key.code {
                    KeyCode::Right => {
                        if let Some(new_pos) = source.seek_by(10.0) {
                            drop(mem::replace(&mut audio, audio_path.and_then(|p| AudioHandle::start_at(p, new_pos))));
                        }
                    }
                    KeyCode::Left => {
                        if let Some(new_pos) = source.seek_by(-10.0) {
                            drop(mem::replace(&mut audio, audio_path.and_then(|p| AudioHandle::start_at(p, new_pos))));
                        }
                    }
                    KeyCode::Char('q') => running.store(false, Ordering::SeqCst),
                    KeyCode::Char('c')
                        if key.modifiers.contains(KeyModifiers::CONTROL) =>
                    {
                        running.store(false, Ordering::SeqCst);
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{Rgb, RgbImage};
    use std::time::Duration;

    struct MockSource {
        frames: Vec<RgbImage>,
        index: usize,
    }

    impl MockSource {
        fn new(frames: Vec<RgbImage>) -> Self {
            Self { frames, index: 0 }
        }
    }

    impl FrameSource for MockSource {
        fn next_frame(&mut self, buf: &mut RgbImage) -> Option<Duration> {
            if self.index >= self.frames.len() {
                return None;
            }
            let src = &self.frames[self.index];
            self.index += 1;
            let (w, h) = (src.width(), src.height());
            if buf.width() != w || buf.height() != h {
                *buf = RgbImage::new(w, h);
            }
            for (x, y, px) in src.enumerate_pixels() {
                buf.put_pixel(x, y, *px);
            }
            Some(Duration::from_millis(0))
        }
    }

    #[test]
    fn play_empty_source_returns_ok() {
        let mut source = MockSource::new(vec![]);
        let opts = ResizeOptions { width: Some(4), height: Some(2), scale: None };
        assert!(play(&mut source, &opts, None, false).is_ok());
        assert_eq!(source.index, 0, "no frames should be consumed from empty source");
    }

    #[test]
    fn play_exhausts_mock_source() {
        let mut f1 = RgbImage::new(4, 2);
        f1.put_pixel(0, 0, Rgb([255, 0, 0]));
        let f2 = RgbImage::new(4, 2);
        let f3 = RgbImage::new(4, 2);

        let mut source = MockSource::new(vec![f1, f2, f3]);
        let opts = ResizeOptions { width: Some(4), height: Some(2), scale: None };
        play(&mut source, &opts, None, false).unwrap();

        assert_eq!(source.index, 3, "all 3 frames should have been consumed");
    }

    #[test]
    fn play_handles_mid_stream_dimension_change() {
        let small = RgbImage::new(4, 2);
        let large = RgbImage::new(8, 4);

        let mut source = MockSource::new(vec![small, large]);
        let opts = ResizeOptions { width: Some(4), height: Some(2), scale: None };
        play(&mut source, &opts, None, false).unwrap();

        assert_eq!(source.index, 2, "both frames with different sizes should be consumed");
    }
}
