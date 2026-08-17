#![allow(missing_copy_implementations)]
//! Macro for making a static NSString.
//!
//! This closely follows what clang does, see:
//! - Apple: <https://github.com/llvm/llvm-project/blob/release/13.x/clang/lib/CodeGen/CodeGenModule.cpp#L5057-L5249>
//! - GNUStep 2.0 (not yet supported): <https://github.com/llvm/llvm-project/blob/release/13.x/clang/lib/CodeGen/CGObjCGNU.cpp#L973-L1118>
//! - Other (not yet supported): <https://github.com/llvm/llvm-project/blob/release/13.x/clang/lib/CodeGen/CGObjCGNU.cpp#L2471-L2507>
//!
//! Note that this uses the `CFString` static, while `clang` has support for
//! generating a pure `NSString`. We don't support that yet (since I don't
//! know the use-case), but we definitely could!
//! See: <https://github.com/llvm/llvm-project/blob/release/13.x/clang/lib/CodeGen/CGObjCMac.cpp#L2007-L2068>
//!
//! See also the following crates that implement UTF-16 conversion:
//! `utf16_lit`, `windows`, `const_utf16`, `wide-literals`, ...
use core::ffi::c_void;

use crate::Foundation::NSString;
use objc2::runtime::AnyClass;

// This is defined in CoreFoundation, but we don't emit a link attribute
// here because it is already linked via Foundation.
//
// Although this is a "private" (underscored) symbol, it is directly
// referenced in Objective-C binaries. So it's safe for us to reference.
extern "C" {
    pub static __CFConstantStringClassReference: AnyClass;
}

/// Structure used to describe a constant `CFString`.
///
/// This struct is the same as [`CF_CONST_STRING`], which contains
/// [`CFRuntimeBase`]. While the documentation clearly says that the ABI of
/// `CFRuntimeBase` should not be relied on, we can rely on it as long as we
/// only do it with regards to `CFString` (because `clang` does this as well).
///
/// [`CFRuntimeBase`]: <https://github.com/apple-oss-distributions/CF/blob/CF-1153.18/CFRuntime.h#L216-L228>
/// [`CF_CONST_STRING`]: <https://github.com/apple-oss-distributions/CF/blob/CF-1153.18/CFInternal.h#L332-L336>
#[repr(C)]
#[derive(Debug)]
pub struct CFConstString {
    isa: &'static AnyClass,
    // Important that we don't use `usize` here, since that would be wrong on
    // big-endian systems!
    cfinfo: u32,
    #[cfg(target_pointer_width = "64")]
    _rc: u32,
    data: *const c_void,
    len: usize,
}

// Required to place in a `static`.
unsafe impl Sync for CFConstString {}

impl CFConstString {
    // From `CFString.c`:
    // <https://github.com/apple-oss-distributions/CF/blob/CF-1153.18/CFString.c#L184-L212>
    // > !!! Note: Constant CFStrings use the bit patterns:
    // > C8 (11001000 = default allocator, not inline, not freed contents; 8-bit; has NULL byte; doesn't have length; is immutable)
    // > D0 (11010000 = default allocator, not inline, not freed contents; Unicode; is immutable)
    // > The bit usages should not be modified in a way that would effect these bit patterns.
    //
    // Hence CoreFoundation guarantees that these two are always valid.
    //
    // The `CFTypeID` of `CFStringRef` is guaranteed to always be 7:
    // <https://github.com/apple-oss-distributions/CF/blob/CF-1153.18/CFRuntime.c#L982>
    const FLAGS_ASCII: u32 = 0x07_C8;
    const FLAGS_UTF16: u32 = 0x07_D0;

    pub const unsafe fn new_ascii(isa: &'static AnyClass, data: &'static [u8]) -> Self {
        Self {
            isa,
            cfinfo: Self::FLAGS_ASCII,
            #[cfg(target_pointer_width = "64")]
            _rc: 0,
            data: data.as_ptr().cast(),
            // The length does not include the trailing NUL.
            len: data.len() - 1,
        }
    }

    pub const unsafe fn new_utf16(isa: &'static AnyClass, data: &'static [u16]) -> Self {
        Self {
            isa,
            cfinfo: Self::FLAGS_UTF16,
            #[cfg(target_pointer_width = "64")]
            _rc: 0,
            data: data.as_ptr().cast(),
            // The length does not include the trailing NUL.
            len: data.len() - 1,
        }
    }

    #[inline]
    pub const fn as_nsstring_const(&self) -> &NSString {
        let ptr: *const Self = self;
        unsafe { &*ptr.cast::<NSString>() }
    }

    // This is deliberately not `const` to prevent the result from being used
    // in other statics, since that is only possible if the
    // `unstable-static-nsstring` feature is enabled.
    #[inline]
    pub fn as_nsstring(&self) -> &NSString {
        self.as_nsstring_const()
    }
}

/// Returns `true` if `bytes` is entirely ASCII with no interior NULs.
pub const fn is_ascii_no_nul(bytes: &[u8]) -> bool {
    let mut i = 0;
    while i < bytes.len() {
        let byte = bytes[i];
        if !byte.is_ascii() || byte == b'\0' {
            return false;
        }
        i += 1;
    }
    true
}

#[derive(Debug)]
pub struct Utf16Char {
    pub repr: [u16; 2],
    pub len: usize,
}

impl Utf16Char {
    const fn encode(ch: u32) -> Self {
        if ch <= 0xffff {
            Self {
                repr: [ch as u16, 0],
                len: 1,
            }
        } else {
            let payload = ch - 0x10000;
            let hi = (payload >> 10) | 0xd800;
            let lo = (payload & 0x3ff) | 0xdc00;
            Self {
                repr: [hi as u16, lo as u16],
                len: 2,
            }
        }
    }

    #[cfg(test)]
    fn as_slice(&self) -> &[u16] {
        &self.repr[..self.len]
    }
}

#[derive(Debug)]
pub struct EncodeUtf16Iter {
    str: &'static [u8],
    index: usize,
}

impl EncodeUtf16Iter {
    pub const fn new(str: &'static [u8]) -> Self {
        Self { str, index: 0 }
    }

    pub const fn next(self) -> Option<(Self, Utf16Char)> {
        if self.index >= self.str.len() {
            None
        } else {
            let (index, ch) = decode_utf8(self.str, self.index);
            Some((Self { index, ..self }, Utf16Char::encode(ch)))
        }
    }
}

// (&str bytes, index) -> (new index, decoded char)
const fn decode_utf8(s: &[u8], i: usize) -> (usize, u32) {
    let b0 = s[i];
    match b0 {
        // one-byte seq
        0b0000_0000..=0b0111_1111 => {
            let decoded = b0 as u32;
            (i + 1, decoded)
        }
        // two-byte seq
        0b1100_0000..=0b1101_1111 => {
            let decoded = ((b0 as u32 & 0x1f) << 6) | (s[i + 1] as u32 & 0x3f);
            (i + 2, decoded)
        }
        // 3 byte seq
        0b1110_0000..=0b1110_1111 => {
            let decoded = ((b0 as u32 & 0x0f) << 12)
                | ((s[i + 1] as u32 & 0x3f) << 6)
                | (s[i + 2] as u32 & 0x3f);
            (i + 3, decoded)
        }
        // 4 byte seq
        0b1111_0000..=0b1111_0111 => {
            let decoded = ((b0 as u32 & 0x07) << 18)
                | ((s[i + 1] as u32 & 0x3f) << 12)
                | ((s[i + 2] as u32 & 0x3f) << 6)
                | (s[i + 3] as u32 & 0x3f);
            (i + 4, decoded)
        }
        // continuation bytes, or never-valid bytes.
        0b1000_0000..=0b1011_1111 | 0b1111_1000..=0b1111_1111 => {
            panic!("Encountered invalid bytes")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_ascii() {
        assert!(is_ascii_no_nul(b"a"));
        assert!(is_ascii_no_nul(b"abc"));

        assert!(!is_ascii_no_nul(b"\xff"));

        assert!(!is_ascii_no_nul(b"\0"));
        assert!(!is_ascii_no_nul(b"a\0b"));
        assert!(!is_ascii_no_nul(b"ab\0"));
        assert!(!is_ascii_no_nul(b"a\0b\0"));
    }

    #[test]
    #[ignore = "slow, enable this if working on ns_string!"]
    fn test_decode_utf8() {
        for c in '\u{0}'..=core::char::MAX {
            let mut buf;
            for off in 0..4 {
                // Ensure we see garbage if we read outside bounds.
                buf = [0xff; 8];
                let len = c.encode_utf8(&mut buf[off..(off + 4)]).len();
                let (end_idx, decoded) = decode_utf8(&buf, off);
                assert_eq!(
                    (end_idx, decoded),
                    (off + len, c as u32),
                    "failed for U+{code:04X} ({ch:?}) encoded as {buf:#x?} over {range:?}",
                    code = c as u32,
                    ch = c,
                    buf = &buf[off..(off + len)],
                    range = off..(off + len),
                );
            }
        }
    }

    #[test]
    #[ignore = "slow, enable this if working on ns_string!"]
    fn encode_utf16() {
        for c in '\u{0}'..=core::char::MAX {
            assert_eq!(
                c.encode_utf16(&mut [0u16; 2]),
                Utf16Char::encode(c as u32).as_slice(),
                "failed for U+{:04X} ({:?})",
                c as u32,
                c
            );
        }
    }
}
