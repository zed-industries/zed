//! # Objective-C interface and runtime bindings
//!
//! Quick links:
//! - [All Topics][crate::topics].
//! - [All examples].
//! - [About framework crates][crate::topics::about_generated].
//! - [List of framework crates][crate::topics::about_generated::list].
//!
#![doc = concat!("[All examples]: https://github.com/madsmtm/objc2/tree/objc2-", env!("CARGO_PKG_VERSION"), "/examples")]
//!
//! Objective-C was the standard programming language on Apple platforms like
//! macOS, iOS, iPadOS, tvOS and watchOS. It is an object-oriented language
//! centered around "sending messages" to its instances - this can for the
//! most part be viewed as a function call.
//!
//! It has since been superseded by Swift, but most of the core libraries and
//! frameworks that are in use on Apple systems are still written in
//! Objective-C, and hence we would like the ability to interact with these
//! using Rust. This crate enables bi-directional interop with Objective-C, in
//! as safe a manner as possible.
//!
//!
//! ## Example
//!
//! Most of the time, you'll want to use one of [the framework crates], which
//! contain bindings to `CoreFoundation`, `Foundation`, `AppKit`, `Metal`,
//! `UIKit`, `WebKit` and so on.
//!
//! In this example we're going to be using [`objc2-foundation`] and
//! [`objc2-app-kit`] to create a simple GUI application that displays a
//! "Hello World" label.
//!
//! ```console
//! $ # Add the necessary crates to your project.
//! $ cargo add objc2 objc2-foundation objc2-app-kit
//! ```
//!
#![cfg_attr(target_os = "macos", doc = "```no_run")]
#![cfg_attr(not(target_os = "macos"), doc = "```ignore")]
#![doc = include_str!("../examples/hello_world_app.rs")]
//! ```
//!
//! [the framework crates]: crate::topics::about_generated
//! [`objc2-foundation`]: https://docs.rs/objc2-foundation
//! [`objc2-app-kit`]: https://docs.rs/objc2-app-kit
//!
//!
//! ## Crate features
//!
//! This crate exports several optional cargo features, see [`Cargo.toml`] for
//! an overview and description of these.
//!
//! The features in the framework crates are described [here][cr-feat]. Note
//! that if you're developing a library for others to use, you might want to
//! reduce compile times by disabling default features and only enabling the
//! features you need.
//!
#![doc = concat!(
    "[`Cargo.toml`]: https://docs.rs/crate/objc2/",
    env!("CARGO_PKG_VERSION"),
    "/source/Cargo.toml.orig",
)]
//! [cr-feat]: crate::topics::about_generated::cargo_features
//!
//!
//! ## Supported operating systems
//!
//! - macOS: `10.12-15.5`
//! - iOS: `10.0-18.5` (including iPadOS and Mac Catalyst)
//! - tvOS: `10.0-18.5`
//! - watchOS: `5.0-11.5`
//! - visionOS: `1.0-2.5`
//!
//! The minimum versions are the same as those supported by `rustc`. Higher
//! versions will also work, but the framework crates will not have bindings
//! available for newer APIs.
//!
//! The framework bindings are generated from the SDKs in Xcode 16.4. The
//! Xcode version are updated usually within a week of [GitHub Actions]
//! supporting the new Xcode version, and we try to schedule crate releases
//! such that align fairly closely with Xcode updates. We only support stable
//! Xcode versions.
//!
//! Note that the bindings are currently generated in a very macOS-centric
//! manner, so they may try to use types from AppKit, even on iOS, see for
//! example [#637](https://github.com/madsmtm/objc2/issues/637).
//!
//! The bindings _can_ also be used on Linux or *BSD utilizing the
//! [GNUstep Objective-C runtime](https://github.com/gnustep/libobjc2), see
//! the [`ffi`] module for how to configure this, but this is very much
//! second-class.
//!
//! [GitHub actions]: https://github.com/actions/runner-images
//!
//!
//! ## Minimum Supported Rust Version (MSRV)
//!
//! The _currently_ minimum supported Rust version is `1.71` (to be able to
//! use `extern "C-unwind"` functions); this is _not_ defined by policy,
//! though, so it may change in at any time in a patch release.
//!
//! Help us define a policy over in [#203].
//!
//! [#203]: https://github.com/madsmtm/objc2/issues/203

#![no_std]
#![cfg_attr(
    feature = "unstable-autoreleasesafe",
    feature(negative_impls, auto_traits)
)]
#![cfg_attr(
    feature = "unstable-arbitrary-self-types",
    feature(arbitrary_self_types)
)]
#![cfg_attr(
    feature = "unstable-coerce-pointee",
    feature(derive_coerce_pointee, trait_upcasting)
)]
// Note: `doc_notable_trait` doesn't really make sense for us, it's only shown
// for functions returning a specific trait.
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, doc(auto_cfg(hide(feature = "unstable-objfw"))))]
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(clippy::missing_errors_doc)]
#![warn(clippy::missing_panics_doc)]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2/0.6.3")]

#[cfg(not(feature = "alloc"))]
compile_error!("The `alloc` feature currently must be enabled.");

#[cfg(not(feature = "std"))]
compile_error!("The `std` feature currently must be enabled.");

extern crate alloc;
extern crate std;

pub use self::downcast::DowncastTarget;
#[doc(no_inline)]
pub use self::encode::{Encode, Encoding, RefEncode};
pub use self::main_thread_marker::MainThreadMarker;
pub use self::top_level_traits::{
    AnyThread, ClassType, DefinedClass, MainThreadOnly, Message, ProtocolType, ThreadKind,
};

#[cfg(any(feature = "unstable-static-sel", feature = "unstable-static-class"))]
#[doc(hidden)]
pub use objc2_proc_macros::__hash_idents;

#[cfg(not(any(feature = "unstable-static-sel", feature = "unstable-static-class")))]
#[doc(hidden)]
#[macro_export]
macro_rules! __hash_idents {
    // Noop; used to make our other macros a bit easier to read
    ($($x:tt)*) => {
        ()
    };
}

// Note: While this is not public, it is still a breaking change to change,
// since framework crates rely on it.
#[doc(hidden)]
pub mod __framework_prelude;
#[doc(hidden)]
pub mod __macro_helpers;
mod downcast;
pub mod encode;
pub mod exception;
pub mod ffi;
mod macros;
mod main_thread_marker;
pub mod rc;
pub mod runtime;
#[cfg(test)]
mod test_utils;
mod top_level_traits;
#[cfg(any(docsrs, doc, doctest, test))]
pub mod topics;
mod verify;

/// Deprecated location for a few things that are now in the [`runtime`]
/// module.
#[deprecated = "Moved to the `runtime` module"]
pub mod declare {
    use super::runtime;
    pub use super::runtime::{ClassBuilder, ProtocolBuilder};

    /// Use [`runtime::ClassBuilder`] instead.
    #[deprecated = "Use `runtime::ClassBuilder` instead."]
    pub type ClassDecl = runtime::ClassBuilder;

    /// Use [`runtime::ProtocolBuilder`] instead.
    #[deprecated = "Use `runtime::ProtocolBuilder` instead."]
    pub type ProtocolDecl = runtime::ProtocolBuilder;
}

/// Deprecated alias of [`DefinedClass`].
#[deprecated = "renamed to DefinedClass"]
pub use DefinedClass as DeclaredClass;

/// Deprecated alias of [`AnyThread`].
#[deprecated = "renamed to AnyThread"]
pub use AnyThread as AllocAnyThread;

#[cfg(not(feature = "std"))]
compile_error!("The `std` feature currently must be enabled.");

#[cfg(all(
    not(docsrs),
    not(any(
        target_vendor = "apple",
        feature = "unstable-compiler-rt",
        feature = "gnustep-1-7",
        feature = "unstable-objfw",
    ))
))]
compile_error!("`objc2` only works on Apple platforms. Pass `--target aarch64-apple-darwin` or similar to compile for macOS.\n(If you're absolutely certain that you're using GNUStep, you can specify that with the `gnustep-x-y` Cargo feature instead).");

#[cfg(all(feature = "gnustep-1-7", feature = "unstable-objfw"))]
compile_error!("Only one runtime may be selected");

#[cfg(feature = "unstable-objfw")]
compile_error!("ObjFW is not yet supported");

// Link to libobjc
#[cfg_attr(not(feature = "unstable-objfw"), link(name = "objc", kind = "dylib"))]
// Link to libobjfw-rt
#[cfg_attr(feature = "unstable-objfw", link(name = "objfw-rt", kind = "dylib"))]
extern "C" {}

// Link to Foundation to make NSObject and OS version lookup work.
#[cfg_attr(target_vendor = "apple", link(name = "Foundation", kind = "framework"))]
#[cfg_attr(
    all(feature = "gnustep-1-7", not(feature = "unstable-compiler-rt")),
    link(name = "gnustep-base", kind = "dylib")
)]
extern "C" {}
