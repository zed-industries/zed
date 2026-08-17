use core::ffi::c_char;
use core::slice;
use core::str;

use crate::ffi::NSUInteger;
use crate::msg_send;
use crate::rc::AutoreleasePool;
use crate::runtime::NSObject;

// Note: While this is not public, it is still a breaking change to modify,
// since `objc2-foundation` relies on it.
#[cfg(not(feature = "gnustep-1-7"))]
pub const UTF8_ENCODING: usize = 4;
#[cfg(feature = "gnustep-1-7")]
pub const UTF8_ENCODING: i32 = 4;

/// The number of UTF-8 code units in the given string.
///
/// # Safety
///
/// The object must be an instance of `NSString`.
//
// Note: While this is not public, it is still a breaking change to modify,
// since `objc2-foundation` relies on it.
#[inline]
pub unsafe fn nsstring_len(obj: &NSObject) -> NSUInteger {
    unsafe { msg_send![obj, lengthOfBytesUsingEncoding: UTF8_ENCODING] }
}

/// Extract a [`str`](`prim@str`) representation out of the given NSString.
///
/// Uses [`UTF8String`] under the hood.
///
/// [`UTF8String`]: https://developer.apple.com/documentation/foundation/nsstring/1411189-utf8string?language=objc
///
///
/// # Safety
///
/// - The object must be an instance of `NSString`.
/// - The returned string must not be moved outside the autorelease pool into
///   which it (may) have been released.
///
/// Furthermore, the object must not, as is always the case for strings, be
/// mutated in parallel.
//
// Note: While this is not public, it is still a breaking change to modify,
// since `objc2-foundation` relies on it.
pub unsafe fn nsstring_to_str<'r, 's: 'r, 'p: 'r>(
    obj: &'s NSObject,
    pool: AutoreleasePool<'p>,
) -> &'r str {
    // This is necessary until `auto` types stabilizes.
    pool.__verify_is_inner();

    // The documentation on `UTF8String` is quite sparse, but with educated
    // guesses, testing, reading the code for `CFString` and a bit of
    // reverse-engineering, we've determined that `NSString` stores a pointer
    // to the string data, sometimes with an UTF-8 encoding (usual for ascii
    // data), sometimes in other encodings (often UTF-16).
    //
    // `UTF8String` then checks the internal encoding:
    // - If the data is UTF-8 encoded, and (since macOS 10.6) if the string is
    //   immutable, it returns the internal pointer using
    //   `CFStringGetCStringPtr`.
    // - Otherwise, if the data is in another encoding or is mutable, it
    //   creates a new allocation, writes the UTF-8 representation of the
    //   string into it, autoreleases the allocation, and returns a pointer to
    //   it (similar to `CFStringGetCString`).
    //
    // If the string is a tagged pointer, or a custom subclass, then another
    // code-path is taken that always creates a new allocation and copies the
    // string into that using (effectively) `length` and `characterAtIndex:`.
    //
    // As a result, the lifetime of the returned pointer is either the same as
    // the passed-in `NSString` OR the lifetime of the current / innermost
    // `@autoreleasepool`.
    //
    // Furthermore, we can allow creating a `&str` from `&obj`, even if the
    // string is originally a `NSMutableString` which may be mutated later on,
    // since in that case the lifetime will be tied to the pool and not the
    // string.
    let bytes: *const c_char = unsafe { msg_send![obj, UTF8String] };
    let bytes: *const u8 = bytes.cast();

    // SAFETY: Caller ensures that the object is an instance of `NSString`.
    let len = unsafe { nsstring_len(obj) };

    // SAFETY:
    // The held AutoreleasePool is the innermost, and the reference is
    // constrained both by the pool and the NSString.
    //
    // `len` is the length of the string in the UTF-8 encoding.
    //
    // `bytes` is a null-terminated C string (with length = len + 1), so
    // it is never a NULL pointer.
    let bytes: &'r [u8] = unsafe { slice::from_raw_parts(bytes, len) };

    // SAFETY: The bytes are valid UTF-8.
    #[cfg(not(debug_assertions))]
    unsafe {
        str::from_utf8_unchecked(bytes)
    }

    #[cfg(debug_assertions)]
    {
        str::from_utf8(bytes).expect("invalid UTF-8 in NSString")
    }
}
