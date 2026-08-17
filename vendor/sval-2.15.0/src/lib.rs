/*!
Structured, streaming values.

`sval` is a serialization framework that treats data as a flat stream of tokens.
The source of that data could be some Rust object or parsed from some encoding.
It's well suited to self-describing, text-based formats like JSON.

# Getting started

For a complete introduction, see [the project readme](https://github.com/sval-rs/sval).

Add `sval` to your `Cargo.toml`:

```toml
[dependencies.sval]
version = "2.15.0"
```

By default, `sval` doesn't depend on Rust's standard library or integrate with its collection types.
To include them, add the `alloc` or `std` Cargo features:

```toml
[dependencies.sval]
version = "2.15.0"
features = ["std"]
```

`sval` provides procedural macros for deriving its traits.
Add the `derive` Cargo feature to enable them:

```toml
[dependencies.sval]
version = "2.15.0"
features = ["derive"]
```

# The `Value` trait

[`Value`] is a trait for data types to implement that surfaces their structure through visitors called _streams_.
`Value` is like `serde`'s `Serialize`. It can also be used like `serde`'s `Deserialize`.

Many standard types in Rust implement the `Value` trait. It can be derived on your own types using the `sval_derive` library.

# The `Stream` trait

[`Stream`] is a trait for data formats and visitors to implement that observes the structure of _values_.
`Stream` is like `serde`'s `Serializer`. It can also be used like `serde`'s `Deserializer`.

# Data-model

`sval`'s data-model is defined by the [`Stream`] trait. It includes:

- Null
- Booleans (`true`, `false`)
- Text blobs
- Binary blobs
- Integers (`u8`-`u128`, `i8`-`i128`)
- Binary floating points (`f32`-`f64`)
- Sequences
- Maps
- Tags
- Tagged values
- Records
- Tuples
- Enums

# Tags

[`Tag`] is a type for extending `sval`'s data-model with new kinds of values.
Rust's own `()` and `Option<T>` types are expressed as tags.
Other examples of tags include text that encodes RFC3339 timestamps or RFC4122 UUIDs.

The [`tags`] module contains built-in tags.
Other libraries may define their own tags too.
*/

#![no_std]
#![deny(missing_docs)]

#[cfg(feature = "std")]
extern crate std;

#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate alloc;
#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate core;

#[cfg(all(feature = "alloc", not(feature = "std")))]
mod std {
    pub use crate::{
        alloc::{borrow, boxed, collections, string, vec},
        core::{cmp, convert, fmt, hash, marker, mem, ops, result, str, write},
    };
}

#[cfg(all(not(feature = "alloc"), not(feature = "std")))]
extern crate core as std;

#[cfg(feature = "derive")]
#[doc(inline)]
pub use sval_derive_macros::*;

mod data;
mod result;
mod stream;
mod value;

#[doc(inline)]
pub use self::{data::*, result::*, stream::*, value::*};

/**
A generic streaming result.
*/
pub type Result<T = (), E = Error> = std::result::Result<T, E>;

/**
Stream a value through a stream.
*/
pub fn stream<'sval>(
    stream: &mut (impl Stream<'sval> + ?Sized),
    value: &'sval (impl Value + ?Sized),
) -> Result {
    stream.value(value)
}

/**
Stream a value through a stream with an arbitrarily short lifetime.
*/
pub fn stream_computed<'sval>(
    stream: &mut (impl Stream<'sval> + ?Sized),
    value: impl Value,
) -> Result {
    stream.value_computed(&value)
}

// NOTE: Tests for implementations of `Value` are in `sval_test`
