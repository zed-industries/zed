#[cfg(feature = "copy")]
macro_rules! default {
    (#[inline] $($fn:tt)*) => { #[inline] default $($fn)* };
    ($($fn:tt)*) => { default $($fn)* };
}

#[cfg(not(feature = "copy"))]
macro_rules! default {
    (#[inline] $($fn:tt)*) => { #[inline] $($fn)* };
    ($($fn:tt)*) => { $($fn)* };
}

/// Returns a tuple of `(field_pos, field_out)`, where `field_pos` is the "position",
/// i.e. offset in bytes, of the field relative to the base address of the struct and `field_out`
/// is a `*mut` that points to the field directly.
///
/// This is essentially a convenience wrapper around [`core::ptr::addr_of_mut!`] that also
/// gives back the relative offset of the field, as these are often needed together. You will often
/// see the return values named `(fp, fo)` in internal use of this macro, which stand for `field_pos`
/// and `field_out` respectively as discussed above.
///
/// # Example
///
/// ```
/// use core::mem::MaybeUninit;
/// use rkyv::out_field;
///
/// // The macro works on repr(Rust) structs as well, but for the purposes of asserting
/// // an exact guaranteed position of each field in this example, we'll use repr(C)
/// #[repr(C)]
/// struct Example {
///     a: i32,
///     b: bool,
/// }
///
/// let mut result = MaybeUninit::<Example>::zeroed();
/// let out = result.as_mut_ptr();
///
/// let (a_pos, a_out) = out_field!(out.a);
/// assert_eq!(a_pos, 0); // guaranteed by repr(C) layout, repr(Rust) has free reign
/// unsafe { a_out.write(42); }
///
/// let (b_pos, b_out) = out_field!(out.b);
/// assert_eq!(b_pos, 4); // guaranteed by repr(C) layout, repr(Rust) has free reign
/// unsafe { b_out.write(true); }
///
/// let result = unsafe { result.assume_init() };
/// assert_eq!(result.a, 42);
/// assert_eq!(result.b, true);
/// ```
#[macro_export]
macro_rules! out_field {
    ($out:ident.$field:tt) => {{
        #[allow(unused_unsafe)]
        unsafe {
            let fo = ::core::ptr::addr_of_mut!((*$out).$field);
            (fo.cast::<u8>().offset_from($out.cast::<u8>()) as usize, fo)
        }
    }};
}

/// Returns the unarchived value of the given archived primitive.
///
/// This macro is not needed for most use cases. Its primary purpose is to simultaneously:
/// - Convert values from (potentially) different archived primitives to their native counterparts
/// - Allow transformation in `const` contexts
/// - Prevent linter warnings from unused `into()` calls
///
/// Users should feel free to use the more ergonomic `into()` where appropriate.
#[macro_export]
macro_rules! from_archived {
    ($expr:expr) => {{
        #[cfg(not(any(feature = "archive_le", feature = "archive_be")))]
        {
            $expr
        }
        #[cfg(any(feature = "archive_le", feature = "archive_be"))]
        {
            ($expr).value()
        }
    }};
}

#[cfg(any(feature = "archive_le", feature = "archive_be"))]
pub use crate::rend::NativeEndian;

/// Returns the archived value of the given archived primitive.
///
/// This macro is not needed for most use cases. Its primary purpose is to simultaneously:
/// - Convert values from (potentially) different primitives to their archived counterparts
/// - Allow transformation in `const` contexts
/// - Prevent linter warnings from unused `into()` calls
///
/// Users should feel free to use the more ergonomic `into()` where appropriate.
#[macro_export]
macro_rules! to_archived {
    ($expr:expr) => {{
        #[cfg(not(any(feature = "archive_le", feature = "archive_be")))]
        {
            $expr
        }
        #[cfg(feature = "archive_le")]
        {
            $crate::macros::NativeEndian { value: $expr }.to_le()
        }
        #[cfg(feature = "archive_be")]
        {
            $crate::macros::NativeEndian { value: $expr }.to_be()
        }
    }};
}

#[cfg(not(any(feature = "size_16", feature = "size_32", feature = "size_64")))]
core::compile_error!(r#"one of ["size_16", "size_32", or "size_64"] features must be enabled"#);

#[cfg(all(feature = "size_16", feature = "size_32"))]
core::compile_error!(
    "\"size_16\" and \"size_32\" are mutually-exclusive features. You may need to set \
    `default-features = false` or compile with `--no-default-features`."
);
#[cfg(all(feature = "size_16", feature = "size_64"))]
core::compile_error!(
    "\"size_16\" and \"size_64\" are mutually-exclusive features. You may need to set \
    `default-features = false` or compile with `--no-default-features`."
);
#[cfg(all(feature = "size_32", feature = "size_64"))]
core::compile_error!(
    "\"size_32\" and \"size_64\" are mutually-exclusive features. You may need to set \
    `default-features = false` or compile with `--no-default-features`."
);

#[cfg(feature = "size_16")]
macro_rules! pick_size_type {
    ($s16:ty, $s32:ty, $s64:ty) => {
        $s16
    };
    ($s16:ty, $s32:ty, $s64:ty,) => {
        pick_size_type!($s16, $s32, $s64)
    };
}

#[cfg(feature = "size_32")]
macro_rules! pick_size_type {
    ($s16:ty, $s32:ty, $s64:ty) => {
        $s32
    };
    ($s16:ty, $s32:ty, $s64:ty,) => {
        pick_size_type!($s16, $s32, $s64)
    };
}

#[cfg(feature = "size_64")]
macro_rules! pick_size_type {
    ($s16:ty, $s32:ty, $s64:ty) => {
        $s64
    };
    ($s16:ty, $s32:ty, $s64:ty,) => {
        pick_size_type!($s16, $s32, $s64)
    };
}
