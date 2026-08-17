//! Structured values.
//!
//! This crate contains the [`ValueBag`] type, a container for an anonymous structured value.
//! `ValueBag`s can be captured in various ways and then formatted, inspected, and serialized
//! without losing their original structure.
//!
//! The producer of a [`ValueBag`] may use a different strategy for capturing than the eventual
//! consumer. They don't need to coordinate directly.

#![doc(html_root_url = "https://docs.rs/value-bag/1.12.0")]
#![no_std]
#![allow(
    clippy::unnecessary_fallible_conversions,
    clippy::explicit_auto_deref,
    clippy::wrong_self_convention
)]

/*
# Crate design

This library internally ties several frameworks together. The details of how
this is done are hidden from end-users. It looks roughly like this:

            ┌─────┐     ┌──────┐
            │sval2│     │serde1│  1. libs on crates.io
            └──┬──┘     └─┬─┬──┘
               ├──────────┘ │
       ┌───────▼──┐     ┌───▼───────┐
       │meta/sval2│     │meta/serde1│  2. meta crates with features
       └───────┬──┘     └───┬───────┘
               │            │
 ┌─────────────▼──┐     ┌───▼─────────────┐
 │internal/sval/v2◄─────┤internal/serde/v1│  3. internal modules with `InternalVisitor`
 └─────────────┬──┘     └───┬─────────────┘
               │            │
        ┌──────▼────────┬───▼────────────┐
        │Internal::Sval2│Internal::Serde1│  4. variants in `Internal` enum
        └───────────────┼────────────────┘
                        │
┌───────────────────────▼────────────────────────┐
│ValueBag::capture_sval2│ValueBag::capture_serde1│  5. ctors on `ValueBag`
└───────────────────────┼────────────────────────┘
                        │
┌───────────────────────▼───────────────────────────┐
│impl Value for ValueBag│impl Serialize for ValueBag│  6. trait impls on `ValueBag`
└───────────────────────┴───────────────────────────┘

## 1. libs on crates.io

These are the frameworks like `serde` or `sval`.

## 2. meta crates with features

These are crates that are internal to `value-bag`. They depend on the public
framework and any utility crates that come along with it. They also expose
features for any other framework. This is done this way so `value-bag` can use
Cargo's `crate?/feature` syntax to conditionally add framework support.

## 3. internal modules with `InternalVisitor`

These are modules in `value-bag` that integrate the framework using the
`InternalVisitor` trait. This makes it possible for that framework to cast
primitive values and pass-through any other framework.

## 4. variants in `Internal` enum

These are individual variants on the `Internal` enum that the `ValueBag`
type wraps. Each framework has one or more variants in this enum.

## 5. ctors on `ValueBag`

These are constructors for producers of `ValueBag`s that accept a value
implementing a serialization trait from a specific framework, like
`serde::Serialize` or `sval::Value`.

## 7. trait impls on `ValueBag`

These are trait impls for consumers of `ValueBag`s that serialize the
underlying value, bridging it if it was produced for a different framework.
*/

#[cfg(any(feature = "std", test))]
#[macro_use]
#[allow(unused_imports)]
extern crate std;

#[cfg(all(not(test), feature = "alloc", not(feature = "std")))]
#[macro_use]
#[allow(unused_imports)]
extern crate core;

#[cfg(all(not(test), feature = "alloc", not(feature = "std")))]
#[macro_use]
#[allow(unused_imports)]
extern crate alloc;

#[cfg(all(not(test), feature = "alloc", not(feature = "std")))]
#[allow(unused_imports)]
mod std {
    pub use crate::{
        alloc::{borrow, boxed, string, vec},
        core::*,
    };

    #[cfg(feature = "owned")]
    pub use crate::alloc::sync;
}

#[cfg(not(any(feature = "alloc", feature = "std", test)))]
#[macro_use]
#[allow(unused_imports)]
extern crate core as std;

mod error;
pub mod fill;
mod impls;
mod internal;
pub mod visit;

#[cfg(any(test, feature = "test"))]
pub mod test;

#[cfg(feature = "owned")]
mod owned;
#[cfg(feature = "owned")]
pub use self::owned::*;

pub use self::error::Error;

/// A dynamic structured value.
///
/// # Capturing values
///
/// There are a few ways to capture a value:
///
/// - Using the `ValueBag::capture_*` and `ValueBag::from_*` methods.
/// - Using the standard `From` trait.
/// - Using the `Fill` API.
///
/// ## Using the `ValueBag::capture_*` methods
///
/// `ValueBag` offers a few constructor methods that capture values of different kinds.
/// These methods require a `T: 'static` to support downcasting.
///
/// ```
/// use value_bag::ValueBag;
///
/// let value = ValueBag::capture_debug(&42i32);
///
/// assert_eq!(Some(42), value.to_i64());
/// ```
///
/// Capturing a value using these methods will retain type information so that
/// the contents of the bag can be serialized using an appropriate type.
///
/// For cases where the `'static` bound can't be satisfied, there's also a few
/// constructors that exclude it.
///
/// ```
/// # use std::fmt::Debug;
/// use value_bag::ValueBag;
///
/// let value = ValueBag::from_debug(&42i32);
///
/// assert_eq!(None, value.to_i64());
/// ```
///
/// These `ValueBag::from_*` methods are lossy though and `ValueBag::capture_*` should be preferred.
///
/// ## Using the standard `From` trait
///
/// Primitive types can be converted into a `ValueBag` using the standard `From` trait.
///
/// ```
/// use value_bag::ValueBag;
///
/// let value = ValueBag::from(42i32);
///
/// assert_eq!(Some(42), value.to_i64());
/// ```
///
/// ## Using the `Fill` API
///
/// The [`fill`] module provides a way to bridge APIs that may not be directly
/// compatible with other constructor methods.
///
/// The `Fill` trait is automatically implemented for closures, so can usually
/// be used in libraries that can't implement the trait themselves.
///
/// ```
/// use value_bag::{ValueBag, fill::Slot};
///
/// let value = ValueBag::from_fill(&|slot: Slot| {
///     #[derive(Debug)]
///     struct MyShortLivedValue;
///
///     slot.fill_debug(&MyShortLivedValue)
/// });
///
/// assert_eq!("MyShortLivedValue", format!("{:?}", value));
/// ```
///
/// The trait can also be implemented manually:
///
/// ```
/// # use std::fmt::Debug;
/// use value_bag::{ValueBag, Error, fill::{Slot, Fill}};
///
/// struct FillDebug;
///
/// impl Fill for FillDebug {
///     fn fill(&self, slot: Slot) -> Result<(), Error> {
///         slot.fill_debug(&42i32 as &dyn Debug)
///     }
/// }
///
/// let value = ValueBag::from_fill(&FillDebug);
///
/// assert_eq!(None, value.to_i64());
/// ```
///
/// # Inspecting values
///
/// Once you have a `ValueBag` there are also a few ways to inspect it:
///
/// - Using `std::fmt`
/// - Using `sval`
/// - Using `serde`
/// - Using the `ValueBag::visit` method.
/// - Using the `ValueBag::to_*` methods.
/// - Using the `ValueBag::downcast_ref` method.
///
/// ## Using the `ValueBag::visit` method
///
/// The [`visit`] module provides a simple visitor API that can be used to inspect
/// the structure of primitives stored in a `ValueBag`.
/// More complex datatypes can then be handled using `std::fmt`, `sval`, or `serde`.
///
/// ```
/// #[cfg(not(feature = "std"))] fn main() {}
/// #[cfg(feature = "std")]
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # fn escape(buf: &[u8]) -> &[u8] { buf }
/// # fn itoa_fmt<T>(num: T) -> Vec<u8> { vec![] }
/// # fn ryu_fmt<T>(num: T) -> Vec<u8> { vec![] }
/// # use std::io::Write;
/// use value_bag::{ValueBag, Error, visit::Visit};
///
/// // Implement some simple custom serialization
/// struct MyVisit(Vec<u8>);
/// impl<'v> Visit<'v> for MyVisit {
///     fn visit_any(&mut self, v: ValueBag) -> Result<(), Error> {
///         // Fallback to `Debug` if we didn't visit the value specially
///         write!(&mut self.0, "{:?}", v).map_err(|_| Error::msg("failed to write value"))
///     }
///
///     fn visit_u64(&mut self, v: u64) -> Result<(), Error> {
///         self.0.extend_from_slice(itoa_fmt(v).as_slice());
///         Ok(())
///     }
///
///     fn visit_i64(&mut self, v: i64) -> Result<(), Error> {
///         self.0.extend_from_slice(itoa_fmt(v).as_slice());
///         Ok(())
///     }
///
///     fn visit_f64(&mut self, v: f64) -> Result<(), Error> {
///         self.0.extend_from_slice(ryu_fmt(v).as_slice());
///         Ok(())
///     }
///
///     fn visit_str(&mut self, v: &str) -> Result<(), Error> {
///         self.0.push(b'\"');
///         self.0.extend_from_slice(escape(v.as_bytes()));
///         self.0.push(b'\"');
///         Ok(())
///     }
///
///     fn visit_bool(&mut self, v: bool) -> Result<(), Error> {
///         self.0.extend_from_slice(if v { b"true" } else { b"false" });
///         Ok(())
///     }
/// }
///
/// let value = ValueBag::from(42i64);
///
/// let mut visitor = MyVisit(vec![]);
/// value.visit(&mut visitor)?;
/// # Ok(())
/// # }
/// ```
///
/// ## Using `std::fmt`
///
/// Any `ValueBag` can be formatted using the `std::fmt` machinery as either `Debug`
/// or `Display`.
///
/// ```
/// use value_bag::ValueBag;
///
/// let value = ValueBag::from(true);
///
/// assert_eq!("true", format!("{:?}", value));
/// ```
///
/// ## Using `sval`
///
/// When the `sval2` feature is enabled, any `ValueBag` can be serialized using `sval`.
/// This makes it possible to visit any typed structure captured in the `ValueBag`,
/// including complex datatypes like maps and sequences.
///
/// `sval` doesn't need to allocate so can be used in no-std environments.
///
/// First, enable the `sval2` feature in your `Cargo.toml`:
///
/// ```toml
/// [dependencies.value-bag]
/// features = ["sval2"]
/// ```
///
/// Then stream the contents of the `ValueBag` using `sval`.
///
/// ```
/// # #[cfg(not(all(feature = "std", feature = "sval2")))] fn main() {}
/// # #[cfg(all(feature = "std", feature = "sval2"))]
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # use value_bag_sval2::json as sval_json;
/// use value_bag::ValueBag;
///
/// let value = ValueBag::from(42i64);
/// let json = sval_json::stream_to_string(value)?;
/// # Ok(())
/// # }
/// ```
///
/// ## Using `serde`
///
/// When the `serde1` feature is enabled, any `ValueBag` can be serialized using `serde`.
/// This makes it possible to visit any typed structure captured in the `ValueBag`,
/// including complex datatypes like maps and sequences.
///
/// `serde` needs a few temporary allocations, so also brings in the `std` feature.
///
/// First, enable the `serde1` feature in your `Cargo.toml`:
///
/// ```toml
/// [dependencies.value-bag]
/// features = ["serde1"]
/// ```
///
/// Then stream the contents of the `ValueBag` using `serde`.
///
/// ```
/// # #[cfg(not(all(feature = "std", feature = "serde1")))] fn main() {}
/// # #[cfg(all(feature = "std", feature = "serde1"))]
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # use value_bag_serde1::json as serde_json;
/// use value_bag::ValueBag;
///
/// let value = ValueBag::from(42i64);
/// let json = serde_json::to_string(&value)?;
/// # Ok(())
/// # }
/// ```
///
/// Also see [`serde.rs`](https://serde.rs) for more examples of writing your own serializers.
///
/// ## Using the `ValueBag::to_*` methods
///
/// `ValueBag` provides a set of methods for attempting to pull a concrete value out.
/// These are useful for ad-hoc analysis but aren't intended for exhaustively serializing
/// the contents of a `ValueBag`.
///
/// ```
/// use value_bag::ValueBag;
///
/// let value = ValueBag::capture_display(&42u64);
///
/// assert_eq!(Some(42u64), value.to_u64());
/// ```
///
/// ## Using the `ValueBag::downcast_ref` method
///
/// When a `ValueBag` is created using one of the `capture_*` constructors, it can be downcast
/// back to its original value.
/// This can also be useful for ad-hoc analysis where there's a common possible non-primitive
/// type that could be captured.
///
/// ```
/// # #[derive(Debug)] struct SystemTime;
/// # fn now() -> SystemTime { SystemTime }
/// use value_bag::ValueBag;
///
/// let timestamp = now();
/// let value = ValueBag::capture_debug(&timestamp);
///
/// assert!(value.downcast_ref::<SystemTime>().is_some());
/// ```
///
/// # Working with sequences
///
/// The `seq` feature of `value-bag` enables utilities for working with values that are sequences.
/// First, enable the `seq` feature in your `Cargo.toml`:
///
/// ```toml
/// [dependencies.value-bag]
/// features = ["seq"]
/// ```
///
/// Slices and arrays can be captured as sequences:
///
/// ```
/// # #[cfg(not(all(feature = "serde1", feature = "seq")))] fn main() {}
/// # #[cfg(all(feature = "serde1", feature = "seq"))]
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # use value_bag_serde1::json as serde_json;
/// use value_bag::ValueBag;
///
/// let value = ValueBag::from_seq_slice(&[1, 2, 3]);
///
/// assert_eq!("[1,2,3]", serde_json::to_string(&value)?);
/// # Ok(())
/// # }
/// ```
///
/// A sequence captured with either `sval` or `serde` can have its elements extracted:
///
/// ```
/// # #[cfg(not(all(feature = "serde1", feature = "seq")))] fn main() {}
/// # #[cfg(all(feature = "serde1", feature = "seq"))]
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use value_bag::ValueBag;
///
/// let value = ValueBag::from_serde1(&[1.0, 2.0, 3.0]);
///
/// let seq = value.to_f64_seq::<Vec<Option<f64>>>().ok_or("not a sequence")?;
///
/// assert_eq!(vec![Some(1.0), Some(2.0), Some(3.0)], seq);
/// # Ok(())
/// # }
/// ```
#[derive(Clone)]
pub struct ValueBag<'v> {
    inner: internal::Internal<'v>,
}

impl<'v> ValueBag<'v> {
    /// Get an empty `ValueBag`.
    #[inline]
    pub const fn empty() -> ValueBag<'v> {
        ValueBag {
            inner: internal::Internal::None,
        }
    }

    /// Get a `ValueBag` from an `Option`.
    ///
    /// This method will return `ValueBag::empty` if the value is `None`.
    #[inline]
    pub fn from_option(v: Option<impl Into<ValueBag<'v>>>) -> ValueBag<'v> {
        match v {
            Some(v) => v.into(),
            None => ValueBag::empty(),
        }
    }

    /// Get a `ValueBag` from a `u8`.
    #[inline]
    pub const fn from_u8(v: u8) -> ValueBag<'v> {
        ValueBag {
            inner: internal::Internal::Unsigned(v as u64),
        }
    }

    /// Get a `ValueBag` from a `u16`.
    #[inline]
    pub const fn from_u16(v: u16) -> ValueBag<'v> {
        ValueBag {
            inner: internal::Internal::Unsigned(v as u64),
        }
    }

    /// Get a `ValueBag` from a `u32`.
    #[inline]
    pub const fn from_u32(v: u32) -> ValueBag<'v> {
        ValueBag {
            inner: internal::Internal::Unsigned(v as u64),
        }
    }

    /// Get a `ValueBag` from a `u64`.
    #[inline]
    pub const fn from_u64(v: u64) -> ValueBag<'v> {
        ValueBag {
            inner: internal::Internal::Unsigned(v),
        }
    }

    /// Get a `ValueBag` from a `usize`.
    #[inline]
    pub const fn from_usize(v: usize) -> ValueBag<'v> {
        ValueBag {
            inner: internal::Internal::Unsigned(v as u64),
        }
    }

    /// Get a `ValueBag` from a `u128`.
    #[inline]
    pub const fn from_u128_ref(v: &'v u128) -> ValueBag<'v> {
        ValueBag {
            #[cfg(not(feature = "inline-i128"))]
            inner: internal::Internal::BigUnsigned(v),
            #[cfg(feature = "inline-i128")]
            inner: internal::Internal::BigUnsigned(*v),
        }
    }

    /// Get a `ValueBag` from a `u128`.
    #[inline]
    #[cfg(feature = "inline-i128")]
    pub const fn from_u128(v: u128) -> ValueBag<'v> {
        ValueBag {
            inner: internal::Internal::BigUnsigned(v),
        }
    }

    /// Get a `ValueBag` from a `i8`.
    #[inline]
    pub const fn from_i8(v: i8) -> ValueBag<'v> {
        ValueBag {
            inner: internal::Internal::Signed(v as i64),
        }
    }

    /// Get a `ValueBag` from a `i16`.
    #[inline]
    pub const fn from_i16(v: i16) -> ValueBag<'v> {
        ValueBag {
            inner: internal::Internal::Signed(v as i64),
        }
    }

    /// Get a `ValueBag` from a `i32`.
    #[inline]
    pub const fn from_i32(v: i32) -> ValueBag<'v> {
        ValueBag {
            inner: internal::Internal::Signed(v as i64),
        }
    }

    /// Get a `ValueBag` from a `i64`.
    #[inline]
    pub const fn from_i64(v: i64) -> ValueBag<'v> {
        ValueBag {
            inner: internal::Internal::Signed(v),
        }
    }

    /// Get a `ValueBag` from a `isize`.
    #[inline]
    pub const fn from_isize(v: isize) -> ValueBag<'v> {
        ValueBag {
            inner: internal::Internal::Signed(v as i64),
        }
    }

    /// Get a `ValueBag` from a `i128`.
    #[inline]
    pub const fn from_i128_ref(v: &'v i128) -> ValueBag<'v> {
        ValueBag {
            #[cfg(not(feature = "inline-i128"))]
            inner: internal::Internal::BigSigned(v),
            #[cfg(feature = "inline-i128")]
            inner: internal::Internal::BigSigned(*v),
        }
    }

    /// Get a `ValueBag` from a `i128`.
    #[inline]
    #[cfg(feature = "inline-i128")]
    pub const fn from_i128(v: i128) -> ValueBag<'v> {
        ValueBag {
            inner: internal::Internal::BigSigned(v),
        }
    }

    /// Get a `ValueBag` from a `f32`.
    #[inline]
    pub const fn from_f32(v: f32) -> ValueBag<'v> {
        ValueBag {
            inner: internal::Internal::Float(v as f64),
        }
    }

    /// Get a `ValueBag` from a `f64`.
    #[inline]
    pub const fn from_f64(v: f64) -> ValueBag<'v> {
        ValueBag {
            inner: internal::Internal::Float(v),
        }
    }

    /// Get a `ValueBag` from a `bool`.
    #[inline]
    pub const fn from_bool(v: bool) -> ValueBag<'v> {
        ValueBag {
            inner: internal::Internal::Bool(v),
        }
    }

    /// Get a `ValueBag` from a `str`.
    #[inline]
    pub const fn from_str(v: &'v str) -> ValueBag<'v> {
        ValueBag {
            inner: internal::Internal::Str(v),
        }
    }

    /// Get a `ValueBag` from a `char`.
    #[inline]
    pub const fn from_char(v: char) -> ValueBag<'v> {
        ValueBag {
            inner: internal::Internal::Char(v),
        }
    }

    /// Get a `ValueBag` from a reference to a `ValueBag`.
    #[inline]
    pub const fn by_ref(&self) -> ValueBag<'_> {
        ValueBag {
            inner: self.inner.by_ref(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::std::mem;

    #[cfg(feature = "inline-i128")]
    const SIZE_LIMIT_U64: usize = 4;
    #[cfg(not(feature = "inline-i128"))]
    const SIZE_LIMIT_U64: usize = 3;

    #[test]
    fn value_bag_size() {
        let size = mem::size_of::<ValueBag<'_>>();
        let limit = mem::size_of::<u64>() * SIZE_LIMIT_U64;

        if size > limit {
            panic!(
                "`ValueBag` size ({} bytes) is too large (expected up to {} bytes)",
                size, limit,
            );
        }
    }
}
