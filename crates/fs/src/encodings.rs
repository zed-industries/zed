//! Encoding and decoding utilities using the `encoding_rs` crate.
use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

use std::sync::atomic::AtomicBool;

use anyhow::Result;
use encoding_rs::Encoding;

/// A wrapper around `encoding_rs::Encoding` to implement `Send` and `Sync`.
/// Since the reference is static, it is safe to send it across threads.
pub struct EncodingWrapper(pub &'static Encoding);

impl Debug for EncodingWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(&format!("EncodingWrapper{:?}", self.0))
            .field(&self.0.name())
            .finish()
    }
}

impl Default for EncodingWrapper {
    fn default() -> Self {
        EncodingWrapper(encoding_rs::UTF_8)
    }
}

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
        detect_utf16: bool,
        buffer_encoding: Option<Arc<Mutex<&'static Encoding>>>,
    ) -> Result<String> {
        // Check if the input starts with a BOM for UTF-16 encodings only if detect_utf16 is true.
        println!("{}", force);
        println!("{}", detect_utf16);
        if detect_utf16 {
            if let Some(encoding) = match input.get(..2) {
                Some([0xFF, 0xFE]) => Some(encoding_rs::UTF_16LE),
                Some([0xFE, 0xFF]) => Some(encoding_rs::UTF_16BE),
                _ => None,
            } {
                self.0 = encoding;

                if let Some(v) = buffer_encoding
                    && let Ok(mut v) = v.lock()
                {
                    *v = encoding;
                }
            }
        }

        let (cow, had_errors) = self.0.decode_with_bom_removal(&input);

        if force {
            return Ok(cow.to_string());
        }

        if !had_errors {
            Ok(cow.to_string())
        } else {
            Err(anyhow::anyhow!(
                "The file contains invalid bytes for the specified encoding: {}.\nThis usually means that the file is not a regular text file, or is encoded in a different encoding.\nContinuing to open it may result in data loss if saved.",
                self.0.name()
            ))
        }
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

            Ok(cow.into_owned())
        }
    }
}

/// Convert a byte vector from a specified encoding to a UTF-8 string.
pub async fn to_utf8(
    input: Vec<u8>,
    mut encoding: EncodingWrapper,
    force: bool,
    detect_utf16: bool,
    buffer_encoding: Option<Arc<Mutex<&'static Encoding>>>,
) -> Result<String> {
    encoding
        .decode(input, force, detect_utf16, buffer_encoding)
        .await
}

/// Convert a UTF-8 string to a byte vector in a specified encoding.
pub async fn from_utf8(input: String, target: EncodingWrapper) -> Result<Vec<u8>> {
    target.encode(input).await
}

pub struct EncodingOptions {
    pub encoding: Arc<Mutex<EncodingWrapper>>,
    pub force: AtomicBool,
    pub detect_utf16: AtomicBool,
}

impl Default for EncodingOptions {
    fn default() -> Self {
        EncodingOptions {
            encoding: Arc::new(Mutex::new(EncodingWrapper::default())),
            force: AtomicBool::new(false),
            detect_utf16: AtomicBool::new(true),
        }
    }
}
