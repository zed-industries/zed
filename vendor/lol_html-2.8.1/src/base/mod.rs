#[macro_use]
mod debug_trace;

mod align;
mod bytes;
mod encoding;
mod range;
mod spanned;

pub(crate) use self::align::Align;
pub(crate) use self::bytes::{Bytes, BytesCow, HasReplacementsError};
pub use self::encoding::SharedEncoding;
pub(crate) use self::range::Range;
pub use self::spanned::SourceLocation;
pub(crate) use self::spanned::{Spanned, SpannedRawBytes};

/// Unlike eq_ignore_ascii_case it only lowercases the first arg
pub(crate) fn eq_case_insensitive(mixed_case: &[u8], lowercased: &[u8]) -> bool {
    debug_assert!(lowercased.iter().all(|&b| b == b.to_ascii_lowercase()));

    if mixed_case.len() != lowercased.len() {
        return false;
    }

    for i in 0..mixed_case.len() {
        if mixed_case[i].to_ascii_lowercase() != lowercased[i] {
            return false;
        }
    }

    true
}
