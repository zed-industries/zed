use core::{
    any::TypeId,
    convert::TryInto,
    fmt,
    marker::PhantomData,
    ops::{Add, Div},
};

use crate::{
    bool_mask::{HasBoolMask, LazySelect},
    cast::{ComponentOrder, Packed, UintCast},
    color_difference::Wcag21RelativeContrast,
    convert::FromColorUnclamped,
    encoding::{FromLinear, IntoLinear, Linear, Srgb},
    luma::LumaStandard,
    num::{Arithmetics, MinMax, PartialCmp, Real},
    stimulus::{FromStimulus, Stimulus, StimulusColor},
    white_point::D65,
    Alpha, IntoColor, Xyz, Yxy,
};

/// Luminance with an alpha component. See the [`Lumaa` implementation
/// in `Alpha`](crate::Alpha#Lumaa).
pub type Lumaa<S = Srgb, T = f32> = Alpha<Luma<S, T>, T>;

/// Luminance.
///
/// Luma is a purely gray scale color space, which is included more for
/// completeness than anything else, and represents how bright a color is
/// perceived to be. It's basically the `Y` component of [CIE
/// XYZ](crate::Xyz). The lack of any form of hue representation limits
/// the set of operations that can be performed on it.
#[derive(Debug, ArrayCast, FromColorUnclamped, WithAlpha)]
#[cfg_attr(feature = "serializing", derive(Serialize, Deserialize))]
#[palette(
    palette_internal,
    luma_standard = "S",
    component = "T",
    skip_derives(Xyz, Yxy, Luma)
)]
#[repr(C)]
#[doc(alias = "gray")]
#[doc(alias = "grey")]
pub struct Luma<S = Srgb, T = f32> {
    /// The lightness of the color. 0.0 is black and 1.0 is white.
    pub luma: T,

    /// The kind of RGB standard. sRGB is the default.
    #[cfg_attr(feature = "serializing", serde(skip))]
    #[palette(unsafe_zero_sized)]
    pub standard: PhantomData<S>,
}

impl<S, T> Luma<S, T> {
    /// Create a luminance color.
    pub const fn new(luma: T) -> Luma<S, T> {
        Luma {
            luma,
            standard: PhantomData,
        }
    }

    /// Convert into another component type.
    pub fn into_format<U>(self) -> Luma<S, U>
    where
        U: FromStimulus<T>,
    {
        Luma {
            luma: U::from_stimulus(self.luma),
            standard: PhantomData,
        }
    }

    /// Convert from another component type.
    pub fn from_format<U>(color: Luma<S, U>) -> Self
    where
        T: FromStimulus<U>,
    {
        color.into_format()
    }

    /// Convert to a `(luma,)` tuple.
    pub fn into_components(self) -> (T,) {
        (self.luma,)
    }

    /// Convert from a `(luma,)` tuple.
    pub fn from_components((luma,): (T,)) -> Self {
        Self::new(luma)
    }

    fn reinterpret_as<S2>(self) -> Luma<S2, T>
    where
        S: LumaStandard,
        S2: LumaStandard<WhitePoint = S::WhitePoint>,
    {
        Luma {
            luma: self.luma,
            standard: PhantomData,
        }
    }
}

impl<S, T> Luma<S, T>
where
    T: Stimulus,
{
    /// Return the `luma` value minimum.
    pub fn min_luma() -> T {
        T::zero()
    }

    /// Return the `luma` value maximum.
    pub fn max_luma() -> T {
        T::max_intensity()
    }
}

impl<S> Luma<S, u8> {
    /// Convert to a packed `u16` with with specifiable component order.
    ///
    /// ```
    /// use palette::{luma, SrgbLuma};
    ///
    /// let integer = SrgbLuma::new(96u8).into_u16::<luma::channels::La>();
    /// assert_eq!(0x60FF, integer);
    /// ```
    ///
    /// It's also possible to use `From` and `Into`, which defaults to the
    /// `0xAALL` component order:
    ///
    /// ```
    /// use palette::SrgbLuma;
    ///
    /// let integer = u16::from(SrgbLuma::new(96u8));
    /// assert_eq!(0xFF60, integer);
    /// ```
    ///
    /// See [Packed](crate::cast::Packed) for more details.
    #[inline]
    pub fn into_u16<O>(self) -> u16
    where
        O: ComponentOrder<Lumaa<S, u8>, u16>,
    {
        O::pack(Lumaa::from(self))
    }

    /// Convert from a packed `u16` with specifiable component order.
    ///
    /// ```
    /// use palette::{luma, SrgbLuma};
    ///
    /// let luma = SrgbLuma::from_u16::<luma::channels::La>(0x60FF);
    /// assert_eq!(SrgbLuma::new(96u8), luma);
    /// ```
    ///
    /// It's also possible to use `From` and `Into`, which defaults to the
    /// `0xAALL` component order:
    ///
    /// ```
    /// use palette::SrgbLuma;
    ///
    /// let luma = SrgbLuma::from(0x60u16);
    /// assert_eq!(SrgbLuma::new(96u8), luma);
    /// ```
    ///
    /// See [Packed](crate::cast::Packed) for more details.
    #[inline]
    pub fn from_u16<O>(color: u16) -> Self
    where
        O: ComponentOrder<Lumaa<S, u8>, u16>,
    {
        O::unpack(color).color
    }
}

impl<S, T> Luma<S, T>
where
    S: LumaStandard,
{
    /// Convert the color to linear luminance.
    ///
    /// Some transfer functions allow the component type to be converted at the
    /// same time. This is usually offered with increased performance, compared
    /// to using [`into_format`][Luma::into_format].
    ///
    /// ```
    /// use palette::{SrgbLuma, LinLuma};
    ///
    /// let linear: LinLuma<_, f32> = SrgbLuma::new(96u8).into_linear();
    /// ```
    ///
    /// See the transfer function types in the [`encoding`](crate::encoding)
    /// module for details and performance characteristics.
    pub fn into_linear<U>(self) -> Luma<Linear<S::WhitePoint>, U>
    where
        S::TransferFn: IntoLinear<U, T>,
    {
        Luma::new(S::TransferFn::into_linear(self.luma))
    }

    /// Convert linear luminance to non-linear luminance.
    ///
    /// Some transfer functions allow the component type to be converted at the
    /// same time. This is usually offered with increased performance, compared
    /// to using [`into_format`][Luma::into_format].
    ///
    /// ```
    /// use palette::{SrgbLuma, LinLuma};
    ///
    /// let encoded = SrgbLuma::<u8>::from_linear(LinLuma::new(0.95f32));
    /// ```
    ///
    /// See the transfer function types in the [`encoding`](crate::encoding)
    /// module for details and performance characteristics.
    pub fn from_linear<U>(color: Luma<Linear<S::WhitePoint>, U>) -> Luma<S, T>
    where
        S::TransferFn: FromLinear<U, T>,
    {
        Luma::new(S::TransferFn::from_linear(color.luma))
    }
}

impl<Wp, T> Luma<Linear<Wp>, T> {
    /// Convert a linear color to a different encoding.
    ///
    /// Some transfer functions allow the component type to be converted at the
    /// same time. This is usually offered with increased performance, compared
    /// to using [`into_format`][Luma::into_format].
    ///
    /// ```
    /// use palette::{SrgbLuma, LinLuma};
    ///
    /// let encoded: SrgbLuma<u8> = LinLuma::new(0.95f32).into_encoding();
    /// ```
    ///
    /// See the transfer function types in the [`encoding`](crate::encoding)
    /// module for details and performance characteristics.
    pub fn into_encoding<U, St>(self) -> Luma<St, U>
    where
        St: LumaStandard<WhitePoint = Wp>,
        St::TransferFn: FromLinear<T, U>,
    {
        Luma::<St, U>::from_linear(self)
    }

    /// Convert from linear luminance from a different encoding.
    ///
    /// Some transfer functions allow the component type to be converted at the
    /// same time. This is usually offered with increased performance, compared
    /// to using [`into_format`][Luma::into_format].
    ///
    /// ```
    /// use palette::{SrgbLuma, LinLuma};
    ///
    /// let linear = LinLuma::<_, f32>::from_encoding(SrgbLuma::new(96u8));
    /// ```
    ///
    /// See the transfer function types in the [`encoding`](crate::encoding)
    /// module for details and performance characteristics.
    pub fn from_encoding<U, St>(color: Luma<St, U>) -> Self
    where
        St: LumaStandard<WhitePoint = Wp>,
        St::TransferFn: IntoLinear<T, U>,
    {
        color.into_linear()
    }
}

// Safety:
//
// Luma is a transparent wrapper around its component, which fulfills the
// requirements of UintCast.
unsafe impl<S> UintCast for Luma<S, u8> {
    type Uint = u8;
}

// Safety:
//
// Luma is a transparent wrapper around its component, which fulfills the
// requirements of UintCast.
unsafe impl<S> UintCast for Luma<S, u16> {
    type Uint = u16;
}

// Safety:
//
// Luma is a transparent wrapper around its component, which fulfills the
// requirements of UintCast.
unsafe impl<S> UintCast for Luma<S, u32> {
    type Uint = u32;
}

// Safety:
//
// Luma is a transparent wrapper around its component, which fulfills the
// requirements of UintCast.
unsafe impl<S> UintCast for Luma<S, u64> {
    type Uint = u64;
}

// Safety:
//
// Luma is a transparent wrapper around its component, which fulfills the
// requirements of UintCast.
unsafe impl<S> UintCast for Luma<S, u128> {
    type Uint = u128;
}

///<span id="Lumaa"></span>[`Lumaa`](crate::luma::Lumaa) implementations.
impl<S, T, A> Alpha<Luma<S, T>, A> {
    /// Create a luminance color with transparency.
    pub const fn new(luma: T, alpha: A) -> Self {
        Alpha {
            color: Luma::new(luma),
            alpha,
        }
    }

    /// Convert into another component type.
    pub fn into_format<U, B>(self) -> Alpha<Luma<S, U>, B>
    where
        U: FromStimulus<T>,
        B: FromStimulus<A>,
    {
        Alpha {
            color: self.color.into_format(),
            alpha: B::from_stimulus(self.alpha),
        }
    }

    /// Convert from another component type.
    pub fn from_format<U, B>(color: Alpha<Luma<S, U>, B>) -> Self
    where
        T: FromStimulus<U>,
        A: FromStimulus<B>,
    {
        color.into_format()
    }

    /// Convert to a `(luma, alpha)` tuple.
    pub fn into_components(self) -> (T, A) {
        (self.color.luma, self.alpha)
    }

    /// Convert from a `(luma, alpha)` tuple.
    pub fn from_components((luma, alpha): (T, A)) -> Self {
        Self::new(luma, alpha)
    }
}

impl<S> Lumaa<S, u8> {
    /// Convert to a packed `u16` with with a specific component order.
    ///
    /// ```
    /// use palette::{luma, SrgbLumaa};
    ///
    /// let integer = SrgbLumaa::new(96u8, 255).into_u16::<luma::channels::Al>();
    /// assert_eq!(0xFF60, integer);
    /// ```
    ///
    /// It's also possible to use `From` and `Into`, which defaults to the
    /// `0xLLAA` component order:
    ///
    /// ```
    /// use palette::SrgbLumaa;
    ///
    /// let integer = u16::from(SrgbLumaa::new(96u8, 255));
    /// assert_eq!(0x60FF, integer);
    /// ```
    ///
    /// See [Packed](crate::cast::Packed) for more details.
    #[inline]
    pub fn into_u16<O>(self) -> u16
    where
        O: ComponentOrder<Lumaa<S, u8>, u16>,
    {
        O::pack(self)
    }

    /// Convert from a packed `u16` with a specific component order.
    ///
    /// ```
    /// use palette::{luma, SrgbLumaa};
    ///
    /// let luma = SrgbLumaa::from_u16::<luma::channels::Al>(0xFF60);
    /// assert_eq!(SrgbLumaa::new(96u8, 255), luma);
    /// ```
    ///
    /// It's also possible to use `From` and `Into`, which defaults to the
    /// `0xLLAA` component order:
    ///
    /// ```
    /// use palette::SrgbLumaa;
    ///
    /// let luma = SrgbLumaa::from(0x60FF);
    /// assert_eq!(SrgbLumaa::new(96u8, 255), luma);
    /// ```
    ///
    /// See [Packed](crate::cast::Packed) for more details.
    #[inline]
    pub fn from_u16<O>(color: u16) -> Self
    where
        O: ComponentOrder<Lumaa<S, u8>, u16>,
    {
        O::unpack(color)
    }
}

impl<S, T, A> Alpha<Luma<S, T>, A>
where
    S: LumaStandard,
{
    /// Convert the color to linear luminance with transparency.
    ///
    /// Some transfer functions allow the component type to be converted at the
    /// same time. This is usually offered with increased performance, compared
    /// to using [`into_format`][Luma::into_format].
    ///
    /// ```
    /// use palette::{SrgbLumaa, LinLumaa};
    ///
    /// let linear: LinLumaa<_, f32> = SrgbLumaa::new(96u8, 38).into_linear();
    /// ```
    ///
    /// See the transfer function types in the [`encoding`](crate::encoding)
    /// module for details and performance characteristics.
    pub fn into_linear<U, B>(self) -> Alpha<Luma<Linear<S::WhitePoint>, U>, B>
    where
        S::TransferFn: IntoLinear<U, T>,
        B: FromStimulus<A>,
    {
        Alpha {
            color: self.color.into_linear(),
            alpha: B::from_stimulus(self.alpha),
        }
    }

    /// Convert linear luminance to non-linear luminance with transparency.
    ///
    /// Some transfer functions allow the component type to be converted at the
    /// same time. This is usually offered with increased performance, compared
    /// to using [`into_format`][Luma::into_format].
    ///
    /// ```
    /// use palette::{SrgbLumaa, LinLumaa};
    ///
    /// let encoded = SrgbLumaa::<u8>::from_linear(LinLumaa::new(0.95f32, 0.75));
    /// ```
    ///
    /// See the transfer function types in the [`encoding`](crate::encoding)
    /// module for details and performance characteristics.
    pub fn from_linear<U, B>(color: Alpha<Luma<Linear<S::WhitePoint>, U>, B>) -> Self
    where
        S::TransferFn: FromLinear<U, T>,
        A: FromStimulus<B>,
    {
        Alpha {
            color: Luma::from_linear(color.color),
            alpha: A::from_stimulus(color.alpha),
        }
    }
}

impl<Wp, T, A> Alpha<Luma<Linear<Wp>, T>, A> {
    /// Convert a linear color to a different encoding with transparency.
    ///
    /// Some transfer functions allow the component type to be converted at the
    /// same time. This is usually offered with increased performance, compared
    /// to using [`into_format`][Luma::into_format].
    ///
    /// ```
    /// use palette::{SrgbLumaa, LinLumaa};
    ///
    /// let encoded: SrgbLumaa<u8> = LinLumaa::new(0.95f32, 0.75).into_encoding();
    /// ```
    ///
    /// See the transfer function types in the [`encoding`](crate::encoding)
    /// module for details and performance characteristics.
    pub fn into_encoding<U, B, St>(self) -> Alpha<Luma<St, U>, B>
    where
        St: LumaStandard<WhitePoint = Wp>,
        St::TransferFn: FromLinear<T, U>,
        B: FromStimulus<A>,
    {
        Alpha::<Luma<St, U>, B>::from_linear(self)
    }

    /// Convert to linear luminance from a different encoding with transparency.
    ///
    /// Some transfer functions allow the component type to be converted at the
    /// same time. This is usually offered with increased performance, compared
    /// to using [`into_format`][Luma::into_format].
    ///
    /// ```
    /// use palette::{SrgbLumaa, LinLumaa};
    ///
    /// let linear = LinLumaa::<_, f32>::from_encoding(SrgbLumaa::new(96u8, 38));
    /// ```
    ///
    /// See the transfer function types in the [`encoding`](crate::encoding)
    /// module for details and performance characteristics.
    pub fn from_encoding<U, B, St>(color: Alpha<Luma<St, U>, B>) -> Self
    where
        St: LumaStandard<WhitePoint = Wp>,
        St::TransferFn: IntoLinear<T, U>,
        A: FromStimulus<B>,
    {
        color.into_linear()
    }
}

impl_reference_component_methods!(Luma<S>, [luma], standard);
impl_struct_of_arrays_methods!(Luma<S>, [luma], standard);

impl<S1, S2, T> FromColorUnclamped<Luma<S2, T>> for Luma<S1, T>
where
    S1: LumaStandard + 'static,
    S2: LumaStandard<WhitePoint = S1::WhitePoint> + 'static,
    S1::TransferFn: FromLinear<T, T>,
    S2::TransferFn: IntoLinear<T, T>,
{
    fn from_color_unclamped(color: Luma<S2, T>) -> Self {
        if TypeId::of::<S1>() == TypeId::of::<S2>() {
            color.reinterpret_as()
        } else {
            Self::from_linear(color.into_linear().reinterpret_as())
        }
    }
}

impl<S, T> FromColorUnclamped<Xyz<S::WhitePoint, T>> for Luma<S, T>
where
    S: LumaStandard,
    S::TransferFn: FromLinear<T, T>,
{
    fn from_color_unclamped(color: Xyz<S::WhitePoint, T>) -> Self {
        Self::from_linear(Luma {
            luma: color.y,
            standard: PhantomData,
        })
    }
}

impl<S, T> FromColorUnclamped<Yxy<S::WhitePoint, T>> for Luma<S, T>
where
    S: LumaStandard,
    S::TransferFn: FromLinear<T, T>,
{
    fn from_color_unclamped(color: Yxy<S::WhitePoint, T>) -> Self {
        Self::from_linear(Luma {
            luma: color.luma,
            standard: PhantomData,
        })
    }
}

impl_tuple_conversion!(Luma<S> as (T));

impl_is_within_bounds! {
    Luma<S> {
        luma => [Self::min_luma(), Self::max_luma()]
    }
    where T: Stimulus
}
impl_clamp! {
    Luma<S> {
        luma => [Self::min_luma(), Self::max_luma()]
    }
    other {standard}
    where T: Stimulus
}

impl_mix!(Luma<S>);
impl_lighten!(Luma<S> increase {luma => [Self::min_luma(), Self::max_luma()]} other {} phantom: standard where T: Stimulus);
impl_premultiply!(Luma<S> {luma} phantom: standard);
impl_euclidean_distance!(Luma<S> {luma});

impl<S, T> StimulusColor for Luma<S, T> where T: Stimulus {}

impl<S, T> HasBoolMask for Luma<S, T>
where
    T: HasBoolMask,
{
    type Mask = T::Mask;
}

impl<S, T> Default for Luma<S, T>
where
    T: Stimulus,
{
    fn default() -> Luma<S, T> {
        Luma::new(Self::min_luma())
    }
}

impl_color_add!(Luma<S>, [luma], standard);
impl_color_sub!(Luma<S>, [luma], standard);
impl_color_mul!(Luma<S>, [luma], standard);
impl_color_div!(Luma<S>, [luma], standard);

impl_array_casts!(Luma<S, T>, [T; 1]);

impl<S, T> AsRef<T> for Luma<S, T> {
    #[inline]
    fn as_ref(&self) -> &T {
        &self.luma
    }
}

impl<S, T> AsMut<T> for Luma<S, T> {
    #[inline]
    fn as_mut(&mut self) -> &mut T {
        &mut self.luma
    }
}

impl<S, T> From<T> for Luma<S, T> {
    #[inline]
    fn from(luma: T) -> Self {
        Self::new(luma)
    }
}

macro_rules! impl_luma_cast_other {
    ($($other: ty),+) => {
        $(
            impl<'a, S> From<&'a $other> for &'a Luma<S, $other>
            where
                $other: AsRef<Luma<S, $other>>,
            {
                #[inline]
                fn from(luma: &'a $other) -> Self {
                    luma.as_ref()
                }
            }

            impl<'a, S> From<&'a mut $other> for &'a mut Luma<S, $other>
            where
                $other: AsMut<Luma<S, $other>>,
            {
                #[inline]
                fn from(luma: &'a mut $other) -> Self {
                    luma.as_mut()
                }
            }

            impl<S> AsRef<Luma<S, $other>> for $other {
                #[inline]
                fn as_ref(&self) -> &Luma<S, $other> {
                    core::slice::from_ref(self).try_into().unwrap()
                }
            }

            impl<S> AsMut<Luma<S, $other>> for $other {
                #[inline]
                fn as_mut(&mut self) -> &mut Luma<S, $other> {
                    core::slice::from_mut(self).try_into().unwrap()
                }
            }

            impl<S> From<Luma<S, $other>> for $other {
                #[inline]
                fn from(color: Luma<S, $other>) -> Self {
                    color.luma
                }
            }

            impl<'a, S> From<&'a Luma<S, $other>> for &'a $other {
                #[inline]
                fn from(color: &'a Luma<S, $other>) -> Self {
                    color.as_ref()
                }
            }

            impl<'a, S> From<&'a mut Luma<S, $other>> for &'a mut $other {
                #[inline]
                fn from(color: &'a mut Luma<S, $other>) -> Self {
                    color.as_mut()
                }
            }
        )+
    };
}
impl_luma_cast_other!(u8, u16, u32, u64, u128, f32, f64);

impl<S, T, P, O> From<Luma<S, T>> for Packed<O, P>
where
    O: ComponentOrder<Lumaa<S, T>, P>,
    Lumaa<S, T>: From<Luma<S, T>>,
{
    #[inline]
    fn from(color: Luma<S, T>) -> Self {
        Self::from(Lumaa::from(color))
    }
}

impl<S, T, O, P> From<Lumaa<S, T>> for Packed<O, P>
where
    O: ComponentOrder<Lumaa<S, T>, P>,
{
    #[inline]
    fn from(color: Lumaa<S, T>) -> Self {
        Packed::pack(color)
    }
}

impl<S, O, P> From<Packed<O, P>> for Luma<S, u8>
where
    O: ComponentOrder<Lumaa<S, u8>, P>,
{
    #[inline]
    fn from(packed: Packed<O, P>) -> Self {
        Lumaa::from(packed).color
    }
}

impl<S, T, O, P> From<Packed<O, P>> for Lumaa<S, T>
where
    O: ComponentOrder<Lumaa<S, T>, P>,
{
    #[inline]
    fn from(packed: Packed<O, P>) -> Self {
        packed.unpack()
    }
}

impl<S> From<u16> for Luma<S, u8> {
    #[inline]
    fn from(color: u16) -> Self {
        Self::from_u16::<super::channels::Al>(color)
    }
}

impl<S> From<u16> for Lumaa<S, u8> {
    #[inline]
    fn from(color: u16) -> Self {
        Self::from_u16::<super::channels::La>(color)
    }
}

impl<S> From<Luma<S, u8>> for u16 {
    #[inline]
    fn from(color: Luma<S, u8>) -> Self {
        Luma::into_u16::<super::channels::Al>(color)
    }
}

impl<S> From<Lumaa<S, u8>> for u16 {
    #[inline]
    fn from(color: Lumaa<S, u8>) -> Self {
        Lumaa::into_u16::<super::channels::La>(color)
    }
}

impl_simd_array_conversion!(Luma<S>, [luma], standard);
impl_struct_of_array_traits!(Luma<S>, [luma], standard);

impl_copy_clone!(Luma<S>, [luma], standard);
impl_eq!(Luma<S>, [luma]);

impl<S, T> fmt::LowerHex for Luma<S, T>
where
    T: fmt::LowerHex,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let size = f.width().unwrap_or(::core::mem::size_of::<T>() * 2);
        write!(f, "{:0width$x}", self.luma, width = size)
    }
}

impl<S, T> fmt::UpperHex for Luma<S, T>
where
    T: fmt::UpperHex,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let size = f.width().unwrap_or(::core::mem::size_of::<T>() * 2);
        write!(f, "{:0width$X}", self.luma, width = size)
    }
}

#[allow(deprecated)]
impl<S, T> crate::RelativeContrast for Luma<S, T>
where
    T: Real + Arithmetics + PartialCmp,
    T::Mask: LazySelect<T>,
    S: LumaStandard,
    S::TransferFn: IntoLinear<T, T>,
{
    type Scalar = T;

    #[inline]
    fn get_contrast_ratio(self, other: Self) -> T {
        let luma1 = self.into_linear();
        let luma2 = other.into_linear();

        crate::contrast_ratio(luma1.luma, luma2.luma)
    }
}

impl<S, T> Wcag21RelativeContrast for Luma<S, T>
where
    Self: IntoColor<Luma<Linear<D65>, T>>,
    S: LumaStandard<WhitePoint = D65>,
    T: Real + Add<T, Output = T> + Div<T, Output = T> + PartialCmp + MinMax,
{
    type Scalar = T;

    fn relative_luminance(self) -> Luma<Linear<D65>, Self::Scalar> {
        self.into_color()
    }
}

impl_rand_traits_cartesian!(UniformLuma, Luma<S> {luma} phantom: standard: PhantomData<S>);

#[cfg(feature = "bytemuck")]
unsafe impl<S, T> bytemuck::Zeroable for Luma<S, T> where T: bytemuck::Zeroable {}

#[cfg(feature = "bytemuck")]
unsafe impl<S: 'static, T> bytemuck::Pod for Luma<S, T> where T: bytemuck::Pod {}

#[cfg(test)]
mod test {
    use crate::encoding::Srgb;
    use crate::Luma;

    test_convert_into_from_xyz!(Luma);

    #[test]
    fn ranges() {
        assert_ranges! {
            Luma<Srgb, f64>;
            clamped {
                luma: 0.0 => 1.0
            }
            clamped_min {}
            unclamped {}
        }
    }

    raw_pixel_conversion_tests!(Luma<Srgb>: luma);

    #[test]
    fn lower_hex() {
        assert_eq!(format!("{:x}", Luma::<Srgb, u8>::new(161)), "a1");
    }

    #[test]
    fn lower_hex_small_numbers() {
        assert_eq!(format!("{:x}", Luma::<Srgb, u8>::new(1)), "01");
        assert_eq!(format!("{:x}", Luma::<Srgb, u16>::new(1)), "0001");
        assert_eq!(format!("{:x}", Luma::<Srgb, u32>::new(1)), "00000001");
        assert_eq!(
            format!("{:x}", Luma::<Srgb, u64>::new(1)),
            "0000000000000001"
        );
    }

    #[test]
    fn lower_hex_custom_width() {
        assert_eq!(format!("{:03x}", Luma::<Srgb, u8>::new(1)), "001");
        assert_eq!(format!("{:03x}", Luma::<Srgb, u16>::new(1)), "001");
        assert_eq!(format!("{:03x}", Luma::<Srgb, u32>::new(1)), "001");
        assert_eq!(format!("{:03x}", Luma::<Srgb, u64>::new(1)), "001");
    }

    #[test]
    fn upper_hex() {
        assert_eq!(format!("{:X}", Luma::<Srgb, u8>::new(161)), "A1");
    }

    #[test]
    fn upper_hex_small_numbers() {
        assert_eq!(format!("{:X}", Luma::<Srgb, u8>::new(1)), "01");
        assert_eq!(format!("{:X}", Luma::<Srgb, u16>::new(1)), "0001");
        assert_eq!(format!("{:X}", Luma::<Srgb, u32>::new(1)), "00000001");
        assert_eq!(
            format!("{:X}", Luma::<Srgb, u64>::new(1)),
            "0000000000000001"
        );
    }

    #[test]
    fn upper_hex_custom_width() {
        assert_eq!(format!("{:03X}", Luma::<Srgb, u8>::new(1)), "001");
        assert_eq!(format!("{:03X}", Luma::<Srgb, u16>::new(1)), "001");
        assert_eq!(format!("{:03X}", Luma::<Srgb, u32>::new(1)), "001");
        assert_eq!(format!("{:03X}", Luma::<Srgb, u64>::new(1)), "001");
    }

    #[test]
    fn check_min_max_components() {
        assert_eq!(Luma::<Srgb, f32>::min_luma(), 0.0);
        assert_eq!(Luma::<Srgb, f32>::max_luma(), 1.0);
    }

    struct_of_arrays_tests!(
        Luma<Srgb>[luma] phantom: standard,
        super::Lumaa::new(0.1f32, 0.4),
        super::Lumaa::new(0.2, 0.5),
        super::Lumaa::new(0.3, 0.6)
    );

    #[cfg(feature = "serializing")]
    #[test]
    fn serialize() {
        let serialized = ::serde_json::to_string(&Luma::<Srgb>::new(0.3)).unwrap();

        assert_eq!(serialized, r#"{"luma":0.3}"#);
    }

    #[cfg(feature = "serializing")]
    #[test]
    fn deserialize() {
        let deserialized: Luma<Srgb> = ::serde_json::from_str(r#"{"luma":0.3}"#).unwrap();

        assert_eq!(deserialized, Luma::<Srgb>::new(0.3));
    }

    test_uniform_distribution! {
        Luma<Srgb, f32> {
            luma: (0.0, 1.0)
        },
        min: Luma::new(0.0f32),
        max: Luma::new(1.0)
    }
}
