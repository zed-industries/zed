//! # Objective-C interface and runtime bindings
//!
//! Quick links:
//! - [All Topics][crate::topics].
//! - [About framework crates][crate::topics::about_generated].
//! - [List of framework crates][crate::topics::about_generated::list].
//!
//! Objective-C was the standard programming language on Apple platforms like
//! macOS, iOS, iPadOS, tvOS and watchOS. It is an object-oriented language
//! centered around "sending messages" to its instances - this can for the
//! most part be viewed as a function call.
//!
//! It has since been superseded by Swift, but most of the core libraries and
//! frameworks that are in use on Apple systems are still written in
//! Objective-C, and hence we would like the ability to interract with these
//! using Rust. This crate enables you to do that, in as safe a manner as
//! possible.
//!
//!
//! ## Basic usage
//!
//! This example illustrates major parts of the functionality in this crate:
//!
//! First, we allocate a new [`NSObject`] using [`ClassType::alloc`].
//! Next, we initialize this object. It is ensured to be deallocated using
//! [`rc::Retained`].
//! Now we're free to send messages to the object to our hearts desire using
//! the [`msg_send!`] or [`msg_send_id!`] macros (depending on the return type
//! of the method).
//! Finally, the `Retained` goes out of scope, and the object is released and
//! deallocated.
//!
//! ```
//! use objc2::{msg_send, msg_send_id, ClassType};
//! use objc2::ffi::NSUInteger;
//! use objc2::rc::Retained;
//! use objc2::runtime::{NSObject, NSObjectProtocol};
//!
//! // Creation
//!
//! let obj1: Retained<NSObject> = unsafe {
//!     msg_send_id![NSObject::alloc(), init]
//! };
//! // Or
//! let obj2 = NSObject::new();
//!
//! // Usage
//!
//! let hash1: NSUInteger = unsafe { msg_send![&obj1, hash] };
//! let hash2: NSUInteger = unsafe { msg_send![&obj2, hash] };
//! assert_ne!(hash1, hash2);
//!
//! let is_kind: bool = unsafe {
//!     msg_send![&obj1, isKindOfClass: NSObject::class()]
//! };
//! assert!(is_kind);
//!
//! let obj1_self: Retained<NSObject> = unsafe { msg_send_id![&obj1, self] };
//! assert_eq!(obj1, obj1_self);
//!
//! // Deallocation on drop
//! ```
//!
//! Note that this example contains a lot of `unsafe` (which should all
//! ideally be justified with a `// SAFETY` comment). This is required because
//! our compiler can verify very little about the Objective-C invocation,
//! including all argument and return types used in [`msg_send!`]. We could
//! have accidentally made `hash` an `f32`, or any other type, and this would
//! trigger undefined behaviour!
//!
//! See [the framework crates] for much more ergonomic usage of the system
//! frameworks like `Foundation`, `AppKit`, `UIKit` and so on.
//!
//! Anyhow, all of this `unsafe` nicely leads us to another feature that this
//! crate has:
//!
//! [`NSObject`]: crate::runtime::NSObject
//! [`rc::Retained`]: crate::rc::Retained
//! [the framework crates]: crate::topics::about_generated
//!
//!
//! ## Encodings and message type verification
//!
//! The Objective-C runtime includes encodings for each method that describe
//! the argument and return types. See the [`encode`] module for a full
//! overview of what this is.
//!
//! The important part is: To make message sending safer, all arguments and
//! return values for messages must implement [`encode::Encode`]. This allows
//! the Rust compiler to prevent you from passing e.g. a [`Vec`] into
//! Objective-C, which would both be UB and leak the vector.
//!
//! Furthermore, we can take advantage of the encodings provided by the
//! runtime to verify that the types used in Rust actually match the types
//! encoded for the method. This is not a perfect solution for ensuring safety
//! (some Rust types have the same Objective-C encoding, but are not
//! equivalent, such as `&T` and `*const T`), but it gets us much closer to
//! it!
//!
//! When `debug_assertions` are enabled we check the encoding every time you
//! send a message, and the message send will panic if they are not
//! equivalent.
//!
//! To take the example above, if we changed the `hash` method's return type
//! as in the following example, it'll panic if debug assertions are enabled:
//!
//! ```should_panic
//! # use objc2::{msg_send, ClassType};
//! # use objc2::runtime::NSObject;
//! #
//! # let obj1 = NSObject::new();
//! #
//! // Wrong return type - this is UB!
//! let hash1: f32 = unsafe { msg_send![&obj1, hash] };
//! #
//! # panic!("does not panic in release mode, so for testing we make it!");
//! ```
//!
//! This library contains further such debug checks.
//!
//! [`Vec`]: std::vec::Vec
//!
//!
//! ## Crate features
//!
//! This crate exports several optional cargo features, see [`Cargo.toml`] for
//! an overview and description of these.
//!
//! [`Cargo.toml`]: https://github.com/madsmtm/objc2/blob/master/crates/objc2/Cargo.toml
//!
//!
//! ## Support for other Operating Systems
//!
//! The bindings can be used on Linux or *BSD utilizing the
//! [GNUstep Objective-C runtime](https://www.github.com/gnustep/libobjc2),
//! see the [`objc-sys`][`objc_sys`] crate for how to configure this.
//!
//!
//! ## Other functionality
//!
//! That was a quick introduction, this library also has [support for handling
//! exceptions][exc], [the ability to declare Objective-C
//! classes][declare_class!], [advanced reference-counting utilities][rc], and more -
//! peruse the documentation at will!
//!
//! [exc]: crate::exception
//! [rc]: crate::rc

#![no_std]
#![cfg_attr(
    feature = "unstable-autoreleasesafe",
    feature(negative_impls, auto_traits)
)]
#![cfg_attr(feature = "unstable-c-unwind", feature(c_unwind))]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![warn(missing_docs)]
#![warn(clippy::missing_errors_doc)]
#![warn(clippy::missing_panics_doc)]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2/0.5.2")]

#[cfg(not(feature = "alloc"))]
compile_error!("The `alloc` feature currently must be enabled.");

#[cfg(not(feature = "std"))]
compile_error!("The `std` feature currently must be enabled.");

extern crate alloc;
extern crate std;

#[doc(no_inline)]
pub use objc_sys as ffi;

#[doc(no_inline)]
pub use self::encode::{Encode, Encoding, RefEncode};
pub use self::top_level_traits::{ClassType, DeclaredClass, Message, ProtocolType};

#[cfg(feature = "objc2-proc-macros")]
#[doc(hidden)]
pub use objc2_proc_macros::__hash_idents;

#[cfg(not(feature = "objc2-proc-macros"))]
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
pub mod encode;
pub mod exception;
mod macros;
pub mod mutability;
pub mod rc;
pub mod runtime;
#[cfg(test)]
mod test_utils;
mod top_level_traits;
#[cfg(any(doc, doctest, test))]
pub mod topics;
mod verify;

/// Deprecated location for a few things that are now in the [`runtime`]
/// module.
#[deprecated = "Moved to the `runtime` module"]
pub mod declare {
    pub use super::runtime::{ClassBuilder, ProtocolBuilder};
    use super::*;

    /// Use [`runtime::ClassBuilder`] instead.
    #[deprecated = "Use `runtime::ClassBuilder` instead."]
    pub type ClassDecl = runtime::ClassBuilder;

    /// Use [`runtime::ProtocolBuilder`] instead.
    #[deprecated = "Use `runtime::ProtocolBuilder` instead."]
    pub type ProtocolDecl = runtime::ProtocolBuilder;
}

// Link to Foundation to make NSObject work
#[cfg_attr(target_vendor = "apple", link(name = "Foundation", kind = "framework"))]
#[cfg_attr(
    all(feature = "gnustep-1-7", not(feature = "unstable-compiler-rt")),
    link(name = "gnustep-base", kind = "dylib")
)]
extern "C" {}
