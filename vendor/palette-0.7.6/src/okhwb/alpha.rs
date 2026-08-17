use crate::angle::FromAngle;
use crate::okhwb::Okhwb;
use crate::stimulus::FromStimulus;
use crate::{Alpha, OklabHue};

/// Okhwb with an alpha component. See the [`Okhwba` implementation in
/// `Alpha`](crate::Alpha#Okhwba).
pub type Okhwba<T = f32> = Alpha<Okhwb<T>, T>;

///<span id="Okhwba"></span>[`Okhwba`](crate::Okhwba) implementations.
impl<T, A> Alpha<Okhwb<T>, A> {
    /// Create an `Okhwb` color with transparency.
    pub fn new<H: Into<OklabHue<T>>>(hue: H, whiteness: T, blackness: T, alpha: A) -> Self {
        Alpha {
            color: Okhwb::new(hue.into(), whiteness, blackness),
            alpha,
        }
    }

    /// Create an `Okhwba` color. This is the same as `Okhwba::new` without the
    /// generic hue type. It's temporary until `const fn` supports traits.
    pub const fn new_const(hue: OklabHue<T>, whiteness: T, blackness: T, alpha: A) -> Self {
        Alpha {
            color: Okhwb::new_const(hue, whiteness, blackness),
            alpha,
        }
    }

    /// Convert into another component type.
    pub fn into_format<U, B>(self) -> Alpha<Okhwb<U>, B>
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
    pub fn from_format<U, B>(color: Alpha<Okhwb<U>, B>) -> Self
    where
        T: FromStimulus<U> + FromAngle<U>,
        A: FromStimulus<B>,
    {
        color.into_format()
    }

    /// Convert to a `(hue, whiteness, blackness, alpha)` tuple.
    pub fn into_components(self) -> (OklabHue<T>, T, T, A) {
        (
            self.color.hue,
            self.color.whiteness,
            self.color.blackness,
            self.alpha,
        )
    }

    /// Convert from a `(hue, whiteness, blackness, alpha)` tuple.
    pub fn from_components<H: Into<OklabHue<T>>>(
        (hue, whiteness, blackness, alpha): (H, T, T, A),
    ) -> Self {
        Self::new(hue, whiteness, blackness, alpha)
    }
}
