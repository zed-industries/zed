//! Types for the Okhwb color space.

use core::fmt::Debug;

pub use alpha::Okhwba;

use crate::{
    angle::FromAngle,
    convert::FromColorUnclamped,
    num::{Arithmetics, One},
    stimulus::{FromStimulus, Stimulus},
    white_point::D65,
    HasBoolMask, Okhsv, OklabHue,
};

pub use self::properties::Iter;

#[cfg(feature = "random")]
pub use self::random::UniformOkhwb;

mod alpha;
mod properties;
#[cfg(feature = "random")]
mod random;
#[cfg(test)]
#[cfg(feature = "approx")]
mod visual_eq;

/// A Hue/Whiteness/Blackness representation of [`Oklab`][crate::Oklab] in the
/// `sRGB` color space, similar to [`Hwb`][crate::Okhwb].
#[derive(Debug, Copy, Clone, ArrayCast, FromColorUnclamped, WithAlpha)]
#[cfg_attr(feature = "serializing", derive(Serialize, Deserialize))]
#[palette(
    palette_internal,
    white_point = "D65",
    component = "T",
    skip_derives(Okhwb, Okhsv)
)]
#[repr(C)]
pub struct Okhwb<T = f32> {
    /// The hue of the color, in degrees of a circle.
    ///
    /// For fully saturated, bright colors
    /// * 0° corresponds to a kind of magenta-pink (RBG #ff0188),
    /// * 90° to a kind of yellow (RBG RGB #ffcb00)
    /// * 180° to a kind of cyan (RBG #00ffe1) and
    /// * 240° to a kind of blue (RBG #00aefe).
    ///
    /// For s == 0 or v == 0, the hue is irrelevant.
    #[palette(unsafe_same_layout_as = "T")]
    pub hue: OklabHue<T>,

    /// The amount of white, mixed in the pure hue, ranging from `0.0` to `1.0`.
    /// `0.0` produces pure, possibly black color. `1.0` a white or grey.
    pub whiteness: T,

    /// The amount of black, mixed in the pure hue, ranging from `0.0` to `1.0`.
    /// `0.0` produces a pure bright or whitened color. `1.0` a black or grey.
    pub blackness: T,
}

impl<T> Okhwb<T> {
    /// Create an `Okhwb` color.
    pub fn new<H: Into<OklabHue<T>>>(hue: H, whiteness: T, blackness: T) -> Self {
        let hue = hue.into();
        Self {
            hue,
            whiteness,
            blackness,
        }
    }

    /// Create an `Okhwb` color. This is the same as `Okhwb::new` without the
    /// generic hue type. It's temporary until `const fn` supports traits.
    pub const fn new_const(hue: OklabHue<T>, whiteness: T, blackness: T) -> Self {
        Self {
            hue,
            whiteness,
            blackness,
        }
    }
    /// Convert into another component type.
    pub fn into_format<U>(self) -> Okhwb<U>
    where
        U: FromStimulus<T> + FromAngle<T>,
    {
        Okhwb {
            hue: self.hue.into_format(),
            whiteness: U::from_stimulus(self.whiteness),
            blackness: U::from_stimulus(self.blackness),
        }
    }
    /// Convert to a `(h, w, b)` tuple.
    pub fn into_components(self) -> (OklabHue<T>, T, T) {
        (self.hue, self.whiteness, self.blackness)
    }

    /// Convert from a `(h, w, b)` tuple.
    pub fn from_components<H: Into<OklabHue<T>>>((hue, whiteness, blackness): (H, T, T)) -> Self {
        Self::new(hue, whiteness, blackness)
    }
}

impl<T> Okhwb<T>
where
    T: Stimulus,
{
    /// Return the `whiteness` value minimum.
    pub fn min_whiteness() -> T {
        T::zero()
    }

    /// Return the `whiteness` value maximum.
    pub fn max_whiteness() -> T {
        T::max_intensity()
    }

    /// Return the `blackness` value minimum.
    pub fn min_blackness() -> T {
        T::zero()
    }

    /// Return the `blackness` value maximum.
    pub fn max_blackness() -> T {
        T::max_intensity()
    }
}

impl_reference_component_methods_hue!(Okhwb, [whiteness, blackness]);
impl_struct_of_arrays_methods_hue!(Okhwb, [whiteness, blackness]);

impl<T> FromColorUnclamped<Okhsv<T>> for Okhwb<T>
where
    T: One + Arithmetics,
{
    /// Converts `lab` to `Okhwb` in the bounds of sRGB.
    fn from_color_unclamped(hsv: Okhsv<T>) -> Self {
        // See <https://bottosson.github.io/posts/colorpicker/#okhwb>.
        Self {
            hue: hsv.hue,
            whiteness: (T::one() - hsv.saturation) * &hsv.value,
            blackness: T::one() - hsv.value,
        }
    }
}

impl<T> HasBoolMask for Okhwb<T>
where
    T: HasBoolMask,
{
    type Mask = T::Mask;
}

impl<T> Default for Okhwb<T>
where
    T: Stimulus,
    OklabHue<T>: Default,
{
    fn default() -> Okhwb<T> {
        Okhwb::new(
            OklabHue::default(),
            Self::min_whiteness(),
            Self::max_blackness(),
        )
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl<T> bytemuck::Zeroable for Okhwb<T> where T: bytemuck::Zeroable {}

#[cfg(feature = "bytemuck")]
unsafe impl<T> bytemuck::Pod for Okhwb<T> where T: bytemuck::Pod {}

#[cfg(test)]
mod tests {
    use crate::Okhwb;

    test_convert_into_from_xyz!(Okhwb);

    #[cfg(feature = "approx")]
    mod conversion {
        use crate::{
            convert::FromColorUnclamped, encoding, rgb::Rgb, visual::VisuallyEqual, LinSrgb, Okhsv,
            Okhwb, Oklab,
        };

        #[cfg_attr(miri, ignore)]
        #[test]
        fn test_roundtrip_okhwb_oklab_is_original() {
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
                    "white",
                    Oklab::from_color_unclamped(LinSrgb::new(1.0, 1.0, 1.0)),
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
            ];

            const EPSILON: f64 = 1e-14;

            for (name, color) in colors {
                let rgb: Rgb<encoding::Srgb, u8> =
                    crate::Srgb::<f64>::from_color_unclamped(color).into_format();
                println!(
                    "\n\
                    roundtrip of {} (#{:x} / {:?})\n\
                    =================================================",
                    name, rgb, color
                );

                let okhsv = Okhsv::from_color_unclamped(color);
                println!("Okhsv: {:?}", okhsv);
                let okhwb_from_okhsv = Okhwb::from_color_unclamped(okhsv);
                let okhwb = Okhwb::from_color_unclamped(color);
                println!("Okhwb: {:?}", okhwb);
                assert!(
                Okhwb::visually_eq(okhwb, okhwb_from_okhsv, EPSILON),
                "Okhwb \n{:?} is not visually equal to Okhwb from Okhsv \n{:?}\nwithin EPSILON {}",
                okhwb,
                okhwb_from_okhsv,
                EPSILON
            );
                let okhsv_from_okhwb = Okhsv::from_color_unclamped(okhwb);
                assert!(
                Okhsv::visually_eq(okhsv, okhsv_from_okhwb, EPSILON),
                "Okhsv \n{:?} is not visually equal to Okhsv from Okhsv from Okhwb \n{:?}\nwithin EPSILON {}",
                okhsv,
                okhsv_from_okhwb, EPSILON
            );

                let roundtrip_color = Oklab::from_color_unclamped(okhwb);
                let oklab_from_okhsv = Oklab::from_color_unclamped(okhsv);
                assert!(
                    Oklab::visually_eq(roundtrip_color, oklab_from_okhsv, EPSILON),
                    "roundtrip color \n{:?} does not match \n{:?}\nwithin EPSILON {}",
                    roundtrip_color,
                    oklab_from_okhsv,
                    EPSILON
                );
                assert!(
                    Oklab::visually_eq(roundtrip_color, color, EPSILON),
                    "'{}' failed.\n\
                {:?}\n\
                !=\n\
                \n{:?}\n",
                    name,
                    roundtrip_color,
                    color
                );
            }
        }
    }

    struct_of_arrays_tests!(
        Okhwb[hue, whiteness, blackness],
        super::Okhwba::new(0.1f32, 0.2, 0.3, 0.4),
        super::Okhwba::new(0.2, 0.3, 0.4, 0.5),
        super::Okhwba::new(0.3, 0.4, 0.5, 0.6)
    );
}
