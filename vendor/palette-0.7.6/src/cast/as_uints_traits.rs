use super::{from_uint_slice, from_uint_slice_mut, into_uint_slice, into_uint_slice_mut, UintCast};

/// Trait for casting a reference to a collection of colors into a reference to
/// a collection of unsigned integers without copying.
///
/// This trait is meant as a more convenient alternative to the free functions
/// in [`cast`][crate::cast], to allow method chaining among other things.
///
/// ## Examples
///
/// ```
/// use palette::{cast::AsUints, rgb::PackedArgb, Srgba};
///
/// let array: [PackedArgb; 2] = [
///     Srgba::new(0x17, 0xC6, 0x4C, 0xFF).into(),
///     Srgba::new(0x5D, 0x12, 0xD6, 0xFF).into()
/// ];
/// let slice: &[PackedArgb] = &[
///     Srgba::new(0x17, 0xC6, 0x4C, 0xFF).into(),
///     Srgba::new(0x5D, 0x12, 0xD6, 0xFF).into()
/// ];
/// let vec: Vec<PackedArgb> = vec![
///     Srgba::new(0x17, 0xC6, 0x4C, 0xFF).into(),
///     Srgba::new(0x5D, 0x12, 0xD6, 0xFF).into()
/// ];
///
/// assert_eq!(array.as_uints(), &[0xFF17C64C, 0xFF5D12D6]);
/// assert_eq!(slice.as_uints(), &[0xFF17C64C, 0xFF5D12D6]);
/// assert_eq!(vec.as_uints(), &[0xFF17C64C, 0xFF5D12D6]);
/// ```
pub trait AsUints<A: ?Sized> {
    /// Cast this collection of colors into a collection of unsigned integers.
    fn as_uints(&self) -> &A;
}

/// Trait for casting a mutable reference to a collection of colors into a
/// mutable reference to a collection of unsigned integers without copying.
///
/// This trait is meant as a more convenient alternative to the free functions
/// in [`cast`][crate::cast], to allow method chaining among other things.
///
/// ## Examples
///
/// ```
/// use palette::{cast::AsUintsMut, rgb::PackedArgb, Srgba};
///
/// let mut array: [PackedArgb; 2] = [
///     Srgba::new(0x17, 0xC6, 0x4C, 0xFF).into(),
///     Srgba::new(0x5D, 0x12, 0xD6, 0xFF).into()
/// ];
/// let slice_mut: &mut [PackedArgb] = &mut [
///     Srgba::new(0x17, 0xC6, 0x4C, 0xFF).into(),
///     Srgba::new(0x5D, 0x12, 0xD6, 0xFF).into()
/// ];
/// let mut vec: Vec<PackedArgb> = vec![
///     Srgba::new(0x17, 0xC6, 0x4C, 0xFF).into(),
///     Srgba::new(0x5D, 0x12, 0xD6, 0xFF).into()
/// ];
///
/// assert_eq!(array.as_uints_mut(), &mut [0xFF17C64C, 0xFF5D12D6]);
/// assert_eq!(slice_mut.as_uints_mut(), &mut [0xFF17C64C, 0xFF5D12D6]);
/// assert_eq!(vec.as_uints_mut(), &mut [0xFF17C64C, 0xFF5D12D6]);
/// ```
pub trait AsUintsMut<A: ?Sized> {
    /// Cast this collection of colors into a mutable collection of unsigned integers.
    fn as_uints_mut(&mut self) -> &mut A;
}

macro_rules! impl_as_uints {
    ($($owning:ty $(where ($($ty_input:tt)+))?),*) => {
        $(
            impl<'a, C $(, $($ty_input)+)?> AsUints<[C::Uint]> for $owning
            where
                C: UintCast,
            {
                #[inline]
                fn as_uints(&self) -> &[C::Uint] {
                    into_uint_slice(self.as_ref())
                }
            }

            impl<'a, C $(, $($ty_input)+)?> AsUintsMut<[C::Uint]> for $owning
            where
                C: UintCast,
            {
                #[inline]
                fn as_uints_mut(&mut self) -> &mut [C::Uint] {
                    into_uint_slice_mut(self.as_mut())
                }
            }
        )*
    };
}

impl_as_uints!([C], [C; N] where (const N: usize));

#[cfg(feature = "alloc")]
impl_as_uints!(alloc::boxed::Box<[C]>, alloc::vec::Vec<C>);

/// Trait for casting a reference to a collection of unsigned integers into a
/// reference to a collection of colors without copying.
///
/// This trait is meant as a more convenient alternative to the free functions
/// in [`cast`][crate::cast], to allow method chaining among other things.
///
/// ## Examples
///
/// ```
/// use palette::{cast::UintsAs, rgb::PackedArgb, Srgba};
///
/// let array: [_; 2] = [0xFF17C64C, 0xFF5D12D6];
/// let slice: &[_] = &[0xFF17C64C, 0xFF5D12D6];
/// let vec: Vec<_> = vec![0xFF17C64C, 0xFF5D12D6];
///
/// let colors: &[PackedArgb] = array.uints_as();
/// assert_eq!(
///     colors,
///     &[
///         Srgba::new(0x17, 0xC6, 0x4C, 0xFF).into(),
///         Srgba::new(0x5D, 0x12, 0xD6, 0xFF).into()
///     ]
/// );
///
/// let colors: &[PackedArgb] = slice.uints_as();
/// assert_eq!(
///     colors,
///     &[
///         Srgba::new(0x17, 0xC6, 0x4C, 0xFF).into(),
///         Srgba::new(0x5D, 0x12, 0xD6, 0xFF).into()
///     ]
/// );
///
/// let colors: &[PackedArgb] = vec.uints_as();
/// assert_eq!(
///     colors,
///     &[
///         Srgba::new(0x17, 0xC6, 0x4C, 0xFF).into(),
///         Srgba::new(0x5D, 0x12, 0xD6, 0xFF).into()
///     ]
/// );
/// ```
pub trait UintsAs<C: ?Sized> {
    /// Cast this collection of unsigned integers into a collection of colors.
    fn uints_as(&self) -> &C;
}

/// Trait for casting a mutable reference to a collection of unsigned integers
/// into a mutable reference to a collection of colors without copying.
///
/// This trait is meant as a more convenient alternative to the free functions
/// in [`cast`][crate::cast], to allow method chaining among other things.
///
/// ## Examples
///
/// ```
/// use palette::{cast::UintsAsMut, rgb::PackedArgb, Srgba};
///
/// let mut array: [_; 2] = [0xFF17C64C, 0xFF5D12D6];
/// let slice_mut: &mut [_] = &mut [0xFF17C64C, 0xFF5D12D6];
/// let mut vec: Vec<_> = vec![0xFF17C64C, 0xFF5D12D6];
///
/// let colors: &mut [PackedArgb] = array.uints_as_mut();
/// assert_eq!(
///     colors,
///     &mut [
///         Srgba::new(0x17, 0xC6, 0x4C, 0xFF).into(),
///         Srgba::new(0x5D, 0x12, 0xD6, 0xFF).into()
///     ]
/// );
///
/// let colors: &mut [PackedArgb] = slice_mut.uints_as_mut();
/// assert_eq!(
///     colors,
///     &mut [
///         Srgba::new(0x17, 0xC6, 0x4C, 0xFF).into(),
///         Srgba::new(0x5D, 0x12, 0xD6, 0xFF).into()
///     ]
/// );
///
/// let colors: &mut [PackedArgb] = vec.uints_as_mut();
/// assert_eq!(
///     colors,
///     &mut [
///         Srgba::new(0x17, 0xC6, 0x4C, 0xFF).into(),
///         Srgba::new(0x5D, 0x12, 0xD6, 0xFF).into()
///     ]
/// );
/// ```
pub trait UintsAsMut<C: ?Sized> {
    /// Cast this collection of unsigned integers into a mutable collection of colors.
    fn uints_as_mut(&mut self) -> &mut C;
}

macro_rules! impl_uints_as {
    ($($owning:ty $(where ($($ty_input:tt)+))?),*) => {
        $(
            impl<'a, C $(, $($ty_input)+)?> UintsAs<[C]> for $owning
            where
                C: UintCast,
            {
                #[inline]
                fn uints_as(&self) -> &[C] {
                    from_uint_slice(self.as_ref())
                }
            }

            impl<'a, C $(, $($ty_input)+)?> UintsAsMut<[C]> for $owning
            where
                C: UintCast,
            {
                #[inline]
                fn uints_as_mut(&mut self) -> &mut [C] {
                    from_uint_slice_mut(self.as_mut())
                }
            }
        )*
    };
}

impl_uints_as!([C::Uint], [C::Uint; N] where (const N: usize));

#[cfg(feature = "alloc")]
impl_uints_as!(alloc::boxed::Box<[C::Uint]>, alloc::vec::Vec<C::Uint>);

#[cfg(test)]
mod test {
    use crate::{rgb::PackedRgba, Srgba};

    use super::{AsUints, AsUintsMut, UintsAs, UintsAsMut};

    #[test]
    fn as_uints() {
        let slice: &[PackedRgba] = &[Srgba::new(1, 2, 3, 4).into(), Srgba::new(5, 6, 7, 8).into()];
        let slice_mut: &mut [PackedRgba] =
            &mut [Srgba::new(1, 2, 3, 4).into(), Srgba::new(5, 6, 7, 8).into()];
        let mut slice_box: Box<[PackedRgba]> =
            vec![Srgba::new(1, 2, 3, 4).into(), Srgba::new(5, 6, 7, 8).into()].into_boxed_slice();
        let mut vec: Vec<PackedRgba> =
            vec![Srgba::new(1, 2, 3, 4).into(), Srgba::new(5, 6, 7, 8).into()];
        let mut array: [PackedRgba; 2] =
            [Srgba::new(1, 2, 3, 4).into(), Srgba::new(5, 6, 7, 8).into()];

        let _: &[u32] = slice.as_uints();
        let _: &[u32] = slice_box.as_uints();
        let _: &[u32] = vec.as_uints();
        let _: &[u32] = array.as_uints();

        let _: &mut [u32] = slice_mut.as_uints_mut();
        let _: &mut [u32] = slice_box.as_uints_mut();
        let _: &mut [u32] = vec.as_uints_mut();
        let _: &mut [u32] = array.as_uints_mut();
    }

    #[test]
    fn uints_as() {
        let slice: &[u32] = &[0x01020304, 0x05060708];
        let slice_mut: &mut [u32] = &mut [0x01020304, 0x05060708];
        let mut slice_box: Box<[u32]> = vec![0x01020304, 0x05060708].into_boxed_slice();
        let mut vec: Vec<u32> = vec![0x01020304, 0x05060708];
        let mut array: [u32; 2] = [0x01020304, 0x05060708];

        let _: &[PackedRgba] = slice.uints_as();
        let _: &[PackedRgba] = slice_box.uints_as();
        let _: &[PackedRgba] = vec.uints_as();
        let _: &[PackedRgba] = array.uints_as();

        let _: &mut [PackedRgba] = slice_mut.uints_as_mut();
        let _: &mut [PackedRgba] = slice_box.uints_as_mut();
        let _: &mut [PackedRgba] = vec.uints_as_mut();
        let _: &mut [PackedRgba] = array.uints_as_mut();
    }
}
