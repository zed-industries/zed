#![allow(deprecated)]

use crate::{Affine3A, Mat4, Quat, Vec3, Vec3A, Vec3Swizzles};
use core::ops::Mul;

#[cfg(feature = "rand")]
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

/**
 * A transform containing non-uniform scale, rotation and translation.
 *
 * Scale and translation are stored as `Vec3A` for better performance.
 */
#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(C)]
#[deprecated(
    since = "0.15.0",
    note = "Moving to a separate crate, see https://github.com/bitshifter/glam-rs/issues/175"
)]
pub struct TransformSRT {
    pub rotation: Quat,
    pub translation: Vec3,
    pub scale: Vec3,
}

impl Default for TransformSRT {
    #[inline]
    fn default() -> Self {
        Self::IDENTITY
    }
}

/**
 * A transform containing rotation and translation.
 *
 * Translation is stored as a `Vec3A` for better performance.
 */
#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(C)]
#[deprecated(
    since = "0.15.0",
    note = "Moving to a separate crate, see https://github.com/bitshifter/glam-rs/issues/175"
)]
pub struct TransformRT {
    pub rotation: Quat,
    pub translation: Vec3,
}

impl Default for TransformRT {
    #[inline]
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl TransformSRT {
    /// The identity transforms that does nothing.
    pub const IDENTITY: Self = Self {
        scale: Vec3::ONE,
        rotation: Quat::IDENTITY,
        translation: Vec3::ZERO,
    };

    /// All NaN:s.
    pub const NAN: Self = Self {
        scale: Vec3::NAN,
        rotation: Quat::NAN,
        translation: Vec3::NAN,
    };

    #[inline]
    pub fn from_scale_rotation_translation(scale: Vec3, rotation: Quat, translation: Vec3) -> Self {
        Self {
            rotation,
            translation,
            scale,
        }
    }

    /// Returns `true` if, and only if, all elements are finite.
    /// If any element is either `NaN`, positive or negative infinity, this will return `false`.
    #[inline]
    pub fn is_finite(&self) -> bool {
        self.rotation.is_finite() && self.translation.is_finite()
    }

    /// Returns `true` if, and only if, any element is `NaN`.
    #[inline]
    pub fn is_nan(&self) -> bool {
        self.rotation.is_nan() || self.translation.is_nan()
    }

    #[inline]
    pub fn inverse(&self) -> Self {
        let scale = self.scale.recip();
        let rotation = self.rotation.conjugate();
        let translation = -(rotation * (self.translation * scale));
        Self {
            rotation,
            translation,
            scale,
        }
    }

    #[inline]
    pub fn normalize(&self) -> Self {
        let rotation = self.rotation.normalize();
        Self {
            scale: self.scale,
            rotation,
            translation: self.translation,
        }
    }

    #[inline]
    pub fn mul_transform(&self, other: &Self) -> Self {
        mul_srt_srt(self, other)
    }

    #[deprecated(
        since = "0.15.0",
        note = "Please use `transform_point3(other)` instead"
    )]
    #[inline]
    pub fn transform_vec3(&self, other: Vec3) -> Vec3 {
        self.transform_point3(other)
    }

    #[inline]
    pub fn transform_point3(&self, other: Vec3) -> Vec3 {
        self.transform_point3a(other.into()).into()
    }

    #[inline]
    pub fn transform_vector3(&self, other: Vec3) -> Vec3 {
        self.transform_vector3a(other.into()).into()
    }

    #[inline]
    pub fn transform_point3a(&self, other: Vec3A) -> Vec3A {
        (self.rotation * (other * Vec3A::from(self.scale))) + Vec3A::from(self.translation)
    }

    #[inline]
    pub fn transform_vector3a(&self, other: Vec3A) -> Vec3A {
        self.rotation * (other * Vec3A::from(self.scale))
    }

    /// Returns true if the absolute difference of all elements between `self`
    /// and `other` is less than or equal to `max_abs_diff`.
    ///
    /// This can be used to compare if two `Mat4`'s contain similar elements. It
    /// works best when comparing with a known value. The `max_abs_diff` that
    /// should be used used depends on the values being compared against.
    ///
    /// For more on floating point comparisons see
    /// https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/
    #[inline]
    pub fn abs_diff_eq(&self, other: Self, max_abs_diff: f32) -> bool {
        self.scale.abs_diff_eq(other.scale, max_abs_diff)
            && self.rotation.abs_diff_eq(other.rotation, max_abs_diff)
            && self
                .translation
                .abs_diff_eq(other.translation, max_abs_diff)
    }
}

#[inline]
fn mul_srt_srt(lhs: &TransformSRT, rhs: &TransformSRT) -> TransformSRT {
    // Based on https://github.com/nfrechette/rtm `rtm::qvv_mul`
    let lhs_scale = Vec3A::from(lhs.scale);
    let rhs_scale = Vec3A::from(rhs.scale);
    let min_scale = lhs_scale.min(rhs_scale);
    let scale = lhs_scale * rhs_scale;

    if min_scale.cmplt(Vec3A::ZERO).any() {
        // If negative scale, we go through a matrix
        let lhs_mtx =
            Affine3A::from_scale_rotation_translation(lhs.scale, lhs.rotation, lhs.translation);
        let rhs_mtx =
            Affine3A::from_scale_rotation_translation(rhs.scale, rhs.rotation, rhs.translation);
        let mut result_mtx = lhs_mtx * rhs_mtx;

        let sign = scale.signum();
        result_mtx.x_axis = result_mtx.x_axis.normalize() * sign.xxx();
        result_mtx.y_axis = result_mtx.y_axis.normalize() * sign.yyy();
        result_mtx.z_axis = result_mtx.z_axis.normalize() * sign.zzz();

        let scale = Vec3::from(scale);
        let rotation = Quat::from_affine3(&result_mtx);
        let translation = Vec3::from(result_mtx.translation);
        TransformSRT {
            rotation,
            translation,
            scale,
        }
    } else {
        let rotation = lhs.rotation * rhs.rotation;
        let translation = (rhs.rotation * (lhs.translation * rhs.scale)) + rhs.translation;
        TransformSRT {
            rotation,
            translation,
            scale: scale.into(),
        }
    }
}

#[inline]
fn mul_rt_rt(lhs: &TransformRT, rhs: &TransformRT) -> TransformRT {
    let rotation = lhs.rotation * rhs.rotation;
    let translation = (rhs.rotation * lhs.translation) + rhs.translation;
    TransformRT {
        rotation,
        translation,
    }
}

impl TransformRT {
    /// The identity transforms that does nothing.
    pub const IDENTITY: Self = Self {
        rotation: Quat::IDENTITY,
        translation: Vec3::ZERO,
    };

    /// All NaN:s.
    pub const NAN: Self = Self {
        rotation: Quat::NAN,
        translation: Vec3::NAN,
    };

    #[inline]
    pub fn from_rotation_translation(rotation: Quat, translation: Vec3) -> Self {
        Self {
            rotation,
            translation,
        }
    }

    /// Returns `true` if, and only if, all elements are finite.
    /// If any element is either `NaN`, positive or negative infinity, this will return `false`.
    #[inline]
    pub fn is_finite(&self) -> bool {
        self.rotation.is_finite() && self.translation.is_finite()
    }

    /// Returns `true` if, and only if, any element is `NaN`.
    #[inline]
    pub fn is_nan(&self) -> bool {
        self.rotation.is_nan() || self.translation.is_nan()
    }

    #[inline]
    pub fn inverse(&self) -> Self {
        let rotation = self.rotation.conjugate();
        let translation = -(rotation * self.translation);
        Self {
            rotation,
            translation,
        }
    }

    #[inline]
    pub fn normalize(&self) -> Self {
        let rotation = self.rotation.normalize();
        Self {
            rotation,
            translation: self.translation,
        }
    }

    #[inline]
    pub fn mul_transform(&self, other: &Self) -> Self {
        mul_rt_rt(self, other)
    }

    #[deprecated(
        since = "0.15.0",
        note = "Please use `transform_point3(other)` instead"
    )]
    #[inline]
    pub fn transform_vec3(self, other: Vec3) -> Vec3 {
        self.transform_point3(other)
    }

    #[inline]
    pub fn transform_point3(&self, other: Vec3) -> Vec3 {
        self.transform_point3a(other.into()).into()
    }

    #[inline]
    pub fn transform_vector3(&self, other: Vec3) -> Vec3 {
        self.transform_vector3a(other.into()).into()
    }

    #[inline]
    pub fn transform_point3a(&self, other: Vec3A) -> Vec3A {
        (self.rotation * other) + Vec3A::from(self.translation)
    }

    #[inline]
    pub fn transform_vector3a(&self, other: Vec3A) -> Vec3A {
        self.rotation * other
    }

    /// Returns true if the absolute difference of all elements between `self`
    /// and `other` is less than or equal to `max_abs_diff`.
    ///
    /// This can be used to compare if two `Mat4`'s contain similar elements. It
    /// works best when comparing with a known value. The `max_abs_diff` that
    /// should be used used depends on the values being compared against.
    ///
    /// For more on floating point comparisons see
    /// https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/
    #[inline]
    pub fn abs_diff_eq(&self, other: Self, max_abs_diff: f32) -> bool {
        self.rotation.abs_diff_eq(other.rotation, max_abs_diff)
            && self
                .translation
                .abs_diff_eq(other.translation, max_abs_diff)
    }
}

impl Mul<TransformRT> for TransformRT {
    type Output = TransformRT;
    #[inline]
    fn mul(self, other: TransformRT) -> TransformRT {
        mul_rt_rt(&self, &other)
    }
}

impl Mul<TransformSRT> for TransformSRT {
    type Output = Self;
    #[inline]
    fn mul(self, other: Self) -> Self::Output {
        mul_srt_srt(&self, &other)
    }
}

impl Mul<TransformRT> for TransformSRT {
    type Output = TransformSRT;
    #[inline]
    fn mul(self, other: TransformRT) -> Self::Output {
        mul_srt_srt(&self, &other.into())
    }
}

impl Mul<TransformSRT> for TransformRT {
    type Output = TransformSRT;
    #[inline]
    fn mul(self, other: TransformSRT) -> Self::Output {
        mul_srt_srt(&self.into(), &other)
    }
}

impl From<TransformRT> for TransformSRT {
    #[inline]
    fn from(tr: TransformRT) -> Self {
        Self {
            translation: tr.translation,
            rotation: tr.rotation,
            scale: Vec3::ONE,
        }
    }
}

#[cfg(feature = "rand")]
impl Distribution<TransformRT> for Standard {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TransformRT {
        TransformRT::from_rotation_translation(
            rng.gen::<Quat>(),
            Vec3::new(
                rng.gen_range(core::f32::MIN..=core::f32::MAX),
                rng.gen_range(core::f32::MIN..=core::f32::MAX),
                rng.gen_range(core::f32::MIN..=core::f32::MAX),
            ),
        )
    }
}

#[cfg(feature = "rand")]
impl Distribution<TransformSRT> for Standard {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TransformSRT {
        let mut gen_non_zero = || loop {
            let f: f32 = rng.gen_range(core::f32::MIN..=core::f32::MAX);
            if f.abs() > core::f32::MIN_POSITIVE {
                return f;
            }
        };
        TransformSRT::from_scale_rotation_translation(
            Vec3::new(gen_non_zero(), gen_non_zero(), gen_non_zero()),
            rng.gen::<Quat>(),
            Vec3::new(
                rng.gen_range(core::f32::MIN..=core::f32::MAX),
                rng.gen_range(core::f32::MIN..=core::f32::MAX),
                rng.gen_range(core::f32::MIN..=core::f32::MAX),
            ),
        )
    }
}

impl From<TransformSRT> for Mat4 {
    #[inline]
    fn from(srt: TransformSRT) -> Self {
        Self::from_scale_rotation_translation(srt.scale, srt.rotation, srt.translation)
    }
}

impl From<TransformRT> for Mat4 {
    #[inline]
    fn from(rt: TransformRT) -> Self {
        Self::from_rotation_translation(rt.rotation, rt.translation)
    }
}

impl From<TransformSRT> for Affine3A {
    #[inline]
    fn from(srt: TransformSRT) -> Self {
        Self::from_scale_rotation_translation(srt.scale, srt.rotation, srt.translation)
    }
}

impl From<TransformRT> for Affine3A {
    #[inline]
    fn from(rt: TransformRT) -> Self {
        Self::from_rotation_translation(rt.rotation, rt.translation)
    }
}
