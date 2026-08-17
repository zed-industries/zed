//! A testing library that makes it easy(ish) to add intentional errors to a program
//!
//! When testing error-handling codepaths, it is often useful to programmatically tell parts of the
//! code to fail. This crate provides the [`failspot!()`][failspot] macro, which can be used
//! to mark a spot in the codepath where an intentional failure can be toggled on and off
//! from testing code.
//!
//! Adding it to code is fairly simple:
//!
//! ```no_run
//! # use {failspot::failspot, std::{error::Error, fs, io, path::Path}};
//! # failspot::failspot_name! { pub enum FailSpotName { FailDataRead } }
//! # fn main() {
//! #     read_data_file("/tmp/foo".as_ref());
//! # }
//! fn read_data_file(path: &Path) -> Result<Vec<u8>, Box<dyn Error>> {
//!     // `FailDataRead` is a variant of an enum that was declared in our crate
//!     // `bail` returns `Err` and does type conversion on the error type
//!     failspot!(FailDataRead bail(io::Error::other("failed due to test config")));
//!     Ok(fs::read(path)?)
//! }
//! ```
//!
//! The [`failspot_name!()`][failspot_name] macro is used to declare an enum that can be used
//! to name a failspot. Its syntax is identical to a regular enum declaration:
//!
//! ```no_run
//! # use failspot::failspot_name;
//! failspot_name! {
//!     pub enum FailSpotName {
//!         StuffWentWrong,
//!     }
//! }
//! ```
//!
//! # Syntaxes
//!
//! The [`failspot!()`][failspot] macro has **four main syntaxes**. Each takes either a
//! **long form** or a **short form**.
//!
//! The **long form** explicitly contains the path to the enum containing the failspot names. This
//! is useful if there are multiple enums with different failspot names, or if the enum is only
//! accessible at a non-standard name or path.
//!
//! If the enum can be reached at the name `crate::FailSpotName` (either because it was declared in
//! the crate root or re-exported there), the **short form** versions of these macros
//! can be used. The examples below make this clear:
//!
//! ## "if-else" syntax (long form)
//!
//! Useful when standard if-else behavior is desired:
//!
//! ```
//! # use {failspot::failspot};
//! # pub mod my_module {
//! #     failspot::failspot_name! { pub enum MyFailName { FailDataRead } }
//! # }
//! # fn main() {
//! // In "long form", the entire path to the enum must be spelled out
//! let _enabled = failspot!(if <crate::my_module::MyFailName>::FailDataRead {
//!     println!("Data read failure enabled");
//!     true
//! } else {
//!     println!("Data read failure disabled");
//!     false
//! });
//! # }
//! ```
//!
//! When the `enabled` feature is not on, the compiler will see the second block verbatim.
//!
//! ### The same code with the short-form
//!
//! If an enum named `FailSpotName` is reachable at the crate root, like this:
//!
//! ```
//! # use failspot::failspot_name;
//! // lib.rs
//! failspot_name! {
//!     pub enum FailSpotName {
//!         FailDataRead,
//!     }
//! }
//! ```
//!
//! The short form can be used, like this:
//!
//! ```
//! # use failspot::{failspot, failspot_name};
//! # failspot_name! { pub enum FailSpotName { FailDataRead } }
//! # fn main() {
//! let _enabled = failspot!(if FailDataRead {
//!     println!("Data read failure enabled");
//!     true
//! } else {
//!     println!("Data read failure disabled");
//!     false
//! });
//! # }
//! ```
//!
//! ## "quick expression" syntax
//!
//! Useful when a short block of syntax should be evaluated when the failspot is enabled:
//!
//! ```
//! # use {failspot::failspot};
//! # failspot::failspot_name! { pub enum FailSpotName { FailDataRead } }
//! # fn main() {
//! failspot!(FailDataRead println!("Data read failure enabled"));
//!
//! // Also good for panicking
//! failspot!(FailDataRead panic!());
//!
//! // Or for just early returning
//! failspot!(FailDataRead return);
//!
//! // Multiple statements can be run, but use sparingly as things start to get ugly.
//! failspot!(FailDataRead println!("Data read failure enabled"); return);
//! # }
//! ```
//!
//! When the `enabled` feature is not on, the macro will evaluate to the empty block, `{}`.
//!
//! ## "bail" syntax
//!
//! Useful for returning an `Err` with error-type conversion:
//!
//! ```
//! # use {failspot::failspot, std::error::Error};
//! # failspot::failspot_name! { pub enum FailSpotName { FailDataRead } }
//! fn main() -> Result<(), Box<dyn Error>>  {
//!     failspot!(FailDataRead bail(std::io::Error::other("Data read failure enabled")));
//!     Ok(())
//! }
//! ```
//!
//! When the `enabled` feature is not on, the macro will evaluate to the empty block, `{}`.
//!
//! ## "bool" syntax
//!
//! Useful to just evaluate whether the failspot is enabled or not:
//!
//! ```
//! # use {failspot::failspot, std::error::Error};
//! # failspot::failspot_name! { pub enum FailSpotName { FailDataRead } }
//! # fn main() {
//! # let bytes_to_read = 5;
//! let fail_the_read = bytes_to_read > 5000 || failspot!(FailDataRead);
//! # }
//! ```
//!
//! When the `enabled` feature is not on, the macro will evaluate to the token `false`.

#![cfg_attr(
    feature = "enabled",
    doc = r#"
# Testing code

Testing code should see the documentation for the [testing] module."#
)]
#![forbid(missing_docs)]

#[cfg(feature = "enabled")]
pub mod testing;

/// Declares a spot that can trigger intentional failures
///
/// See the [crate-level docs][crate] for details.
#[cfg(feature = "enabled")]
#[allow(clippy::crate_in_macro_def)]
#[macro_export]
macro_rules! failspot {
(
    if <$e: ty>::$n: ident {
        $($enabled: tt)*
    } $(else {
        $($disabled: tt)*
    })?
) => {{
    if $crate::failspot!(<$e>::$n) {
        $($enabled)*
    } $(else {
        $($disabled)*
    })?
}};

(
    if $n: ident {
        $($enabled: tt)*
    } $(else {
        $($disabled: tt)*
    })?
) => {
    $crate::failspot!(
        if <crate::FailSpotName>::$n {
            $($enabled)*
        } $(else {
            $($disabled)*
        })?
    )
};

(<$e: ty>::$n: ident bail($err: expr)) => {{
    if $crate::failspot!(<$e>::$n) {
        return Err($err.into());
    }
}};
($n: ident bail($err: expr)) => {
    $crate::failspot!(<crate::FailSpotName>::$n bail($err))
};
(<$e: ty>::$n: ident) => {{
    <$e>::enabled(<$e>::$n)
}};
($n: ident) => {
    $crate::failspot!(<crate::FailSpotName>::$n)
};
(<$e: ty>::$n: ident $($enabled: tt)+) => {{
    if $crate::failspot!(<$e>::$n) {
        $($enabled)+
    }
}};
($n: ident $($enabled: tt)+) => {
    $crate::failspot!(<crate::FailSpotName>::$n $($enabled)+)
}}

/// Declares a spot that can trigger intentional failures
///
/// See the [crate-level docs][crate] for details.
#[cfg(not(feature = "enabled"))]
#[allow(clippy::crate_in_macro_def)]
#[macro_export]
macro_rules! failspot {
(
    if <$e: ty>::$n: ident {
        $($enabled: tt)*
    } $(else {
        $($disabled: tt)*
    })?
) => {{$($($disabled)*)?}};

(
    if $n: ident {
        $($enabled: tt)*
    } $(else {
        $($disabled: tt)*
    })?
) => {
    $crate::failspot!(
        if <crate::FailSpotName>::$n {
            $($enabled)*
        } $(else {
            $($disabled)*
        })?
    )
};
(<$e: ty>::$n: ident bail($err: expr)) => {{}};
($n: ident bail($err: expr)) => {
    $crate::failspot!(<crate::FailSpotName>::$n bail($err))
};
(<$e: ty>::$n: ident) => {false};
($n: ident) => {
    $crate::failspot!(<crate::FailSpotName>::$n)
};
(<$e: ty>::$n: ident $($enabled: tt)+) => {{}};
($n: ident $($enabled: tt)+) => {
    $crate::failspot!(<crate::FailSpotName>::$n $($enabled)+)
}}

/// Declares an enum that can be used as a name for a [failspot]
///
/// When feature `enabled` is off, this macro does nothing.
///
/// See the [crate-level docs][crate] for details.
#[cfg(feature = "enabled")]
#[macro_export]
macro_rules! failspot_name {{
    $(#[$m:meta])*
    $p:vis enum $n:ident {
    $(
        $(#[$a:meta])*
        $k:ident
    ),+ $(,)*
    }
} => {
    $crate::flagset::flags! {
        $(#[$m])*
        $p enum $n: usize {
        $(
            $(#[$a])*
            $k
        ),+
        }
    }
    $crate::failspot_global!($n);
}}

/// Declares an enum that can be used as a name for a [failspot]
///
/// When feature `enabled` is off, this macro does nothing.
///
/// See the [crate-level docs][crate] for details.
#[cfg(not(feature = "enabled"))]
#[macro_export]
macro_rules! failspot_name(($($t: tt)*) => ());

#[doc(hidden)]
#[macro_export]
macro_rules! failspot_global(($n: ident) => {
    impl $n {
        pub fn enabled(name: Self) -> bool {
            Self::global_config().enabled(name)
        }
        pub fn testing_client() -> $crate::testing::Client<'static, $n> {
            Self::global_config().client()
        }
        fn global_config() -> &'static $crate::testing::Config<$n> {
            static GLOBAL: std::sync::LazyLock<$crate::testing::Config<$n>> =
                std::sync::LazyLock::new($crate::testing::Config::default);
            &GLOBAL
        }
    }
});

#[cfg(feature = "enabled")]
#[doc(hidden)]
pub use flagset;
