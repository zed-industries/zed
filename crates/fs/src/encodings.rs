//! Encoding and decoding utilities using the `encoding_rs` crate.
use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

use anyhow::Result;
use encoding_rs::Encoding;

/// A wrapper around `encoding_rs::Encoding` to implement `Send` and `Sync`.
/// Since the reference is static, it is safe to send it across threads.
pub struct EncodingWrapper(&'static Encoding);

impl Debug for EncodingWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(&format!("EncodingWrapper{:?}", self.0))
            .field(&self.0.name())
            .finish()
    }
}

pub struct EncodingWrapperVisitor;

impl PartialEq for EncodingWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.0.name() == other.0.name()
    }
}

unsafe impl Send for EncodingWrapper {}
unsafe impl Sync for EncodingWrapper {}

impl Clone for EncodingWrapper {
    fn clone(&self) -> Self {
        EncodingWrapper(self.0)
    }
}

impl EncodingWrapper {
    pub fn new(encoding: &'static Encoding) -> EncodingWrapper {
        EncodingWrapper(encoding)
    }

    pub fn get_encoding(&self) -> &'static Encoding {
        self.0
    }

    pub async fn decode(
        &mut self,
        input: Vec<u8>,
        force: bool,
        buffer_encoding: Option<Arc<Mutex<&'static Encoding>>>,
    ) -> Result<String> {
        // Check if the input starts with a BOM for UTF-16 encodings only if not forced to
        // use the encoding specified.
        if !force {
            if let Some(encoding) = match input.get(..2) {
                Some([0xFF, 0xFE]) => Some(encoding_rs::UTF_16LE),
                Some([0xFE, 0xFF]) => Some(encoding_rs::UTF_16BE),
                _ => None,
            } {
                self.0 = encoding;

                if let Some(v) = buffer_encoding {
                    if let Ok(mut v) = (*v).lock() {
                        *v = encoding;
                    }
                }
            }
        }

        let (cow, _had_errors) = self.0.decode_with_bom_removal(&input);

        // `encoding_rs` handles invalid bytes by replacing them with replacement characters
        // in the output string, so we return the result even if there were errors.
        // This preserves the original behaviour where files with invalid bytes could still be opened.
        Ok(cow.into_owned())
    }

    pub async fn encode(&self, input: String) -> Result<Vec<u8>> {
        if self.0 == encoding_rs::UTF_16BE {
            let mut data = Vec::<u8>::with_capacity(input.len() * 2);

            // Convert the input string to UTF-16BE bytes
            let utf16be_bytes = input.encode_utf16().flat_map(|u| u.to_be_bytes());

            data.extend(utf16be_bytes);
            return Ok(data);
        } else if self.0 == encoding_rs::UTF_16LE {
            let mut data = Vec::<u8>::with_capacity(input.len() * 2);

            // Convert the input string to UTF-16LE bytes
            let utf16le_bytes = input.encode_utf16().flat_map(|u| u.to_le_bytes());

            data.extend(utf16le_bytes);
            return Ok(data);
        } else {
            let (cow, _encoding_used, _had_errors) = self.0.encode(&input);
            // `encoding_rs` handles unencodable characters by replacing them with
            // appropriate substitutes in the output, so we return the result even if there were errors.
            // This maintains consistency with the decode behaviour.
            Ok(cow.into_owned())
        }
    }
}

/// Convert a byte vector from a specified encoding to a UTF-8 string.
pub async fn to_utf8(
    input: Vec<u8>,
    mut encoding: EncodingWrapper,
    force: bool,
    buffer_encoding: Option<Arc<Mutex<&'static Encoding>>>,
) -> Result<String> {
    encoding.decode(input, force, buffer_encoding).await
}

/// Convert a UTF-8 string to a byte vector in a specified encoding.
pub async fn from_utf8(input: String, target: EncodingWrapper) -> Result<Vec<u8>> {
    target.encode(input).await
}
