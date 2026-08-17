//! PEM encoder.

use crate::{
    grammar, Base64Encoder, Error, LineEnding, Result, BASE64_WRAP_WIDTH,
    ENCAPSULATION_BOUNDARY_DELIMITER, POST_ENCAPSULATION_BOUNDARY, PRE_ENCAPSULATION_BOUNDARY,
};
use base64ct::{Base64, Encoding};
use core::str;

#[cfg(feature = "alloc")]
use alloc::string::String;

#[cfg(feature = "std")]
use std::io;

/// Compute the length of a PEM encoded document which encapsulates a
/// Base64-encoded body including line endings every 64 characters.
///
/// The `input_len` parameter specifies the length of the raw input
/// bytes prior to Base64 encoding.
///
/// Note that the current implementation of this function computes an upper
/// bound of the length and the actual encoded document may be slightly shorter
/// (typically 1-byte). Downstream consumers of this function should check the
/// actual encoded length and potentially truncate buffers allocated using this
/// function to estimate the encapsulated size.
///
/// Use [`encoded_len`] (when possible) to obtain a precise length.
///
/// ## Returns
/// - `Ok(len)` on success
/// - `Err(Error::Length)` on length overflow
pub fn encapsulated_len(label: &str, line_ending: LineEnding, input_len: usize) -> Result<usize> {
    encapsulated_len_wrapped(label, BASE64_WRAP_WIDTH, line_ending, input_len)
}

/// Compute the length of a PEM encoded document with the Base64 body
/// line wrapped at the specified `width`.
///
/// This is the same as [`encapsulated_len`], which defaults to a width of 64.
///
/// Note that per [RFC7468 § 2] encoding PEM with any other wrap width besides
/// 64 is technically non-compliant:
///
/// > Generators MUST wrap the base64-encoded lines so that each line
/// > consists of exactly 64 characters except for the final line, which
/// > will encode the remainder of the data (within the 64-character line
/// > boundary)
///
/// [RFC7468 § 2]: https://datatracker.ietf.org/doc/html/rfc7468#section-2
pub fn encapsulated_len_wrapped(
    label: &str,
    line_width: usize,
    line_ending: LineEnding,
    input_len: usize,
) -> Result<usize> {
    if line_width < 4 {
        return Err(Error::Length);
    }

    let base64_len = input_len
        .checked_mul(4)
        .and_then(|n| n.checked_div(3))
        .and_then(|n| n.checked_add(3))
        .ok_or(Error::Length)?
        & !3;

    let base64_len_wrapped = base64_len_wrapped(base64_len, line_width, line_ending)?;
    encapsulated_len_inner(label, line_ending, base64_len_wrapped)
}

/// Get the length of a PEM encoded document with the given bytes and label.
///
/// This function computes a precise length of the PEM encoding of the given
/// `input` data.
///
/// ## Returns
/// - `Ok(len)` on success
/// - `Err(Error::Length)` on length overflow
pub fn encoded_len(label: &str, line_ending: LineEnding, input: &[u8]) -> Result<usize> {
    let base64_len = Base64::encoded_len(input);
    let base64_len_wrapped = base64_len_wrapped(base64_len, BASE64_WRAP_WIDTH, line_ending)?;
    encapsulated_len_inner(label, line_ending, base64_len_wrapped)
}

/// Encode a PEM document according to RFC 7468's "Strict" grammar.
pub fn encode<'o>(
    type_label: &str,
    line_ending: LineEnding,
    input: &[u8],
    buf: &'o mut [u8],
) -> Result<&'o str> {
    let mut encoder = Encoder::new(type_label, line_ending, buf)?;
    encoder.encode(input)?;
    let encoded_len = encoder.finish()?;
    let output = &buf[..encoded_len];

    // Sanity check
    debug_assert!(str::from_utf8(output).is_ok());

    // Ensure `output` contains characters from the lower 7-bit ASCII set
    if output.iter().fold(0u8, |acc, &byte| acc | (byte & 0x80)) == 0 {
        // Use unchecked conversion to avoid applying UTF-8 checks to potentially
        // secret PEM documents (and therefore introducing a potential timing
        // sidechannel)
        //
        // SAFETY: contents of this buffer are controlled entirely by the encoder,
        // which ensures the contents are always a valid (ASCII) subset of UTF-8.
        // It's also additionally sanity checked by two assertions above to ensure
        // the validity (with the always-on runtime check implemented in a
        // constant time-ish manner.
        #[allow(unsafe_code)]
        Ok(unsafe { str::from_utf8_unchecked(output) })
    } else {
        Err(Error::CharacterEncoding)
    }
}

/// Encode a PEM document according to RFC 7468's "Strict" grammar, returning
/// the result as a [`String`].
#[cfg(feature = "alloc")]
pub fn encode_string(label: &str, line_ending: LineEnding, input: &[u8]) -> Result<String> {
    let expected_len = encoded_len(label, line_ending, input)?;
    let mut buf = vec![0u8; expected_len];
    let actual_len = encode(label, line_ending, input, &mut buf)?.len();
    debug_assert_eq!(expected_len, actual_len);
    String::from_utf8(buf).map_err(|_| Error::CharacterEncoding)
}

/// Compute the encapsulated length of Base64 data of the given length.
fn encapsulated_len_inner(
    label: &str,
    line_ending: LineEnding,
    base64_len: usize,
) -> Result<usize> {
    [
        PRE_ENCAPSULATION_BOUNDARY.len(),
        label.as_bytes().len(),
        ENCAPSULATION_BOUNDARY_DELIMITER.len(),
        line_ending.len(),
        base64_len,
        line_ending.len(),
        POST_ENCAPSULATION_BOUNDARY.len(),
        label.as_bytes().len(),
        ENCAPSULATION_BOUNDARY_DELIMITER.len(),
        line_ending.len(),
    ]
    .into_iter()
    .try_fold(0usize, |acc, len| acc.checked_add(len))
    .ok_or(Error::Length)
}

/// Compute Base64 length line-wrapped at the specified width with the given
/// line ending.
fn base64_len_wrapped(
    base64_len: usize,
    line_width: usize,
    line_ending: LineEnding,
) -> Result<usize> {
    base64_len
        .saturating_sub(1)
        .checked_div(line_width)
        .and_then(|lines| lines.checked_mul(line_ending.len()))
        .and_then(|len| len.checked_add(base64_len))
        .ok_or(Error::Length)
}

/// Buffered PEM encoder.
///
/// Stateful buffered encoder type which encodes an input PEM document according
/// to RFC 7468's "Strict" grammar.
pub struct Encoder<'l, 'o> {
    /// PEM type label.
    type_label: &'l str,

    /// Line ending used to wrap Base64.
    line_ending: LineEnding,

    /// Buffered Base64 encoder.
    base64: Base64Encoder<'o>,
}

impl<'l, 'o> Encoder<'l, 'o> {
    /// Create a new PEM [`Encoder`] with the default options which
    /// writes output into the provided buffer.
    ///
    /// Uses the default 64-character line wrapping.
    pub fn new(type_label: &'l str, line_ending: LineEnding, out: &'o mut [u8]) -> Result<Self> {
        Self::new_wrapped(type_label, BASE64_WRAP_WIDTH, line_ending, out)
    }

    /// Create a new PEM [`Encoder`] which wraps at the given line width.
    ///
    /// Note that per [RFC7468 § 2] encoding PEM with any other wrap width besides
    /// 64 is technically non-compliant:
    ///
    /// > Generators MUST wrap the base64-encoded lines so that each line
    /// > consists of exactly 64 characters except for the final line, which
    /// > will encode the remainder of the data (within the 64-character line
    /// > boundary)
    ///
    /// This method is provided with the intended purpose of implementing the
    /// OpenSSH private key format, which uses a non-standard wrap width of 70.
    ///
    /// [RFC7468 § 2]: https://datatracker.ietf.org/doc/html/rfc7468#section-2
    pub fn new_wrapped(
        type_label: &'l str,
        line_width: usize,
        line_ending: LineEnding,
        mut out: &'o mut [u8],
    ) -> Result<Self> {
        grammar::validate_label(type_label.as_bytes())?;

        for boundary_part in [
            PRE_ENCAPSULATION_BOUNDARY,
            type_label.as_bytes(),
            ENCAPSULATION_BOUNDARY_DELIMITER,
            line_ending.as_bytes(),
        ] {
            if out.len() < boundary_part.len() {
                return Err(Error::Length);
            }

            let (part, rest) = out.split_at_mut(boundary_part.len());
            out = rest;

            part.copy_from_slice(boundary_part);
        }

        let base64 = Base64Encoder::new_wrapped(out, line_width, line_ending)?;

        Ok(Self {
            type_label,
            line_ending,
            base64,
        })
    }

    /// Get the PEM type label used for this document.
    pub fn type_label(&self) -> &'l str {
        self.type_label
    }

    /// Encode the provided input data.
    ///
    /// This method can be called as many times as needed with any sized input
    /// to write data encoded data into the output buffer, so long as there is
    /// sufficient space in the buffer to handle the resulting Base64 encoded
    /// data.
    pub fn encode(&mut self, input: &[u8]) -> Result<()> {
        self.base64.encode(input)?;
        Ok(())
    }

    /// Borrow the inner [`Base64Encoder`].
    pub fn base64_encoder(&mut self) -> &mut Base64Encoder<'o> {
        &mut self.base64
    }

    /// Finish encoding PEM, writing the post-encapsulation boundary.
    ///
    /// On success, returns the total number of bytes written to the output
    /// buffer.
    pub fn finish(self) -> Result<usize> {
        let (base64, mut out) = self.base64.finish_with_remaining()?;

        for boundary_part in [
            self.line_ending.as_bytes(),
            POST_ENCAPSULATION_BOUNDARY,
            self.type_label.as_bytes(),
            ENCAPSULATION_BOUNDARY_DELIMITER,
            self.line_ending.as_bytes(),
        ] {
            if out.len() < boundary_part.len() {
                return Err(Error::Length);
            }

            let (part, rest) = out.split_at_mut(boundary_part.len());
            out = rest;

            part.copy_from_slice(boundary_part);
        }

        encapsulated_len_inner(self.type_label, self.line_ending, base64.len())
    }
}

#[cfg(feature = "std")]
impl<'l, 'o> io::Write for Encoder<'l, 'o> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.encode(buf)?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        // TODO(tarcieri): return an error if there's still data remaining in the buffer?
        Ok(())
    }
}
