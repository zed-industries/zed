//! Types for luma and luminance (grayscale) values.

pub mod channels;
#[allow(clippy::module_inception)]
mod luma;

use crate::encoding::{Gamma, Linear, Srgb};
use crate::white_point::D65;

pub use self::luma::{Iter, Luma, Lumaa};

/// sRGB encoded luminance.
pub type SrgbLuma<T = f32> = Luma<Srgb, T>;
/// sRGB encoded luminance with an alpha component.
pub type SrgbLumaa<T = f32> = Lumaa<Srgb, T>;

/// Linear luminance.
#[doc(alias = "linear")]
pub type LinLuma<Wp = D65, T = f32> = Luma<Linear<Wp>, T>;
/// Linear luminance with an alpha component.
#[doc(alias = "linear")]
pub type LinLumaa<Wp = D65, T = f32> = Lumaa<Linear<Wp>, T>;

/// Gamma 2.2 encoded luminance.
pub type GammaLuma<T = f32> = Luma<Gamma<D65>, T>;
/// Gamma 2.2 encoded luminance with an alpha component.
pub type GammaLumaa<T = f32> = Lumaa<Gamma<D65>, T>;

/// A white point and a transfer function.
pub trait LumaStandard {
    /// The white point of the color space.
    type WhitePoint;

    /// The transfer function for the luminance component.
    type TransferFn;
}

impl<Wp, Tf> LumaStandard for (Wp, Tf) {
    type WhitePoint = Wp;
    type TransferFn = Tf;
}

/// A packed representation of Luma+Alpha in LA order.
pub type PackedLumaa<P = u16> = crate::cast::Packed<channels::La, P>;

/// A packed representation of Luma+Alpha in AL order.
pub type PackedAluma<P = u16> = crate::cast::Packed<channels::Al, P>;
