//! Ambient Authority
//!
//! Capability-oriented library APIs should re-export the [`ambient_authority`]
//! function and [`AmbientAuthority`] type in their own API, and include this
//! type as an argument to any function which utilizes ambient authority.
//!
//! For example:
//!
//! ```rust
//! // Re-export the `AmbientAuthority` type.
//! pub use ambient_authority::{ambient_authority, AmbientAuthority};
//!
//! // Declare functions that use ambient authority with an `AmbientAuthority`
//! // argument.
//! pub fn do_a_thing(_: AmbientAuthority) {
//!     println!("hello world");
//! }
//! ```
//!
//! To use an API that uses [`AmbientAuthority`], call [`ambient_authority`]
//! and pass the result:
//!
//! ```rust,ignore
//! use ambient_authority::ambient_authority;
//!
//! do_a_thing(ambient_authority());
//! ```

#![deny(missing_docs)]
#![deny(clippy::disallowed_method)]
#![forbid(unsafe_code)]
#![no_std]

/// Instances of this `AmbientAuthority` type serve to indicate that the
/// [`ambient_authority`] function has been called, indicating that the user
/// has explicitly opted into using ambient authority.
#[derive(Copy, Clone)]
pub struct AmbientAuthority(());

/// Return an `AmbientAuthority` value, which allows use of functions that
/// include an `AmbientAuthority` argument.
#[inline]
#[must_use]
#[allow(clippy::missing_const_for_fn)]
pub fn ambient_authority() -> AmbientAuthority {
    AmbientAuthority(())
}

/// Internal implementation detail for macros.
///
/// For users concerned about resource names such as file names or network
/// addresses being influenced by untrusted code, file names in static string
/// literals and network addresses in integer literals are ok. This function
/// is similar to `ambient_authority` but is meant to be used in macros that
/// verify that the resource identifiers are known at compile time. Users of
/// the `disallowed_method` clippy lint can opt to allow or disallow this
/// function separately from the general `ambient_authority` function.
#[inline]
#[must_use]
#[doc(hidden)]
pub const fn ambient_authority_known_at_compile_time() -> AmbientAuthority {
    AmbientAuthority(())
}
