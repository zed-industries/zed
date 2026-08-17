use super::{
    from_array_array, from_array_slice, from_array_slice_mut, into_array_array, into_array_slice,
    into_array_slice_mut, ArrayCast,
};

#[cfg(feature = "alloc")]
use super::{from_array_slice_box, from_array_vec, into_array_slice_box, into_array_vec};

/// Trait for casting a collection of colors from a collection of arrays without
/// copying.
///
/// This trait is meant as a more convenient alternative to the free functions
/// in [`cast`][crate::cast], to allow method chaining among other things.
///
/// ## Examples
///
/// ```
/// use palette::{cast::FromArrays, Srgb};
///
/// let array: [_; 2] = [[64, 139, 10], [93, 18, 214]];
/// let slice: &[_] = &[[64, 139, 10], [93, 18, 214]];
/// let slice_mut: &mut [_] = &mut [[64, 139, 10], [93, 18, 214]];
/// let vec: Vec<_> = vec![[64, 139, 10], [93, 18, 214]];
///
/// assert_eq!(
///     <[Srgb<u8>; 2]>::from_arrays(array),
///     [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)]
/// );
///
/// assert_eq!(
///     <&[Srgb<u8>]>::from_arrays(slice),
///     [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)]
/// );
///
/// assert_eq!(
///     <&mut [Srgb<u8>]>::from_arrays(slice_mut),
///     [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)]
/// );
///
/// assert_eq!(
///     Vec::<Srgb<u8>>::from_arrays(vec),
///     vec![Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)]
/// );
/// ```
///
/// Owning types can be cast as slices, too:
///
/// ```
/// use palette::{cast::FromArrays, Srgb};
///
/// let array: [_; 2] = [[64, 139, 10], [93, 18, 214]];
/// let mut vec: Vec<_> = vec![[64, 139, 10], [93, 18, 214]];
///
/// assert_eq!(
///     <&[Srgb<u8>]>::from_arrays(&array),
///     [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)]
/// );
///
/// assert_eq!(
///     <&mut [Srgb<u8>]>::from_arrays(&mut vec),
///     [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)]
/// );
/// ```
pub trait FromArrays<A> {
    /// Cast a collection of arrays into an collection of colors.
    fn from_arrays(arrays: A) -> Self;
}

impl<T, C, const N: usize, const M: usize> FromArrays<[[T; N]; M]> for [C; M]
where
    C: ArrayCast<Array = [T; N]>,
{
    #[inline]
    fn from_arrays(arrays: [[T; N]; M]) -> Self {
        from_array_array(arrays)
    }
}

macro_rules! impl_from_arrays_slice {
    ($($owning:ty $(where ($($ty_input:tt)+))?),*) => {
        $(
            impl<'a, T, C, const N: usize $(, $($ty_input)+)?> FromArrays<&'a $owning> for &'a [C]
            where
                C: ArrayCast<Array = [T; N]>,
            {
                #[inline]
                fn from_arrays(arrays: &'a $owning) -> Self {
                    from_array_slice(arrays)
                }
            }

            impl<'a, T, C, const N: usize $(, $($ty_input)+)?> FromArrays<&'a mut $owning> for &'a mut [C]
            where
                C: ArrayCast<Array = [T; N]>,
            {
                #[inline]
                fn from_arrays(arrays: &'a mut $owning) -> Self {
                    from_array_slice_mut(arrays)
                }
            }
        )*
    };
}

impl_from_arrays_slice!([[T; N]], [[T; N]; M] where (const M: usize));

#[cfg(feature = "alloc")]
impl_from_arrays_slice!(alloc::boxed::Box<[[T; N]]>, alloc::vec::Vec<[T; N]>);

#[cfg(feature = "alloc")]
impl<T, C, const N: usize> FromArrays<alloc::boxed::Box<[[T; N]]>> for alloc::boxed::Box<[C]>
where
    C: ArrayCast<Array = [T; N]>,
{
    #[inline]
    fn from_arrays(arrays: alloc::boxed::Box<[[T; N]]>) -> Self {
        from_array_slice_box(arrays)
    }
}

#[cfg(feature = "alloc")]
impl<T, C, const N: usize> FromArrays<alloc::vec::Vec<[T; N]>> for alloc::vec::Vec<C>
where
    C: ArrayCast<Array = [T; N]>,
{
    #[inline]
    fn from_arrays(arrays: alloc::vec::Vec<[T; N]>) -> Self {
        from_array_vec(arrays)
    }
}

/// Trait for casting a collection of colors into a collection of arrays without
/// copying.
///
/// This trait is meant as a more convenient alternative to the free functions
/// in [`cast`][crate::cast], to allow method chaining among other things.
///
/// ## Examples
///
/// ```
/// use palette::{cast::IntoArrays, Srgb};
///
/// let array: [_; 2] = [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)];
/// let slice: &[_] = &[Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)];
/// let slice_mut: &mut [_] = &mut [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)];
/// let vec: Vec<_> = vec![Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)];
///
/// assert_eq!(array.into_arrays(), [[64, 139, 10], [93, 18, 214]]);
/// assert_eq!(slice.into_arrays(), [[64, 139, 10], [93, 18, 214]]);
/// assert_eq!(slice_mut.into_arrays(), [[64, 139, 10], [93, 18, 214]]);
/// assert_eq!(vec.into_arrays(), vec![[64, 139, 10], [93, 18, 214]]);
/// ```
///
/// Owning types can be cast as slices, too:
///
/// ```
/// use palette::{cast::IntoArrays, Srgb};
///
/// let array: [_; 2] = [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)];
/// let mut vec: Vec<_> = vec![Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)];
///
/// assert_eq!((&array).into_arrays(), [[64, 139, 10], [93, 18, 214]]);
/// assert_eq!((&mut vec).into_arrays(), [[64, 139, 10], [93, 18, 214]]);
/// ```
pub trait IntoArrays<A> {
    /// Cast this collection of colors into a collection of arrays.
    fn into_arrays(self) -> A;
}

impl<T, C, const N: usize, const M: usize> IntoArrays<[[T; N]; M]> for [C; M]
where
    C: ArrayCast<Array = [T; N]>,
{
    #[inline]
    fn into_arrays(self) -> [[T; N]; M] {
        into_array_array(self)
    }
}

macro_rules! impl_into_arrays_slice {
    ($($owning:ty $(where ($($ty_input:tt)+))?),*) => {
        $(
            impl<'a, T, C, const N: usize $(, $($ty_input)+)?> IntoArrays<&'a [[T; N]]> for &'a $owning
            where
                C: ArrayCast<Array = [T; N]>,
            {
                #[inline]
                fn into_arrays(self) -> &'a [[T; N]]  {
                    into_array_slice(self)
                }
            }

            impl<'a, T, C, const N: usize $(, $($ty_input)+)?> IntoArrays<&'a mut [[T; N]]> for &'a mut $owning
            where
                C: ArrayCast<Array = [T; N]>,
            {
                #[inline]
                fn into_arrays(self) -> &'a mut [[T; N]] {
                    into_array_slice_mut(self)
                }
            }
        )*
    };
}

impl_into_arrays_slice!([C], [C; M] where (const M: usize));

#[cfg(feature = "alloc")]
impl_into_arrays_slice!(alloc::boxed::Box<[C]>, alloc::vec::Vec<C>);

#[cfg(feature = "alloc")]
impl<T, C, const N: usize> IntoArrays<alloc::boxed::Box<[[T; N]]>> for alloc::boxed::Box<[C]>
where
    C: ArrayCast<Array = [T; N]>,
{
    #[inline]
    fn into_arrays(self) -> alloc::boxed::Box<[[T; N]]> {
        into_array_slice_box(self)
    }
}

#[cfg(feature = "alloc")]
impl<T, C, const N: usize> IntoArrays<alloc::vec::Vec<[T; N]>> for alloc::vec::Vec<C>
where
    C: ArrayCast<Array = [T; N]>,
{
    #[inline]
    fn into_arrays(self) -> alloc::vec::Vec<[T; N]> {
        into_array_vec(self)
    }
}

/// Trait for casting a collection of arrays from a collection of colors without
/// copying.
///
/// This trait is meant as a more convenient alternative to the free functions
/// in [`cast`][crate::cast], to allow method chaining among other things.
///
/// ## Examples
///
/// ```
/// use palette::{cast::ArraysFrom, Srgb};
///
/// let array: [_; 2] = [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)];
/// let slice: &[_] = &[Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)];
/// let slice_mut: &mut [_] = &mut [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)];
/// let vec: Vec<_> = vec![Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)];
///
/// assert_eq!(<[_; 2]>::arrays_from(array), [[64, 139, 10], [93, 18, 214]]);
/// assert_eq!(<&[_]>::arrays_from(slice), [[64, 139, 10], [93, 18, 214]]);
/// assert_eq!(<&mut [_]>::arrays_from(slice_mut), [[64, 139, 10], [93, 18, 214]]);
/// assert_eq!(Vec::<_>::arrays_from(vec), vec![[64, 139, 10], [93, 18, 214]]);
/// ```
///
/// Owning types can be cast as slices, too:
///
/// ```
/// use palette::{cast::ArraysFrom, Srgb};
///
/// let array: [_; 2] = [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)];
/// let mut vec: Vec<_> = vec![Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)];
///
/// assert_eq!(<&[_]>::arrays_from(&array), [[64, 139, 10], [93, 18, 214]]);
/// assert_eq!(<&mut [_]>::arrays_from(&mut vec), [[64, 139, 10], [93, 18, 214]]);
/// ```
pub trait ArraysFrom<C> {
    /// Cast a collection of colors into a collection of arrays.
    fn arrays_from(colors: C) -> Self;
}

impl<T, C> ArraysFrom<C> for T
where
    C: IntoArrays<T>,
{
    #[inline]
    fn arrays_from(colors: C) -> Self {
        colors.into_arrays()
    }
}

/// Trait for casting a collection of arrays into a collection of colors
/// without copying.
///
/// This trait is meant as a more convenient alternative to the free functions
/// in [`cast`][crate::cast], to allow method chaining among other things.
///
/// ## Examples
///
/// ```
/// use palette::{cast::ArraysInto, Srgb};
///
/// let array: [_; 2] = [[64, 139, 10], [93, 18, 214]];
/// let slice: &[_] = &[[64, 139, 10], [93, 18, 214]];
/// let slice_mut: &mut [_] = &mut [[64, 139, 10], [93, 18, 214]];
/// let vec: Vec<_> = vec![[64, 139, 10], [93, 18, 214]];
///
/// let colors: [Srgb<u8>; 2] = array.arrays_into();
/// assert_eq!(colors, [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)]);
///
/// let colors: &[Srgb<u8>] = slice.arrays_into();
/// assert_eq!(colors, [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)]);
///
/// let colors: &mut [Srgb<u8>] = slice_mut.arrays_into();
/// assert_eq!(colors, [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)]);
///
/// let colors: Vec<Srgb<u8>> = vec.arrays_into();
/// assert_eq!(colors, vec![Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)]);
/// ```
///
/// Owning types can be cast as slices, too:
///
/// ```
/// use palette::{cast::ArraysInto, Srgb};
///
/// let array: [_; 2] = [[64, 139, 10], [93, 18, 214]];
/// let mut vec: Vec<_> = vec![[64, 139, 10], [93, 18, 214]];
///
/// let colors: &[Srgb<u8>] = (&array).arrays_into();
/// assert_eq!(colors, [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)]);
///
/// let colors: &mut [Srgb<u8>] = (&mut vec).arrays_into();
/// assert_eq!(colors, [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)]);
/// ```
pub trait ArraysInto<C> {
    /// Cast this collection of arrays into a collection of colors.
    fn arrays_into(self) -> C;
}

impl<T, C> ArraysInto<C> for T
where
    C: FromArrays<T>,
{
    #[inline]
    fn arrays_into(self) -> C {
        C::from_arrays(self)
    }
}

#[cfg(test)]
mod test {
    use crate::Srgb;

    use super::{ArraysFrom, ArraysInto, FromArrays, IntoArrays};

    #[test]
    fn from_arrays() {
        let slice: &[[u8; 3]] = &[[1, 2, 3], [4, 5, 6]];
        let slice_mut: &mut [[u8; 3]] = &mut [[1, 2, 3], [4, 5, 6]];
        let mut array: [[u8; 3]; 2] = [[1, 2, 3], [4, 5, 6]];

        let _ = <&[Srgb<u8>]>::from_arrays(slice);
        let _ = <&[Srgb<u8>]>::from_arrays(&array);

        let _ = <&mut [Srgb<u8>]>::from_arrays(slice_mut);
        let _ = <&mut [Srgb<u8>]>::from_arrays(&mut array);

        let _ = <[Srgb<u8>; 2]>::from_arrays(array);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn from_arrays_alloc() {
        let mut slice_box: Box<[[u8; 3]]> = vec![[1, 2, 3], [4, 5, 6]].into_boxed_slice();
        let mut vec: Vec<[u8; 3]> = vec![[1, 2, 3], [4, 5, 6]];

        let _ = <&[Srgb<u8>]>::from_arrays(&slice_box);
        let _ = <&[Srgb<u8>]>::from_arrays(&vec);

        let _ = <&mut [Srgb<u8>]>::from_arrays(&mut slice_box);
        let _ = <&mut [Srgb<u8>]>::from_arrays(&mut vec);

        let _ = Box::<[Srgb<u8>]>::from_arrays(slice_box);
        let _ = Vec::<Srgb<u8>>::from_arrays(vec);
    }

    #[test]
    fn arrays_into() {
        let slice: &[[u8; 3]] = &[[1, 2, 3], [4, 5, 6]];
        let slice_mut: &mut [[u8; 3]] = &mut [[1, 2, 3], [4, 5, 6]];
        let mut array: [[u8; 3]; 2] = [[1, 2, 3], [4, 5, 6]];

        let _: &[Srgb<u8>] = slice.arrays_into();
        let _: &[Srgb<u8>] = (&array).arrays_into();

        let _: &mut [Srgb<u8>] = slice_mut.arrays_into();
        let _: &mut [Srgb<u8>] = (&mut array).arrays_into();

        let _: [Srgb<u8>; 2] = array.arrays_into();
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn arrays_into_alloc() {
        let mut slice_box: Box<[[u8; 3]]> = vec![[1, 2, 3], [4, 5, 6]].into_boxed_slice();
        let mut vec: Vec<[u8; 3]> = vec![[1, 2, 3], [4, 5, 6]];

        let _: &[Srgb<u8>] = (&slice_box).arrays_into();
        let _: &[Srgb<u8>] = (&vec).arrays_into();

        let _: &mut [Srgb<u8>] = (&mut slice_box).arrays_into();
        let _: &mut [Srgb<u8>] = (&mut vec).arrays_into();

        let _: Box<[Srgb<u8>]> = slice_box.arrays_into();
        let _: Vec<Srgb<u8>> = vec.arrays_into();
    }

    #[test]
    fn into_arrays() {
        let slice: &[Srgb<u8>] = &[Srgb::new(1, 2, 3), Srgb::new(4, 5, 6)];
        let slice_mut: &mut [Srgb<u8>] = &mut [Srgb::new(1, 2, 3), Srgb::new(4, 5, 6)];
        let mut array: [Srgb<u8>; 2] = [Srgb::new(1, 2, 3), Srgb::new(4, 5, 6)];

        let _: &[[u8; 3]] = slice.into_arrays();
        let _: &[[u8; 3]] = (&array).into_arrays();

        let _: &mut [[u8; 3]] = slice_mut.into_arrays();
        let _: &mut [[u8; 3]] = (&mut array).into_arrays();

        let _: [[u8; 3]; 2] = array.into_arrays();
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn into_arrays_alloc() {
        let mut slice_box: Box<[Srgb<u8>]> =
            vec![Srgb::new(1, 2, 3), Srgb::new(4, 5, 6)].into_boxed_slice();
        let mut vec: Vec<Srgb<u8>> = vec![Srgb::new(1, 2, 3), Srgb::new(4, 5, 6)];

        let _: &[[u8; 3]] = (&slice_box).into_arrays();
        let _: &[[u8; 3]] = (&vec).into_arrays();

        let _: &mut [[u8; 3]] = (&mut slice_box).into_arrays();
        let _: &mut [[u8; 3]] = (&mut vec).into_arrays();

        let _: Box<[[u8; 3]]> = slice_box.into_arrays();
        let _: Vec<[u8; 3]> = vec.into_arrays();
    }

    #[test]
    fn arrays_from() {
        let slice: &[Srgb<u8>] = &[Srgb::new(1, 2, 3), Srgb::new(4, 5, 6)];
        let slice_mut: &mut [Srgb<u8>] = &mut [Srgb::new(1, 2, 3), Srgb::new(4, 5, 6)];
        let mut array: [Srgb<u8>; 2] = [Srgb::new(1, 2, 3), Srgb::new(4, 5, 6)];

        let _ = <&[[u8; 3]]>::arrays_from(slice);
        let _ = <&[[u8; 3]]>::arrays_from(&array);

        let _ = <&mut [[u8; 3]]>::arrays_from(slice_mut);
        let _ = <&mut [[u8; 3]]>::arrays_from(&mut array);

        let _ = <[[u8; 3]; 2]>::arrays_from(array);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn arrays_from_alloc() {
        let mut slice_box: Box<[Srgb<u8>]> =
            vec![Srgb::new(1, 2, 3), Srgb::new(4, 5, 6)].into_boxed_slice();
        let mut vec: Vec<Srgb<u8>> = vec![Srgb::new(1, 2, 3), Srgb::new(4, 5, 6)];

        let _ = <&[[u8; 3]]>::arrays_from(&slice_box);
        let _ = <&[[u8; 3]]>::arrays_from(&vec);

        let _ = <&mut [[u8; 3]]>::arrays_from(&mut slice_box);
        let _ = <&mut [[u8; 3]]>::arrays_from(&mut vec);

        let _ = Box::<[[u8; 3]]>::arrays_from(slice_box);
        let _ = Vec::<[u8; 3]>::arrays_from(vec);
    }
}
