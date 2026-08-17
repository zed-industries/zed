use super::{
    from_uint_array, from_uint_slice, from_uint_slice_mut, into_uint_array, into_uint_slice,
    into_uint_slice_mut, UintCast,
};

#[cfg(feature = "alloc")]
use super::{from_uint_slice_box, from_uint_vec, into_uint_slice_box, into_uint_vec};

/// Trait for casting a collection of colors from a collection of unsigned
/// integers without copying.
///
/// This trait is meant as a more convenient alternative to the free functions
/// in [`cast`][crate::cast], to allow method chaining among other things.
///
/// ## Examples
///
/// ```
/// use palette::{cast::FromUints, rgb::PackedArgb, Srgba};
///
/// let array: [_; 2] = [0xFF17C64C, 0xFF5D12D6];
/// let slice: &[_] = &[0xFF17C64C, 0xFF5D12D6];
/// let slice_mut: &mut [_] = &mut [0xFF17C64C, 0xFF5D12D6];
/// let vec: Vec<_> = vec![0xFF17C64C, 0xFF5D12D6];
///
/// assert_eq!(
///     <[PackedArgb; 2]>::from_uints(array),
///     [
///         Srgba::new(0x17, 0xC6, 0x4C, 0xFF).into(),
///         Srgba::new(0x5D, 0x12, 0xD6, 0xFF).into()
///     ]
/// );
///
/// assert_eq!(
///     <&[PackedArgb]>::from_uints(slice),
///     [
///         Srgba::new(0x17, 0xC6, 0x4C, 0xFF).into(),
///         Srgba::new(0x5D, 0x12, 0xD6, 0xFF).into()
///     ]
/// );
///
/// assert_eq!(
///     <&mut [PackedArgb]>::from_uints(slice_mut),
///     [
///         Srgba::new(0x17, 0xC6, 0x4C, 0xFF).into(),
///         Srgba::new(0x5D, 0x12, 0xD6, 0xFF).into()
///     ]
/// );
///
/// assert_eq!(
///     Vec::<PackedArgb>::from_uints(vec),
///     vec![
///         Srgba::new(0x17, 0xC6, 0x4C, 0xFF).into(),
///         Srgba::new(0x5D, 0x12, 0xD6, 0xFF).into()
///     ]
/// );
/// ```
///
/// Owning types can be cast as slices, too:
///
/// ```
/// use palette::{cast::FromUints, rgb::PackedArgb, Srgba};
///
/// let array: [_; 2] = [0xFF17C64C, 0xFF5D12D6];
/// let mut vec: Vec<_> = vec![0xFF17C64C, 0xFF5D12D6];
///
/// assert_eq!(
///     <&[PackedArgb]>::from_uints(&array),
///     [
///         Srgba::new(0x17, 0xC6, 0x4C, 0xFF).into(),
///         Srgba::new(0x5D, 0x12, 0xD6, 0xFF).into()
///     ]
/// );
///
/// assert_eq!(
///     <&mut [PackedArgb]>::from_uints(&mut vec),
///     [
///         Srgba::new(0x17, 0xC6, 0x4C, 0xFF).into(),
///         Srgba::new(0x5D, 0x12, 0xD6, 0xFF).into()
///     ]
/// );
/// ```
pub trait FromUints<U> {
    /// Cast a collection of unsigned integers into an collection of colors.
    fn from_uints(uints: U) -> Self;
}

impl<C, const N: usize> FromUints<[C::Uint; N]> for [C; N]
where
    C: UintCast,
{
    #[inline]
    fn from_uints(uints: [C::Uint; N]) -> Self {
        from_uint_array(uints)
    }
}

macro_rules! impl_from_uints_slice {
    ($($owning:ty $(where ($($ty_input:tt)+))?),*) => {
        $(
            impl<'a, C $(, $($ty_input)+)?> FromUints<&'a $owning> for &'a [C]
            where
                C: UintCast,
            {
                #[inline]
                fn from_uints(uints: &'a $owning) -> Self {
                    from_uint_slice(uints)
                }
            }

            impl<'a, C $(, $($ty_input)+)?> FromUints<&'a mut $owning> for &'a mut [C]
            where
                C: UintCast,
            {
                #[inline]
                fn from_uints(uints: &'a mut $owning) -> Self {
                    from_uint_slice_mut(uints)
                }
            }
        )*
    };
}

impl_from_uints_slice!([C::Uint], [C::Uint; N] where (const N: usize));

#[cfg(feature = "alloc")]
impl_from_uints_slice!(alloc::boxed::Box<[C::Uint]>, alloc::vec::Vec<C::Uint>);

#[cfg(feature = "alloc")]
impl<C> FromUints<alloc::boxed::Box<[C::Uint]>> for alloc::boxed::Box<[C]>
where
    C: UintCast,
{
    #[inline]
    fn from_uints(uints: alloc::boxed::Box<[C::Uint]>) -> Self {
        from_uint_slice_box(uints)
    }
}

#[cfg(feature = "alloc")]
impl<C> FromUints<alloc::vec::Vec<C::Uint>> for alloc::vec::Vec<C>
where
    C: UintCast,
{
    #[inline]
    fn from_uints(uints: alloc::vec::Vec<C::Uint>) -> Self {
        from_uint_vec(uints)
    }
}

/// Trait for casting a collection of colors into a collection of unsigned
/// integers without copying.
///
/// This trait is meant as a more convenient alternative to the free functions
/// in [`cast`][crate::cast], to allow method chaining among other things.
///
/// ## Examples
///
/// ```
/// use palette::{cast::IntoUints, rgb::PackedArgb, Srgba};
///
/// let array: [PackedArgb; 2] = [
///     Srgba::new(0x17, 0xC6, 0x4C, 0xFF).into(),
///     Srgba::new(0x5D, 0x12, 0xD6, 0xFF).into()
/// ];
/// let slice: &[PackedArgb] = &[
///     Srgba::new(0x17, 0xC6, 0x4C, 0xFF).into(),
///     Srgba::new(0x5D, 0x12, 0xD6, 0xFF).into()
/// ];
/// let slice_mut: &mut [PackedArgb] = &mut [
///     Srgba::new(0x17, 0xC6, 0x4C, 0xFF).into(),
///     Srgba::new(0x5D, 0x12, 0xD6, 0xFF).into()
/// ];
/// let vec: Vec<PackedArgb> = vec![
///     Srgba::new(0x17, 0xC6, 0x4C, 0xFF).into(),
///     Srgba::new(0x5D, 0x12, 0xD6, 0xFF).into()
/// ];
///
/// assert_eq!(array.into_uints(), [0xFF17C64C, 0xFF5D12D6]);
/// assert_eq!(slice.into_uints(), [0xFF17C64C, 0xFF5D12D6]);
/// assert_eq!(slice_mut.into_uints(), [0xFF17C64C, 0xFF5D12D6]);
/// assert_eq!(vec.into_uints(), vec![0xFF17C64C, 0xFF5D12D6]);
/// ```
///
/// Owning types can be cast as slices, too:
///
/// ```
/// use palette::{cast::IntoUints, rgb::PackedArgb, Srgba};
///
/// let array: [PackedArgb; 2] = [
///     Srgba::new(0x17, 0xC6, 0x4C, 0xFF).into(),
///     Srgba::new(0x5D, 0x12, 0xD6, 0xFF).into()
/// ];
/// let mut vec: Vec<PackedArgb> = vec![
///     Srgba::new(0x17, 0xC6, 0x4C, 0xFF).into(),
///     Srgba::new(0x5D, 0x12, 0xD6, 0xFF).into()
/// ];
///
/// assert_eq!((&array).into_uints(), [0xFF17C64C, 0xFF5D12D6]);
/// assert_eq!((&mut vec).into_uints(), [0xFF17C64C, 0xFF5D12D6]);
/// ```
pub trait IntoUints<U> {
    /// Cast this collection of colors into a collection of unsigned integers.
    fn into_uints(self) -> U;
}

impl<C, const N: usize> IntoUints<[C::Uint; N]> for [C; N]
where
    C: UintCast,
{
    #[inline]
    fn into_uints(self) -> [C::Uint; N] {
        into_uint_array(self)
    }
}

macro_rules! impl_into_uints_slice {
    ($($owning:ty $(where ($($ty_input:tt)+))?),*) => {
        $(
            impl<'a, C $(, $($ty_input)+)?> IntoUints<&'a [C::Uint]> for &'a $owning
            where
                C: UintCast,
            {
                #[inline]
                fn into_uints(self) -> &'a [C::Uint]  {
                    into_uint_slice(self)
                }
            }

            impl<'a, C $(, $($ty_input)+)?> IntoUints<&'a mut [C::Uint]> for &'a mut $owning
            where
                C: UintCast,
            {
                #[inline]
                fn into_uints(self) -> &'a mut [C::Uint] {
                    into_uint_slice_mut(self)
                }
            }
        )*
    };
}

impl_into_uints_slice!([C], [C; M] where (const M: usize));

#[cfg(feature = "alloc")]
impl_into_uints_slice!(alloc::boxed::Box<[C]>, alloc::vec::Vec<C>);

#[cfg(feature = "alloc")]
impl<C> IntoUints<alloc::boxed::Box<[C::Uint]>> for alloc::boxed::Box<[C]>
where
    C: UintCast,
{
    #[inline]
    fn into_uints(self) -> alloc::boxed::Box<[C::Uint]> {
        into_uint_slice_box(self)
    }
}

#[cfg(feature = "alloc")]
impl<C> IntoUints<alloc::vec::Vec<C::Uint>> for alloc::vec::Vec<C>
where
    C: UintCast,
{
    #[inline]
    fn into_uints(self) -> alloc::vec::Vec<C::Uint> {
        into_uint_vec(self)
    }
}

/// Trait for casting a collection of unsigned integers from a collection of
/// colors without copying.
///
/// This trait is meant as a more convenient alternative to the free functions
/// in [`cast`][crate::cast], to allow method chaining among other things.
///
/// ## Examples
///
/// ```
/// use palette::{cast::UintsFrom, rgb::PackedArgb, Srgba};
///
/// let array: [PackedArgb; 2] = [
///     Srgba::new(0x17, 0xC6, 0x4C, 0xFF).into(),
///     Srgba::new(0x5D, 0x12, 0xD6, 0xFF).into()
/// ];
/// let slice: &[PackedArgb] = &[
///     Srgba::new(0x17, 0xC6, 0x4C, 0xFF).into(),
///     Srgba::new(0x5D, 0x12, 0xD6, 0xFF).into()
/// ];
/// let slice_mut: &mut [PackedArgb] = &mut [
///     Srgba::new(0x17, 0xC6, 0x4C, 0xFF).into(),
///     Srgba::new(0x5D, 0x12, 0xD6, 0xFF).into()
/// ];
/// let vec: Vec<PackedArgb> = vec![
///     Srgba::new(0x17, 0xC6, 0x4C, 0xFF).into(),
///     Srgba::new(0x5D, 0x12, 0xD6, 0xFF).into()
/// ];
///
/// assert_eq!(<[_; 2]>::uints_from(array), [0xFF17C64C, 0xFF5D12D6]);
/// assert_eq!(<&[_]>::uints_from(slice), [0xFF17C64C, 0xFF5D12D6]);
/// assert_eq!(<&mut [_]>::uints_from(slice_mut), [0xFF17C64C, 0xFF5D12D6]);
/// assert_eq!(Vec::<_>::uints_from(vec), vec![0xFF17C64C, 0xFF5D12D6]);
/// ```
///
/// Owning types can be cast as slices, too:
///
/// ```
/// use palette::{cast::UintsFrom, rgb::PackedArgb, Srgba};
///
/// let array: [PackedArgb; 2] = [
///     Srgba::new(0x17, 0xC6, 0x4C, 0xFF).into(),
///     Srgba::new(0x5D, 0x12, 0xD6, 0xFF).into()
/// ];
/// let mut vec: Vec<PackedArgb> = vec![
///     Srgba::new(0x17, 0xC6, 0x4C, 0xFF).into(),
///     Srgba::new(0x5D, 0x12, 0xD6, 0xFF).into()
/// ];
///
/// assert_eq!(<&[_]>::uints_from(&array), [0xFF17C64C, 0xFF5D12D6]);
/// assert_eq!(<&mut [_]>::uints_from(&mut vec), [0xFF17C64C, 0xFF5D12D6]);
/// ```
pub trait UintsFrom<C> {
    /// Cast a collection of colors into a collection of unsigned integers.
    fn uints_from(colors: C) -> Self;
}

impl<C, U> UintsFrom<C> for U
where
    C: IntoUints<U>,
{
    #[inline]
    fn uints_from(colors: C) -> Self {
        colors.into_uints()
    }
}

/// Trait for casting a collection of unsigned integers into a collection of
/// colors without copying.
///
/// This trait is meant as a more convenient alternative to the free functions
/// in [`cast`][crate::cast], to allow method chaining among other things.
///
/// ## Examples
///
/// ```
/// use palette::{cast::UintsInto, rgb::PackedArgb, Srgba};
///
/// let array: [_; 2] = [0xFF17C64C, 0xFF5D12D6];
/// let slice: &[_] = &[0xFF17C64C, 0xFF5D12D6];
/// let slice_mut: &mut [_] = &mut [0xFF17C64C, 0xFF5D12D6];
/// let vec: Vec<_> = vec![0xFF17C64C, 0xFF5D12D6];
///
/// let colors: [PackedArgb; 2] = array.uints_into();
/// assert_eq!(colors, [
///     Srgba::new(0x17, 0xC6, 0x4C, 0xFF).into(),
///     Srgba::new(0x5D, 0x12, 0xD6, 0xFF).into()
/// ]);
///
/// let colors: &[PackedArgb] = slice.uints_into();
/// assert_eq!(colors, [
///     Srgba::new(0x17, 0xC6, 0x4C, 0xFF).into(),
///     Srgba::new(0x5D, 0x12, 0xD6, 0xFF).into()
/// ]);
///
/// let colors: &mut [PackedArgb] = slice_mut.uints_into();
/// assert_eq!(colors, [
///     Srgba::new(0x17, 0xC6, 0x4C, 0xFF).into(),
///     Srgba::new(0x5D, 0x12, 0xD6, 0xFF).into()
/// ]);
///
/// let colors: Vec<PackedArgb> = vec.uints_into();
/// assert_eq!(colors, vec![
///     Srgba::new(0x17, 0xC6, 0x4C, 0xFF).into(),
///     Srgba::new(0x5D, 0x12, 0xD6, 0xFF).into()
/// ]);
/// ```
///
/// Owning types can be cast as slices, too:
///
/// ```
/// use palette::{cast::UintsInto, rgb::PackedArgb, Srgba};
///
/// let array: [_; 2] = [0xFF17C64C, 0xFF5D12D6];
/// let mut vec: Vec<_> = vec![0xFF17C64C, 0xFF5D12D6];
///
/// let colors: &[PackedArgb] = (&array).uints_into();
/// assert_eq!(colors, [
///     Srgba::new(0x17, 0xC6, 0x4C, 0xFF).into(),
///     Srgba::new(0x5D, 0x12, 0xD6, 0xFF).into()
/// ]);
///
/// let colors: &mut [PackedArgb] = (&mut vec).uints_into();
/// assert_eq!(colors, [
///     Srgba::new(0x17, 0xC6, 0x4C, 0xFF).into(),
///     Srgba::new(0x5D, 0x12, 0xD6, 0xFF).into()
/// ]);
/// ```
pub trait UintsInto<C> {
    /// Cast this collection of unsigned integers into a collection of colors.
    fn uints_into(self) -> C;
}

impl<C, U> UintsInto<C> for U
where
    C: FromUints<U>,
{
    #[inline]
    fn uints_into(self) -> C {
        C::from_uints(self)
    }
}

#[cfg(test)]
mod test {
    use crate::{rgb::PackedRgba, Srgba};

    use super::{FromUints, IntoUints, UintsFrom, UintsInto};

    #[test]
    fn from_uints() {
        let slice: &[u32] = &[0x01020304, 0x05060708];
        let slice_mut: &mut [u32] = &mut [0x01020304, 0x05060708];
        let mut array: [u32; 2] = [0x01020304, 0x05060708];

        let _ = <&[PackedRgba]>::from_uints(slice);
        let _ = <&[PackedRgba]>::from_uints(&array);

        let _ = <&mut [PackedRgba]>::from_uints(slice_mut);
        let _ = <&mut [PackedRgba]>::from_uints(&mut array);

        let _ = <[PackedRgba; 2]>::from_uints(array);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn from_uints_alloc() {
        let mut slice_box: Box<[u32]> = vec![0x01020304, 0x05060708].into_boxed_slice();
        let mut vec: Vec<u32> = vec![0x01020304, 0x05060708];

        let _ = <&[PackedRgba]>::from_uints(&slice_box);
        let _ = <&[PackedRgba]>::from_uints(&vec);

        let _ = <&mut [PackedRgba]>::from_uints(&mut slice_box);
        let _ = <&mut [PackedRgba]>::from_uints(&mut vec);

        let _ = Box::<[PackedRgba]>::from_uints(slice_box);
        let _ = Vec::<PackedRgba>::from_uints(vec);
    }

    #[test]
    fn uints_into() {
        let slice: &[u32] = &[0x01020304, 0x05060708];
        let slice_mut: &mut [u32] = &mut [0x01020304, 0x05060708];
        let mut array: [u32; 2] = [0x01020304, 0x05060708];

        let _: &[PackedRgba] = slice.uints_into();
        let _: &[PackedRgba] = (&array).uints_into();

        let _: &mut [PackedRgba] = slice_mut.uints_into();
        let _: &mut [PackedRgba] = (&mut array).uints_into();

        let _: [PackedRgba; 2] = array.uints_into();
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn uints_into_alloc() {
        let mut slice_box: Box<[u32]> = vec![0x01020304, 0x05060708].into_boxed_slice();
        let mut vec: Vec<u32> = vec![0x01020304, 0x05060708];

        let _: &[PackedRgba] = (&slice_box).uints_into();
        let _: &[PackedRgba] = (&vec).uints_into();

        let _: &mut [PackedRgba] = (&mut slice_box).uints_into();
        let _: &mut [PackedRgba] = (&mut vec).uints_into();

        let _: Box<[PackedRgba]> = slice_box.uints_into();
        let _: Vec<PackedRgba> = vec.uints_into();
    }

    #[test]
    fn into_uints() {
        let slice: &[PackedRgba] = &[Srgba::new(1, 2, 3, 4).into(), Srgba::new(5, 6, 7, 8).into()];
        let slice_mut: &mut [PackedRgba] =
            &mut [Srgba::new(1, 2, 3, 4).into(), Srgba::new(5, 6, 7, 8).into()];
        let mut array: [PackedRgba; 2] =
            [Srgba::new(1, 2, 3, 4).into(), Srgba::new(5, 6, 7, 8).into()];

        let _: &[u32] = slice.into_uints();
        let _: &[u32] = (&array).into_uints();

        let _: &mut [u32] = slice_mut.into_uints();
        let _: &mut [u32] = (&mut array).into_uints();

        let _: [u32; 2] = array.into_uints();
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn into_uints_alloc() {
        let mut slice_box: Box<[PackedRgba]> =
            vec![Srgba::new(1, 2, 3, 4).into(), Srgba::new(5, 6, 7, 8).into()].into_boxed_slice();
        let mut vec: Vec<PackedRgba> =
            vec![Srgba::new(1, 2, 3, 4).into(), Srgba::new(5, 6, 7, 8).into()];

        let _: &[u32] = (&slice_box).into_uints();
        let _: &[u32] = (&vec).into_uints();

        let _: &mut [u32] = (&mut slice_box).into_uints();
        let _: &mut [u32] = (&mut vec).into_uints();

        let _: Box<[u32]> = slice_box.into_uints();
        let _: Vec<u32> = vec.into_uints();
    }

    #[test]
    fn uints_from() {
        let slice: &[PackedRgba] = &[Srgba::new(1, 2, 3, 4).into(), Srgba::new(5, 6, 7, 8).into()];
        let slice_mut: &mut [PackedRgba] =
            &mut [Srgba::new(1, 2, 3, 4).into(), Srgba::new(5, 6, 7, 8).into()];
        let mut array: [PackedRgba; 2] =
            [Srgba::new(1, 2, 3, 4).into(), Srgba::new(5, 6, 7, 8).into()];

        let _ = <&[u32]>::uints_from(slice);
        let _ = <&[u32]>::uints_from(&array);

        let _ = <&mut [u32]>::uints_from(slice_mut);
        let _ = <&mut [u32]>::uints_from(&mut array);

        let _ = <[u32; 2]>::uints_from(array);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn uints_from_alloc() {
        let mut slice_box: Box<[PackedRgba]> =
            vec![Srgba::new(1, 2, 3, 4).into(), Srgba::new(5, 6, 7, 8).into()].into_boxed_slice();
        let mut vec: Vec<PackedRgba> =
            vec![Srgba::new(1, 2, 3, 4).into(), Srgba::new(5, 6, 7, 8).into()];

        let _ = <&[u32]>::uints_from(&slice_box);
        let _ = <&[u32]>::uints_from(&vec);

        let _ = <&mut [u32]>::uints_from(&mut slice_box);
        let _ = <&mut [u32]>::uints_from(&mut vec);

        let _ = Box::<[u32]>::uints_from(slice_box);
        let _ = Vec::<u32>::uints_from(vec);
    }
}
