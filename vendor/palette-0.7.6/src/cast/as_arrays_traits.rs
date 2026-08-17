use super::{
    from_array_slice, from_array_slice_mut, into_array_slice, into_array_slice_mut, ArrayCast,
};

/// Trait for casting a reference to a collection of colors into a reference to
/// a collection of arrays without copying.
///
/// This trait is meant as a more convenient alternative to the free functions
/// in [`cast`][crate::cast], to allow method chaining among other things.
///
/// ## Examples
///
/// ```
/// use palette::{cast::AsArrays, Srgb};
///
/// let array: [_; 2] = [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)];
/// let slice: &[_] = &[Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)];
/// let vec: Vec<_> = vec![Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)];
///
/// assert_eq!(array.as_arrays(), &[[64, 139, 10], [93, 18, 214]]);
/// assert_eq!(slice.as_arrays(), &[[64, 139, 10], [93, 18, 214]]);
/// assert_eq!(vec.as_arrays(), &[[64, 139, 10], [93, 18, 214]]);
/// ```
pub trait AsArrays<A: ?Sized> {
    /// Cast this collection of colors into a collection of arrays.
    fn as_arrays(&self) -> &A;
}

/// Trait for casting a mutable reference to a collection of colors into a
/// mutable reference to a collection of arrays without copying.
///
/// This trait is meant as a more convenient alternative to the free functions
/// in [`cast`][crate::cast], to allow method chaining among other things.
///
/// ## Examples
///
/// ```
/// use palette::{cast::AsArraysMut, Srgb};
///
/// let mut array: [_; 2] = [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)];
/// let slice_mut: &mut [_] = &mut [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)];
/// let mut vec: Vec<_> = vec![Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)];
///
/// assert_eq!(array.as_arrays_mut(), &mut [[64, 139, 10], [93, 18, 214]]);
/// assert_eq!(slice_mut.as_arrays_mut(), &mut [[64, 139, 10], [93, 18, 214]]);
/// assert_eq!(vec.as_arrays_mut(), &mut [[64, 139, 10], [93, 18, 214]]);
/// ```
pub trait AsArraysMut<A: ?Sized> {
    /// Cast this collection of colors into a mutable collection of arrays.
    fn as_arrays_mut(&mut self) -> &mut A;
}

macro_rules! impl_as_arrays {
    ($($owning:ty $(where ($($ty_input:tt)+))?),*) => {
        $(
            impl<'a, T, C, const N: usize $(, $($ty_input)+)?> AsArrays<[[T; N]]> for $owning
            where
                C: ArrayCast<Array = [T; N]>,
            {
                #[inline]
                fn as_arrays(&self) -> &[[T; N]] {
                    into_array_slice(self.as_ref())
                }
            }

            impl<'a, T, C, const N: usize $(, $($ty_input)+)?> AsArraysMut<[[T; N]]> for $owning
            where
                C: ArrayCast<Array = [T; N]>,
            {
                #[inline]
                fn as_arrays_mut(&mut self) -> &mut [[T; N]] {
                    into_array_slice_mut(self.as_mut())
                }
            }
        )*
    };
}

impl_as_arrays!([C], [C; M] where (const M: usize));

#[cfg(feature = "alloc")]
impl_as_arrays!(alloc::boxed::Box<[C]>, alloc::vec::Vec<C>);

/// Trait for casting a reference to collection of arrays into a reference to
/// collection of colors without copying.
///
/// This trait is meant as a more convenient alternative to the free functions
/// in [`cast`][crate::cast], to allow method chaining among other things.
///
/// ## Examples
///
/// ```
/// use palette::{cast::ArraysAs, Srgb};
///
/// let array: [_; 2] = [[64, 139, 10], [93, 18, 214]];
/// let slice: &[_] = &[[64, 139, 10], [93, 18, 214]];
/// let vec: Vec<_> = vec![[64, 139, 10], [93, 18, 214]];
///
/// let colors: &[Srgb<u8>] = array.arrays_as();
/// assert_eq!(colors, &[Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)]);
///
/// let colors: &[Srgb<u8>] = slice.arrays_as();
/// assert_eq!(colors, &[Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)]);
///
/// let colors: &[Srgb<u8>] = vec.arrays_as();
/// assert_eq!(colors, &[Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)]);
/// ```
pub trait ArraysAs<C: ?Sized> {
    /// Cast this collection of arrays into a collection of colors.
    fn arrays_as(&self) -> &C;
}

/// Trait for casting a mutable reference to collection of arrays into a mutable
/// reference to collection of colors without copying.
///
/// This trait is meant as a more convenient alternative to the free functions
/// in [`cast`][crate::cast], to allow method chaining among other things.
///
/// ## Examples
///
/// ```
/// use palette::{cast::ArraysAsMut, Srgb};
///
/// let mut array: [_; 2] = [[64, 139, 10], [93, 18, 214]];
/// let slice_mut: &mut [_] = &mut [[64, 139, 10], [93, 18, 214]];
/// let mut vec: Vec<_> = vec![[64, 139, 10], [93, 18, 214]];
///
/// let colors: &mut [Srgb<u8>] = array.arrays_as_mut();
/// assert_eq!(colors, &mut [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)]);
///
/// let colors: &mut [Srgb<u8>] = slice_mut.arrays_as_mut();
/// assert_eq!(colors, &mut [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)]);
///
/// let colors: &mut [Srgb<u8>] = vec.arrays_as_mut();
/// assert_eq!(colors, &mut [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)]);
/// ```
pub trait ArraysAsMut<C: ?Sized> {
    /// Cast this collection of arrays into a mutable collection of colors.
    fn arrays_as_mut(&mut self) -> &mut C;
}

macro_rules! impl_arrays_as {
    ($($owning:ty $(where ($($ty_input:tt)+))?),*) => {
        $(
            impl<'a, T, C, const N: usize $(, $($ty_input)+)?> ArraysAs<[C]> for $owning
            where
                C: ArrayCast<Array = [T; N]>,
            {
                #[inline]
                fn arrays_as(&self) -> &[C] {
                    from_array_slice(self.as_ref())
                }
            }

            impl<'a, T, C, const N: usize $(, $($ty_input)+)?> ArraysAsMut<[C]> for $owning
            where
                C: ArrayCast<Array = [T; N]>,
            {
                #[inline]
                fn arrays_as_mut(&mut self) -> &mut [C] {
                    from_array_slice_mut(self.as_mut())
                }
            }
        )*
    };
}

impl_arrays_as!([[T; N]], [[T; N]; M] where (const M: usize));

#[cfg(feature = "alloc")]
impl_arrays_as!(alloc::boxed::Box<[[T; N]]>, alloc::vec::Vec<[T; N]>);

#[cfg(test)]
mod test {
    use crate::Srgb;

    use super::{ArraysAs, ArraysAsMut, AsArrays, AsArraysMut};

    #[test]
    fn as_arrays() {
        let slice: &[Srgb<u8>] = &[Srgb::new(1, 2, 3), Srgb::new(4, 5, 6)];
        let slice_mut: &mut [Srgb<u8>] = &mut [Srgb::new(1, 2, 3), Srgb::new(4, 5, 6)];
        let mut slice_box: Box<[Srgb<u8>]> =
            vec![Srgb::new(1, 2, 3), Srgb::new(4, 5, 6)].into_boxed_slice();
        let mut vec: Vec<Srgb<u8>> = vec![Srgb::new(1, 2, 3), Srgb::new(4, 5, 6)];
        let mut array: [Srgb<u8>; 2] = [Srgb::new(1, 2, 3), Srgb::new(4, 5, 6)];

        let _: &[[u8; 3]] = slice.as_arrays();
        let _: &[[u8; 3]] = slice_box.as_arrays();
        let _: &[[u8; 3]] = vec.as_arrays();
        let _: &[[u8; 3]] = array.as_arrays();

        let _: &mut [[u8; 3]] = slice_mut.as_arrays_mut();
        let _: &mut [[u8; 3]] = slice_box.as_arrays_mut();
        let _: &mut [[u8; 3]] = vec.as_arrays_mut();
        let _: &mut [[u8; 3]] = array.as_arrays_mut();
    }

    #[test]
    fn arrays_as() {
        let slice: &[[u8; 3]] = &[[1, 2, 3], [4, 5, 6]];
        let slice_mut: &mut [[u8; 3]] = &mut [[1, 2, 3], [4, 5, 6]];
        let mut slice_box: Box<[[u8; 3]]> = vec![[1, 2, 3], [4, 5, 6]].into_boxed_slice();
        let mut vec: Vec<[u8; 3]> = vec![[1, 2, 3], [4, 5, 6]];
        let mut array: [[u8; 3]; 2] = [[1, 2, 3], [4, 5, 6]];

        let _: &[Srgb<u8>] = slice.arrays_as();
        let _: &[Srgb<u8>] = slice_box.arrays_as();
        let _: &[Srgb<u8>] = vec.arrays_as();
        let _: &[Srgb<u8>] = array.arrays_as();

        let _: &mut [Srgb<u8>] = slice_mut.arrays_as_mut();
        let _: &mut [Srgb<u8>] = slice_box.arrays_as_mut();
        let _: &mut [Srgb<u8>] = vec.arrays_as_mut();
        let _: &mut [Srgb<u8>] = array.arrays_as_mut();
    }
}
