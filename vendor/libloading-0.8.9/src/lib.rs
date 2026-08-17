//! Bindings around the platform's dynamic library loading primitives with greatly improved memory safety.
//!
//! Using this library allows the loading of [dynamic libraries](struct.Library.html), also known as
//! shared libraries, and the use of the functions and static variables they contain.
//!
//! The `libloading` crate exposes a cross-platform interface to load a library and make use of its
//! contents, but little is done to hide the differences in behaviour between platforms.
//! The API documentation strives to document such differences as much as possible.
//!
//! Platform-specific APIs are also available in the [`os`](crate::os) module. These APIs are more
//! flexible, but less safe.
//!
//! # Installation
//!
//! Add the `libloading` library to your dependencies in `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! libloading = "0.8"
//! ```
//!
//! # Usage
//!
//! In your code, run the following:
//!
//! ```no_run
//! fn call_dynamic() -> Result<u32, Box<dyn std::error::Error>> {
//!     unsafe {
//!         let lib = libloading::Library::new("/path/to/liblibrary.so")?;
//!         let func: libloading::Symbol<unsafe extern fn() -> u32> = lib.get(b"my_func")?;
//!         Ok(func())
//!     }
//! }
//! ```
//!
//! The compiler will ensure that the loaded function will not outlive the `Library` from which it comes,
//! preventing the most common memory-safety issues.
#![cfg_attr(
    any(unix, windows),
    deny(missing_docs, clippy::all, unreachable_pub, unused)
)]
#![cfg_attr(libloading_docs, feature(doc_cfg))]

pub mod changelog;
mod error;
pub mod os;
#[cfg(any(unix, windows, libloading_docs))]
mod safe;
mod util;

pub use self::error::Error;
#[cfg(any(unix, windows, libloading_docs))]
pub use self::safe::{Library, Symbol};
use std::env::consts::{DLL_PREFIX, DLL_SUFFIX};
use std::ffi::{OsStr, OsString};

/// Converts a library name to a filename generally appropriate for use on the system.
///
/// This function will prepend prefixes (such as `lib`) and suffixes (such as `.so`) to the library
/// `name` to construct the filename.
///
/// # Examples
///
/// It can be used to load global libraries in a platform independent manner:
///
/// ```
/// use libloading::{Library, library_filename};
/// // Will attempt to load `libLLVM.so` on Linux, `libLLVM.dylib` on macOS and `LLVM.dll` on
/// // Windows.
/// let library = unsafe {
///     Library::new(library_filename("LLVM"))
/// };
/// ```
pub fn library_filename<S: AsRef<OsStr>>(name: S) -> OsString {
    let name = name.as_ref();
    let mut string = OsString::with_capacity(name.len() + DLL_PREFIX.len() + DLL_SUFFIX.len());
    string.push(DLL_PREFIX);
    string.push(name);
    string.push(DLL_SUFFIX);
    string
}
