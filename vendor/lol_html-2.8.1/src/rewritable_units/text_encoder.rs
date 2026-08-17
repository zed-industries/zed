use encoding_rs::{CoderResult, Encoder, Encoding, UTF_8};
use thiserror::Error;

/// Input contained non-UTF-8 byte sequence
///
/// [`StreamingHandlerSink::write_utf8_chunk`][crate::html_content::StreamingHandlerSink::write_utf8_chunk]
/// will not fail on an incomplete UTF-8 sequence at the end of the chunk,
/// but it will report errors if incomplete UTF-8 sequences are within the chunk, or the next call starts with
/// bytes that don't match the previous call's trailing bytes.
#[derive(Error, Debug, Eq, PartialEq, Copy, Clone)]
#[error("Invalid UTF-8")]
pub struct Utf8Error;

/// Temporary buffer used for encoding_rs output
enum Buffer {
    /// Stack buffer avoids heap allocation, and lets go back quickly to the ASCII fast path.
    Stack([u8; 63]), // leave a byte for the enum's tag, so that the enum has 64-byte size
    /// Used when encoding_rs asks for a larger buffer, or the content is large enough for small buffer roundtrips to add up
    Heap(Vec<u8>),
}

impl Buffer {
    /// Arbitrary limit when to switch from a small on-stack buffer to heap allocation
    const CONTENT_WRITE_LENGTH_LONG_ENOUGH_TO_USE_LARGER_BUFFER: usize = 1 << 20;

    /// Arbitrary, about a page size
    const DEFAULT_HEAP_BUFFER_SIZE: usize = 4096;

    fn buffer_for_length(&mut self, content_len: usize) -> &mut [u8] {
        match self {
            Self::Heap(buf) => buf.as_mut_slice(),
            // Long non-ASCII content could take lots of roundtrips through the encoder
            buf if content_len >= Self::CONTENT_WRITE_LENGTH_LONG_ENOUGH_TO_USE_LARGER_BUFFER => {
                *buf = Self::Heap(vec![0; Self::DEFAULT_HEAP_BUFFER_SIZE]);
                match buf {
                    Self::Heap(buf) => buf.as_mut(),
                    _ => unreachable!(),
                }
            }
            Self::Stack(buf) => buf.as_mut_slice(),
        }
    }
}

pub(crate) struct TextEncoder {
    encoder: Encoder,
    buffer: Buffer,
}

impl TextEncoder {
    #[inline]
    pub fn new(encoding: &'static Encoding) -> Self {
        debug_assert!(encoding != UTF_8);
        debug_assert!(encoding.is_ascii_compatible());
        Self {
            encoder: encoding.new_encoder(),
            buffer: Buffer::Stack([0; 63]),
        }
    }

    /// This is more efficient than `Bytes::from_str`, because it can output non-UTF-8/non-ASCII encodings
    /// without heap allocations.
    /// It also avoids methods that have UB: <https://github.com/hsivonen/encoding_rs/issues/79>
    #[inline(never)]
    pub fn encode(&mut self, mut content: &str, output_handler: &mut dyn FnMut(&[u8])) {
        loop {
            // First, fast path for ASCII-only prefix
            debug_assert!(!self.encoder.has_pending_state()); // ASCII-compatible encodings are not supposed to have it
            let ascii_len = Encoding::ascii_valid_up_to(content.as_bytes());
            if let Some((ascii, remainder)) = content.split_at_checked(ascii_len) {
                if !ascii.is_empty() {
                    (output_handler)(ascii.as_bytes());
                }
                if remainder.is_empty() {
                    return;
                }
                content = remainder;
            }

            // Now the content starts with non-ASCII byte, so encoding_rs may need a buffer to convert to.
            let buffer = self.buffer.buffer_for_length(content.len());

            // last == true is needed only for the stateful ISO-JP encoding, which this library doesn't allow
            let (result, read, written, _) = self.encoder.encode_from_utf8(content, buffer, false);

            if written > 0 && written <= buffer.len() {
                (output_handler)(&buffer[..written]);
            }
            content = match content.get(read..) {
                Some(rest) if !rest.is_empty() => rest,
                _ => return,
            };

            match result {
                CoderResult::InputEmpty => {
                    debug_assert!(content.is_empty());
                    return;
                }
                // we've made progress, and can try again without growing the buffer
                CoderResult::OutputFull if written > 0 => {}
                CoderResult::OutputFull => {
                    // this never happens, encoding_rs only needs a dozen bytes.
                    if buffer.len() >= Buffer::DEFAULT_HEAP_BUFFER_SIZE {
                        debug_assert!(false, "encoding_rs stalled");
                        return;
                    }
                    self.buffer = Buffer::Heap(vec![0; Buffer::DEFAULT_HEAP_BUFFER_SIZE]);
                }
            }
        }
    }
}

const fn is_continuation_byte(b: u8) -> bool {
    (b >> 6) == 0b10
}

const fn utf8_width(b: u8) -> u8 {
    b.leading_ones() as _
}

/// Stitches together UTF-8 from byte writes that may split UTF-8 sequences into multiple fragments
pub(crate) struct IncompleteUtf8Resync {
    /// Buffers an incomplete UTF-8 sequence
    char_bytes: [u8; 4],
    /// Number of bytes in `bytes`
    char_len: u8,
}

impl IncompleteUtf8Resync {
    pub const fn new() -> Self {
        Self {
            char_bytes: [0; 4],
            char_len: 0,
        }
    }

    /// Returns a valid UTF-8 fragment, and not-yet-checked remainder of the bytes.
    ///
    /// Call `discard_incomplete()` after the last write to flush any partially-written chars.
    pub fn utf8_bytes_to_slice<'buf, 'src: 'buf>(
        &'buf mut self,
        mut content: &'src [u8],
    ) -> Result<(&'buf str, &'src [u8]), Utf8Error> {
        // There may be incomplete char buffered from previous write, that must be continued now
        if self.char_len > 0 {
            let mut must_emit_now = false;
            while let Some((&next_byte, rest)) = content.split_first() {
                if is_continuation_byte(next_byte) {
                    if let Some(buf) = self.char_bytes.get_mut(self.char_len as usize) {
                        *buf = next_byte;
                        self.char_len += 1;
                        content = rest;
                        continue;
                    }
                    // overlong sequences fall here, and will be checked when the char_bytes is flushed
                }
                must_emit_now = true;
                break;
            }

            if self.char_len >= utf8_width(self.char_bytes[0]) {
                must_emit_now = true;
            }

            if must_emit_now {
                let char_buf = self
                    .char_bytes
                    .get(..self.char_len as usize)
                    .ok_or(Utf8Error)?;
                self.char_len = 0;
                let ch = std::str::from_utf8(char_buf).map_err(|_| Utf8Error)?;
                Ok((ch, content))
            } else {
                // a partial write has ended without fully completing a char (it's possible to write 1 byte at a time)
                debug_assert!(content.is_empty());
                Ok(("", b""))
            }
        } else {
            match std::str::from_utf8(content) {
                Ok(src) => Ok((src, b"")),
                // error_len means invalid bytes somewhere, not just incomplete 1-3 bytes at the end
                Err(err) if err.error_len().is_some() => Err(Utf8Error),
                Err(err) => {
                    let (valid, invalid) = content
                        .split_at_checked(err.valid_up_to())
                        .ok_or(Utf8Error)?;
                    // save the incomplete bytes from the end for the next write
                    self.char_bytes
                        .get_mut(..invalid.len())
                        .ok_or(Utf8Error)?
                        .copy_from_slice(invalid);
                    self.char_len = invalid.len() as _;
                    // valid_up_to promises it is always valid
                    let valid = std::str::from_utf8(valid).map_err(|_| Utf8Error)?;
                    Ok((valid, b""))
                }
            }
        }
    }

    /// True if there were incomplete invalid bytes in the buffer
    pub fn discard_incomplete(&mut self) -> bool {
        if self.char_len > 0 {
            self.char_len = 0;
            true
        } else {
            false
        }
    }
}

#[test]
fn chars() {
    let boundaries = "🐈°文字化けしない"
        .as_bytes()
        .iter()
        .map(|&ch| {
            if is_continuation_byte(ch) {
                '.'
            } else {
                (b'0' + utf8_width(ch)) as char
            }
        })
        .collect::<String>();
    assert_eq!("4...2.3..3..3..3..3..3..3..", boundaries);
}
