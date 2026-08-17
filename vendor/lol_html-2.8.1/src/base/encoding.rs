use crate::rewriter::AsciiCompatibleEncoding;
use encoding_rs::Encoding;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

/// This serves as a map from integer to [`Encoding`], which allows more efficient
/// sets/gets of the [`SharedEncoding`].
static ALL_ENCODINGS: [&Encoding; 40] = [
    encoding_rs::UTF_8,
    encoding_rs::SHIFT_JIS,
    encoding_rs::BIG5,
    encoding_rs::EUC_JP,
    encoding_rs::EUC_KR,
    encoding_rs::GB18030,
    encoding_rs::GBK,
    encoding_rs::IBM866,
    encoding_rs::ISO_8859_2,
    encoding_rs::ISO_8859_3,
    encoding_rs::ISO_8859_4,
    encoding_rs::ISO_8859_5,
    encoding_rs::ISO_8859_6,
    encoding_rs::ISO_8859_7,
    encoding_rs::ISO_8859_8_I,
    encoding_rs::ISO_8859_8,
    encoding_rs::ISO_8859_10,
    encoding_rs::ISO_8859_13,
    encoding_rs::ISO_8859_14,
    encoding_rs::ISO_8859_15,
    encoding_rs::ISO_8859_16,
    encoding_rs::KOI8_R,
    encoding_rs::KOI8_U,
    encoding_rs::MACINTOSH,
    encoding_rs::WINDOWS_1250,
    encoding_rs::WINDOWS_1251,
    encoding_rs::WINDOWS_1252,
    encoding_rs::WINDOWS_1253,
    encoding_rs::WINDOWS_1254,
    encoding_rs::WINDOWS_1255,
    encoding_rs::WINDOWS_1256,
    encoding_rs::WINDOWS_1257,
    encoding_rs::WINDOWS_1258,
    encoding_rs::WINDOWS_874,
    encoding_rs::X_MAC_CYRILLIC,
    encoding_rs::X_USER_DEFINED,
    // non-ASCII-compatible
    encoding_rs::REPLACEMENT,
    encoding_rs::UTF_16BE,
    encoding_rs::UTF_16LE,
    encoding_rs::ISO_2022_JP,
];

#[cfg_attr(debug_assertions, track_caller)]
fn encoding_to_index(encoding: AsciiCompatibleEncoding) -> usize {
    let encoding: &'static Encoding = encoding.into();

    let index = ALL_ENCODINGS.iter().position(|&e| e == encoding);
    debug_assert!(
        index.is_some(),
        "the ALL_ENCODINGS is not complete and needs to be updated"
    );
    index.unwrap_or(0)
}

/// A charset encoding that can be shared and modified.
///
/// This is, for instance, used to adapt the charset dynamically in a [`crate::HtmlRewriter`] if it
/// encounters a `meta` tag that specifies the charset (that behavior is dependent on
/// [`crate::Settings::adjust_charset_on_meta_tag`]).
// Pub only for integration tests
#[derive(Clone)]
pub struct SharedEncoding {
    encoding: Arc<AtomicUsize>,
}

impl SharedEncoding {
    #[must_use]
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn new(encoding: AsciiCompatibleEncoding) -> Self {
        Self {
            encoding: Arc::new(AtomicUsize::new(encoding_to_index(encoding))),
        }
    }

    #[must_use]
    pub fn get(&self) -> &'static Encoding {
        let encoding = self.encoding.load(Ordering::Relaxed);
        // it will never be out of range, but get() avoids a panic branch
        ALL_ENCODINGS.get(encoding).unwrap_or(&ALL_ENCODINGS[0])
    }

    #[cfg_attr(debug_assertions, track_caller)]
    pub fn set(&self, encoding: AsciiCompatibleEncoding) {
        self.encoding
            .store(encoding_to_index(encoding), Ordering::Relaxed);
    }
}

#[cfg(test)]
mod tests {
    use crate::AsciiCompatibleEncoding;
    use crate::base::SharedEncoding;
    use crate::base::encoding::ALL_ENCODINGS;

    #[test]
    fn test_encoding_round_trip() {
        let shared_encoding = SharedEncoding::new(AsciiCompatibleEncoding::utf_8());

        for encoding in ALL_ENCODINGS {
            if let Some(ascii_compat_encoding) = AsciiCompatibleEncoding::new(encoding) {
                shared_encoding.set(ascii_compat_encoding);
                assert_eq!(shared_encoding.get(), encoding);
            }
        }
    }
}
