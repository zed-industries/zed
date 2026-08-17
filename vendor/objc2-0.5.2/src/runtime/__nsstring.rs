use core::slice;
use core::str;
use std::os::raw::c_char;

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
pub unsafe fn nsstring_len(obj: &NSObject) -> NSUInteger {
    unsafe { msg_send![obj, lengthOfBytesUsingEncoding: UTF8_ENCODING] }
}

/// Extract a [`str`](`prim@str`) representation out of the given NSString.
///
/// # Safety
///
/// The object must be an instance of `NSString`.
//
// Note: While this is not public, it is still a breaking change to modify,
// since `objc2-foundation` relies on it.
pub unsafe fn nsstring_to_str<'r, 's: 'r, 'p: 'r>(
    obj: &'s NSObject,
    pool: AutoreleasePool<'p>,
) -> &'r str {
    // This is necessary until `auto` types stabilizes.
    pool.__verify_is_inner();

    // The documentation on `UTF8String` is a bit sparse, but with
    // educated guesses and testing I've determined that NSString stores
    // a pointer to the string data, sometimes with an UTF-8 encoding,
    // (usual for ascii data), sometimes in other encodings (UTF-16?).
    //
    // `UTF8String` then checks the internal encoding:
    // - If the data is UTF-8 encoded, it returns the internal pointer.
    // - If the data is in another encoding, it creates a new allocation,
    //   writes the UTF-8 representation of the string into it,
    //   autoreleases the allocation and returns a pointer to it.
    //
    // So the lifetime of the returned pointer is either the same as the
    // NSString OR the lifetime of the innermost @autoreleasepool.
    //
    // https://developer.apple.com/documentation/foundation/nsstring/1411189-utf8string?language=objc
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

    // TODO: Always UTF-8, so should we use `from_utf8_unchecked`?
    str::from_utf8(bytes).unwrap()
}
