//! `bincode` uses a Builder-pattern to configure the Serializers and Deserializers in this
//! crate. This means that if you need to customize the behavior of `bincode`, you should create an
//! instance of the `DefaultOptions` struct:
//!
//! ```rust
//! use bincode::Options;
//! let my_options = bincode::DefaultOptions::new();
//! ```
//!
//! # Options Struct vs bincode functions
//!
//! Due to historical reasons, the default options used by the `serialize()` and `deserialize()`
//! family of functions are different than the default options created by the `DefaultOptions` struct:
//!
//! |          | Byte limit | Endianness | Int Encoding | Trailing Behavior |
//! |----------|------------|------------|--------------|-------------------|
//! | struct   | Unlimited  | Little     | Varint       | Reject            |
//! | function | Unlimited  | Little     | Fixint       | Allow             |
//!
//! This means that if you want to use the `Serialize` / `Deserialize` structs with the same
//! settings as the functions, you should adjust the `DefaultOptions` struct like so:
//!
//! ```rust
//! use bincode::Options;
//! let my_options = bincode::DefaultOptions::new()
//!     .with_fixint_encoding()
//!     .allow_trailing_bytes();
//! ```

use de::read::BincodeRead;
use error::Result;
use serde;
use std::io::{Read, Write};
use std::marker::PhantomData;

pub(crate) use self::endian::BincodeByteOrder;
pub(crate) use self::int::IntEncoding;
pub(crate) use self::internal::*;
pub(crate) use self::limit::SizeLimit;
pub(crate) use self::trailing::TrailingBytes;

pub use self::endian::{BigEndian, LittleEndian, NativeEndian};
pub use self::int::{FixintEncoding, VarintEncoding};
pub use self::legacy::*;
pub use self::limit::{Bounded, Infinite};
pub use self::trailing::{AllowTrailing, RejectTrailing};

mod endian;
mod int;
mod legacy;
mod limit;
mod trailing;

/// The default options for bincode serialization/deserialization.
///
/// ### Defaults
/// By default bincode will use little-endian encoding for multi-byte integers, and will not
/// limit the number of serialized/deserialized bytes.
///
/// ### Configuring `DefaultOptions`
///
/// `DefaultOptions` implements the [Options] trait, which means it exposes functions to change the behavior of bincode.
///
/// For example, if you wanted to limit the bincode deserializer to 1 kilobyte of user input:
///
/// ```rust
/// use bincode::Options;
/// let my_options = bincode::DefaultOptions::new().with_limit(1024);
/// ```
///
/// ### DefaultOptions struct vs. functions
///
/// The default configuration used by this struct is not the same as that used by the bincode
/// helper functions in the root of this crate. See the
/// [config](index.html#options-struct-vs-bincode-functions) module for more details
#[derive(Copy, Clone)]
pub struct DefaultOptions(Infinite);

impl DefaultOptions {
    /// Get a default configuration object.
    ///
    /// ### Default Configuration:
    ///
    /// | Byte limit | Endianness | Int Encoding | Trailing Behavior |
    /// |------------|------------|--------------|-------------------|
    /// | Unlimited  | Little     | Varint       | Reject            |
    pub fn new() -> DefaultOptions {
        DefaultOptions(Infinite)
    }
}

impl Default for DefaultOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl InternalOptions for DefaultOptions {
    type Limit = Infinite;
    type Endian = LittleEndian;
    type IntEncoding = VarintEncoding;
    type Trailing = RejectTrailing;

    #[inline(always)]
    fn limit(&mut self) -> &mut Infinite {
        &mut self.0
    }
}

/// A configuration builder trait whose options Bincode will use
/// while serializing and deserializing.
///
/// ### Options
/// Endianness: The endianness with which multi-byte integers will be read/written.  *default: little endian*
///
/// Limit: The maximum number of bytes that will be read/written in a bincode serialize/deserialize. *default: unlimited*
///
/// Int Encoding: The encoding used for numbers, enum discriminants, and lengths. *default: varint*
///
/// Trailing Behavior: The behavior when there are trailing bytes left over in a slice after deserialization. *default: reject*
///
/// ### Byte Limit Details
/// The purpose of byte-limiting is to prevent Denial-Of-Service attacks whereby malicious attackers get bincode
/// deserialization to crash your process by allocating too much memory or keeping a connection open for too long.
///
/// When a byte limit is set, bincode will return `Err` on any deserialization that goes over the limit, or any
/// serialization that goes over the limit.
pub trait Options: InternalOptions + Sized {
    /// Sets the byte limit to be unlimited.
    /// This is the default.
    fn with_no_limit(self) -> WithOtherLimit<Self, Infinite> {
        WithOtherLimit::new(self, Infinite)
    }

    /// Sets the byte limit to `limit`.
    fn with_limit(self, limit: u64) -> WithOtherLimit<Self, Bounded> {
        WithOtherLimit::new(self, Bounded(limit))
    }

    /// Sets the endianness to little-endian
    /// This is the default.
    fn with_little_endian(self) -> WithOtherEndian<Self, LittleEndian> {
        WithOtherEndian::new(self)
    }

    /// Sets the endianness to big-endian
    fn with_big_endian(self) -> WithOtherEndian<Self, BigEndian> {
        WithOtherEndian::new(self)
    }

    /// Sets the endianness to the the machine-native endianness
    fn with_native_endian(self) -> WithOtherEndian<Self, NativeEndian> {
        WithOtherEndian::new(self)
    }

    /// Sets the length encoding to varint
    fn with_varint_encoding(self) -> WithOtherIntEncoding<Self, VarintEncoding> {
        WithOtherIntEncoding::new(self)
    }

    /// Sets the length encoding to be fixed
    fn with_fixint_encoding(self) -> WithOtherIntEncoding<Self, FixintEncoding> {
        WithOtherIntEncoding::new(self)
    }

    /// Sets the deserializer to reject trailing bytes
    fn reject_trailing_bytes(self) -> WithOtherTrailing<Self, RejectTrailing> {
        WithOtherTrailing::new(self)
    }

    /// Sets the deserializer to allow trailing bytes
    fn allow_trailing_bytes(self) -> WithOtherTrailing<Self, AllowTrailing> {
        WithOtherTrailing::new(self)
    }

    /// Serializes a serializable object into a `Vec` of bytes using this configuration
    #[inline(always)]
    fn serialize<S: ?Sized + serde::Serialize>(self, t: &S) -> Result<Vec<u8>> {
        ::internal::serialize(t, self)
    }

    /// Returns the size that an object would be if serialized using Bincode with this configuration
    #[inline(always)]
    fn serialized_size<T: ?Sized + serde::Serialize>(self, t: &T) -> Result<u64> {
        ::internal::serialized_size(t, self)
    }

    /// Serializes an object directly into a `Writer` using this configuration
    ///
    /// If the serialization would take more bytes than allowed by the size limit, an error
    /// is returned and *no bytes* will be written into the `Writer`
    #[inline(always)]
    fn serialize_into<W: Write, T: ?Sized + serde::Serialize>(self, w: W, t: &T) -> Result<()> {
        ::internal::serialize_into(w, t, self)
    }

    /// Deserializes a slice of bytes into an instance of `T` using this configuration
    #[inline(always)]
    fn deserialize<'a, T: serde::Deserialize<'a>>(self, bytes: &'a [u8]) -> Result<T> {
        ::internal::deserialize(bytes, self)
    }

    /// TODO: document
    #[doc(hidden)]
    #[inline(always)]
    fn deserialize_in_place<'a, R, T>(self, reader: R, place: &mut T) -> Result<()>
    where
        R: BincodeRead<'a>,
        T: serde::de::Deserialize<'a>,
    {
        ::internal::deserialize_in_place(reader, self, place)
    }

    /// Deserializes a slice of bytes with state `seed` using this configuration.
    #[inline(always)]
    fn deserialize_seed<'a, T: serde::de::DeserializeSeed<'a>>(
        self,
        seed: T,
        bytes: &'a [u8],
    ) -> Result<T::Value> {
        ::internal::deserialize_seed(seed, bytes, self)
    }

    /// Deserializes an object directly from a `Read`er using this configuration
    ///
    /// If this returns an `Error`, `reader` may be in an invalid state.
    #[inline(always)]
    fn deserialize_from<R: Read, T: serde::de::DeserializeOwned>(self, reader: R) -> Result<T> {
        ::internal::deserialize_from(reader, self)
    }

    /// Deserializes an object directly from a `Read`er with state `seed` using this configuration
    ///
    /// If this returns an `Error`, `reader` may be in an invalid state.
    #[inline(always)]
    fn deserialize_from_seed<'a, R: Read, T: serde::de::DeserializeSeed<'a>>(
        self,
        seed: T,
        reader: R,
    ) -> Result<T::Value> {
        ::internal::deserialize_from_seed(seed, reader, self)
    }

    /// Deserializes an object from a custom `BincodeRead`er using the default configuration.
    /// It is highly recommended to use `deserialize_from` unless you need to implement
    /// `BincodeRead` for performance reasons.
    ///
    /// If this returns an `Error`, `reader` may be in an invalid state.
    #[inline(always)]
    fn deserialize_from_custom<'a, R: BincodeRead<'a>, T: serde::de::DeserializeOwned>(
        self,
        reader: R,
    ) -> Result<T> {
        ::internal::deserialize_from_custom(reader, self)
    }

    /// Deserializes an object from a custom `BincodeRead`er with state `seed` using the default
    /// configuration. It is highly recommended to use `deserialize_from` unless you need to
    /// implement `BincodeRead` for performance reasons.
    ///
    /// If this returns an `Error`, `reader` may be in an invalid state.
    #[inline(always)]
    fn deserialize_from_custom_seed<'a, R: BincodeRead<'a>, T: serde::de::DeserializeSeed<'a>>(
        self,
        seed: T,
        reader: R,
    ) -> Result<T::Value> {
        ::internal::deserialize_from_custom_seed(seed, reader, self)
    }
}

impl<T: InternalOptions> Options for T {}

/// A configuration struct with a user-specified byte limit
#[derive(Clone, Copy)]
pub struct WithOtherLimit<O: Options, L: SizeLimit> {
    _options: O,
    pub(crate) new_limit: L,
}

/// A configuration struct with a user-specified endian order
#[derive(Clone, Copy)]
pub struct WithOtherEndian<O: Options, E: BincodeByteOrder> {
    options: O,
    _endian: PhantomData<E>,
}

/// A configuration struct with a user-specified length encoding
#[derive(Clone, Copy)]
pub struct WithOtherIntEncoding<O: Options, I: IntEncoding> {
    options: O,
    _length: PhantomData<I>,
}

/// A configuration struct with a user-specified trailing bytes behavior.
#[derive(Clone, Copy)]
pub struct WithOtherTrailing<O: Options, T: TrailingBytes> {
    options: O,
    _trailing: PhantomData<T>,
}

impl<O: Options, L: SizeLimit> WithOtherLimit<O, L> {
    #[inline(always)]
    pub(crate) fn new(options: O, limit: L) -> WithOtherLimit<O, L> {
        WithOtherLimit {
            _options: options,
            new_limit: limit,
        }
    }
}

impl<O: Options, E: BincodeByteOrder> WithOtherEndian<O, E> {
    #[inline(always)]
    pub(crate) fn new(options: O) -> WithOtherEndian<O, E> {
        WithOtherEndian {
            options,
            _endian: PhantomData,
        }
    }
}

impl<O: Options, I: IntEncoding> WithOtherIntEncoding<O, I> {
    #[inline(always)]
    pub(crate) fn new(options: O) -> WithOtherIntEncoding<O, I> {
        WithOtherIntEncoding {
            options,
            _length: PhantomData,
        }
    }
}

impl<O: Options, T: TrailingBytes> WithOtherTrailing<O, T> {
    #[inline(always)]
    pub(crate) fn new(options: O) -> WithOtherTrailing<O, T> {
        WithOtherTrailing {
            options,
            _trailing: PhantomData,
        }
    }
}

impl<O: Options, E: BincodeByteOrder + 'static> InternalOptions for WithOtherEndian<O, E> {
    type Limit = O::Limit;
    type Endian = E;
    type IntEncoding = O::IntEncoding;
    type Trailing = O::Trailing;
    #[inline(always)]
    fn limit(&mut self) -> &mut O::Limit {
        self.options.limit()
    }
}

impl<O: Options, L: SizeLimit + 'static> InternalOptions for WithOtherLimit<O, L> {
    type Limit = L;
    type Endian = O::Endian;
    type IntEncoding = O::IntEncoding;
    type Trailing = O::Trailing;
    fn limit(&mut self) -> &mut L {
        &mut self.new_limit
    }
}

impl<O: Options, I: IntEncoding + 'static> InternalOptions for WithOtherIntEncoding<O, I> {
    type Limit = O::Limit;
    type Endian = O::Endian;
    type IntEncoding = I;
    type Trailing = O::Trailing;

    fn limit(&mut self) -> &mut O::Limit {
        self.options.limit()
    }
}

impl<O: Options, T: TrailingBytes + 'static> InternalOptions for WithOtherTrailing<O, T> {
    type Limit = O::Limit;
    type Endian = O::Endian;
    type IntEncoding = O::IntEncoding;
    type Trailing = T;

    fn limit(&mut self) -> &mut O::Limit {
        self.options.limit()
    }
}

mod internal {
    use super::*;

    pub trait InternalOptions {
        type Limit: SizeLimit + 'static;
        type Endian: BincodeByteOrder + 'static;
        type IntEncoding: IntEncoding + 'static;
        type Trailing: TrailingBytes + 'static;

        fn limit(&mut self) -> &mut Self::Limit;
    }

    impl<'a, O: InternalOptions> InternalOptions for &'a mut O {
        type Limit = O::Limit;
        type Endian = O::Endian;
        type IntEncoding = O::IntEncoding;
        type Trailing = O::Trailing;

        #[inline(always)]
        fn limit(&mut self) -> &mut Self::Limit {
            (*self).limit()
        }
    }
}
