use {
    crate::{Arbitrary, Error, MaxRecursionReached, Result, Unstructured},
    core::{
        mem,
        num::{
            NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
            NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize, Wrapping,
        },
    },
};

macro_rules! impl_arbitrary_for_integers {
    ( $( $ty:ty; )* ) => {
        $(
            impl<'a> Arbitrary<'a> for $ty {
                fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
                    let mut buf = [0; mem::size_of::<$ty>()];
                    u.fill_buffer(&mut buf)?;
                    Ok(Self::from_le_bytes(buf))
                }

                #[inline]
                fn size_hint(_depth: usize) -> (usize, Option<usize>) {
                    let n = mem::size_of::<$ty>();
                    (n, Some(n))
                }

            }
        )*
    }
}

impl_arbitrary_for_integers! {
    u8;
    u16;
    u32;
    u64;
    u128;
    i8;
    i16;
    i32;
    i64;
    i128;
}

// Note: We forward Arbitrary for i/usize to i/u64 in order to simplify corpus
// compatibility between 32-bit and 64-bit builds. This introduces dead space in
// 32-bit builds but keeps the input layout independent of the build platform.
impl<'a> Arbitrary<'a> for usize {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        u.arbitrary::<u64>().map(|x| x as usize)
    }

    #[inline]
    fn size_hint(depth: usize) -> (usize, Option<usize>) {
        <u64 as Arbitrary>::size_hint(depth)
    }
}

impl<'a> Arbitrary<'a> for isize {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        u.arbitrary::<i64>().map(|x| x as isize)
    }

    #[inline]
    fn size_hint(depth: usize) -> (usize, Option<usize>) {
        <i64 as Arbitrary>::size_hint(depth)
    }
}

macro_rules! impl_arbitrary_for_floats {
    ( $( $ty:ident : $unsigned:ty; )* ) => {
        $(
            impl<'a> Arbitrary<'a> for $ty {
                fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
                    Ok(Self::from_bits(<$unsigned as Arbitrary<'a>>::arbitrary(u)?))
                }

                #[inline]
                fn size_hint(depth: usize) -> (usize, Option<usize>) {
                    <$unsigned as Arbitrary<'a>>::size_hint(depth)
                }
            }
        )*
    }
}

impl_arbitrary_for_floats! {
    f32: u32;
    f64: u64;
}

macro_rules! implement_nonzero_int {
    ($nonzero:ty, $int:ty) => {
        impl<'a> Arbitrary<'a> for $nonzero {
            fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
                match Self::new(<$int as Arbitrary<'a>>::arbitrary(u)?) {
                    Some(n) => Ok(n),
                    None => Err(Error::IncorrectFormat),
                }
            }

            #[inline]
            fn size_hint(depth: usize) -> (usize, Option<usize>) {
                <$int as Arbitrary<'a>>::size_hint(depth)
            }
        }
    };
}

implement_nonzero_int! { NonZeroI8, i8 }
implement_nonzero_int! { NonZeroI16, i16 }
implement_nonzero_int! { NonZeroI32, i32 }
implement_nonzero_int! { NonZeroI64, i64 }
implement_nonzero_int! { NonZeroI128, i128 }
implement_nonzero_int! { NonZeroIsize, isize }
implement_nonzero_int! { NonZeroU8, u8 }
implement_nonzero_int! { NonZeroU16, u16 }
implement_nonzero_int! { NonZeroU32, u32 }
implement_nonzero_int! { NonZeroU64, u64 }
implement_nonzero_int! { NonZeroU128, u128 }
implement_nonzero_int! { NonZeroUsize, usize }

impl<'a, A> Arbitrary<'a> for Wrapping<A>
where
    A: Arbitrary<'a>,
{
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        Arbitrary::arbitrary(u).map(Wrapping)
    }

    #[inline]
    fn size_hint(depth: usize) -> (usize, Option<usize>) {
        Self::try_size_hint(depth).unwrap_or_default()
    }

    #[inline]
    fn try_size_hint(depth: usize) -> Result<(usize, Option<usize>), MaxRecursionReached> {
        <A as Arbitrary<'a>>::try_size_hint(depth)
    }
}
