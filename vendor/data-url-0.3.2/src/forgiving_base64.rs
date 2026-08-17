//! <https://infra.spec.whatwg.org/#forgiving-base64-decode>

use alloc::vec::Vec;
use core::fmt;

#[derive(Debug)]
pub struct InvalidBase64(InvalidBase64Details);

impl fmt::Display for InvalidBase64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            InvalidBase64Details::UnexpectedSymbol(code_point) => {
                write!(f, "symbol with codepoint {} not expected", code_point)
            }
            InvalidBase64Details::AlphabetSymbolAfterPadding => {
                write!(f, "alphabet symbol present after padding")
            }
            InvalidBase64Details::LoneAlphabetSymbol => write!(f, "lone alphabet symbol present"),
            InvalidBase64Details::Padding => write!(f, "incorrect padding"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for InvalidBase64 {}

#[derive(Debug)]
enum InvalidBase64Details {
    UnexpectedSymbol(u8),
    AlphabetSymbolAfterPadding,
    LoneAlphabetSymbol,
    Padding,
}

#[derive(Debug)]
pub enum DecodeError<E> {
    InvalidBase64(InvalidBase64),
    WriteError(E),
}

impl<E: fmt::Display> fmt::Display for DecodeError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidBase64(inner) => write!(f, "base64 not valid: {}", inner),
            Self::WriteError(err) => write!(f, "write error: {}", err),
        }
    }
}

#[cfg(feature = "std")]
impl<E: std::error::Error> std::error::Error for DecodeError<E> {}

impl<E> From<InvalidBase64Details> for DecodeError<E> {
    fn from(e: InvalidBase64Details) -> Self {
        Self::InvalidBase64(InvalidBase64(e))
    }
}

pub(crate) enum Impossible {}

impl From<DecodeError<Impossible>> for InvalidBase64 {
    fn from(e: DecodeError<Impossible>) -> Self {
        match e {
            DecodeError::InvalidBase64(e) => e,
            DecodeError::WriteError(e) => match e {},
        }
    }
}

/// `input` is assumed to be in an ASCII-compatible encoding
pub fn decode_to_vec(input: &[u8]) -> Result<Vec<u8>, InvalidBase64> {
    let mut v = Vec::new();
    {
        let mut decoder = Decoder::new(|bytes| {
            v.extend_from_slice(bytes);
            Ok(())
        });
        decoder.feed(input)?;
        decoder.finish()?;
    }
    Ok(v)
}

/// <https://infra.spec.whatwg.org/#forgiving-base64-decode>
pub struct Decoder<F, E>
where
    F: FnMut(&[u8]) -> Result<(), E>,
{
    write_bytes: F,
    bit_buffer: u32,
    buffer_bit_length: u8,
    padding_symbols: u8,
}

impl<F, E> Decoder<F, E>
where
    F: FnMut(&[u8]) -> Result<(), E>,
{
    pub fn new(write_bytes: F) -> Self {
        Self {
            write_bytes,
            bit_buffer: 0,
            buffer_bit_length: 0,
            padding_symbols: 0,
        }
    }

    /// Feed to the decoder partial input in an ASCII-compatible encoding
    pub fn feed(&mut self, input: &[u8]) -> Result<(), DecodeError<E>> {
        for &byte in input.iter() {
            let value = BASE64_DECODE_TABLE[byte as usize];
            if value < 0 {
                // A character that’s not part of the alphabet

                // Remove ASCII whitespace
                if matches!(byte, b' ' | b'\t' | b'\n' | b'\r' | b'\x0C') {
                    continue;
                }

                if byte == b'=' {
                    self.padding_symbols = self.padding_symbols.saturating_add(1);
                    continue;
                }

                return Err(InvalidBase64Details::UnexpectedSymbol(byte).into());
            }
            if self.padding_symbols > 0 {
                return Err(InvalidBase64Details::AlphabetSymbolAfterPadding.into());
            }
            self.bit_buffer <<= 6;
            self.bit_buffer |= value as u32;
            // 18 before incrementing means we’ve just reached 24
            if self.buffer_bit_length < 18 {
                self.buffer_bit_length += 6;
            } else {
                // We’ve accumulated four times 6 bits, which equals three times 8 bits.
                let byte_buffer = [
                    (self.bit_buffer >> 16) as u8,
                    (self.bit_buffer >> 8) as u8,
                    self.bit_buffer as u8,
                ];
                (self.write_bytes)(&byte_buffer).map_err(DecodeError::WriteError)?;
                self.buffer_bit_length = 0;
                // No need to reset bit_buffer,
                // since next time we’re only gonna read relevant bits.
            }
        }
        Ok(())
    }

    /// Call this to signal the end of the input
    pub fn finish(mut self) -> Result<(), DecodeError<E>> {
        match (self.buffer_bit_length, self.padding_symbols) {
            (0, 0) => {
                // A multiple of four of alphabet symbols, and nothing else.
            }
            (12, 2) | (12, 0) => {
                // A multiple of four of alphabet symbols, followed by two more symbols,
                // optionally followed by two padding characters (which make a total multiple of four).
                let byte_buffer = [(self.bit_buffer >> 4) as u8];
                (self.write_bytes)(&byte_buffer).map_err(DecodeError::WriteError)?;
            }
            (18, 1) | (18, 0) => {
                // A multiple of four of alphabet symbols, followed by three more symbols,
                // optionally followed by one padding character (which make a total multiple of four).
                let byte_buffer = [(self.bit_buffer >> 10) as u8, (self.bit_buffer >> 2) as u8];
                (self.write_bytes)(&byte_buffer).map_err(DecodeError::WriteError)?;
            }
            (6, _) => return Err(InvalidBase64Details::LoneAlphabetSymbol.into()),
            _ => return Err(InvalidBase64Details::Padding.into()),
        }
        Ok(())
    }
}

/// Generated by `make_base64_decode_table.py` based on "Table 1: The Base 64 Alphabet"
/// at <https://tools.ietf.org/html/rfc4648#section-4>
///
/// Array indices are the byte value of symbols.
/// Array values are their positions in the base64 alphabet,
/// or -1 for symbols not in the alphabet.
/// The position contributes 6 bits to the decoded bytes.
#[rustfmt::skip]
const BASE64_DECODE_TABLE: [i8; 256] = [
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 62, -1, -1, -1, 63,
    52, 53, 54, 55, 56, 57, 58, 59, 60, 61, -1, -1, -1, -1, -1, -1,
    -1,  0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14,
    15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, -1, -1, -1, -1, -1,
    -1, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40,
    41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
];
