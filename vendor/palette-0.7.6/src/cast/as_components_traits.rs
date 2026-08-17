use core::fmt::Debug;

use super::{
    into_component_slice, into_component_slice_mut, try_from_component_slice,
    try_from_component_slice_mut, ArrayCast, SliceCastError,
};

/// Trait for casting a reference to a collection of colors into a reference to
/// a collection of color components without copying.
///
/// This trait is meant as a more convenient alternative to the free functions
/// in [`cast`][crate::cast], to allow method chaining among other things.
///
/// ## Examples
///
/// ```
/// use palette::{cast::AsComponents, Srgb};
///
/// let array: [_; 2] = [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)];
/// let slice: &[_] = &[Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)];
/// let vec: Vec<_> = vec![Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)];
///
/// assert_eq!(array.as_components(), &[64, 139, 10, 93, 18, 214]);
/// assert_eq!(slice.as_components(), &[64, 139, 10, 93, 18, 214]);
/// assert_eq!(vec.as_components(), &[64, 139, 10, 93, 18, 214]);
/// ```
pub trait AsComponents<C: ?Sized> {
    /// Cast this collection of colors into a collection of color components.
    fn as_components(&self) -> &C;
}

/// Trait for casting a mutable reference to a collection of colors into a
/// mutable reference to a collection of color components without copying.
///
/// This trait is meant as a more convenient alternative to the free functions
/// in [`cast`][crate::cast], to allow method chaining among other things.
///
/// ## Examples
///
/// ```
/// use palette::{cast::AsComponentsMut, Srgb};
///
/// let mut array: [_; 2] = [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)];
/// let slice_mut: &mut [_] = &mut [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)];
/// let mut vec: Vec<_> = vec![Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)];
///
/// assert_eq!(array.as_components_mut(), &mut [64, 139, 10, 93, 18, 214]);
/// assert_eq!(slice_mut.as_components_mut(), &mut [64, 139, 10, 93, 18, 214]);
/// assert_eq!(vec.as_components_mut(), &mut [64, 139, 10, 93, 18, 214]);
/// ```
pub trait AsComponentsMut<C: ?Sized> {
    /// Cast this collection of colors into a mutable collection of color
    /// components.
    fn as_components_mut(&mut self) -> &mut C;
}

macro_rules! impl_as_components {
    ($($owning:ty $(where ($($ty_input:tt)+))?),*) => {
        $(
            impl<'a, T, C, const N: usize $(, $($ty_input)+)?> AsComponents<[T]> for $owning
            where
                C: ArrayCast<Array = [T; N]>,
            {
                #[inline]
                fn as_components(&self) -> &[T] {
                    into_component_slice(self.as_ref())
                }
            }

            impl<'a, T, C, const N: usize $(, $($ty_input)+)?> AsComponentsMut<[T]> for $owning
            where
                C: ArrayCast<Array = [T; N]>,
            {
                #[inline]
                fn as_components_mut(&mut self) -> &mut [T] {
                    into_component_slice_mut(self.as_mut())
                }
            }
        )*
    };
}

impl_as_components!([C], [C; M] where (const M: usize));

#[cfg(feature = "alloc")]
impl_as_components!(alloc::boxed::Box<[C]>, alloc::vec::Vec<C>);

/// Trait for trying to cast a reference to collection of color components into
/// a reference to collection of colors without copying.
///
/// This trait is meant as a more convenient alternative to the free functions
/// in [`cast`][crate::cast], to allow method chaining among other things.
///
/// ## Errors
///
/// The cast will return an error if the cast fails, such as when the length of
/// the input is not a multiple of the color's array length.
///
/// ## Examples
///
/// ```
/// use palette::{cast::TryComponentsAs, Srgb};
///
/// let array: [_; 6] = [64, 139, 10, 93, 18, 214];
/// let slice: &[_] = &[64, 139, 10, 93, 18, 214];
/// let vec: Vec<_> = vec![64, 139, 10, 93, 18, 214];
///
/// let colors: Result<&[Srgb<u8>], _> = array.try_components_as();
/// assert_eq!(colors, Ok(&[Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)][..]));
///
/// let colors: Result<&[Srgb<u8>], _> = slice.try_components_as();
/// assert_eq!(colors, Ok(&[Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)][..]));
///
/// let colors: Result<&[Srgb<u8>], _> = vec.try_components_as();
/// assert_eq!(colors, Ok(&[Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)][..]));
/// ```
///
/// This produces an error:
///
/// ```
/// use palette::{cast::TryComponentsAs, Srgb};
///
/// let components = [64, 139, 10, 93, 18]; // Not a multiple of 3
/// let colors: Result<&[Srgb<u8>], _> = components.try_components_as();
/// assert!(colors.is_err());
/// ```
pub trait TryComponentsAs<C: ?Sized> {
    /// The error for when `try_components_as` fails to cast.
    type Error;

    /// Try to cast this collection of color components into a reference to a
    /// collection of colors.
    ///
    /// Return an error if the conversion can't be done, such as when the number
    /// of items in `self` isn't a multiple of the number of components in the
    /// color type.
    fn try_components_as(&self) -> Result<&C, Self::Error>;
}

/// Trait for trying to cast a mutable reference to collection of color
/// components into a mutable reference to collection of colors without copying.
///
/// This trait is meant as a more convenient alternative to the free functions
/// in [`cast`][crate::cast], to allow method chaining among other things.
///
/// ## Errors
///
/// The cast will return an error if the cast fails, such as when the length of
/// the input is not a multiple of the color's array length.
///
/// ## Examples
///
/// ```
/// use palette::{cast::TryComponentsAsMut, Srgb};
///
/// let mut array: [_; 6] = [64, 139, 10, 93, 18, 214];
/// let slice_mut: &mut [_] = &mut [64, 139, 10, 93, 18, 214];
/// let mut vec: Vec<_> = vec![64, 139, 10, 93, 18, 214];
///
/// let colors: Result<&mut [Srgb<u8>], _> = array.try_components_as_mut();
/// assert_eq!(colors, Ok(&mut [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)][..]));
///
/// let colors: Result<&mut [Srgb<u8>], _> = slice_mut.try_components_as_mut();
/// assert_eq!(colors, Ok(&mut [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)][..]));
///
/// let colors: Result<&mut [Srgb<u8>], _> = vec.try_components_as_mut();
/// assert_eq!(colors, Ok(&mut [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)][..]));
/// ```
///
/// This produces an error:
///
/// ```
/// use palette::{cast::TryComponentsAsMut, Srgb};
///
/// let mut components = [64, 139, 10, 93, 18]; // Not a multiple of 3
/// let colors: Result<&mut [Srgb<u8>], _> = components.try_components_as_mut();
/// assert!(colors.is_err());
/// ```
pub trait TryComponentsAsMut<C: ?Sized> {
    /// The error for when `try_components_as_mut` fails to cast.
    type Error;

    /// Try to cast this collection of color components into a mutable reference
    /// to a collection of colors.
    ///
    /// Return an error if the conversion can't be done, such as when the number
    /// of items in `self` isn't a multiple of the number of components in the
    /// color type.
    fn try_components_as_mut(&mut self) -> Result<&mut C, Self::Error>;
}

macro_rules! impl_try_components_as {
    ($($owning:ty $(where ($($ty_input:tt)+))?),*) => {
        $(
            impl<'a, T, C, const N: usize $(, $($ty_input)+)?> TryComponentsAs<[C]> for $owning
            where
                C: ArrayCast<Array = [T; N]>,
            {
                type Error = SliceCastError;

                #[inline]
                fn try_components_as(&self) -> Result<&[C], Self::Error> {
                    try_from_component_slice(self.as_ref())
                }
            }

            impl<'a, T, C, const N: usize $(, $($ty_input)+)?> TryComponentsAsMut<[C]> for $owning
            where
                C: ArrayCast<Array = [T; N]>,
            {
                type Error = SliceCastError;

                #[inline]
                fn try_components_as_mut(&mut self) -> Result<&mut [C], Self::Error> {
                    try_from_component_slice_mut(self.as_mut())
                }
            }
        )*
    };
}

impl_try_components_as!([T], [T; M] where (const M: usize));

#[cfg(feature = "alloc")]
impl_try_components_as!(alloc::boxed::Box<[T]>, alloc::vec::Vec<T>);

/// Trait for casting a reference to collection of color components into a
/// reference to collection of colors without copying.
///
/// This trait is meant as a more convenient alternative to the free functions
/// in [`cast`][crate::cast], to allow method chaining among other things.
///
/// ## Panics
///
/// The cast will panic if the cast fails, such as when the length of the input
/// is not a multiple of the color's array length.
///
/// ## Examples
///
/// ```
/// use palette::{cast::ComponentsAs, Srgb};
///
/// let array: [_; 6] = [64, 139, 10, 93, 18, 214];
/// let slice: &[_] = &[64, 139, 10, 93, 18, 214];
/// let vec: Vec<_> = vec![64, 139, 10, 93, 18, 214];
///
/// let colors: &[Srgb<u8>] = array.components_as();
/// assert_eq!(colors, &[Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)]);
///
/// let colors: &[Srgb<u8>] = slice.components_as();
/// assert_eq!(colors, &[Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)]);
///
/// let colors: &[Srgb<u8>] = vec.components_as();
/// assert_eq!(colors, &[Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)]);
/// ```
///
/// This panics:
///
/// ```should_panic
/// use palette::{cast::ComponentsAs, Srgb};
///
/// let components = [64, 139, 10, 93, 18, 214, 0, 123]; // Not a multiple of 3
/// let colors: &[Srgb<u8>] = components.components_as();
/// ```
pub trait ComponentsAs<C: ?Sized> {
    /// Cast this collection of color components into a reference to a
    /// collection of colors.
    ///
    /// ## Panics
    /// If the conversion can't be done, such as when the number of items in
    /// `self` isn't a multiple of the number of components in the color type.
    fn components_as(&self) -> &C;
}

impl<T, C> ComponentsAs<C> for T
where
    T: TryComponentsAs<C> + ?Sized,
    T::Error: Debug,
    C: ?Sized,
{
    fn components_as(&self) -> &C {
        self.try_components_as().unwrap()
    }
}

/// Trait for casting a mutable reference to collection of color components into
/// a mutable reference to collection of colors without copying.
///
/// This trait is meant as a more convenient alternative to the free functions
/// in [`cast`][crate::cast], to allow method chaining among other things.
///
/// ## Panics
///
/// The cast will panic if the cast fails, such as when the length of the input
/// is not a multiple of the color's array length.
///
/// ## Examples
///
/// ```
/// use palette::{cast::ComponentsAsMut, Srgb};
///
/// let mut array: [_; 6] = [64, 139, 10, 93, 18, 214];
/// let slice_mut: &mut [_] = &mut [64, 139, 10, 93, 18, 214];
/// let mut vec: Vec<_> = vec![64, 139, 10, 93, 18, 214];
///
/// let colors: &mut [Srgb<u8>] = array.components_as_mut();
/// assert_eq!(colors, &mut [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)]);
///
/// let colors: &mut [Srgb<u8>] = slice_mut.components_as_mut();
/// assert_eq!(colors, &mut [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)]);
///
/// let colors: &mut [Srgb<u8>] = vec.components_as_mut();
/// assert_eq!(colors, &mut [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)]);
/// ```
///
/// This panics:
///
/// ```should_panic
/// use palette::{cast::ComponentsAsMut, Srgb};
///
/// let mut components = [64, 139, 10, 93, 18, 214, 0, 123]; // Not a multiple of 3
/// let colors: &mut [Srgb<u8>] = components.components_as_mut();
/// ```
pub trait ComponentsAsMut<C: ?Sized> {
    /// Cast this collection of color components into a mutable reference to a
    /// collection of colors.
    ///
    /// ## Panics
    /// If the conversion can't be done, such as when the number of items in
    /// `self` isn't a multiple of the number of components in the color type.
    fn components_as_mut(&mut self) -> &mut C;
}

impl<T, C> ComponentsAsMut<C> for T
where
    T: TryComponentsAsMut<C> + ?Sized,
    T::Error: Debug,
    C: ?Sized,
{
    fn components_as_mut(&mut self) -> &mut C {
        self.try_components_as_mut().unwrap()
    }
}

#[cfg(test)]
mod test {
    use crate::Srgb;

    use super::{
        AsComponents, AsComponentsMut, ComponentsAs, ComponentsAsMut, TryComponentsAs,
        TryComponentsAsMut,
    };

    #[test]
    fn as_components() {
        let slice: &[Srgb<u8>] = &[Srgb::new(1, 2, 3), Srgb::new(4, 5, 6)];
        let slice_mut: &mut [Srgb<u8>] = &mut [Srgb::new(1, 2, 3), Srgb::new(4, 5, 6)];
        let mut slice_box: Box<[Srgb<u8>]> =
            vec![Srgb::new(1, 2, 3), Srgb::new(4, 5, 6)].into_boxed_slice();
        let mut vec: Vec<Srgb<u8>> = vec![Srgb::new(1, 2, 3), Srgb::new(4, 5, 6)];
        let mut array: [Srgb<u8>; 2] = [Srgb::new(1, 2, 3), Srgb::new(4, 5, 6)];

        let _: &[u8] = slice.as_components();
        let _: &[u8] = slice_box.as_components();
        let _: &[u8] = vec.as_components();
        let _: &[u8] = array.as_components();

        let _: &mut [u8] = slice_mut.as_components_mut();
        let _: &mut [u8] = slice_box.as_components_mut();
        let _: &mut [u8] = vec.as_components_mut();
        let _: &mut [u8] = array.as_components_mut();
    }

    #[test]
    fn try_components_as() {
        let slice: &[u8] = &[1, 2, 3, 4, 5, 6];
        let slice_mut: &mut [u8] = &mut [1, 2, 3, 4, 5, 6];
        let mut slice_box: Box<[u8]> = vec![1, 2, 3, 4, 5, 6].into_boxed_slice();
        let mut vec: Vec<u8> = vec![1, 2, 3, 4, 5, 6];
        let mut array: [u8; 6] = [1, 2, 3, 4, 5, 6];

        let _: &[Srgb<u8>] = slice.try_components_as().unwrap();
        let _: &[Srgb<u8>] = slice_box.try_components_as().unwrap();
        let _: &[Srgb<u8>] = vec.try_components_as().unwrap();
        let _: &[Srgb<u8>] = array.try_components_as().unwrap();

        let _: &mut [Srgb<u8>] = slice_mut.try_components_as_mut().unwrap();
        let _: &mut [Srgb<u8>] = slice_box.try_components_as_mut().unwrap();
        let _: &mut [Srgb<u8>] = vec.try_components_as_mut().unwrap();
        let _: &mut [Srgb<u8>] = array.try_components_as_mut().unwrap();
    }

    #[test]
    fn components_as() {
        let slice: &[u8] = &[1, 2, 3, 4, 5, 6];
        let slice_mut: &mut [u8] = &mut [1, 2, 3, 4, 5, 6];
        let mut slice_box: Box<[u8]> = vec![1, 2, 3, 4, 5, 6].into_boxed_slice();
        let mut vec: Vec<u8> = vec![1, 2, 3, 4, 5, 6];
        let mut array: [u8; 6] = [1, 2, 3, 4, 5, 6];

        let _: &[Srgb<u8>] = slice.components_as();
        let _: &[Srgb<u8>] = slice_box.components_as();
        let _: &[Srgb<u8>] = vec.components_as();
        let _: &[Srgb<u8>] = array.components_as();

        let _: &mut [Srgb<u8>] = slice_mut.components_as_mut();
        let _: &mut [Srgb<u8>] = slice_box.components_as_mut();
        let _: &mut [Srgb<u8>] = vec.components_as_mut();
        let _: &mut [Srgb<u8>] = array.components_as_mut();
    }
}
