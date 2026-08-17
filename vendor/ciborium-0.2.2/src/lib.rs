// SPDX-License-Identifier: Apache-2.0

//! Welcome to Ciborium!
//!
//! Ciborium contains CBOR serialization and deserialization implementations for serde.
//!
//! # Quick Start
//!
//! You're probably looking for [`from_reader()`](crate::de::from_reader)
//! and [`into_writer()`](crate::ser::into_writer), which are
//! the main functions. Note that byte slices are also readers and writers and can be
//! passed to these functions just as streams can.
//!
//! For dynamic CBOR value creation/inspection, see [`Value`](crate::value::Value).
//!
//! # Design Decisions
//!
//! ## Always Serialize Numeric Values to the Smallest Size
//!
//! Although the CBOR specification has differing numeric widths, this is only
//! a form of compression on the wire and is not intended to directly
//! represent an "integer width" or "float width." Therefore, ciborium always
//! serializes numbers to the smallest possible lossless encoding. For example,
//! we serialize `1u128` as a single byte (`01`). Likewise, we will also freely
//! decode that single byte into a `u128`.
//!
//! While there is some minor performance cost for this, there are several
//! reasons for this choice. First, the specification seems to imply it by
//! using a separate bit for the sign. Second, the specification requires
//! that implementations handle leading zeroes; a liberal reading of which
//! implies a requirement for lossless coercion. Third, dynamic languages like
//! Python have no notion of "integer width," making this is a practical
//! choice for maximizing wire compatibility with those languages.
//!
//! This coercion is **always** lossless. For floats, this implies that we
//! only coerce to a smaller size if coercion back to the original size has
//! the same raw bits as the original.
//!
//! ## Compatibility with Other Implementations
//!
//! The ciborium project follows the [Robustness Principle](https://en.wikipedia.org/wiki/Robustness_principle).
//! Therefore, we aim to be liberal in what we accept. This implies that we
//! aim to be wire-compatible with other implementations in decoding, but
//! not necessarily encoding.
//!
//! One notable example of this is that `serde_cbor` uses fixed-width encoding
//! of numbers and doesn't losslessly coerce. This implies that `ciborium` will
//! successfully decode `serde_cbor` encodings, but the opposite may not be the
//! case.
//!
//! ## Representing Map as a Sequence of Values
//!
//! Other serde parsers have generally taken the route of using `BTreeMap` or
//! `HashMap` to implement their encoding's underlying `Map` type. This crate
//! chooses to represent the `Map` type using `Vec<(Value, Value)>` instead.
//!
//! This decision was made because this type preserves the order of the pairs
//! on the wire. Further, for those that need the properties of `BTreeMap` or
//! `HashMap`, you can simply `collect()` the values into the respective type.
//! This provides maximum flexibility.
//!
//! ## Low-level Library
//!
//! The ciborium crate has the beginnings of a low-level library in the
//! (private) `basic` module. We may extend this to be more robust and expose
//! it for application consumption once we have it in a good state. If you'd
//! like to collaborate with us on that, please contact us. Alternatively,
//! we might fork this code into a separate crate with no serde dependency.
//!
//! ## Internal Types
//!
//! The ciborium crate contains a number of internal types that implement
//! useful serde traits. While these are not currently exposed, we might
//! choose to expose them in the future if there is demand. Generally, this
//! crate takes a conservative approach to exposing APIs to avoid breakage.
//!
//! ## Packed Encoding?
//!
//! Packed encoding uses numerical offsets to represent structure field names
//! and enum variant names. This can save significant space on the wire.
//!
//! While the authors of this crate like packed encoding, it should generally
//! be avoided because it can be fragile as it exposes invariants of your Rust
//! code to remote actors. We might consider adding this in the future. If you
//! are interested in this, please contact us.

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::cargo)]
#![allow(clippy::unit_arg)]

extern crate alloc;

pub mod de;
pub mod ser;
pub mod tag;
pub mod value;

// Re-export the [items recommended by serde](https://serde.rs/conventions.html).
#[doc(inline)]
pub use crate::de::from_reader;
#[doc(inline)]
pub use crate::de::from_reader_with_buffer;

#[doc(inline)]
pub use crate::ser::into_writer;

#[doc(inline)]
pub use crate::value::Value;

/// Build a `Value` conveniently.
///
/// The syntax should be intuitive if you are familiar with JSON. You can also
/// inline simple Rust expressions, including custom values that implement
/// `serde::Serialize`. Note that this macro returns `Result<Value, Error>`,
/// so you should handle the error appropriately.
///
/// ```
/// use ciborium::cbor;
///
/// let value = cbor!({
///     "code" => 415,
///     "message" => null,
///     "continue" => false,
///     "extra" => { "numbers" => [8.2341e+4, 0.251425] },
/// }).unwrap();
/// ```
#[macro_export]
macro_rules! cbor {
    (@map {$($key:expr => $val:expr),*} $(,)?) => {{
        $crate::value::Value::Map(vec![
            $(
                (cbor!( $key )?, cbor!( $val )?)
            ),*
        ])
    }};

    (@map {$($key:expr => $val:expr),*} { $($nkey:tt)* } => $($next:tt)*) => {
        cbor!(
            @map
            { $($key => $val),* }
            cbor!({ $($nkey)* })? =>
            $($next)*
        )
    };

    (@map {$($key:expr => $val:expr),*} [ $($nkey:tt)* ] => $($next:tt)*) => {
        cbor!(
            @map
            { $($key => $val),* }
            cbor!([ $($nkey)* ])? =>
            $($next)*
        )
    };

    (@map {$($key:expr => $val:expr),*} $nkey:expr => { $($nval:tt)* }, $($next:tt)*) => {
        cbor!(
            @map
            { $($key => $val,)* $nkey => cbor!({ $($nval)* })? }
            $($next)*
        )
    };

    (@map {$($key:expr => $val:expr),*} $nkey:expr => [ $($nval:tt)* ], $($next:tt)*) => {
        cbor!(
            @map
            { $($key => $val,)* $nkey => cbor!([ $($nval)* ])? }
            $($next)*
        )
    };

    (@map {$($key:expr => $val:expr),*} $nkey:expr => $nval:expr, $($next:tt)*) => {
        cbor!(
            @map
            { $($key => $val,)* $nkey => cbor!($nval)? }
            $($next)*
        )
    };

    (@seq [$($val:expr),*] $(,)?) => {
        $crate::value::Value::Array(
            vec![$( cbor!($val)? ),*]
        )
    };

    (@seq [$($val:expr),*] { $($item:tt)* }, $($next:tt)*) => {
        cbor!(
            @seq
            [ $($val,)* cbor!({ $($item)* })? ]
            $($next)*
        )
    };

    (@seq [$($val:expr),*] [ $($item:tt)* ], $($next:tt)*) => {
        cbor!(
            @seq
            [ $($val,)* cbor!([ $($item)* ])? ]
            $($next)*
        )
    };

    (@seq [$($val:expr),*] $item:expr, $($next:tt)*) => {
        cbor!(
            @seq
            [ $($val,)* $item ]
            $($next)*
        )
    };

    ({ $($next:tt)* }) => {(||{
        ::core::result::Result::<_, $crate::value::Error>::from(Ok(cbor!(@map {} $($next)* ,)))
    })()};

    ([ $($next:tt)* ]) => {(||{
        ::core::result::Result::<_, $crate::value::Error>::from(Ok(cbor!(@seq [] $($next)* ,)))
    })()};

    ($val:expr) => {{
        #[allow(unused_imports)]
        use $crate::value::Value::Null as null;
        $crate::value::Value::serialized(&$val)
    }};
}
