use std::io::{self, BufRead};
use std::error::Error;
use std::fmt;
use std::str;
use super::*;

/// Wraps a `std::io::BufRead` buffered byte stream and decode it as UTF-8.
pub struct BufReadDecoder<B: BufRead> {
    buf_read: B,
    bytes_consumed: usize,
    incomplete: Incomplete,
}

#[derive(Debug)]
pub enum BufReadDecoderError<'a> {
    /// Represents one UTF-8 error in the byte stream.
    ///
    /// In lossy decoding, each such error should be replaced with U+FFFD.
    /// (See `BufReadDecoder::next_lossy` and `BufReadDecoderError::lossy`.)
    InvalidByteSequence(&'a [u8]),

    /// An I/O error from the underlying byte stream
    Io(io::Error),
}

impl<'a> BufReadDecoderError<'a> {
    /// Replace UTF-8 errors with U+FFFD
    pub fn lossy(self) -> Result<&'static str, io::Error> {
        match self {
            BufReadDecoderError::Io(error) => Err(error),
            BufReadDecoderError::InvalidByteSequence(_) => Ok(REPLACEMENT_CHARACTER),
        }
    }
}

impl<'a> fmt::Display for BufReadDecoderError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BufReadDecoderError::InvalidByteSequence(bytes) => {
                write!(f, "invalid byte sequence: {:02x?}", bytes)
            }
            BufReadDecoderError::Io(ref err) => write!(f, "underlying bytestream error: {}", err),
        }
    }
}

impl<'a> Error for BufReadDecoderError<'a> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            BufReadDecoderError::InvalidByteSequence(_) => None,
            BufReadDecoderError::Io(ref err) => Some(err),
        }
    }
}

impl<B: BufRead> BufReadDecoder<B> {
    /// This is to `Read::read_to_string` what `String::from_utf8_lossy` is to `String::from_utf8`.
    pub fn read_to_string_lossy(buf_read: B) -> io::Result<String> {
        let mut decoder = Self::new(buf_read);
        let mut string = String::new();
        while let Some(result) = decoder.next_lossy() {
            string.push_str(result?)
        }
        Ok(string)
    }

    pub fn new(buf_read: B) -> Self {
        Self {
            buf_read,
            bytes_consumed: 0,
            incomplete: Incomplete::empty(),
        }
    }

    /// Same as `BufReadDecoder::next_strict`, but replace UTF-8 errors with U+FFFD.
    pub fn next_lossy(&mut self) -> Option<io::Result<&str>> {
        self.next_strict().map(|result| result.or_else(|e| e.lossy()))
    }

    /// Decode and consume the next chunk of UTF-8 input.
    ///
    /// This method is intended to be called repeatedly until it returns `None`,
    /// which represents EOF from the underlying byte stream.
    /// This is similar to `Iterator::next`,
    /// except that decoded chunks borrow the decoder (~iterator)
    /// so they need to be handled or copied before the next chunk can start decoding.
    pub fn next_strict(&mut self) -> Option<Result<&str, BufReadDecoderError>> {
        enum BytesSource {
            BufRead(usize),
            Incomplete,
        }
        macro_rules! try_io {
            ($io_result: expr) => {
                match $io_result {
                    Ok(value) => value,
                    Err(error) => return Some(Err(BufReadDecoderError::Io(error)))
                }
            }
        }
        let (source, result) = loop {
            if self.bytes_consumed > 0 {
                self.buf_read.consume(self.bytes_consumed);
                self.bytes_consumed = 0;
            }
            let buf = try_io!(self.buf_read.fill_buf());

            // Force loop iteration to go through an explicit `continue`
            enum Unreachable {}
            let _: Unreachable = if self.incomplete.is_empty() {
                if buf.is_empty() {
                    return None  // EOF
                }
                match str::from_utf8(buf) {
                    Ok(_) => {
                        break (BytesSource::BufRead(buf.len()), Ok(()))
                    }
                    Err(error) => {
                        let valid_up_to = error.valid_up_to();
                        if valid_up_to > 0 {
                            break (BytesSource::BufRead(valid_up_to), Ok(()))
                        }
                        match error.error_len() {
                            Some(invalid_sequence_length) => {
                                break (BytesSource::BufRead(invalid_sequence_length), Err(()))
                            }
                            None => {
                                self.bytes_consumed = buf.len();
                                self.incomplete = Incomplete::new(buf);
                                // need more input bytes
                                continue
                            }
                        }
                    }
                }
            } else {
                if buf.is_empty() {
                    break (BytesSource::Incomplete, Err(()))  // EOF with incomplete code point
                }
                let (consumed, opt_result) = self.incomplete.try_complete_offsets(buf);
                self.bytes_consumed = consumed;
                match opt_result {
                    None => {
                        // need more input bytes
                        continue
                    }
                    Some(result) => {
                        break (BytesSource::Incomplete, result)
                    }
                }
            };
        };
        let bytes = match source {
            BytesSource::BufRead(byte_count) => {
                self.bytes_consumed = byte_count;
                let buf = try_io!(self.buf_read.fill_buf());
                &buf[..byte_count]
            }
            BytesSource::Incomplete => {
                self.incomplete.take_buffer()
            }
        };
        match result {
            Ok(()) => Some(Ok(unsafe { str::from_utf8_unchecked(bytes) })),
            Err(()) => Some(Err(BufReadDecoderError::InvalidByteSequence(bytes))),
        }
    }
}
