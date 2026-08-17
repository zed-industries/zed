//! Decoder for PEM encapsulated data.
//!
//! From RFC 7468 Section 2:
//!
//! > Textual encoding begins with a line comprising "-----BEGIN ", a
//! > label, and "-----", and ends with a line comprising "-----END ", a
//! > label, and "-----".  Between these lines, or "encapsulation
//! > boundaries", are base64-encoded data according to Section 4 of
//! > [RFC 4648].
//!
//! [RFC 4648]: https://datatracker.ietf.org/doc/html/rfc4648

use crate::{
    grammar, Base64Decoder, Error, Result, BASE64_WRAP_WIDTH, POST_ENCAPSULATION_BOUNDARY,
    PRE_ENCAPSULATION_BOUNDARY,
};
use core::str;

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

#[cfg(feature = "std")]
use std::io;

/// Decode a PEM document according to RFC 7468's "Strict" grammar.
///
/// On success, writes the decoded document into the provided buffer, returning
/// the decoded label and the portion of the provided buffer containing the
/// decoded message.
pub fn decode<'i, 'o>(pem: &'i [u8], buf: &'o mut [u8]) -> Result<(&'i str, &'o [u8])> {
    let mut decoder = Decoder::new(pem).map_err(|e| check_for_headers(pem, e))?;
    let type_label = decoder.type_label();
    let buf = buf
        .get_mut(..decoder.remaining_len())
        .ok_or(Error::Length)?;
    let decoded = decoder.decode(buf).map_err(|e| check_for_headers(pem, e))?;

    if decoder.base64.is_finished() {
        Ok((type_label, decoded))
    } else {
        Err(Error::Length)
    }
}

/// Decode a PEM document according to RFC 7468's "Strict" grammar, returning
/// the result as a [`Vec`] upon success.
#[cfg(feature = "alloc")]
pub fn decode_vec(pem: &[u8]) -> Result<(&str, Vec<u8>)> {
    let mut decoder = Decoder::new(pem).map_err(|e| check_for_headers(pem, e))?;
    let type_label = decoder.type_label();
    let mut buf = Vec::new();
    decoder
        .decode_to_end(&mut buf)
        .map_err(|e| check_for_headers(pem, e))?;
    Ok((type_label, buf))
}

/// Decode the encapsulation boundaries of a PEM document according to RFC 7468's "Strict" grammar.
///
/// On success, returning the decoded label.
pub fn decode_label(pem: &[u8]) -> Result<&str> {
    Ok(Encapsulation::try_from(pem)?.label())
}

/// Buffered PEM decoder.
///
/// Stateful buffered decoder type which decodes an input PEM document according
/// to RFC 7468's "Strict" grammar.
#[derive(Clone)]
pub struct Decoder<'i> {
    /// PEM type label.
    type_label: &'i str,

    /// Buffered Base64 decoder.
    base64: Base64Decoder<'i>,
}

impl<'i> Decoder<'i> {
    /// Create a new PEM [`Decoder`] with the default options.
    ///
    /// Uses the default 64-character line wrapping.
    pub fn new(pem: &'i [u8]) -> Result<Self> {
        Self::new_wrapped(pem, BASE64_WRAP_WIDTH)
    }

    /// Create a new PEM [`Decoder`] which wraps at the given line width.
    pub fn new_wrapped(pem: &'i [u8], line_width: usize) -> Result<Self> {
        let encapsulation = Encapsulation::try_from(pem)?;
        let type_label = encapsulation.label();
        let base64 = Base64Decoder::new_wrapped(encapsulation.encapsulated_text, line_width)?;
        Ok(Self { type_label, base64 })
    }

    /// Get the PEM type label for the input document.
    pub fn type_label(&self) -> &'i str {
        self.type_label
    }

    /// Decode data into the provided output buffer.
    ///
    /// There must be at least as much remaining Base64 input to be decoded
    /// in order to completely fill `buf`.
    pub fn decode<'o>(&mut self, buf: &'o mut [u8]) -> Result<&'o [u8]> {
        Ok(self.base64.decode(buf)?)
    }

    /// Decode all of the remaining data in the input buffer into `buf`.
    #[cfg(feature = "alloc")]
    pub fn decode_to_end<'o>(&mut self, buf: &'o mut Vec<u8>) -> Result<&'o [u8]> {
        Ok(self.base64.decode_to_end(buf)?)
    }

    /// Get the decoded length of the remaining PEM data after Base64 decoding.
    pub fn remaining_len(&self) -> usize {
        self.base64.remaining_len()
    }

    /// Are we finished decoding the PEM input?
    pub fn is_finished(&self) -> bool {
        self.base64.is_finished()
    }
}

impl<'i> From<Decoder<'i>> for Base64Decoder<'i> {
    fn from(decoder: Decoder<'i>) -> Base64Decoder<'i> {
        decoder.base64
    }
}

#[cfg(feature = "std")]
impl<'i> io::Read for Decoder<'i> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.base64.read(buf)
    }

    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        self.base64.read_to_end(buf)
    }

    fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
        self.base64.read_exact(buf)
    }
}

/// PEM encapsulation parser.
///
/// This parser performs an initial pass over the data, locating the
/// pre-encapsulation (`---BEGIN [...]---`) and post-encapsulation
/// (`---END [...]`) boundaries while attempting to avoid branching
/// on the potentially secret Base64-encoded data encapsulated between
/// the two boundaries.
///
/// It only supports a single encapsulated message at present. Future work
/// could potentially include extending it provide an iterator over a series
/// of encapsulated messages.
#[derive(Copy, Clone, Debug)]
struct Encapsulation<'a> {
    /// Type label extracted from the pre/post-encapsulation boundaries.
    ///
    /// From RFC 7468 Section 2:
    ///
    /// > The type of data encoded is labeled depending on the type label in
    /// > the "-----BEGIN " line (pre-encapsulation boundary).  For example,
    /// > the line may be "-----BEGIN CERTIFICATE-----" to indicate that the
    /// > content is a PKIX certificate (see further below).  Generators MUST
    /// > put the same label on the "-----END " line (post-encapsulation
    /// > boundary) as the corresponding "-----BEGIN " line.  Labels are
    /// > formally case-sensitive, uppercase, and comprised of zero or more
    /// > characters; they do not contain consecutive spaces or hyphen-minuses,
    /// > nor do they contain spaces or hyphen-minuses at either end.  Parsers
    /// > MAY disregard the label in the post-encapsulation boundary instead of
    /// > signaling an error if there is a label mismatch: some extant
    /// > implementations require the labels to match; others do not.
    label: &'a str,

    /// Encapsulated text portion contained between the boundaries.
    ///
    /// This data should be encoded as Base64, however this type performs no
    /// validation of it so it can be handled in constant-time.
    encapsulated_text: &'a [u8],
}

impl<'a> Encapsulation<'a> {
    /// Parse the type label and encapsulated text from between the
    /// pre/post-encapsulation boundaries.
    pub fn parse(data: &'a [u8]) -> Result<Self> {
        // Strip the "preamble": optional text occurring before the pre-encapsulation boundary
        let data = grammar::strip_preamble(data)?;

        // Parse pre-encapsulation boundary (including label)
        let data = data
            .strip_prefix(PRE_ENCAPSULATION_BOUNDARY)
            .ok_or(Error::PreEncapsulationBoundary)?;

        let (label, body) = grammar::split_label(data).ok_or(Error::Label)?;

        let mut body = match grammar::strip_trailing_eol(body).unwrap_or(body) {
            [head @ .., b'-', b'-', b'-', b'-', b'-'] => head,
            _ => return Err(Error::PreEncapsulationBoundary),
        };

        // Ensure body ends with a properly labeled post-encapsulation boundary
        for &slice in [POST_ENCAPSULATION_BOUNDARY, label.as_bytes()].iter().rev() {
            // Ensure the input ends with the post encapsulation boundary as
            // well as a matching label
            if !body.ends_with(slice) {
                return Err(Error::PostEncapsulationBoundary);
            }

            let len = body.len().checked_sub(slice.len()).ok_or(Error::Length)?;
            body = body.get(..len).ok_or(Error::PostEncapsulationBoundary)?;
        }

        let encapsulated_text =
            grammar::strip_trailing_eol(body).ok_or(Error::PostEncapsulationBoundary)?;

        Ok(Self {
            label,
            encapsulated_text,
        })
    }

    /// Get the label parsed from the encapsulation boundaries.
    pub fn label(self) -> &'a str {
        self.label
    }
}

impl<'a> TryFrom<&'a [u8]> for Encapsulation<'a> {
    type Error = Error;

    fn try_from(bytes: &'a [u8]) -> Result<Self> {
        Self::parse(bytes)
    }
}

/// Check for PEM headers in the input, as they are disallowed by RFC7468.
///
/// Returns `Error::HeaderDisallowed` if headers are encountered.
fn check_for_headers(pem: &[u8], err: Error) -> Error {
    if err == Error::Base64(base64ct::Error::InvalidEncoding)
        && pem.iter().any(|&b| b == grammar::CHAR_COLON)
    {
        Error::HeaderDisallowed
    } else {
        err
    }
}

#[cfg(test)]
mod tests {
    use super::Encapsulation;

    #[test]
    fn pkcs8_example() {
        let pem = include_bytes!("../tests/examples/pkcs8.pem");
        let encapsulation = Encapsulation::parse(pem).unwrap();
        assert_eq!(encapsulation.label, "PRIVATE KEY");

        assert_eq!(
            encapsulation.encapsulated_text,
            &[
                77, 67, 52, 67, 65, 81, 65, 119, 66, 81, 89, 68, 75, 50, 86, 119, 66, 67, 73, 69,
                73, 66, 102, 116, 110, 72, 80, 112, 50, 50, 83, 101, 119, 89, 109, 109, 69, 111,
                77, 99, 88, 56, 86, 119, 73, 52, 73, 72, 119, 97, 113, 100, 43, 57, 76, 70, 80,
                106, 47, 49, 53, 101, 113, 70
            ]
        );
    }
}
