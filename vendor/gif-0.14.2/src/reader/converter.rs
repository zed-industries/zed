use alloc::borrow::Cow;
use alloc::vec::Vec;
use core::iter;
use core::mem;

use super::decoder::{DecodingError, OutputBuffer, PLTE_CHANNELS};
use crate::common::Frame;
use crate::MemoryLimit;

pub(crate) const N_CHANNELS: usize = 4;

/// Output mode for the image data
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ColorOutput {
    /// The decoder expands the image data to 32bit RGBA.
    /// This affects:
    ///
    ///  - The buffer buffer of the `Frame` returned by [`Decoder::read_next_frame`].
    ///  - `Decoder::fill_buffer`, `Decoder::buffer_size` and `Decoder::line_length`.
    RGBA = 0,
    /// The decoder returns the raw indexed data.
    Indexed = 1,
}

pub(crate) type FillBufferCallback<'a> =
    &'a mut dyn FnMut(&mut OutputBuffer<'_>) -> Result<usize, DecodingError>;

/// Deinterlaces and expands to RGBA if needed
pub(crate) struct PixelConverter {
    color_output: ColorOutput,
    buffer: Vec<u8>,
    global_palette: Option<Vec<u8>>,
}

impl PixelConverter {
    pub(crate) const fn new(color_output: ColorOutput) -> Self {
        Self {
            color_output,
            buffer: Vec::new(),
            global_palette: None,
        }
    }

    pub(crate) fn check_buffer_size(
        &self,
        frame: &Frame<'_>,
        memory_limit: &MemoryLimit,
    ) -> Result<usize, DecodingError> {
        let pixel_bytes = memory_limit
            .buffer_size(self.color_output, frame.width, frame.height)
            .ok_or_else(|| DecodingError::OutOfMemory)?;

        debug_assert_eq!(
            pixel_bytes,
            self.buffer_size(frame).unwrap(),
            "Checked computation diverges from required buffer size"
        );
        Ok(pixel_bytes)
    }

    #[inline]
    pub(crate) fn read_frame(
        &mut self,
        frame: &mut Frame<'_>,
        data_callback: FillBufferCallback<'_>,
        memory_limit: &MemoryLimit,
    ) -> Result<(), DecodingError> {
        let pixel_bytes = self.check_buffer_size(frame, memory_limit)?;
        let mut vec = match mem::replace(&mut frame.buffer, Cow::Borrowed(&[])) {
            // reuse buffer if possible without reallocating
            Cow::Owned(mut vec) if vec.capacity() >= pixel_bytes => {
                vec.resize(pixel_bytes, 0);
                vec
            }
            // resizing would realloc anyway, and 0-init is faster than a copy
            _ => vec![0; pixel_bytes],
        };
        self.read_into_buffer(frame, &mut vec, data_callback)?;
        frame.buffer = Cow::Owned(vec);
        frame.interlaced = false;
        Ok(())
    }

    #[inline]
    pub(crate) const fn buffer_size(&self, frame: &Frame<'_>) -> Option<usize> {
        self.line_length(frame).checked_mul(frame.height as usize)
    }

    #[inline]
    pub(crate) const fn line_length(&self, frame: &Frame<'_>) -> usize {
        use self::ColorOutput::{Indexed, RGBA};
        match self.color_output {
            RGBA => frame.width as usize * N_CHANNELS,
            Indexed => frame.width as usize,
        }
    }

    /// Use `read_into_buffer` to deinterlace
    #[inline(never)]
    pub(crate) fn fill_buffer(
        &mut self,
        current_frame: &Frame<'_>,
        mut buf: &mut [u8],
        data_callback: FillBufferCallback<'_>,
    ) -> Result<bool, DecodingError> {
        loop {
            let decode_into = match self.color_output {
                // When decoding indexed data, LZW can write the pixels directly
                ColorOutput::Indexed => &mut buf[..],
                // When decoding RGBA, the pixel data will be expanded by a factor of 4,
                // and it's simpler to decode indexed pixels to another buffer first
                ColorOutput::RGBA => {
                    let buffer_size = buf.len() / N_CHANNELS;
                    if buffer_size == 0 {
                        return Err(DecodingError::format("odd-sized buffer"));
                    }
                    if self.buffer.len() < buffer_size {
                        self.buffer.resize(buffer_size, 0);
                    }
                    &mut self.buffer[..buffer_size]
                }
            };
            match data_callback(&mut OutputBuffer::Slice(decode_into))? {
                0 => return Ok(false),
                bytes_decoded => {
                    match self.color_output {
                        ColorOutput::RGBA => {
                            let transparent = current_frame.transparent;
                            let palette: &[u8] = current_frame
                                .palette
                                .as_deref()
                                .or(self.global_palette.as_deref())
                                .unwrap_or_default(); // next_frame_info already checked it won't happen

                            let (pixels, rest) = buf.split_at_mut(bytes_decoded * N_CHANNELS);
                            buf = rest;

                            for (rgba, idx) in pixels
                                .chunks_exact_mut(N_CHANNELS)
                                .zip(self.buffer.iter().copied().take(bytes_decoded))
                            {
                                let plte_offset = PLTE_CHANNELS * idx as usize;
                                if let Some(colors) =
                                    palette.get(plte_offset..plte_offset + PLTE_CHANNELS)
                                {
                                    rgba[0] = colors[0];
                                    rgba[1] = colors[1];
                                    rgba[2] = colors[2];
                                    rgba[3] = if let Some(t) = transparent {
                                        if t == idx {
                                            0x00
                                        } else {
                                            0xFF
                                        }
                                    } else {
                                        0xFF
                                    };
                                }
                            }
                        }
                        ColorOutput::Indexed => {
                            buf = &mut buf[bytes_decoded..];
                        }
                    }
                    if buf.is_empty() {
                        return Ok(true);
                    }
                }
            }
        }
    }

    pub(crate) fn global_palette(&self) -> Option<&[u8]> {
        self.global_palette.as_deref()
    }

    pub(crate) fn set_global_palette(&mut self, palette: Vec<u8>) {
        self.global_palette = if !palette.is_empty() {
            Some(palette)
        } else {
            None
        };
    }

    /// Applies deinterlacing
    ///
    /// Set `frame.interlaced = false` afterwards if you're putting the buffer back into the `Frame`
    pub(crate) fn read_into_buffer(
        &mut self,
        frame: &Frame<'_>,
        buf: &mut [u8],
        data_callback: FillBufferCallback<'_>,
    ) -> Result<(), DecodingError> {
        if frame.interlaced {
            let width = self.line_length(frame);
            for row in (InterlaceIterator {
                len: frame.height,
                next: 0,
                pass: 0,
            }) {
                // this can't overflow 32-bit, because row never equals (maximum) height
                let start = row * width;
                // Handle a too-small buffer and 32-bit usize overflow without panicking
                let line = buf
                    .get_mut(start..)
                    .and_then(|b| b.get_mut(..width))
                    .ok_or_else(|| DecodingError::format("buffer too small"))?;
                if !self.fill_buffer(frame, line, data_callback)? {
                    return Err(DecodingError::format("image truncated"));
                }
            }
        } else {
            let buf = self
                .buffer_size(frame)
                .and_then(|buffer_size| buf.get_mut(..buffer_size))
                .ok_or_else(|| DecodingError::format("buffer too small"))?;
            if !self.fill_buffer(frame, buf, data_callback)? {
                return Err(DecodingError::format("image truncated"));
            }
        };
        Ok(())
    }
}

struct InterlaceIterator {
    len: u16,
    next: usize,
    pass: usize,
}

impl iter::Iterator for InterlaceIterator {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }
        // although the pass never goes out of bounds thanks to len==0,
        // the optimizer doesn't see it. get()? avoids costlier panicking code.
        let mut next = self.next + *[8, 8, 4, 2].get(self.pass)?;
        while next >= self.len as usize {
            debug_assert!(self.pass < 4);
            next = *[4, 2, 1, 0].get(self.pass)?;
            self.pass += 1;
        }
        mem::swap(&mut next, &mut self.next);
        Some(next)
    }
}

#[cfg(test)]
mod test {
    use alloc::vec::Vec;

    use super::InterlaceIterator;

    #[rustfmt::skip]
    #[test]
    fn test_interlace_iterator() {
        for &(len, expect) in &[
            (0, &[][..]),
            (1, &[0][..]),
            (2, &[0, 1][..]),
            (3, &[0, 2, 1][..]),
            (4, &[0, 2, 1, 3][..]),
            (5, &[0, 4, 2, 1, 3][..]),
            (6, &[0, 4, 2, 1, 3, 5][..]),
            (7, &[0, 4, 2, 6, 1, 3, 5][..]),
            (8, &[0, 4, 2, 6, 1, 3, 5, 7][..]),
            (9, &[0, 8, 4, 2, 6, 1, 3, 5, 7][..]),
            (10, &[0, 8, 4, 2, 6, 1, 3, 5, 7, 9][..]),
            (11, &[0, 8, 4, 2, 6, 10, 1, 3, 5, 7, 9][..]),
            (12, &[0, 8, 4, 2, 6, 10, 1, 3, 5, 7, 9, 11][..]),
            (13, &[0, 8, 4, 12, 2, 6, 10, 1, 3, 5, 7, 9, 11][..]),
            (14, &[0, 8, 4, 12, 2, 6, 10, 1, 3, 5, 7, 9, 11, 13][..]),
            (15, &[0, 8, 4, 12, 2, 6, 10, 14, 1, 3, 5, 7, 9, 11, 13][..]),
            (16, &[0, 8, 4, 12, 2, 6, 10, 14, 1, 3, 5, 7, 9, 11, 13, 15][..]),
            (17, &[0, 8, 16, 4, 12, 2, 6, 10, 14, 1, 3, 5, 7, 9, 11, 13, 15][..]),
        ] {
            let iter = InterlaceIterator { len, next: 0, pass: 0 };
            let lines = iter.collect::<Vec<_>>();
            assert_eq!(lines, expect);
        }
    }

    #[test]
    fn interlace_max() {
        let iter = InterlaceIterator {
            len: 0xFFFF,
            next: 0,
            pass: 0,
        };
        assert_eq!(65533, iter.last().unwrap());
    }
}
