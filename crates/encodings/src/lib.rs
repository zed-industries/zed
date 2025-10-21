use encoding_rs;
use std::{
    fmt::Debug,
    sync::{Arc, Mutex, atomic::AtomicBool},
};

pub use encoding_rs::{
    BIG5, EUC_JP, EUC_KR, GB18030, GBK, IBM866, ISO_2022_JP, ISO_8859_2, ISO_8859_3, ISO_8859_4,
    ISO_8859_5, ISO_8859_6, ISO_8859_7, ISO_8859_8, ISO_8859_8_I, ISO_8859_10, ISO_8859_13,
    ISO_8859_14, ISO_8859_15, ISO_8859_16, KOI8_R, KOI8_U, MACINTOSH, SHIFT_JIS, UTF_8, UTF_16BE,
    UTF_16LE, WINDOWS_874, WINDOWS_1250, WINDOWS_1251, WINDOWS_1252, WINDOWS_1253, WINDOWS_1254,
    WINDOWS_1255, WINDOWS_1256, WINDOWS_1257, WINDOWS_1258, X_MAC_CYRILLIC,
};

pub struct Encoding(Mutex<&'static encoding_rs::Encoding>);

impl Debug for Encoding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(&format!("Encoding{:?}", self.0))
            .field(&self.get().name())
            .finish()
    }
}

impl Clone for Encoding {
    fn clone(&self) -> Self {
        Encoding(Mutex::new(self.get()))
    }
}

impl Default for Encoding {
    fn default() -> Self {
        Encoding(Mutex::new(UTF_8))
    }
}

unsafe impl Send for Encoding {}
unsafe impl Sync for Encoding {}

impl Encoding {
    pub fn new(encoding: &'static encoding_rs::Encoding) -> Self {
        Self(Mutex::new(encoding))
    }

    pub fn set(&self, encoding: &'static encoding_rs::Encoding) {
        *self.0.lock().unwrap() = encoding;
    }

    pub fn get(&self) -> &'static encoding_rs::Encoding {
        *self.0.lock().unwrap()
    }

    pub async fn decode(
        &self,
        input: Vec<u8>,
        force: bool,
        detect_utf16: bool,
        buffer_encoding: Option<Arc<Encoding>>,
    ) -> anyhow::Result<String> {
        // Check if the input starts with a BOM for UTF-16 encodings only if detect_utf16 is true.
        if detect_utf16 {
            if let Some(encoding) = match input.get(..2) {
                Some([0xFF, 0xFE]) => Some(UTF_16LE),
                Some([0xFE, 0xFF]) => Some(UTF_16BE),
                _ => None,
            } {
                self.set(encoding);

                if let Some(v) = buffer_encoding {
                    v.set(encoding)
                }
            }
        }

        let (cow, had_errors) = self.get().decode_with_bom_removal(&input);

        if force {
            return Ok(cow.to_string());
        }

        if !had_errors {
            Ok(cow.to_string())
        } else {
            Err(anyhow::anyhow!(
                "The file contains invalid bytes for the specified encoding: {}.\nThis usually means that the file is not a regular text file, or is encoded in a different encoding.\nContinuing to open it may result in data loss if saved.",
                self.get().name()
            ))
        }
    }

    pub async fn encode(&self, input: String) -> anyhow::Result<Vec<u8>> {
        if self.get() == UTF_16BE {
            let mut data = Vec::<u8>::with_capacity(input.len() * 2);

            // Convert the input string to UTF-16BE bytes
            let utf16be_bytes = input.encode_utf16().flat_map(|u| u.to_be_bytes());

            data.extend(utf16be_bytes);
            return Ok(data);
        } else if self.get() == UTF_16LE {
            let mut data = Vec::<u8>::with_capacity(input.len() * 2);

            // Convert the input string to UTF-16LE bytes
            let utf16le_bytes = input.encode_utf16().flat_map(|u| u.to_le_bytes());

            data.extend(utf16le_bytes);
            return Ok(data);
        } else {
            let (cow, _encoding_used, _had_errors) = self.get().encode(&input);

            Ok(cow.into_owned())
        }
    }

    pub fn reset(&self) {
        self.set(UTF_8);
    }
}

/// Convert a byte vector from a specified encoding to a UTF-8 string.
pub async fn to_utf8(
    input: Vec<u8>,
    encoding: Encoding,
    force: bool,
    detect_utf16: bool,
    buffer_encoding: Option<Arc<Encoding>>,
) -> anyhow::Result<String> {
    encoding
        .decode(input, force, detect_utf16, buffer_encoding)
        .await
}

/// Convert a UTF-8 string to a byte vector in a specified encoding.
pub async fn from_utf8(input: String, target: Encoding) -> anyhow::Result<Vec<u8>> {
    target.encode(input).await
}

pub struct EncodingOptions {
    pub encoding: Arc<Encoding>,
    pub force: AtomicBool,
    pub detect_utf16: AtomicBool,
}

impl EncodingOptions {
    pub fn reset(&self) {
        self.encoding.reset();

        self.force
            .store(false, std::sync::atomic::Ordering::Release);

        self.detect_utf16
            .store(true, std::sync::atomic::Ordering::Release);
    }
}

impl Default for EncodingOptions {
    fn default() -> Self {
        EncodingOptions {
            encoding: Arc::new(Encoding::default()),
            force: AtomicBool::new(false),
            detect_utf16: AtomicBool::new(true),
        }
    }
}
