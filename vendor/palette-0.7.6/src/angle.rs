//! Traits for working with angular values, such as for in hues.

use crate::{
    bool_mask::HasBoolMask,
    num::{Real, Round},
};

#[cfg(feature = "wide")]
mod wide;

/// Represents types that can express half of a rotation (i.e. 180 degrees).
pub trait HalfRotation {
    /// Return a value that represents half of a rotation (i.e. 180 degrees).
    #[must_use]
    fn half_rotation() -> Self;
}

/// Represents types that can express a full rotation (i.e. 360 degrees).
pub trait FullRotation {
    /// Return a value that represents a full rotation (i.e. 360 degrees).
    #[must_use]
    fn full_rotation() -> Self;
}

/// Angle values that are real numbers and can represent both radians and
/// degrees.
pub trait RealAngle: Real {
    /// Consider `self` to be radians and convert it to degrees.
    #[must_use]
    fn radians_to_degrees(self) -> Self;

    /// Consider `self` to be degrees and convert it to radians.
    #[must_use]
    fn degrees_to_radians(self) -> Self;
}

/// Angular equality, where 0 degrees and 360 degrees are equal.
pub trait AngleEq: HasBoolMask {
    /// Check if `self` and `other` represent the same angle on a circle.
    #[must_use]
    fn angle_eq(&self, other: &Self) -> Self::Mask;
}

/// Angle types that can represent the full circle using positive and negative
/// values.
pub trait SignedAngle {
    /// Normalize `self` to a range corresponding to -180 to 180 degrees.
    #[must_use]
    fn normalize_signed_angle(self) -> Self;
}

/// Angle types that can represent the full circle as positive values.
pub trait UnsignedAngle {
    /// Normalize `self` to a range corresponding to 0 to 360 degrees.
    #[must_use]
    fn normalize_unsigned_angle(self) -> Self;
}

/// Performs value-to-value conversion between angle types. See also [`IntoAngle`].
pub trait FromAngle<T> {
    /// Performs a conversion from `angle`.
    fn from_angle(angle: T) -> Self;
}

impl<T> FromAngle<T> for T {
    #[inline]
    fn from_angle(angle: Self) -> Self {
        angle
    }
}

/// Performs value-to-value conversion between angle types. See also [`IntoAngle`].
pub trait IntoAngle<T> {
    /// Performs a conversion into `T`.
    fn into_angle(self) -> T;
}

impl<T, U> IntoAngle<U> for T
where
    U: FromAngle<T>,
{
    #[inline]
    fn into_angle(self) -> U {
        U::from_angle(self)
    }
}

macro_rules! impl_angle_float {
    ($($ty: ident),+) => {
        $(
            impl HalfRotation for $ty {
                #[inline]
                fn half_rotation() -> Self {
                    180.0
                }
            }

            impl FullRotation for $ty {
                #[inline]
                fn full_rotation() -> Self {
                    360.0
                }
            }

            impl RealAngle for $ty {
                #[inline]
                fn degrees_to_radians(self) -> Self {
                    self.to_radians()
                }

                #[inline]
                fn radians_to_degrees(self) -> Self {
                    self.to_degrees()
                }
            }

            impl AngleEq for $ty {
                #[inline]
                fn angle_eq(&self, other: &Self) -> bool {
                    self.normalize_unsigned_angle() == other.normalize_unsigned_angle()
                }
            }

            impl SignedAngle for $ty {
                #[inline]
                fn normalize_signed_angle(self) -> Self {
                    self - Round::ceil(((self + 180.0) / 360.0) - 1.0) * 360.0
                }
            }

            impl UnsignedAngle for $ty {
                #[inline]
                fn normalize_unsigned_angle(self) -> Self {
                    self - (Round::floor(self / 360.0) * 360.0)
                }
            }
        )+
    };
}

macro_rules! impl_from_angle_float {
    ($ty: ident to $other_ty: ident) => {
        impl FromAngle<$other_ty> for $ty {
            #[inline]
            fn from_angle(angle: $other_ty) -> Self {
                angle as $ty
            }
        }
    };
}

macro_rules! impl_from_angle_u8 {
    ($($float_ty: ident),*) => {
        $(
            impl FromAngle<u8> for $float_ty {
                #[inline]
                fn from_angle(angle: u8) -> Self {
                    (angle as $float_ty / 256.0) * Self::full_rotation()
                }
            }

            impl FromAngle<$float_ty> for u8 {
                #[inline]
                fn from_angle(angle: $float_ty) -> Self {
                    let normalized = angle.normalize_unsigned_angle() / $float_ty::full_rotation();
                    let rounded = (normalized * 256.0).round();

                    if rounded > 255.5 {
                        0
                    } else {
                        rounded as u8
                    }
                }
            }
        )*
    };
}

impl_angle_float!(f32, f64);
impl_from_angle_float!(f32 to f64);
impl_from_angle_float!(f64 to f32);
impl_from_angle_u8!(f32, f64);

impl HalfRotation for u8 {
    #[inline]
    fn half_rotation() -> Self {
        128
    }
}

impl AngleEq for u8 {
    #[inline]
    fn angle_eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl UnsignedAngle for u8 {
    #[inline]
    fn normalize_unsigned_angle(self) -> Self {
        self
    }
}

#[cfg(test)]
mod test {
    use crate::RgbHue;

    #[test]
    fn f32_to_u8() {
        let hue_f32 = RgbHue::new(180.0f32);
        let hue_u8 = hue_f32.into_format::<u8>();
        assert_eq!(hue_u8, RgbHue::new(128u8));
    }

    #[test]
    fn u8_to_f32() {
        let hue_f32 = RgbHue::new(128u8);
        let hue_u8 = hue_f32.into_format::<f32>();
        assert_eq!(hue_u8, RgbHue::new(180.0f32));
    }
}
