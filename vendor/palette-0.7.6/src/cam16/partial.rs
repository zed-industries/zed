use crate::{
    cam16::Cam16UcsJmh,
    convert::FromColorUnclamped,
    num::{Arithmetics, Exp, One, Real},
    Alpha,
};

macro_rules! make_partial_cam16 {
    (
        $(#[$type_meta: meta])*
        $module: ident :: $name: ident {
            $(#[$luminance_meta: meta])+
            $luminance: ident : $luminance_ty: ident,
            $(#[$chromaticity_meta: meta])+
            $chromaticity: ident : $chromaticity_ty: ident
        }
    ) => {
        pub use $module::$name;

        #[doc = concat!("Partial CIE CAM16, with ", stringify!($luminance), " and ", stringify!($chromaticity), ", and helper types.")]
        pub mod $module {
            use crate::{
                bool_mask::HasBoolMask,
                cam16::{BakedParameters, Cam16, WhitePointParameter, Cam16FromUnclamped, IntoCam16Unclamped, Cam16IntoUnclamped},
                convert::FromColorUnclamped,
                hues::{Cam16Hue, Cam16HueIter},
                num::{FromScalar, Zero},
                Alpha, Xyz,
            };

            use crate::cam16::math::chromaticity::*;
            use crate::cam16::math::luminance::*;

            #[doc = concat!("Partial CIE CAM16, with ", stringify!($luminance), " and ", stringify!($chromaticity), ".")]
            ///
            /// It contains enough information for converting CAM16 to other
            /// color spaces. See [Cam16] for more details about CIE CAM16.
            ///
            /// The full list of partial CAM16 variants is:
            ///
            /// * [`Cam16Jch`](crate::cam16::Cam16Jch): lightness and chroma.
            /// * [`Cam16Jmh`](crate::cam16::Cam16Jmh): lightness and
            ///   colorfulness.
            /// * [`Cam16Jsh`](crate::cam16::Cam16Jsh): lightness and
            ///   saturation.
            /// * [`Cam16Qch`](crate::cam16::Cam16Qch): brightness and chroma.
            /// * [`Cam16Qmh`](crate::cam16::Cam16Qmh): brightness and
            ///   colorfulness.
            /// * [`Cam16Qsh`](crate::cam16::Cam16Qsh): brightness and
            ///   saturation.
            ///
            /// # Creating a Value
            ///
            /// Any partial CAM16 set can be obtained from the full set of
            /// attributes. It's also possible to convert directly to it, using
            #[doc = concat!("[`from_xyz`][", stringify!($name), "::from_xyz],")]
            /// or to create a new value by calling
            #[doc = concat!("[`new`][", stringify!($name), "::new].")]
            /// ```
            /// use palette::{
            ///     Srgb, FromColor, IntoColor, hues::Cam16Hue,
            #[doc = concat!("    cam16::{Cam16, Parameters, ", stringify!($name), "},")]
            /// };
            ///
            #[doc = concat!("let partial = ", stringify!($name), "::new(50.0f32, 80.0, 120.0);")]
            ///
            /// // There's also `new_const`:
            #[doc = concat!("const PARTIAL: ", stringify!($name), "<f32> = ", stringify!($name), "::new_const(50.0, 80.0, Cam16Hue::new(120.0));")]
            ///
            /// // Customize these according to the viewing conditions:
            /// let mut example_parameters = Parameters::default_static_wp(40.0);
            ///
            /// // Partial CAM16 from sRGB, or most other color spaces:
            /// let rgb = Srgb::new(0.3f32, 0.8, 0.1);
            #[doc = concat!("let partial_from_rgb = ", stringify!($name), "::from_xyz(rgb.into_color(), example_parameters);")]
            ///
            /// // Partial CAM16 from sRGB, via full CAM16:
            /// let rgb = Srgb::new(0.3f32, 0.8, 0.1);
            /// let cam16_from_rgb = Cam16::from_xyz(rgb.into_color(), example_parameters);
            #[doc = concat!("let partial_from_full = ", stringify!($name), "::from(cam16_from_rgb);")]
            ///
            /// // Direct conversion has the same result as going via full CAM16.
            /// assert_eq!(partial_from_rgb, partial_from_full);
            ///
            /// // It's also possible to convert from (and to) arrays and tuples:
            #[doc = concat!("let partial_from_array = ", stringify!($name), "::from([50.0f32, 80.0, 120.0]);")]
            #[doc = concat!("let partial_from_tuple = ", stringify!($name), "::from((50.0f32, 80.0, 120.0));")]
            ///  ```
            #[derive(Clone, Copy, Debug, Default, ArrayCast, WithAlpha, FromColorUnclamped)]
            #[palette(
                palette_internal,
                component = "T",
                skip_derives(Cam16, $name)
            )]
            $(#[$type_meta])*
            #[repr(C)]
            pub struct $name<T> {
                $(#[$luminance_meta])+
                pub $luminance: T,

                $(#[$chromaticity_meta])+
                pub $chromaticity: T,

                /// The [hue](https://cie.co.at/eilvterm/17-22-067) (h) of the color.
                ///
                /// See [`Cam16::hue`][crate::cam16::Cam16::hue].
                #[palette(unsafe_same_layout_as = "T")]
                pub hue: Cam16Hue<T>,
            }

            impl<T> $name<T> {
                /// Create a partial CIE CAM16 color.
                #[inline]
                pub fn new<H>($luminance: T, $chromaticity: T, hue: H) -> Self
                where
                    H: Into<Cam16Hue<T>>,
                {
                    Self::new_const($luminance.into(), $chromaticity.into(), hue.into())
                }

                #[doc = concat!("Create a partial CIE CAM16 color. This is the same as `", stringify!($name), "::new`")]
                /// without the generic hue type. It's temporary until `const fn`
                /// supports traits.
                #[inline]
                pub const fn new_const($luminance: T, $chromaticity: T, hue: Cam16Hue<T>) -> Self {
                    Self {
                        $luminance,
                        $chromaticity,
                        hue,
                    }
                }

                #[doc = concat!("Convert to a `(", stringify!($luminance), ", ", stringify!($chromaticity), ", hue)` tuple.")]
                #[inline]
                pub fn into_components(self) -> (T, T, Cam16Hue<T>) {
                    (self.$luminance, self.$chromaticity, self.hue)
                }

                #[doc = concat!("Convert from a `(", stringify!($luminance), ", ", stringify!($chromaticity), ", hue)` tuple.")]
                #[inline]
                pub fn from_components<H>(($luminance, $chromaticity, hue): (T, T, H)) -> Self
                where
                    H: Into<Cam16Hue<T>>,
                {
                    Self::new($luminance, $chromaticity, hue)
                }

                /// Derive partial CIE CAM16 attributes for the provided color, under the provided
                /// viewing conditions.
                ///
                /// ```
                #[doc = concat!("use palette::{Srgb, IntoColor, cam16::{", stringify!($name), ", Parameters}};")]
                ///
                /// // Customize these according to the viewing conditions:
                /// let mut example_parameters = Parameters::default_static_wp(40.0);
                ///
                /// let rgb = Srgb::new(0.3f32, 0.8, 0.1);
                #[doc = concat!("let partial = ", stringify!($name), "::from_xyz(rgb.into_color(), example_parameters);")]
                /// ```
                ///
                /// It's also possible to "pre-bake" the parameters, to avoid recalculate
                /// some of the derived values when converting multiple color value.
                ///
                /// ```
                #[doc = concat!("use palette::{Srgb, IntoColor, cam16::{", stringify!($name), ", Parameters}};")]
                ///
                /// // Customize these according to the viewing conditions:
                /// let mut example_parameters = Parameters::default_static_wp(40.0);
                ///
                /// let baked_parameters = example_parameters.bake();
                ///
                /// let rgb = Srgb::new(0.3f32, 0.8, 0.1);
                #[doc = concat!("let partial = ", stringify!($name), "::from_xyz(rgb.into_color(), baked_parameters);")]
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

                /// Construct an XYZ color from these CIE CAM16 attributes, under the
                /// provided viewing conditions.
                ///
                /// ```
                #[doc = concat!("use palette::{Srgb, FromColor, cam16::{", stringify!($name), ", Parameters}};")]
                ///
                /// // Customize these according to the viewing conditions:
                /// let mut example_parameters = Parameters::default_static_wp(40.0);
                ///
                #[doc = concat!("let partial = ", stringify!($name), "::new(50.0f32, 80.0, 120.0);")]
                /// let rgb = Srgb::from_color(partial.into_xyz(example_parameters));
                /// ```
                ///
                /// It's also possible to "pre-bake" the parameters, to avoid recalculate
                /// some of the derived values when converting multiple color value.
                ///
                /// ```
                #[doc = concat!("use palette::{Srgb, FromColor, cam16::{", stringify!($name), ", Parameters}};")]
                ///
                /// // Customize these according to the viewing conditions:
                /// let mut example_parameters = Parameters::default_static_wp(40.0);
                ///
                /// let baked_parameters = example_parameters.bake();
                ///
                #[doc = concat!("let partial = ", stringify!($name), "::new(50.0f32, 80.0, 120.0);")]
                /// let rgb = Srgb::from_color(partial.into_xyz(baked_parameters));
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

                /// Create a partial set of CIE CAM16 attributes.
                ///
                #[doc = concat!("It's also possible to use `", stringify!($name), "::from` or `Cam16::into`.")]
                #[inline]
                pub fn from_full(full: Cam16<T>) -> Self {
                    Self {
                        hue: full.hue,
                        $chromaticity: full.$chromaticity,
                        $luminance: full.$luminance,
                    }
                }

                /// Reconstruct a full set of CIE CAM16 attributes, using the original viewing conditions.
                ///
                /// ```
                #[doc = concat!("use palette::{Srgb, IntoColor, cam16::{Cam16, ", stringify!($name), ", Parameters}};")]
                /// use approx::assert_relative_eq;
                ///
                /// // Customize these according to the viewing conditions:
                /// let mut example_parameters = Parameters::default_static_wp(40.0);
                ///
                /// // Optional, but saves some work:
                /// let baked_parameters = example_parameters.bake();
                ///
                /// let rgb = Srgb::new(0.3f64, 0.8, 0.1);
                /// let cam16 = Cam16::from_xyz(rgb.into_color(), baked_parameters);
                #[doc = concat!("let partial = ", stringify!($name), "::from(cam16);")]
                /// let reconstructed = partial.into_full(baked_parameters);
                ///
                /// assert_relative_eq!(cam16, reconstructed, epsilon = 0.0000000000001);
                ///  ```
                #[inline]
                pub fn into_full<WpParam>(self, parameters: impl Into<BakedParameters<WpParam, T::Scalar>>) -> Cam16<T>
                where
                    Self: IntoCam16Unclamped<WpParam, Cam16<T>, Scalar = T::Scalar>,
                    T: FromScalar
                {
                    self.into_cam16_unclamped(parameters.into())
                }

                // Turn the chromaticity and luminance into dynamically decided
                // attributes, to help conversion to a full set of attributes.
                #[inline(always)]
                pub(crate) fn into_dynamic(self) -> (LuminanceType<T>, ChromaticityType<T>, Cam16Hue<T>) {
                    (
                        LuminanceType::$luminance_ty(self.$luminance),
                        ChromaticityType::$chromaticity_ty(self.$chromaticity),
                        self.hue,
                    )
                }
            }

            #[doc = concat!(r#""<span id=""#, stringify!($name), r#"a"></span>[`"#, stringify!($name), "a`](crate::cam16::", stringify!($name), "a)")]
            ///implementations.
            impl<T, A> Alpha<$name<T>, A> {
                /// Create a partial CIE CAM16 color with transparency.
                #[inline]
                pub fn new<H: Into<Cam16Hue<T>>>($luminance: T, $chromaticity: T, hue: H, alpha: A) -> Self{
                    Self::new_const($luminance.into(), $chromaticity.into(), hue.into(), alpha)
                }

                /// Create a partial CIE CAM16 color with transparency. This is the
                #[doc = concat!("same as `", stringify!($name), "::new` without the generic hue type. It's temporary until")]
                /// `const fn` supports traits.
                #[inline]
                pub const fn new_const($luminance: T, $chromaticity: T, hue: Cam16Hue<T>, alpha: A) -> Self {
                    Alpha {
                        color: $name::new_const($luminance, $chromaticity, hue),
                        alpha,
                    }
                }

                #[doc = concat!("Convert to a `(", stringify!($luminance), ", ", stringify!($chromaticity), ", hue, alpha)` tuple.")]
                #[inline]
                pub fn into_components(self) -> (T, T, Cam16Hue<T>, A) {
                    (
                        self.color.$luminance,
                        self.color.$chromaticity,
                        self.color.hue,
                        self.alpha,
                    )
                }

                #[doc = concat!("Convert from a `(", stringify!($luminance), ", ", stringify!($chromaticity), ", hue, alpha)` tuple.")]
                #[inline]
                pub fn from_components<H: Into<Cam16Hue<T>>>(
                    ($luminance, $chromaticity, hue, alpha): (T, T, H, A),
                ) -> Self {
                    Self::new($luminance, $chromaticity, hue, alpha)
                }

                /// Derive partial CIE CAM16 attributes with transparency, for the provided
                /// color, under the provided viewing conditions.
                ///
                /// ```
                #[doc = concat!("use palette::{Srgba, IntoColor, cam16::{", stringify!($name), "a, Parameters}};")]
                ///
                /// // Customize these according to the viewing conditions:
                /// let mut example_parameters = Parameters::default_static_wp(40.0);
                ///
                /// let rgba = Srgba::new(0.3f32, 0.8, 0.1, 0.9);
                #[doc = concat!("let partial = ", stringify!($name), "a::from_xyz(rgba.into_color(), example_parameters);")]
                /// ```
                ///
                /// It's also possible to "pre-bake" the parameters, to avoid recalculate
                /// some of the derived values when converting multiple color value.
                ///
                /// ```
                #[doc = concat!("use palette::{Srgba, IntoColor, cam16::{", stringify!($name), "a, Parameters}};")]
                ///
                /// // Customize these according to the viewing conditions:
                /// let mut example_parameters = Parameters::default_static_wp(40.0);
                ///
                /// let baked_parameters = example_parameters.bake();
                ///
                /// let rgba = Srgba::new(0.3f32, 0.8, 0.1, 0.9);
                #[doc = concat!("let partial = ", stringify!($name), "a::from_xyz(rgba.into_color(), baked_parameters);")]
                /// ```
                #[inline]
                pub fn from_xyz<WpParam>(
                    color: Alpha<Xyz<WpParam::StaticWp, T>, A>,
                    parameters: impl Into<BakedParameters<WpParam, T::Scalar>>,
                ) -> Self
                where
                    Xyz<WpParam::StaticWp, T>: IntoCam16Unclamped<WpParam, $name<T>, Scalar = T::Scalar>,
                    T: FromScalar,
                    WpParam: WhitePointParameter<T::Scalar>,
                {
                    let Alpha { color, alpha } = color;

                    Alpha {
                        color: $name::from_xyz(color, parameters),
                        alpha,
                    }
                }

                /// Construct an XYZ color with transparency, from these CIE CAM16
                /// attributes, under the provided viewing conditions.
                ///
                /// ```
                #[doc = concat!("use palette::{Srgba, FromColor, cam16::{", stringify!($name), "a, Parameters}};")]
                ///
                /// // Customize these according to the viewing conditions:
                /// let mut example_parameters = Parameters::default_static_wp(40.0);
                ///
                #[doc = concat!("let partial = ", stringify!($name), "a::new(50.0f32, 80.0, 120.0, 0.9);")]
                /// let rgba = Srgba::from_color(partial.into_xyz(example_parameters));
                /// ```
                ///
                /// It's also possible to "pre-bake" the parameters, to avoid recalculate
                /// some of the derived values when converting multiple color value.
                ///
                /// ```
                #[doc = concat!("use palette::{Srgba, FromColor, cam16::{", stringify!($name), "a, Parameters}};")]
                ///
                /// // Customize these according to the viewing conditions:
                /// let mut example_parameters = Parameters::default_static_wp(40.0);
                ///
                /// let baked_parameters = example_parameters.bake();
                ///
                #[doc = concat!("let partial = ", stringify!($name), "a::new(50.0f32, 80.0, 120.0, 0.9);")]
                /// let rgba = Srgba::from_color(partial.into_xyz(baked_parameters));
                /// ```
                #[inline]
                pub fn into_xyz<WpParam>(
                    self,
                    parameters: impl Into<BakedParameters<WpParam, T::Scalar>>,
                ) -> Alpha<Xyz<WpParam::StaticWp, T>, A>
                where
                    $name<T>: Cam16IntoUnclamped<WpParam, Xyz<WpParam::StaticWp, T>, Scalar = T::Scalar>,
                    WpParam: WhitePointParameter<T>,
                    T: FromScalar,
                {
                    let Alpha { color, alpha } = self;

                    Alpha {
                        color: color.into_xyz(parameters),
                        alpha,
                    }
                }

                /// Create a partial set of CIE CAM16 attributes with transparency.
                ///
                #[doc = concat!("It's also possible to use `", stringify!($name), "a::from` or `Cam16a::into`.")]
                #[inline]
                pub fn from_full(full: Alpha<Cam16<T>, A>) -> Self {
                    let Alpha { color, alpha } = full;

                    Alpha {
                        color: $name::from_full(color),
                        alpha,
                    }
                }

                /// Reconstruct a full set of CIE CAM16 attributes with transparency, using
                /// the original viewing conditions.
                ///
                /// ```
                #[doc = concat!("use palette::{Srgba, IntoColor, cam16::{Cam16a, ", stringify!($name), "a, Parameters}};")]
                /// use approx::assert_relative_eq;
                ///
                /// // Customize these according to the viewing conditions:
                /// let mut example_parameters = Parameters::default_static_wp(40.0);
                ///
                /// // Optional, but saves some work:
                /// let baked_parameters = example_parameters.bake();
                ///
                /// let rgba = Srgba::new(0.3f64, 0.8, 0.1, 0.9);
                /// let cam16a = Cam16a::from_xyz(rgba.into_color(), baked_parameters);
                #[doc = concat!("let partial = ", stringify!($name), "a::from(cam16a);")]
                /// let reconstructed = partial.into_full(baked_parameters);
                ///
                /// assert_relative_eq!(cam16a, reconstructed, epsilon = 0.0000000000001);
                ///  ```
                #[inline]
                pub fn into_full<WpParam>(
                    self,
                    parameters: impl Into<BakedParameters<WpParam, T::Scalar>>,
                ) -> Alpha<Cam16<T>, A>
                where
                    $name<T>: IntoCam16Unclamped<WpParam, Cam16<T>, Scalar = T::Scalar>,
                    WpParam: WhitePointParameter<T>,
                    T: FromScalar,
                {
                    let Alpha { color, alpha } = self;

                    Alpha {
                        color: color.into_full(parameters),
                        alpha,
                    }
                }
            }

            impl<T> FromColorUnclamped<Self> for $name<T> {
                #[inline]
                fn from_color_unclamped(val: Self) -> Self {
                    val
                }
            }

            impl<T> FromColorUnclamped<Cam16<T>> for $name<T> {
                #[inline]
                fn from_color_unclamped(val: Cam16<T>) -> Self {
                    Self::from_full(val)
                }
            }

            impl<WpParam, T> Cam16FromUnclamped<WpParam, Xyz<WpParam::StaticWp, T>> for $name<T>
            where
                Xyz<WpParam::StaticWp, T>: IntoCam16Unclamped<WpParam, Cam16<T>>,
                WpParam: WhitePointParameter<T>,
            {
                type Scalar = <Xyz<WpParam::StaticWp, T> as IntoCam16Unclamped<WpParam, Cam16<T>>>::Scalar;

                fn cam16_from_unclamped(color: Xyz<WpParam::StaticWp, T>, parameters: BakedParameters<WpParam, Self::Scalar>) -> Self {
                    color.into_cam16_unclamped(parameters).into()
                }
            }

            impl<T> From<Cam16<T>> for $name<T> {
                #[inline]
                fn from(value: Cam16<T>) -> Self {
                    Self::from_full(value)
                }
            }

            impl<T, A> From<Alpha<Cam16<T>, A>> for Alpha<$name<T>, A> {
                #[inline]
                fn from(value: Alpha<Cam16<T>, A>) -> Self {
                    Self::from_full(value)
                }
            }

            impl<T> HasBoolMask for $name<T>
            where
                T: HasBoolMask,
            {
                type Mask = T::Mask;
            }

            #[cfg(feature = "bytemuck")]
            unsafe impl<T> bytemuck::Zeroable for $name<T> where T: bytemuck::Zeroable {}

            #[cfg(feature = "bytemuck")]
            unsafe impl<T> bytemuck::Pod for $name<T> where T: bytemuck::Pod {}

            impl_reference_component_methods_hue!($name, [$luminance, $chromaticity]);
            impl_struct_of_arrays_methods_hue!($name, [$luminance, $chromaticity]);

            impl_tuple_conversion_hue!($name as (T, T, H), Cam16Hue);

            impl_is_within_bounds! {
                $name {
                    $luminance => [T::zero(), None],
                    $chromaticity => [T::zero(), None]
                }
                where T: Zero
            }
            impl_clamp! {
                $name {
                    $luminance => [T::zero()],
                    $chromaticity => [T::zero()]
                }
                other {hue}
                where T: Zero
            }

            impl_mix_hue!($name {$luminance, $chromaticity});
            impl_hue_ops!($name, Cam16Hue);

            impl_color_add!($name, [$luminance, $chromaticity, hue]);
            impl_color_sub!($name, [$luminance, $chromaticity, hue]);

            impl_array_casts!($name<T>, [T; 3]);
            impl_simd_array_conversion_hue!($name, [$luminance, $chromaticity]);
            impl_struct_of_array_traits_hue!($name, Cam16HueIter, [$luminance, $chromaticity]);

            impl_eq_hue!($name, Cam16Hue, [$luminance, $chromaticity, hue]);
        }
    };
}

/// Partial CIE CAM16 with lightness, chroma, and an alpha component.
///
/// See the [`Cam16Jcha` implementation in `Alpha`](crate::Alpha#Cam16Jcha).
pub type Cam16Jcha<T> = Alpha<Cam16Jch<T>, T>;
make_partial_cam16! {
    cam16_jch::Cam16Jch {
        /// The [lightness](https://cie.co.at/eilvterm/17-22-063) (J) of the
        /// color.
        ///
        /// See [`Cam16::lightness`][crate::cam16::Cam16::lightness].
        lightness: Lightness,

        /// The [chroma](https://cie.co.at/eilvterm/17-22-074) (C) of the color.
        ///
        /// See [`Cam16::chroma`][crate::cam16::Cam16::chroma].
        chroma: Chroma
    }
}

/// Partial CIE CAM16 with lightness, colorfulness, and an alpha component.
///
/// See the [`Cam16Jmha` implementation in `Alpha`](crate::Alpha#Cam16Jmha).
pub type Cam16Jmha<T> = Alpha<Cam16Jmh<T>, T>;
make_partial_cam16! {
    ///
    /// `Cam16Jmh` can also convert from CAM16-UCS types, such as
    /// [`Cam16UcsJmh`][crate::cam16::Cam16UcsJmh].
    ///
    /// ```
    /// use palette::{Srgb, FromColor, cam16::{Cam16Jmh, Cam16UcsJmh}};
    ///
    /// let ucs = Cam16UcsJmh::new(50.0f32, 80.0, 120.0);
    /// let partial_from_ucs = Cam16Jmh::from_color(ucs);
    /// ```
    #[palette(skip_derives(Cam16UcsJmh))]
    cam16_jmh::Cam16Jmh {
        /// The [lightness](https://cie.co.at/eilvterm/17-22-063) (J) of the
        /// color.
        ///
        /// See [`Cam16::lightness`][crate::cam16::Cam16::lightness].
        lightness: Lightness,

        /// The [colorfulness](https://cie.co.at/eilvterm/17-22-072) (M) of the
        /// color.
        ///
        /// See [`Cam16::colorfulness`][crate::cam16::Cam16::colorfulness].
        colorfulness: Colorfulness
    }
}

/// Partial CIE CAM16 with lightness, saturation, and an alpha component.
///
/// See the [`Cam16Jsha` implementation in `Alpha`](crate::Alpha#Cam16Jsha).
pub type Cam16Jsha<T> = Alpha<Cam16Jsh<T>, T>;
make_partial_cam16! {
    cam16_jsh::Cam16Jsh {
        /// The [lightness](https://cie.co.at/eilvterm/17-22-063) (J) of the
        /// color.
        ///
        /// See [`Cam16::lightness`][crate::cam16::Cam16::lightness].
        lightness: Lightness,

        /// The [saturation](https://cie.co.at/eilvterm/17-22-073) (s) of the
        /// color.
        ///
        /// See ['Cam16::saturation][crate::cam16::Cam16::saturation].
        saturation: Saturation
    }
}

/// Partial CIE CAM16 with brightness, chroma, and an alpha component.
///
/// See the [`Cam16Qcha` implementation in `Alpha`](crate::Alpha#Cam16Qcha).
pub type Cam16Qcha<T> = Alpha<Cam16Qch<T>, T>;
make_partial_cam16! {
    cam16_qch::Cam16Qch {
        /// The [brightness](https://cie.co.at/eilvterm/17-22-059) (Q) of the
        /// color.
        ///
        /// See [`Cam16::brightness`][crate::cam16::Cam16::brightness].
        brightness: Brightness,

        /// The [chroma](https://cie.co.at/eilvterm/17-22-074) (C) of the color.
        ///
        /// See [`Cam16::chroma`][crate::cam16::Cam16::chroma].
        chroma: Chroma
    }
}

/// Partial CIE CAM16 with brightness, colorfulness, and an alpha component.
///
/// See the [`Cam16Qmha` implementation in `Alpha`](crate::Alpha#Cam16Qmha).
pub type Cam16Qmha<T> = Alpha<Cam16Qmh<T>, T>;
make_partial_cam16! {
    cam16_qmh::Cam16Qmh {
        /// The [brightness](https://cie.co.at/eilvterm/17-22-059) (Q) of the
        /// color.
        ///
        /// See [`Cam16::brightness`][crate::cam16::Cam16::brightness].
        brightness: Brightness,

        /// The [colorfulness](https://cie.co.at/eilvterm/17-22-072) (M) of the
        /// color.
        ///
        /// See [`Cam16::colorfulness`][crate::cam16::Cam16::colorfulness].
        colorfulness: Colorfulness
    }
}

/// Partial CIE CAM16 with brightness, saturation, and an alpha component.
///
/// See the [`Cam16Qsha` implementation in `Alpha`](crate::Alpha#Cam16Qsha).
pub type Cam16Qsha<T> = Alpha<Cam16Qsh<T>, T>;
make_partial_cam16! {
    cam16_qsh::Cam16Qsh {
        /// The [brightness](https://cie.co.at/eilvterm/17-22-059) (Q) of the
        /// color.
        ///
        /// See [`Cam16::brightness`][crate::cam16::Cam16::brightness].
        brightness: Brightness,

        /// The [saturation](https://cie.co.at/eilvterm/17-22-073) (s) of the
        /// color.
        ///
        /// See ['Cam16::saturation][crate::cam16::Cam16::saturation].
        saturation: Saturation
    }
}

impl<T> FromColorUnclamped<Cam16UcsJmh<T>> for Cam16Jmh<T>
where
    T: Real + One + Exp + Arithmetics + Clone,
{
    #[inline]
    fn from_color_unclamped(val: Cam16UcsJmh<T>) -> Self {
        let colorfulness =
            ((val.colorfulness * T::from_f64(0.0228)).exp() - T::one()) / T::from_f64(0.0228);
        let lightness =
            val.lightness.clone() / (T::from_f64(1.7) - T::from_f64(0.007) * val.lightness);

        Self {
            hue: val.hue,
            colorfulness,
            lightness,
        }
    }
}

#[cfg(test)]
#[cfg(feature = "approx")]
mod test {
    use super::{Cam16Jch, Cam16Jmh, Cam16Jsh, Cam16Qch, Cam16Qmh, Cam16Qsh};
    use crate::{
        cam16::{Cam16, Parameters, StaticWp},
        convert::IntoColorUnclamped,
        white_point::D65,
        Srgb,
    };

    macro_rules! assert_partial_to_full {
        ($cam16: expr) => {assert_partial_to_full!($cam16,)};
        ($cam16: expr, $($params:tt)*) => {
            assert_relative_eq!(
                Cam16Jch::from($cam16).into_full(Parameters::<StaticWp<D65>, _>::TEST_DEFAULTS),
                $cam16,
                $($params)*
            );
            assert_relative_eq!(
                Cam16Jmh::from($cam16).into_full(Parameters::<StaticWp<D65>, _>::TEST_DEFAULTS),
                $cam16,
                $($params)*
            );
            assert_relative_eq!(
                Cam16Jsh::from($cam16).into_full(Parameters::<StaticWp<D65>, _>::TEST_DEFAULTS),
                $cam16,
                $($params)*
            );

            assert_relative_eq!(
                Cam16Qch::from($cam16).into_full(Parameters::<StaticWp<D65>, _>::TEST_DEFAULTS),
                $cam16,
                $($params)*
            );
            assert_relative_eq!(
                Cam16Qmh::from($cam16).into_full(Parameters::<StaticWp<D65>, _>::TEST_DEFAULTS),
                $cam16,
                $($params)*
            );
            assert_relative_eq!(
                Cam16Qsh::from($cam16).into_full(Parameters::<StaticWp<D65>, _>::TEST_DEFAULTS),
                $cam16,
                $($params)*
            );
        };
    }

    #[test]
    fn example_blue() {
        // Uses the example color from https://observablehq.com/@jrus/cam16
        let xyz = Srgb::from(0x5588cc).into_linear().into_color_unclamped();
        let cam16: Cam16<f64> = Cam16::from_xyz(xyz, Parameters::TEST_DEFAULTS);
        assert_partial_to_full!(cam16, epsilon = 0.0000000000001);
    }

    #[test]
    fn black() {
        // Checks against the output from https://observablehq.com/@jrus/cam16
        let xyz = Srgb::from(0x000000).into_linear().into_color_unclamped();
        let cam16: Cam16<f64> = Cam16::from_xyz(xyz, Parameters::TEST_DEFAULTS);
        assert_partial_to_full!(cam16);
    }

    #[test]
    fn white() {
        // Checks against the output from https://observablehq.com/@jrus/cam16
        let xyz = Srgb::from(0xffffff).into_linear().into_color_unclamped();
        let cam16: Cam16<f64> = Cam16::from_xyz(xyz, Parameters::TEST_DEFAULTS);
        assert_partial_to_full!(cam16, epsilon = 0.000000000000001);
    }

    #[test]
    fn red() {
        // Checks against the output from https://observablehq.com/@jrus/cam16
        let xyz = Srgb::from(0xff0000).into_linear().into_color_unclamped();
        let cam16: Cam16<f64> = Cam16::from_xyz(xyz, Parameters::TEST_DEFAULTS);
        assert_partial_to_full!(cam16, epsilon = 0.0000000000001);
    }

    #[test]
    fn green() {
        // Checks against the output from https://observablehq.com/@jrus/cam16
        let xyz = Srgb::from(0x00ff00).into_linear().into_color_unclamped();
        let cam16: Cam16<f64> = Cam16::from_xyz(xyz, Parameters::TEST_DEFAULTS);
        assert_partial_to_full!(cam16, epsilon = 0.0000000000001);
    }

    #[test]
    fn blue() {
        // Checks against the output from https://observablehq.com/@jrus/cam16
        let xyz = Srgb::from(0x0000ff).into_linear().into_color_unclamped();
        let cam16: Cam16<f64> = Cam16::from_xyz(xyz, Parameters::TEST_DEFAULTS);
        assert_partial_to_full!(cam16);
    }
}
