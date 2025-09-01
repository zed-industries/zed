//! Encoding and decoding utilities using the `encoding_rs` crate.
use std::fmt::Debug;

use anyhow::{Error, Result};
use encoding_rs::Encoding;
use serde::{Deserialize, de::Visitor};

/// A wrapper around `encoding_rs::Encoding` to implement `Send` and `Sync`.
/// Since the reference is static, it is safe to send it across threads.
pub struct EncodingWrapper(&'static Encoding);

impl Debug for EncodingWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("EncodingWrapper")
            .field(&self.0.name())
            .finish()
    }
}

pub struct EncodingWrapperVisitor;

impl<'vi> Visitor<'vi> for EncodingWrapperVisitor {
    type Value = EncodingWrapper;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid encoding name")
    }

    fn visit_str<E: serde::de::Error>(self, encoding: &str) -> Result<EncodingWrapper, E> {
        Ok(EncodingWrapper(
            Encoding::for_label(encoding.as_bytes())
                .ok_or_else(|| serde::de::Error::custom("Invalid Encoding"))?,
        ))
    }

    fn visit_string<E: serde::de::Error>(self, encoding: String) -> Result<EncodingWrapper, E> {
        Ok(EncodingWrapper(
            Encoding::for_label(encoding.as_bytes())
                .ok_or_else(|| serde::de::Error::custom("Invalid Encoding"))?,
        ))
    }
}

impl<'de> Deserialize<'de> for EncodingWrapper {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(EncodingWrapperVisitor)
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

    pub async fn decode(&self, input: Vec<u8>) -> Result<String> {
        let (cow, _encoding_used, _had_errors) = self.0.decode(&input);
        // encoding_rs handles invalid bytes by replacing them with replacement characters
        // in the output string, so we return the result even if there were errors.
        // This preserves the original behavior where files with invalid bytes could still be opened.
        Ok(cow.into_owned())
    }

    pub async fn encode(&self, input: String) -> Result<Vec<u8>> {
        let (cow, _encoding_used, _had_errors) = self.0.encode(&input);
        // encoding_rs handles unencodable characters by replacing them with 
        // appropriate substitutes in the output, so we return the result even if there were errors.
        // This maintains consistency with the decode behavior.
        Ok(cow.into_owned())
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

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::BackgroundExecutor;
    
    #[gpui::test]
    async fn test_decode_with_invalid_bytes(_: BackgroundExecutor) {
        // Test that files with invalid bytes can still be decoded
        // This is a regression test for the issue where files couldn't be opened
        // when they contained invalid bytes for the specified encoding
        
        // Create some invalid UTF-8 bytes
        let invalid_bytes = vec![0xFF, 0xFE, 0x00, 0x48]; // Invalid UTF-8 sequence
        
        let encoding = EncodingWrapper::new(encoding_rs::UTF_8);
        let result = encoding.decode(invalid_bytes).await;
        
        // The decode should succeed, not fail
        assert!(result.is_ok(), "Decode should succeed even with invalid bytes");
        
        let decoded = result.unwrap();
        // The result should contain replacement characters for invalid sequences
        assert!(!decoded.is_empty(), "Decoded string should not be empty");
        
        // Test with Windows-1252 and some bytes that might be invalid
        let maybe_invalid_bytes = vec![0x81, 0x8D, 0x8F, 0x90, 0x9D]; // Some potentially problematic bytes
        let encoding = EncodingWrapper::new(encoding_rs::WINDOWS_1252);
        let result = encoding.decode(maybe_invalid_bytes).await;
        
        // Should still succeed
        assert!(result.is_ok(), "Decode should succeed with Windows-1252 even with potentially invalid bytes");
    }
    
    #[gpui::test]
    async fn test_encode_with_unencodable_chars(_: BackgroundExecutor) {
        // Test that strings with unencodable characters can still be encoded
        let input = "Hello 世界 🌍".to_string(); // Contains Unicode that may not encode to all formats
        
        let encoding = EncodingWrapper::new(encoding_rs::WINDOWS_1252);
        let result = encoding.encode(input).await;
        
        // The encode should succeed, not fail
        assert!(result.is_ok(), "Encode should succeed even with unencodable characters");
        
        let encoded = result.unwrap();
        assert!(!encoded.is_empty(), "Encoded bytes should not be empty");
    }
}
