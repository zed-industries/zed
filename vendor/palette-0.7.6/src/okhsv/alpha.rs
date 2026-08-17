use super::Okhsv;
use crate::angle::FromAngle;
use crate::hues::OklabHue;
use crate::num::{MinMax, Zero};
use crate::stimulus::FromStimulus;
use crate::Alpha;

/// Okhsv with an alpha component. See the [`Okhsva` implementation in
/// `Alpha`](crate::Alpha#Okhsva).
pub type Okhsva<T = f32> = Alpha<Okhsv<T>, T>;

///<span id="Hsva"></span>[`Hsva`](crate::Hsva) implementations.
impl<T, A> Alpha<Okhsv<T>, A> {
    /// Create an `Okhsv` color with transparency.
    pub fn new<H: Into<OklabHue<T>>>(hue: H, saturation: T, value: T, alpha: A) -> Self {
        Alpha {
            color: Okhsv::new(hue.into(), saturation, value),
            alpha,
        }
    }

    /// Create an `Okhsva` color. This is the same as `Okhsva::new` without the
    /// generic hue type. It's temporary until `const fn` supports traits.
    pub const fn new_const(hue: OklabHue<T>, saturation: T, value: T, alpha: A) -> Self {
        Alpha {
            color: Okhsv::new_const(hue, saturation, value),
            alpha,
        }
    }

    /// Convert into another component type.
    pub fn into_format<U, B>(self) -> Alpha<Okhsv<U>, B>
    where
        U: FromStimulus<T> + FromAngle<T>,
        B: FromStimulus<A>,
    {
        Alpha {
            color: self.color.into_format(),
            alpha: B::from_stimulus(self.alpha),
        }
    }

    /// Convert from another component type.
    pub fn from_format<U, B>(color: Alpha<Okhsv<U>, B>) -> Self
    where
        T: FromStimulus<U> + FromAngle<U>,
        A: FromStimulus<B>,
        U: Zero + MinMax,
    {
        color.into_format()
    }

    /// Convert to a `(hue, saturation, value, alpha)` tuple.
    pub fn into_components(self) -> (OklabHue<T>, T, T, A) {
        (
            self.color.hue,
            self.color.saturation,
            self.color.value,
            self.alpha,
        )
    }

    /// Convert from a `(hue, saturation, value, alpha)` tuple.
    pub fn from_components<H: Into<OklabHue<T>>>(
        (hue, saturation, value, alpha): (H, T, T, A),
    ) -> Self {
        Self::new(hue, saturation, value, alpha)
    }
}
