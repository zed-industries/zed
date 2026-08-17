//! Types for the Oklch color space.

pub use alpha::Oklcha;

use crate::{
    bool_mask::HasBoolMask,
    convert::FromColorUnclamped,
    num::{Hypot, One, Zero},
    white_point::D65,
    GetHue, Oklab, OklabHue,
};

pub use self::properties::Iter;

#[cfg(feature = "random")]
pub use self::random::UniformOklch;

mod alpha;
mod properties;
#[cfg(feature = "random")]
mod random;

/// Oklch, a polar version of [Oklab].
///
/// It is Oklab’s equivalent of [CIE L\*C\*h°](crate::Lch).
///
/// It's a cylindrical color space, like [HSL](crate::Hsl) and
/// [HSV](crate::Hsv). This gives it the same ability to directly change
/// the hue and colorfulness of a color, while preserving other visual aspects.
///
/// It assumes a D65 whitepoint and normal well-lit viewing conditions,
/// like Oklab.
#[derive(Debug, Copy, Clone, ArrayCast, FromColorUnclamped, WithAlpha)]
#[cfg_attr(feature = "serializing", derive(Serialize, Deserialize))]
#[palette(
    palette_internal,
    white_point = "D65",
    component = "T",
    skip_derives(Oklab, Oklch)
)]
#[repr(C)]
pub struct Oklch<T = f32> {
    /// L is the lightness of the color. 0 gives absolute black and 1 gives the brightest white.
    pub l: T,

    /// `chroma` is the colorfulness of the color.
    /// A color with `chroma == 0` is a shade of grey.
    /// In a transformation from `Oklab` it is computed as `chroma = √(a²+b²)`.
    /// `chroma` is unbounded
    pub chroma: T,

    /// h is the hue of the color, in degrees. Decides if it's red, blue, purple,
    /// etc.
    #[palette(unsafe_same_layout_as = "T")]
    pub hue: OklabHue<T>,
}

impl<T> Oklch<T> {
    /// Create an `Oklch` color.
    pub fn new<H: Into<OklabHue<T>>>(l: T, chroma: T, hue: H) -> Self {
        Oklch {
            l,
            chroma,
            hue: hue.into(),
        }
    }

    /// Create an `Oklch` color. This is the same as `Oklch::new` without the
    /// generic hue type. It's temporary until `const fn` supports traits.
    pub const fn new_const(l: T, chroma: T, hue: OklabHue<T>) -> Self {
        Oklch { l, chroma, hue }
    }

    /// Convert to a `(L, C, h)` tuple.
    pub fn into_components(self) -> (T, T, OklabHue<T>) {
        (self.l, self.chroma, self.hue)
    }

    /// Convert from a `(L, C, h)` tuple.
    pub fn from_components<H: Into<OklabHue<T>>>((l, chroma, hue): (T, T, H)) -> Self {
        Self::new(l, chroma, hue)
    }
}

impl<T> Oklch<T>
where
    T: Zero + One,
{
    /// Return the `l` value minimum.
    pub fn min_l() -> T {
        T::zero()
    }

    /// Return the `l` value maximum.
    pub fn max_l() -> T {
        T::one()
    }

    /// Return the `chroma` value minimum.
    pub fn min_chroma() -> T {
        T::zero()
    }
}

impl_reference_component_methods_hue!(Oklch, [l, chroma]);
impl_struct_of_arrays_methods_hue!(Oklch, [l, chroma]);

impl<T> FromColorUnclamped<Oklch<T>> for Oklch<T> {
    fn from_color_unclamped(color: Oklch<T>) -> Self {
        color
    }
}

impl<T> FromColorUnclamped<Oklab<T>> for Oklch<T>
where
    T: Hypot + Clone,
    Oklab<T>: GetHue<Hue = OklabHue<T>>,
{
    fn from_color_unclamped(color: Oklab<T>) -> Self {
        let hue = color.get_hue();
        let chroma = color.get_chroma();
        Oklch::new(color.l, chroma, hue)
    }
}

impl_tuple_conversion_hue!(Oklch as (T, T, H), OklabHue);

impl<T> HasBoolMask for Oklch<T>
where
    T: HasBoolMask,
{
    type Mask = T::Mask;
}

impl<T> Default for Oklch<T>
where
    T: Zero + One,
    OklabHue<T>: Default,
{
    fn default() -> Oklch<T> {
        Oklch::new(Self::min_l(), Self::min_chroma(), OklabHue::default())
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl<T> bytemuck::Zeroable for Oklch<T> where T: bytemuck::Zeroable {}

#[cfg(feature = "bytemuck")]
unsafe impl<T> bytemuck::Pod for Oklch<T> where T: bytemuck::Pod {}

#[cfg(test)]
mod test {
    use crate::Oklch;

    test_convert_into_from_xyz!(Oklch);

    #[cfg(feature = "approx")]
    mod conversion {
        use crate::{
            convert::FromColorUnclamped,
            visual::{VisualColor, VisuallyEqual},
            LinSrgb, Oklab, Oklch, Srgb,
        };

        #[cfg_attr(miri, ignore)]
        #[test]
        fn test_roundtrip_oklch_oklab_is_original() {
            let colors = [
                (
                    "red",
                    Oklab::from_color_unclamped(LinSrgb::new(1.0, 0.0, 0.0)),
                ),
                (
                    "green",
                    Oklab::from_color_unclamped(LinSrgb::new(0.0, 1.0, 0.0)),
                ),
                (
                    "cyan",
                    Oklab::from_color_unclamped(LinSrgb::new(0.0, 1.0, 1.0)),
                ),
                (
                    "magenta",
                    Oklab::from_color_unclamped(LinSrgb::new(1.0, 0.0, 1.0)),
                ),
                (
                    "black",
                    Oklab::from_color_unclamped(LinSrgb::new(0.0, 0.0, 0.0)),
                ),
                (
                    "grey",
                    Oklab::from_color_unclamped(LinSrgb::new(0.5, 0.5, 0.5)),
                ),
                (
                    "yellow",
                    Oklab::from_color_unclamped(LinSrgb::new(1.0, 1.0, 0.0)),
                ),
                (
                    "blue",
                    Oklab::from_color_unclamped(LinSrgb::new(0.0, 0.0, 1.0)),
                ),
                (
                    "white",
                    Oklab::from_color_unclamped(LinSrgb::new(1.0, 1.0, 1.0)),
                ),
            ];

            const EPSILON: f64 = 1e-14;

            for (name, color) in colors {
                let rgb: Srgb<u8> = Srgb::<f64>::from_color_unclamped(color).into_format();
                println!(
                    "\n\
                    roundtrip of {} (#{:x} / {:?})\n\
                    =================================================",
                    name, rgb, color
                );

                println!("Color is white: {}", color.is_white(EPSILON));

                let oklch = Oklch::from_color_unclamped(color);
                println!("Oklch: {:?}", oklch);
                let roundtrip_color = Oklab::from_color_unclamped(oklch);
                assert!(
                    Oklab::visually_eq(roundtrip_color, color, EPSILON),
                    "'{}' failed.\n{:?}\n!=\n{:?}",
                    name,
                    roundtrip_color,
                    color
                );
            }
        }
    }

    #[test]
    fn ranges() {
        // chroma: 0.0 => infinity
        assert_ranges! {
            Oklch< f64>;
            clamped {
                l: 0.0 => 1.0
            }
            clamped_min {}
            unclamped {
                hue: 0.0 => 360.0
            }
        }
    }

    #[test]
    fn check_min_max_components() {
        assert_eq!(Oklch::<f32>::min_l(), 0.0);
        assert_eq!(Oklch::<f32>::max_l(), 1.0);
        assert_eq!(Oklch::<f32>::min_chroma(), 0.0);
    }

    #[cfg(feature = "serializing")]
    #[test]
    fn serialize() {
        let serialized = ::serde_json::to_string(&Oklch::new(0.3, 0.8, 0.1)).unwrap();

        assert_eq!(serialized, r#"{"l":0.3,"chroma":0.8,"hue":0.1}"#);
    }

    #[cfg(feature = "serializing")]
    #[test]
    fn deserialize() {
        let deserialized: Oklch =
            ::serde_json::from_str(r#"{"l":0.3,"chroma":0.8,"hue":0.1}"#).unwrap();

        assert_eq!(deserialized, Oklch::new(0.3, 0.8, 0.1));
    }

    struct_of_arrays_tests!(
        Oklch[l, chroma, hue],
        super::Oklcha::new(0.1f32, 0.2, 0.3, 0.4),
        super::Oklcha::new(0.2, 0.3, 0.4, 0.5),
        super::Oklcha::new(0.3, 0.4, 0.5, 0.6)
    );

    test_uniform_distribution! {
        Oklch<f32> as crate::Oklab {
            l: (0.0, 1.0),
            a: (-0.7, 0.7),
            b: (-0.7, 0.7),
        },
        min: Oklch::new(0.0f32, 0.0, 0.0),
        max: Oklch::new(1.0, 1.0, 360.0)
    }
}
