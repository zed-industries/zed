//! The items here are intentionally defined in a private module not inside of the
//! [`crate::__`] module. This is because that module is marked with `#[deprecated]`
//! which makes all items defined in that module also deprecated.
//!
//! This is not the desired behavior for the items defined here. They are not deprecated,
//! and they are expected to be exposed to the users. However, the users must not reference
//! them through the `bon` crate. Instead, they should use the re-exports from the state
//! module generated for the builder.

use crate::__::{Sealed, Set, Unset};

/// Marker trait that indicates that the member is set, i.e. at least
/// one of its setters was called.
#[rustversion::attr(
    since(1.78.0),
    diagnostic::on_unimplemented(
        message = "the member `{Self}` was not set, but this method requires it to be set",
        label = "the member `{Self}` was not set, but this method requires it to be set"
    )
)]
pub trait IsSet: Sealed {}

/// Marker trait that indicates that the member is unset, i.e. none
/// of its setters was called.
#[rustversion::attr(
    since(1.78.0),
    diagnostic::on_unimplemented(
        message = "the member `{Self}` was already set, but this method requires it to be unset",
        label = "the member `{Self}` was already set, but this method requires it to be unset"
    )
)]
pub trait IsUnset: Sealed {}

#[doc(hidden)]
impl<Name> IsSet for Set<Name> {}

#[doc(hidden)]
impl<Name> IsUnset for Unset<Name> {}
