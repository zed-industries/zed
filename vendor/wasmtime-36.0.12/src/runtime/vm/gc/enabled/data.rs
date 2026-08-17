//! Unchecked methods for working with the data inside GC objects.

use crate::V128;
use core::mem;

/// A plain-old-data type that can be stored in a `ValType` or a `StorageType`.
pub trait PodValType<const SIZE: usize>: Copy {
    /// Read an instance of `Self` from the given native-endian bytes.
    fn read_le(le_bytes: &[u8; SIZE]) -> Self;

    /// Write `self` into the given memory location, as native-endian bytes.
    fn write_le(&self, into: &mut [u8; SIZE]);
}

macro_rules! impl_pod_val_type {
    ( $( $t:ty , )* ) => {
        $(
            impl PodValType<{mem::size_of::<$t>()}> for $t {
                fn read_le(le_bytes: &[u8; mem::size_of::<$t>()]) -> Self {
                    <$t>::from_le_bytes(*le_bytes)
                }
                fn write_le(&self, into: &mut [u8; mem::size_of::<$t>()]) {
                    *into = self.to_le_bytes();
                }
            }
        )*
    };
}

impl_pod_val_type! {
    u8,
    u16,
    u32,
    u64,
    i8,
    i16,
    i32,
    i64,
}

impl PodValType<{ mem::size_of::<V128>() }> for V128 {
    fn read_le(le_bytes: &[u8; mem::size_of::<V128>()]) -> Self {
        u128::from_le_bytes(*le_bytes).into()
    }
    fn write_le(&self, into: &mut [u8; mem::size_of::<V128>()]) {
        *into = self.as_u128().to_le_bytes();
    }
}

/// The backing storage for a GC-managed object.
///
/// Methods on this type do not, generally, check against things like type
/// mismatches or that the given offset to read from even falls on a field
/// boundary. Omitting these checks is memory safe, due to our untrusted,
/// indexed GC heaps. Providing incorrect offsets will result in general
/// incorrectness, such as wrong answers or even panics, however.
///
/// Finally, these methods *will* panic on out-of-bounds accesses, either out of
/// the GC heap's bounds or out of this object's bounds. The former is necessary
/// for preserving the memory safety of indexed GC heaps in the face of (for
/// example) collector bugs, but the latter is just a defensive technique to
/// catch bugs early and prevent action at a distance as much as possible.
#[repr(transparent)]
pub struct VMGcObjectData {
    data: [u8],
}

macro_rules! impl_pod_methods {
    ( $( $T:ty, $read:ident, $write:ident; )* ) => {
        $(
            /// Read a `
            #[doc = stringify!($T)]
            /// ` field this object.
            ///
            /// Panics on out-of-bounds accesses.
            #[inline]
            pub fn $read(&self, offset: u32) -> $T
            {
                self.read_pod::<{ mem::size_of::<$T>() }, $T>(offset)
            }

            /// Write a `
            #[doc = stringify!($T)]
            /// ` into this object.
            ///
            /// Panics on out-of-bounds accesses.
            #[inline]
            pub fn $write(&mut self, offset: u32, val: $T)
            {
                self.write_pod::<{ mem::size_of::<$T>() }, $T>(offset, val);
            }
        )*
    };
}

impl<'a> From<&'a [u8]> for &'a VMGcObjectData {
    #[inline]
    fn from(data: &'a [u8]) -> Self {
        &VMGcObjectData::from_slice(data)
    }
}

impl<'a> From<&'a mut [u8]> for &'a mut VMGcObjectData {
    #[inline]
    fn from(data: &'a mut [u8]) -> Self {
        VMGcObjectData::from_slice_mut(data)
    }
}

impl VMGcObjectData {
    /// Construct a `VMGcObjectData` from the given slice of bytes.
    #[inline]
    pub fn from_slice(data: &[u8]) -> &Self {
        unsafe { mem::transmute(data) }
    }

    /// Construct a `VMGcObjectData` from the given slice of bytes.
    #[inline]
    pub fn from_slice_mut(data: &mut [u8]) -> &mut Self {
        unsafe { mem::transmute(data) }
    }

    /// Read a POD field out of this object.
    ///
    /// Panics on out-of-bounds accesses.
    ///
    /// Don't generally use this method, use `read_u8`, `read_i64`,
    /// etc... instead.
    #[inline]
    fn read_pod<const N: usize, T>(&self, offset: u32) -> T
    where
        T: PodValType<N>,
    {
        assert_eq!(N, mem::size_of::<T>());
        let offset = usize::try_from(offset).unwrap();
        let end = offset.checked_add(N).unwrap();
        let bytes = self.data.get(offset..end).expect("out of bounds field");
        T::read_le(bytes.try_into().unwrap())
    }

    /// Read a POD field out of this object.
    ///
    /// Panics on out-of-bounds accesses.
    ///
    /// Don't generally use this method, use `write_u8`, `write_i64`,
    /// etc... instead.
    #[inline]
    fn write_pod<const N: usize, T>(&mut self, offset: u32, val: T)
    where
        T: PodValType<N>,
    {
        assert_eq!(N, mem::size_of::<T>());
        let offset = usize::try_from(offset).unwrap();
        let end = offset.checked_add(N).unwrap();
        let into = match self.data.get_mut(offset..end) {
            Some(into) => into,
            None => panic!(
                "out of bounds field! field range = {offset:#x}..{end:#x}; object len = {:#x}",
                self.data.as_mut().len(),
            ),
        };
        val.write_le(into.try_into().unwrap());
    }

    /// Get a slice of this object's data.
    ///
    /// Note that GC data is always stored in little-endian order, and this
    /// method does not do any conversions to/from host endianness for you.
    ///
    /// Panics on out-of-bounds accesses.
    #[inline]
    pub fn slice(&self, offset: u32, len: u32) -> &[u8] {
        let start = usize::try_from(offset).unwrap();
        let len = usize::try_from(len).unwrap();
        let end = start.checked_add(len).unwrap();
        self.data.get(start..end).expect("out of bounds slice")
    }

    /// Get a mutable slice of this object's data.
    ///
    /// Note that GC data is always stored in little-endian order, and this
    /// method does not do any conversions to/from host endianness for you.
    ///
    /// Panics on out-of-bounds accesses.
    #[inline]
    pub fn slice_mut(&mut self, offset: u32, len: u32) -> &mut [u8] {
        let start = usize::try_from(offset).unwrap();
        let len = usize::try_from(len).unwrap();
        let end = start.checked_add(len).unwrap();
        self.data.get_mut(start..end).expect("out of bounds slice")
    }

    /// Copy the given slice into this object's data at the given offset.
    ///
    /// Note that GC data is always stored in little-endian order, and this
    /// method does not do any conversions to/from host endianness for you.
    ///
    /// Panics on out-of-bounds accesses.
    #[inline]
    pub fn copy_from_slice(&mut self, offset: u32, src: &[u8]) {
        let offset = usize::try_from(offset).unwrap();
        let end = offset.checked_add(src.len()).unwrap();
        let into = self.data.get_mut(offset..end).expect("out of bounds copy");
        into.copy_from_slice(src);
    }

    impl_pod_methods! {
        u8, read_u8, write_u8;
        u16, read_u16, write_u16;
        u32, read_u32, write_u32;
        u64, read_u64, write_u64;
        i8, read_i8, write_i8;
        i16, read_i16, write_i16;
        i32, read_i32, write_i32;
        i64, read_i64, write_i64;
        V128, read_v128, write_v128;
    }
}
