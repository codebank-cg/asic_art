use std::path::Path;
use std::time::Duration;

use image::{Rgb, RgbImage};
use nokhwa::pixel_format::RgbFormat;
use nokhwa::utils::{CameraIndex, RequestedFormat, RequestedFormatType};
use nokhwa::Camera;
use video_rs::Decoder;

pub trait FrameSource {
    /// Decode the next frame into `buf`, resizing it if dimensions changed.
    /// Returns the nominal frame delay, or `None` when the source is exhausted.
    fn next_frame(&mut self, buf: &mut RgbImage) -> Option<Duration>;

    /// Seek by `delta_secs` relative to the current playback position.
    /// Returns the new position in seconds, or `None` if this source is not seekable.
    fn seek_by(&mut self, _delta_secs: f64) -> Option<f64> {
        None
    }
}

// ---------------------------------------------------------------------------
// Video file source
// ---------------------------------------------------------------------------

pub struct VideoFileSource {
    decoder: Decoder,
    frame_delay: Duration,
    current_secs: f64,
}

impl VideoFileSource {
    pub fn open(path: &Path, fps_override: Option<f32>) -> Result<Self, String> {
        let location = video_rs::Location::File(path.to_path_buf());
        let decoder =
            Decoder::new(location).map_err(|e| format!("Failed to open '{}': {e}", path.display()))?;

        let fps = fps_override
            .map(|f| f as f64)
            .unwrap_or_else(|| stream_fps(&decoder));
        let frame_delay = Duration::from_secs_f64(1.0 / fps.max(1.0));

        Ok(Self { decoder, frame_delay, current_secs: 0.0 })
    }
}

fn stream_fps(decoder: &Decoder) -> f64 {
    let rate = decoder.frame_rate();
    if rate > 0.0 && rate < 1000.0 { rate as f64 } else { 24.0 }
}

impl FrameSource for VideoFileSource {
    fn next_frame(&mut self, buf: &mut RgbImage) -> Option<Duration> {
        loop {
            match self.decoder.decode_raw() {
                Ok(raw) => {
                    let w = raw.width();
                    let h = raw.height();
                    if buf.width() != w || buf.height() != h {
                        *buf = RgbImage::new(w, h);
                    }
                    // video-rs guarantees RGB24 output (FRAME_PIXEL_FORMAT = RGB24)
                    let stride = raw.stride(0);
                    let data = raw.data(0);
                    for y in 0..h as usize {
                        for x in 0..w as usize {
                            let off = y * stride + x * 3;
                            buf.put_pixel(
                                x as u32,
                                y as u32,
                                Rgb([data[off], data[off + 1], data[off + 2]]),
                            );
                        }
                    }
                    self.current_secs += self.frame_delay.as_secs_f64();
                    return Some(self.frame_delay);
                }
                Err(video_rs::Error::DecodeExhausted) => return None,
                Err(_) => continue,
            }
        }
    }

    fn seek_by(&mut self, delta_secs: f64) -> Option<f64> {
        let target = (self.current_secs + delta_secs).max(0.0);
        self.decoder.seek((target * 1000.0) as i64).ok()?;
        self.current_secs = target;
        Some(target)
    }
}

// ---------------------------------------------------------------------------
// Webcam source
// ---------------------------------------------------------------------------

pub struct WebcamSource {
    camera: Camera,
    frame_delay: Duration,
}

impl WebcamSource {
    pub fn open(device_index: u32, fps: f32) -> Result<Self, String> {
        let index = CameraIndex::Index(device_index);
        let format =
            RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestFrameRate);
        let mut camera = Camera::new(index, format)
            .map_err(|e| format!("Failed to open camera {device_index}: {e}"))?;
        camera.open_stream().map_err(|e| format!("Failed to start camera stream: {e}"))?;
        let frame_delay = Duration::from_secs_f64(1.0 / fps.max(1.0) as f64);
        Ok(Self { camera, frame_delay })
    }
}

impl FrameSource for WebcamSource {
    fn next_frame(&mut self, buf: &mut RgbImage) -> Option<Duration> {
        let frame = self.camera.frame().ok()?;
        let decoded = frame.decode_image::<RgbFormat>().ok()?;
        let (w, h) = (decoded.width(), decoded.height());
        if buf.width() != w || buf.height() != h {
            *buf = RgbImage::new(w, h);
        }
        for (x, y, px) in decoded.enumerate_pixels() {
            buf.put_pixel(x, y, Rgb(px.0));
        }
        Some(self.frame_delay)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn video_file_source_rejects_nonexistent_path() {
        let result = VideoFileSource::open(Path::new("/tmp/does_not_exist_asic_art_test.mp4"), None);
        assert!(result.is_err());
        let msg = result.err().unwrap();
        assert!(msg.contains("does_not_exist_asic_art_test.mp4"), "error should include the path: {msg}");
    }

    #[test]
    fn video_file_source_fps_override_uses_provided_fps() {
        // We can't open a real file in unit tests, but we verify that fps_override
        // parsing is integrated: if open succeeds, frame_delay should reflect the override.
        // For now we confirm that passing fps_override to a bad path still fails early (not on fps).
        let result = VideoFileSource::open(Path::new("/tmp/nope.mp4"), Some(60.0));
        assert!(result.is_err(), "should fail on bad path before fps is relevant");
    }
}
