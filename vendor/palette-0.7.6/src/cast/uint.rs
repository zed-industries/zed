use core::mem::{transmute_copy, ManuallyDrop};

/// Marker trait for types that can be represented as an unsigned integer.
///
/// A type that implements this trait is assumed to have the exact same memory
/// layout and representation as an unsigned integer, with the current compile
/// target's endianness. This implies a couple of useful properties:
///
/// * Casting between `T` and `T::Uint` is free and will (or should) be
///   optimized away.
/// * `[T]` can be cast to and from `[T::Uint]`.
///
/// This allows a number of common and useful optimizations, including casting
/// buffers and reusing memory. It does however come with some strict
/// requirements.
///
/// ## Safety
///
/// * The type must be inhabited (eg: no
///   [Infallible](std::convert::Infallible)).
/// * The type must allow any bit pattern (eg: either no requirements or some
///   ability to recover from invalid values).
/// * The type must be either a wrapper around `Self::Uint` or be safe to transmute to and from `Self::Uint`.
/// * The type must not contain any internal padding.
/// * The type must be `repr(C)` or `repr(transparent)`.
/// * The type must have the same size and alignment as `Self::Uint`.
///
/// Note also that the type is assumed to not implement `Drop`. This will
/// rarely, if ever, be an issue. The requirements above ensures that the
/// underlying field types stay the same and will be dropped.
pub unsafe trait UintCast {
    /// An unsigned integer with the same size as `Self`.
    type Uint;
}

/// Cast from a color type to an unsigned integer.
///
/// ```
/// use palette::{cast, rgb::PackedArgb, Srgba};
///
/// let color: PackedArgb = Srgba::new(0x17, 0xC6, 0x4C, 0xFF).into();
/// assert_eq!(cast::into_uint(color), 0xFF17C64C);
/// ```
///
/// It's also possible to use `From` and `Into` when casting built-in types:
///
/// ```
/// use palette::Srgba;
///
/// let color = Srgba::new(23u8, 198, 76, 255);
///
/// // Integers implement `Into`:
/// let uint1: u32 = color.into();
///
/// // Integers implement `From`:
/// let uint2 = u32::from(color);
/// ```
#[inline]
pub fn into_uint<T>(color: T) -> T::Uint
where
    T: UintCast,
{
    assert_eq!(core::mem::size_of::<T::Uint>(), core::mem::size_of::<T>());

    // Safety: The requirements of implementing `UintCast`, as well as the size
    // assert, ensures that transmuting `T` into `T::Uint` is safe.
    unsafe { transmute_copy(&ManuallyDrop::new(color)) }
}

/// Cast from an unsigned integer to a color type.
///
/// ```
/// use palette::{cast, rgb::PackedArgb, Srgba};
///
/// let color: PackedArgb = Srgba::new(0x17, 0xC6, 0x4C, 0xFF).into();
/// assert_eq!(cast::from_uint::<PackedArgb>(0xFF17C64C), color);
/// ```
///
/// It's also possible to use `From` and `Into` when casting built-in types:
///
/// ```
/// use palette::Srgba;
///
/// let uint = 0xFF17C64C;
///
/// // Integers implement `Into`:
/// let color1: Srgba<u8> = uint.into();
///
/// // Colors implement `From`:
/// let color2 = Srgba::from(uint);
/// ```
#[inline]
pub fn from_uint<T>(uint: T::Uint) -> T
where
    T: UintCast,
{
    assert_eq!(core::mem::size_of::<T::Uint>(), core::mem::size_of::<T>());

    // Safety: The requirements of implementing `UintCast`, as well as the size
    // assert, ensures that transmuting `T::Uint` into `T` is safe.
    unsafe { transmute_copy(&ManuallyDrop::new(uint)) }
}

/// Cast from a color type reference to an unsigned integer reference.
///
/// ```
/// use palette::{cast, rgb::PackedArgb, Srgba};
///
/// let color: PackedArgb = Srgba::new(0x17, 0xC6, 0x4C, 0xFF).into();
/// assert_eq!(cast::into_uint_ref(&color), &0xFF17C64C);
/// ```
#[inline]
pub fn into_uint_ref<T>(value: &T) -> &T::Uint
where
    T: UintCast,
{
    assert_eq!(core::mem::size_of::<T::Uint>(), core::mem::size_of::<T>());
    assert_eq!(core::mem::align_of::<T::Uint>(), core::mem::align_of::<T>());

    let value: *const T = value;

    // Safety: The requirements of implementing `UintCast`, as well as the size
    // and alignment asserts, ensures that reading `T` as `T::Uint` is safe.
    unsafe { &*value.cast::<T::Uint>() }
}

/// Cast from an unsigned integer reference to a color type reference.
///
/// ```
/// use palette::{cast, rgb::PackedArgb, Srgba};
///
/// let color: PackedArgb = Srgba::new(0x17, 0xC6, 0x4C, 0xFF).into();
/// assert_eq!(cast::from_uint_ref::<PackedArgb>(&0xFF17C64C), &color);
/// ```
#[inline]
pub fn from_uint_ref<T>(value: &T::Uint) -> &T
where
    T: UintCast,
{
    assert_eq!(core::mem::size_of::<T::Uint>(), core::mem::size_of::<T>());
    assert_eq!(core::mem::align_of::<T::Uint>(), core::mem::align_of::<T>());

    let value: *const T::Uint = value;

    // Safety: The requirements of implementing `UintCast`, as well as the size
    // and alignment asserts, ensures that reading `T::Uint` as `T` is safe.
    unsafe { &*value.cast::<T>() }
}

/// Cast from a mutable color type reference to a mutable unsigned integer reference.
///
/// ```
/// use palette::{cast, rgb::PackedArgb, Srgba};
///
/// let mut color: PackedArgb = Srgba::new(0x17, 0xC6, 0x4C, 0xFF).into();
/// assert_eq!(cast::into_uint_mut(&mut color), &mut 0xFF17C64C);
/// ```
#[inline]
pub fn into_uint_mut<T>(value: &mut T) -> &mut T::Uint
where
    T: UintCast,
{
    assert_eq!(core::mem::size_of::<T::Uint>(), core::mem::size_of::<T>());
    assert_eq!(core::mem::align_of::<T::Uint>(), core::mem::align_of::<T>());

    let value: *mut T = value;

    // Safety: The requirements of implementing `UintCast`, as well as the size
    // and alignment asserts, ensures that reading `T` as `T::Uint` is safe.
    unsafe { &mut *value.cast::<T::Uint>() }
}

/// Cast from a mutable unsigned integer reference to a mutable color type reference.
///
/// ```
/// use palette::{cast, rgb::PackedArgb, Srgba};
///
/// let mut color: PackedArgb = Srgba::new(0x17, 0xC6, 0x4C, 0xFF).into();
/// assert_eq!(cast::from_uint_mut::<PackedArgb>(&mut 0xFF17C64C), &mut color);
/// ```
#[inline]
pub fn from_uint_mut<T>(value: &mut T::Uint) -> &mut T
where
    T: UintCast,
{
    assert_eq!(core::mem::size_of::<T::Uint>(), core::mem::size_of::<T>());
    assert_eq!(core::mem::align_of::<T::Uint>(), core::mem::align_of::<T>());

    let value: *mut T::Uint = value;

    // Safety: The requirements of implementing `UintCast`, as well as the size
    // and alignment asserts, ensures that reading `T::Uint` as `T` is safe.
    unsafe { &mut *value.cast::<T>() }
}

/// Cast from an array of colors to an array of unsigned integers.
///
/// ```
/// use palette::{cast, rgb::PackedArgb, Srgba};
///
/// let colors: [PackedArgb; 2] = [
///     Srgba::new(0x17, 0xC6, 0x4C, 0xFF).into(),
///     Srgba::new(0x5D, 0x12, 0xD6, 0xFF).into()
/// ];
/// assert_eq!(cast::into_uint_array(colors), [0xFF17C64C, 0xFF5D12D6])
/// ```
#[inline]
pub fn into_uint_array<T, const N: usize>(values: [T; N]) -> [T::Uint; N]
where
    T: UintCast,
{
    assert_eq!(core::mem::size_of::<T::Uint>(), core::mem::size_of::<T>());
    assert_eq!(core::mem::align_of::<T::Uint>(), core::mem::align_of::<T>());

    // Safety: The requirements of implementing `UintCast`, as well as the size
    // and alignment asserts, ensures transmuting `T` into `T::Uint` is safe.
    // The length is the same because the size is the same.
    unsafe { transmute_copy(&ManuallyDrop::new(values)) }
}

/// Cast from an array of unsigned integers to an array of colors.
///
/// ```
/// use palette::{cast, rgb::PackedArgb, Srgba};
///
/// let colors: [PackedArgb; 2] = [
///     Srgba::new(0x17, 0xC6, 0x4C, 0xFF).into(),
///     Srgba::new(0x5D, 0x12, 0xD6, 0xFF).into()
/// ];
/// assert_eq!(cast::from_uint_array::<PackedArgb, 2>([0xFF17C64C, 0xFF5D12D6]), colors)
/// ```
#[inline]
pub fn from_uint_array<T, const N: usize>(values: [T::Uint; N]) -> [T; N]
where
    T: UintCast,
{
    assert_eq!(core::mem::size_of::<T::Uint>(), core::mem::size_of::<T>());
    assert_eq!(core::mem::align_of::<T::Uint>(), core::mem::align_of::<T>());

    // Safety: The requirements of implementing `UintCast`, as well as the size
    // and alignment asserts, ensures transmuting `T::Uint` into `T` is safe.
    // The length is the same because the size is the same.
    unsafe { transmute_copy(&ManuallyDrop::new(values)) }
}

/// Cast from a slice of colors to a slice of unsigned integers.
///
/// ```
/// use palette::{cast, rgb::PackedArgb, Srgba};
///
/// let colors: &[PackedArgb] = &[
///     Srgba::new(0x17, 0xC6, 0x4C, 0xFF).into(),
///     Srgba::new(0x5D, 0x12, 0xD6, 0xFF).into()
/// ];
/// assert_eq!(cast::into_uint_slice(colors), &[0xFF17C64C, 0xFF5D12D6])
/// ```
#[inline]
pub fn into_uint_slice<T>(values: &[T]) -> &[T::Uint]
where
    T: UintCast,
{
    assert_eq!(core::mem::size_of::<T::Uint>(), core::mem::size_of::<T>());
    assert_eq!(core::mem::align_of::<T::Uint>(), core::mem::align_of::<T>());

    // Safety: The requirements of implementing `UintCast`, as well as the size
    // and alignment asserts, ensures that reading `T` as `T::Uint` is safe.
    // The length is the same because the size is the same.
    unsafe { core::slice::from_raw_parts(values.as_ptr().cast::<T::Uint>(), values.len()) }
}

/// Cast from a slice of unsigned integers to a slice of colors.
///
/// ```
/// use palette::{cast, rgb::PackedArgb, Srgba};
///
/// let colors: &[PackedArgb] = &[
///     Srgba::new(0x17, 0xC6, 0x4C, 0xFF).into(),
///     Srgba::new(0x5D, 0x12, 0xD6, 0xFF).into()
/// ];
/// assert_eq!(cast::from_uint_slice::<PackedArgb>(&[0xFF17C64C, 0xFF5D12D6]), colors)
/// ```
#[inline]
pub fn from_uint_slice<T>(values: &[T::Uint]) -> &[T]
where
    T: UintCast,
{
    assert_eq!(core::mem::size_of::<T::Uint>(), core::mem::size_of::<T>());
    assert_eq!(core::mem::align_of::<T::Uint>(), core::mem::align_of::<T>());

    // Safety: The requirements of implementing `UintCast`, as well as the size
    // and alignment asserts, ensures that reading `T::Uint` as `T` is safe.
    // The length is the same because the size is the same.
    unsafe { core::slice::from_raw_parts(values.as_ptr().cast::<T>(), values.len()) }
}

/// Cast from a mutable slice of colors to a mutable slice of unsigned integers.
///
/// ```
/// use palette::{cast, rgb::PackedArgb, Srgba};
///
/// let colors: &mut [PackedArgb] = &mut [
///     Srgba::new(0x17, 0xC6, 0x4C, 0xFF).into(),
///     Srgba::new(0x5D, 0x12, 0xD6, 0xFF).into()
/// ];
/// assert_eq!(cast::into_uint_slice_mut(colors), &mut [0xFF17C64C, 0xFF5D12D6])
/// ```
#[inline]
pub fn into_uint_slice_mut<T>(values: &mut [T]) -> &mut [T::Uint]
where
    T: UintCast,
{
    assert_eq!(core::mem::size_of::<T::Uint>(), core::mem::size_of::<T>());
    assert_eq!(core::mem::align_of::<T::Uint>(), core::mem::align_of::<T>());

    // Safety: The requirements of implementing `UintCast`, as well as the size
    // and alignment asserts, ensures that reading `T` as `T::Uint` is safe.
    // The length is the same because the size is the same.
    unsafe { core::slice::from_raw_parts_mut(values.as_mut_ptr().cast::<T::Uint>(), values.len()) }
}

/// Cast from a mutable slice of unsigned integers to a mutable slice of colors.
///
/// ```
/// use palette::{cast, rgb::PackedArgb, Srgba};
///
/// let colors: &mut [PackedArgb] = &mut [
///     Srgba::new(0x17, 0xC6, 0x4C, 0xFF).into(),
///     Srgba::new(0x5D, 0x12, 0xD6, 0xFF).into()
/// ];
/// assert_eq!(cast::from_uint_slice_mut::<PackedArgb>(&mut [0xFF17C64C, 0xFF5D12D6]), colors)
/// ```
#[inline]
pub fn from_uint_slice_mut<T>(values: &mut [T::Uint]) -> &mut [T]
where
    T: UintCast,
{
    assert_eq!(core::mem::size_of::<T::Uint>(), core::mem::size_of::<T>());
    assert_eq!(core::mem::align_of::<T::Uint>(), core::mem::align_of::<T>());

    // Safety: The requirements of implementing `UintCast`, as well as the size
    // and alignment asserts, ensures that reading `T::Uint` as `T` is safe.
    // The length is the same because the size is the same.
    unsafe { core::slice::from_raw_parts_mut(values.as_mut_ptr().cast::<T>(), values.len()) }
}

/// Cast from a boxed slice of colors to a boxed slice of unsigned integers.
///
/// ```
/// use palette::{cast, rgb::PackedArgb, Srgba};
///
/// let colors: Box<[PackedArgb]> = vec![
///     Srgba::new(0x17, 0xC6, 0x4C, 0xFF).into(),
///     Srgba::new(0x5D, 0x12, 0xD6, 0xFF).into()
/// ].into_boxed_slice();
///
/// assert_eq!(
///     cast::into_uint_slice_box(colors),
///     vec![0xFF17C64C, 0xFF5D12D6].into_boxed_slice()
/// )
/// ```
#[cfg(feature = "alloc")]
#[inline]
pub fn into_uint_slice_box<T>(values: alloc::boxed::Box<[T]>) -> alloc::boxed::Box<[T::Uint]>
where
    T: UintCast,
{
    assert_eq!(core::mem::size_of::<T::Uint>(), core::mem::size_of::<T>());
    assert_eq!(core::mem::align_of::<T::Uint>(), core::mem::align_of::<T>());

    let raw: *mut [T::Uint] = into_uint_slice_mut(alloc::boxed::Box::leak(values));

    // Safety: The requirements of implementing `UintCast`, as well as the size
    // and alignment asserts, ensures that reading `T` as `T::Uint` is safe.
    unsafe { alloc::boxed::Box::from_raw(raw) }
}

/// Cast from a boxed slice of unsigned integers to a boxed slice of colors.
///
/// ```
/// use palette::{cast, rgb::PackedArgb, Srgba};
///
/// let colors: Box<[PackedArgb]> = vec![
///     Srgba::new(0x17, 0xC6, 0x4C, 0xFF).into(),
///     Srgba::new(0x5D, 0x12, 0xD6, 0xFF).into()
/// ].into_boxed_slice();
///
/// assert_eq!(
///     cast::from_uint_slice_box(vec![0xFF17C64C, 0xFF5D12D6].into_boxed_slice()),
///     colors
/// )
/// ```
#[cfg(feature = "alloc")]
#[inline]
pub fn from_uint_slice_box<T>(values: alloc::boxed::Box<[T::Uint]>) -> alloc::boxed::Box<[T]>
where
    T: UintCast,
{
    assert_eq!(core::mem::size_of::<T::Uint>(), core::mem::size_of::<T>());
    assert_eq!(core::mem::align_of::<T::Uint>(), core::mem::align_of::<T>());

    let raw: *mut [T] = from_uint_slice_mut(alloc::boxed::Box::leak(values));

    // Safety: The requirements of implementing `UintCast`, as well as the size
    // and alignment asserts, ensures that reading `T::Uint` as `T` is safe.
    unsafe { alloc::boxed::Box::from_raw(raw) }
}

/// Cast from a `Vec` of colors to a `Vec` of unsigned integers.
///
/// ```
/// use palette::{cast, rgb::PackedArgb, Srgba};
///
/// let colors: Vec<PackedArgb> = vec![
///     Srgba::new(0x17, 0xC6, 0x4C, 0xFF).into(),
///     Srgba::new(0x5D, 0x12, 0xD6, 0xFF).into()
/// ];
///
/// assert_eq!(
///     cast::into_uint_vec(colors),
///     vec![0xFF17C64C, 0xFF5D12D6]
/// )
/// ```
#[cfg(feature = "alloc")]
#[inline]
pub fn into_uint_vec<T>(values: alloc::vec::Vec<T>) -> alloc::vec::Vec<T::Uint>
where
    T: UintCast,
{
    assert_eq!(core::mem::size_of::<T::Uint>(), core::mem::size_of::<T>());
    assert_eq!(core::mem::align_of::<T::Uint>(), core::mem::align_of::<T>());
    let mut values = ManuallyDrop::new(values);

    let raw = values.as_mut_ptr();

    // Safety: The requirements of implementing `UintCast`, as well as the size
    // and alignment asserts, ensures that reading `T` as `T::Uint` is safe.
    // Length and capacity are the same because the size is the same.
    unsafe {
        alloc::vec::Vec::from_raw_parts(raw.cast::<T::Uint>(), values.len(), values.capacity())
    }
}

/// Cast from a `Vec` of unsigned integers to a `Vec` of colors.
///
/// ```
/// use palette::{cast, rgb::PackedArgb, Srgba};
///
/// let colors: Vec<PackedArgb> = vec![
///     Srgba::new(0x17, 0xC6, 0x4C, 0xFF).into(),
///     Srgba::new(0x5D, 0x12, 0xD6, 0xFF).into()
/// ];
///
/// assert_eq!(
///     cast::from_uint_vec::<PackedArgb>(vec![0xFF17C64C, 0xFF5D12D6]),
///     colors
/// )
/// ```
#[cfg(feature = "alloc")]
#[inline]
pub fn from_uint_vec<T>(values: alloc::vec::Vec<T::Uint>) -> alloc::vec::Vec<T>
where
    T: UintCast,
{
    assert_eq!(core::mem::size_of::<T::Uint>(), core::mem::size_of::<T>());
    assert_eq!(core::mem::align_of::<T::Uint>(), core::mem::align_of::<T>());
    let mut values = ManuallyDrop::new(values);

    let raw = values.as_mut_ptr();

    // Safety: The requirements of implementing `UintCast`, as well as the size
    // and alignment asserts, ensures that reading `T::Uint` as `T` is safe.
    // Length and capacity are the same because the size is the same.
    unsafe { alloc::vec::Vec::from_raw_parts(raw.cast::<T>(), values.len(), values.capacity()) }
}
