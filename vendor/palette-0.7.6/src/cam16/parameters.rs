use core::marker::PhantomData;

use crate::{
    bool_mask::LazySelect,
    num::{
        Abs, Arithmetics, Clamp, Exp, FromScalar, One, PartialCmp, Powf, Real, Signum, Sqrt, Zero,
    },
    white_point::{self, WhitePoint},
    Xyz,
};

/// Parameters for CAM16 that describe the viewing conditions.
///
/// These parameters describe the viewing conditions for a more accurate color
/// appearance metric. The CAM16 attributes and derived values are only really
/// comparable if they were calculated with the same parameters. The parameters
/// are, however, too dynamic to all be part of the type parameters of
/// [`Cam16`][super::Cam16].
///
/// The default values are mostly a "blank slate", with a couple of educated
/// guesses. Be sure to at least customize the luminances according to the
/// expected environment:
///
/// ```
/// use palette::{Srgb, Xyz, IntoColor, cam16::{Parameters, Surround, Cam16}};
///
/// // 40 nits, 50% background luminance and a dim surrounding:
/// let mut example_parameters = Parameters::default_static_wp(40.0);
/// example_parameters.background_luminance = 0.5;
/// example_parameters.surround = Surround::Dim;
///
/// let example_color_xyz = Srgb::from(0x5588cc).into_linear().into_color();
/// let cam16: Cam16<f64> = Cam16::from_xyz(example_color_xyz, example_parameters);
/// ```
///
/// See also Moroney (2000) [Usage Guidelines for CIECAM97s][moroney_2000] for
/// more information and advice on how to customize these parameters.
///
/// [moroney_2000]:
///     https://www.imaging.org/common/uploaded%20files/pdfs/Papers/2000/PICS-0-81/1611.pdf
#[derive(Clone, Copy)]
#[non_exhaustive]
pub struct Parameters<WpParam, T> {
    /// White point of the test illuminant, *X<sub>w</sub>* *Y<sub>w</sub>*
    /// *Z<sub>w</sub>*. *Y<sub>w</sub>* should be normalized to 1.0.
    ///
    /// Defaults to `Wp` when it implements [`WhitePoint`]. It can also be set
    /// to a custom value if `Wp` results in the wrong white point.
    pub white_point: WpParam,

    /// The average luminance of the environment (test adapting field)
    /// (*L<sub>A</sub>*) in *cd/m<sup>2</sup>* (nits).
    ///
    /// Under a “gray world” assumption this is 20% of the luminance of the
    /// reference white.
    pub adapting_luminance: T,

    /// The luminance factor of the background (*Y<sub>b</sub>*), on a scale
    /// from `0.0` to `1.0` (relative to *Y<sub>w</sub>* = 1.0).
    ///
    /// Defaults to `0.2`, medium grey.
    pub background_luminance: T,

    /// A description of the peripheral area, with a value from 0% to 20%. Any
    /// value outside that range will be clamped to 0% or 20%. It has presets
    /// for "dark", "dim" and "average".
    ///
    /// Defaults to "average" (20%).
    pub surround: Surround<T>,

    /// The degree of discounting of (or adaptation to) the reference
    /// illuminant. Defaults to `Auto`, making the degree of discounting depend
    /// on the other parameters, but can be customized if necessary.
    pub discounting: Discounting<T>,
}

impl<WpParam, T> Parameters<WpParam, T>
where
    WpParam: WhitePointParameter<T>,
{
    fn into_any_white_point(self) -> Parameters<Xyz<white_point::Any, T>, T> {
        Parameters {
            white_point: self.white_point.into_xyz(),
            adapting_luminance: self.adapting_luminance,
            background_luminance: self.background_luminance,
            surround: self.surround,
            discounting: self.discounting,
        }
    }
}

impl<Wp, T> Parameters<StaticWp<Wp>, T> {
    /// Creates a new set of parameters with a static white point and their
    /// default values set.
    ///
    /// These parameters may need to be further customized according to the
    /// viewing conditions.
    #[inline]
    pub fn default_static_wp(adapting_luminance: T) -> Self
    where
        T: Real,
    {
        Self {
            white_point: StaticWp(PhantomData),
            adapting_luminance,
            background_luminance: T::from_f64(0.2),
            surround: Surround::Average,
            discounting: Discounting::Auto,
        }
    }
}

impl<T> Parameters<Xyz<white_point::Any, T>, T> {
    /// Creates a new set of parameters with a dynamic white point and their
    /// default values set.
    ///
    /// These parameters may need to be further customized according to the
    /// viewing conditions.
    #[inline]
    pub fn default_dynamic_wp(white_point: Xyz<white_point::Any, T>, adapting_luminance: T) -> Self
    where
        T: Real,
    {
        Self {
            white_point,
            adapting_luminance,
            background_luminance: T::from_f64(0.2),
            surround: Surround::Average,
            discounting: Discounting::Auto,
        }
    }
}

impl<WpParam, T> Parameters<WpParam, T> {
    /// Pre-bakes the parameters to avoid repeating parts of the calculaitons
    /// when converting to and from CAM16.
    pub fn bake(self) -> BakedParameters<WpParam, T>
    where
        BakedParameters<WpParam, T>: From<Self>,
    {
        self.into()
    }
}

#[cfg(all(test, feature = "approx"))]
impl<Wp> Parameters<StaticWp<Wp>, f64> {
    /// Only used in unit tests and corresponds to the defaults from https://observablehq.com/@jrus/cam16.
    pub(crate) const TEST_DEFAULTS: Self = Self {
        white_point: StaticWp(PhantomData),
        adapting_luminance: 40.0f64,
        background_luminance: 0.2f64, // 20 / 100, since our XYZ is in the range from 0.0 to 1.0
        surround: Surround::Average,
        discounting: Discounting::Auto,
    };
}

/// Pre-calculated variables for CAM16, that only depend on the viewing
/// conditions.
///
/// Derived from [`Parameters`], the `BakedParameters` can help reducing the
/// amount of repeated work required for converting multiple colors.
pub struct BakedParameters<WpParam, T> {
    pub(crate) inner: super::math::DependentParameters<T>,
    white_point: PhantomData<WpParam>,
}

impl<WpParam, T> Clone for BakedParameters<WpParam, T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            white_point: PhantomData,
        }
    }
}

impl<WpParam, T> Copy for BakedParameters<WpParam, T> where T: Copy {}

impl<WpParam, T> From<Parameters<WpParam, T>> for BakedParameters<WpParam, T>
where
    WpParam: WhitePointParameter<T>,
    T: Real
        + FromScalar<Scalar = T>
        + One
        + Zero
        + Clamp
        + PartialCmp
        + Arithmetics
        + Powf
        + Sqrt
        + Exp
        + Abs
        + Signum
        + Clone,
    T::Mask: LazySelect<T>,
{
    fn from(value: Parameters<WpParam, T>) -> Self {
        Self {
            inner: super::math::prepare_parameters(value.into_any_white_point()),
            white_point: PhantomData,
        }
    }
}

/// A description of the peripheral area.
#[derive(Clone, Copy)]
#[non_exhaustive]
pub enum Surround<T> {
    /// Represents a dark room, such as a movie theatre. Corresponds to a
    /// surround value of 0%.
    Dark,

    /// Represents a dimly lit room with a bright TV or monitor. Corresponds to
    /// a surround value of 10%.
    Dim,

    /// Represents a surface color, such as a print on a 20% reflective,
    /// uniformly lit background surface. Corresponds to a surround value of
    /// 20%.
    Average,

    /// Any custom value from 0% to 20%. Any value outside that range will be
    /// clamped to either `0.0` or `20.0`.
    Percent(T),
}

impl<T> Surround<T> {
    pub(crate) fn into_percent(self) -> T
    where
        T: Real + Clamp,
    {
        match self {
            Surround::Dark => T::from_f64(0.0),
            Surround::Dim => T::from_f64(10.0),
            Surround::Average => T::from_f64(20.0),
            Surround::Percent(value) => value.clamp(T::from_f64(0.0), T::from_f64(20.0)),
        }
    }
}

/// The degree of discounting of (or adaptation to) the illuminant.
///
/// See also: <https://en.wikipedia.org/wiki/CIECAM02#CAT02>.
#[derive(Clone, Copy)]
#[non_exhaustive]
pub enum Discounting<T> {
    /// Uses luminance levels and surround conditions to calculate the
    /// discounting, using the original CIECAM16 *D* function. Ranges from
    /// `0.65` to `1.0`.
    Auto,

    /// A value between `0.0` and `1.0`, where `0.0` represents no adaptation,
    /// and `1.0` represents that the observer's vision is fully adapted to the
    /// illuminant. Values outside that range will be clamped.
    Custom(T),
}

/// A trait for types that can be used as white point parameters in
/// [`Parameters`].
pub trait WhitePointParameter<T> {
    /// The static representation of this white point, or [`white_point::Any`]
    /// if it's dynamic.
    type StaticWp;

    /// Returns the XYZ value for this white point.
    fn into_xyz(self) -> Xyz<white_point::Any, T>;
}

impl<T> WhitePointParameter<T> for Xyz<white_point::Any, T> {
    type StaticWp = white_point::Any;

    fn into_xyz(self) -> Xyz<white_point::Any, T> {
        self
    }
}

/// Represents a static white point in [`Parameters`], as opposed to a dynamic
/// [`Xyz`] value.
pub struct StaticWp<Wp>(PhantomData<Wp>);

impl<T, Wp> WhitePointParameter<T> for StaticWp<Wp>
where
    Wp: WhitePoint<T>,
{
    type StaticWp = Wp;

    fn into_xyz(self) -> Xyz<white_point::Any, T> {
        Wp::get_xyz()
    }
}

impl<Wp> Clone for StaticWp<Wp> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<Wp> Copy for StaticWp<Wp> {}
