use core::{
    any::TypeId,
    fmt,
    fmt::Debug,
    marker::PhantomData,
    num::ParseIntError,
    ops::{Add, Div},
    str::FromStr,
};

use crate::{
    alpha::Alpha,
    angle::{RealAngle, UnsignedAngle},
    bool_mask::{BitOps, HasBoolMask, LazySelect},
    cast::{ComponentOrder, Packed},
    color_difference::Wcag21RelativeContrast,
    convert::{FromColorUnclamped, IntoColorUnclamped},
    encoding::{FromLinear, IntoLinear, Linear, Srgb},
    luma::LumaStandard,
    matrix::{matrix_inverse, matrix_map, multiply_xyz_to_rgb, rgb_to_xyz_matrix},
    num::{
        Abs, Arithmetics, FromScalar, IsValidDivisor, MinMax, One, PartialCmp, Real, Recip, Round,
        Trigonometry, Zero,
    },
    oklab::oklab_to_linear_srgb,
    rgb::{RgbSpace, RgbStandard},
    stimulus::{FromStimulus, Stimulus, StimulusColor},
    white_point::{Any, WhitePoint, D65},
    FromColor, GetHue, Hsl, Hsv, IntoColor, Luma, Oklab, RgbHue, Xyz, Yxy,
};

use super::Primaries;

/// Generic RGB with an alpha component. See the [`Rgba` implementation in
/// `Alpha`](crate::Alpha#Rgba).
pub type Rgba<S = Srgb, T = f32> = Alpha<Rgb<S, T>, T>;

/// Generic RGB.
///
/// RGB is probably the most common color space, when it comes to computer
/// graphics, and it's defined as an additive mixture of red, green and blue
/// light, where gray scale colors are created when these three channels are
/// equal in strength.
///
/// # Creating a Value
///
/// RGB comes in different shapes and formats. You will probably want to start
/// with either the [`Srgb`](crate::Srgb) or [`Srgba`](crate::Srgba) alias,
/// which represents the common sRGB format that most images and tools use.
/// Then, depending on your input, you can either just call [`new`](Rgb::new) or
/// convert from another data format.
///
/// ```
/// use palette::Srgb;
///
/// let rgb_u8 = Srgb::new(171u8, 193, 35);
/// let rgb_f32 = Srgb::new(0.3f32, 0.8, 0.1);
///
/// // `new` is also `const`:
/// const RGB_U8: Srgb<u8> = Srgb::new(171, 193, 35);
///
/// // Converting from one number format to another can be as simple as this:
/// let rgb_u8_from_f32_1: Srgb<u8> = Srgb::new(0.3f32, 0.8, 0.1).into();
///
/// // ...or more explicitly like this:
/// let rgb_u8_from_f32_2 = Srgb::new(0.3f32, 0.8, 0.1).into_format::<u8>();
///
/// // Hexadecimal is also supported, with or without the #:
/// let rgb_from_hex1: Srgb<u8> = "#f034e6".parse().unwrap();
/// let rgb_from_hex2: Srgb<u8> = "f034e6".parse().unwrap();
/// assert_eq!(rgb_from_hex1, rgb_from_hex2);
///
/// // This includes the shorthand format:
/// let rgb_from_short_hex: Srgb<u8> = "f3e".parse().unwrap();
/// let rgb_from_long_hex: Srgb<u8> = "ff33ee".parse().unwrap();
/// assert_eq!(rgb_from_short_hex, rgb_from_long_hex);
///
/// // It's also possible to convert from (and to) arrays, tuples and `u32` values:
/// let rgb_from_array = Srgb::from([171u8, 193, 35]);
/// let rgb_from_tuple = Srgb::from((171u8, 193, 35));
/// let rgb_from_u32 = Srgb::from(0x607F00);
/// ```
///
/// # Linear, sRGB and Gamma Correction
///
/// Many conversions and operations on RGB require that it's linear, meaning
/// that gamma correction is required when converting to and from displayable
/// RGB, such as sRGB. It's common to store and send RGB values where the
/// numbers are on a non-linear scale. In a non-linear format, a value of, for
/// example, 0.5 would not represent a light intensity of 50%, which makes some
/// operations (such as blurring) give incorrect results.
///
/// You will probably encounter or use [`LinSrgb`](crate::LinSrgb) or
/// [`LinSrgba`](crate::LinSrgba) at some point. These are aliases for linear
/// sRGB and would usually be obtained by converting an [`Srgb`] value with
/// [`into_linear`](Rgb::into_linear).
///
/// ```no_run
/// use palette::{LinSrgb, Srgb};
///
/// // This function uses linear sRGB for something. But how do we interface with it?
/// fn uses_linear_srgb(input: LinSrgb<f32>) -> LinSrgb<f32> { todo!() }
///
/// // Linear sRGB will usually be created from non-linear sRGB:
/// let output = uses_linear_srgb(Srgb::new(0.3, 0.8, 0.1).into_linear());
///
/// // It's also possible to convert directly from u8 to f32 for sRGB.
/// // This is much faster than using `into_format` first:
/// let output = uses_linear_srgb(Srgb::new(171u8, 193, 35).into_linear());
///
/// // Converting the output back to `Srgb<u8>` (or `Srgb<f32>`) is just as simple:
/// let output_u8 = Srgb::<u8>::from_linear(output);
/// // ..or:
/// let output_u8: Srgb<u8> = output.into_encoding();
/// ```
///
/// It's of course also possible to create a linear value from constants, but
/// it's not necessarily as intuitive. It's best to avoid storing them as
/// `LinSrgb<u8>` (or `LinRgb<_, u8>`) values, to avoid banding among dark
/// colors.
///
/// See the [`encoding`](crate::encoding) module for built-in encoding formats.
///
/// # Storage Formats and Pixel Buffers
///
/// It's common to read and write RGB values as bytes, hexadecimal strings, or
/// sometimes `u32` values. A single RGB value can be converted to all of these
/// formats and more.
///
/// ```no_run
/// use palette::{Srgb, LinSrgb};
///
/// let source: LinSrgb<f32> = todo!();
///
/// let u8_array: [u8; 3] = Srgb::from_linear(source).into();
/// let hex_string1 = format!("#{:x}", Srgb::<u8>::from_linear(source)); // The # is optional.
/// let u32_value: u32 = Srgb::from_linear(source).into();
/// ```
///
/// It's also possible to control the component order.
/// [`PackedArgb`](crate::rgb::PackedArgb) is one of a few aliases for
/// [`Packed`], which represents a color that has been "packed" into a specific
/// data format. This can be a `u32` or `[u8; 4]`, for example. This is helpful
/// for reading and writing colors with a different order than the default RGBA.
///
/// ```no_run
/// use palette::{rgb::PackedArgb, Srgba, LinSrgba};
///
/// let source: LinSrgba<f32> = todo!();
///
/// let u8_array: [u8; 4] = PackedArgb::from(Srgba::from_linear(source)).into();
/// let u32_value: u32 = PackedArgb::from(Srgba::from_linear(source)).into();
/// ```
///
/// If you need to work with colors in a byte buffer, such as `[u8]`, `Vec<u8>`
/// or the `image` crate, there's a quick way to borrow that buffer as a slice
/// of RGB(A) colors. The [`cast`](crate::cast) module has a number of traits
/// and functions for casting values without copying them.
///
/// ```no_run
/// use image::RgbImage;
/// use palette::{cast::ComponentsAsMut, Srgb};
///
/// let mut image: RgbImage = todo!();
/// let pixels: &mut [Srgb<u8>] = image.components_as_mut();
///
/// for pixel in pixels {
///     std::mem::swap(&mut pixel.red, &mut pixel.blue);
/// }
/// ```
#[derive(Debug, ArrayCast, FromColorUnclamped, WithAlpha)]
#[cfg_attr(feature = "serializing", derive(Serialize, Deserialize))]
#[palette(
    palette_internal,
    rgb_standard = "S",
    component = "T",
    skip_derives(Xyz, Hsv, Hsl, Luma, Rgb, Oklab)
)]
#[repr(C)]
pub struct Rgb<S = Srgb, T = f32> {
    /// The amount of red light, where 0.0 is no red light and 1.0 (or 255u8) is
    /// the highest displayable amount.
    pub red: T,

    /// The amount of green light, where 0.0 is no green light and 1.0 (or
    /// 255u8) is the highest displayable amount.
    pub green: T,

    /// The amount of blue light, where 0.0 is no blue light and 1.0 (or 255u8)
    /// is the highest displayable amount.
    pub blue: T,

    /// The kind of RGB standard. sRGB is the default.
    #[cfg_attr(feature = "serializing", serde(skip))]
    #[palette(unsafe_zero_sized)]
    pub standard: PhantomData<S>,
}

impl<S, T> Rgb<S, T> {
    /// Create an RGB color.
    ///
    /// It's possible to create a color in one number format and convert it to
    /// another format with either [`into_format`](Rgb::into_format) or
    /// [`into_linear`](Rgb::into_linear).
    ///
    /// ```
    /// use palette::{Srgb, LinSrgb};
    ///
    /// // Changes only the number format:
    /// let rgb_f32: Srgb<f32> =  Srgb::new(171u8, 193, 35).into_format();
    ///
    /// // Changes the number format and converts to linear in one go.
    /// // This is faster than `.into_format().into_linear()`:
    /// let linear: LinSrgb<f32> = Srgb::new(171u8, 193, 35).into_linear();
    /// ```
    pub const fn new(red: T, green: T, blue: T) -> Rgb<S, T> {
        Rgb {
            red,
            green,
            blue,
            standard: PhantomData,
        }
    }

    /// Convert the RGB components into another number type.
    ///
    /// ```
    /// use palette::Srgb;
    ///
    /// let rgb_u8: Srgb<u8> = Srgb::new(0.3, 0.7, 0.2).into_format();
    /// ```
    ///
    /// See also [`into_linear`](Rgb::into_linear) and
    /// [`into_encoding`](Rgb::into_encoding) for a faster option if you need to
    /// change between linear and non-linear encoding at the same time.
    pub fn into_format<U>(self) -> Rgb<S, U>
    where
        U: FromStimulus<T>,
    {
        Rgb {
            red: U::from_stimulus(self.red),
            green: U::from_stimulus(self.green),
            blue: U::from_stimulus(self.blue),
            standard: PhantomData,
        }
    }

    /// Convert the RGB components from another number type.
    ///
    /// ```
    /// use palette::Srgb;
    ///
    /// let rgb_u8 = Srgb::<u8>::from_format(Srgb::new(0.3, 0.7, 0.2));
    /// ```
    ///
    /// See also [`from_linear`](Rgb::from_linear) and
    /// [`from_encoding`](Rgb::from_encoding) for a faster option if you need to
    /// change between linear and non-linear encoding at the same time.
    pub fn from_format<U>(color: Rgb<S, U>) -> Self
    where
        T: FromStimulus<U>,
    {
        color.into_format()
    }

    /// Convert to a `(red, green, blue)` tuple.
    pub fn into_components(self) -> (T, T, T) {
        (self.red, self.green, self.blue)
    }

    /// Convert from a `(red, green, blue)` tuple.
    pub fn from_components((red, green, blue): (T, T, T)) -> Self {
        Self::new(red, green, blue)
    }
}

impl<S, T> Rgb<S, T>
where
    T: Stimulus,
{
    /// Return the `red` value minimum.
    pub fn min_red() -> T {
        T::zero()
    }

    /// Return the `red` value maximum.
    pub fn max_red() -> T {
        T::max_intensity()
    }

    /// Return the `green` value minimum.
    pub fn min_green() -> T {
        T::zero()
    }

    /// Return the `green` value maximum.
    pub fn max_green() -> T {
        T::max_intensity()
    }

    /// Return the `blue` value minimum.
    pub fn min_blue() -> T {
        T::zero()
    }

    /// Return the `blue` value maximum.
    pub fn max_blue() -> T {
        T::max_intensity()
    }
}

impl<S> Rgb<S, u8> {
    /// Convert to a packed `u32` with with specifiable component order.
    ///
    /// ```
    /// use palette::{rgb, Srgb};
    ///
    /// let integer = Srgb::new(96u8, 127, 0).into_u32::<rgb::channels::Rgba>();
    /// assert_eq!(0x607F00FF, integer);
    /// ```
    ///
    /// It's also possible to use `From` and `Into`, which defaults to the
    /// `0xAARRGGBB` component order:
    ///
    /// ```
    /// use palette::Srgb;
    ///
    /// let integer = u32::from(Srgb::new(96u8, 127, 0));
    /// assert_eq!(0xFF607F00, integer);
    /// ```
    ///
    /// See [Packed](crate::cast::Packed) for more details.
    #[inline]
    pub fn into_u32<O>(self) -> u32
    where
        O: ComponentOrder<Rgba<S, u8>, u32>,
    {
        O::pack(Rgba::from(self))
    }

    /// Convert from a packed `u32` with specifiable component order.
    ///
    /// ```
    /// use palette::{rgb, Srgb};
    ///
    /// let rgb = Srgb::from_u32::<rgb::channels::Rgba>(0x607F00FF);
    /// assert_eq!(Srgb::new(96u8, 127, 0), rgb);
    /// ```
    ///
    /// It's also possible to use `From` and `Into`, which defaults to the
    /// `0xAARRGGBB` component order:
    ///
    /// ```
    /// use palette::Srgb;
    ///
    /// let rgb = Srgb::from(0x607F00);
    /// assert_eq!(Srgb::new(96u8, 127, 0), rgb);
    /// ```
    ///
    /// See [Packed](crate::cast::Packed) for more details.
    #[inline]
    pub fn from_u32<O>(color: u32) -> Self
    where
        O: ComponentOrder<Rgba<S, u8>, u32>,
    {
        O::unpack(color).color
    }
}

impl<S: RgbStandard, T> Rgb<S, T> {
    /// Convert the color to linear RGB.
    ///
    /// Some transfer functions allow the component type to be converted at the
    /// same time. This is usually offered with increased performance, compared
    /// to using [`into_format`][Rgb::into_format].
    ///
    /// ```
    /// use palette::{Srgb, LinSrgb};
    ///
    /// let linear: LinSrgb<f32> = Srgb::new(96u8, 127, 0).into_linear();
    /// ```
    ///
    /// See the transfer function types in the [`encoding`](crate::encoding)
    /// module for details and performance characteristics.
    #[inline(always)]
    pub fn into_linear<U>(self) -> Rgb<Linear<S::Space>, U>
    where
        S::TransferFn: IntoLinear<U, T>,
    {
        Rgb::new(
            S::TransferFn::into_linear(self.red),
            S::TransferFn::into_linear(self.green),
            S::TransferFn::into_linear(self.blue),
        )
    }

    /// Convert linear RGB to non-linear RGB.
    ///
    /// Some transfer functions allow the component type to be converted at the
    /// same time. This is usually offered with increased performance, compared
    /// to using [`into_format`][Rgb::into_format].
    ///
    /// ```
    /// use palette::{Srgb, LinSrgb};
    ///
    /// let encoded = Srgb::<u8>::from_linear(LinSrgb::new(0.95f32, 0.90, 0.30));
    /// ```
    ///
    /// See the transfer function types in the [`encoding`](crate::encoding)
    /// module for details and performance characteristics.
    #[inline(always)]
    pub fn from_linear<U>(color: Rgb<Linear<S::Space>, U>) -> Self
    where
        S::TransferFn: FromLinear<U, T>,
    {
        Rgb::new(
            S::TransferFn::from_linear(color.red),
            S::TransferFn::from_linear(color.green),
            S::TransferFn::from_linear(color.blue),
        )
    }
}

impl<S: RgbSpace, T> Rgb<Linear<S>, T> {
    /// Convert a linear color to a different encoding.
    ///
    /// Some transfer functions allow the component type to be converted at the
    /// same time. This is usually offered with increased performance, compared
    /// to using [`into_format`][Rgb::into_format].
    ///
    /// ```
    /// use palette::{Srgb, LinSrgb};
    ///
    /// let encoded: Srgb<u8> = LinSrgb::new(0.95f32, 0.90, 0.30).into_encoding();
    /// ```
    ///
    /// See the transfer function types in the [`encoding`](crate::encoding)
    /// module for details and performance characteristics.
    pub fn into_encoding<U, St>(self) -> Rgb<St, U>
    where
        St: RgbStandard<Space = S>,
        St::TransferFn: FromLinear<T, U>,
    {
        Rgb::<St, U>::from_linear(self)
    }

    /// Convert linear RGB from a different encoding.
    ///
    /// Some transfer functions allow the component type to be converted at the
    /// same time. This is usually offered with increased performance, compared
    /// to using [`into_format`][Rgb::into_format].
    ///
    /// ```
    /// use palette::{Srgb, LinSrgb};
    ///
    /// let linear = LinSrgb::<f32>::from_encoding(Srgb::new(96u8, 127, 0));
    /// ```
    ///
    /// See the transfer function types in the [`encoding`](crate::encoding)
    /// module for details and performance characteristics.
    pub fn from_encoding<U, St>(color: Rgb<St, U>) -> Self
    where
        St: RgbStandard<Space = S>,
        St::TransferFn: IntoLinear<T, U>,
    {
        color.into_linear()
    }
}

impl<S, T> Rgb<S, T>
where
    S: RgbStandard,
{
    #[inline]
    pub(crate) fn reinterpret_as<St>(self) -> Rgb<St, T>
    where
        S::Space: RgbSpace<WhitePoint = <St::Space as RgbSpace>::WhitePoint>,
        St: RgbStandard,
    {
        Rgb {
            red: self.red,
            green: self.green,
            blue: self.blue,
            standard: PhantomData,
        }
    }
}

/// <span id="Rgba"></span>[`Rgba`](crate::rgb::Rgba) implementations.
impl<S, T, A> Alpha<Rgb<S, T>, A> {
    /// Non-linear RGB.
    pub const fn new(red: T, green: T, blue: T, alpha: A) -> Self {
        Alpha {
            color: Rgb::new(red, green, blue),
            alpha,
        }
    }

    /// Convert the RGBA components into other number types.
    ///
    /// ```
    /// use palette::Srgba;
    ///
    /// let rgba_u8: Srgba<u8> = Srgba::new(0.3, 0.7, 0.2, 0.5).into_format();
    /// ```
    ///
    /// See also `into_linear` and `into_encoding` for a faster option if you
    /// need to change between linear and non-linear encoding at the same time.
    pub fn into_format<U, B>(self) -> Alpha<Rgb<S, U>, B>
    where
        U: FromStimulus<T>,
        B: FromStimulus<A>,
    {
        Alpha {
            color: self.color.into_format(),
            alpha: B::from_stimulus(self.alpha),
        }
    }

    /// Convert the RGBA components from other number types.
    ///
    /// ```
    /// use palette::Srgba;
    ///
    /// let rgba_u8 = Srgba::<u8>::from_format(Srgba::new(0.3, 0.7, 0.2, 0.5));
    /// ```
    ///
    /// See also `from_linear` and `from_encoding` for a faster option if you
    /// need to change between linear and non-linear encoding at the same time.
    pub fn from_format<U, B>(color: Alpha<Rgb<S, U>, B>) -> Self
    where
        T: FromStimulus<U>,
        A: FromStimulus<B>,
    {
        color.into_format()
    }

    /// Convert to a `(red, green, blue, alpha)` tuple.
    pub fn into_components(self) -> (T, T, T, A) {
        (
            self.color.red,
            self.color.green,
            self.color.blue,
            self.alpha,
        )
    }

    /// Convert from a `(red, green, blue, alpha)` tuple.
    pub fn from_components((red, green, blue, alpha): (T, T, T, A)) -> Self {
        Self::new(red, green, blue, alpha)
    }
}

impl<S> Rgba<S, u8> {
    /// Convert to a packed `u32` with with specifiable component order.
    ///
    /// ```
    /// use palette::{rgb, Srgba};
    ///
    /// let integer = Srgba::new(96u8, 127, 0, 255).into_u32::<rgb::channels::Argb>();
    /// assert_eq!(0xFF607F00, integer);
    /// ```
    ///
    /// It's also possible to use `From` and `Into`, which defaults to the
    /// `0xRRGGBBAA` component order:
    ///
    /// ```
    /// use palette::Srgba;
    ///
    /// let integer = u32::from(Srgba::new(96u8, 127, 0, 255));
    /// assert_eq!(0x607F00FF, integer);
    /// ```
    ///
    /// See [Packed](crate::cast::Packed) for more details.
    #[inline]
    pub fn into_u32<O>(self) -> u32
    where
        O: ComponentOrder<Rgba<S, u8>, u32>,
    {
        O::pack(self)
    }

    /// Convert from a packed `u32` with specifiable component order.
    ///
    /// ```
    /// use palette::{rgb, Srgba};
    ///
    /// let rgba = Srgba::from_u32::<rgb::channels::Argb>(0xFF607F00);
    /// assert_eq!(Srgba::new(96u8, 127, 0, 255), rgba);
    /// ```
    ///
    /// It's also possible to use `From` and `Into`, which defaults to the
    /// `0xRRGGBBAA` component order:
    ///
    /// ```
    /// use palette::Srgba;
    ///
    /// let rgba = Srgba::from(0x607F00FF);
    /// assert_eq!(Srgba::new(96u8, 127, 0, 255), rgba);
    /// ```
    ///
    /// See [Packed](crate::cast::Packed) for more details.
    #[inline]
    pub fn from_u32<O>(color: u32) -> Self
    where
        O: ComponentOrder<Rgba<S, u8>, u32>,
    {
        O::unpack(color)
    }
}

impl<S: RgbStandard, T, A> Alpha<Rgb<S, T>, A> {
    /// Convert the color to linear RGB with transparency.
    ///
    /// Some transfer functions allow the component type to be converted at the
    /// same time. This is usually offered with increased performance, compared
    /// to using [`into_format`][Rgb::into_format].
    ///
    /// ```
    /// use palette::{Srgba, LinSrgba};
    ///
    /// let linear: LinSrgba<f32> = Srgba::new(96u8, 127, 0, 38).into_linear();
    /// ```
    ///
    /// See the transfer function types in the [`encoding`](crate::encoding)
    /// module for details and performance characteristics.
    pub fn into_linear<U, B>(self) -> Alpha<Rgb<Linear<S::Space>, U>, B>
    where
        S::TransferFn: IntoLinear<U, T>,
        B: FromStimulus<A>,
    {
        Alpha {
            color: self.color.into_linear(),
            alpha: B::from_stimulus(self.alpha),
        }
    }

    /// Convert linear RGB to non-linear RGB with transparency.
    ///
    /// Some transfer functions allow the component type to be converted at the
    /// same time. This is usually offered with increased performance, compared
    /// to using [`into_format`][Rgb::into_format].
    ///
    /// ```
    /// use palette::{Srgba, LinSrgba};
    ///
    /// let encoded = Srgba::<u8>::from_linear(LinSrgba::new(0.95f32, 0.90, 0.30, 0.75));
    /// ```
    ///
    /// See the transfer function types in the [`encoding`](crate::encoding)
    /// module for details and performance characteristics.
    pub fn from_linear<U, B>(color: Alpha<Rgb<Linear<S::Space>, U>, B>) -> Self
    where
        S::TransferFn: FromLinear<U, T>,
        A: FromStimulus<B>,
    {
        Alpha {
            color: Rgb::from_linear(color.color),
            alpha: A::from_stimulus(color.alpha),
        }
    }
}

impl<S: RgbSpace, T, A> Alpha<Rgb<Linear<S>, T>, A> {
    /// Convert a linear color to a different encoding with transparency.
    ///
    /// Some transfer functions allow the component type to be converted at the
    /// same time. This is usually offered with increased performance, compared
    /// to using [`into_format`][Rgb::into_format].
    ///
    /// ```
    /// use palette::{Srgba, LinSrgba};
    ///
    /// let encoded: Srgba<u8> = LinSrgba::new(0.95f32, 0.90, 0.30, 0.75).into_encoding();
    /// ```
    ///
    /// See the transfer function types in the [`encoding`](crate::encoding)
    /// module for details and performance characteristics.
    pub fn into_encoding<U, B, St>(self) -> Alpha<Rgb<St, U>, B>
    where
        St: RgbStandard<Space = S>,
        St::TransferFn: FromLinear<T, U>,
        B: FromStimulus<A>,
    {
        Alpha::<Rgb<St, U>, B>::from_linear(self)
    }

    /// Convert RGB from a different encoding to linear with transparency.
    ///
    /// Some transfer functions allow the component type to be converted at the
    /// same time. This is usually offered with increased performance, compared
    /// to using [`into_format`][Rgb::into_format].
    ///
    /// ```
    /// use palette::{Srgba, LinSrgba};
    ///
    /// let linear = LinSrgba::<f32>::from_encoding(Srgba::new(96u8, 127, 0, 38));
    /// ```
    ///
    /// See the transfer function types in the [`encoding`](crate::encoding)
    /// module for details and performance characteristics.
    pub fn from_encoding<U, B, St>(color: Alpha<Rgb<St, U>, B>) -> Self
    where
        St: RgbStandard<Space = S>,
        St::TransferFn: IntoLinear<T, U>,
        A: FromStimulus<B>,
    {
        color.into_linear()
    }
}

impl_reference_component_methods!(Rgb<S>, [red, green, blue], standard);
impl_struct_of_arrays_methods!(Rgb<S>, [red, green, blue], standard);

impl<S1, S2, T> FromColorUnclamped<Rgb<S2, T>> for Rgb<S1, T>
where
    S1: RgbStandard + 'static,
    S2: RgbStandard + 'static,
    S1::TransferFn: FromLinear<T, T>,
    S2::TransferFn: IntoLinear<T, T>,
    S2::Space: RgbSpace<WhitePoint = <S1::Space as RgbSpace>::WhitePoint>,
    Xyz<<S2::Space as RgbSpace>::WhitePoint, T>: FromColorUnclamped<Rgb<S2, T>>,
    Rgb<S1, T>: FromColorUnclamped<Xyz<<S1::Space as RgbSpace>::WhitePoint, T>>,
{
    fn from_color_unclamped(rgb: Rgb<S2, T>) -> Self {
        let rgb_space1 = TypeId::of::<<S1::Space as RgbSpace>::Primaries>();
        let rgb_space2 = TypeId::of::<<S2::Space as RgbSpace>::Primaries>();

        if TypeId::of::<S1>() == TypeId::of::<S2>() {
            rgb.reinterpret_as()
        } else if rgb_space1 == rgb_space2 {
            Self::from_linear(rgb.into_linear().reinterpret_as())
        } else {
            Self::from_color_unclamped(Xyz::from_color_unclamped(rgb))
        }
    }
}

impl<S, T> FromColorUnclamped<Xyz<<S::Space as RgbSpace>::WhitePoint, T>> for Rgb<S, T>
where
    S: RgbStandard,
    S::TransferFn: FromLinear<T, T>,
    <S::Space as RgbSpace>::Primaries: Primaries<T::Scalar>,
    <S::Space as RgbSpace>::WhitePoint: WhitePoint<T::Scalar>,
    T: Arithmetics + FromScalar,
    T::Scalar: Real
        + Recip
        + IsValidDivisor<Mask = bool>
        + Arithmetics
        + Clone
        + FromScalar<Scalar = T::Scalar>,
    Yxy<Any, T::Scalar>: IntoColorUnclamped<Xyz<Any, T::Scalar>>,
{
    fn from_color_unclamped(color: Xyz<<S::Space as RgbSpace>::WhitePoint, T>) -> Self {
        let transform_matrix = S::Space::xyz_to_rgb_matrix().map_or_else(
            || matrix_inverse(rgb_to_xyz_matrix::<S::Space, T::Scalar>()),
            |matrix| matrix_map(matrix, T::Scalar::from_f64),
        );
        Self::from_linear(multiply_xyz_to_rgb(transform_matrix, color))
    }
}

impl<S, T> FromColorUnclamped<Hsl<S, T>> for Rgb<S, T>
where
    T: Real
        + RealAngle
        + UnsignedAngle
        + Zero
        + One
        + Abs
        + Round
        + PartialCmp
        + Arithmetics
        + Clone,
    T::Mask: LazySelect<T> + BitOps + Clone,
{
    fn from_color_unclamped(hsl: Hsl<S, T>) -> Self {
        let c = (T::one() - (hsl.lightness.clone() * T::from_f64(2.0) - T::one()).abs())
            * hsl.saturation;
        let h = hsl.hue.into_positive_degrees() / T::from_f64(60.0);
        // We avoid using %, since it's not always (or never?) supported in SIMD
        let h_mod_two = h.clone() - Round::floor(h.clone() * T::from_f64(0.5)) * T::from_f64(2.0);
        let x = c.clone() * (T::one() - (h_mod_two - T::one()).abs());
        let m = hsl.lightness - c.clone() * T::from_f64(0.5);

        let is_zone0 = h.gt_eq(&T::zero()) & h.lt(&T::one());
        let is_zone1 = h.gt_eq(&T::one()) & h.lt(&T::from_f64(2.0));
        let is_zone2 = h.gt_eq(&T::from_f64(2.0)) & h.lt(&T::from_f64(3.0));
        let is_zone3 = h.gt_eq(&T::from_f64(3.0)) & h.lt(&T::from_f64(4.0));
        let is_zone4 = h.gt_eq(&T::from_f64(4.0)) & h.lt(&T::from_f64(5.0));

        let red = lazy_select! {
            if is_zone1.clone() | &is_zone4 => x.clone(),
            if is_zone2.clone() | &is_zone3 => T::zero(),
            else => c.clone(),
        };

        let green = lazy_select! {
            if is_zone0.clone() | &is_zone3 => x.clone(),
            if is_zone1.clone() | &is_zone2 => c.clone(),
            else => T::zero(),
        };

        let blue = lazy_select! {
            if is_zone0 | is_zone1 => T::zero(),
            if is_zone3 | is_zone4 => c,
            else => x,
        };

        Rgb {
            red: red + m.clone(),
            green: green + m.clone(),
            blue: blue + m,
            standard: PhantomData,
        }
    }
}

impl<S, T> FromColorUnclamped<Hsv<S, T>> for Rgb<S, T>
where
    T: Real
        + RealAngle
        + UnsignedAngle
        + Round
        + Zero
        + One
        + Abs
        + PartialCmp
        + Arithmetics
        + Clone,
    T::Mask: LazySelect<T> + BitOps + Clone,
{
    fn from_color_unclamped(hsv: Hsv<S, T>) -> Self {
        let c = hsv.value.clone() * hsv.saturation;
        let h = hsv.hue.into_positive_degrees() / T::from_f64(60.0);
        // We avoid using %, since it's not always (or never?) supported in SIMD
        let h_mod_two = h.clone() - Round::floor(h.clone() * T::from_f64(0.5)) * T::from_f64(2.0);
        let x = c.clone() * (T::one() - (h_mod_two - T::one()).abs());
        let m = hsv.value - c.clone();

        let is_zone0 = h.gt_eq(&T::zero()) & h.lt(&T::one());
        let is_zone1 = h.gt_eq(&T::one()) & h.lt(&T::from_f64(2.0));
        let is_zone2 = h.gt_eq(&T::from_f64(2.0)) & h.lt(&T::from_f64(3.0));
        let is_zone3 = h.gt_eq(&T::from_f64(3.0)) & h.lt(&T::from_f64(4.0));
        let is_zone4 = h.gt_eq(&T::from_f64(4.0)) & h.lt(&T::from_f64(5.0));

        let red = lazy_select! {
            if is_zone1.clone() | &is_zone4 => x.clone(),
            if is_zone2.clone() | &is_zone3 => T::zero(),
            else => c.clone(),
        };

        let green = lazy_select! {
            if is_zone0.clone() | &is_zone3 => x.clone(),
            if is_zone1.clone() | &is_zone2 => c.clone(),
            else => T::zero(),
        };

        let blue = lazy_select! {
            if is_zone0 | is_zone1 => T::zero(),
            if is_zone3 | is_zone4 => c,
            else => x,
        };

        Rgb {
            red: red + m.clone(),
            green: green + m.clone(),
            blue: blue + m,
            standard: PhantomData,
        }
    }
}

impl<S, St, T> FromColorUnclamped<Luma<St, T>> for Rgb<S, T>
where
    S: RgbStandard + 'static,
    St: LumaStandard<WhitePoint = <S::Space as RgbSpace>::WhitePoint> + 'static,
    S::TransferFn: FromLinear<T, T>,
    St::TransferFn: IntoLinear<T, T>,
    T: Clone,
{
    #[inline]
    fn from_color_unclamped(color: Luma<St, T>) -> Self {
        if TypeId::of::<S::TransferFn>() == TypeId::of::<St::TransferFn>() {
            Rgb {
                red: color.luma.clone(),
                green: color.luma.clone(),
                blue: color.luma,
                standard: PhantomData,
            }
        } else {
            let luma = color.into_linear();

            Self::from_linear(Rgb {
                red: luma.luma.clone(),
                green: luma.luma.clone(),
                blue: luma.luma,
                standard: PhantomData,
            })
        }
    }
}

impl<S, T> FromColorUnclamped<Oklab<T>> for Rgb<S, T>
where
    T: Real + Arithmetics + Copy,
    S: RgbStandard,
    S::TransferFn: FromLinear<T, T>,
    S::Space: RgbSpace<WhitePoint = D65> + 'static,
    Rgb<Linear<Srgb>, T>: IntoColorUnclamped<Self>,
    Xyz<D65, T>: FromColorUnclamped<Oklab<T>> + IntoColorUnclamped<Self>,
{
    fn from_color_unclamped(oklab: Oklab<T>) -> Self {
        if TypeId::of::<<S as RgbStandard>::Space>() == TypeId::of::<Srgb>() {
            // Use direct sRGB to Oklab conversion
            // Rounding errors are likely a contributing factor to differences.
            // Also the conversion via XYZ doesn't use pre-defined matrices (yet)
            oklab_to_linear_srgb(oklab).into_color_unclamped()
        } else {
            // Convert via XYZ
            Xyz::from_color_unclamped(oklab).into_color_unclamped()
        }
    }
}

impl_is_within_bounds! {
    Rgb<S> {
        red => [Self::min_red(), Self::max_red()],
        green => [Self::min_green(), Self::max_green()],
        blue => [Self::min_blue(), Self::max_blue()]
    }
    where T: Stimulus
}
impl_clamp! {
    Rgb<S> {
        red => [Self::min_red(), Self::max_red()],
        green => [Self::min_green(), Self::max_green()],
        blue => [Self::min_blue(), Self::max_blue()]
    }
    other {standard}
    where T: Stimulus
}

impl_mix!(Rgb<S>);
impl_lighten! {
    Rgb<S>
    increase {
        red => [Self::min_red(), Self::max_red()],
        green => [Self::min_green(), Self::max_green()],
        blue => [Self::min_blue(), Self::max_blue()]
    }
    other {}
    phantom: standard
    where T: Stimulus,
}

impl<S, T> GetHue for Rgb<S, T>
where
    T: Real + RealAngle + Trigonometry + Arithmetics + Clone,
{
    type Hue = RgbHue<T>;

    fn get_hue(&self) -> RgbHue<T> {
        let sqrt_3: T = T::from_f64(1.73205081);

        RgbHue::from_cartesian(
            self.red.clone() * T::from_f64(2.0) - self.green.clone() - self.blue.clone(),
            sqrt_3 * (self.green.clone() - self.blue.clone()),
        )
    }
}

impl_premultiply!(Rgb<S> {red, green, blue} phantom: standard);
impl_euclidean_distance!(Rgb<S> {red, green, blue});

impl<S, T> StimulusColor for Rgb<S, T> where T: Stimulus {}

impl<S, T> HasBoolMask for Rgb<S, T>
where
    T: HasBoolMask,
{
    type Mask = T::Mask;
}

impl<S, T> Default for Rgb<S, T>
where
    T: Stimulus,
{
    fn default() -> Rgb<S, T> {
        Rgb::new(Self::min_red(), Self::min_green(), Self::min_blue())
    }
}

impl_color_add!(Rgb<S>, [red, green, blue], standard);
impl_color_sub!(Rgb<S>, [red, green, blue], standard);
impl_color_mul!(Rgb<S>, [red, green, blue], standard);
impl_color_div!(Rgb<S>, [red, green, blue], standard);

impl_tuple_conversion!(Rgb<S> as (T, T, T));
impl_array_casts!(Rgb<S, T>, [T; 3]);
impl_simd_array_conversion!(Rgb<S>, [red, green, blue], standard);
impl_struct_of_array_traits!(Rgb<S>, [red, green, blue], standard);

impl_eq!(Rgb<S>, [red, green, blue]);
impl_copy_clone!(Rgb<S>, [red, green, blue], standard);

impl<S, T> fmt::LowerHex for Rgb<S, T>
where
    T: fmt::LowerHex,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let size = f.width().unwrap_or(::core::mem::size_of::<T>() * 2);
        write!(
            f,
            "{:0width$x}{:0width$x}{:0width$x}",
            self.red,
            self.green,
            self.blue,
            width = size
        )
    }
}

impl<S, T> fmt::UpperHex for Rgb<S, T>
where
    T: fmt::UpperHex,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let size = f.width().unwrap_or(::core::mem::size_of::<T>() * 2);
        write!(
            f,
            "{:0width$X}{:0width$X}{:0width$X}",
            self.red,
            self.green,
            self.blue,
            width = size
        )
    }
}

/// Error type for parsing a string of hexadecimal characters to an `Rgb` color.
#[derive(Debug)]
pub enum FromHexError {
    /// An error occurred while parsing the string into a valid integer.
    ParseIntError(ParseIntError),
    /// The hex value was not in a valid 3 or 6 character format.
    HexFormatError(&'static str),
    /// The hex value was not in a valid 4 or 8 character format.
    RgbaHexFormatError(&'static str),
}

impl From<ParseIntError> for FromHexError {
    fn from(err: ParseIntError) -> FromHexError {
        FromHexError::ParseIntError(err)
    }
}

impl From<&'static str> for FromHexError {
    fn from(err: &'static str) -> FromHexError {
        FromHexError::HexFormatError(err)
    }
}

impl core::fmt::Display for FromHexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FromHexError::ParseIntError(e) => write!(f, "{}", e),
            FromHexError::HexFormatError(s) => write!(
                f,
                "{}, please use format '#fff', 'fff', '#ffffff' or 'ffffff'.",
                s
            ),
            FromHexError::RgbaHexFormatError(s) => write!(
                f,
                "{}, please use format '#ffff', 'ffff', '#ffffffff' or 'ffffffff'.",
                s
            ),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for FromHexError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            FromHexError::HexFormatError(_s) => None,
            FromHexError::RgbaHexFormatError(_s) => None,
            FromHexError::ParseIntError(e) => Some(e),
        }
    }
}

impl<S> FromStr for Rgb<S, u8> {
    type Err = FromHexError;

    /// Parses a color hex code of format '#ff00bb' or '#abc' (with or without the leading '#') into a
    /// [`Rgb<S, u8>`] instance.
    fn from_str(hex: &str) -> Result<Self, Self::Err> {
        let hex_code = hex.strip_prefix('#').map_or(hex, |stripped| stripped);
        match hex_code.len() {
            3 => {
                let red = u8::from_str_radix(&hex_code[..1], 16)?;
                let green = u8::from_str_radix(&hex_code[1..2], 16)?;
                let blue = u8::from_str_radix(&hex_code[2..3], 16)?;
                let col: Rgb<S, u8> = Rgb::new(red * 17, green * 17, blue * 17);
                Ok(col)
            }
            6 => {
                let red = u8::from_str_radix(&hex_code[..2], 16)?;
                let green = u8::from_str_radix(&hex_code[2..4], 16)?;
                let blue = u8::from_str_radix(&hex_code[4..6], 16)?;
                let col: Rgb<S, u8> = Rgb::new(red, green, blue);
                Ok(col)
            }
            _ => Err(FromHexError::HexFormatError("invalid hex code format")),
        }
    }
}

impl<S> FromStr for Rgba<S, u8> {
    type Err = FromHexError;

    /// Parses a color hex code of format '#ff00bbff' or '#abcd' (with or without the leading '#') into a
    /// [`Rgba<S, u8>`] instance.
    fn from_str(hex: &str) -> Result<Self, Self::Err> {
        let hex_code = hex.strip_prefix('#').map_or(hex, |stripped| stripped);
        match hex_code.len() {
            4 => {
                let red = u8::from_str_radix(&hex_code[..1], 16)?;
                let green = u8::from_str_radix(&hex_code[1..2], 16)?;
                let blue = u8::from_str_radix(&hex_code[2..3], 16)?;
                let alpha = u8::from_str_radix(&hex_code[3..4], 16)?;
                let col: Rgba<S, u8> = Rgba::new(red * 17, green * 17, blue * 17, alpha * 17);
                Ok(col)
            }
            8 => {
                let red = u8::from_str_radix(&hex_code[..2], 16)?;
                let green = u8::from_str_radix(&hex_code[2..4], 16)?;
                let blue = u8::from_str_radix(&hex_code[4..6], 16)?;
                let alpha = u8::from_str_radix(&hex_code[6..8], 16)?;
                let col: Rgba<S, u8> = Rgba::new(red, green, blue, alpha);
                Ok(col)
            }
            _ => Err(FromHexError::RgbaHexFormatError("invalid hex code format")),
        }
    }
}

impl<S, T, P, O> From<Rgb<S, T>> for Packed<O, P>
where
    O: ComponentOrder<Rgba<S, T>, P>,
    Rgba<S, T>: From<Rgb<S, T>>,
{
    #[inline]
    fn from(color: Rgb<S, T>) -> Self {
        Self::from(Rgba::from(color))
    }
}

impl<S, T, O, P> From<Rgba<S, T>> for Packed<O, P>
where
    O: ComponentOrder<Rgba<S, T>, P>,
{
    #[inline]
    fn from(color: Rgba<S, T>) -> Self {
        Packed::pack(color)
    }
}

impl<S, O, P> From<Packed<O, P>> for Rgb<S, u8>
where
    O: ComponentOrder<Rgba<S, u8>, P>,
{
    #[inline]
    fn from(packed: Packed<O, P>) -> Self {
        Rgba::from(packed).color
    }
}

impl<S, T, O, P> From<Packed<O, P>> for Rgba<S, T>
where
    O: ComponentOrder<Rgba<S, T>, P>,
{
    #[inline]
    fn from(packed: Packed<O, P>) -> Self {
        packed.unpack()
    }
}

impl<S> From<u32> for Rgb<S, u8> {
    #[inline]
    fn from(color: u32) -> Self {
        Self::from_u32::<super::channels::Argb>(color)
    }
}

impl<S> From<u32> for Rgba<S, u8> {
    #[inline]
    fn from(color: u32) -> Self {
        Self::from_u32::<super::channels::Rgba>(color)
    }
}

impl<S> From<Rgb<S, u8>> for u32 {
    #[inline]
    fn from(color: Rgb<S, u8>) -> Self {
        Rgb::into_u32::<super::channels::Argb>(color)
    }
}

impl<S> From<Rgba<S, u8>> for u32 {
    #[inline]
    fn from(color: Rgba<S, u8>) -> Self {
        Rgba::into_u32::<super::channels::Rgba>(color)
    }
}

impl<S> From<Rgb<S, u8>> for Rgb<S, f32> {
    #[inline]
    fn from(color: Rgb<S, u8>) -> Self {
        color.into_format()
    }
}

impl<S> From<Rgba<S, u8>> for Rgba<S, f32> {
    #[inline]
    fn from(color: Rgba<S, u8>) -> Self {
        color.into_format()
    }
}

impl<S> From<Rgb<S, f32>> for Rgb<S, u8> {
    #[inline]
    fn from(color: Rgb<S, f32>) -> Self {
        color.into_format()
    }
}

impl<S> From<Rgba<S, f32>> for Rgba<S, u8> {
    #[inline]
    fn from(color: Rgba<S, f32>) -> Self {
        color.into_format()
    }
}

impl<S> From<Rgb<S, u8>> for Rgb<S, f64> {
    #[inline]
    fn from(color: Rgb<S, u8>) -> Self {
        color.into_format()
    }
}

impl<S> From<Rgba<S, u8>> for Rgba<S, f64> {
    #[inline]
    fn from(color: Rgba<S, u8>) -> Self {
        color.into_format()
    }
}

impl<S> From<Rgb<S, f64>> for Rgb<S, u8> {
    #[inline]
    fn from(color: Rgb<S, f64>) -> Self {
        color.into_format()
    }
}

impl<S> From<Rgba<S, f64>> for Rgba<S, u8> {
    #[inline]
    fn from(color: Rgba<S, f64>) -> Self {
        color.into_format()
    }
}

impl<S> From<Rgb<S, f32>> for Rgb<S, f64> {
    #[inline]
    fn from(color: Rgb<S, f32>) -> Self {
        color.into_format()
    }
}

impl<S> From<Rgba<S, f32>> for Rgba<S, f64> {
    #[inline]
    fn from(color: Rgba<S, f32>) -> Self {
        color.into_format()
    }
}

impl<S> From<Rgb<S, f64>> for Rgb<S, f32> {
    #[inline]
    fn from(color: Rgb<S, f64>) -> Self {
        color.into_format()
    }
}

impl<S> From<Rgba<S, f64>> for Rgba<S, f32> {
    #[inline]
    fn from(color: Rgba<S, f64>) -> Self {
        color.into_format()
    }
}

#[allow(deprecated)]
impl<S, T> crate::RelativeContrast for Rgb<S, T>
where
    T: Real + Arithmetics + PartialCmp,
    T::Mask: LazySelect<T>,
    S: RgbStandard,
    Xyz<<<S as RgbStandard>::Space as RgbSpace>::WhitePoint, T>: FromColor<Self>,
{
    type Scalar = T;

    #[inline]
    fn get_contrast_ratio(self, other: Self) -> T {
        let xyz1 = Xyz::from_color(self);
        let xyz2 = Xyz::from_color(other);

        crate::contrast_ratio(xyz1.y, xyz2.y)
    }
}

impl<S, T> Wcag21RelativeContrast for Rgb<S, T>
where
    Self: IntoColor<Luma<Linear<D65>, T>>,
    S: RgbStandard<Space = crate::encoding::srgb::Srgb>,
    T: Real + Add<T, Output = T> + Div<T, Output = T> + PartialCmp + MinMax,
{
    type Scalar = T;

    fn relative_luminance(self) -> Luma<Linear<D65>, Self::Scalar> {
        self.into_color()
    }
}

impl_rand_traits_cartesian!(UniformRgb, Rgb<S> {red, green, blue} phantom: standard: PhantomData<S>);

#[cfg(feature = "bytemuck")]
unsafe impl<S, T> bytemuck::Zeroable for Rgb<S, T> where T: bytemuck::Zeroable {}

#[cfg(feature = "bytemuck")]
unsafe impl<S: 'static, T> bytemuck::Pod for Rgb<S, T> where T: bytemuck::Pod {}

#[cfg(test)]
mod test {
    use core::str::FromStr;

    use crate::encoding::Srgb;
    use crate::rgb::channels;

    use super::{Rgb, Rgba};

    test_convert_into_from_xyz!(Rgb);

    #[test]
    fn ranges() {
        assert_ranges! {
            Rgb<Srgb, f64>;
            clamped {
                red: 0.0 => 1.0,
                green: 0.0 => 1.0,
                blue: 0.0 => 1.0
            }
            clamped_min {}
            unclamped {}
        }
    }

    raw_pixel_conversion_tests!(Rgb<Srgb>: red, green, blue);
    raw_pixel_conversion_fail_tests!(Rgb<Srgb>: red, green, blue);

    #[test]
    fn lower_hex() {
        assert_eq!(
            format!("{:x}", Rgb::<Srgb, u8>::new(171, 193, 35)),
            "abc123"
        );
    }

    #[test]
    fn lower_hex_small_numbers() {
        assert_eq!(format!("{:x}", Rgb::<Srgb, u8>::new(1, 2, 3)), "010203");
        assert_eq!(
            format!("{:x}", Rgb::<Srgb, u16>::new(1, 2, 3)),
            "000100020003"
        );
        assert_eq!(
            format!("{:x}", Rgb::<Srgb, u32>::new(1, 2, 3)),
            "000000010000000200000003"
        );
        assert_eq!(
            format!("{:x}", Rgb::<Srgb, u64>::new(1, 2, 3)),
            "000000000000000100000000000000020000000000000003"
        );
    }

    #[test]
    fn lower_hex_custom_width() {
        assert_eq!(
            format!("{:03x}", Rgb::<Srgb, u8>::new(1, 2, 3)),
            "001002003"
        );
        assert_eq!(
            format!("{:03x}", Rgb::<Srgb, u16>::new(1, 2, 3)),
            "001002003"
        );
        assert_eq!(
            format!("{:03x}", Rgb::<Srgb, u32>::new(1, 2, 3)),
            "001002003"
        );
        assert_eq!(
            format!("{:03x}", Rgb::<Srgb, u64>::new(1, 2, 3)),
            "001002003"
        );
    }

    #[test]
    fn upper_hex() {
        assert_eq!(
            format!("{:X}", Rgb::<Srgb, u8>::new(171, 193, 35)),
            "ABC123"
        );
    }

    #[test]
    fn upper_hex_small_numbers() {
        assert_eq!(format!("{:X}", Rgb::<Srgb, u8>::new(1, 2, 3)), "010203");
        assert_eq!(
            format!("{:X}", Rgb::<Srgb, u16>::new(1, 2, 3)),
            "000100020003"
        );
        assert_eq!(
            format!("{:X}", Rgb::<Srgb, u32>::new(1, 2, 3)),
            "000000010000000200000003"
        );
        assert_eq!(
            format!("{:X}", Rgb::<Srgb, u64>::new(1, 2, 3)),
            "000000000000000100000000000000020000000000000003"
        );
    }

    #[test]
    fn upper_hex_custom_width() {
        assert_eq!(
            format!("{:03X}", Rgb::<Srgb, u8>::new(1, 2, 3)),
            "001002003"
        );
        assert_eq!(
            format!("{:03X}", Rgb::<Srgb, u16>::new(1, 2, 3)),
            "001002003"
        );
        assert_eq!(
            format!("{:03X}", Rgb::<Srgb, u32>::new(1, 2, 3)),
            "001002003"
        );
        assert_eq!(
            format!("{:03X}", Rgb::<Srgb, u64>::new(1, 2, 3)),
            "001002003"
        );
    }

    #[test]
    fn rgb_hex_into_from() {
        let c1 = Rgb::<Srgb, u8>::from_u32::<channels::Argb>(0x1100_7FFF);
        let c2 = Rgb::<Srgb, u8>::new(0u8, 127, 255);
        assert_eq!(c1, c2);
        assert_eq!(Rgb::<Srgb, u8>::into_u32::<channels::Argb>(c1), 0xFF00_7FFF);

        let c1 = Rgba::<Srgb, u8>::from_u32::<channels::Rgba>(0x007F_FF80);
        let c2 = Rgba::<Srgb, u8>::new(0u8, 127, 255, 128);
        assert_eq!(c1, c2);
        assert_eq!(
            Rgba::<Srgb, u8>::into_u32::<channels::Rgba>(c1),
            0x007F_FF80
        );

        assert_eq!(
            Rgb::<Srgb, u8>::from(0x7FFF_FF80),
            Rgb::from((255u8, 255, 128))
        );
        assert_eq!(
            Rgba::<Srgb, u8>::from(0x7FFF_FF80),
            Rgba::from((127u8, 255, 255, 128))
        );
    }

    #[cfg(feature = "serializing")]
    #[test]
    fn serialize() {
        let serialized = ::serde_json::to_string(&Rgb::<Srgb>::new(0.3, 0.8, 0.1)).unwrap();

        assert_eq!(serialized, r#"{"red":0.3,"green":0.8,"blue":0.1}"#);
    }

    #[cfg(feature = "serializing")]
    #[test]
    fn deserialize() {
        let deserialized: Rgb<Srgb> =
            ::serde_json::from_str(r#"{"red":0.3,"green":0.8,"blue":0.1}"#).unwrap();

        assert_eq!(deserialized, Rgb::<Srgb>::new(0.3, 0.8, 0.1));
    }

    #[test]
    fn from_str() {
        let c = Rgb::<Srgb, u8>::from_str("#ffffff");
        assert!(c.is_ok());
        assert_eq!(c.unwrap(), Rgb::<Srgb, u8>::new(255, 255, 255));
        let c = Rgb::<Srgb, u8>::from_str("#gggggg");
        assert!(c.is_err());
        let c = Rgb::<Srgb, u8>::from_str("#fff");
        assert!(c.is_ok());
        assert_eq!(c.unwrap(), Rgb::<Srgb, u8>::new(255, 255, 255));
        let c = Rgb::<Srgb, u8>::from_str("#000000");
        assert!(c.is_ok());
        assert_eq!(c.unwrap(), Rgb::<Srgb, u8>::new(0, 0, 0));
        let c = Rgb::<Srgb, u8>::from_str("");
        assert!(c.is_err());
        let c = Rgb::<Srgb, u8>::from_str("#123456");
        assert!(c.is_ok());
        assert_eq!(c.unwrap(), Rgb::<Srgb, u8>::new(18, 52, 86));
        let c = Rgb::<Srgb, u8>::from_str("#iii");
        assert!(c.is_err());
        assert_eq!(
            format!("{}", c.err().unwrap()),
            "invalid digit found in string"
        );
        let c = Rgb::<Srgb, u8>::from_str("#08f");
        assert_eq!(c.unwrap(), Rgb::<Srgb, u8>::new(0, 136, 255));
        let c = Rgb::<Srgb, u8>::from_str("08f");
        assert_eq!(c.unwrap(), Rgb::<Srgb, u8>::new(0, 136, 255));
        let c = Rgb::<Srgb, u8>::from_str("ffffff");
        assert_eq!(c.unwrap(), Rgb::<Srgb, u8>::new(255, 255, 255));
        let c = Rgb::<Srgb, u8>::from_str("#12");
        assert!(c.is_err());
        assert_eq!(
            format!("{}", c.err().unwrap()),
            "invalid hex code format, \
             please use format \'#fff\', \'fff\', \'#ffffff\' or \'ffffff\'."
        );
        let c = Rgb::<Srgb, u8>::from_str("da0bce");
        assert_eq!(c.unwrap(), Rgb::<Srgb, u8>::new(218, 11, 206));
        let c = Rgb::<Srgb, u8>::from_str("f034e6");
        assert_eq!(c.unwrap(), Rgb::<Srgb, u8>::new(240, 52, 230));
        let c = Rgb::<Srgb, u8>::from_str("abc");
        assert_eq!(c.unwrap(), Rgb::<Srgb, u8>::new(170, 187, 204));
        let c = Rgba::<Srgb, u8>::from_str("#08ff");
        assert_eq!(c.unwrap(), Rgba::<Srgb, u8>::new(0, 136, 255, 255));
        let c = Rgba::<Srgb, u8>::from_str("08f0");
        assert_eq!(c.unwrap(), Rgba::<Srgb, u8>::new(0, 136, 255, 0));
        let c = Rgba::<Srgb, u8>::from_str("#da0bce80");
        assert_eq!(c.unwrap(), Rgba::<Srgb, u8>::new(218, 11, 206, 128));
        let c = Rgba::<Srgb, u8>::from_str("f034e680");
        assert_eq!(c.unwrap(), Rgba::<Srgb, u8>::new(240, 52, 230, 128));
        let c = Rgba::<Srgb, u8>::from_str("#ffffffff");
        assert_eq!(c.unwrap(), Rgba::<Srgb, u8>::new(255, 255, 255, 255));
        let c = Rgba::<Srgb, u8>::from_str("#ffff");
        assert_eq!(c.unwrap(), Rgba::<Srgb, u8>::new(255, 255, 255, 255));
        let c = Rgba::<Srgb, u8>::from_str("#gggggggg");
        assert!(c.is_err());
        assert_eq!(
            format!("{}", c.err().unwrap()),
            "invalid digit found in string"
        );
        let c = Rgba::<Srgb, u8>::from_str("#fff");
        assert_eq!(
            format!("{}", c.err().unwrap()),
            "invalid hex code format, \
            please use format '#ffff', 'ffff', '#ffffffff' or 'ffffffff'."
        );
        let c = Rgba::<Srgb, u8>::from_str("#ffffff");
        assert_eq!(
            format!("{}", c.err().unwrap()),
            "invalid hex code format, \
            please use format '#ffff', 'ffff', '#ffffffff' or 'ffffffff'."
        );
    }

    #[test]
    fn check_min_max_components() {
        assert_eq!(Rgb::<Srgb, f32>::min_red(), 0.0);
        assert_eq!(Rgb::<Srgb, f32>::min_green(), 0.0);
        assert_eq!(Rgb::<Srgb, f32>::min_blue(), 0.0);
        assert_eq!(Rgb::<Srgb, f32>::max_red(), 1.0);
        assert_eq!(Rgb::<Srgb, f32>::max_green(), 1.0);
        assert_eq!(Rgb::<Srgb, f32>::max_blue(), 1.0);
    }

    struct_of_arrays_tests!(
        Rgb<Srgb>[red, green, blue] phantom: standard,
        Rgba::new(0.1f32, 0.2, 0.3, 0.4),
        Rgba::new(0.2, 0.3, 0.4, 0.5),
        Rgba::new(0.3, 0.4, 0.5, 0.6)
    );

    test_uniform_distribution! {
        Rgb<Srgb, f32> {
            red: (0.0, 1.0),
            green: (0.0, 1.0),
            blue: (0.0, 1.0)
        },
        min: Rgb::new(0.0f32, 0.0, 0.0),
        max: Rgb::new(1.0, 1.0, 1.0)
    }
}
