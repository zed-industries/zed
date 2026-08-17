use crate::hues::OklabHue;
use crate::{angle::FromAngle, stimulus::FromStimulus, Alpha};

use super::Okhsl;

/// Okhsl with an alpha component.
pub type Okhsla<T = f32> = Alpha<Okhsl<T>, T>;

///<span id="Okhsla"></span>[`Okhsla`](crate::Okhsla) implementations.
impl<T, A> Alpha<Okhsl<T>, A> {
    /// Create an `Okhsl` color with transparency.
    pub fn new<H: Into<OklabHue<T>>>(hue: H, saturation: T, lightness: T, alpha: A) -> Self {
        Alpha {
            color: Okhsl::new(hue, saturation, lightness),
            alpha,
        }
    }

    /// Create an `Okhsla` color. This is the same as `Okhsla::new` without the
    /// generic hue type. It's temporary until `const fn` supports traits.
    pub const fn new_const(hue: OklabHue<T>, saturation: T, lightness: T, alpha: A) -> Self {
        Alpha {
            color: Okhsl::new_const(hue, saturation, lightness),
            alpha,
        }
    }

    /// Convert into another component type.
    pub fn into_format<U, B>(self) -> Alpha<Okhsl<U>, B>
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
    pub fn from_format<U, B>(color: Alpha<Okhsl<U>, B>) -> Self
    where
        T: FromStimulus<U> + FromAngle<U>,
        A: FromStimulus<B>,
    {
        color.into_format()
    }

    /// Convert to a `(hue, saturation, lightness, alpha)` tuple.
    pub fn into_components(self) -> (OklabHue<T>, T, T, A) {
        (
            self.color.hue,
            self.color.saturation,
            self.color.lightness,
            self.alpha,
        )
    }

    /// Convert from a `(hue, saturation, lightness, alpha)` tuple.
    pub fn from_components<H: Into<OklabHue<T>>>(
        (hue, saturation, lightness, alpha): (H, T, T, A),
    ) -> Self {
        Self::new(hue, saturation, lightness, alpha)
    }
}
