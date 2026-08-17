use core::marker::PhantomData;

use crate::cast::UintCast;

use super::ArrayCast;

/// A color packed into a compact format, such as an unsigned integer.
///
/// `Packed` implements [ArrayCast](crate::cast::ArrayCast) and
/// [UintCast](crate::cast::UintCast) so it can easily be constructed from
/// slices, arrays and unsigned integers.
///
/// ```
/// // `PackedArgb` is an alias for `Packed<rgb::channels::Argb, P = u32>`.
/// use palette::{rgb::PackedArgb, cast::UintsAs};
///
/// let raw = [0x7F0080u32, 0x60BBCC];
/// let colors: &[PackedArgb] = raw.uints_as();
///
/// assert_eq!(colors.len(), 2);
/// assert_eq!(colors[0].color, 0x7F0080);
/// assert_eq!(colors[1].color, 0x60BBCC);
/// ```
///
/// ## Packed Integer Type Represented in `u32`.
///
/// A common example of a packed format is when an RGBA color is encoded as a
/// hexadecimal number (such as `0x7F0080` from above). Two hexadecimal digits
/// (8-bits) express each value of the Red, Green, Blue, and Alpha components in
/// the RGBA color.
///
/// Note that conversion from float to integer component types in Palette rounds
/// to nearest even: an `Rgb` component of `0.5` will convert to `0x80`/`128`,
/// not `0x7F`/`127`.
///
/// ```
/// use approx::assert_relative_eq;
/// use palette::{Srgb, Srgba};
/// use palette::rgb::{PackedArgb, PackedRgba};
///
/// let packed: PackedArgb = Srgb::new(0.5, 0.0, 0.5).into_format().into();
/// assert_eq!(0xFF80_0080, packed.color);
///
/// let unpacked: Srgba<u8> = PackedRgba::from(0xFFFF_FF80u32).into();
/// assert_relative_eq!(
///     Srgba::new(1.0, 1.0, 1.0, 0.5),
///     unpacked.into_format(),
///     epsilon = 0.01
/// );
///
/// // By default, `Packed` uses `Argb` order for creating `Rgb` colors to make
/// // entering 6-digit hex numbers more convenient
/// let rgb = Srgb::from(0xFF8000);
/// assert_eq!(Srgb::new(0xFF, 0x80, 0x00), rgb);
///
/// let rgba = Srgba::from(0xFF80007F);
/// assert_eq!(Srgba::new(0xFF, 0x80, 0x00, 0x7F), rgba);
/// ```
///
/// When an `Rgb` type is packed, the alpha value will be `0xFF` in the
/// corresponding `u32`. Converting from a packed color type back to an `Rgb`
/// type will disregard the alpha value.
#[derive(Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct Packed<O, P> {
    /// The color packed into a type `P`, such as `u32` or `[u8; 4]`.
    pub color: P,

    /// The channel order for the color components in the packed data. See
    /// [`ComponentOrder`].
    pub channel_order: PhantomData<O>,
}

impl<O, P> Packed<O, P> {
    /// Transform a color value into a packed memory representation.
    #[inline]
    pub fn pack<C>(color: C) -> Self
    where
        O: ComponentOrder<C, P>,
    {
        Packed {
            color: O::pack(color),
            channel_order: PhantomData,
        }
    }

    /// Transform a packed color into a regular color value.
    #[inline]
    pub fn unpack<C>(self) -> C
    where
        O: ComponentOrder<C, P>,
    {
        O::unpack(self.color)
    }
}

impl<O, P> Copy for Packed<O, P> where P: Copy {}

impl<O, P> Clone for Packed<O, P>
where
    P: Clone,
{
    #[inline]
    fn clone(&self) -> Self {
        Self {
            color: self.color.clone(),
            channel_order: PhantomData,
        }
    }
}

// Safety:
//
// `Packed` is a transparent wrapper around `[u8; N]`, which fulfills the
// requirements of `ArrayCast`.
unsafe impl<O, T, const N: usize> ArrayCast for Packed<O, [T; N]> {
    type Array = [T; N];
}

// Safety:
//
// `Packed` is a transparent wrapper around `u8`, which fulfills the
// requirements of `UintCast`.
unsafe impl<O> UintCast for Packed<O, u8> {
    type Uint = u8;
}

// Safety:
//
// `Packed` is a transparent wrapper around `u16`, which fulfills the
// requirements of `UintCast`.
unsafe impl<O> UintCast for Packed<O, u16> {
    type Uint = u16;
}

// Safety:
//
// `Packed` is a transparent wrapper around `u32`, which fulfills the
// requirements of `UintCast`.
unsafe impl<O> UintCast for Packed<O, u32> {
    type Uint = u32;
}

// Safety:
//
// `Packed` is a transparent wrapper around `u64`, which fulfills the
// requirements of `UintCast`.
unsafe impl<O> UintCast for Packed<O, u64> {
    type Uint = u64;
}

// Safety:
//
// `Packed` is a transparent wrapper around `u128`, which fulfills the
// requirements of `UintCast`.
unsafe impl<O> UintCast for Packed<O, u128> {
    type Uint = u128;
}

impl_array_casts!([O, T, const N: usize] Packed<O, [T; N]>, [T; N]);
impl_uint_casts_self!(Packed<O, P>, P, where Packed<O, P>: UintCast<Uint = P>);
impl_uint_casts_other!([O] Packed<O, u8>, u8);
impl_uint_casts_other!([O] Packed<O, u16>, u16);
impl_uint_casts_other!([O] Packed<O, u32>, u32);
impl_uint_casts_other!([O] Packed<O, u64>, u64);
impl_uint_casts_other!([O] Packed<O, u128>, u128);

#[cfg(feature = "bytemuck")]
unsafe impl<O, P> bytemuck::Zeroable for Packed<O, P> where P: bytemuck::Zeroable {}
#[cfg(feature = "bytemuck")]
unsafe impl<O: 'static, P> bytemuck::Pod for Packed<O, P> where P: bytemuck::Pod {}

/// Packs and unpacks color types with some component order.
///
/// As an example, RGBA channels may be ordered as `ABGR`, `ARGB`, `BGRA`, or
/// `RGBA`.
pub trait ComponentOrder<C, P> {
    /// Combine the components of a color into the packed format.
    fn pack(color: C) -> P;

    /// Split the packed color into its separate components.
    fn unpack(packed: P) -> C;
}

impl<C, T> ComponentOrder<C, u8> for T
where
    T: ComponentOrder<C, [u8; 1]>,
{
    #[inline]
    fn pack(color: C) -> u8 {
        let [packed] = T::pack(color);
        packed
    }

    #[inline]
    fn unpack(packed: u8) -> C {
        T::unpack([packed])
    }
}

impl<C, T> ComponentOrder<C, u16> for T
where
    T: ComponentOrder<C, [u8; 2]>,
{
    #[inline]
    fn pack(color: C) -> u16 {
        u16::from_be_bytes(T::pack(color))
    }

    #[inline]
    fn unpack(packed: u16) -> C {
        T::unpack(packed.to_be_bytes())
    }
}

impl<C, T> ComponentOrder<C, u32> for T
where
    T: ComponentOrder<C, [u8; 4]>,
{
    #[inline]
    fn pack(color: C) -> u32 {
        u32::from_be_bytes(T::pack(color))
    }

    #[inline]
    fn unpack(packed: u32) -> C {
        T::unpack(packed.to_be_bytes())
    }
}

impl<C, T> ComponentOrder<C, u64> for T
where
    T: ComponentOrder<C, [u8; 8]>,
{
    #[inline]
    fn pack(color: C) -> u64 {
        u64::from_be_bytes(T::pack(color))
    }

    #[inline]
    fn unpack(packed: u64) -> C {
        T::unpack(packed.to_be_bytes())
    }
}

impl<C, T> ComponentOrder<C, u128> for T
where
    T: ComponentOrder<C, [u8; 16]>,
{
    #[inline]
    fn pack(color: C) -> u128 {
        u128::from_be_bytes(T::pack(color))
    }

    #[inline]
    fn unpack(packed: u128) -> C {
        T::unpack(packed.to_be_bytes())
    }
}
