use core::{convert::Infallible, fmt::Debug};

use crate::ArrayExt;

use super::{
    from_component_array, into_component_array, into_component_slice, into_component_slice_mut,
    try_from_component_slice, try_from_component_slice_mut, ArrayCast, SliceCastError,
};

#[cfg(feature = "alloc")]
use super::{
    into_component_slice_box, into_component_vec, try_from_component_slice_box,
    try_from_component_vec, BoxedSliceCastError, VecCastError,
};

/// Trait for trying to cast a collection of colors from a collection of color
/// components without copying.
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
/// use palette::{cast::TryFromComponents, Srgb};
///
/// let array: [_; 6] = [64, 139, 10, 93, 18, 214];
/// let slice: &[_] = &[64, 139, 10, 93, 18, 214];
/// let slice_mut: &mut [_] = &mut [64, 139, 10, 93, 18, 214];
/// let vec: Vec<_> = vec![64, 139, 10, 93, 18, 214];
///
/// assert_eq!(
///     <[Srgb<u8>; 2]>::try_from_components(array),
///     Ok([Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)])
/// );
///
/// assert_eq!(
///     <&[Srgb<u8>]>::try_from_components(slice),
///     Ok([Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)].as_ref())
/// );
///
/// assert_eq!(
///     <&mut [Srgb<u8>]>::try_from_components(slice_mut),
///     Ok([Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)].as_mut())
/// );
///
/// assert_eq!(
///     Vec::<Srgb<u8>>::try_from_components(vec),
///     Ok(vec![Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)])
/// );
/// ```
///
/// Owning types can be cast as slices, too:
///
/// ```
/// use palette::{cast::TryFromComponents, Srgb};
///
/// let array: [_; 6] = [64, 139, 10, 93, 18, 214];
/// let mut vec: Vec<_> = vec![64, 139, 10, 93, 18, 214];
///
/// assert_eq!(
///     <&[Srgb<u8>]>::try_from_components(&array),
///     Ok([Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)].as_ref())
/// );
///
/// assert_eq!(
///     <&mut [Srgb<u8>]>::try_from_components(&mut vec),
///     Ok([Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)].as_mut())
/// );
/// ```
///
/// This produces an error:
///
/// ```
/// use palette::{cast::TryFromComponents, Srgb};
///
/// let components = &[64, 139, 10, 93, 18]; // Not a multiple of 3
/// assert!(<&[Srgb<u8>]>::try_from_components(components).is_err());
/// ```
pub trait TryFromComponents<C>: Sized {
    /// The error for when `try_from_components` fails to cast.
    type Error;

    /// Try to cast a collection of color components into an collection of
    /// colors.
    ///
    /// Return an error if the conversion can't be done, such as when the number
    /// of items in `components` isn't a multiple of the number of components in
    /// the color type.
    fn try_from_components(components: C) -> Result<Self, Self::Error>;
}

impl<T, C, const N: usize, const M: usize> TryFromComponents<[T; N]> for [C; M]
where
    C: ArrayCast,
    C::Array: ArrayExt<Item = T>,
{
    type Error = Infallible; // We don't provide a `try_*` option for arrays.

    #[inline]
    fn try_from_components(components: [T; N]) -> Result<Self, Self::Error> {
        Ok(from_component_array(components))
    }
}

macro_rules! impl_try_from_components_slice {
    ($($owning:ty $(where ($($ty_input:tt)+))?),*) => {
        $(
            impl<'a, T, C $(, $($ty_input)+)?> TryFromComponents<&'a $owning> for &'a [C]
            where
                T: 'a,
                C: ArrayCast,
                C::Array: ArrayExt<Item = T>,
            {
                type Error = SliceCastError;

                #[inline]
                fn try_from_components(components: &'a $owning) -> Result<Self, Self::Error> {
                    try_from_component_slice(components)
                }
            }

            impl<'a, T, C $(, $($ty_input)+)?> TryFromComponents<&'a mut $owning> for &'a mut [C]
            where
                T: 'a,
                C: ArrayCast,
                C::Array: ArrayExt<Item = T>,
            {
                type Error = SliceCastError;

                #[inline]
                fn try_from_components(components: &'a mut $owning) -> Result<Self, Self::Error> {
                    try_from_component_slice_mut(components)
                }
            }
        )*
    };
}

impl_try_from_components_slice!([T], [T; N] where (const N: usize));

#[cfg(feature = "alloc")]
impl_try_from_components_slice!(alloc::boxed::Box<[T]>, alloc::vec::Vec<T>);

#[cfg(feature = "alloc")]
impl<T, C> TryFromComponents<alloc::boxed::Box<[T]>> for alloc::boxed::Box<[C]>
where
    C: ArrayCast,
    C::Array: ArrayExt<Item = T>,
{
    type Error = BoxedSliceCastError<T>;

    #[inline]
    fn try_from_components(components: alloc::boxed::Box<[T]>) -> Result<Self, Self::Error> {
        try_from_component_slice_box(components)
    }
}

#[cfg(feature = "alloc")]
impl<T, C> TryFromComponents<alloc::vec::Vec<T>> for alloc::vec::Vec<C>
where
    C: ArrayCast,
    C::Array: ArrayExt<Item = T>,
{
    type Error = VecCastError<T>;

    #[inline]
    fn try_from_components(components: alloc::vec::Vec<T>) -> Result<Self, Self::Error> {
        try_from_component_vec(components)
    }
}

/// Trait for casting a collection of colors from a collection of color
/// components without copying.
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
/// use palette::{cast::FromComponents, Srgb};
///
/// let array: [_; 6] = [64, 139, 10, 93, 18, 214];
/// let slice: &[_] = &[64, 139, 10, 93, 18, 214];
/// let slice_mut: &mut [_] = &mut [64, 139, 10, 93, 18, 214];
/// let vec: Vec<_> = vec![64, 139, 10, 93, 18, 214];
///
/// assert_eq!(
///     <[Srgb<u8>; 2]>::from_components(array),
///     [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)]
/// );
///
/// assert_eq!(
///     <&[Srgb<u8>]>::from_components(slice),
///     [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)]
/// );
///
/// assert_eq!(
///     <&mut [Srgb<u8>]>::from_components(slice_mut),
///     [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)]
/// );
///
/// assert_eq!(
///     Vec::<Srgb<u8>>::from_components(vec),
///     vec![Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)]
/// );
/// ```
///
/// Owning types can be cast as slices, too:
///
/// ```
/// use palette::{cast::FromComponents, Srgb};
///
/// let array: [_; 6] = [64, 139, 10, 93, 18, 214];
/// let mut vec: Vec<_> = vec![64, 139, 10, 93, 18, 214];
///
/// assert_eq!(
///     <&[Srgb<u8>]>::from_components(&array),
///     [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)]
/// );
///
/// assert_eq!(
///     <&mut [Srgb<u8>]>::from_components(&mut vec),
///     [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)]
/// );
/// ```
///
/// This panics:
///
/// ```should_panic
/// use palette::{cast::FromComponents, Srgb};
///
/// let components = &[64, 139, 10, 93, 18, 214, 0, 123]; // Not a multiple of 3
/// <&[Srgb<u8>]>::from_components(components);
/// ```
pub trait FromComponents<C> {
    /// Cast a collection of color components into an collection of colors.
    ///
    /// ## Panics
    /// If the conversion can't be done, such as when the number of items in
    /// `components` isn't a multiple of the number of components in the color
    /// type.
    fn from_components(components: C) -> Self;
}

impl<T, C> FromComponents<C> for T
where
    T: TryFromComponents<C>,
    T::Error: Debug,
{
    #[inline]
    fn from_components(components: C) -> Self {
        Self::try_from_components(components).unwrap()
    }
}

/// Trait for casting a collection of colors into a collection of color
/// components without copying.
///
/// This trait is meant as a more convenient alternative to the free functions
/// in [`cast`][crate::cast], to allow method chaining among other things.
///
/// ## Examples
///
/// ```
/// use palette::{cast::IntoComponents, Srgb};
///
/// let array: [_; 2] = [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)];
/// let slice: &[_] = &[Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)];
/// let slice_mut: &mut [_] = &mut [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)];
/// let vec: Vec<_> = vec![Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)];
///
/// assert_eq!(array.into_components(), [64, 139, 10, 93, 18, 214]);
/// assert_eq!(slice.into_components(), [64, 139, 10, 93, 18, 214]);
/// assert_eq!(slice_mut.into_components(), [64, 139, 10, 93, 18, 214]);
/// assert_eq!(vec.into_components(), vec![64, 139, 10, 93, 18, 214]);
/// ```
///
/// Owning types can be cast as slices, too:
///
/// ```
/// use palette::{cast::IntoComponents, Srgb};
///
/// let array: [_; 2] = [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)];
/// let mut vec: Vec<_> = vec![Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)];
///
/// assert_eq!((&array).into_components(), [64, 139, 10, 93, 18, 214]);
/// assert_eq!((&mut vec).into_components(), [64, 139, 10, 93, 18, 214]);
/// ```
pub trait IntoComponents<C> {
    /// Cast this collection of colors into a collection of color components.
    fn into_components(self) -> C;
}

impl<T, C, const N: usize, const M: usize> IntoComponents<[T; M]> for [C; N]
where
    C: ArrayCast,
    C::Array: ArrayExt<Item = T>,
{
    #[inline]
    fn into_components(self) -> [T; M] {
        into_component_array(self)
    }
}

macro_rules! impl_into_components_slice {
    ($($owning:ty $(where ($($ty_input:tt)+))?),*) => {
        $(
            impl<'a, T, C $(, $($ty_input)+)?> IntoComponents<&'a [T]> for &'a $owning
            where
                T: 'a,
                C: ArrayCast,
                C::Array: ArrayExt<Item = T>,
            {
                #[inline]
                fn into_components(self) -> &'a [T]  {
                    into_component_slice(self)
                }
            }

            impl<'a, T, C $(, $($ty_input)+)?> IntoComponents<&'a mut [T]> for &'a mut $owning
            where
                T: 'a,
                C: ArrayCast,
                C::Array: ArrayExt<Item = T>,
            {
                #[inline]
                fn into_components(self) -> &'a mut [T] {
                    into_component_slice_mut(self)
                }
            }
        )*
    };
}

impl_into_components_slice!([C], [C; N] where (const N: usize));

#[cfg(feature = "alloc")]
impl_into_components_slice!(alloc::boxed::Box<[C]>, alloc::vec::Vec<C>);

#[cfg(feature = "alloc")]
impl<T, C> IntoComponents<alloc::boxed::Box<[T]>> for alloc::boxed::Box<[C]>
where
    C: ArrayCast,
    C::Array: ArrayExt<Item = T>,
{
    #[inline]
    fn into_components(self) -> alloc::boxed::Box<[T]> {
        into_component_slice_box(self)
    }
}

#[cfg(feature = "alloc")]
impl<T, C> IntoComponents<alloc::vec::Vec<T>> for alloc::vec::Vec<C>
where
    C: ArrayCast,
    C::Array: ArrayExt<Item = T>,
{
    #[inline]
    fn into_components(self) -> alloc::vec::Vec<T> {
        into_component_vec(self)
    }
}

/// Trait for casting a collection of color components into a collection of
/// colors without copying.
///
/// This trait is meant as a more convenient alternative to the free functions
/// in [`cast`][crate::cast], to allow method chaining among other things.
///
/// ## Examples
///
/// ```
/// use palette::{cast::ComponentsFrom, Srgb};
///
/// let array: [_; 2] = [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)];
/// let slice: &[_] = &[Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)];
/// let slice_mut: &mut [_] = &mut [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)];
/// let vec: Vec<_> = vec![Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)];
///
/// assert_eq!(<[_; 6]>::components_from(array), [64, 139, 10, 93, 18, 214]);
/// assert_eq!(<&[_]>::components_from(slice), [64, 139, 10, 93, 18, 214]);
/// assert_eq!(<&mut [_]>::components_from(slice_mut), [64, 139, 10, 93, 18, 214]);
/// assert_eq!(Vec::<_>::components_from(vec), vec![64, 139, 10, 93, 18, 214]);
/// ```
///
/// Owning types can be cast as slices, too:
///
/// ```
/// use palette::{cast::ComponentsFrom, Srgb};
///
/// let array: [_; 2] = [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)];
/// let mut vec: Vec<_> = vec![Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)];
///
/// assert_eq!(<&[_]>::components_from(&array), [64, 139, 10, 93, 18, 214]);
/// assert_eq!(<&mut [_]>::components_from(&mut vec), [64, 139, 10, 93, 18, 214]);
/// ```
pub trait ComponentsFrom<C> {
    /// Cast a collection of colors into a collection of color components.
    fn components_from(colors: C) -> Self;
}

impl<T, C> ComponentsFrom<C> for T
where
    C: IntoComponents<T>,
{
    #[inline]
    fn components_from(colors: C) -> Self {
        colors.into_components()
    }
}

/// Trait for trying to cast a collection of color components from a collection
/// of colors without copying.
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
/// use palette::{cast::TryComponentsInto, Srgb};
///
/// let array: [_; 6] = [64, 139, 10, 93, 18, 214];
/// let slice: &[_] = &[64, 139, 10, 93, 18, 214];
/// let slice_mut: &mut [_] = &mut [64, 139, 10, 93, 18, 214];
/// let vec: Vec<_> = vec![64, 139, 10, 93, 18, 214];
///
/// assert_eq!(
///     array.try_components_into(),
///     Ok([Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)])
/// );
///
/// assert_eq!(
///     slice.try_components_into(),
///     Ok([Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)].as_ref())
/// );
///
/// assert_eq!(
///     slice_mut.try_components_into(),
///     Ok([Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)].as_mut())
/// );
///
/// assert_eq!(
///     vec.try_components_into(),
///     Ok(vec![Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)])
/// );
/// ```
///
/// Owning types can be cast as slices, too:
///
/// ```
/// use palette::{cast::TryComponentsInto, Srgb};
///
/// let array: [_; 6] = [64, 139, 10, 93, 18, 214];
/// let mut vec: Vec<_> = vec![64, 139, 10, 93, 18, 214];
///
/// assert_eq!(
///     (&array).try_components_into(),
///     Ok([Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)].as_ref())
/// );
///
/// assert_eq!(
///     (&mut vec).try_components_into(),
///     Ok([Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)].as_mut())
/// );
/// ```
///
/// This produces an error:
///
/// ```
/// use palette::{cast::TryComponentsInto, Srgb};
///
/// let components = &[64, 139, 10, 93, 18]; // Not a multiple of 3
/// let colors: Result<&[Srgb<u8>], _> = components.try_components_into();
/// assert!(colors.is_err());
/// ```
pub trait TryComponentsInto<C>: Sized {
    /// The error for when `try_into_colors` fails to cast.
    type Error;

    /// Try to cast this collection of color components into a collection of
    /// colors.
    ///
    /// Return an error if the conversion can't be done, such as when the number
    /// of items in `self` isn't a multiple of the number of components in the
    /// color type.
    fn try_components_into(self) -> Result<C, Self::Error>;
}

/// Trait for casting a collection of color components from a collection of
/// colors without copying.
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
/// use palette::{cast::ComponentsInto, Srgb};
///
/// let array: [_; 6] = [64, 139, 10, 93, 18, 214];
/// let slice: &[_] = &[64, 139, 10, 93, 18, 214];
/// let slice_mut: &mut [_] = &mut [64, 139, 10, 93, 18, 214];
/// let vec: Vec<_> = vec![64, 139, 10, 93, 18, 214];
///
/// let colors: [Srgb<u8>; 2] = array.components_into();
/// assert_eq!(colors, [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)]);
///
/// let colors: &[Srgb<u8>] = slice.components_into();
/// assert_eq!(colors, [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)]);
///
/// let colors: &mut [Srgb<u8>] = slice_mut.components_into();
/// assert_eq!(colors, [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)]);
///
/// let colors: Vec<Srgb<u8>> = vec.components_into();
/// assert_eq!(colors, vec![Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)]);
/// ```
///
/// Owning types can be cast as slices, too:
///
/// ```
/// use palette::{cast::ComponentsInto, Srgb};
///
/// let array: [_; 6] = [64, 139, 10, 93, 18, 214];
/// let mut vec: Vec<_> = vec![64, 139, 10, 93, 18, 214];
///
/// let colors: &[Srgb<u8>] = (&array).components_into();
/// assert_eq!(colors, [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)]);
///
/// let colors: &mut [Srgb<u8>] = (&mut vec).components_into();
/// assert_eq!(colors, [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)]);
/// ```
///
/// This panics:
///
/// ```should_panic
/// use palette::{cast::ComponentsInto, Srgb};
///
/// let components = &[64, 139, 10, 93, 18, 214, 0, 123]; // Not a multiple of 3
/// let colors: &[Srgb<u8>] = components.components_into();
/// ```
pub trait ComponentsInto<C> {
    /// Cast this collection of color components into a collection of colors.
    ///
    /// ## Panics
    /// If the conversion can't be done, such as when the number of items in
    /// `self` isn't a multiple of the number of components in the color type.
    fn components_into(self) -> C;
}

impl<T, C> ComponentsInto<C> for T
where
    T: TryComponentsInto<C>,
    T::Error: Debug,
{
    #[inline]
    fn components_into(self) -> C {
        self.try_components_into().unwrap()
    }
}

impl<T, C> TryComponentsInto<C> for T
where
    C: TryFromComponents<T>,
{
    type Error = C::Error;

    #[inline]
    fn try_components_into(self) -> Result<C, Self::Error> {
        C::try_from_components(self)
    }
}

#[cfg(test)]
mod test {
    use crate::Srgb;

    use super::{
        ComponentsFrom, ComponentsInto, FromComponents, IntoComponents, TryComponentsInto,
        TryFromComponents,
    };

    #[test]
    fn try_from_components() {
        let slice: &[u8] = &[1, 2, 3, 4, 5, 6];
        let slice_mut: &mut [u8] = &mut [1, 2, 3, 4, 5, 6];
        let mut array: [u8; 6] = [1, 2, 3, 4, 5, 6];

        let _ = <&[Srgb<u8>]>::try_from_components(slice).unwrap();
        let _ = <&[Srgb<u8>]>::try_from_components(&array).unwrap();

        let _ = <&mut [Srgb<u8>]>::try_from_components(slice_mut).unwrap();
        let _ = <&mut [Srgb<u8>]>::try_from_components(&mut array).unwrap();

        let _ = <[Srgb<u8>; 2]>::try_from_components(array).unwrap();
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn try_from_components_alloc() {
        let mut slice_box: Box<[u8]> = vec![1, 2, 3, 4, 5, 6].into_boxed_slice();
        let mut vec: Vec<u8> = vec![1, 2, 3, 4, 5, 6];

        let _ = <&[Srgb<u8>]>::try_from_components(&slice_box).unwrap();
        let _ = <&[Srgb<u8>]>::try_from_components(&vec).unwrap();

        let _ = <&mut [Srgb<u8>]>::try_from_components(&mut slice_box).unwrap();
        let _ = <&mut [Srgb<u8>]>::try_from_components(&mut vec).unwrap();

        let _ = Box::<[Srgb<u8>]>::try_from_components(slice_box).unwrap();
        let _ = Vec::<Srgb<u8>>::try_from_components(vec).unwrap();
    }

    #[test]
    fn try_components_into() {
        let slice: &[u8] = &[1, 2, 3, 4, 5, 6];
        let slice_mut: &mut [u8] = &mut [1, 2, 3, 4, 5, 6];
        let mut array: [u8; 6] = [1, 2, 3, 4, 5, 6];

        let _: &[Srgb<u8>] = slice.try_components_into().unwrap();
        let _: &[Srgb<u8>] = (&array).try_components_into().unwrap();

        let _: &mut [Srgb<u8>] = slice_mut.try_components_into().unwrap();
        let _: &mut [Srgb<u8>] = (&mut array).try_components_into().unwrap();

        let _: [Srgb<u8>; 2] = array.try_components_into().unwrap();
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn try_components_into_alloc() {
        let mut slice_box: Box<[u8]> = vec![1, 2, 3, 4, 5, 6].into_boxed_slice();
        let mut vec: Vec<u8> = vec![1, 2, 3, 4, 5, 6];

        let _: &[Srgb<u8>] = (&slice_box).try_components_into().unwrap();
        let _: &[Srgb<u8>] = (&vec).try_components_into().unwrap();

        let _: &mut [Srgb<u8>] = (&mut slice_box).try_components_into().unwrap();
        let _: &mut [Srgb<u8>] = (&mut vec).try_components_into().unwrap();

        let _: Box<[Srgb<u8>]> = slice_box.try_components_into().unwrap();
        let _: Vec<Srgb<u8>> = vec.try_components_into().unwrap();
    }

    #[test]
    fn from_components() {
        let slice: &[u8] = &[1, 2, 3, 4, 5, 6];
        let slice_mut: &mut [u8] = &mut [1, 2, 3, 4, 5, 6];
        let mut array: [u8; 6] = [1, 2, 3, 4, 5, 6];

        let _ = <&[Srgb<u8>]>::from_components(slice);
        let _ = <&[Srgb<u8>]>::from_components(&array);

        let _ = <&mut [Srgb<u8>]>::from_components(slice_mut);
        let _ = <&mut [Srgb<u8>]>::from_components(&mut array);

        let _ = <[Srgb<u8>; 2]>::from_components(array);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn from_components_alloc() {
        let mut slice_box: Box<[u8]> = vec![1, 2, 3, 4, 5, 6].into_boxed_slice();
        let mut vec: Vec<u8> = vec![1, 2, 3, 4, 5, 6];

        let _ = <&[Srgb<u8>]>::from_components(&slice_box);
        let _ = <&[Srgb<u8>]>::from_components(&vec);

        let _ = <&mut [Srgb<u8>]>::from_components(&mut slice_box);
        let _ = <&mut [Srgb<u8>]>::from_components(&mut vec);

        let _ = Box::<[Srgb<u8>]>::from_components(slice_box);
        let _ = Vec::<Srgb<u8>>::from_components(vec);
    }

    #[test]
    fn components_into() {
        let slice: &[u8] = &[1, 2, 3, 4, 5, 6];
        let slice_mut: &mut [u8] = &mut [1, 2, 3, 4, 5, 6];
        let mut array: [u8; 6] = [1, 2, 3, 4, 5, 6];

        let _: &[Srgb<u8>] = slice.components_into();
        let _: &[Srgb<u8>] = (&array).components_into();

        let _: &mut [Srgb<u8>] = slice_mut.components_into();
        let _: &mut [Srgb<u8>] = (&mut array).components_into();

        let _: [Srgb<u8>; 2] = array.components_into();
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn components_into_alloc() {
        let mut slice_box: Box<[u8]> = vec![1, 2, 3, 4, 5, 6].into_boxed_slice();
        let mut vec: Vec<u8> = vec![1, 2, 3, 4, 5, 6];

        let _: &[Srgb<u8>] = (&slice_box).components_into();
        let _: &[Srgb<u8>] = (&vec).components_into();

        let _: &mut [Srgb<u8>] = (&mut slice_box).components_into();
        let _: &mut [Srgb<u8>] = (&mut vec).components_into();

        let _: Box<[Srgb<u8>]> = slice_box.components_into();
        let _: Vec<Srgb<u8>> = vec.components_into();
    }

    #[test]
    fn into_components() {
        let slice: &[Srgb<u8>] = &[Srgb::new(1, 2, 3), Srgb::new(4, 5, 6)];
        let slice_mut: &mut [Srgb<u8>] = &mut [Srgb::new(1, 2, 3), Srgb::new(4, 5, 6)];
        let mut array: [Srgb<u8>; 2] = [Srgb::new(1, 2, 3), Srgb::new(4, 5, 6)];

        let _: &[u8] = slice.into_components();
        let _: &[u8] = (&array).into_components();

        let _: &mut [u8] = slice_mut.into_components();
        let _: &mut [u8] = (&mut array).into_components();

        let _: [u8; 6] = array.into_components();
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn into_components_alloc() {
        let mut slice_box: Box<[Srgb<u8>]> =
            vec![Srgb::new(1, 2, 3), Srgb::new(4, 5, 6)].into_boxed_slice();
        let mut vec: Vec<Srgb<u8>> = vec![Srgb::new(1, 2, 3), Srgb::new(4, 5, 6)];

        let _: &[u8] = (&slice_box).into_components();
        let _: &[u8] = (&vec).into_components();

        let _: &mut [u8] = (&mut slice_box).into_components();
        let _: &mut [u8] = (&mut vec).into_components();

        let _: Box<[u8]> = slice_box.into_components();
        let _: Vec<u8> = vec.into_components();
    }

    #[test]
    fn components_from() {
        let slice: &[Srgb<u8>] = &[Srgb::new(1, 2, 3), Srgb::new(4, 5, 6)];
        let slice_mut: &mut [Srgb<u8>] = &mut [Srgb::new(1, 2, 3), Srgb::new(4, 5, 6)];
        let mut array: [Srgb<u8>; 2] = [Srgb::new(1, 2, 3), Srgb::new(4, 5, 6)];

        let _ = <&[u8]>::components_from(slice);
        let _ = <&[u8]>::components_from(&array);

        let _ = <&mut [u8]>::components_from(slice_mut);
        let _ = <&mut [u8]>::components_from(&mut array);

        let _ = <[u8; 6]>::components_from(array);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn components_from_alloc() {
        let mut slice_box: Box<[Srgb<u8>]> =
            vec![Srgb::new(1, 2, 3), Srgb::new(4, 5, 6)].into_boxed_slice();
        let mut vec: Vec<Srgb<u8>> = vec![Srgb::new(1, 2, 3), Srgb::new(4, 5, 6)];

        let _ = <&[u8]>::components_from(&slice_box);
        let _ = <&[u8]>::components_from(&vec);

        let _ = <&mut [u8]>::components_from(&mut slice_box);
        let _ = <&mut [u8]>::components_from(&mut vec);

        let _ = Box::<[u8]>::components_from(slice_box);
        let _ = Vec::<u8>::components_from(vec);
    }
}
