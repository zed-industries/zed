//! Serialization/deserialization support.
//!
//! The upstream Java project has established several different types of serialization. We have
//! currently implemented V2 and V2 + DEFLATE (following the names used by the Java implementation).
//!
//! These formats are compact binary representations of the state of the histogram. They are
//! intended to be used for archival or transmission to other systems for further analysis. A
//! typical use case would be to periodically serialize a histogram, save it somewhere, and reset
//! the histogram.
//!
//! Histograms are designed to be added, subtracted, and otherwise manipulated, and an efficient
//! storage format facilitates this. As an example, you might be capturing histograms once a minute
//! to have a granular view into your performance over time, but you might also want to see longer
//! trends over an hour or day. Simply deserialize the last 60 minutes worth to recreate their
//! in-memory `Histogram` form, add them all together into one `Histogram`, and perform whatever
//! calculations you wish on the resulting histogram. This would allow you to correctly calculate
//! the 99.99th percentile for the entire hour, for instance, which is not something you can do
//! if you have only stored percentiles (as opposed to the entire histogram) for each minute.
//!
//! # Performance concerns
//!
//! Serialization is quite fast; serializing a histogram in V2 format that represents 1 to
//! `u64::max_value()` with 3 digits of precision with tens of thousands of recorded counts takes
//! about 40 microseconds on an E5-1650v3 Xeon. Deserialization is about 3x slower, but that will
//! improve as there are still some optimizations to perform.
//!
//! For the V2 format, the space used for a histogram will depend mainly on precision since higher
//! precision will reduce the extent to which different values are grouped into the same bucket.
//! Having a large value range (e.g. 1 to `u64::max_value()`) will not directly impact the size if
//! there are many zero counts as zeros are compressed away.
//!
//! V2 + DEFLATE is significantly slower to serialize (around 10x) but only a little bit slower to
//! deserialize (less than 2x). YMMV depending on the compressibility of your histogram data, the
//! speed of the underlying storage medium, etc. Naturally, you can always compress at a later time:
//! there's no reason why you couldn't serialize as V2 and then later re-serialize it as V2 +
//! DEFLATE on another system (perhaps as a batch job) for better archival storage density.
//!
//! # API
//!
//! Each serialization format has its own serializer struct, but since each format is reliably
//! distinguishable from each other, there is only one `Deserializer` struct that will work for
//! any of the formats this library implements.
//!
//! Serializers and deserializers are intended to be re-used for many histograms. You can use them
//! for one histogram and throw them away; it will just be less efficient as the cost of their
//! internal buffers will not be amortized across many histograms.
//!
//! Serializers can write to any `Write` implementation, and `Deserializer` can read from any
//! `Read`. This should make it easy to use them in almost any context, as everything from i/o
//! streams to `Vec<u8>` can be a `Read` or `Write`.
//!
//! # Interval logs
//!
//! See the `interval_log` module.
//!
//! ### Integration with general-purpose serialization libraries
//!
//! In general, serializing histograms should be straightforward: pick the serialization format
//! that is suitable for your requirements (e.g. based on what formats are supported by other tools
//! that will consume the serialized histograms) and use the corresponding struct.
//!
//! However, there are some approaches to serialization like [serde's
//! `Serialize`](https://docs.serde.rs/serde/trait.Serialize.html) or [`rustc_serialize`'s
//! `Encodable`](https://doc.rust-lang.org/rustc-serialize/rustc_serialize/trait.Encodable.html)
//! that effectively require that only one way of serialization can be used because a trait can
//! only be implemented once for a struct. This is too restrictive for histograms since they
//! inherently have multiple ways of being serialized, so as a library we cannot pick the format
//! for you. If you need to interoperate with such a restriction, a good approach is to first pick
//! your serialization format (V2, etc) like you normally would, then make a wrapper struct. The
//! wrapper effectively gives you a struct whose sole opportunity to implement a trait you can
//! expend to satisfy the way serde, etc, are structured.
//!
//! Here's a sketch of how that would look for serde's `Serialize`:
//!
//! ```
//! use hdrhistogram::Histogram;
//! use hdrhistogram::serialization::{Serializer, V2Serializer};
//!
//! mod serde {
//!     // part of serde, simplified
//!     pub trait Serializer {
//!        // ...
//!        fn serialize_bytes(self, value: &[u8]) -> Result<(), ()>;
//!        // ...
//!     }
//!
//!     // also in serde
//!     pub trait Serialize {
//!         fn serialize<S: Serializer>(&self, serializer: S) -> Result<(), ()>;
//!     }
//! }
//!
//! // your custom wrapper
//! #[allow(dead_code)] // to muffle warnings compiling this example
//! struct V2HistogramWrapper {
//!     histogram: Histogram<u64>
//! }
//!
//! impl serde::Serialize for V2HistogramWrapper {
//!     fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<(), ()> {
//!         // Not optimal to not re-use the vec and serializer, but it'll work
//!         let mut vec = Vec::new();
//!         // Pick the serialization format you want to use. Here, we use plain V2, but V2 +
//!         // DEFLATE is also available.
//!         // Map errors as appropriate for your use case.
//!         V2Serializer::new().serialize(&self.histogram, &mut vec)
//!             .map_err(|_| ())?;
//!         serializer.serialize_bytes(&vec)?;
//!         Ok(())
//!     }
//! }
//! ```
//!
//! # Examples
//!
//! Creating, serializing, and deserializing a single histogram using a `Vec<u8>` as a `Write` and a
//! `&[u8]` slice from the vec as a `Read`.
//!
//! ```
//! use hdrhistogram::Histogram;
//! use hdrhistogram::serialization::{Deserializer, Serializer, V2Serializer};
//!
//! let mut vec = Vec::new();
//! let orig_histogram = Histogram::<u64>::new(1).unwrap();
//! V2Serializer::new().serialize(&orig_histogram, &mut vec).unwrap();
//!
//! let _histogram: Histogram<u64> = Deserializer::new()
//!     .deserialize(&mut vec.as_slice()).unwrap();
//! ```
//!
//! This example shows serializing several histograms into a `Vec<u8>` and deserializing them again,
//! at which point they are summed into one histogram (for further hypothetical analysis).
//!
//! ```
//! use hdrhistogram::Histogram;
//! use hdrhistogram::serialization::{Deserializer, Serializer, V2Serializer};
//! use std::io::Cursor;
//!
//! // Naturally, do real error handling instead of unwrap() everywhere
//!
//! let num_histograms = 4;
//! let mut histograms = Vec::new();
//!
//! // Make some histograms
//! for _ in 0..num_histograms {
//!     let mut h = Histogram::<u64>::new_with_bounds(1, u64::max_value(), 3).unwrap();
//!     h.record_n(42, 7).unwrap();
//!     histograms.push(h);
//! }
//!
//! let mut buf = Vec::new();
//! let mut serializer = V2Serializer::new();
//!
//! // Save them to the buffer
//! for h in histograms.iter() {
//!     serializer.serialize(h, &mut buf).unwrap();
//! }
//!
//! // Read them back out again
//! let mut deserializer = Deserializer::new();
//! let mut cursor = Cursor::new(&buf);
//!
//! let mut accumulator =
//!     Histogram::<u64>::new_with_bounds(1, u64::max_value(), 3).unwrap();
//!
//! for _ in 0..num_histograms {
//!     let h: Histogram<u64> = deserializer.deserialize(&mut cursor).unwrap();
//!
//!     // behold, they are restored as they were originally
//!     assert_eq!(7, h.count_at(42));
//!     assert_eq!(0, h.count_at(1000));
//!
//!     accumulator.add(h).unwrap();
//! }
//!
//! // all the counts are there
//! assert_eq!(num_histograms * 7, accumulator.count_at(42));
//! ```
//!

use std::{fmt, io};

use super::{Counter, Histogram};

#[cfg(test)]
mod tests;

#[cfg(all(test, feature = "bench_private"))]
mod benchmarks;

mod v2_serializer;
pub use self::v2_serializer::{V2SerializeError, V2Serializer};

mod v2_deflate_serializer;
pub use self::v2_deflate_serializer::{V2DeflateSerializeError, V2DeflateSerializer};

mod deserializer;
pub use self::deserializer::{DeserializeError, Deserializer};

pub mod interval_log;

const V2_COOKIE_BASE: u32 = 0x1c84_9303;
const V2_COMPRESSED_COOKIE_BASE: u32 = 0x1c84_9304;

const V2_COOKIE: u32 = V2_COOKIE_BASE | 0x10;
const V2_COMPRESSED_COOKIE: u32 = V2_COMPRESSED_COOKIE_BASE | 0x10;

const V2_HEADER_SIZE: usize = 40;

/// Histogram serializer.
///
/// Different implementations serialize to different formats.
pub trait Serializer {
    /// Error type returned when serialization fails.
    type SerializeError: fmt::Debug;

    /// Serialize the histogram into the provided writer.
    /// Returns the number of bytes written, or an error.
    ///
    /// Note that `Vec<u8>` is a reasonable `Write` implementation for simple usage.
    fn serialize<T: Counter, W: io::Write>(
        &mut self,
        h: &Histogram<T>,
        writer: &mut W,
    ) -> Result<usize, Self::SerializeError>;
}
