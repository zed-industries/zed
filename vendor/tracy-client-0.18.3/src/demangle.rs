//! Custom symbol demangling support.
//!
//! By default, Tracy demangles symbols using the C++ ABI, which is not fully compatible with Rust
//! symbol mangling.
//!
//! With the `demangle` feature enabled, clients must register a custom demangling function.
//! This can be done by calling the [`register_demangler!`][macro] macro with either no arguments
//! to use the [default demangler](default), or with a path to a custom demangler function. See
//! [its documentation][macro] for how to use it.
//!
//! Note that only one demangler can be registered at a time. Attempting to register multiple
//! demanglers will result in a linking failure due to multiple definitions of the underlying
//! `extern "C"` function.
//!
//! [macro]: crate::register_demangler

use std::fmt;

/// Opaque buffer used to write demangled symbols.
///
/// The only exposed API is currently [`fmt::Write`].
///
/// See [the module-level documentation](self) for more information.
pub struct Buffer(String);

impl fmt::Write for Buffer {
    #[inline]
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.0.write_str(s)
    }

    #[inline]
    fn write_char(&mut self, c: char) -> fmt::Result {
        self.0.write_char(c)
    }
}

impl Buffer {
    const fn new() -> Self {
        Self(String::new())
    }

    fn clear_on_err<T, E>(&mut self, f: impl FnOnce(&mut Self) -> Result<T, E>) -> Result<T, E> {
        let r = f(self);
        if r.is_err() {
            self.0.clear();
        }
        r
    }
}

/// Demangles a Rust symbol using [`rustc_demangle`].
///
/// See [the module-level documentation](self) for more information.
pub fn default(s: &str, buffer: &mut impl fmt::Write) -> fmt::Result {
    let Ok(demangled) = rustc_demangle::try_demangle(s) else {
        return Err(fmt::Error);
    };
    // Use `:#` formatting to elide the hash.
    write!(buffer, "{demangled:#}")
}

/// Symbol demangler that does nothing.
///
/// See [the module-level documentation](self) for more information.
pub fn noop(_: &str, _: &mut impl fmt::Write) -> fmt::Result {
    Err(fmt::Error)
}

pub(super) mod internal {
    use super::Buffer;
    use std::ffi::c_char;
    use std::fmt::{self, Write};
    use std::ptr::null;

    /// Demangling glue.
    pub unsafe fn implementation<F>(mangled: *const c_char, run: F) -> *const c_char
    where
        F: FnOnce(&str, &mut Buffer) -> fmt::Result,
    {
        // https://github.com/wolfpld/tracy/blob/d4a4b623968d99a7403cd93bae5247ed0735680a/public/client/TracyCallstack.cpp#L57-L67
        // > The demangling function is responsible for managing memory for this string.
        // > It is expected that it will be internally reused.
        // > When a call to ___tracy_demangle is made, previous contents of the string memory
        // > do not need to be preserved.
        static mut BUFFER: Buffer = Buffer::new();

        if mangled.is_null() {
            return null();
        }
        let cstr = unsafe { std::ffi::CStr::from_ptr(mangled) };
        let Ok(str) = cstr.to_str() else {
            return null();
        };

        let buffer = unsafe { &mut *std::ptr::addr_of_mut!(BUFFER) };
        buffer.0.clear();
        let result = buffer.clear_on_err(|buffer| {
            run(str, buffer)?;
            match buffer.0.as_bytes().split_last() {
                None | Some((&0, [])) => return Err(fmt::Error),
                Some((_, v)) if v.contains(&0) => return Err(fmt::Error),
                Some((&0, _)) => return Ok(()),
                _ => (),
            }
            buffer.write_char('\0')?;
            Ok(())
        });
        match result {
            Ok(()) => {
                debug_assert_eq!(buffer.0.as_bytes().last().copied(), Some(0));
                buffer.0.as_ptr().cast()
            }
            Err(fmt::Error) => null(),
        }
    }
}

/// Registers a custom demangler function.
///
/// A [default implementation](default) for demangling Rust symbols can be registered by passing no
/// arguments.
///
/// Custom implementations can be registered by passing a function with the following signature:
/// `fn(mangled: &str, buffer: &mut tracy_client::demangle::Buffer) -> std::fmt::Result`
///
/// Custom demanglers:
/// - Must not write null bytes to the buffer.
/// - Returning `Err` or leaving the buffer unchanged will result in the symbol being displayed as-is.
///
/// See [the module-level documentation](self) for more information.
///
/// # Examples
///
/// ```
/// use tracy_client::{demangle, register_demangler};
///
/// // Register the default demangler.
/// # #[cfg(any())]
/// register_demangler!();
///
/// // Register a noop demangler.
/// # #[cfg(any())]
/// register_demangler!(demangle::noop);
///
/// // Register a custom demangler.
/// # #[cfg(any())]
/// register_demangler!(my_demangler);
///
/// fn my_demangler(s: &str, buffer: &mut demangle::Buffer) -> std::fmt::Result {
///     // Custom demangling logic...
///     use std::fmt::Write;
///     write!(buffer, "{s}")?;
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! register_demangler {
    () => {
        $crate::register_demangler!($crate::internal::demangle::default);
    };

    ($path:path) => {
        const _: () = {
            #[no_mangle]
            unsafe extern "C" fn ___tracy_demangle(
                mangled: *const std::ffi::c_char,
            ) -> *const std::ffi::c_char {
                unsafe { $crate::internal::demangle::implementation(mangled, $path) }
            }
        };
    };
}
