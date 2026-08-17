// SPDX-FileCopyrightText: 2020 Robin Krahl <robin.krahl@ireas.org>
// SPDX-License-Identifier: Apache-2.0 or MIT

//! Provides [`Merge`][], a trait for objects that can be merged.
//!
//! # Usage
//!
//! ```
//! trait Merge {
//!     fn merge(&mut self, other: Self);
//! }
//! ```
//!
//! The [`Merge`][] trait can be used to merge two objects of the same type into one.  The intended
//! use case is merging configuration from different sources, for example environment variables,
//! multiple configuration files and command-line arguments, see the [`args.rs`][] example.
//!
//! `Merge` is implemented for `Option` and can be derived for structs.  When deriving the `Merge`
//! trait for a struct, you can provide custom merge strategies for the fields that don’t implement
//! `Merge`.  A merge strategy is a function with the signature `fn merge<T>(left: &mut T, right:
//! T)` that merges `right` into `left`.  The submodules of this crate provide strategies for the
//! most common types, but you can also define your own strategies.
//!
//! ## Features
//!
//! This crate has the following features:
//!
//! - `derive` (default):  Enables the derive macro for the `Merge` trait using the `merge_derive`
//!   crate.
//! - `num` (default): Enables the merge strategies in the `num` module that require the
//!   `num_traits` crate.
//! - `std` (default): Enables the merge strategies in the `vec` module that require the standard
//!   library.  If this feature is not set, `merge` is a `no_std` library.
//!
//! # Example
//!
//! ```
//! use merge::Merge;
//!
//! #[derive(Merge)]
//! struct User {
//!     // Fields with the skip attribute are skipped by Merge
//!     #[merge(skip)]
//!     pub name: &'static str,
//!
//!     // The Merge implementation for Option replaces its value if it is None
//!     pub location: Option<&'static str>,
//!
//!     // The strategy attribute is used to customize the merge behavior
//!     #[merge(strategy = merge::vec::append)]
//!     pub groups: Vec<&'static str>,
//! }
//!
//! let defaults = User {
//!     name: "",
//!     location: Some("Internet"),
//!     groups: vec!["rust"],
//! };
//! let mut ferris = User {
//!     name: "Ferris",
//!     location: None,
//!     groups: vec!["mascot"],
//! };
//! ferris.merge(defaults);
//!
//! assert_eq!("Ferris", ferris.name);
//! assert_eq!(Some("Internet"), ferris.location);
//! assert_eq!(vec!["mascot", "rust"], ferris.groups);
//! ```
//!
//! [`Merge`]: trait.Merge.html
//! [`args.rs`]: https://git.sr.ht/~ireas/merge-rs/tree/master/examples/args.rs

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "derive")]
pub use merge_derive::*;

/// A trait for objects that can be merged.
///
/// # Deriving
///
/// `Merge` can be derived for structs if the `derive` feature is enabled.  The generated
/// implementation calls the `merge` method for all fields, or the merge strategy function if set.
/// You can use these field attributes to configure the generated implementation:
/// - `skip`: Skip this field in the `merge` method.
/// - `strategy = f`: Call `f(self.field, other.field)` instead of calling the `merge` function for
///    this field.
///
/// # Examples
///
/// Using the `Merge` implementation for `Option`:
///
/// ```
/// use merge::Merge as _;
///
/// let mut val = None;
/// val.merge(Some(42));
/// assert_eq!(Some(42), val);
/// ```
///
/// Deriving `Merge` for a struct:
///
/// ```
/// use merge::Merge;
///
/// #[derive(Debug, PartialEq, Merge)]
/// struct S {
///     option: Option<usize>,
///
///     #[merge(skip)]
///     s: String,
///
///     #[merge(strategy = merge::bool::overwrite_false)]
///     flag: bool,
/// }
///
/// let mut val = S {
///     option: None,
///     s: "some ignored value".to_owned(),
///     flag: false,
/// };
/// val.merge(S {
///     option: Some(42),
///     s: "some other ignored value".to_owned(),
///     flag: true,
/// });
/// assert_eq!(S {
///     option: Some(42),
///     s: "some ignored value".to_owned(),
///     flag: true,
/// }, val);
/// ```
pub trait Merge {
    /// Merge another object into this object.
    fn merge(&mut self, other: Self);
}

impl<T> Merge for Option<T> {
    fn merge(&mut self, mut other: Self) {
        if !self.is_some() {
            *self = other.take();
        }
    }
}

/// Merge strategies for boolean types.
pub mod bool {
    /// Overwrite left with right if the value of left is false.
    pub fn overwrite_false(left: &mut bool, right: bool) {
        if !*left {
            *left = right;
        }
    }

    /// Overwrite left with right if the value of left is true.
    pub fn overwrite_true(left: &mut bool, right: bool) {
        if *left {
            *left = right;
        }
    }
}

/// Merge strategies for numeric types.
///
/// These strategies are only available if the `num` feature is enabled.
#[cfg(feature = "num")]
pub mod num {
    /// Set left to the saturated some of left and right.
    pub fn saturating_add<T: num_traits::SaturatingAdd>(left: &mut T, right: T) {
        *left = left.saturating_add(&right);
    }

    /// Overwrite left with right if the value of left is zero.
    pub fn overwrite_zero<T: num_traits::Zero>(left: &mut T, right: T) {
        if left.is_zero() {
            *left = right;
        }
    }
}

/// Merge strategies for types that form a total order.
pub mod ord {
    use core::cmp;

    /// Set left to the maximum of left and right.
    pub fn max<T: cmp::Ord>(left: &mut T, right: T) {
        if cmp::Ord::cmp(left, &right) == cmp::Ordering::Less {
            *left = right;
        }
    }

    /// Set left to the minimum of left and right.
    pub fn min<T: cmp::Ord>(left: &mut T, right: T) {
        if cmp::Ord::cmp(left, &right) == cmp::Ordering::Greater {
            *left = right;
        }
    }
}

/// Merge strategies for vectors.
///
/// These strategies are only available if the `std` feature is enabled.
#[cfg(feature = "std")]
pub mod vec {
    /// Overwrite left with right if left is empty.
    pub fn overwrite_empty<T>(left: &mut Vec<T>, mut right: Vec<T>) {
        if left.is_empty() {
            left.append(&mut right);
        }
    }

    /// Append the contents of right to left.
    pub fn append<T>(left: &mut Vec<T>, mut right: Vec<T>) {
        left.append(&mut right);
    }

    /// Prepend the contents of right to left.
    pub fn prepend<T>(left: &mut Vec<T>, mut right: Vec<T>) {
        right.append(left);
        *left = right;
    }
}
