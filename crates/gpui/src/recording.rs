//! Frame-by-frame recording utilities for visual tests and PR demo GIFs.

use std::{fs::File, io::BufWriter, path::Path, time::Duration};

use image::{Delay, Frame, RgbaImage, codecs::gif::{GifEncoder, Repeat}};

/// A single captured frame with its display duration.
pub struct CapturedFrame {
    /// The rendered pixel data for this frame.
    pub image: RgbaImage,
    /// How long this frame is displayed in the output.
    pub delay: Duration,
}

/// Accumulates rendered frames and exports them as a GIF or directory of PNGs.
pub struct FrameRecorder {
    frames: Vec<CapturedFrame>,
    default_frame_delay: Duration,
}

impl FrameRecorder {
    /// Creates a new recorder. All frames pushed via [`push_frame`] use `default_frame_delay`.
    pub fn new(default_frame_delay: Duration) -> Self {
        Self {
            frames: Vec::new(),
            default_frame_delay,
        }
    }

    /// Appends a frame using the recorder's default delay.
    pub fn push_frame(&mut self, image: RgbaImage) {
        let delay = self.default_frame_delay;
        self.frames.push(CapturedFrame { image, delay });
    }

    /// Appends a frame with an explicit delay, overriding the default.
    pub fn push_frame_with_delay(&mut self, image: RgbaImage, delay: Duration) {
        self.frames.push(CapturedFrame { image, delay });
    }

    /// Returns the number of frames accumulated so far.
    pub fn frame_count(&self) -> usize {
        self.frames.len()
    }

    /// Encodes all frames as an infinitely looping animated GIF at `path`.
    pub fn export_gif(&self, path: &Path) -> anyhow::Result<()> {
        let file = File::create(path)?;
        let mut encoder = GifEncoder::new_with_speed(BufWriter::new(file), 10);
        encoder.set_repeat(Repeat::Infinite)?;
        for captured in &self.frames {
            let delay = Delay::from_saturating_duration(captured.delay);
            let frame = Frame::from_parts(captured.image.clone(), 0, 0, delay);
            encoder.encode_frame(frame)?;
        }
        Ok(())
    }

    /// Writes each frame as a numbered PNG into `directory` (must already exist).
    ///
    /// Files are named `frame_0000.png`, `frame_0001.png`, etc. Suitable for
    /// piping into ffmpeg or gifski for high-quality output.
    pub fn export_frames_to_directory(&self, directory: &Path) -> anyhow::Result<()> {
        for (index, captured) in self.frames.iter().enumerate() {
            let filename = format!("frame_{:04}.png", index);
            captured.image.save(directory.join(filename))?;
        }
        Ok(())
    }
}
