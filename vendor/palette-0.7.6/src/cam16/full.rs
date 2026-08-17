use crate::{
    angle::RealAngle,
    bool_mask::{HasBoolMask, LazySelect},
    hues::Cam16Hue,
    num::{Abs, Arithmetics, FromScalar, PartialCmp, Powf, Real, Signum, Sqrt, Trigonometry, Zero},
    Alpha, GetHue, Xyz,
};

use super::{
    BakedParameters, Cam16FromUnclamped, Cam16IntoUnclamped, Cam16Jch, Cam16Jmh, Cam16Jsh,
    Cam16Qch, Cam16Qmh, Cam16Qsh, FromCam16Unclamped, IntoCam16Unclamped, WhitePointParameter,
};

/// CIE CAM16 with an alpha component.
///
/// See the [`Cam16a` implementation in `Alpha`](crate::Alpha#Cam16a).
pub type Cam16a<T> = Alpha<Cam16<T>, T>;

/// The CIE CAM16 color appearance model.
///
/// It's a set of six technically defined attributes that describe the
/// appearance of a color under certain viewing conditions, and it's a successor
/// of [CIECAM02](https://en.wikipedia.org/wiki/CIECAM02). The viewing
/// conditions are defined using [`Parameters`][super::Parameters], and two sets
/// of parameters can be used to translate the appearance of a color from one
/// set of viewing conditions to another.
///
/// The use of the viewing conditions parameters sets `Cam16` and its derived
/// types apart from most other color types in this library. It's, for example,
/// not possible to use [`FromColor`][crate::FromColor] and friends to convert
/// to and from other types, since that would require default viewing conditions
/// to exist. Instead, the explicit [`Cam16::from_xyz`] and [`Cam16::into_xyz`]
/// are there to bridge the gap.
///
/// Not all attributes are used when converting _from_ CAM16, since they are
/// correlated and derived from each other. This library also provides partial
/// versions of this struct, to make it easier to correctly specify a minimum
/// attribute set.
///
/// The full list of partial CAM16 variants is:
///
/// * [`Cam16Jch`](crate::cam16::Cam16Jch): lightness and chroma.
/// * [`Cam16Jmh`](crate::cam16::Cam16Jmh): lightness and colorfulness.
/// * [`Cam16Jsh`](crate::cam16::Cam16Jsh): lightness and saturation.
/// * [`Cam16Qch`](crate::cam16::Cam16Qch): brightness and chroma.
/// * [`Cam16Qmh`](crate::cam16::Cam16Qmh): brightness and colorfulness.
/// * [`Cam16Qsh`](crate::cam16::Cam16Qsh): brightness and saturation.
///
/// # CAM16-UCS
///
/// While CIE CAM16 is a model of color appearance, it's not necessarily
/// suitable as a color space. Instead, there is the CAM16-UCS (CAM16 uniform
/// color space), that's based off of the lightness, colorfulness and hue
/// attributes. This colorspace is represented by the
/// [`Cam16UcsJmh`][crate::cam16::Cam16UcsJmh] and
/// [`Cam16UcsJab`][crate::cam16::Cam16UcsJab] types.
///
/// # Creating a Value
///
/// A `Cam16` value would typically come from another color space, or one of the
/// partial sets of CAM16 attributes. All of which require known viewing
/// conditions.
///
/// ```
/// use palette::{
///     Srgb, FromColor, IntoColor,
///     cam16::{Cam16, Parameters, Cam16Jmh, Cam16UcsJmh},
/// };
///
/// // Customize these according to the viewing conditions:
/// let mut example_parameters = Parameters::default_static_wp(40.0);
///
/// // CAM16 from sRGB, or most other color spaces:
/// let rgb = Srgb::new(0.3f32, 0.8, 0.1);
/// let cam16_from_rgb = Cam16::from_xyz(rgb.into_color(), example_parameters);
///
/// // Full CAM16 from a partial set (any partial set can be used):
/// let partial = Cam16Jmh::new(50.0f32, 80.0, 120.0);
/// let cam16_from_partial = partial.into_full(example_parameters);
///
/// // Full CAM16 from CAM16-UCS J'M'h':
/// let ucs = Cam16UcsJmh::new(50.0f32, 80.0, 120.0);
/// let cam16_from_ucs = Cam16Jmh::from_color(ucs).into_full(example_parameters);
/// ```
#[derive(Clone, Copy, Debug, WithAlpha, Default)]
#[palette(palette_internal, component = "T")]
#[repr(C)]
pub struct Cam16<T> {
    /// The [lightness](https://cie.co.at/eilvterm/17-22-063) (J) of the
    /// color.
    ///
    /// It's a perception of the color's luminance, but not linear to it, and is
    /// relative to the reference white. The lightness of black is `0.0` and the
    /// lightness of white is `100.0`.
    ///
    /// Lightness behaves similarly to L\* in [`Lch`][crate::Lch] or lightness
    /// in [`Hsl`][crate::Hsl].
    ///
    /// See also <https://en.wikipedia.org/wiki/Lightness>.
    #[doc(alias = "J")]
    pub lightness: T,

    /// The [chroma](https://cie.co.at/eilvterm/17-22-074) (C) of
    /// the color.
    ///
    /// It's how chromatic the color appears in comparison with a grey color of
    /// the same lightness. Changing the perceived chroma doesn't change the
    /// perceived lightness, and vice versa.
    ///
    /// Chroma behaves similarly to chroma in [`Lch`][crate::Lch] or saturation
    /// in [`Hsl`][crate::Hsl].
    ///
    /// See also <https://en.wikipedia.org/wiki/Colorfulness#Chroma>.
    #[doc(alias = "C")]
    pub chroma: T,

    /// The [hue](https://cie.co.at/eilvterm/17-22-067) (h) of the color.
    ///
    /// The color's position around a color circle, in degrees.
    ///
    /// See also <https://en.wikipedia.org/wiki/Hue>.
    #[doc(alias = "h")]
    pub hue: Cam16Hue<T>,

    /// The [brightness](https://cie.co.at/eilvterm/17-22-059) (Q) of the
    /// color.
    ///
    /// It's the perception of how much light appears to shine from an object.
    /// As opposed to `lightness`, this is not in comparison to a reference
    /// white, but in more absolute terms. Lightness and brightness area also
    /// not linearly correlated in CAM16.
    ///
    /// Brightness behaves similarly to value in [`Hsv`][crate::Hsv].
    ///
    /// See also <https://en.wikipedia.org/wiki/Brightness>.
    #[doc(alias = "Q")]
    pub brightness: T,

    /// The [colorfulness](https://cie.co.at/eilvterm/17-22-072) (M) of the
    /// color.
    ///
    /// It's a perception of how chromatic the color is and usually increases
    /// with luminance, unless the brightness is very high.
    ///
    /// See also <https://en.wikipedia.org/wiki/Colorfulness>.
    #[doc(alias = "M")]
    pub colorfulness: T,

    /// The [saturation](https://cie.co.at/eilvterm/17-22-073)
    /// (s) of the color.
    ///
    /// It's the colorfulness of a color in proportion to its own brightness.
    /// The perceived saturation should stay the same when the perceived
    /// brightness changes, and vice versa.
    ///
    /// Saturation behaves similarly to saturation in [`Hsv`][crate::Hsv].
    ///
    /// See also <https://en.wikipedia.org/wiki/Colorfulness#Saturation>.
    #[doc(alias = "s")]
    pub saturation: T,
}

impl<T> Cam16<T> {
    /// Derive CIE CAM16 attributes for the provided color, under the provided
    /// viewing conditions.
    ///
    /// ```
    /// use palette::{Srgb, IntoColor, cam16::{Cam16, Parameters}};
    ///
    /// // Customize these according to the viewing conditions:
    /// let mut example_parameters = Parameters::default_static_wp(40.0);
    ///
    /// let rgb = Srgb::new(0.3f32, 0.8, 0.1);
    /// let cam16 = Cam16::from_xyz(rgb.into_color(), example_parameters);
    /// ```
    ///
    /// It's also possible to "pre-bake" the parameters, to avoid recalculate
    /// some of the derived values when converting multiple color value.
    ///
    /// ```
    /// use palette::{Srgb, IntoColor, cam16::{Cam16, Parameters}};
    ///
    /// // Customize these according to the viewing conditions:
    /// let mut example_parameters = Parameters::default_static_wp(40.0);
    ///
    /// let baked_parameters = example_parameters.bake();
    ///
    /// let rgb = Srgb::new(0.3f32, 0.8, 0.1);
    /// let cam16 = Cam16::from_xyz(rgb.into_color(), baked_parameters);
    /// ```
    #[inline]
    pub fn from_xyz<WpParam>(
        color: Xyz<WpParam::StaticWp, T>,
        parameters: impl Into<BakedParameters<WpParam, T::Scalar>>,
    ) -> Self
    where
        Xyz<WpParam::StaticWp, T>: IntoCam16Unclamped<WpParam, Self, Scalar = T::Scalar>,
        T: FromScalar,
        WpParam: WhitePointParameter<T::Scalar>,
    {
        color.into_cam16_unclamped(parameters.into())
    }

    /// Construct an XYZ color that matches these CIE CAM16 attributes, under
    /// the provided viewing conditions.
    ///
    /// <p class="warning">
    /// This assumes that all of the correlated attributes are consistent, as
    /// only some of them are actually used. You may want to use one of the
    /// partial CAM16 representations for more control over which set of
    /// attributes that should be.
    /// </p>
    ///
    /// ```
    /// use palette::{Srgb, FromColor, cam16::{Cam16, Parameters}};
    /// # fn get_cam16_value() -> Cam16<f32> {Cam16::default()}
    ///
    /// // Customize these according to the viewing conditions:
    /// let mut example_parameters = Parameters::default_static_wp(40.0);
    ///
    /// let cam16: Cam16<f32> = get_cam16_value();
    /// let rgb = Srgb::from_color(cam16.into_xyz(example_parameters));
    /// ```
    ///
    /// It's also possible to "pre-bake" the parameters, to avoid recalculate
    /// some of the derived values when converting multiple color value.
    ///
    /// ```
    /// use palette::{Srgb, FromColor, cam16::{Cam16, Parameters}};
    /// # fn get_cam16_value() -> Cam16<f32> {Cam16::default()}
    ///
    /// // Customize these according to the viewing conditions:
    /// let mut example_parameters = Parameters::default_static_wp(40.0);
    ///
    /// let baked_parameters = example_parameters.bake();
    ///
    /// let cam16: Cam16<f32> = get_cam16_value();
    /// let rgb = Srgb::from_color(cam16.into_xyz(baked_parameters));
    /// ```
    #[inline]
    pub fn into_xyz<WpParam>(
        self,
        parameters: impl Into<BakedParameters<WpParam, T::Scalar>>,
    ) -> Xyz<WpParam::StaticWp, T>
    where
        Self: Cam16IntoUnclamped<WpParam, Xyz<WpParam::StaticWp, T>, Scalar = T::Scalar>,
        WpParam: WhitePointParameter<T>,
        T: FromScalar,
    {
        self.cam16_into_unclamped(parameters.into())
    }
}

///<span id="Cam16a"></span>[`Cam16a`](crate::cam16::Cam16a) implementations.
impl<T, A> Alpha<Cam16<T>, A> {
    /// Derive CIE CAM16 attributes with transparency for the provided color,
    /// under the provided viewing conditions.
    ///
    /// ```
    /// use palette::{Srgba, IntoColor, cam16::{Cam16a, Parameters}};
    ///
    /// // Customize these according to the viewing conditions:
    /// let mut example_parameters = Parameters::default_static_wp(40.0);
    ///
    /// let rgba = Srgba::new(0.3f32, 0.8, 0.1, 0.9);
    /// let cam16a = Cam16a::from_xyz(rgba.into_color(), example_parameters);
    /// ```
    ///
    /// It's also possible to "pre-bake" the parameters, to avoid recalculate
    /// some of the derived values when converting multiple color value.
    ///
    /// ```
    /// use palette::{Srgba, IntoColor, cam16::{Cam16a, Parameters}};
    ///
    /// // Customize these according to the viewing conditions:
    /// let mut example_parameters = Parameters::default_static_wp(40.0);
    ///
    /// let baked_parameters = example_parameters.bake();
    ///
    /// let rgba = Srgba::new(0.3f32, 0.8, 0.1, 0.9);
    /// let cam16a = Cam16a::from_xyz(rgba.into_color(), baked_parameters);
    /// ```
    #[inline]
    pub fn from_xyz<WpParam>(
        color: Alpha<Xyz<WpParam::StaticWp, T>, A>,
        parameters: impl Into<BakedParameters<WpParam, T::Scalar>>,
    ) -> Self
    where
        Xyz<WpParam::StaticWp, T>: IntoCam16Unclamped<WpParam, Cam16<T>, Scalar = T::Scalar>,
        T: FromScalar,
        WpParam: WhitePointParameter<T::Scalar>,
    {
        let Alpha { color, alpha } = color;

        Alpha {
            color: Cam16::from_xyz(color, parameters),
            alpha,
        }
    }

    /// Construct an XYZ color with transparency, that matches these CIE CAM16
    /// attributes, under the provided viewing conditions.
    ///
    /// <p class="warning">
    /// This assumes that all of the correlated attributes are consistent, as
    /// only some of them are actually used. You may want to use one of the
    /// partial CAM16 representations for more control over which set of
    /// attributes that should be.
    /// </p>
    ///
    /// ```
    /// use palette::{Srgba, FromColor, cam16::{Cam16a, Parameters}};
    /// # fn get_cam16a_value() -> Cam16a<f32> {Cam16a::default()}
    ///
    /// // Customize these according to the viewing conditions:
    /// let mut example_parameters = Parameters::default_static_wp(40.0);
    ///
    /// let cam16a = get_cam16a_value();
    /// let rgba = Srgba::from_color(cam16a.into_xyz(example_parameters));
    /// ```
    ///
    /// It's also possible to "pre-bake" the parameters, to avoid recalculate
    /// some of the derived values when converting multiple color value.
    ///
    /// ```
    /// use palette::{Srgba, FromColor, cam16::{Cam16a, Parameters}};
    /// # fn get_cam16a_value() -> Cam16a<f32> {Cam16a::default()}
    ///
    /// // Customize these according to the viewing conditions:
    /// let mut example_parameters = Parameters::default_static_wp(40.0);
    ///
    /// let baked_parameters = example_parameters.bake();
    ///
    /// let cam16a = get_cam16a_value();
    /// let rgba = Srgba::from_color(cam16a.into_xyz(baked_parameters));
    /// ```
    #[inline]
    pub fn into_xyz<WpParam>(
        self,
        parameters: impl Into<BakedParameters<WpParam, T::Scalar>>,
    ) -> Alpha<Xyz<WpParam::StaticWp, T>, A>
    where
        Cam16<T>: Cam16IntoUnclamped<WpParam, Xyz<WpParam::StaticWp, T>, Scalar = T::Scalar>,
        WpParam: WhitePointParameter<T>,
        T: FromScalar,
    {
        let Alpha { color, alpha } = self;

        Alpha {
            color: color.into_xyz(parameters),
            alpha,
        }
    }
}

impl<WpParam, T> Cam16FromUnclamped<WpParam, Xyz<WpParam::StaticWp, T>> for Cam16<T>
where
    WpParam: WhitePointParameter<T::Scalar>,
    T: Real
        + FromScalar
        + Arithmetics
        + Powf
        + Sqrt
        + Abs
        + Signum
        + Trigonometry
        + RealAngle
        + Clone,
    T::Scalar: Clone,
{
    type Scalar = T::Scalar;

    fn cam16_from_unclamped(
        color: Xyz<WpParam::StaticWp, T>,
        parameters: BakedParameters<WpParam, Self::Scalar>,
    ) -> Self {
        super::math::xyz_to_cam16(color.with_white_point(), parameters.inner)
    }
}

macro_rules! impl_from_cam16_partial {
    ($($name: ident),+) => {
        $(
            impl<WpParam, T> Cam16FromUnclamped<WpParam, $name<T>> for Cam16<T>
            where
                WpParam: WhitePointParameter<T>,
                T: Real + FromScalar + Zero + Arithmetics + Sqrt + PartialCmp + Clone,
                T::Mask: LazySelect<T> + Clone,
                T::Scalar: Clone
            {
                type Scalar = T::Scalar;

                fn cam16_from_unclamped(
                    cam16: $name<T>,
                    parameters: crate::cam16::BakedParameters<WpParam, Self::Scalar>,
                ) -> Self {
                    let (
                        luminance,
                        chromaticity,
                        hue,
                    ) = cam16.into_dynamic();

                    let (lightness, brightness) = luminance.into_cam16(parameters.clone());
                    let (chroma, colorfulness, saturation) =
                        chromaticity.into_cam16(lightness.clone(), parameters);

                    Cam16 {
                        lightness,
                        chroma,
                        hue,
                        brightness,
                        colorfulness,
                        saturation,
                    }
                }
            }

            impl<WpParam, T> FromCam16Unclamped<WpParam, $name<T>> for Cam16<T>
            where
                Self: Cam16FromUnclamped<WpParam, $name<T>>,
            {
                type Scalar = <Self as Cam16FromUnclamped<WpParam, $name<T>>>::Scalar;

                fn from_cam16_unclamped(
                    cam16: $name<T>,
                    parameters: crate::cam16::BakedParameters<WpParam, Self::Scalar>,
                ) -> Self {
                    Self::cam16_from_unclamped(cam16, parameters)
                }
            }
        )+
    };
}

impl_from_cam16_partial!(Cam16Jmh, Cam16Jch, Cam16Jsh, Cam16Qmh, Cam16Qch, Cam16Qsh);

impl<T> GetHue for Cam16<T>
where
    T: Clone,
{
    type Hue = Cam16Hue<T>;

    fn get_hue(&self) -> Cam16Hue<T> {
        self.hue.clone()
    }
}

impl<T> HasBoolMask for Cam16<T>
where
    T: HasBoolMask,
{
    type Mask = T::Mask;
}

// Macro implementations

impl_is_within_bounds! {
    Cam16 {
        lightness => [T::zero(), None],
        chroma => [T::zero(), None],
        brightness => [T::zero(), None],
        colorfulness => [T::zero(), None],
        saturation => [T::zero(), None]
    }
    where T: Zero
}
impl_clamp! {
    Cam16 {
        lightness => [T::zero()],
        chroma => [T::zero()],
        brightness => [T::zero()],
        colorfulness => [T::zero()],
        saturation => [T::zero()]
    }
    other {hue}
    where T: Zero
}

impl_eq_hue!(
    Cam16,
    Cam16Hue,
    [lightness, chroma, brightness, colorfulness, saturation]
);
impl_simd_array_conversion_hue!(
    Cam16,
    [lightness, chroma, brightness, colorfulness, saturation]
);

// Unit test

#[cfg(test)]
#[cfg(feature = "approx")]
mod test {
    use crate::{
        cam16::{
            math::{chromaticity::ChromaticityType, luminance::LuminanceType},
            BakedParameters, Cam16Jch, Parameters,
        },
        convert::{FromColorUnclamped, IntoColorUnclamped},
        Srgb,
    };

    use super::Cam16;

    macro_rules! assert_cam16_to_rgb {
        ($cam16:expr, $rgb:expr, $($params:tt)*) => {
            let cam16 = $cam16;
            let parameters = BakedParameters::from(Parameters::TEST_DEFAULTS);

            let rgb: Srgb<f64> = cam16.into_xyz(parameters).into_color_unclamped();
            assert_relative_eq!(rgb, $rgb, $($params)*);

            let chromaticities = [
                ChromaticityType::Chroma(cam16.chroma),
                ChromaticityType::Colorfulness(cam16.colorfulness),
                ChromaticityType::Saturation(cam16.saturation),
            ];
            let luminances = [
                LuminanceType::Lightness(cam16.lightness),
                LuminanceType::Brightness(cam16.brightness),
            ];

            for luminance in luminances {
                for chromaticity in chromaticities {
                    let partial = (
                        luminance,
                        chromaticity,
                        cam16.hue,
                    );

                    let xyz = crate::cam16::math::cam16_to_xyz(partial, parameters.inner).with_white_point();

                    assert_relative_eq!(
                        Srgb::<f64>::from_color_unclamped(xyz),
                        $rgb,
                        $($params)*
                    );
                }
            }
        };
    }

    #[test]
    fn converts_with_jch() {
        let parameters = Parameters::TEST_DEFAULTS.bake();
        let xyz = Srgb::from(0x5588cc).into_linear().into_color_unclamped();
        let mut cam16: Cam16<f64> = Cam16::from_xyz(xyz, parameters);
        let cam16jch = Cam16Jch::from_full(cam16);

        // Zero the other attributes so they produce errors if they are used.
        cam16.brightness = 0.0;
        cam16.colorfulness = 0.0;
        cam16.saturation = 0.0;

        assert_eq!(cam16.into_xyz(parameters), cam16jch.into_xyz(parameters));
    }

    #[test]
    fn example_blue() {
        // Uses the example color from https://observablehq.com/@jrus/cam16
        let xyz = Srgb::from(0x5588cc).into_linear().into_color_unclamped();
        let mut cam16: Cam16<f64> = Cam16::from_xyz(xyz, Parameters::TEST_DEFAULTS);
        cam16.hue = cam16.hue.into_positive_degrees().into();

        assert_relative_eq!(
            cam16,
            Cam16 {
                lightness: 45.544264720360346,
                chroma: 45.07001048293764,
                hue: 259.225345298129.into(),
                brightness: 132.96974182692045,
                colorfulness: 39.4130607870103,
                saturation: 54.4432031413259,
            },
            epsilon = 0.01
        );

        assert_cam16_to_rgb!(
            cam16,
            Srgb::from(0x5588cc).into_format(),
            epsilon = 0.0000001
        );
    }

    #[test]
    fn black() {
        // Checks against the output from https://observablehq.com/@jrus/cam16
        let xyz = Srgb::from(0x000000).into_linear().into_color_unclamped();
        let mut cam16: Cam16<f64> = Cam16::from_xyz(xyz, Parameters::TEST_DEFAULTS);
        cam16.hue = cam16.hue.into_positive_degrees().into();

        assert_relative_eq!(
            cam16,
            Cam16 {
                lightness: 0.0,
                chroma: 0.0,
                hue: 0.0.into(),
                brightness: 0.0,
                colorfulness: 0.0,
                saturation: 0.0,
            },
            epsilon = 0.01
        );

        assert_cam16_to_rgb!(
            cam16,
            Srgb::from(0x000000).into_format(),
            epsilon = 0.0000001
        );
    }

    #[test]
    fn white() {
        // Checks against the output from https://observablehq.com/@jrus/cam16
        let xyz = Srgb::from(0xffffff).into_linear().into_color_unclamped();
        let mut cam16: Cam16<f64> = Cam16::from_xyz(xyz, Parameters::TEST_DEFAULTS);
        cam16.hue = cam16.hue.into_positive_degrees().into();

        assert_relative_eq!(
            cam16,
            Cam16 {
                lightness: 99.99955537650459,
                chroma: 2.1815254387079435,
                hue: 209.49854407518228.into(),
                brightness: 197.03120459014184,
                colorfulness: 1.9077118865271965,
                saturation: 9.839859256901553,
            },
            epsilon = 0.1
        );

        assert_cam16_to_rgb!(
            cam16,
            Srgb::from(0xffffff).into_format(),
            epsilon = 0.0000001
        );
    }

    #[test]
    fn red() {
        // Checks against the output from https://observablehq.com/@jrus/cam16
        let xyz = Srgb::from(0xff0000).into_linear().into_color_unclamped();
        let mut cam16: Cam16<f64> = Cam16::from_xyz(xyz, Parameters::TEST_DEFAULTS);
        cam16.hue = cam16.hue.into_positive_degrees().into();

        assert_relative_eq!(
            cam16,
            Cam16 {
                lightness: 46.23623443823762,
                chroma: 113.27879472174797,
                hue: 27.412485587695937.into(),
                brightness: 133.9760614641257,
                colorfulness: 99.06063864657237,
                saturation: 85.98782392745971,
            },
            epsilon = 0.01
        );

        assert_cam16_to_rgb!(cam16, Srgb::from(0xff0000).into_format(), epsilon = 0.00001);
    }

    #[test]
    fn green() {
        // Checks against the output from https://observablehq.com/@jrus/cam16
        let xyz = Srgb::from(0x00ff00).into_linear().into_color_unclamped();
        let mut cam16: Cam16<f64> = Cam16::from_xyz(xyz, Parameters::TEST_DEFAULTS);
        cam16.hue = cam16.hue.into_positive_degrees().into();

        assert_relative_eq!(
            cam16,
            Cam16 {
                lightness: 79.23121430933533,
                chroma: 107.77869525794452,
                hue: 141.93451307926003.into(),
                brightness: 175.38164288466993,
                colorfulness: 94.25088262080988,
                saturation: 73.30787758114869,
            },
            epsilon = 0.01
        );

        assert_cam16_to_rgb!(
            cam16,
            Srgb::from(0x00ff00).into_format(),
            epsilon = 0.000001
        );
    }

    #[test]
    fn blue() {
        // Checks against the output from https://observablehq.com/@jrus/cam16
        let xyz = Srgb::from(0x0000ff).into_linear().into_color_unclamped();
        let mut cam16: Cam16<f64> = Cam16::from_xyz(xyz, Parameters::TEST_DEFAULTS);
        cam16.hue = cam16.hue.into_positive_degrees().into();

        assert_relative_eq!(
            cam16,
            Cam16 {
                lightness: 25.22701796474445,
                chroma: 86.59618504567312,
                hue: 282.81848901862566.into(),
                brightness: 98.96210767195342,
                colorfulness: 75.72708922311855,
                saturation: 87.47645277637828,
            },
            epsilon = 0.01
        );

        assert_cam16_to_rgb!(
            cam16,
            Srgb::from(0x0000ff).into_format(),
            epsilon = 0.000001
        );
    }

    #[cfg(feature = "wide")]
    #[test]
    fn simd() {
        let white_srgb = Srgb::from(0xffffff).into_format();
        let white_cam16 = Cam16 {
            lightness: 99.99955537650459,
            chroma: 2.1815254387079435,
            hue: 209.49854407518228.into(),
            brightness: 197.03120459014184,
            colorfulness: 1.9077118865271965,
            saturation: 9.839859256901553,
        };

        let red_srgb = Srgb::from(0xff0000).into_format();
        let red_cam16 = Cam16 {
            lightness: 46.23623443823762,
            chroma: 113.27879472174797,
            hue: 27.412485587695937.into(),
            brightness: 133.9760614641257,
            colorfulness: 99.06063864657237,
            saturation: 85.98782392745971,
        };

        let green_srgb = Srgb::from(0x00ff00).into_format();
        let green_cam16 = Cam16 {
            lightness: 79.23121430933533,
            chroma: 107.77869525794452,
            hue: 141.93451307926003.into(),
            brightness: 175.38164288466993,
            colorfulness: 94.25088262080988,
            saturation: 73.30787758114869,
        };

        let blue_srgb = Srgb::from(0x0000ff).into_format();
        let blue_cam16 = Cam16 {
            lightness: 25.22701796474445,
            chroma: 86.59618504567312,
            hue: 282.81848901862566.into(),
            brightness: 98.96210767195342,
            colorfulness: 75.72708922311855,
            saturation: 87.47645277637828,
        };

        let srgb = Srgb::<wide::f64x4>::from([white_srgb, red_srgb, green_srgb, blue_srgb]);
        let xyz = srgb.into_linear().into_color_unclamped();
        let mut cam16 = Cam16::from_xyz(xyz, Parameters::TEST_DEFAULTS);
        cam16.hue = cam16.hue.into_positive_degrees().into();

        assert_relative_eq!(
            &<[Cam16<_>; 4]>::from(cam16)[..],
            &[white_cam16, red_cam16, green_cam16, blue_cam16][..],
            epsilon = 0.1
        );

        let srgb = Srgb::from_color_unclamped(cam16.into_xyz(Parameters::TEST_DEFAULTS));

        assert_relative_eq!(
            &<[Srgb<_>; 4]>::from(srgb)[..],
            &[white_srgb, red_srgb, green_srgb, blue_srgb][..],
            epsilon = 0.00001
        );
    }
}
