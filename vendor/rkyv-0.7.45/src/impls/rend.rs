use crate::{rend::*, Archive, Archived, Deserialize, Fallible, Serialize};
#[cfg(has_atomics)]
use core::sync::atomic::Ordering;

macro_rules! impl_rend_primitive {
    ($type:ty) => {
        impl Archive for $type {
            type Archived = Self;
            type Resolver = ();

            #[inline]
            unsafe fn resolve(&self, _: usize, _: Self::Resolver, out: *mut Self::Archived) {
                out.write(*self);
            }
        }

        // Safety: rend primitives always have the same representation archived and unarchived and
        // contain no padding
        #[cfg(feature = "copy")]
        unsafe impl crate::copy::ArchiveCopySafe for $type {}

        impl<S: Fallible + ?Sized> Serialize<S> for $type {
            #[inline]
            fn serialize(&self, _: &mut S) -> Result<Self::Resolver, S::Error> {
                Ok(())
            }
        }

        impl<D: Fallible + ?Sized> Deserialize<$type, D> for Archived<$type> {
            #[inline]
            fn deserialize(&self, _: &mut D) -> Result<$type, D::Error> {
                Ok(*self)
            }
        }
    };
}

#[cfg(has_atomics)]
macro_rules! impl_rend_atomic {
    ($type:ty, $prim:ty) => {
        impl Archive for $type {
            type Archived = $prim;
            type Resolver = ();

            #[inline]
            unsafe fn resolve(&self, _: usize, _: Self::Resolver, out: *mut Self::Archived) {
                out.write(<$prim>::new(self.load(Ordering::Relaxed)));
            }
        }

        impl<S: Fallible + ?Sized> Serialize<S> for $type {
            #[inline]
            fn serialize(&self, _: &mut S) -> Result<Self::Resolver, S::Error> {
                Ok(())
            }
        }

        impl<D: Fallible + ?Sized> Deserialize<$type, D> for $prim {
            #[inline]
            fn deserialize(&self, _: &mut D) -> Result<$type, D::Error> {
                Ok(self.value().into())
            }
        }
    };
}

impl_rend_primitive!(i16_be);
impl_rend_primitive!(i32_be);
impl_rend_primitive!(i64_be);
impl_rend_primitive!(i128_be);
impl_rend_primitive!(u16_be);
impl_rend_primitive!(u32_be);
impl_rend_primitive!(u64_be);
impl_rend_primitive!(u128_be);

impl_rend_primitive!(f32_be);
impl_rend_primitive!(f64_be);

impl_rend_primitive!(char_be);

impl_rend_primitive!(NonZeroI16_be);
impl_rend_primitive!(NonZeroI32_be);
impl_rend_primitive!(NonZeroI64_be);
impl_rend_primitive!(NonZeroI128_be);
impl_rend_primitive!(NonZeroU16_be);
impl_rend_primitive!(NonZeroU32_be);
impl_rend_primitive!(NonZeroU64_be);
impl_rend_primitive!(NonZeroU128_be);

#[cfg(has_atomics)]
impl_rend_atomic!(AtomicI16_be, i16_be);
#[cfg(has_atomics)]
impl_rend_atomic!(AtomicI32_be, i32_be);
#[cfg(has_atomics_64)]
impl_rend_atomic!(AtomicI64_be, i64_be);
#[cfg(has_atomics)]
impl_rend_atomic!(AtomicU16_be, u16_be);
#[cfg(has_atomics)]
impl_rend_atomic!(AtomicU32_be, u32_be);
#[cfg(has_atomics_64)]
impl_rend_atomic!(AtomicU64_be, u64_be);

impl_rend_primitive!(i16_le);
impl_rend_primitive!(i32_le);
impl_rend_primitive!(i64_le);
impl_rend_primitive!(i128_le);
impl_rend_primitive!(u16_le);
impl_rend_primitive!(u32_le);
impl_rend_primitive!(u64_le);
impl_rend_primitive!(u128_le);

impl_rend_primitive!(f32_le);
impl_rend_primitive!(f64_le);

impl_rend_primitive!(char_le);

impl_rend_primitive!(NonZeroI16_le);
impl_rend_primitive!(NonZeroI32_le);
impl_rend_primitive!(NonZeroI64_le);
impl_rend_primitive!(NonZeroI128_le);
impl_rend_primitive!(NonZeroU16_le);
impl_rend_primitive!(NonZeroU32_le);
impl_rend_primitive!(NonZeroU64_le);
impl_rend_primitive!(NonZeroU128_le);

#[cfg(has_atomics)]
impl_rend_atomic!(AtomicI16_le, i16_le);
#[cfg(has_atomics)]
impl_rend_atomic!(AtomicI32_le, i32_le);
#[cfg(has_atomics_64)]
impl_rend_atomic!(AtomicI64_le, i64_le);
#[cfg(has_atomics)]
impl_rend_atomic!(AtomicU16_le, u16_le);
#[cfg(has_atomics)]
impl_rend_atomic!(AtomicU32_le, u32_le);
#[cfg(has_atomics_64)]
impl_rend_atomic!(AtomicU64_le, u64_le);

#[cfg(test)]
mod tests {
    use crate::{
        archived_root, ser::serializers::CoreSerializer, ser::Serializer, Deserialize, Infallible,
        Serialize,
    };
    use core::fmt;

    type DefaultSerializer = CoreSerializer<256, 256>;

    fn test_archive<T>(value: &T)
    where
        T: fmt::Debug + PartialEq + Serialize<DefaultSerializer>,
        T::Archived: fmt::Debug + PartialEq<T> + Deserialize<T, Infallible>,
    {
        let mut serializer = DefaultSerializer::default();
        serializer
            .serialize_value(value)
            .expect("failed to archive value");
        let len = serializer.pos();
        let buffer = serializer.into_serializer().into_inner();

        let archived_value = unsafe { archived_root::<T>(&buffer[0..len]) };
        assert_eq!(archived_value, value);
        let mut deserializer = Infallible;
        assert_eq!(
            &archived_value.deserialize(&mut deserializer).unwrap(),
            value
        );
    }

    #[test]
    fn archive_rend() {
        use crate::rend::*;

        test_archive(&f32_be::new(1234567f32));
        test_archive(&f64_be::new(12345678901234f64));
        test_archive(&i16_be::new(12345i16));
        test_archive(&i32_be::new(1234567890i32));
        test_archive(&i64_be::new(1234567890123456789i64));
        test_archive(&i128_be::new(123456789012345678901234567890123456789i128));
        test_archive(&u16_be::new(12345u16));
        test_archive(&u32_be::new(1234567890u32));
        test_archive(&u64_be::new(12345678901234567890u64));
        test_archive(&u128_be::new(123456789012345678901234567890123456789u128));

        test_archive(&f32_le::new(1234567f32));
        test_archive(&f64_le::new(12345678901234f64));
        test_archive(&i16_le::new(12345i16));
        test_archive(&i32_le::new(1234567890i32));
        test_archive(&i64_le::new(1234567890123456789i64));
        test_archive(&i128_le::new(123456789012345678901234567890123456789i128));
        test_archive(&u16_le::new(12345u16));
        test_archive(&u32_le::new(1234567890u32));
        test_archive(&u64_le::new(12345678901234567890u64));
        test_archive(&u128_le::new(123456789012345678901234567890123456789u128));
    }

    #[test]
    fn archive_rend_endianness() {
        // Check representations to make sure endianness is preserved
        use crate::{
            rend::{BigEndian, LittleEndian},
            ser::Serializer,
        };

        // Big endian
        let value = BigEndian::<i32>::new(0x12345678);

        let mut serializer = DefaultSerializer::default();
        serializer.serialize_value(&value).unwrap();
        let buf = serializer.into_serializer().into_inner();

        assert_eq!(&buf[0..4], &[0x12, 0x34, 0x56, 0x78]);

        // Little endian
        let value = LittleEndian::<i32>::new(0x12345678i32);

        let mut serializer = DefaultSerializer::default();
        serializer.serialize_value(&value).unwrap();
        let buf = serializer.into_serializer().into_inner();

        assert_eq!(&buf[0..4], &[0x78, 0x56, 0x34, 0x12]);
    }

    #[test]
    fn archive_rend_nonzero() {
        use crate::rend::*;
        use core::num::{
            NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroU128, NonZeroU16, NonZeroU32,
            NonZeroU64,
        };

        unsafe {
            test_archive(&NonZeroI16_be::new(NonZeroI16::new_unchecked(12345)));
            test_archive(&NonZeroI32_be::new(NonZeroI32::new_unchecked(1234567890)));
            test_archive(&NonZeroI64_be::new(NonZeroI64::new_unchecked(
                1234567890123456789,
            )));
            test_archive(&NonZeroI128_be::new(NonZeroI128::new_unchecked(
                123456789012345678901234567890123456789,
            )));
            test_archive(&NonZeroU16_be::new(NonZeroU16::new_unchecked(12345)));
            test_archive(&NonZeroU32_be::new(NonZeroU32::new_unchecked(1234567890)));
            test_archive(&NonZeroU64_be::new(NonZeroU64::new_unchecked(
                1234567890123456789,
            )));
            test_archive(&NonZeroU128_be::new(NonZeroU128::new_unchecked(
                123456789012345678901234567890123456789,
            )));

            test_archive(&NonZeroI16_le::new(NonZeroI16::new_unchecked(12345)));
            test_archive(&NonZeroI32_le::new(NonZeroI32::new_unchecked(1234567890)));
            test_archive(&NonZeroI64_le::new(NonZeroI64::new_unchecked(
                1234567890123456789,
            )));
            test_archive(&NonZeroI128_le::new(NonZeroI128::new_unchecked(
                123456789012345678901234567890123456789,
            )));
            test_archive(&NonZeroU16_le::new(NonZeroU16::new_unchecked(12345)));
            test_archive(&NonZeroU32_le::new(NonZeroU32::new_unchecked(1234567890)));
            test_archive(&NonZeroU64_le::new(NonZeroU64::new_unchecked(
                1234567890123456789,
            )));
            test_archive(&NonZeroU128_le::new(NonZeroU128::new_unchecked(
                123456789012345678901234567890123456789,
            )));
        }
    }
}
