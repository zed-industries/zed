use crate::{Alpha, OklabHue};

use super::Oklch;

/// Oklch with an alpha component. See the [`Oklcha` implementation in
/// `Alpha`](crate::Alpha#Oklcha).
pub type Oklcha<T = f32> = Alpha<Oklch<T>, T>;

///<span id="Oklcha"></span>[`Oklcha`](crate::Oklcha) implementations.
impl<T, A> Alpha<Oklch<T>, A> {
    /// Create an Oklch color with transparency.
    pub fn new<H: Into<OklabHue<T>>>(l: T, chroma: T, hue: H, alpha: A) -> Self {
        Alpha {
            color: Oklch::new(l, chroma, hue),
            alpha,
        }
    }

    /// Create an `Oklcha` color. This is the same as `Oklcha::new` without the
    /// generic hue type. It's temporary until `const fn` supports traits.
    pub const fn new_const(l: T, chroma: T, hue: OklabHue<T>, alpha: A) -> Self {
        Alpha {
            color: Oklch::new_const(l, chroma, hue),
            alpha,
        }
    }

    /// Convert to a `(L, C, h, alpha)` tuple.
    pub fn into_components(self) -> (T, T, OklabHue<T>, A) {
        (self.color.l, self.color.chroma, self.color.hue, self.alpha)
    }

    /// Convert from a `(L, C, h, alpha)` tuple.
    pub fn from_components<H: Into<OklabHue<T>>>((l, chroma, hue, alpha): (T, T, H, A)) -> Self {
        Self::new(l, chroma, hue, alpha)
    }
}
