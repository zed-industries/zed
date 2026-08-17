//! Types for the CIE L\*C\*h° color space.

use core::{
    marker::PhantomData,
    ops::{BitAnd, BitOr},
};

use crate::{
    angle::RealAngle,
    bool_mask::{HasBoolMask, LazySelect},
    color_difference::{get_ciede2000_difference, Ciede2000, DeltaE, ImprovedDeltaE, LabColorDiff},
    convert::{FromColorUnclamped, IntoColorUnclamped},
    hues::LabHueIter,
    num::{Abs, Arithmetics, Exp, Hypot, One, PartialCmp, Powi, Real, Sqrt, Trigonometry, Zero},
    white_point::D65,
    Alpha, FromColor, GetHue, Lab, LabHue, Xyz,
};

/// CIE L\*C\*h° with an alpha component. See the [`Lcha` implementation in
/// `Alpha`](crate::Alpha#Lcha).
pub type Lcha<Wp = D65, T = f32> = Alpha<Lch<Wp, T>, T>;

/// CIE L\*C\*h°, a polar version of [CIE L\*a\*b\*](crate::Lab).
///
/// L\*C\*h° shares its range and perceptual uniformity with L\*a\*b\*, but
/// it's a cylindrical color space, like [HSL](crate::Hsl) and
/// [HSV](crate::Hsv). This gives it the same ability to directly change
/// the hue and colorfulness of a color, while preserving other visual aspects.
#[derive(Debug, ArrayCast, FromColorUnclamped, WithAlpha)]
#[cfg_attr(feature = "serializing", derive(Serialize, Deserialize))]
#[palette(
    palette_internal,
    white_point = "Wp",
    component = "T",
    skip_derives(Lab, Lch)
)]
#[repr(C)]
pub struct Lch<Wp = D65, T = f32> {
    /// L\* is the lightness of the color. 0.0 gives absolute black and 100.0
    /// gives the brightest white.
    pub l: T,

    /// C\* is the colorfulness of the color. It's similar to saturation. 0.0
    /// gives gray scale colors, and numbers around 128-181 gives fully
    /// saturated colors. The upper limit of 128 should
    /// include the whole L\*a\*b\* space and some more.
    pub chroma: T,

    /// The hue of the color, in degrees. Decides if it's red, blue, purple,
    /// etc.
    #[palette(unsafe_same_layout_as = "T")]
    pub hue: LabHue<T>,

    /// The white point associated with the color's illuminant and observer.
    /// D65 for 2 degree observer is used by default.
    #[cfg_attr(feature = "serializing", serde(skip))]
    #[palette(unsafe_zero_sized)]
    pub white_point: PhantomData<Wp>,
}

impl<Wp, T> Lch<Wp, T> {
    /// Create a CIE L\*C\*h° color.
    pub fn new<H: Into<LabHue<T>>>(l: T, chroma: T, hue: H) -> Self {
        Self::new_const(l, chroma, hue.into())
    }

    /// Create a CIE L\*C\*h° color. This is the same as `Lch::new` without the
    /// generic hue type. It's temporary until `const fn` supports traits.
    pub const fn new_const(l: T, chroma: T, hue: LabHue<T>) -> Self {
        Lch {
            l,
            chroma,
            hue,
            white_point: PhantomData,
        }
    }

    /// Convert to a `(L\*, C\*, h°)` tuple.
    pub fn into_components(self) -> (T, T, LabHue<T>) {
        (self.l, self.chroma, self.hue)
    }

    /// Convert from a `(L\*, C\*, h°)` tuple.
    pub fn from_components<H: Into<LabHue<T>>>((l, chroma, hue): (T, T, H)) -> Self {
        Self::new(l, chroma, hue)
    }
}

impl<Wp, T> Lch<Wp, T>
where
    T: Zero + Real,
{
    /// Return the `l` value minimum.
    pub fn min_l() -> T {
        T::zero()
    }

    /// Return the `l` value maximum.
    pub fn max_l() -> T {
        T::from_f64(100.0)
    }

    /// Return the `chroma` value minimum.
    pub fn min_chroma() -> T {
        T::zero()
    }

    /// Return the `chroma` value maximum. This value does not cover the entire
    /// color space, but covers enough to be practical for downsampling to
    /// smaller color spaces like sRGB.
    pub fn max_chroma() -> T {
        T::from_f64(128.0)
    }

    /// Return the `chroma` extended maximum value. This value covers the entire
    /// color space and is included for completeness, but the additional range
    /// should be unnecessary for most use cases.
    pub fn max_extended_chroma() -> T {
        T::from_f64(crate::num::Sqrt::sqrt(128.0f64 * 128.0 + 128.0 * 128.0))
    }
}

///<span id="Lcha"></span>[`Lcha`](crate::Lcha) implementations.
impl<Wp, T, A> Alpha<Lch<Wp, T>, A> {
    /// Create a CIE L\*C\*h° color with transparency.
    pub fn new<H: Into<LabHue<T>>>(l: T, chroma: T, hue: H, alpha: A) -> Self {
        Self::new_const(l, chroma, hue.into(), alpha)
    }

    /// Create a CIE L\*C\*h° color with transparency. This is the same as
    /// `Lcha::new` without the generic hue type. It's temporary until `const
    /// fn` supports traits.
    pub const fn new_const(l: T, chroma: T, hue: LabHue<T>, alpha: A) -> Self {
        Alpha {
            color: Lch::new_const(l, chroma, hue),
            alpha,
        }
    }

    /// Convert to a `(L\*, C\*, h°, alpha)` tuple.
    pub fn into_components(self) -> (T, T, LabHue<T>, A) {
        (self.color.l, self.color.chroma, self.color.hue, self.alpha)
    }

    /// Convert from a `(L\*, C\*, h°, alpha)` tuple.
    pub fn from_components<H: Into<LabHue<T>>>((l, chroma, hue, alpha): (T, T, H, A)) -> Self {
        Self::new(l, chroma, hue, alpha)
    }
}

impl_reference_component_methods_hue!(Lch<Wp>, [l, chroma], white_point);
impl_struct_of_arrays_methods_hue!(Lch<Wp>, [l, chroma], white_point);

impl<Wp, T> FromColorUnclamped<Lch<Wp, T>> for Lch<Wp, T> {
    fn from_color_unclamped(color: Lch<Wp, T>) -> Self {
        color
    }
}

impl<Wp, T> FromColorUnclamped<Lab<Wp, T>> for Lch<Wp, T>
where
    T: Zero + Hypot,
    Lab<Wp, T>: GetHue<Hue = LabHue<T>>,
{
    fn from_color_unclamped(color: Lab<Wp, T>) -> Self {
        Lch {
            hue: color.get_hue(),
            l: color.l,
            chroma: color.a.hypot(color.b),
            white_point: PhantomData,
        }
    }
}

impl_tuple_conversion_hue!(Lch<Wp> as (T, T, H), LabHue);

impl_is_within_bounds! {
    Lch<Wp> {
        l => [Self::min_l(), Self::max_l()],
        chroma => [Self::min_chroma(), None]
    }
    where T: Real + Zero
}
impl_clamp! {
    Lch<Wp> {
        l => [Self::min_l(), Self::max_l()],
        chroma => [Self::min_chroma()]
    }
    other {hue, white_point}
    where T: Real + Zero
}

impl_mix_hue!(Lch<Wp> {l, chroma} phantom: white_point);
impl_lighten!(Lch<Wp> increase {l => [Self::min_l(), Self::max_l()]} other {hue, chroma} phantom: white_point);
impl_saturate!(Lch<Wp> increase {chroma => [Self::min_chroma(), Self::max_chroma()]} other {hue, l} phantom: white_point);
impl_hue_ops!(Lch<Wp>, LabHue);

impl<Wp, T> DeltaE for Lch<Wp, T>
where
    Lab<Wp, T>: FromColorUnclamped<Self> + DeltaE<Scalar = T>,
{
    type Scalar = T;

    #[inline]
    fn delta_e(self, other: Self) -> Self::Scalar {
        // The definitions of delta E for Lch and Lab are equivalent. Converting
        // to Lab is the fastest way, so far.
        Lab::from_color_unclamped(self).delta_e(other.into_color_unclamped())
    }
}

impl<Wp, T> ImprovedDeltaE for Lch<Wp, T>
where
    Lab<Wp, T>: FromColorUnclamped<Self> + ImprovedDeltaE<Scalar = T>,
{
    #[inline]
    fn improved_delta_e(self, other: Self) -> Self::Scalar {
        // The definitions of delta E for Lch and Lab are equivalent.
        Lab::from_color_unclamped(self).improved_delta_e(other.into_color_unclamped())
    }
}

/// CIEDE2000 distance metric for color difference.
#[allow(deprecated)]
impl<Wp, T> crate::ColorDifference for Lch<Wp, T>
where
    T: Real
        + RealAngle
        + One
        + Zero
        + Trigonometry
        + Abs
        + Sqrt
        + Powi
        + Exp
        + Arithmetics
        + PartialCmp
        + Clone,
    T::Mask: LazySelect<T> + BitAnd<Output = T::Mask> + BitOr<Output = T::Mask>,
    Self: Into<LabColorDiff<T>>,
{
    type Scalar = T;

    #[inline]
    fn get_color_difference(self, other: Lch<Wp, T>) -> Self::Scalar {
        get_ciede2000_difference(self.into(), other.into())
    }
}

impl<Wp, T> Ciede2000 for Lch<Wp, T>
where
    T: Real
        + RealAngle
        + One
        + Zero
        + Powi
        + Exp
        + Trigonometry
        + Abs
        + Sqrt
        + Arithmetics
        + PartialCmp
        + Clone,
    T::Mask: LazySelect<T> + BitAnd<Output = T::Mask> + BitOr<Output = T::Mask>,
    Self: IntoColorUnclamped<Lab<Wp, T>>,
{
    type Scalar = T;

    #[inline]
    fn difference(self, other: Self) -> Self::Scalar {
        get_ciede2000_difference(self.into(), other.into())
    }
}

impl<Wp, T> HasBoolMask for Lch<Wp, T>
where
    T: HasBoolMask,
{
    type Mask = T::Mask;
}

impl<Wp, T> Default for Lch<Wp, T>
where
    T: Zero + Real,
    LabHue<T>: Default,
{
    fn default() -> Lch<Wp, T> {
        Lch::new(Self::min_l(), Self::min_chroma(), LabHue::default())
    }
}

impl_color_add!(Lch<Wp>, [l, chroma, hue], white_point);
impl_color_sub!(Lch<Wp>, [l, chroma, hue], white_point);

impl_array_casts!(Lch<Wp, T>, [T; 3]);
impl_simd_array_conversion_hue!(Lch<Wp>, [l, chroma], white_point);
impl_struct_of_array_traits_hue!(Lch<Wp>, LabHueIter, [l, chroma], white_point);

impl_eq_hue!(Lch<Wp>, LabHue, [l, chroma, hue]);
impl_copy_clone!(Lch<Wp>, [l, chroma, hue], white_point);

#[allow(deprecated)]
impl<Wp, T> crate::RelativeContrast for Lch<Wp, T>
where
    T: Real + Arithmetics + PartialCmp,
    T::Mask: LazySelect<T>,
    Xyz<Wp, T>: FromColor<Self>,
{
    type Scalar = T;

    #[inline]
    fn get_contrast_ratio(self, other: Self) -> T {
        let xyz1 = Xyz::from_color(self);
        let xyz2 = Xyz::from_color(other);

        crate::contrast_ratio(xyz1.y, xyz2.y)
    }
}

impl_rand_traits_cylinder!(
    UniformLch,
    Lch<Wp> {
        hue: UniformLabHue => LabHue,
        height: l => [|l: T| l * Lch::<Wp, T>::max_l()],
        radius: chroma => [|chroma| chroma *  Lch::<Wp, T>::max_chroma()]
    }
    phantom: white_point: PhantomData<Wp>
    where T: Real + Zero + core::ops::Mul<Output = T>,
);

#[cfg(feature = "bytemuck")]
unsafe impl<Wp, T> bytemuck::Zeroable for Lch<Wp, T> where T: bytemuck::Zeroable {}

#[cfg(feature = "bytemuck")]
unsafe impl<Wp: 'static, T> bytemuck::Pod for Lch<Wp, T> where T: bytemuck::Pod {}

#[cfg(test)]
mod test {
    use crate::{white_point::D65, Lch};

    #[cfg(all(feature = "alloc", feature = "approx"))]
    use crate::{
        color_difference::{DeltaE, ImprovedDeltaE},
        convert::IntoColorUnclamped,
        Lab,
    };

    test_convert_into_from_xyz!(Lch);

    #[test]
    fn ranges() {
        assert_ranges! {
            Lch<D65, f64>;
            clamped {
                l: 0.0 => 100.0
            }
            clamped_min {
                chroma: 0.0 => 200.0
            }
            unclamped {
                hue: -360.0 => 360.0
            }
        }
    }

    raw_pixel_conversion_tests!(Lch<D65>: l, chroma, hue);
    raw_pixel_conversion_fail_tests!(Lch<D65>: l, chroma, hue);

    #[test]
    fn check_min_max_components() {
        assert_eq!(Lch::<D65, f64>::min_l(), 0.0);
        assert_eq!(Lch::<D65, f64>::max_l(), 100.0);
        assert_eq!(Lch::<D65, f64>::min_chroma(), 0.0);
        assert_eq!(Lch::<D65, f64>::max_chroma(), 128.0);

        #[cfg(feature = "approx")]
        assert_relative_eq!(Lch::<D65, f64>::max_extended_chroma(), 181.01933598375618);
    }

    #[cfg(feature = "approx")]
    #[test]
    fn delta_e_large_hue_diff() {
        use crate::color_difference::DeltaE;

        let lhs1 = Lch::<D65, f64>::new(50.0, 64.0, -730.0);
        let rhs1 = Lch::new(50.0, 64.0, 730.0);

        let lhs2 = Lch::<D65, f64>::new(50.0, 64.0, -10.0);
        let rhs2 = Lch::new(50.0, 64.0, 10.0);

        assert_relative_eq!(
            lhs1.delta_e(rhs1),
            lhs2.delta_e(rhs2),
            epsilon = 0.0000000000001
        );
    }

    // Lab and Lch have the same delta E.
    #[cfg(all(feature = "alloc", feature = "approx"))]
    #[test]
    fn lab_delta_e_equality() {
        let mut lab_colors: Vec<Lab<D65, f64>> = Vec::new();

        for l_step in 0i8..5 {
            for a_step in -2i8..3 {
                for b_step in -2i8..3 {
                    lab_colors.push(Lab::new(
                        l_step as f64 * 25.0,
                        a_step as f64 * 60.0,
                        b_step as f64 * 60.0,
                    ))
                }
            }
        }

        let lch_colors: Vec<Lch<_, _>> = lab_colors.clone().into_color_unclamped();

        for (&lhs_lab, &lhs_lch) in lab_colors.iter().zip(&lch_colors) {
            for (&rhs_lab, &rhs_lch) in lab_colors.iter().zip(&lch_colors) {
                let delta_e_lab = lhs_lab.delta_e(rhs_lab);
                let delta_e_lch = lhs_lch.delta_e(rhs_lch);
                assert_relative_eq!(delta_e_lab, delta_e_lch, epsilon = 0.0000000000001);
            }
        }
    }

    // Lab and Lch have the same delta E, so should also have the same improved
    // delta E.
    #[cfg(all(feature = "alloc", feature = "approx"))]
    #[test]
    fn lab_improved_delta_e_equality() {
        let mut lab_colors: Vec<Lab<D65, f64>> = Vec::new();

        for l_step in 0i8..5 {
            for a_step in -2i8..3 {
                for b_step in -2i8..3 {
                    lab_colors.push(Lab::new(
                        l_step as f64 * 25.0,
                        a_step as f64 * 60.0,
                        b_step as f64 * 60.0,
                    ))
                }
            }
        }

        let lch_colors: Vec<Lch<_, _>> = lab_colors.clone().into_color_unclamped();

        for (&lhs_lab, &lhs_lch) in lab_colors.iter().zip(&lch_colors) {
            for (&rhs_lab, &rhs_lch) in lab_colors.iter().zip(&lch_colors) {
                let delta_e_lab = lhs_lab.improved_delta_e(rhs_lab);
                let delta_e_lch = lhs_lch.improved_delta_e(rhs_lch);
                assert_relative_eq!(delta_e_lab, delta_e_lch, epsilon = 0.0000000000001);
            }
        }
    }

    struct_of_arrays_tests!(
        Lch<D65>[l, chroma, hue] phantom: white_point,
        super::Lcha::new(0.1f32, 0.2, 0.3, 0.4),
        super::Lcha::new(0.2, 0.3, 0.4, 0.5),
        super::Lcha::new(0.3, 0.4, 0.5, 0.6)
    );

    #[cfg(feature = "serializing")]
    #[test]
    fn serialize() {
        let serialized = ::serde_json::to_string(&Lch::<D65>::new(0.3, 0.8, 0.1)).unwrap();

        assert_eq!(serialized, r#"{"l":0.3,"chroma":0.8,"hue":0.1}"#);
    }

    #[cfg(feature = "serializing")]
    #[test]
    fn deserialize() {
        let deserialized: Lch =
            ::serde_json::from_str(r#"{"l":0.3,"chroma":0.8,"hue":0.1}"#).unwrap();

        assert_eq!(deserialized, Lch::new(0.3, 0.8, 0.1));
    }

    test_uniform_distribution! {
        Lch<D65, f32> as crate::Lab {
            l: (0.0, 100.0),
            a: (-89.0, 89.0),
            b: (-89.0, 89.0),
        },
        min: Lch::new(0.0f32, 0.0, 0.0),
        max: Lch::new(100.0, 128.0, 360.0)
    }
}
