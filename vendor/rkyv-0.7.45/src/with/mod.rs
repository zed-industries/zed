//! Wrapper type support and commonly used wrappers.
//!
//! Wrappers can be applied with the `#[with(...)]` attribute in the
//! [`Archive`](macro@crate::Archive) macro. See [`With`] for examples.

#[cfg(feature = "alloc")]
mod alloc;
#[cfg(has_atomics)]
mod atomic;
mod core;
#[cfg(feature = "std")]
mod std;

use crate::{Archive, Deserialize, Fallible, Serialize};
use ::core::{fmt, marker::PhantomData, mem::transmute, ops::Deref};

/// A transparent wrapper for archived fields.
///
/// This is used by the `#[with(...)]` attribute in the [`Archive`](macro@crate::Archive) macro to
/// create transparent serialization wrappers. Those wrappers leverage [`ArchiveWith`] to change
/// how the type is archived, serialized, and deserialized.
///
/// When a field is serialized, a reference to the field (i.e. `&T`) can be cast to a reference to a
/// wrapping `With` (i.e. `With<T, Wrapper>`) and serialized instead. This is safe to do because
/// `With` is a transparent wrapper and is shaped exactly the same as the underlying field.
///
/// # Example
///
/// ```
/// use rkyv::{Archive, with::Inline};
///
/// #[derive(Archive)]
/// struct Example<'a> {
///     // This will archive as if it were With<&'a i32, Inline>. That will delegate the archival
///     // to the ArchiveWith implementation of Inline for &T.
///     #[with(Inline)]
///     a: &'a i32,
/// }
/// ```
#[repr(transparent)]
#[derive(Debug)]
pub struct With<F: ?Sized, W> {
    _phantom: PhantomData<W>,
    field: F,
}

impl<F: ?Sized, W> With<F, W> {
    /// Casts a `With` reference from a reference to the underlying field.
    ///
    /// This is always safe to do because `With` is a transparent wrapper.
    #[inline]
    pub fn cast(field: &F) -> &'_ With<F, W> {
        // Safety: transmuting from an unsized type reference to a reference to a transparent
        // wrapper is safe because they both have the same data address and metadata
        #[allow(clippy::transmute_ptr_to_ptr)]
        unsafe {
            transmute(field)
        }
    }
}

impl<F, W> With<F, W> {
    /// Unwraps a `With` into the underlying field.
    #[inline]
    pub fn into_inner(self) -> F {
        self.field
    }
}

impl<F: ?Sized, W> AsRef<F> for With<F, W> {
    fn as_ref(&self) -> &F {
        &self.field
    }
}

/// A variant of [`Archive`] that works with [`With`] wrappers.
///
/// Creating a wrapper allows users to customize how fields are archived easily without changing the
/// unarchived type.
///
/// This trait allows wrapper types to transparently change the archive behaviors for struct fields.
/// When a field is serialized, its reference may be converted to a [`With`] reference, and that
/// reference may be serialized instead. `With` references look for implementations of `ArchiveWith`
/// to determine how a wrapped field should be treated.
///
/// # Example
///
/// ```
/// use rkyv::{
///     archived_root,
///     ser::{
///         serializers::AllocSerializer,
///         Serializer,
///     },
///     with::{
///         ArchiveWith,
///         DeserializeWith,
///         SerializeWith,
///     },
///     Archive,
///     Archived,
///     Deserialize,
///     Fallible,
///     Infallible,
///     Resolver,
///     Serialize,
/// };
///
/// struct Incremented;
///
/// impl ArchiveWith<i32> for Incremented {
///     type Archived = Archived<i32>;
///     type Resolver = Resolver<i32>;
///
///     unsafe fn resolve_with(field: &i32, pos: usize, _: (), out: *mut Self::Archived) {
///         let incremented = field + 1;
///         incremented.resolve(pos, (), out);
///     }
/// }
///
/// impl<S: Fallible + ?Sized> SerializeWith<i32, S> for Incremented
/// where
///     i32: Serialize<S>,
/// {
///     fn serialize_with(field: &i32, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
///         let incremented = field + 1;
///         incremented.serialize(serializer)
///     }
/// }
///
/// impl<D: Fallible + ?Sized> DeserializeWith<Archived<i32>, i32, D> for Incremented
/// where
///     Archived<i32>: Deserialize<i32, D>,
/// {
///     fn deserialize_with(field: &Archived<i32>, deserializer: &mut D) -> Result<i32, D::Error> {
///         Ok(field.deserialize(deserializer)? - 1)
///     }
/// }
///
/// #[derive(Archive, Deserialize, Serialize)]
/// struct Example {
///     #[with(Incremented)]
///     a: i32,
///     // Another i32 field, but not incremented this time
///     b: i32,
/// }
///
/// let value = Example {
///     a: 4,
///     b: 9,
/// };
///
/// let mut serializer = AllocSerializer::<4096>::default();
/// serializer.serialize_value(&value).unwrap();
/// let buf = serializer.into_serializer().into_inner();
///
/// let archived = unsafe { archived_root::<Example>(buf.as_ref()) };
/// // The wrapped field has been incremented
/// assert_eq!(archived.a, 5);
/// // ... and the unwrapped field has not
/// assert_eq!(archived.b, 9);
///
/// let deserialized: Example = archived.deserialize(&mut Infallible).unwrap();
/// // The wrapped field is back to normal
/// assert_eq!(deserialized.a, 4);
/// // ... and the unwrapped field is unchanged
/// assert_eq!(deserialized.b, 9);
/// ```
pub trait ArchiveWith<F: ?Sized> {
    /// The archived type of a `With<F, Self>`.
    type Archived;
    /// The resolver of a `With<F, Self>`.
    type Resolver;

    /// Resolves the archived type using a reference to the field type `F`.
    ///
    /// # Safety
    ///
    /// - `pos` must be the position of `out` within the archive
    /// - `resolver` must be the result of serializing `field`
    unsafe fn resolve_with(
        field: &F,
        pos: usize,
        resolver: Self::Resolver,
        out: *mut Self::Archived,
    );
}

impl<F: ?Sized, W: ArchiveWith<F>> Archive for With<F, W> {
    type Archived = W::Archived;
    type Resolver = W::Resolver;

    #[inline]
    unsafe fn resolve(&self, pos: usize, resolver: Self::Resolver, out: *mut Self::Archived) {
        W::resolve_with(&self.field, pos, resolver, out.cast());
    }
}

/// A variant of `Serialize` that works with `With` wrappers.
pub trait SerializeWith<F: ?Sized, S: Fallible + ?Sized>: ArchiveWith<F> {
    /// Serializes the field type `F` using the given serializer.
    fn serialize_with(field: &F, serializer: &mut S) -> Result<Self::Resolver, S::Error>;
}

impl<F: ?Sized, W: SerializeWith<F, S>, S: Fallible + ?Sized> Serialize<S> for With<F, W> {
    #[inline]
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        W::serialize_with(&self.field, serializer)
    }
}

/// A variant of `Deserialize` that works with `With` wrappers.
pub trait DeserializeWith<F: ?Sized, T, D: Fallible + ?Sized> {
    /// Deserializes the field type `F` using the given deserializer.
    fn deserialize_with(field: &F, deserializer: &mut D) -> Result<T, D::Error>;
}

impl<F, W, T, D> Deserialize<With<T, W>, D> for F
where
    F: ?Sized,
    W: DeserializeWith<F, T, D>,
    D: Fallible + ?Sized,
{
    #[inline]
    fn deserialize(&self, deserializer: &mut D) -> Result<With<T, W>, D::Error> {
        Ok(With {
            _phantom: PhantomData,
            field: W::deserialize_with(self, deserializer)?,
        })
    }
}

/// A wrapper to make a type immutable.
#[repr(transparent)]
#[derive(Debug)]
pub struct Immutable<T: ?Sized>(T);

impl<T: ?Sized> Immutable<T> {
    /// Gets the underlying immutable value.
    #[inline]
    pub fn value(&self) -> &T {
        &self.0
    }
}

impl<T: ?Sized> Deref for Immutable<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(feature = "validation")]
const _: () = {
    use bytecheck::CheckBytes;

    impl<T: CheckBytes<C> + ?Sized, C: ?Sized> CheckBytes<C> for Immutable<T> {
        type Error = T::Error;

        unsafe fn check_bytes<'a>(
            value: *const Self,
            context: &mut C,
        ) -> Result<&'a Self, Self::Error> {
            CheckBytes::check_bytes(::core::ptr::addr_of!((*value).0), context)?;
            Ok(&*value)
        }
    }
};

/// A generic wrapper that allows wrapping an `Option<T>`.
///
/// # Example
///
/// ```
/// use rkyv::{Archive, with::{Map, RefAsBox}};
///
/// #[derive(Archive)]
/// struct Example<'a> {
///     #[with(Map<RefAsBox>)]
///     option: Option<&'a i32>,
///     #[with(Map<RefAsBox>)]
///     vec: Vec<&'a i32>,
/// }
/// ```
#[derive(Debug)]
pub struct Map<Archivable> {
    _type: PhantomData<Archivable>,
}

/// A wrapper that archives an atomic with an underlying atomic.
///
/// By default, atomics are archived with an underlying integer.
///
/// # Safety
///
/// This wrapper is only safe to use when the backing memory for wrapped types is mutable.
///
/// # Example
///
/// ```
/// # #[cfg(has_atomics)]
/// use core::sync::atomic::AtomicU32;
/// use rkyv::{Archive, with::Atomic};
///
/// # #[cfg(has_atomics)]
/// #[derive(Archive)]
/// struct Example {
///     #[with(Atomic)]
///     a: AtomicU32,
/// }
/// ```
#[derive(Debug)]
pub struct Atomic;

/// A wrapper that serializes a reference inline.
///
/// References serialized with `Inline` cannot be deserialized because the struct cannot own the
/// deserialized value.
///
/// # Example
///
/// ```
/// use rkyv::{Archive, with::Inline};
///
/// #[derive(Archive)]
/// struct Example<'a> {
///     #[with(Inline)]
///     a: &'a i32,
/// }
/// ```
#[derive(Debug)]
pub struct Inline;

/// A wrapper that serializes a reference as if it were boxed.
///
/// Unlike [`Inline`], unsized references can be serialized with `Boxed`.
///
/// References serialized with `Boxed` cannot be deserialized because the struct cannot own the
/// deserialized value.
///
/// # Example
///
/// ```
/// use rkyv::{Archive, with::Boxed};
///
/// #[derive(Archive)]
/// struct Example<'a> {
///     #[with(Boxed)]
///     a: &'a str,
/// }
/// ```
#[deprecated = "Use `RefAsBox` for references, or `AsBox` for direct fields"]
pub type Boxed = RefAsBox;

/// A wrapper that serializes a field into a box.
///
/// This functions similarly to [`RefAsBox`], but is for regular fields instead of references.
///
/// # Example
///
/// ```
/// use rkyv::{Archive, with::AsBox};
///
/// #[derive(Archive)]
/// struct Example {
///     #[with(AsBox)]
///     a: i32,
///     #[with(AsBox)]
///     b: str,
/// }
/// ```
#[derive(Debug)]
pub struct AsBox;

/// A wrapper that serializes a reference as if it were boxed.
///
/// Unlike [`Inline`], unsized references can be serialized with `RefAsBox`.
///
/// References serialized with `RefAsBox` cannot be deserialized because the struct cannot own the
/// deserialized value.
///
/// # Example
///
/// ```
/// use rkyv::{Archive, with::RefAsBox};
///
/// #[derive(Archive)]
/// struct Example<'a> {
///     #[with(RefAsBox)]
///     a: &'a i32,
///     #[with(RefAsBox)]
///     b: &'a str,
/// }
/// ```
#[derive(Debug)]
pub struct RefAsBox;

/// A wrapper that attempts to convert a type to and from UTF-8.
///
/// Types like `OsString` and `PathBuf` aren't guaranteed to be encoded as UTF-8, but they usually
/// are anyway. Using this wrapper will archive them as if they were regular `String`s.
///
/// Regular serializers don't support the custom error handling needed for this type by default. To
/// use this wrapper, a custom serializer with an error type satisfying
/// `<S as Fallible>::Error: From<AsStringError>` must be provided.
///
/// # Example
///
/// ```
/// use std::{ffi::OsString, path::PathBuf};
/// use rkyv::{Archive, with::AsString};
///
/// #[derive(Archive)]
/// struct Example {
///     #[with(AsString)]
///     os_string: OsString,
///     #[with(AsString)]
///     path: PathBuf,
/// }
/// ```
#[derive(Debug)]
pub struct AsString;

/// Errors that can occur when serializing a [`AsString`] wrapper.
#[derive(Debug)]
pub enum AsStringError {
    /// The `OsString` or `PathBuf` was not valid UTF-8.
    InvalidUTF8,
}

impl fmt::Display for AsStringError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid UTF-8")
    }
}

#[cfg(feature = "std")]
impl ::std::error::Error for AsStringError {}

/// A wrapper that locks a lock and serializes the value immutably.
///
/// This wrapper can panic under very specific circumstances when:
///
/// 1. `serialize_with` is called and succeeds in locking the value to serialize it.
/// 2. Another thread locks the value and panics, poisoning the lock
/// 3. `resolve_with` is called and gets a poisoned value.
///
/// Unfortunately, it's not possible to work around this issue. If your code absolutely must not
/// panic under any circumstances, it's recommended that you lock your values and then serialize
/// them while locked.
///
/// Additionally, mutating the data protected by a mutex between the serialize and resolve steps may
/// cause undefined behavior in the resolve step. **Uses of this wrapper should be considered
/// unsafe** with the requirement that the data not be mutated between these two steps.
///
/// Regular serializers don't support the custom error handling needed for this type by default. To
/// use this wrapper, a custom serializer with an error type satisfying
/// `<S as Fallible>::Error: From<LockError>` must be provided.
///
/// # Example
///
/// ```
/// use std::sync::Mutex;
/// use rkyv::{Archive, with::Lock};
///
/// #[derive(Archive)]
/// struct Example {
///     #[with(Lock)]
///     a: Mutex<i32>,
/// }
/// ```
#[derive(Debug)]
pub struct Lock;

/// Errors that can occur while serializing a [`Lock`] wrapper
#[derive(Debug)]
pub enum LockError {
    /// The mutex was poisoned
    Poisoned,
}

impl fmt::Display for LockError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "lock poisoned")
    }
}

#[cfg(feature = "std")]
impl ::std::error::Error for LockError {}

/// A wrapper that serializes a `Cow` as if it were owned.
///
/// # Example
///
/// ```
/// use std::borrow::Cow;
/// use rkyv::{Archive, with::AsOwned};
///
/// #[derive(Archive)]
/// struct Example<'a> {
///     #[with(AsOwned)]
///     a: Cow<'a, str>,
/// }
/// ```
#[derive(Debug)]
pub struct AsOwned;

/// A wrapper that serializes associative containers as a `Vec` of key-value pairs.
///
/// This provides faster serialization for containers like `HashMap` and `BTreeMap` by serializing
/// the key-value pairs directly instead of building a data structure in the buffer.
///
/// # Example
///
/// ```
/// use std::collections::HashMap;
/// use rkyv::{Archive, with::AsVec};
///
/// #[derive(Archive)]
/// struct Example {
///     #[with(AsVec)]
///     values: HashMap<String, u32>,
/// }
/// ```
#[derive(Debug)]
pub struct AsVec;

/// A wrapper that niches some type combinations.
///
/// A common type combination is `Option<Box<T>>`. By using a null pointer, the archived version can
/// save some space on-disk.
///
/// # Example
///
/// ```
/// use core::mem::size_of;
/// use rkyv::{Archive, Archived, with::Niche};
///
/// #[derive(Archive)]
/// struct BasicExample {
///     value: Option<Box<str>>,
/// }
///
/// #[derive(Archive)]
/// struct NichedExample {
///     #[with(Niche)]
///     value: Option<Box<str>>,
/// }
///
/// assert!(size_of::<Archived<BasicExample>>() > size_of::<Archived<NichedExample>>());
/// ```
#[derive(Debug)]
pub struct Niche;

/// A wrapper that provides specialized, performant implementations of serialization and
/// deserialization.
///
/// This wrapper can be used with containers like `Vec`, but care must be taken to ensure that they
/// contain copy-safe types. Copy-safe types must be trivially copyable (have the same archived and
/// unarchived representations) and contain no padding bytes. In situations where copying
/// uninitialized bytes the output is acceptable, this wrapper may be used with containers of types
/// that contain padding bytes.
///
/// # Safety
///
/// Using this wrapper with containers containing non-copy-safe types may result in undefined
/// behavior.
///
/// # Example
///
/// ```
/// use rkyv::{Archive, with::CopyOptimize};
///
/// #[derive(Archive)]
/// struct Example {
///     #[with(CopyOptimize)]
///     bytes: Vec<u8>,
/// }
/// ```
#[derive(Debug)]
pub struct CopyOptimize;

/// A wrapper that converts a [`SystemTime`](::std::time::SystemTime) to a
/// [`Duration`](::std::time::Duration) since [`UNIX_EPOCH`](::std::time::UNIX_EPOCH).
///
/// If the serialized time occurs before the UNIX epoch, serialization will panic during `resolve`.
/// The resulting archived time will be an [`ArchivedDuration`](crate::time::ArchivedDuration)
/// relative to the UNIX epoch.
///
/// Regular serializers don't support the custom error handling needed for this type by default. To
/// use this wrapper, a custom serializer with an error type satisfying
/// `<S as Fallible>::Error: From<UnixTimestampError>` must be provided.
///
/// # Example
///
/// ```
/// use rkyv::{Archive, with::UnixTimestamp};
/// use std::time::SystemTime;
///
/// #[derive(Archive)]
/// struct Example {
///     #[with(UnixTimestamp)]
///     time: SystemTime,
/// }
#[derive(Debug)]
pub struct UnixTimestamp;

/// Errors that can occur when serializing a [`UnixTimestamp`] wrapper.
#[derive(Debug)]
pub enum UnixTimestampError {
    /// The `SystemTime` occurred prior to the UNIX epoch.
    TimeBeforeUnixEpoch,
}

impl fmt::Display for UnixTimestampError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "time occurred before the UNIX epoch")
    }
}

#[cfg(feature = "std")]
impl ::std::error::Error for UnixTimestampError {}

/// A wrapper that provides an optimized bulk data array. This is primarily intended for large
/// amounts of raw data, like bytes, floats, or integers.
///
/// This wrapper can be used with containers like `Vec`, but care must be taken to ensure that they
/// contain copy-safe types. Copy-safe types must be trivially copyable (have the same archived and
/// unarchived representations) and contain no padding bytes. In situations where copying
/// uninitialized bytes the output is acceptable, this wrapper may be used with containers of types
/// that contain padding bytes.
///
/// Unlike [`CopyOptimize`], this wrapper will also skip validation for its elements. If the
/// elements of the container can have any invalid bit patterns (e.g. `char`, `bool`, complex
/// containers, etc.), then using `Raw` in an insecure setting can lead to undefined behavior. Take
/// great caution!
///
/// # Safety
///
/// Using this wrapper with containers containing non-copy-safe types or types that require
/// validation may result in undefined behavior.
///
/// # Example
///
/// ```
/// use rkyv::{Archive, with::Raw};
///
/// #[derive(Archive)]
/// struct Example {
///     #[with(Raw)]
///     bytes: Vec<u8>,
///     #[with(Raw)]
///     vertices: Vec<[f32; 3]>,
/// }
/// ```
#[derive(Debug)]
pub struct Raw;

/// A wrapper that allows serialize-unsafe types to be serialized.
///
/// Types like `Cell` and `UnsafeCell` may contain serializable types, but have unsafe access
/// semantics due to interior mutability. They may be safe to serialize, but only under conditions
/// that rkyv is unable to guarantee.
///
/// This wrapper enables serializing these types, and places the burden of verifying that their
/// access semantics are used safely on the user.
///
/// # Safety
///
/// Using this wrapper on types with interior mutability can create races conditions or allow access
/// to data in an invalid state if access semantics are not followed properly. During serialization,
/// the data must not be modified.
///
/// # Example
///
/// ```
/// use rkyv::{Archive, with::Unsafe};
/// use core::cell::{Cell, UnsafeCell};
///
/// #[derive(Archive)]
/// struct Example {
///     #[with(Unsafe)]
///     cell: Cell<String>,
///     #[with(Unsafe)]
///     unsafe_cell: UnsafeCell<String>,
/// }
/// ```
#[derive(Debug)]
pub struct Unsafe;

/// A wrapper that skips serializing a field.
///
/// Skipped fields must implement `Default` to be deserialized.
///
/// # Example
///
/// ```
/// use rkyv::{Archive, with::Skip};
///
/// #[derive(Archive)]
/// struct Example {
///     #[with(Skip)]
///     a: u32,
/// }
/// ```
#[derive(Debug)]
pub struct Skip;
