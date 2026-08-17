use crate::{
    with::{ArchiveWith, Atomic, DeserializeWith, SerializeWith, With},
    Archived, Fallible,
};
use core::sync::atomic::{
    AtomicBool, AtomicI16, AtomicI32, AtomicI8, AtomicU16, AtomicU32, AtomicU8, Ordering,
};
#[cfg(has_atomics_64)]
use core::sync::atomic::{AtomicI64, AtomicU64};

macro_rules! impl_atomic {
    (@serialize_deserialize $type:ty) => {
        impl<S: Fallible + ?Sized> SerializeWith<$type, S> for Atomic {
            #[inline]
            fn serialize_with(_: &$type, _: &mut S) -> Result<Self::Resolver, S::Error> {
                Ok(())
            }
        }
    };
    ($type:ty) => {
        impl ArchiveWith<$type> for Atomic {
            type Archived = $type;
            type Resolver = ();

            #[inline]
            unsafe fn resolve_with(field: &$type, _: usize, _: Self::Resolver, out: *mut Self::Archived) {
                (&*out).store(field.load(Ordering::Relaxed), Ordering::Relaxed);
            }
        }

        impl_atomic!(@serialize_deserialize $type);

        impl<D: Fallible + ?Sized> DeserializeWith<$type, $type, D> for Atomic {
            #[inline]
            fn deserialize_with(field: &$type, _: &mut D) -> Result<$type, D::Error> {
                Ok(field.load(Ordering::Relaxed).into())
            }
        }
    };
    (@multibyte $type:ty) => {
        impl ArchiveWith<$type> for Atomic {
            #[cfg(not(any(feature = "archive_le", feature = "archive_be")))]
            type Archived = $type;
            #[cfg(feature = "archive_le")]
            type Archived = crate::rend::LittleEndian<$type>;
            #[cfg(feature = "archive_be")]
            type Archived = crate::rend::BigEndian<$type>;

            type Resolver = ();

            #[inline]
            unsafe fn resolve_with(field: &$type, _: usize, _: Self::Resolver, out: *mut Self::Archived) {
                (&*out).store(field.load(Ordering::Relaxed), Ordering::Relaxed);
            }
        }

        impl_atomic!(@serialize_deserialize $type);

        impl<D: Fallible + ?Sized> DeserializeWith<Archived<With<$type, Self>>, $type, D> for Atomic {
            #[inline]
            fn deserialize_with(field: &Archived<With<$type, Self>>, _: &mut D) -> Result<$type, D::Error> {
                Ok(field.load(Ordering::Relaxed).into())
            }
        }
    };
}

impl_atomic!(AtomicBool);
impl_atomic!(AtomicI8);
impl_atomic!(AtomicU8);

impl_atomic!(@multibyte AtomicI16);
impl_atomic!(@multibyte AtomicI32);
#[cfg(has_atomics_64)]
impl_atomic!(@multibyte AtomicI64);
impl_atomic!(@multibyte AtomicU16);
impl_atomic!(@multibyte AtomicU32);
#[cfg(has_atomics_64)]
impl_atomic!(@multibyte AtomicU64);

// AtomicUsize

// We can't implement Archive for AtomicUsize if the platform does not have 64-bit atomics but the
// size type is 64-bit
#[cfg(any(has_atomics_64, not(feature = "size_64")))]
const _: () = {
    use crate::FixedUsize;
    use core::sync::atomic::AtomicUsize;

    #[cfg(not(has_atomics_64))]
    type FixedAtomicUsize = pick_size_type!(AtomicU16, AtomicU32, ());
    #[cfg(has_atomics_64)]
    type FixedAtomicUsize = pick_size_type!(AtomicU16, AtomicU32, AtomicU64);

    impl ArchiveWith<AtomicUsize> for Atomic {
        type Archived = Archived<With<FixedAtomicUsize, Self>>;
        type Resolver = ();

        #[inline]
        unsafe fn resolve_with(
            field: &AtomicUsize,
            _: usize,
            _: Self::Resolver,
            out: *mut Self::Archived,
        ) {
            (*out).store(
                field.load(Ordering::Relaxed) as FixedUsize,
                Ordering::Relaxed,
            );
        }
    }

    impl<S: Fallible + ?Sized> SerializeWith<AtomicUsize, S> for Atomic {
        #[inline]
        fn serialize_with(_: &AtomicUsize, _: &mut S) -> Result<Self::Resolver, S::Error> {
            Ok(())
        }
    }

    impl<D: Fallible + ?Sized>
        DeserializeWith<<Self as ArchiveWith<FixedAtomicUsize>>::Archived, AtomicUsize, D>
        for Atomic
    {
        #[inline]
        fn deserialize_with(
            field: &<Self as ArchiveWith<FixedAtomicUsize>>::Archived,
            _: &mut D,
        ) -> Result<AtomicUsize, D::Error> {
            Ok((field.load(Ordering::Relaxed) as usize).into())
        }
    }
};

// AtomicIsize

// We can't implement Archive for AtomicIsize if the platform does not have 64-bit atomics but the
// size type is 64-bit
#[cfg(any(has_atomics_64, not(feature = "size_64")))]
const _: () = {
    use crate::FixedIsize;
    use core::sync::atomic::AtomicIsize;

    #[cfg(not(has_atomics_64))]
    type FixedAtomicIsize = pick_size_type!(AtomicI16, AtomicI32, ());
    #[cfg(has_atomics_64)]
    type FixedAtomicIsize = pick_size_type!(AtomicI16, AtomicI32, AtomicI64);

    impl ArchiveWith<AtomicIsize> for Atomic {
        type Archived = Archived<With<FixedAtomicIsize, Self>>;
        type Resolver = ();

        #[inline]
        unsafe fn resolve_with(
            field: &AtomicIsize,
            _: usize,
            _: Self::Resolver,
            out: *mut Self::Archived,
        ) {
            (*out).store(
                field.load(Ordering::Relaxed) as FixedIsize,
                Ordering::Relaxed,
            );
        }
    }

    impl<S: Fallible + ?Sized> SerializeWith<AtomicIsize, S> for Atomic {
        #[inline]
        fn serialize_with(_: &AtomicIsize, _: &mut S) -> Result<Self::Resolver, S::Error> {
            Ok(())
        }
    }

    impl<D: Fallible + ?Sized> DeserializeWith<Archived<With<AtomicIsize, Self>>, AtomicIsize, D>
        for Atomic
    {
        #[inline]
        fn deserialize_with(
            field: &Archived<With<AtomicIsize, Self>>,
            _: &mut D,
        ) -> Result<AtomicIsize, D::Error> {
            Ok((field.load(Ordering::Relaxed) as isize).into())
        }
    }
};
