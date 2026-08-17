#![allow(clippy::upper_case_acronyms)]
use core::{fmt, hash};

use crate::encode::{Encode, Encoding, RefEncode};

// Apple's objc4.
//
// Don't be fooled by the backup definition in `objc.h`; __OBJC_BOOL_IS_BOOL
// is always defined by `clang` when compiling Objective-C sources. The below
// cfgs are determined experimentally via cross compiling.
//
// See also <https://www.jviotti.com/2024/01/05/is-objective-c-bool-a-boolean-type-it-depends.html>.
#[cfg(all(not(feature = "gnustep-1-7"), not(feature = "unstable-objfw")))]
mod inner {
    // __OBJC_BOOL_IS_BOOL
    #[cfg(any(
        // aarch64-apple-*
        target_arch = "aarch64",
        // + x86_64-apple-ios (i.e. the simulator) (but not x86_64-apple-ios-macabi)
        all(target_os = "ios", target_pointer_width = "64", not(target_abi_macabi)),
        // + x86_64-apple-tvos
        all(target_os = "tvos", target_pointer_width = "64"),
        // + *-apple-watchos
        target_os = "watchos",
    ))]
    // C: _Bool
    pub(crate) type BOOL = bool;

    // Inverse of the above
    #[cfg(not(any(
        target_arch = "aarch64",
        all(target_os = "ios", target_pointer_width = "64", not(target_abi_macabi)),
        all(target_os = "tvos", target_pointer_width = "64"),
        target_os = "watchos",
    )))]
    // C: (explicitly) signed char
    pub(crate) type BOOL = i8;
}

// GNUStep's and Microsoft's libobjc2
#[cfg(all(
    feature = "gnustep-1-7",
    feature = "unstable-gnustep-strict-apple-compat"
))]
mod inner {
    // C: (explicitly) signed char
    pub(crate) type BOOL = i8;
}

#[cfg(all(
    feature = "gnustep-1-7",
    not(feature = "unstable-gnustep-strict-apple-compat")
))]
mod inner {
    // windows && !32bit-MinGW
    #[cfg(all(windows, not(all(target_pointer_width = "64", target_env = "gnu"))))]
    pub(crate) type BOOL = core::ffi::c_int;

    // The inverse
    #[cfg(not(all(windows, not(all(target_pointer_width = "64", target_env = "gnu")))))]
    // C: unsigned char
    pub(crate) type BOOL = u8;
}

// ObjFW
#[cfg(feature = "unstable-objfw")]
mod inner {
    // Defined in ObjFW-RT.h
    // C: signed char
    // This has changed since v0.90, but we don't support that yet.
    pub(crate) type BOOL = i8;

    // Note that ObjFW usually uses `bool` in return types, but that doesn't
    // change the ABI, so we'll use `BOOL` there as well, for ease of use.
}

/// The Objective-C `BOOL` type.
///
/// The type of `BOOL` varies across platforms, so we expose this wrapper. It
/// is intended that you convert this into a Rust [`bool`] with the
/// [`Bool::as_bool`] method as soon as possible.
///
/// This is FFI-safe and can be used directly with `msg_send!` and `extern`
/// functions as a substitute for `BOOL` in Objective-C. If your Objective-C
/// code uses C99 `_Bool`, you should use a `#[repr(transparent)]` wrapper
/// around `bool` instead.
///
/// Note that this is able to contain more states than `bool` on some
/// platforms, but these cases should not be relied on! The comparison traits
/// `PartialEq`, `PartialOrd` etc. will completely ignore these states.
///
/// See also the [corresponding documentation entry][docs].
///
/// [docs]: https://developer.apple.com/documentation/objectivec/bool?language=objc
#[repr(transparent)]
#[derive(Copy, Clone, Default)]
pub struct Bool {
    value: inner::BOOL,
}

impl Bool {
    /// The equivalent of [`true`] for Objective-C's `BOOL` type.
    #[allow(clippy::unnecessary_cast)]
    pub const YES: Self = Self::from_raw(true as inner::BOOL); // true -> 1

    /// The equivalent of [`false`] for Objective-C's `BOOL` type.
    #[allow(clippy::unnecessary_cast)]
    pub const NO: Self = Self::from_raw(false as inner::BOOL); // false -> 0

    /// Creates an Objective-C boolean from a Rust boolean.
    #[inline]
    pub const fn new(value: bool) -> Self {
        // true as BOOL => 1 (YES)
        // false as BOOL => 0 (NO)
        let value = value as inner::BOOL;
        Self { value }
    }

    /// Creates this from a raw boolean value.
    ///
    /// Avoid this, and instead use `Bool` in the raw FFI signature.
    #[inline]
    pub const fn from_raw(value: inner::BOOL) -> Self {
        Self { value }
    }

    /// Retrieves the inner boolean type.
    ///
    /// Avoid this, and instead use `Bool` in the raw FFI signature.
    #[inline]
    pub const fn as_raw(self) -> inner::BOOL {
        self.value
    }

    /// Returns `true` if `self` is [`NO`][Self::NO].
    ///
    /// You should prefer using [`as_bool`][Self::as_bool].
    #[inline]
    pub const fn is_false(self) -> bool {
        !self.as_bool()
    }

    /// Returns `true` if `self` is not [`NO`][Self::NO].
    ///
    /// You should prefer using [`as_bool`][Self::as_bool].
    #[inline]
    pub const fn is_true(self) -> bool {
        self.as_bool()
    }

    /// Converts this into the [`bool`] equivalent.
    #[inline]
    pub const fn as_bool(self) -> bool {
        // Always compare with 0 (NO)
        // This is what happens with the `!` operator / when using `if` in C.
        self.value != (false as inner::BOOL)
    }
}

impl From<bool> for Bool {
    #[inline]
    fn from(b: bool) -> Bool {
        Bool::new(b)
    }
}

impl From<Bool> for bool {
    #[inline]
    fn from(b: Bool) -> bool {
        b.as_bool()
    }
}

impl fmt::Debug for Bool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(if self.as_bool() { "YES" } else { "NO" })
    }
}

// Implement comparison traits by first converting to a boolean.

impl PartialEq for Bool {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.as_bool() == other.as_bool()
    }
}

impl Eq for Bool {}

impl hash::Hash for Bool {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.as_bool().hash(state);
    }
}

impl PartialOrd for Bool {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Bool {
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.as_bool().cmp(&other.as_bool())
    }
}

trait Helper {
    const __ENCODING: Encoding;
}

impl<T: Encode> Helper for T {
    const __ENCODING: Encoding = T::ENCODING;
}

impl Helper for bool {
    const __ENCODING: Encoding = Encoding::Bool;
}

// SAFETY: `Bool` is `repr(transparent)`.
unsafe impl Encode for Bool {
    // i8::__ENCODING == Encoding::Char
    // u8::__ENCODING == Encoding::UChar
    // bool::__ENCODING == Encoding::Bool
    // i32::__ENCODING == Encoding::Int
    const ENCODING: Encoding = inner::BOOL::__ENCODING;
}

// Note that we shouldn't delegate to `BOOL`'s  `ENCODING_REF` since `BOOL` is
// sometimes `i8`/`u8`, and their `ENCODING_REF`s are `Encoding::String`,
// which is incorrect for `BOOL`:
//
// ```objc
// @encode(BOOL); // -> "c", "C" or "B"
// @encode(BOOL*); // -> "^c", "^C" or "^B"
// @encode(char); // -> "c" or "C"
// @encode(char*); // -> "*"
// ```
unsafe impl RefEncode for Bool {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::__macro_helpers::{ConvertArgument, ConvertReturn};
    use alloc::format;

    #[test]
    fn test_basic() {
        let b = Bool::new(true);
        assert!(b.as_bool());
        assert!(b.is_true());
        assert!(!b.is_false());
        assert!(bool::from(b));
        assert_eq!(b.as_raw() as usize, 1);

        let b = Bool::new(false);
        assert!(!b.as_bool());
        assert!(!b.is_true());
        assert!(b.is_false());
        assert!(!bool::from(b));
        assert_eq!(b.as_raw() as usize, 0);
    }

    #[test]
    fn test_associated_constants() {
        let b = Bool::YES;
        assert!(b.as_bool());
        assert!(b.is_true());
        assert_eq!(b.as_raw() as usize, 1);

        let b = Bool::NO;
        assert!(!b.as_bool());
        assert!(b.is_false());
        assert_eq!(b.as_raw() as usize, 0);
    }

    #[test]
    fn test_encode() {
        assert_eq!(bool::__ENCODING, Encoding::Bool);

        assert_eq!(
            <bool as ConvertArgument>::__Inner::__ENCODING,
            <bool as ConvertArgument>::__Inner::ENCODING
        );
        assert_eq!(
            <bool as ConvertReturn<()>>::Inner::__ENCODING,
            <bool as ConvertReturn<()>>::Inner::ENCODING
        );
    }

    #[test]
    fn test_impls() {
        let b: Bool = Default::default();
        assert!(!b.as_bool());
        assert!(b.is_false());

        assert!(Bool::from(true).as_bool());
        assert!(Bool::from(true).is_true());
        assert!(Bool::from(false).is_false());

        assert!(Bool::from(true).is_true());
        assert!(Bool::from(false).is_false());

        assert_eq!(Bool::new(true), Bool::new(true));
        assert_eq!(Bool::new(false), Bool::new(false));

        assert!(Bool::new(false) < Bool::new(true));
    }

    #[test]
    fn test_debug() {
        assert_eq!(format!("{:?}", Bool::from(true)), "YES");
        assert_eq!(format!("{:?}", Bool::from(false)), "NO");
    }

    #[test]
    // Test on platform where we know the type of BOOL
    #[cfg(all(target_vendor = "apple", target_os = "macos", target_arch = "x86_64"))]
    fn test_outside_normal() {
        let b = Bool::from_raw(42);
        assert!(b.is_true());
        assert!(!b.is_false());
        assert_eq!(b.as_raw(), 42);

        // PartialEq ignores extra data
        assert_eq!(b, Bool::new(true));
        assert_ne!(b, Bool::new(false));
    }
}
