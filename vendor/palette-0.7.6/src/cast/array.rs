use core::mem::{transmute_copy, ManuallyDrop};

#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::{boxed::Box, vec::Vec};

pub use palette_derive::ArrayCast;

use crate::ArrayExt;

/// Marker trait for types that can be represented as a fixed size array.
///
/// A type that implements this trait is assumed to have the exact same memory
/// layout and representation as a fixed size array. This implies a couple of
/// useful properties:
///
/// * Casting between `T` and `T::Array` is free and will (or should) be
///   optimized away.
/// * `[T]` can be cast to and from `[T::Array]`, which can be cast to and from
///   `[U]` where `T::Array = [U; N]` and the length is a multiple of `N`.
///
/// This allows a number of common and useful optimizations, including casting
/// buffers and reusing memory. It does however come with some strict
/// requirements and the recommendation is to use `#[derive(ArrayCast)]` which
/// checks for them automatically.
///
/// # Deriving
///
/// `ArrayCast` can be automatically derived. The only requirements are that the
/// type is a `struct`, that it has a `#[repr(C)]` or `#[repr(transparent)]`
/// attribute, and that all of its fields have the same types. It stays on the
/// conservative side and will show an error if any of those requirements are
/// not fulfilled. If some fields have different types, but the same memory
/// layout, or are zero-sized, they can be marked with attributes to show that
/// their types are safe to use.
///
/// ## Field Attributes
///
/// * `#[palette_unsafe_same_layout_as = "SomeType"]`: Mark the field as having
///   the same memory layout as `SomeType`.
///
///   **Safety:** corrupt data and undefined behavior may occur if this is not
///   true!
///
/// * `#[palette_unsafe_zero_sized]`: Mark the field as being zero-sized, and
///   thus not taking up any memory space. This means that it can be ignored.
///
///   **Safety:** corrupt data and undefined behavior may occur if this is not
///   true!
///
/// ## Examples
///
/// Basic use:
///
/// ```rust
/// use palette::cast::{self, ArrayCast};
///
/// #[derive(PartialEq, Debug, ArrayCast)]
/// #[repr(C)]
/// struct MyCmyk {
///     cyan: f32,
///     magenta: f32,
///     yellow: f32,
///     key: f32,
/// }
///
/// let buffer = [0.1, 0.2, 0.3, 0.4];
/// let color: MyCmyk = cast::from_array(buffer);
///
/// assert_eq!(
///     color,
///     MyCmyk {
///         cyan: 0.1,
///         magenta: 0.2,
///         yellow: 0.3,
///         key: 0.4,
///     }
/// );
/// ```
///
/// Heterogenous field types:
///
/// ```rust
/// use std::marker::PhantomData;
///
/// use palette::{cast::{self, ArrayCast}, encoding::Srgb, RgbHue};
///
/// #[derive(PartialEq, Debug, ArrayCast)]
/// #[repr(C)]
/// struct MyCoolColor<S> {
///     #[palette(unsafe_zero_sized)]
///     standard: PhantomData<S>,
///     // RgbHue is a wrapper with `#[repr(C)]`, so it can safely
///     // be converted straight from `f32`.
///     #[palette(unsafe_same_layout_as = "f32")]
///     hue: RgbHue<f32>,
///     lumen: f32,
///     chroma: f32,
/// }
///
/// let buffer = [172.0, 100.0, 0.3];
/// let color: MyCoolColor<Srgb> = cast::from_array(buffer);
///
/// assert_eq!(
///     color,
///     MyCoolColor {
///         hue: 172.0.into(),
///         lumen: 100.0,
///         chroma: 0.3,
///         standard: PhantomData,
///     }
/// );
/// ```
///
/// ## Safety
///
/// * The type must be inhabited (eg: no
///   [Infallible](std::convert::Infallible)).
/// * The type must allow any values in the array items (eg: either no
///   requirements or some ability to recover from invalid values).
/// * The type must be homogeneous (eg: all fields have the same type, or are
///   wrappers that implement `ArrayCast` with the same field type, or are zero
///   sized).
/// * The length of `Array` must be the sum of the number of color component
///   fields in the type and in any possible compound fields.
/// * The type must be `repr(C)` or `repr(transparent)`.
/// * The type must have the same size and alignment as `Self::Array`.
///
/// Note also that the type is assumed to not implement `Drop`. This will
/// rarely, if ever, be an issue. The requirements above ensures that the
/// underlying field types stay the same and will be dropped.
///
/// For example:
///
/// * `Srgb<T>` can be cast to `[T; 3]` because it has three non-zero sized
///   fields of type `T`.
/// * `Alpha<Srgb<T>, T>` can be cast to `[T; 4]`, that is `3 + 1` items,
///   because it's the sum of the three items from `Srgb` and the one extra
///   `alpha` field.
/// * `Alpha<Srgb<T>, U>` is not allowed because `T` and `U` are different
///   types.
pub unsafe trait ArrayCast: Sized {
    /// The output type of a cast to an array.
    type Array: ArrayExt;
}

/// Cast from a color type to an array.
///
/// ```
/// use palette::{cast, Srgb};
///
/// let color = Srgb::new(23u8, 198, 76);
/// assert_eq!(cast::into_array(color), [23, 198, 76]);
/// ```
///
/// It's also possible to use `From` and `Into` when casting built-in types:
///
/// ```
/// use palette::Srgb;
///
/// let color = Srgb::new(23u8, 198, 76);
///
/// // Colors implement `Into`:
/// let array1: [_; 3] = color.into();
///
/// // Arrays implement `From`:
/// let array2 = <[_; 3]>::from(color);
/// ```
#[inline]
pub fn into_array<T>(color: T) -> T::Array
where
    T: ArrayCast,
{
    assert_eq!(core::mem::size_of::<T::Array>(), core::mem::size_of::<T>());

    // Safety: The requirements of implementing `ArrayCast`, as well as the size
    // assert, ensures that transmuting `T` into `T::Array` is safe.
    unsafe { transmute_copy(&ManuallyDrop::new(color)) }
}

/// Cast from an array to a color type.
///
/// ```
/// use palette::{cast, Srgb};
///
/// let array = [23, 198, 76];
/// assert_eq!(cast::from_array::<Srgb<u8>>(array),  Srgb::new(23, 198, 76));
/// ```
///
/// It's also possible to use `From` and `Into` when casting built-in types:
///
/// ```
/// use palette::Srgb;
///
/// let array = [23, 198, 76];
///
/// // Arrays implement `Into`:
/// let color1: Srgb<u8> = array.into();
///
/// // Colors implement `From`:
/// let color2 = Srgb::from(array);
/// ```
#[inline]
pub fn from_array<T>(array: T::Array) -> T
where
    T: ArrayCast,
{
    assert_eq!(core::mem::size_of::<T::Array>(), core::mem::size_of::<T>());

    // Safety: The requirements of implementing `ArrayCast`, as well as the size
    // assert, ensures that transmuting `T::Array` into `T` is safe.
    unsafe { transmute_copy(&ManuallyDrop::new(array)) }
}

/// Cast from a color type reference to an array reference.
///
/// ```
/// use palette::{cast, Srgb};
///
/// let color = Srgb::new(23u8, 198, 76);
/// assert_eq!(cast::into_array_ref(&color), &[23, 198, 76]);
/// ```
///
/// It's also possible to use `From`, `Into` and `AsRef` when casting built-in
/// types:
///
/// ```
/// use palette::Srgb;
///
/// let color = Srgb::new(23u8, 198, 76);
///
/// // Colors implement `AsRef`:
/// let array1: &[_; 3] = color.as_ref();
///
/// // Color references implement `Into`:
/// let array2: &[_; 3] = (&color).into();
//
/// // Array references implement `From`:
/// let array3 = <&[_; 3]>::from(&color);
/// ```
#[inline]
pub fn into_array_ref<T>(value: &T) -> &T::Array
where
    T: ArrayCast,
{
    assert_eq!(core::mem::size_of::<T::Array>(), core::mem::size_of::<T>());
    assert_eq!(
        core::mem::align_of::<T::Array>(),
        core::mem::align_of::<T>()
    );

    let value: *const T = value;

    // Safety: The requirements of implementing `ArrayCast`, as well as the size
    // and alignment asserts, ensures that reading `T` as `T::Array` is safe.
    unsafe { &*value.cast::<T::Array>() }
}

/// Cast from an array reference to a color type reference.
///
/// ```
/// use palette::{cast, Srgb};
///
/// let array = [23, 198, 76];
/// assert_eq!(cast::from_array_ref::<Srgb<u8>>(&array),  &Srgb::new(23, 198, 76));
/// ```
///
/// It's also possible to use `From`, `Into` and `AsRef` when casting built-in
/// types:
///
/// ```
/// use palette::Srgb;
///
/// let array = [23, 198, 76];
///
/// // Arrays implement `AsRef`:
/// let color1: &Srgb<u8> = array.as_ref();
///
/// // Array references implement `Into`:
/// let color2: &Srgb<u8> = (&array).into();
///
/// // Color references implement `From`:
/// let color3 = <&Srgb<u8>>::from(&array);
/// ```
#[inline]
pub fn from_array_ref<T>(value: &T::Array) -> &T
where
    T: ArrayCast,
{
    assert_eq!(core::mem::size_of::<T::Array>(), core::mem::size_of::<T>());
    assert_eq!(
        core::mem::align_of::<T::Array>(),
        core::mem::align_of::<T>()
    );

    let value: *const T::Array = value;

    // Safety: The requirements of implementing `ArrayCast`, as well as the size
    // and alignment asserts, ensures that reading `T::Array` as `T` is safe.
    unsafe { &*value.cast::<T>() }
}

/// Cast from a mutable color type reference to a mutable array reference.
///
/// ```
/// use palette::{cast, Srgb};
///
/// let mut color = Srgb::new(23u8, 198, 76);
/// assert_eq!(cast::into_array_mut(&mut color), &mut [23, 198, 76]);
/// ```
///
/// It's also possible to use `From`, `Into` and `AsMut` when casting built-in
/// types:
///
/// ```
/// use palette::Srgb;
///
/// let mut color = Srgb::new(23u8, 198, 76);
///
/// // Colors implement `AsMut`:
/// let array1: &mut [_; 3] = color.as_mut();
///
/// // Color references implement `Into`:
/// let array2: &mut [_; 3] = (&mut color).into();
//
/// // Array references implement `From`:
/// let array3 = <&mut [_; 3]>::from(&mut color);
/// ```
#[inline]
pub fn into_array_mut<T>(value: &mut T) -> &mut T::Array
where
    T: ArrayCast,
{
    assert_eq!(core::mem::size_of::<T::Array>(), core::mem::size_of::<T>());
    assert_eq!(
        core::mem::align_of::<T::Array>(),
        core::mem::align_of::<T>()
    );

    let value: *mut T = value;

    // Safety: The requirements of implementing `ArrayCast`, as well as the size
    // and alignment asserts, ensures that reading `T` as `T::Array` is safe.
    unsafe { &mut *value.cast::<T::Array>() }
}

/// Cast from a mutable array reference to a mutable color type reference.
///
/// ```
/// use palette::{cast, Srgb};
///
/// let mut array = [23, 198, 76];
/// assert_eq!(cast::from_array_mut::<Srgb<u8>>(&mut array),  &mut Srgb::new(23, 198, 76));
/// ```
///
/// It's also possible to use `From`, `Into` and `AsMut` when casting built-in
/// types:
///
/// ```
/// use palette::Srgb;
///
/// let mut array = [23, 198, 76];
///
/// // Arrays implement `AsMut`:
/// let color1: &mut Srgb<u8> = array.as_mut();
///
/// // Array references implement `Into`:
/// let color2: &mut Srgb<u8> = (&mut array).into();
///
/// // Color references implement `From`:
/// let color3 = <&mut Srgb<u8>>::from(&mut array);
/// ```
#[inline]
pub fn from_array_mut<T>(value: &mut T::Array) -> &mut T
where
    T: ArrayCast,
{
    assert_eq!(core::mem::size_of::<T::Array>(), core::mem::size_of::<T>());
    assert_eq!(
        core::mem::align_of::<T::Array>(),
        core::mem::align_of::<T>()
    );

    let value: *mut T::Array = value;

    // Safety: The requirements of implementing `ArrayCast`, as well as the size
    // and alignment asserts, ensures that reading `T::Array` as `T` is safe.
    unsafe { &mut *value.cast::<T>() }
}

/// Cast from an array of colors to an array of arrays.
///
/// ```
/// use palette::{cast, Srgb};
///
/// let colors = [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)];
/// assert_eq!(cast::into_array_array(colors), [[64, 139, 10], [93, 18, 214]])
/// ```
#[inline]
pub fn into_array_array<T, const N: usize>(values: [T; N]) -> [T::Array; N]
where
    T: ArrayCast,
{
    assert_eq!(core::mem::size_of::<T::Array>(), core::mem::size_of::<T>());
    assert_eq!(
        core::mem::align_of::<T::Array>(),
        core::mem::align_of::<T>()
    );

    // Safety: The requirements of implementing `ArrayCast`, as well as the size
    // and alignment asserts, ensures that reading `T` as `T::Array` is safe.
    // The length and memory layout of the array is the same because the size is
    // the same.
    unsafe { transmute_copy(&ManuallyDrop::new(values)) }
}

/// Cast from an array of colors to an array of color components.
///
/// ## Panics
///
/// It's unfortunately not able to infer the length of the component array,
/// until generic const expressions are stabilized. In the meantime, it's going
/// to panic if `M` isn't `N * T::Array::LENGTH`. A future version will remove
/// the `M` parameter and make the mismatch a compiler error.
///
/// No `try_*` alternative is provided, since the array size can't be changed
/// during runtime.
///
/// ## Examples
///
/// ```
/// use palette::{cast, Srgb};
///
/// let colors = [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)];
/// assert_eq!(cast::into_component_array(colors), [64, 139, 10, 93, 18, 214])
/// ```
///
/// It panics when the array lengths don't match up:
///
/// ```should_panic
/// use palette::{cast, Srgb};
///
/// let colors = [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)];
/// assert_eq!(cast::into_component_array(colors), [64, 139, 10]) // Too short
/// ```
#[inline]
pub fn into_component_array<T, const N: usize, const M: usize>(
    values: [T; N],
) -> [<T::Array as ArrayExt>::Item; M]
where
    T: ArrayCast,
{
    assert_eq!(core::mem::size_of::<T::Array>(), core::mem::size_of::<T>());
    assert_eq!(
        core::mem::align_of::<T::Array>(),
        core::mem::align_of::<T>()
    );

    // This check can be replaced with `[<T::Array as ArrayExt>::Item; N *
    // T::Array::LENGTH]` when generic const expressions are stabilized.
    assert_eq!(
        N * T::Array::LENGTH,
        M,
        "expected the output array to have length {}, but its length is {}",
        N * T::Array::LENGTH,
        M
    );
    assert_eq!(
        core::mem::size_of::<[T; N]>(),
        core::mem::size_of::<[<T::Array as ArrayExt>::Item; M]>()
    );
    assert_eq!(
        core::mem::align_of::<[T; N]>(),
        core::mem::align_of::<[<T::Array as ArrayExt>::Item; M]>()
    );

    // Safety: The requirements of implementing `ArrayCast`, as well as the size
    // and alignment asserts, ensures that reading `T` as `T::Array` is safe.
    // The sizes and memory layout of the arrays are also asserted to be the
    // same.
    unsafe { transmute_copy(&ManuallyDrop::new(values)) }
}

/// Cast from an array of arrays to an array of colors.
///
/// ```
/// use palette::{cast, Srgb};
///
/// let arrays = [[64, 139, 10], [93, 18, 214]];
/// assert_eq!(
///     cast::from_array_array::<Srgb<u8>, 2>(arrays),
///     [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)]
/// )
/// ```
#[inline]
pub fn from_array_array<T, const N: usize>(values: [T::Array; N]) -> [T; N]
where
    T: ArrayCast,
{
    assert_eq!(core::mem::size_of::<T::Array>(), core::mem::size_of::<T>());
    assert_eq!(
        core::mem::align_of::<T::Array>(),
        core::mem::align_of::<T>()
    );

    // Safety: The requirements of implementing `ArrayCast`, as well as the size
    // and alignment asserts, ensures that reading `T::Array` as `T` is safe.
    // The length and memory layout of the array is the same because the size is
    // the same.
    unsafe { transmute_copy(&ManuallyDrop::new(values)) }
}

/// Cast from an array of color components to an array of colors.
///
/// ## Panics
///
/// The cast will panic if the length of the input array is not a multiple of
/// the color's array length. This is unfortunately unavoidable until generic
/// const expressions are stabilized.
///
/// No `try_*` alternative is provided, since the array size can't be changed
/// during runtime.
///
/// ## Examples
///
/// ```
/// use palette::{cast, Srgb};
///
/// let components = [64, 139, 10, 93, 18, 214];
/// assert_eq!(
///     cast::from_component_array::<Srgb<u8>, 6, 2>(components),
///     [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)]
/// );
/// ```
///
/// This panics:
///
/// ```should_panic
/// use palette::{cast, Srgb};
///
/// let components = [64, 139, 10, 93, 18]; // Not a multiple of 3
/// assert_eq!(
///     cast::from_component_array::<Srgb<u8>, 5, 2>(components),
///     [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)]
/// );
/// ```
#[inline]
pub fn from_component_array<T, const N: usize, const M: usize>(
    values: [<T::Array as ArrayExt>::Item; N],
) -> [T; M]
where
    T: ArrayCast,
{
    assert_eq!(core::mem::size_of::<T::Array>(), core::mem::size_of::<T>());
    assert_eq!(
        core::mem::align_of::<T::Array>(),
        core::mem::align_of::<T>()
    );

    // These checks can be replaced with `[<T::Array as ArrayExt>::Item; N /
    // T::Array::LENGTH]` and a compile time check for `values.len() %
    // T::Array::LENGTH == 0` when generic const expressions are stabilized.
    assert_eq!(
        N % T::Array::LENGTH,
        0,
        "expected the array length ({}) to be divisible by {}",
        N,
        T::Array::LENGTH
    );
    assert_eq!(
        N / T::Array::LENGTH,
        M,
        "expected the output array to have length {}, but its length is {}",
        N / T::Array::LENGTH,
        M
    );
    assert_eq!(
        core::mem::size_of::<[<T::Array as ArrayExt>::Item; N]>(),
        core::mem::size_of::<[T; M]>()
    );
    assert_eq!(
        core::mem::align_of::<[<T::Array as ArrayExt>::Item; N]>(),
        core::mem::align_of::<[T; M]>()
    );

    // Safety: The requirements of implementing `ArrayCast`, as well as the size
    // and alignment asserts, ensures that reading `T` as `T::Array` is safe.
    // The sizes and memory layout of the arrays are also asserted to be the
    // same.
    unsafe { transmute_copy(&ManuallyDrop::new(values)) }
}

/// Cast from a slice of colors to a slice of arrays.
///
/// ```
/// use palette::{cast, Srgb};
///
/// let colors = &[Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)];
/// assert_eq!(cast::into_array_slice(colors), &[[64, 139, 10], [93, 18, 214]])
/// ```
#[inline]
pub fn into_array_slice<T>(values: &[T]) -> &[T::Array]
where
    T: ArrayCast,
{
    assert_eq!(core::mem::size_of::<T::Array>(), core::mem::size_of::<T>());
    assert_eq!(
        core::mem::align_of::<T::Array>(),
        core::mem::align_of::<T>()
    );

    // Safety: The requirements of implementing `ArrayCast`, as well as the size
    // and alignment asserts, ensures that reading `T` as `T::Array` is safe.
    // The length is the same because the size is the same.
    unsafe { core::slice::from_raw_parts(values.as_ptr().cast::<T::Array>(), values.len()) }
}

/// Cast from a slice of colors to a slice of color components.
///
/// ```
/// use palette::{cast, Srgb};
///
/// let colors = &[Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)];
/// assert_eq!(cast::into_component_slice(colors), &[64, 139, 10, 93, 18, 214])
/// ```
#[inline]
pub fn into_component_slice<T>(values: &[T]) -> &[<T::Array as ArrayExt>::Item]
where
    T: ArrayCast,
{
    assert_eq!(core::mem::size_of::<T::Array>(), core::mem::size_of::<T>());
    assert_eq!(
        core::mem::align_of::<T::Array>(),
        core::mem::align_of::<T>()
    );

    let length = values.len() * T::Array::LENGTH;

    // Safety: The requirements of implementing `ArrayCast`, as well as the size
    // and alignment asserts, ensures that reading `[T]` as `[T::Array::Item]`
    // is safe. The length is multiplied by the array length.
    unsafe {
        core::slice::from_raw_parts(
            values.as_ptr().cast::<<T::Array as ArrayExt>::Item>(),
            length,
        )
    }
}

/// Cast from a slice of arrays to a slice of colors.
///
/// ```
/// use palette::{cast, Srgb};
///
/// let arrays = &[[64, 139, 10], [93, 18, 214]];
/// assert_eq!(
///     cast::from_array_slice::<Srgb<u8>>(arrays),
///     &[Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)]
/// )
/// ```
#[inline]
pub fn from_array_slice<T>(values: &[T::Array]) -> &[T]
where
    T: ArrayCast,
{
    assert_eq!(core::mem::size_of::<T::Array>(), core::mem::size_of::<T>());
    assert_eq!(
        core::mem::align_of::<T::Array>(),
        core::mem::align_of::<T>()
    );

    // Safety: The requirements of implementing `ArrayCast`, as well as the size
    // and alignment asserts, ensures that reading `T::Array` as `T` is safe.
    // The length is the same because the size is the same.
    unsafe { core::slice::from_raw_parts(values.as_ptr().cast::<T>(), values.len()) }
}

/// The same as [`try_from_component_slice`] but panics on error.
///
/// ## Panics
///
/// The cast will panic if the length of the input slice is not a multiple of
/// the color's array length.
///
/// ## Examples
///
/// ```
/// use palette::{cast, Srgb};
///
/// let components = &[64, 139, 10, 93, 18, 214];
/// assert_eq!(
///     cast::from_component_slice::<Srgb<u8>>(components),
///     &[Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)]
/// )
/// ```
///
/// This panics:
///
/// ```should_panic
/// use palette::{cast, Srgb};
///
/// let components = &[64, 139, 10, 93, 18, 214, 0, 123]; // Not a multiple of 3
/// cast::from_component_slice::<Srgb<u8>>(components);
/// ```
#[inline]
pub fn from_component_slice<T>(values: &[<T::Array as ArrayExt>::Item]) -> &[T]
where
    T: ArrayCast,
{
    try_from_component_slice(values).unwrap()
}

/// Cast from a slice of color components to a slice of colors.
///
/// ## Errors
///
/// The cast will return an error if the length of the input slice is not a
/// multiple of the color's array length.
///
/// ## Examples
///
/// ```
/// use palette::{cast, Srgb};
///
/// let components = &[64, 139, 10, 93, 18, 214];
/// assert_eq!(
///     cast::try_from_component_slice::<Srgb<u8>>(components),
///     Ok([Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)].as_ref())
/// )
/// ```
///
/// This produces an error:
///
/// ```
/// use palette::{cast, Srgb};
///
/// let components = &[64, 139, 10, 93, 18]; // Not a multiple of 3
/// assert!(cast::try_from_component_slice::<Srgb<u8>>(components).is_err());
/// ```
#[inline]
pub fn try_from_component_slice<T>(
    values: &[<T::Array as ArrayExt>::Item],
) -> Result<&[T], SliceCastError>
where
    T: ArrayCast,
{
    assert_eq!(core::mem::size_of::<T::Array>(), core::mem::size_of::<T>());
    assert_eq!(
        core::mem::align_of::<T::Array>(),
        core::mem::align_of::<T>()
    );

    if values.len() % T::Array::LENGTH != 0 {
        return Err(SliceCastError);
    }

    let length = values.len() / T::Array::LENGTH;
    let raw = values.as_ptr().cast::<T>();

    // Safety: The requirements of implementing `ArrayCast`, as well as the size
    // and alignment asserts, ensures that reading `T` as `T::Array` is safe.
    unsafe { Ok(core::slice::from_raw_parts(raw, length)) }
}

/// Cast from a mutable slice of colors to a mutable slice of arrays.
///
/// ```
/// use palette::{cast, Srgb};
///
/// let colors = &mut [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)];
/// assert_eq!(
///     cast::into_array_slice_mut(colors),
///     &mut [[64, 139, 10], [93, 18, 214]]
/// )
/// ```
#[inline]
pub fn into_array_slice_mut<T>(values: &mut [T]) -> &mut [T::Array]
where
    T: ArrayCast,
{
    assert_eq!(core::mem::size_of::<T::Array>(), core::mem::size_of::<T>());
    assert_eq!(
        core::mem::align_of::<T::Array>(),
        core::mem::align_of::<T>()
    );

    // Safety: The requirements of implementing `ArrayCast`, as well as the size
    // and alignment asserts, ensures that reading `T` as `T::Array` is safe.
    // The length is the same because the size is the same.
    unsafe { core::slice::from_raw_parts_mut(values.as_mut_ptr().cast::<T::Array>(), values.len()) }
}

/// Cast from a slice of colors to a slice of color components.
///
/// ```
/// use palette::{cast, Srgb};
///
/// let colors = &mut [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)];
/// assert_eq!(
///     cast::into_component_slice_mut(colors),
///     &mut [64, 139, 10, 93, 18, 214]
/// )
/// ```
#[inline]
pub fn into_component_slice_mut<T>(values: &mut [T]) -> &mut [<T::Array as ArrayExt>::Item]
where
    T: ArrayCast,
{
    assert_eq!(core::mem::size_of::<T::Array>(), core::mem::size_of::<T>());
    assert_eq!(
        core::mem::align_of::<T::Array>(),
        core::mem::align_of::<T>()
    );

    let length = values.len() * T::Array::LENGTH;

    // Safety: The requirements of implementing `ArrayCast`, as well as the size
    // and alignment asserts, ensures that reading `[T]` as `[T::Array::Item]`
    // is safe. The length is multiplied by the array length.
    unsafe {
        core::slice::from_raw_parts_mut(
            values.as_mut_ptr().cast::<<T::Array as ArrayExt>::Item>(),
            length,
        )
    }
}

/// Cast from a mutable slice of arrays to a mutable slice of colors.
///
/// ```
/// use palette::{cast, Srgb};
///
/// let arrays = &mut [[64, 139, 10], [93, 18, 214]];
/// assert_eq!(
///     cast::from_array_slice_mut::<Srgb<u8>>(arrays),
///     &mut [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)]
/// )
/// ```
#[inline]
pub fn from_array_slice_mut<T>(values: &mut [T::Array]) -> &mut [T]
where
    T: ArrayCast,
{
    assert_eq!(core::mem::size_of::<T::Array>(), core::mem::size_of::<T>());
    assert_eq!(
        core::mem::align_of::<T::Array>(),
        core::mem::align_of::<T>()
    );

    // Safety: The requirements of implementing `ArrayCast`, as well as the size
    // and alignment asserts, ensures that reading `T::Array` as `T` is safe.
    // The length is the same because the size is the same.
    unsafe { core::slice::from_raw_parts_mut(values.as_mut_ptr().cast::<T>(), values.len()) }
}

/// The same as [`try_from_component_slice_mut`] but panics on error.
///
/// ## Panics
///
/// The cast will panic if the length of the input slice is not a multiple of
/// the color's array length.
///
/// ## Examples
///
/// ```
/// use palette::{cast, Srgb};
///
/// let components = &mut [64, 139, 10, 93, 18, 214];
/// assert_eq!(
///     cast::from_component_slice_mut::<Srgb<u8>>(components),
///     &mut [Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)]
/// )
/// ```
///
/// This panics:
///
/// ```should_panic
/// use palette::{cast, Srgb};
///
/// let components = &mut [64, 139, 10, 93, 18, 214, 0, 123]; // Not a multiple of 3
/// cast::from_component_slice_mut::<Srgb<u8>>(components);
/// ```
#[inline]
pub fn from_component_slice_mut<T>(values: &mut [<T::Array as ArrayExt>::Item]) -> &mut [T]
where
    T: ArrayCast,
{
    try_from_component_slice_mut(values).unwrap()
}

/// Cast from a mutable slice of color components to a slice of colors.
///
/// ## Errors
///
/// The cast will return an error if the length of the input slice is not a
/// multiple of the color's array length.
///
/// ## Examples
///
/// ```
/// use palette::{cast, Srgb};
///
/// let components = &mut [64, 139, 10, 93, 18, 214];
/// assert_eq!(
///     cast::try_from_component_slice_mut::<Srgb<u8>>(components),
///     Ok([Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)].as_mut())
/// )
/// ```
///
/// This produces an error:
///
/// ```
/// use palette::{cast, Srgb};
///
/// let components = &mut [64, 139, 10, 93, 18]; // Not a multiple of 3
/// assert!(cast::try_from_component_slice_mut::<Srgb<u8>>(components).is_err());
/// ```
#[inline]
pub fn try_from_component_slice_mut<T>(
    values: &mut [<T::Array as ArrayExt>::Item],
) -> Result<&mut [T], SliceCastError>
where
    T: ArrayCast,
{
    assert_eq!(core::mem::size_of::<T::Array>(), core::mem::size_of::<T>());
    assert_eq!(
        core::mem::align_of::<T::Array>(),
        core::mem::align_of::<T>()
    );

    if values.len() % T::Array::LENGTH != 0 {
        return Err(SliceCastError);
    }

    let length = values.len() / T::Array::LENGTH;
    let raw = values.as_mut_ptr().cast::<T>();

    // Safety: The requirements of implementing `ArrayCast`, as well as the size
    // and alignment asserts, ensures that reading `T` as `T::Array` is safe.
    unsafe { Ok(core::slice::from_raw_parts_mut(raw, length)) }
}

/// Cast from a boxed color type to a boxed array.
///
/// ```
/// use palette::{cast, Srgb};
///
/// let color = Box::new(Srgb::new(23u8, 198, 76));
/// assert_eq!(cast::into_array_box(color), Box::new([23, 198, 76]));
/// ```
///
/// It's also possible to use `From` and `Into` when casting built-in types:
///
/// ```
/// use palette::Srgb;
///
/// // Boxed colors implement `Into`:
/// let color1 = Box::new(Srgb::new(23u8, 198, 76));
/// let array1: Box<[_; 3]> = color1.into();
///
/// // Boxed arrays implement `From`:
/// let color2 = Box::new(Srgb::new(23u8, 198, 76));
/// let array2 = <Box<[_; 3]>>::from(color2);
/// ```
#[cfg(feature = "alloc")]
#[inline]
pub fn into_array_box<T>(value: Box<T>) -> Box<T::Array>
where
    T: ArrayCast,
{
    assert_eq!(core::mem::size_of::<T::Array>(), core::mem::size_of::<T>());
    assert_eq!(
        core::mem::align_of::<T::Array>(),
        core::mem::align_of::<T>()
    );

    let raw = Box::into_raw(value);

    // Safety: The requirements of implementing `ArrayCast`, as well as the size
    // and alignment asserts, ensures that reading `T` as `T::Array` is safe.
    unsafe { Box::from_raw(raw.cast::<T::Array>()) }
}

/// Cast from a boxed array to a boxed color type.
///
/// ```
/// use palette::{cast, Srgb};
///
/// let array = Box::new([23, 198, 76]);
/// assert_eq!(cast::from_array_box::<Srgb<u8>>(array),  Box::new(Srgb::new(23, 198, 76)));
/// ```
///
/// It's also possible to use `From` and `Into` when casting built-in types:
///
/// ```
/// use palette::Srgb;
///
///
/// // Boxed arrays implement `Into`:
/// let array1 = Box::new([23, 198, 76]);
/// let color1: Box<Srgb<u8>> = array1.into();
///
/// // Boxed colors implement `From`:
/// let array2 = Box::new([23, 198, 76]);
/// let color2 = <Box<Srgb<u8>>>::from(array2);
/// ```
#[cfg(feature = "alloc")]
#[inline]
pub fn from_array_box<T>(value: Box<T::Array>) -> Box<T>
where
    T: ArrayCast,
{
    assert_eq!(core::mem::size_of::<T::Array>(), core::mem::size_of::<T>());
    assert_eq!(
        core::mem::align_of::<T::Array>(),
        core::mem::align_of::<T>()
    );

    let raw = Box::into_raw(value);

    // Safety: The requirements of implementing `ArrayCast`, as well as the size
    // and alignment asserts, ensures that reading `T::Array` as `T` is safe.
    unsafe { Box::from_raw(raw.cast::<T>()) }
}

/// Cast from a boxed slice of colors to a boxed slice of arrays.
///
/// ```
/// use palette::{cast, Srgb};
///
/// let colors = vec![Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)].into_boxed_slice();
/// assert_eq!(
///     cast::into_array_slice_box(colors),
///     vec![[64, 139, 10], [93, 18, 214]].into_boxed_slice()
/// )
/// ```
#[cfg(feature = "alloc")]
#[inline]
pub fn into_array_slice_box<T>(values: Box<[T]>) -> Box<[T::Array]>
where
    T: ArrayCast,
{
    let raw: *mut [T::Array] = into_array_slice_mut(Box::leak(values));

    // Safety: We know the pointer comes from a `Box` and thus has the correct lifetime.
    unsafe { Box::from_raw(raw) }
}

/// Cast from a boxed slice of colors to a boxed slice of color components.
///
/// ```
/// use palette::{cast, Srgb};
///
/// let colors = vec![Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)].into_boxed_slice();
/// assert_eq!(
///     cast::into_component_slice_box(colors),
///     vec![64, 139, 10, 93, 18, 214].into_boxed_slice()
/// )
/// ```
#[cfg(feature = "alloc")]
#[inline]
pub fn into_component_slice_box<T>(values: Box<[T]>) -> Box<[<T::Array as ArrayExt>::Item]>
where
    T: ArrayCast,
{
    let raw: *mut [<T::Array as ArrayExt>::Item] = into_component_slice_mut(Box::leak(values));

    // Safety: We know the pointer comes from a `Box` and thus has the correct lifetime.
    unsafe { Box::from_raw(raw) }
}

/// Cast from a boxed slice of arrays to a boxed slice of colors.
///
/// ```
/// use palette::{cast, Srgb};
///
/// let arrays = vec![[64, 139, 10], [93, 18, 214]].into_boxed_slice();
/// assert_eq!(
///     cast::from_array_slice_box::<Srgb<u8>>(arrays),
///     vec![Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)].into_boxed_slice()
/// )
/// ```
#[cfg(feature = "alloc")]
#[inline]
pub fn from_array_slice_box<T>(values: Box<[T::Array]>) -> Box<[T]>
where
    T: ArrayCast,
{
    let raw: *mut [T] = from_array_slice_mut(Box::leak(values));

    // Safety: We know the pointer comes from a `Box` and thus has the correct lifetime.
    unsafe { Box::from_raw(raw) }
}

/// The same as [`try_from_component_slice_box`] but panics on error.
///
/// ## Panics
///
/// The cast will panic if the length of the input slice is not a multiple of
/// the color's array length.
///
/// ## Examples
///
/// ```
/// use palette::{cast, Srgb};
///
/// let components = vec![64, 139, 10, 93, 18, 214].into_boxed_slice();
/// assert_eq!(
///     cast::from_component_slice_box::<Srgb<u8>>(components),
///     vec![Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)].into_boxed_slice()
/// )
/// ```
///
/// This panics:
///
/// ```should_panic
/// use palette::{cast, Srgb};
///
/// // Not a multiple of 3:
/// let components = vec![64, 139, 10, 93, 18, 214, 0, 123].into_boxed_slice();
/// cast::from_component_slice_box::<Srgb<u8>>(components);
/// ```
#[cfg(feature = "alloc")]
#[inline]
pub fn from_component_slice_box<T>(values: Box<[<T::Array as ArrayExt>::Item]>) -> Box<[T]>
where
    T: ArrayCast,
{
    try_from_component_slice_box(values).unwrap()
}

/// Cast from a boxed slice of color components to a boxed slice of colors.
///
/// ## Errors
///
/// The cast will return an error if the length of the input slice is not a
/// multiple of the color's array length.
///
/// ## Examples
///
/// ```
/// use palette::{cast, Srgb};
///
/// let components = vec![64, 139, 10, 93, 18, 214].into_boxed_slice();
/// assert_eq!(
///     cast::try_from_component_slice_box::<Srgb<u8>>(components),
///     Ok(vec![Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)].into_boxed_slice())
/// )
/// ```
///
/// This produces an error:
///
/// ```
/// use palette::{cast, Srgb};
///
/// // Not a multiple of 3:
/// let components = vec![64, 139, 10, 93, 18].into_boxed_slice();
///
/// if let Err(error) = cast::try_from_component_slice_box::<Srgb<u8>>(components) {
///     // We get the original values back on error:
///     assert_eq!(
///         error.values,
///         vec![64, 139, 10, 93, 18].into_boxed_slice()
///     );
/// } else {
///     unreachable!();
/// }
/// ```
#[cfg(feature = "alloc")]
#[inline]
pub fn try_from_component_slice_box<T>(
    values: Box<[<T::Array as ArrayExt>::Item]>,
) -> Result<Box<[T]>, BoxedSliceCastError<<T::Array as ArrayExt>::Item>>
where
    T: ArrayCast,
{
    if values.len() % T::Array::LENGTH != 0 {
        return Err(BoxedSliceCastError { values });
    }

    let raw: *mut [T] = from_component_slice_mut(Box::leak(values));

    // Safety: We know the pointer comes from a `Box` and thus has the correct lifetime.
    unsafe { Ok(Box::from_raw(raw)) }
}

/// Cast from a `Vec` of colors to a `Vec` of arrays.
///
/// ```
/// use palette::{cast, Srgb};
///
/// let colors = vec![Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)];
/// assert_eq!(
///     cast::into_array_vec(colors),
///     vec![[64, 139, 10], [93, 18, 214]]
/// )
/// ```
#[cfg(feature = "alloc")]
#[inline]
pub fn into_array_vec<T>(values: Vec<T>) -> Vec<T::Array>
where
    T: ArrayCast,
{
    assert_eq!(core::mem::size_of::<T::Array>(), core::mem::size_of::<T>());
    assert_eq!(
        core::mem::align_of::<T::Array>(),
        core::mem::align_of::<T>()
    );
    let mut values = ManuallyDrop::new(values);

    let raw = values.as_mut_ptr();

    // Safety: The requirements of implementing `ArrayCast`, as well as the size
    // and alignment asserts, ensures that reading `T` as `T::Array` is safe.
    // Length and capacity are the same because the size is the same.
    unsafe { Vec::from_raw_parts(raw.cast::<T::Array>(), values.len(), values.capacity()) }
}

/// Cast from a `Vec` of colors to a `Vec` of color components.
///
/// ```
/// use palette::{cast, Srgb};
///
/// let colors = vec![Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)];
/// assert_eq!(
///     cast::into_component_vec(colors),
///     vec![64, 139, 10, 93, 18, 214]
/// )
/// ```
#[cfg(feature = "alloc")]
#[inline]
pub fn into_component_vec<T>(values: Vec<T>) -> Vec<<T::Array as ArrayExt>::Item>
where
    T: ArrayCast,
{
    assert_eq!(core::mem::size_of::<T::Array>(), core::mem::size_of::<T>());
    assert_eq!(
        core::mem::align_of::<T::Array>(),
        core::mem::align_of::<T>()
    );
    let mut values = ManuallyDrop::new(values);

    let raw = values.as_mut_ptr();
    let length = values.len() * T::Array::LENGTH;
    let capacity = values.capacity() * T::Array::LENGTH;

    // Safety: The requirements of implementing `ArrayCast`, as well as the size
    // and alignment asserts, ensures that reading `T` as `T::Array` is safe.
    // The length and capacity are multiplied by the array length.
    unsafe { Vec::from_raw_parts(raw.cast::<<T::Array as ArrayExt>::Item>(), length, capacity) }
}

/// Cast from a `Vec` of arrays to a `Vec` of colors.
///
/// ```
/// use palette::{cast, Srgb};
///
/// let arrays = vec![[64, 139, 10], [93, 18, 214]];
/// assert_eq!(
///     cast::from_array_vec::<Srgb<u8>>(arrays),
///     vec![Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)]
/// )
/// ```
#[cfg(feature = "alloc")]
#[inline]
pub fn from_array_vec<T>(values: Vec<T::Array>) -> Vec<T>
where
    T: ArrayCast,
{
    assert_eq!(core::mem::size_of::<T::Array>(), core::mem::size_of::<T>());
    assert_eq!(
        core::mem::align_of::<T::Array>(),
        core::mem::align_of::<T>()
    );
    let mut values = ManuallyDrop::new(values);

    let raw = values.as_mut_ptr();

    // Safety: The requirements of implementing `ArrayCast`, as well as the size
    // and alignment asserts, ensures that reading `T::Array` as `T` is safe.
    // Length and capacity are the same because the size is the same.
    unsafe { Vec::from_raw_parts(raw.cast::<T>(), values.len(), values.capacity()) }
}

/// The same as [`try_from_component_vec`] but panics on error.
///
/// ## Panics
///
/// The cast will panic if the length or capacity of the input `Vec` is not a
/// multiple of the color's array length.
///
/// ## Examples
///
/// ```
/// use palette::{cast, Srgb};
///
/// let components = vec![64, 139, 10, 93, 18, 214];
/// assert_eq!(
///     cast::from_component_vec::<Srgb<u8>>(components),
///     vec![Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)]
/// )
/// ```
///
/// This panics due to the incorrect length:
///
/// ```should_panic
/// use palette::{cast, Srgb};
///
/// // Not a multiple of 3:
/// let components = vec![64, 139, 10, 93, 18, 214, 0, 123];
/// cast::from_component_vec::<Srgb<u8>>(components);
/// ```
///
/// This panics due to the incorrect capacity:
///
/// ```should_panic
/// use palette::{cast, Srgb};
///
/// let mut components = vec![64, 139, 10, 93, 18, 214];
/// components.reserve_exact(2); // Not a multiple of 3
/// cast::from_component_vec::<Srgb<u8>>(components);
/// ```
#[cfg(feature = "alloc")]
#[inline]
pub fn from_component_vec<T>(values: Vec<<T::Array as ArrayExt>::Item>) -> Vec<T>
where
    T: ArrayCast,
{
    try_from_component_vec(values).unwrap()
}

/// Cast from a `Vec` of color components to a `Vec` of colors.
///
/// ## Errors
///
/// The cast will return an error if the length or capacity of the input `Vec`
/// is not a multiple of the color's array length.
///
/// ## Examples
///
/// ```
/// use palette::{cast, Srgb};
///
/// let components = vec![64, 139, 10, 93, 18, 214];
/// assert_eq!(
///     cast::try_from_component_vec::<Srgb<u8>>(components),
///     Ok(vec![Srgb::new(64u8, 139, 10), Srgb::new(93, 18, 214)])
/// )
/// ```
///
/// This produces an error due to the incorrect length:
///
/// ```
/// use palette::{cast, Srgb};
///
/// // Not a multiple of 3:
/// let components = vec![64, 139, 10, 93, 18];
///
/// if let Err(error) = cast::try_from_component_vec::<Srgb<u8>>(components) {
///     // We get the original values back on error:
///     assert_eq!(error.values, vec![64, 139, 10, 93, 18]);
/// } else {
///     unreachable!();
/// }
/// ```
///
/// This produces an error due to the incorrect capacity:
///
/// ```
/// use palette::{cast, Srgb};
///
/// let mut components = vec![64, 139, 10, 93, 18, 214];
/// components.reserve_exact(2); // Not a multiple of 3
///
/// if let Err(error) = cast::try_from_component_vec::<Srgb<u8>>(components) {
///     // We get the original values back on error:
///     assert_eq!(error.values, vec![64, 139, 10, 93, 18, 214]);
///     assert_eq!(error.values.capacity(), 8);
/// } else {
///     unreachable!();
/// }
/// ```
#[cfg(feature = "alloc")]
#[inline]
pub fn try_from_component_vec<T>(
    values: Vec<<T::Array as ArrayExt>::Item>,
) -> Result<Vec<T>, VecCastError<<T::Array as ArrayExt>::Item>>
where
    T: ArrayCast,
{
    assert_eq!(core::mem::size_of::<T::Array>(), core::mem::size_of::<T>());
    assert_eq!(
        core::mem::align_of::<T::Array>(),
        core::mem::align_of::<T>()
    );

    if values.len() % T::Array::LENGTH != 0 {
        return Err(VecCastError {
            values,
            kind: VecCastErrorKind::LengthMismatch,
        });
    }

    if values.capacity() % T::Array::LENGTH != 0 {
        return Err(VecCastError {
            values,
            kind: VecCastErrorKind::CapacityMismatch,
        });
    }

    let mut values = ManuallyDrop::new(values);

    let raw = values.as_mut_ptr();
    let length = values.len() / T::Array::LENGTH;
    let capacity = values.capacity() / T::Array::LENGTH;

    // Safety: The requirements of implementing `ArrayCast`, as well as the size
    // and alignment asserts, ensures that reading `T` as `T::Array` is safe.
    // The length and capacity are multiplies of the array length.
    unsafe { Ok(Vec::from_raw_parts(raw.cast::<T>(), length, capacity)) }
}

/// Map values of color A to values of color B without creating a new `Vec`.
///
/// This uses the guarantees of [`ArrayCast`] to reuse the allocation.
#[cfg(feature = "alloc")]
#[inline]
pub fn map_vec_in_place<A, B, F>(values: Vec<A>, mut map: F) -> Vec<B>
where
    A: ArrayCast,
    B: ArrayCast<Array = A::Array>,
    F: FnMut(A) -> B,
{
    // We are checking `B` in advance, to stop the program before any work is
    // done. `A` is checked when converting to arrays.
    assert_eq!(core::mem::size_of::<B::Array>(), core::mem::size_of::<B>());
    assert_eq!(
        core::mem::align_of::<B::Array>(),
        core::mem::align_of::<B>()
    );

    let mut values = ManuallyDrop::new(into_array_vec(values));

    for item in &mut *values {
        // Safety: We will put a new value back below, and `values` will not be dropped on panic.
        let input = unsafe { core::ptr::read(item) };

        let output = into_array::<B>(map(from_array::<A>(input)));

        // Safety: `output` is derived from the original value, so this is putting it back into place.
        unsafe { core::ptr::write(item, output) };
    }

    from_array_vec(ManuallyDrop::into_inner(values))
}

/// Map values of color A to values of color B without creating a new `Box<[B]>`.
///
/// This uses the guarantees of [`ArrayCast`] to reuse the allocation.
#[cfg(feature = "alloc")]
#[inline]
pub fn map_slice_box_in_place<A, B, F>(values: Box<[A]>, mut map: F) -> Box<[B]>
where
    A: ArrayCast,
    B: ArrayCast<Array = A::Array>,
    F: FnMut(A) -> B,
{
    // We are checking `B` in advance, to stop the program before any work is
    // done. `A` is checked when converting to arrays.
    assert_eq!(core::mem::size_of::<B::Array>(), core::mem::size_of::<B>());
    assert_eq!(
        core::mem::align_of::<B::Array>(),
        core::mem::align_of::<B>()
    );

    let mut values = ManuallyDrop::new(into_array_slice_box(values));

    for item in &mut **values {
        // Safety: We will put a new value back below, and `values` will not be dropped on panic.
        let input = unsafe { core::ptr::read(item) };

        let output = into_array::<B>(map(from_array::<A>(input)));

        // Safety: `output` is derived from the original value, so this is putting it back into place.
        unsafe { core::ptr::write(item, output) };
    }

    from_array_slice_box(ManuallyDrop::into_inner(values))
}

/// The error type returned when casting a slice of components fails.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SliceCastError;

impl core::fmt::Display for SliceCastError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("could not convert component slice to colors")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for SliceCastError {}

/// The error type returned when casting a boxed slice of components fails.
#[derive(Clone, PartialEq, Eq)]
#[cfg(feature = "alloc")]
pub struct BoxedSliceCastError<T> {
    /// The original values.
    pub values: Box<[T]>,
}

#[cfg(feature = "alloc")]
impl<T> core::fmt::Debug for BoxedSliceCastError<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("BoxedSliceCastError").finish()
    }
}

#[cfg(feature = "alloc")]
impl<T> core::fmt::Display for BoxedSliceCastError<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("could not convert boxed component slice to colors")
    }
}

#[cfg(feature = "std")]
impl<T> std::error::Error for BoxedSliceCastError<T> {}

/// The error type returned when casting a `Vec` of components fails.
#[derive(Clone, PartialEq, Eq)]
#[cfg(feature = "alloc")]
pub struct VecCastError<T> {
    /// The type of error that occurred.
    pub kind: VecCastErrorKind,

    /// The original values.
    pub values: Vec<T>,
}

#[cfg(feature = "alloc")]
impl<T> core::fmt::Debug for VecCastError<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("VecCastError")
            .field("kind", &self.kind)
            .finish()
    }
}

#[cfg(feature = "alloc")]
impl<T> core::fmt::Display for VecCastError<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("could not convert component vector to colors")
    }
}

#[cfg(feature = "std")]
impl<T> std::error::Error for VecCastError<T> {}

/// The type of error that is returned when casting a `Vec` of components.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg(feature = "alloc")]
pub enum VecCastErrorKind {
    /// The type of error returned when the length of a `Vec` didn't match the
    /// requirements.
    LengthMismatch,

    /// The type of error returned when the capacity of a `Vec` didn't match the
    /// requirements.
    CapacityMismatch,
}

#[cfg(test)]
mod test {
    #[cfg(feature = "alloc")]
    use crate::{LinSrgb, Srgb};

    #[cfg(feature = "alloc")]
    #[test]
    fn array_vec_len_cap() {
        let mut original = vec![
            Srgb::new(255u8, 0, 0),
            Srgb::new(0, 255, 0),
            Srgb::new(0, 0, 255),
        ];
        original.reserve_exact(5); // Bringing it to 8

        let colors_arrays = super::into_array_vec(original);
        assert_eq!(colors_arrays.len(), 3);
        assert_eq!(colors_arrays.capacity(), 8);

        let colors = super::from_array_vec::<Srgb<_>>(colors_arrays);
        assert_eq!(colors.len(), 3);
        assert_eq!(colors.capacity(), 8);

        let colors_components = super::into_component_vec(colors);
        assert_eq!(colors_components.len(), 9);
        assert_eq!(colors_components.capacity(), 24);

        let colors = super::from_component_vec::<Srgb<_>>(colors_components);
        assert_eq!(colors.len(), 3);
        assert_eq!(colors.capacity(), 8);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn map_vec_in_place() {
        fn do_things(rgb: Srgb) -> LinSrgb {
            let mut linear = rgb.into_linear();
            core::mem::swap(&mut linear.red, &mut linear.blue);
            linear
        }

        let values = vec![Srgb::new(0.8, 1.0, 0.2), Srgb::new(0.9, 0.1, 0.3)];
        let result = super::map_vec_in_place(values, do_things);
        assert_eq!(
            result,
            vec![
                do_things(Srgb::new(0.8, 1.0, 0.2)),
                do_things(Srgb::new(0.9, 0.1, 0.3))
            ]
        )
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn map_slice_box_in_place() {
        fn do_things(rgb: Srgb) -> LinSrgb {
            let mut linear = rgb.into_linear();
            core::mem::swap(&mut linear.red, &mut linear.blue);
            linear
        }

        let values = vec![Srgb::new(0.8, 1.0, 0.2), Srgb::new(0.9, 0.1, 0.3)].into_boxed_slice();
        let result = super::map_slice_box_in_place(values, do_things);
        assert_eq!(
            result,
            vec![
                do_things(Srgb::new(0.8, 1.0, 0.2)),
                do_things(Srgb::new(0.9, 0.1, 0.3))
            ]
            .into_boxed_slice()
        )
    }
}
