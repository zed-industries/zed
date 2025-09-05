//! Encoding and decoding utilities using the `encoding_rs` crate.
use std::fmt::Debug;

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

    pub async fn decode(&self, input: Vec<u8>) -> Result<String> {
        let (cow, _had_errors) = self.0.decode_with_bom_removal(&input);
        // `encoding_rs` handles invalid bytes by replacing them with replacement characters
        // in the output string, so we return the result even if there were errors.
        // This preserves the original behaviour where files with invalid bytes could still be opened.
        Ok(cow.into_owned())
    }

    pub async fn encode(&self, input: String) -> Result<Vec<u8>> {
        if self.0 == encoding_rs::UTF_16BE {
            let mut data: Vec<u8> = vec![];
            let utf = input.encode_utf16().collect::<Vec<u16>>();

            for i in utf {
                let byte = i.to_be_bytes();
                for b in byte {
                    data.push(b);
                }
            }
            return Ok(data);
        } else if self.0 == encoding_rs::UTF_16LE {
            let mut data: Vec<u8> = vec![];
            let utf = input.encode_utf16().collect::<Vec<u16>>();

            for i in utf {
                let byte = i.to_le_bytes();
                for b in byte {
                    data.push(b);
                }
            }
            return Ok(data);
        } else {
            let (cow, _encoding_used, _had_errors) = self.0.encode(&input);
            println!("Encoding: {:?}", self);
            // `encoding_rs` handles unencodable characters by replacing them with
            // appropriate substitutes in the output, so we return the result even if there were errors.
            // This maintains consistency with the decode behaviour.
            Ok(cow.into_owned())
        }
    }
}

/// Convert a byte vector from a specified encoding to a UTF-8 string.
pub async fn to_utf8(input: Vec<u8>, encoding: EncodingWrapper) -> Result<String> {
    encoding.decode(input).await
}

/// Convert a UTF-8 string to a byte vector in a specified encoding.
pub async fn from_utf8(input: String, target: EncodingWrapper) -> Result<Vec<u8>> {
    target.encode(input).await
}
