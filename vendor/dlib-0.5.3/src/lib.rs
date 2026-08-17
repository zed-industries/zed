//! dlib is a small crate providing macros to make easy the use of external system libraries that
//! can or cannot be optionally loaded at runtime, depending on whether a certain feature is enabled.
//!
//! ## Usage
//!
//! dlib defines the `external_library!` macro, which can be invoked in this way:
//!
//! ```rust
//! # use dlib::external_library;
//! # use std::ffi::{c_float, c_int};
//! external_library!(feature="dlopen-foo", Foo, "foo",
//!     statics:
//!         me: c_int,
//!         you: c_float,
//!     functions:
//!         fn foo() -> c_int,
//!         fn bar(c_int, c_float) -> (),
//!         fn baz(*const c_int) -> c_int,
//!     varargs:
//!         fn blah(c_int, c_int ...) -> *const c_void,
//!         fn bleh(c_int ...) -> (),
//! );
//! ```
//!
//! As you can see, it is required to separate static values from functions and from function
//! having variadic arguments. Each of these 3 categories is optional, but the ones used must appear
//! in this order. Return types of the functions must all be explicit (hence `-> ()` for void functions).
//!
//! If the feature named by the `feature` argument (in this example, `dlopen-foo`) is absent on your crate,
//! this macro will expand to an extern block defining each of the items, using the third argument
//! of the macro as a link name:
//!
//! ```rust no_run
//! # use std::ffi::{c_float, c_int, c_void};
//! #[link(name = "foo")]
//! extern "C" {
//!     pub static me: c_int;
//!     pub static you: c_float;
//!     pub fn foo() -> c_int;
//!     pub fn bar(_: c_int, _: c_float) -> ();
//!     pub fn baz(_: *const c_int) -> c_int;
//!     pub fn blah(_: c_int, _: c_int, ...) -> *const c_void;
//!     pub fn bleh(_: c_int, ...) -> ();
//! }
//!
//! ```
//!
//! If the feature named by the `feature` argument is present on your crate, it will expand to a
//! `struct` named by the second argument of the macro, with one field for each of the symbols defined;
//! and a method `open`, which tries to load the library from the name or path given as an argument.
//!
//! ```rust
//! # use dlib::DlError;
//! # use std::ffi::{c_float, c_int, c_void};
//! pub struct Foo {
//!     pub me: &'static c_int,
//!     pub you: &'static c_float,
//!     pub foo: unsafe extern "C" fn() -> c_int,
//!     pub bar: unsafe extern "C" fn(c_int, c_float) -> (),
//!     pub baz: unsafe extern "C" fn(*const c_int) -> c_int,
//!     pub blah: unsafe extern "C" fn(c_int, c_int, ...) -> *const c_void,
//!     pub bleh: unsafe extern "C" fn(c_int, ...) -> (),
//! }
//!
//!
//! impl Foo {
//!     pub unsafe fn open(name: &str) -> Result<Foo, DlError> {
//!         /* ... */
//!         # todo!()
//!     }
//! }
//! ```
//!
//! This method returns `Ok(..)` if the loading was successful. It contains an instance of the defined struct
//! with all of its fields pointing to the appropriate symbol.
//!
//! If the library specified by `name` could not be openened, it returns `Err(DlError::CantOpen(e))`, with
//! `e` the error reported by `libloading` (see [LibLoadingError]);
//!
//! It will also fail on the first missing symbol, with `Err(DlError::MissingSymbol(symb))` where `symb`
//! is a `&str` containing the missing symbol name.
//!
//! Note that this method is unsafe, as loading (and unloading on drop) an external C library can run arbitrary
//! code. As such, you need to ensure that the specific library you want to load is safe to load in the context
//! you want to load it.
//!
//! ## Remaining generic in your crate
//!
//! If you want your crate to remain generic over dlopen vs. linking, simply add a feature to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! dlib = "0.5"
//!
//! [features]
//! dlopen-foo = []
//! ```
//!
//! Then give the name of that feature as the `feature` argument to dlib's macros:
//!
//! ```rust no_run
//! # use dlib::external_library;
//! # use std::ffi::c_int;
//! external_library!(feature="dlopen-foo", Foo, "foo",
//!     functions:
//!         fn foo() -> c_int,
//! );
//! ```
//!
//! `dlib` provides helper macros to dispatch the access to foreign symbols:
//!
//! ```rust no_run
//! # use dlib::{ffi_dispatch, ffi_dispatch_static};
//! # let arg1 = todo!();
//! # let arg2 = todo!();
//! # let function: fn(u32, u32) = todo!();
//! # let my_static_var = todo!();
//! ffi_dispatch!(feature="dlopen-foo", Foo, function, arg1, arg2);
//! ffi_dispatch_static!(feature="dlopen-foo", Foo, my_static_var);
//! ```
//!
//! These will expand to the appropriate value or function call depending on the presence or absence of the
//! `dlopen-foo` feature on your crate.
//!
//! You must still ensure that the functions/statics or the wrapper struct `Foo` are in scope. For example,
//! you could use the [`lazy_static`](https://crates.io/crates/lazy_static) crate to do the initialization,
//! and store the wrapper struct in a static variable that you import wherever needed:
//!
//! ```rust
//! #[cfg(feature = "dlopen-foo")]
//! lazy_static::lazy_static! {
//!     pub static ref FOO_STATIC: Foo =
//!         Foo::open("libfoo.so").ok().expect("could not find libfoo");
//! }
//! ```
//!
//! Then, it can become as simple as putting this on top of all modules using the FFI:
//!
//! ```rust
//! # #![allow(unexpected_cfgs)]
//! # mod ffi {}
//! #[cfg(feature = "dlopen-foo")]
//! use ffi::FOO_STATIC;
//! #[cfg(not(feature = "dlopen-foo"))]
//! use ffi::*;
//! ```
#![warn(missing_docs)]

extern crate libloading;

pub use libloading::Error as LibLoadingError;
#[doc(hidden)]
pub use libloading::{Library, Symbol};

/// Macro for generically invoking a FFI function
///
/// The expected arguments are, in order:
/// - (Optional) The name of the cargo feature conditioning the usage of dlopen, in the form
///   `feature="feature-name"`. If ommited, the feature `"dlopen"` will be used.
/// - A value of the handle generated by the macro [`external_library!`] when the
///   dlopen-controlling feature is enabled
/// - The name of the function to invoke
/// - The arguments to be passed to the function
///
/// The macro invocation evaluates to the return value of the FFI function.
///
/// #### Example
///
/// Assuming an FFI function of signature `fn(u32, u32) -> u32`:
///
/// ```rust,ignore
/// let sum = unsafe { ffi_dispatch!(feature="dlopen", LIBRARY_HANDLE, sum, 2, 2) };
/// ```
#[macro_export]
macro_rules! ffi_dispatch(
    (feature=$feature: expr, $handle: expr, $func: ident, $($arg: expr),*) => (
        {
            #[cfg(feature = $feature)]
            let ret = ($handle.$func)($($arg),*);
            #[cfg(not(feature = $feature))]
            let ret = $func($($arg),*);

            ret
        }
    );
    ($handle: expr, $func: ident, $($arg: expr),*) => (
        // NOTE: this "dlopen" refers to a feature on the crate *using* dlib
        $crate::ffi_dispatch!(feature="dlopen", $handle, $func, $($arg),*)
    );
);

/// Macro for generically accessing a FFI static
///
/// The expected arguments are, in order:
/// - (Optional) The name of the cargo feature conditioning the usage of dlopen, in the form
///   `feature="feature-name"`. If ommited, the feature `"dlopen"` will be used.
/// - A value of the handle generated by the macro [`external_library!`] when the
///   dlopen-controlling feature is enabled
/// - The name of the static
///
/// The macro invocation evaluates to a `&T` reference to the static
///
/// #### Example
///
/// ```rust,ignore
/// let my_static = unsafe { ffi_dispatch!(feature="dlopen", LIBRARY_HANDLE, my_static) };
/// ```
#[macro_export]
macro_rules! ffi_dispatch_static(
    (feature=$feature: expr, $handle: expr, $name: ident) => (
        {
            #[cfg(feature = $feature)]
            let ret = $handle.$name;
            #[cfg(not(feature = $feature))]
            let ret = &$name;

            ret
        }
    );
    ($handle:expr, $name: ident) => (
        $crate::ffi_dispatch_static!(feature="dlopen", $handle, $name)
    );
);

#[doc(hidden)]
#[macro_export]
macro_rules! link_external_library(
    ($link: expr,
        $(statics: $($(#[$sattr:meta])* $sname: ident: $stype: ty),+,)|*
        $(functions: $($(#[$fattr:meta])* fn $fname: ident($($farg: ty),*) -> $fret:ty),+,)|*
        $(varargs: $($(#[$vattr:meta])* fn $vname: ident($($vargs: ty),+) -> $vret: ty),+,)|*
    ) => (
        #[link(name = $link)]
        extern "C" {
            $($(
                $(#[$sattr])*
                pub static $sname: $stype;
            )+)*
            $($(
                $(#[$fattr])*
                pub fn $fname($(_: $farg),*) -> $fret;
            )+)*
            $($(
                $(#[$vattr])*
                pub fn $vname($(_: $vargs),+ , ...) -> $vret;
            )+)*
        }
    );
);

/// An error generated when failing to load a library
#[derive(Debug)]
pub enum DlError {
    /// The requested library would not be opened
    ///
    /// Includes the error reported by `libloading` when trying to
    /// open the library.
    CantOpen(LibLoadingError),
    /// Some required symbol was missing in the library
    MissingSymbol(&'static str),
}

impl std::error::Error for DlError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            DlError::CantOpen(ref e) => Some(e),
            DlError::MissingSymbol(_) => None,
        }
    }
}

impl std::fmt::Display for DlError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            DlError::CantOpen(ref e) => write!(f, "Could not open the requested library: {}", e),
            DlError::MissingSymbol(s) => write!(f, "The requested symbol was missing: {}", s),
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! dlopen_external_library(
    (__struct, $structname: ident,
        $(statics: $($(#[$sattr:meta])* $sname: ident: $stype: ty),+,)|*
        $(functions: $($(#[$fattr:meta])* fn $fname: ident($($farg: ty),*) -> $fret:ty),+,)|*
        $(varargs: $($(#[$vattr:meta])* fn $vname: ident($($vargs: ty),+) -> $vret: ty),+,)|*
    ) => (
        pub struct $structname {
            __lib: $crate::Library,
            $($(
                $(#[$sattr])*
                pub $sname: $crate::Symbol<'static, &'static $stype>,
            )+)*
            $($(
                $(#[$fattr])*
                pub $fname: $crate::Symbol<'static, unsafe extern "C" fn($($farg),*) -> $fret>,
            )+)*
            $($(
                $(#[$vattr])*
                pub $vname: $crate::Symbol<'static, unsafe extern "C" fn($($vargs),+ , ...) -> $vret>,
            )+)*
        }
    );
    (__impl, $structname: ident,
        $(statics: $($(#[$sattr:meta])* $sname: ident: $stype: ty),+,)|*
        $(functions: $($(#[$fattr:meta])* fn $fname: ident($($farg: ty),*) -> $fret:ty),+,)|*
        $(varargs: $($(#[$vattr:meta])* fn $vname: ident($($vargs: ty),+) -> $vret: ty),+,)|*
    ) => (
    impl $structname {
        pub unsafe fn open(name: &str) -> Result<$structname, $crate::DlError> {
            // we use it to ensure the 'static lifetime
            use std::mem::transmute;
            let lib = $crate::Library::new(name).map_err($crate::DlError::CantOpen)?;
            let s = $structname {
                $($($(#[$sattr])* $sname: {
                    let s_name = concat!(stringify!($sname), "\0");
                    transmute(match lib.get::<&'static $stype>(s_name.as_bytes()) {
                        Ok(s) => s,
                        Err(_) => return Err($crate::DlError::MissingSymbol(s_name))
                    })
                },
                )+)*
                $($($(#[$fattr])* $fname: {
                    let s_name = concat!(stringify!($fname), "\0");
                    transmute(match lib.get::<unsafe extern "C" fn($($farg),*) -> $fret>(s_name.as_bytes()) {
                        Ok(s) => s,
                        Err(_) => return Err($crate::DlError::MissingSymbol(s_name))
                    })
                },
                )+)*
                $($($(#[$vattr])* $vname: {
                    let s_name = concat!(stringify!($vname), "\0");
                    transmute(match lib.get::<unsafe extern "C" fn($($vargs),+ , ...) -> $vret>(s_name.as_bytes()) {
                        Ok(s) => s,
                        Err(_) => return Err($crate::DlError::MissingSymbol(s_name))
                    })
                },
                )+)*
                __lib: lib
            };
            Ok(s)
        }
    }
    );
    ($structname: ident,
        $(statics: $($(#[$sattr:meta])* $sname: ident: $stype: ty),+,)|*
        $(functions: $($(#[$fattr:meta])* fn $fname: ident($($farg: ty),*) -> $fret:ty),+,)|*
        $(varargs: $($(#[$vattr:meta])* fn $vname: ident($($vargs: ty),+) -> $vret: ty),+,)|*
    ) => (
        $crate::dlopen_external_library!(__struct,
            $structname, $(statics: $($(#[$sattr])* $sname: $stype),+,)|*
            $(functions: $($(#[$fattr])* fn $fname($($farg),*) -> $fret),+,)|*
            $(varargs: $($(#[$vattr])* fn $vname($($vargs),+) -> $vret),+,)|*
        );
        $crate::dlopen_external_library!(__impl,
            $structname, $(statics: $($(#[$sattr])* $sname: $stype),+,)|*
            $(functions: $($(#[$fattr])* fn $fname($($farg),*) -> $fret),+,)|*
            $(varargs: $($(#[$vattr])* fn $vname($($vargs),+) -> $vret),+,)|*
        );
        unsafe impl Sync for $structname { }
    );
);

/// Main macro of this library, used to generate the the FFI bindings.
///
/// The expected arguments are, in order:
/// - (Optional) The name of the cargo feature conditioning the usage of dlopen, in the form
///   `feature="feature-name"`. If ommited, the feature `"dlopen"` will be used.
/// - The name of the struct that will be generated when the dlopen-controlling feature is
///   enabled
/// - The link name of the target library
/// - The desctription of the statics, functions, and vararg functions that should be linked
///
/// See crate-level documentation for a detailed example of use.
#[macro_export]
macro_rules! external_library(
    (feature=$feature: expr, $structname: ident, $link: expr,
        $(statics: $($(#[$sattr:meta])* $sname: ident: $stype: ty),+,)|*
        $(functions: $($(#[$fattr:meta])* fn $fname: ident($($farg: ty),*) -> $fret:ty),+,)|*
        $(varargs: $($(#[$vattr:meta])* fn $vname: ident($($vargs: ty),+) -> $vret: ty),+,)|*
    ) => (
        #[cfg(feature = $feature)]
        $crate::dlopen_external_library!(
            $structname, $(statics: $($(#[$sattr])* $sname: $stype),+,)|*
            $(functions: $($(#[$fattr])* fn $fname($($farg),*) -> $fret),+,)|*
            $(varargs: $($(#[$vattr])* fn $vname($($vargs),+) -> $vret),+,)|*
        );

        #[cfg(not(feature = $feature))]
        $crate::link_external_library!(
            $link, $(statics: $($(#[$sattr])* $sname: $stype),+,)|*
            $(functions: $($(#[$fattr])* fn $fname($($farg),*) -> $fret),+,)|*
            $(varargs: $($(#[$vattr])* fn $vname($($vargs),+) -> $vret),+,)|*
        );
    );
    ($structname: ident, $link: expr,
        $(statics: $($(#[$sattr:meta])* $sname: ident: $stype: ty),+,)|*
        $(functions: $($(#[$fattr:meta])* fn $fname: ident($($farg: ty),*) -> $fret:ty),+,)|*
        $(varargs: $($(#[$vattr:meta])* fn $vname: ident($($vargs: ty),+) -> $vret: ty),+,)|*
    ) => (
        $crate::external_library!(
            feature="dlopen", $structname, $link,
            $(statics: $($(#[$sattr])* $sname: $stype),+,)|*
            $(functions: $($(#[$fattr])* fn $fname($($farg),*) -> $fret),+,)|*
            $(varargs: $($(#[$vattr])* fn $vname($($vargs),+) -> $vret),+,)|*
        );
    );
);
