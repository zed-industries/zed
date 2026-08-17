//! Objective-C type aliases.
#![allow(non_camel_case_types)]

// # Why isize/usize is correct for NSInteger/NSUInteger
//
// ## Apple
// The documentation clearly states:
//
// > When building 32-bit applications, NSInteger is a 32-bit integer. A
//   64-bit application treats NSInteger as a 64-bit integer.
//
// And the header file defines them like so:
//
//     #if __LP64__ || TARGET_OS_WIN32 || NS_BUILD_32_LIKE_64
//     typedef long NSInteger;
//     typedef unsigned long NSUInteger;
//     #else
//     typedef int NSInteger;
//     typedef unsigned int NSUInteger;
//     #endif
//
// Rust (or at least `libc`) has no targets where c_int/c_uint are not 32-bit,
// so that part is correct. By manual inspection it is found that the only
// platform where c_long/c_ulong differs from isize/usize is on Windows.
// However Apple's libraries are only designed to work on 32-bit Windows, so
// this case should be fine as well.
//
// Likewise for NSUInteger.
//
//
// ## GNUStep / WinObjC
//
// Defined as intptr_t/uintptr_t, which is exactly the same as isize/usize.
//
//
// ## ObjFW
//
// Doesn't define these, but e.g. -[OFString length] returns size_t, so our
// definitions should be correct on effectively all targets.
//
// Things might change slightly in the future, see
// <https://internals.rust-lang.org/t/pre-rfc-usize-is-not-size-t/15369>.

/// A signed integer value type.
///
/// This is guaranteed to always be a type-alias to [`isize`]. That means it
/// is valid to use `#[repr(isize)]` on enums and structs with size
/// `NSInteger`.
///
/// See also the [corresponding documentation entry][docs].
///
/// [docs]: https://developer.apple.com/documentation/objectivec/nsinteger?language=objc
///
///
/// # Examples
///
/// ```
/// use core::mem::size_of;
/// use objc2::ffi::NSInteger;
///
/// #[repr(isize)]
/// pub enum NSComparisonResult {
///     NSOrderedAscending = -1,
///     NSOrderedSame = 0,
///     NSOrderedDescending = 1,
/// }
///
/// assert_eq!(size_of::<NSComparisonResult>(), size_of::<NSInteger>());
/// ```
pub type NSInteger = isize;

/// Describes an unsigned integer.
///
/// This is guaranteed to always be a type-alias to [`usize`]. That means it
/// is valid to use `#[repr(usize)]` on enums and structs with size
/// `NSUInteger`.
///
/// See also the [corresponding documentation entry][docs].
///
/// [docs]: https://developer.apple.com/documentation/objectivec/nsuinteger?language=objc
///
///
/// # Examples
///
/// ```
/// use objc2::ffi::NSUInteger;
///
/// extern "C-unwind" {
///     fn some_external_function() -> NSUInteger;
/// }
/// ```
///
/// ```
/// use core::mem::size_of;
/// use objc2::ffi::NSUInteger;
///
/// #[repr(usize)]
/// enum CLRegionState {
///     Unknown = 0,
///     Inside = 1,
///     Outside = 2,
/// }
///
/// assert_eq!(size_of::<CLRegionState>(), size_of::<NSUInteger>());
/// ```
pub type NSUInteger = usize;

/// The maximum value for a [`NSInteger`].
pub const NSIntegerMax: NSInteger = NSInteger::MAX;

/// The minimum value for a [`NSInteger`].
pub const NSIntegerMin: NSInteger = NSInteger::MIN;

/// The maximum value for a [`NSUInteger`].
pub const NSUIntegerMax: NSUInteger = NSUInteger::MAX;
