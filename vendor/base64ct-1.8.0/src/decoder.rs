//! Buffered Base64 decoder.

use crate::{
    Encoding,
    Error::{self, InvalidLength},
    MIN_LINE_WIDTH, encoding,
    line_ending::{CHAR_CR, CHAR_LF},
};
use core::{cmp, marker::PhantomData};

#[cfg(feature = "alloc")]
use {alloc::vec::Vec, core::iter};

#[cfg(feature = "std")]
use std::io;

#[cfg(doc)]
use crate::{Base64, Base64Unpadded};

/// Stateful Base64 decoder with support for buffered, incremental decoding.
///
/// The `E` type parameter can be any type which impls [`Encoding`] such as
/// [`Base64`] or [`Base64Unpadded`].
#[derive(Clone)]
pub struct Decoder<'i, E: Encoding> {
    /// Current line being processed.
    line: Line<'i>,

    /// Base64 input data reader.
    line_reader: LineReader<'i>,

    /// Length of the remaining data after Base64 decoding.
    remaining_len: usize,

    /// Block buffer used for non-block-aligned data.
    block_buffer: BlockBuffer,

    /// Phantom parameter for the Base64 encoding in use.
    encoding: PhantomData<E>,
}

impl<'i, E: Encoding> Decoder<'i, E> {
    /// Create a new decoder for a byte slice containing contiguous
    /// (non-newline-delimited) Base64-encoded data.
    ///
    /// # Returns
    /// - `Ok(decoder)` on success.
    /// - `Err(Error::InvalidLength)` if the input buffer is empty.
    pub fn new(input: &'i [u8]) -> Result<Self, Error> {
        let line_reader = LineReader::new_unwrapped(input)?;
        let remaining_len = line_reader.decoded_len::<E>()?;

        Ok(Self {
            line: Line::default(),
            line_reader,
            remaining_len,
            block_buffer: BlockBuffer::default(),
            encoding: PhantomData,
        })
    }

    /// Create a new decoder for a byte slice containing Base64 which
    /// line wraps at the given line length.
    ///
    /// Trailing newlines are not supported and must be removed in advance.
    ///
    /// Newlines are handled according to what are roughly [RFC7468] conventions:
    ///
    /// ```text
    /// [parsers] MUST handle different newline conventions
    /// ```
    ///
    /// RFC7468 allows any of the following as newlines, and allows a mixture
    /// of different types of newlines:
    ///
    /// ```text
    /// eol        = CRLF / CR / LF
    /// ```
    ///
    /// # Returns
    /// - `Ok(decoder)` on success.
    /// - `Err(Error::InvalidLength)` if the input buffer is empty or the line
    ///   width is zero.
    ///
    /// [RFC7468]: https://datatracker.ietf.org/doc/html/rfc7468
    pub fn new_wrapped(input: &'i [u8], line_width: usize) -> Result<Self, Error> {
        let line_reader = LineReader::new_wrapped(input, line_width)?;
        let remaining_len = line_reader.decoded_len::<E>()?;

        Ok(Self {
            line: Line::default(),
            line_reader,
            remaining_len,
            block_buffer: BlockBuffer::default(),
            encoding: PhantomData,
        })
    }

    /// Fill the provided buffer with data decoded from Base64.
    ///
    /// Enough Base64 input data must remain to fill the entire buffer.
    ///
    /// # Returns
    /// - `Ok(bytes)` if the expected amount of data was read
    /// - `Err(Error::InvalidLength)` if the exact amount of data couldn't be read
    pub fn decode<'o>(&mut self, out: &'o mut [u8]) -> Result<&'o [u8], Error> {
        if self.is_finished() && !out.is_empty() {
            return Err(InvalidLength);
        }

        let mut out_pos = 0;

        while out_pos < out.len() {
            // If there's data in the block buffer, use it
            if !self.block_buffer.is_empty() {
                let out_rem = out.len().checked_sub(out_pos).ok_or(InvalidLength)?;
                let bytes = self.block_buffer.take(out_rem)?;
                out[out_pos..][..bytes.len()].copy_from_slice(bytes);
                out_pos = out_pos.checked_add(bytes.len()).ok_or(InvalidLength)?;
            }

            // Advance the line reader if necessary
            if self.line.is_empty() && !self.line_reader.is_empty() {
                self.advance_line()?;
            }

            // Attempt to decode a stride of block-aligned data
            let in_blocks = self.line.len() / 4;
            let out_rem = out.len().checked_sub(out_pos).ok_or(InvalidLength)?;
            let out_blocks = out_rem / 3;
            let blocks = cmp::min(in_blocks, out_blocks);
            let in_aligned = self.line.take(blocks.checked_mul(4).ok_or(InvalidLength)?);

            if !in_aligned.is_empty() {
                let out_buf = &mut out[out_pos..][..blocks.checked_mul(3).ok_or(InvalidLength)?];
                let decoded_len = self.perform_decode(in_aligned, out_buf)?.len();
                out_pos = out_pos.checked_add(decoded_len).ok_or(InvalidLength)?;
            }

            if out_pos < out.len() {
                if self.is_finished() {
                    // If we're out of input then we've been requested to decode
                    // more data than is actually available.
                    return Err(InvalidLength);
                } else {
                    // If we still have data available but haven't completely
                    // filled the output slice, we're in a situation where
                    // either the input or output isn't block-aligned, so fill
                    // the internal block buffer.
                    self.fill_block_buffer()?;
                }
            }
        }

        self.remaining_len = self
            .remaining_len
            .checked_sub(out.len())
            .ok_or(InvalidLength)?;

        Ok(out)
    }

    /// Decode all remaining Base64 data, placing the result into `buf`.
    ///
    /// If successful, this function will return the total number of bytes
    /// decoded into `buf`.
    #[cfg(feature = "alloc")]
    pub fn decode_to_end<'o>(&mut self, buf: &'o mut Vec<u8>) -> Result<&'o [u8], Error> {
        let start_len = buf.len();
        let remaining_len = self.remaining_len();
        let total_len = start_len.checked_add(remaining_len).ok_or(InvalidLength)?;

        if total_len > buf.capacity() {
            buf.reserve(total_len.checked_sub(buf.capacity()).ok_or(InvalidLength)?);
        }

        // Append `decoded_len` zeroes to the vector
        buf.extend(iter::repeat_n(0, remaining_len));
        self.decode(&mut buf[start_len..])?;
        Ok(&buf[start_len..])
    }

    /// Get the length of the remaining data after Base64 decoding.
    ///
    /// Decreases every time data is decoded.
    pub fn remaining_len(&self) -> usize {
        self.remaining_len
    }

    /// Has all of the input data been decoded?
    pub fn is_finished(&self) -> bool {
        self.line.is_empty() && self.line_reader.is_empty() && self.block_buffer.is_empty()
    }

    /// Fill the block buffer with data.
    fn fill_block_buffer(&mut self) -> Result<(), Error> {
        let mut buf = [0u8; BlockBuffer::SIZE];

        let decoded = if self.line.len() < 4 && !self.line_reader.is_empty() {
            // Handle input block which is split across lines
            let mut tmp = [0u8; 4];

            // Copy remaining data in the line into tmp
            let line_end = self.line.take(4);
            tmp[..line_end.len()].copy_from_slice(line_end);

            // Advance the line and attempt to fill tmp
            self.advance_line()?;
            let len = 4usize.checked_sub(line_end.len()).ok_or(InvalidLength)?;
            let line_begin = self.line.take(len);
            tmp[line_end.len()..][..line_begin.len()].copy_from_slice(line_begin);

            let tmp_len = line_begin
                .len()
                .checked_add(line_end.len())
                .ok_or(InvalidLength)?;

            self.perform_decode(&tmp[..tmp_len], &mut buf)
        } else {
            let block = self.line.take(4);
            self.perform_decode(block, &mut buf)
        }?;

        self.block_buffer.fill(decoded)
    }

    /// Advance the internal buffer to the next line.
    fn advance_line(&mut self) -> Result<(), Error> {
        debug_assert!(self.line.is_empty(), "expected line buffer to be empty");

        if let Some(line) = self.line_reader.next().transpose()? {
            self.line = line;
            Ok(())
        } else {
            Err(InvalidLength)
        }
    }

    /// Perform Base64 decoding operation.
    fn perform_decode<'o>(&self, src: &[u8], dst: &'o mut [u8]) -> Result<&'o [u8], Error> {
        if self.is_finished() {
            E::decode(src, dst)
        } else {
            E::Unpadded::decode(src, dst)
        }
    }
}

#[cfg(feature = "std")]
impl<E: Encoding> io::Read for Decoder<'_, E> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.is_finished() {
            return Ok(0);
        }
        let slice = match buf.get_mut(..self.remaining_len()) {
            Some(bytes) => bytes,
            None => buf,
        };

        self.decode(slice)?;
        Ok(slice.len())
    }

    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        if self.is_finished() {
            return Ok(0);
        }
        Ok(self.decode_to_end(buf)?.len())
    }

    fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
        self.decode(buf)?;
        Ok(())
    }
}

/// Base64 decode buffer for a 1-block input.
///
/// This handles a partially decoded block of data, i.e. data which has been
/// decoded but not read.
#[derive(Clone, Default, Debug)]
struct BlockBuffer {
    /// 3 decoded bytes from a 4-byte Base64-encoded input.
    decoded: [u8; Self::SIZE],

    /// Length of the buffer.
    length: usize,

    /// Position within the buffer.
    position: usize,
}

impl BlockBuffer {
    /// Size of the buffer in bytes.
    const SIZE: usize = 3;

    /// Fill the buffer by decoding up to 3 bytes of decoded Base64 input.
    fn fill(&mut self, decoded_input: &[u8]) -> Result<(), Error> {
        debug_assert!(self.is_empty());

        if decoded_input.len() > Self::SIZE {
            return Err(InvalidLength);
        }

        self.position = 0;
        self.length = decoded_input.len();
        self.decoded[..decoded_input.len()].copy_from_slice(decoded_input);
        Ok(())
    }

    /// Take a specified number of bytes from the buffer.
    ///
    /// Returns as many bytes as possible, or an empty slice if the buffer has
    /// already been read to completion.
    fn take(&mut self, mut nbytes: usize) -> Result<&[u8], Error> {
        debug_assert!(self.position <= self.length);
        let start_pos = self.position;
        let remaining_len = self.length.checked_sub(start_pos).ok_or(InvalidLength)?;

        if nbytes > remaining_len {
            nbytes = remaining_len;
        }

        self.position = self.position.checked_add(nbytes).ok_or(InvalidLength)?;
        Ok(&self.decoded[start_pos..][..nbytes])
    }

    /// Have all of the bytes in this buffer been consumed?
    fn is_empty(&self) -> bool {
        self.position == self.length
    }
}

/// A single line of linewrapped data, providing a read buffer.
#[derive(Clone, Debug)]
pub struct Line<'i> {
    /// Remaining data in the line
    remaining: &'i [u8],
}

impl Default for Line<'_> {
    fn default() -> Self {
        Self::new(&[])
    }
}

impl<'i> Line<'i> {
    /// Create a new line which wraps the given input data.
    fn new(bytes: &'i [u8]) -> Self {
        Self { remaining: bytes }
    }

    /// Take up to `nbytes` from this line buffer.
    fn take(&mut self, nbytes: usize) -> &'i [u8] {
        let (bytes, rest) = if nbytes < self.remaining.len() {
            self.remaining.split_at(nbytes)
        } else {
            (self.remaining, [].as_ref())
        };

        self.remaining = rest;
        bytes
    }

    /// Slice off a tail of a given length.
    fn slice_tail(&self, nbytes: usize) -> Result<&'i [u8], Error> {
        let offset = self.len().checked_sub(nbytes).ok_or(InvalidLength)?;
        self.remaining.get(offset..).ok_or(InvalidLength)
    }

    /// Get the number of bytes remaining in this line.
    fn len(&self) -> usize {
        self.remaining.len()
    }

    /// Is the buffer for this line empty?
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Trim the newline off the end of this line.
    fn trim_end(&self) -> Self {
        Line::new(match self.remaining {
            [line @ .., CHAR_CR, CHAR_LF] => line,
            [line @ .., CHAR_CR] => line,
            [line @ .., CHAR_LF] => line,
            line => line,
        })
    }
}

/// Iterator over multi-line Base64 input.
#[derive(Clone)]
struct LineReader<'i> {
    /// Remaining linewrapped data to be processed.
    remaining: &'i [u8],

    /// Line width.
    line_width: Option<usize>,
}

impl<'i> LineReader<'i> {
    /// Create a new reader which operates over continugous unwrapped data.
    fn new_unwrapped(bytes: &'i [u8]) -> Result<Self, Error> {
        if bytes.is_empty() {
            Err(InvalidLength)
        } else {
            Ok(Self {
                remaining: bytes,
                line_width: None,
            })
        }
    }

    /// Create a new reader which operates over linewrapped data.
    fn new_wrapped(bytes: &'i [u8], line_width: usize) -> Result<Self, Error> {
        if line_width < MIN_LINE_WIDTH {
            return Err(InvalidLength);
        }

        let mut reader = Self::new_unwrapped(bytes)?;
        reader.line_width = Some(line_width);
        Ok(reader)
    }

    /// Is this line reader empty?
    fn is_empty(&self) -> bool {
        self.remaining.is_empty()
    }

    /// Get the total length of the data decoded from this line reader.
    fn decoded_len<E: Encoding>(&self) -> Result<usize, Error> {
        let mut buffer = [0u8; 4];
        let mut lines = self.clone();
        let mut line = match lines.next().transpose()? {
            Some(l) => l,
            None => return Ok(0),
        };
        let mut base64_len = 0usize;

        loop {
            base64_len = base64_len.checked_add(line.len()).ok_or(InvalidLength)?;

            match lines.next().transpose()? {
                Some(l) => {
                    // Store the end of the line in the buffer so we can
                    // reassemble the last block to determine the real length
                    buffer.copy_from_slice(line.slice_tail(4)?);

                    line = l
                }

                // To compute an exact decoded length we need to decode the
                // last Base64 block and get the decoded length.
                //
                // This is what the somewhat complex code below is doing.
                None => {
                    // Compute number of bytes in the last block (may be unpadded)
                    let base64_last_block_len = match base64_len % 4 {
                        0 => 4,
                        n => n,
                    };

                    // Compute decoded length without the last block
                    let decoded_len = encoding::decoded_len(
                        base64_len
                            .checked_sub(base64_last_block_len)
                            .ok_or(InvalidLength)?,
                    );

                    // Compute the decoded length of the last block
                    let mut out = [0u8; 3];
                    let last_block_len = if line.len() < base64_last_block_len {
                        let buffered_part_len = base64_last_block_len
                            .checked_sub(line.len())
                            .ok_or(InvalidLength)?;

                        let offset = 4usize.checked_sub(buffered_part_len).ok_or(InvalidLength)?;

                        for i in 0..buffered_part_len {
                            buffer[i] = buffer[offset.checked_add(i).ok_or(InvalidLength)?];
                        }

                        buffer[buffered_part_len..][..line.len()].copy_from_slice(line.remaining);
                        let buffer_len = buffered_part_len
                            .checked_add(line.len())
                            .ok_or(InvalidLength)?;

                        E::decode(&buffer[..buffer_len], &mut out)?.len()
                    } else {
                        let last_block = line.slice_tail(base64_last_block_len)?;
                        E::decode(last_block, &mut out)?.len()
                    };

                    return decoded_len.checked_add(last_block_len).ok_or(InvalidLength);
                }
            }
        }
    }
}

impl<'i> Iterator for LineReader<'i> {
    type Item = Result<Line<'i>, Error>;

    fn next(&mut self) -> Option<Result<Line<'i>, Error>> {
        if let Some(line_width) = self.line_width {
            let rest = match self.remaining.get(line_width..) {
                None | Some([]) => {
                    if self.remaining.is_empty() {
                        return None;
                    } else {
                        let line = Line::new(self.remaining).trim_end();
                        self.remaining = &[];
                        return Some(Ok(line));
                    }
                }
                Some([CHAR_CR, CHAR_LF, rest @ ..]) => rest,
                Some([CHAR_CR, rest @ ..]) => rest,
                Some([CHAR_LF, rest @ ..]) => rest,
                _ => {
                    // Expected a leading newline
                    return Some(Err(Error::InvalidEncoding));
                }
            };

            let line = Line::new(&self.remaining[..line_width]);
            self.remaining = rest;
            Some(Ok(line))
        } else if !self.remaining.is_empty() {
            let line = Line::new(self.remaining).trim_end();
            self.remaining = b"";

            if line.is_empty() {
                None
            } else {
                Some(Ok(line))
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use crate::{Base64, Base64Unpadded, Decoder, alphabet::Alphabet, test_vectors::*};

    #[cfg(feature = "std")]
    use {alloc::vec::Vec, std::io::Read};

    #[test]
    fn decode_padded() {
        decode_test(PADDED_BIN, || {
            Decoder::<Base64>::new(PADDED_BASE64.as_bytes()).unwrap()
        })
    }

    #[test]
    fn decode_unpadded() {
        decode_test(UNPADDED_BIN, || {
            Decoder::<Base64Unpadded>::new(UNPADDED_BASE64.as_bytes()).unwrap()
        })
    }

    #[test]
    fn decode_multiline_padded() {
        decode_test(MULTILINE_PADDED_BIN, || {
            Decoder::<Base64>::new_wrapped(MULTILINE_PADDED_BASE64.as_bytes(), 70).unwrap()
        })
    }

    #[test]
    fn decode_multiline_unpadded() {
        decode_test(MULTILINE_UNPADDED_BIN, || {
            Decoder::<Base64Unpadded>::new_wrapped(MULTILINE_UNPADDED_BASE64.as_bytes(), 70)
                .unwrap()
        })
    }

    #[cfg(feature = "std")]
    #[test]
    fn read_multiline_padded() {
        let mut decoder =
            Decoder::<Base64>::new_wrapped(MULTILINE_PADDED_BASE64.as_bytes(), 70).unwrap();

        let mut buf = Vec::new();
        let len = decoder.read_to_end(&mut buf).unwrap();

        assert_eq!(len, MULTILINE_PADDED_BIN.len());
        assert_eq!(buf.as_slice(), MULTILINE_PADDED_BIN);
    }

    #[cfg(feature = "std")]
    #[test]
    fn decode_empty_at_end() {
        let mut decoder = Decoder::<Base64>::new(b"AAAA").unwrap();

        // Strip initial bytes
        let mut buf = vec![0u8; 3];
        assert_eq!(decoder.decode(&mut buf), Ok(&vec![0u8; 3][..]));

        // Now try to read nothing from the end
        let mut buf: Vec<u8> = vec![];

        assert_eq!(decoder.decode(&mut buf), Ok(&[][..]));
    }

    /// Core functionality of a decoding test
    #[allow(clippy::arithmetic_side_effects)]
    fn decode_test<'a, F, V>(expected: &[u8], f: F)
    where
        F: Fn() -> Decoder<'a, V>,
        V: Alphabet,
    {
        for chunk_size in 1..expected.len() {
            let mut decoder = f();
            let mut remaining_len = decoder.remaining_len();
            let mut buffer = [0u8; 1024];

            for chunk in expected.chunks(chunk_size) {
                assert!(!decoder.is_finished());
                let decoded = decoder.decode(&mut buffer[..chunk.len()]).unwrap();
                assert_eq!(chunk, decoded);

                let dlen = decoded.len();
                remaining_len -= dlen;
                assert_eq!(remaining_len, decoder.remaining_len());
            }

            assert!(decoder.is_finished());
            assert_eq!(decoder.remaining_len(), 0);
        }
    }
}
