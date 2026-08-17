// Needed otherwise the rkyv macros generate code incompatible with rust-2024
#![cfg_attr(feature = "rkyv-serialize", allow(unsafe_op_in_unsafe_fn))]
// The macros break if the references are taken out, for some reason.
#![allow(clippy::op_ref)]

#[cfg(feature = "rkyv-serialize")]
use rkyv::bytecheck;

use crate::{
    Isometry3, Matrix4, Normed, OVector, Point3, Quaternion, Scalar, SimdRealField, Translation3,
    U8, Unit, UnitQuaternion, Vector3, Zero,
};
use approx::{AbsDiffEq, RelativeEq, UlpsEq};
#[cfg(feature = "serde-serialize-no-std")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

use simba::scalar::{ClosedNeg, RealField};

/// A dual quaternion.
///
/// # Indexing
///
/// `DualQuaternions` are stored as \[..real, ..dual\].
/// Both of the quaternion components are laid out in `i, j, k, w` order.
///
/// # Example
/// ```
/// # use nalgebra::{DualQuaternion, Quaternion};
///
/// let real = Quaternion::new(1.0, 2.0, 3.0, 4.0);
/// let dual = Quaternion::new(5.0, 6.0, 7.0, 8.0);
///
/// let dq = DualQuaternion::from_real_and_dual(real, dual);
/// assert_eq!(dq[0], 2.0);
/// assert_eq!(dq[1], 3.0);
///
/// assert_eq!(dq[4], 6.0);
/// assert_eq!(dq[7], 5.0);
/// ```
///
/// NOTE:
///  As of December 2020, dual quaternion support is a work in progress.
///  If a feature that you need is missing, feel free to open an issue or a PR.
///  See <https://github.com/dimforge/nalgebra/issues/487>
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[cfg_attr(
    feature = "rkyv-serialize-no-std",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize),
    archive(
        as = "DualQuaternion<T::Archived>",
        bound(archive = "
        T: rkyv::Archive,
        Quaternion<T>: rkyv::Archive<Archived = Quaternion<T::Archived>>
    ")
    )
)]
#[cfg_attr(feature = "rkyv-serialize", derive(bytecheck::CheckBytes))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DualQuaternion<T> {
    /// The real component of the quaternion
    pub real: Quaternion<T>,
    /// The dual component of the quaternion
    pub dual: Quaternion<T>,
}

impl<T: Scalar + Eq> Eq for DualQuaternion<T> {}

impl<T: Scalar> PartialEq for DualQuaternion<T> {
    #[inline]
    fn eq(&self, right: &Self) -> bool {
        self.real == right.real && self.dual == right.dual
    }
}

impl<T: Scalar + Zero> Default for DualQuaternion<T> {
    fn default() -> Self {
        Self {
            real: Quaternion::default(),
            dual: Quaternion::default(),
        }
    }
}

impl<T: SimdRealField> DualQuaternion<T>
where
    T::Element: SimdRealField,
{
    /// Normalizes this quaternion.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use nalgebra::{DualQuaternion, Quaternion};
    /// let real = Quaternion::new(1.0, 2.0, 3.0, 4.0);
    /// let dual = Quaternion::new(5.0, 6.0, 7.0, 8.0);
    /// let dq = DualQuaternion::from_real_and_dual(real, dual);
    ///
    /// let dq_normalized = dq.normalize();
    ///
    /// assert_relative_eq!(dq_normalized.real.norm(), 1.0);
    /// ```
    #[inline]
    #[must_use = "Did you mean to use normalize_mut()?"]
    pub fn normalize(&self) -> Self {
        let real_norm = self.real.norm();

        Self::from_real_and_dual(
            self.real.clone() / real_norm.clone(),
            self.dual.clone() / real_norm,
        )
    }

    /// Normalizes this quaternion.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use nalgebra::{DualQuaternion, Quaternion};
    /// let real = Quaternion::new(1.0, 2.0, 3.0, 4.0);
    /// let dual = Quaternion::new(5.0, 6.0, 7.0, 8.0);
    /// let mut dq = DualQuaternion::from_real_and_dual(real, dual);
    ///
    /// dq.normalize_mut();
    ///
    /// assert_relative_eq!(dq.real.norm(), 1.0);
    /// ```
    #[inline]
    pub fn normalize_mut(&mut self) -> T {
        let real_norm = self.real.norm();
        self.real /= real_norm.clone();
        self.dual /= real_norm.clone();
        real_norm
    }

    /// The conjugate of this dual quaternion, containing the conjugate of
    /// the real and imaginary parts..
    ///
    /// # Example
    /// ```
    /// # use nalgebra::{DualQuaternion, Quaternion};
    /// let real = Quaternion::new(1.0, 2.0, 3.0, 4.0);
    /// let dual = Quaternion::new(5.0, 6.0, 7.0, 8.0);
    /// let dq = DualQuaternion::from_real_and_dual(real, dual);
    ///
    /// let conj = dq.conjugate();
    /// assert!(conj.real.i == -2.0 && conj.real.j == -3.0 && conj.real.k == -4.0);
    /// assert!(conj.real.w == 1.0);
    /// assert!(conj.dual.i == -6.0 && conj.dual.j == -7.0 && conj.dual.k == -8.0);
    /// assert!(conj.dual.w == 5.0);
    /// ```
    #[inline]
    #[must_use = "Did you mean to use conjugate_mut()?"]
    pub fn conjugate(&self) -> Self {
        Self::from_real_and_dual(self.real.conjugate(), self.dual.conjugate())
    }

    /// Replaces this quaternion by its conjugate.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::{DualQuaternion, Quaternion};
    /// let real = Quaternion::new(1.0, 2.0, 3.0, 4.0);
    /// let dual = Quaternion::new(5.0, 6.0, 7.0, 8.0);
    /// let mut dq = DualQuaternion::from_real_and_dual(real, dual);
    ///
    /// dq.conjugate_mut();
    /// assert!(dq.real.i == -2.0 && dq.real.j == -3.0 && dq.real.k == -4.0);
    /// assert!(dq.real.w == 1.0);
    /// assert!(dq.dual.i == -6.0 && dq.dual.j == -7.0 && dq.dual.k == -8.0);
    /// assert!(dq.dual.w == 5.0);
    /// ```
    #[inline]
    pub fn conjugate_mut(&mut self) {
        self.real.conjugate_mut();
        self.dual.conjugate_mut();
    }

    /// Inverts this dual quaternion if it is not zero.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use nalgebra::{DualQuaternion, Quaternion};
    /// let real = Quaternion::new(1.0, 2.0, 3.0, 4.0);
    /// let dual = Quaternion::new(5.0, 6.0, 7.0, 8.0);
    /// let dq = DualQuaternion::from_real_and_dual(real, dual);
    /// let inverse = dq.try_inverse();
    ///
    /// assert!(inverse.is_some());
    /// assert_relative_eq!(inverse.unwrap() * dq, DualQuaternion::identity());
    ///
    /// //Non-invertible case
    /// let zero = Quaternion::new(0.0, 0.0, 0.0, 0.0);
    /// let dq = DualQuaternion::from_real_and_dual(zero, zero);
    /// let inverse = dq.try_inverse();
    ///
    /// assert!(inverse.is_none());
    /// ```
    #[inline]
    #[must_use = "Did you mean to use try_inverse_mut()?"]
    pub fn try_inverse(&self) -> Option<Self>
    where
        T: RealField,
    {
        let mut res = self.clone();
        if res.try_inverse_mut() {
            Some(res)
        } else {
            None
        }
    }

    /// Inverts this dual quaternion in-place if it is not zero.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use nalgebra::{DualQuaternion, Quaternion};
    /// let real = Quaternion::new(1.0, 2.0, 3.0, 4.0);
    /// let dual = Quaternion::new(5.0, 6.0, 7.0, 8.0);
    /// let dq = DualQuaternion::from_real_and_dual(real, dual);
    /// let mut dq_inverse = dq;
    /// dq_inverse.try_inverse_mut();
    ///
    /// assert_relative_eq!(dq_inverse * dq, DualQuaternion::identity());
    ///
    /// //Non-invertible case
    /// let zero = Quaternion::new(0.0, 0.0, 0.0, 0.0);
    /// let mut dq = DualQuaternion::from_real_and_dual(zero, zero);
    /// assert!(!dq.try_inverse_mut());
    /// ```
    #[inline]
    pub fn try_inverse_mut(&mut self) -> bool
    where
        T: RealField,
    {
        let inverted = self.real.try_inverse_mut();
        if inverted {
            self.dual = -self.real.clone() * self.dual.clone() * self.real.clone();
            true
        } else {
            false
        }
    }

    /// Linear interpolation between two dual quaternions.
    ///
    /// Computes `self * (1 - t) + other * t`.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::{DualQuaternion, Quaternion};
    /// let dq1 = DualQuaternion::from_real_and_dual(
    ///     Quaternion::new(1.0, 0.0, 0.0, 4.0),
    ///     Quaternion::new(0.0, 2.0, 0.0, 0.0)
    /// );
    /// let dq2 = DualQuaternion::from_real_and_dual(
    ///     Quaternion::new(2.0, 0.0, 1.0, 0.0),
    ///     Quaternion::new(0.0, 2.0, 0.0, 0.0)
    /// );
    /// assert_eq!(dq1.lerp(&dq2, 0.25), DualQuaternion::from_real_and_dual(
    ///     Quaternion::new(1.25, 0.0, 0.25, 3.0),
    ///     Quaternion::new(0.0, 2.0, 0.0, 0.0)
    /// ));
    /// ```
    #[inline]
    #[must_use]
    pub fn lerp(&self, other: &Self, t: T) -> Self {
        self * (T::one() - t.clone()) + other * t
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl<T> bytemuck::Zeroable for DualQuaternion<T>
where
    T: Scalar + bytemuck::Zeroable,
    Quaternion<T>: bytemuck::Zeroable,
{
}

#[cfg(feature = "bytemuck")]
unsafe impl<T> bytemuck::Pod for DualQuaternion<T>
where
    T: Scalar + bytemuck::Pod,
    Quaternion<T>: bytemuck::Pod,
{
}

#[cfg(feature = "serde-serialize-no-std")]
impl<T: SimdRealField> Serialize for DualQuaternion<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        self.as_ref().serialize(serializer)
    }
}

#[cfg(feature = "serde-serialize-no-std")]
impl<'a, T: SimdRealField> Deserialize<'a> for DualQuaternion<T>
where
    T: Deserialize<'a>,
{
    fn deserialize<Des>(deserializer: Des) -> Result<Self, Des::Error>
    where
        Des: Deserializer<'a>,
    {
        type Dq<T> = [T; 8];

        let dq: Dq<T> = Dq::<T>::deserialize(deserializer)?;

        Ok(Self {
            real: Quaternion::new(dq[3].clone(), dq[0].clone(), dq[1].clone(), dq[2].clone()),
            dual: Quaternion::new(dq[7].clone(), dq[4].clone(), dq[5].clone(), dq[6].clone()),
        })
    }
}

impl<T: RealField> DualQuaternion<T> {
    #[allow(clippy::wrong_self_convention)]
    fn to_vector(&self) -> OVector<T, U8> {
        self.as_ref().clone().into()
    }
}

impl<T: RealField + AbsDiffEq<Epsilon = T>> AbsDiffEq for DualQuaternion<T> {
    type Epsilon = T;

    #[inline]
    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.to_vector().abs_diff_eq(&other.to_vector(), epsilon.clone()) ||
        // Account for the double-covering of S², i.e. q = -q
        self.to_vector().iter().zip(other.to_vector().iter()).all(|(a, b)| a.abs_diff_eq(&-b.clone(), epsilon.clone()))
    }
}

impl<T: RealField + RelativeEq<Epsilon = T>> RelativeEq for DualQuaternion<T> {
    #[inline]
    fn default_max_relative() -> Self::Epsilon {
        T::default_max_relative()
    }

    #[inline]
    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        self.to_vector().relative_eq(&other.to_vector(), epsilon.clone(), max_relative.clone()) ||
        // Account for the double-covering of S², i.e. q = -q
        self.to_vector().iter().zip(other.to_vector().iter()).all(|(a, b)| a.relative_eq(&-b.clone(), epsilon.clone(), max_relative.clone()))
    }
}

impl<T: RealField + UlpsEq<Epsilon = T>> UlpsEq for DualQuaternion<T> {
    #[inline]
    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
    }

    #[inline]
    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        self.to_vector().ulps_eq(&other.to_vector(), epsilon.clone(), max_ulps) ||
        // Account for the double-covering of S², i.e. q = -q.
        self.to_vector().iter().zip(other.to_vector().iter()).all(|(a, b)| a.ulps_eq(&-b.clone(), epsilon.clone(), max_ulps))
    }
}

/// A unit dual quaternion. May be used to represent a rotation followed by a
/// translation.
pub type UnitDualQuaternion<T> = Unit<DualQuaternion<T>>;

impl<T: Scalar + ClosedNeg + PartialEq + SimdRealField> PartialEq for UnitDualQuaternion<T> {
    #[inline]
    fn eq(&self, rhs: &Self) -> bool {
        self.as_ref().eq(rhs.as_ref())
    }
}

impl<T: Scalar + ClosedNeg + Eq + SimdRealField> Eq for UnitDualQuaternion<T> {}

impl<T: SimdRealField> Normed for DualQuaternion<T> {
    type Norm = T::SimdRealField;

    #[inline]
    fn norm(&self) -> T::SimdRealField {
        self.real.norm()
    }

    #[inline]
    fn norm_squared(&self) -> T::SimdRealField {
        self.real.norm_squared()
    }

    #[inline]
    fn scale_mut(&mut self, n: Self::Norm) {
        self.real.scale_mut(n.clone());
        self.dual.scale_mut(n);
    }

    #[inline]
    fn unscale_mut(&mut self, n: Self::Norm) {
        self.real.unscale_mut(n.clone());
        self.dual.unscale_mut(n);
    }
}

impl<T: SimdRealField> UnitDualQuaternion<T>
where
    T::Element: SimdRealField,
{
    /// The underlying dual quaternion.
    ///
    /// Same as `self.as_ref()`.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::{DualQuaternion, UnitDualQuaternion, Quaternion};
    /// let id = UnitDualQuaternion::identity();
    /// assert_eq!(*id.dual_quaternion(), DualQuaternion::from_real_and_dual(
    ///     Quaternion::new(1.0, 0.0, 0.0, 0.0),
    ///     Quaternion::new(0.0, 0.0, 0.0, 0.0)
    /// ));
    /// ```
    #[inline]
    #[must_use]
    pub fn dual_quaternion(&self) -> &DualQuaternion<T> {
        self.as_ref()
    }

    /// Compute the conjugate of this unit quaternion.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::{UnitDualQuaternion, DualQuaternion, Quaternion};
    /// let qr = Quaternion::new(1.0, 2.0, 3.0, 4.0);
    /// let qd = Quaternion::new(5.0, 6.0, 7.0, 8.0);
    /// let unit = UnitDualQuaternion::new_normalize(
    ///     DualQuaternion::from_real_and_dual(qr, qd)
    /// );
    /// let conj = unit.conjugate();
    /// assert_eq!(conj.real, unit.real.conjugate());
    /// assert_eq!(conj.dual, unit.dual.conjugate());
    /// ```
    #[inline]
    #[must_use = "Did you mean to use conjugate_mut()?"]
    pub fn conjugate(&self) -> Self {
        Self::new_unchecked(self.as_ref().conjugate())
    }

    /// Compute the conjugate of this unit quaternion in-place.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::{UnitDualQuaternion, DualQuaternion, Quaternion};
    /// let qr = Quaternion::new(1.0, 2.0, 3.0, 4.0);
    /// let qd = Quaternion::new(5.0, 6.0, 7.0, 8.0);
    /// let unit = UnitDualQuaternion::new_normalize(
    ///     DualQuaternion::from_real_and_dual(qr, qd)
    /// );
    /// let mut conj = unit.clone();
    /// conj.conjugate_mut();
    /// assert_eq!(conj.as_ref().real, unit.as_ref().real.conjugate());
    /// assert_eq!(conj.as_ref().dual, unit.as_ref().dual.conjugate());
    /// ```
    #[inline]
    pub fn conjugate_mut(&mut self) {
        self.as_mut_unchecked().conjugate_mut()
    }

    /// Inverts this dual quaternion if it is not zero.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use nalgebra::{UnitDualQuaternion, Quaternion, DualQuaternion};
    /// let qr = Quaternion::new(1.0, 2.0, 3.0, 4.0);
    /// let qd = Quaternion::new(5.0, 6.0, 7.0, 8.0);
    /// let unit = UnitDualQuaternion::new_normalize(DualQuaternion::from_real_and_dual(qr, qd));
    /// let inv = unit.inverse();
    /// assert_relative_eq!(unit * inv, UnitDualQuaternion::identity(), epsilon = 1.0e-6);
    /// assert_relative_eq!(inv * unit, UnitDualQuaternion::identity(), epsilon = 1.0e-6);
    /// ```
    #[inline]
    #[must_use = "Did you mean to use inverse_mut()?"]
    pub fn inverse(&self) -> Self {
        let real = Unit::new_unchecked(self.as_ref().real.clone())
            .inverse()
            .into_inner();
        let dual = -real.clone() * self.as_ref().dual.clone() * real.clone();
        UnitDualQuaternion::new_unchecked(DualQuaternion { real, dual })
    }

    /// Inverts this dual quaternion in place if it is not zero.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use nalgebra::{UnitDualQuaternion, Quaternion, DualQuaternion};
    /// let qr = Quaternion::new(1.0, 2.0, 3.0, 4.0);
    /// let qd = Quaternion::new(5.0, 6.0, 7.0, 8.0);
    /// let unit = UnitDualQuaternion::new_normalize(DualQuaternion::from_real_and_dual(qr, qd));
    /// let mut inv = unit.clone();
    /// inv.inverse_mut();
    /// assert_relative_eq!(unit * inv, UnitDualQuaternion::identity(), epsilon = 1.0e-6);
    /// assert_relative_eq!(inv * unit, UnitDualQuaternion::identity(), epsilon = 1.0e-6);
    /// ```
    #[inline]
    pub fn inverse_mut(&mut self) {
        let quat = self.as_mut_unchecked();
        quat.real = Unit::new_unchecked(quat.real.clone())
            .inverse()
            .into_inner();
        quat.dual = -quat.real.clone() * quat.dual.clone() * quat.real.clone();
    }

    /// The unit dual quaternion needed to make `self` and `other` coincide.
    ///
    /// The result is such that: `self.isometry_to(other) * self == other`.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use nalgebra::{UnitDualQuaternion, DualQuaternion, Quaternion};
    /// let qr = Quaternion::new(1.0, 2.0, 3.0, 4.0);
    /// let qd = Quaternion::new(5.0, 6.0, 7.0, 8.0);
    /// let dq1 = UnitDualQuaternion::new_normalize(DualQuaternion::from_real_and_dual(qr, qd));
    /// let dq2 = UnitDualQuaternion::new_normalize(DualQuaternion::from_real_and_dual(qd, qr));
    /// let dq_to = dq1.isometry_to(&dq2);
    /// assert_relative_eq!(dq_to * dq1, dq2, epsilon = 1.0e-6);
    /// ```
    #[inline]
    #[must_use]
    pub fn isometry_to(&self, other: &Self) -> Self {
        other / self
    }

    /// Linear interpolation between two unit dual quaternions.
    ///
    /// The result is not normalized.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use nalgebra::{UnitDualQuaternion, DualQuaternion, Quaternion};
    /// let dq1 = UnitDualQuaternion::new_normalize(DualQuaternion::from_real_and_dual(
    ///     Quaternion::new(0.5, 0.0, 0.5, 0.0),
    ///     Quaternion::new(0.0, 0.5, 0.0, 0.5)
    /// ));
    /// let dq2 = UnitDualQuaternion::new_normalize(DualQuaternion::from_real_and_dual(
    ///     Quaternion::new(0.5, 0.0, 0.0, 0.5),
    ///     Quaternion::new(0.5, 0.0, 0.5, 0.0)
    /// ));
    /// assert_relative_eq!(
    ///     UnitDualQuaternion::new_normalize(dq1.lerp(&dq2, 0.5)),
    ///     UnitDualQuaternion::new_normalize(
    ///         DualQuaternion::from_real_and_dual(
    ///             Quaternion::new(0.5, 0.0, 0.25, 0.25),
    ///             Quaternion::new(0.25, 0.25, 0.25, 0.25)
    ///         )
    ///     ),
    ///     epsilon = 1.0e-6
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn lerp(&self, other: &Self, t: T) -> DualQuaternion<T> {
        self.as_ref().lerp(other.as_ref(), t)
    }

    /// Normalized linear interpolation between two unit quaternions.
    ///
    /// This is the same as `self.lerp` except that the result is normalized.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use nalgebra::{UnitDualQuaternion, DualQuaternion, Quaternion};
    /// let dq1 = UnitDualQuaternion::new_normalize(DualQuaternion::from_real_and_dual(
    ///     Quaternion::new(0.5, 0.0, 0.5, 0.0),
    ///     Quaternion::new(0.0, 0.5, 0.0, 0.5)
    /// ));
    /// let dq2 = UnitDualQuaternion::new_normalize(DualQuaternion::from_real_and_dual(
    ///     Quaternion::new(0.5, 0.0, 0.0, 0.5),
    ///     Quaternion::new(0.5, 0.0, 0.5, 0.0)
    /// ));
    /// assert_relative_eq!(dq1.nlerp(&dq2, 0.2), UnitDualQuaternion::new_normalize(
    ///     DualQuaternion::from_real_and_dual(
    ///         Quaternion::new(0.5, 0.0, 0.4, 0.1),
    ///         Quaternion::new(0.1, 0.4, 0.1, 0.4)
    ///     )
    /// ), epsilon = 1.0e-6);
    /// ```
    #[inline]
    #[must_use]
    pub fn nlerp(&self, other: &Self, t: T) -> Self {
        let mut res = self.lerp(other, t);
        let _ = res.normalize_mut();

        Self::new_unchecked(res)
    }

    /// Screw linear interpolation between two unit quaternions. This creates a
    /// smooth arc from one dual-quaternion to another.
    ///
    /// Panics if the angle between both quaternion is 180 degrees (in which
    /// case the interpolation is not well-defined). Use `.try_sclerp`
    /// instead to avoid the panic.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use nalgebra::{UnitDualQuaternion, DualQuaternion, UnitQuaternion, Vector3};
    ///
    /// let dq1 = UnitDualQuaternion::from_parts(
    ///     Vector3::new(0.0, 3.0, 0.0).into(),
    ///     UnitQuaternion::from_euler_angles(std::f32::consts::FRAC_PI_4, 0.0, 0.0),
    /// );
    ///
    /// let dq2 = UnitDualQuaternion::from_parts(
    ///     Vector3::new(0.0, 0.0, 3.0).into(),
    ///     UnitQuaternion::from_euler_angles(-std::f32::consts::PI, 0.0, 0.0),
    /// );
    ///
    /// let dq = dq1.sclerp(&dq2, 1.0 / 3.0);
    ///
    /// assert_relative_eq!(
    ///     dq.rotation().euler_angles().0, std::f32::consts::FRAC_PI_2, epsilon = 1.0e-6
    /// );
    /// assert_relative_eq!(dq.translation().vector.y, 3.0, epsilon = 1.0e-6);
    /// ```
    #[inline]
    #[must_use]
    pub fn sclerp(&self, other: &Self, t: T) -> Self
    where
        T: RealField,
    {
        self.try_sclerp(other, t, T::default_epsilon())
            .expect("DualQuaternion sclerp: ambiguous configuration.")
    }

    /// Computes the screw-linear interpolation between two unit quaternions or
    /// returns `None` if both quaternions are approximately 180 degrees
    /// apart (in which case the interpolation is not well-defined).
    ///
    /// # Arguments
    /// * `self`: the first quaternion to interpolate from.
    /// * `other`: the second quaternion to interpolate toward.
    /// * `t`: the interpolation parameter. Should be between 0 and 1.
    /// * `epsilon`: the value below which the sinus of the angle separating
    ///   both quaternion must be to return `None`.
    #[inline]
    #[must_use]
    pub fn try_sclerp(&self, other: &Self, t: T, epsilon: T) -> Option<Self>
    where
        T: RealField,
    {
        let two = T::one() + T::one();
        let half = T::one() / two.clone();

        // Invert one of the quaternions if we've got a longest-path
        // interpolation.
        let other = {
            let dot_product = self.as_ref().real.coords.dot(&other.as_ref().real.coords);
            if relative_eq!(dot_product, T::zero(), epsilon = epsilon.clone()) {
                return None;
            }

            if dot_product < T::zero() {
                -other.clone()
            } else {
                other.clone()
            }
        };

        let difference = self.as_ref().conjugate() * other.as_ref();
        let norm_squared = difference.real.vector().norm_squared();
        if relative_eq!(norm_squared, T::zero(), epsilon = epsilon) {
            return Some(Self::from_parts(
                self.translation()
                    .vector
                    .lerp(&other.translation().vector, t)
                    .into(),
                self.rotation(),
            ));
        }

        let scalar: T = difference.real.scalar();
        let mut angle = two.clone() * scalar.acos();

        let inverse_norm_squared: T = T::one() / norm_squared;
        let inverse_norm = inverse_norm_squared.sqrt();

        let mut pitch = -two * difference.dual.scalar() * inverse_norm.clone();
        let direction = difference.real.vector() * inverse_norm.clone();
        let moment = (difference.dual.vector()
            - direction.clone() * (pitch.clone() * difference.real.scalar() * half.clone()))
            * inverse_norm;

        angle *= t.clone();
        pitch *= t;

        let sin = (half.clone() * angle.clone()).sin();
        let cos = (half.clone() * angle).cos();

        let real = Quaternion::from_parts(cos.clone(), direction.clone() * sin.clone());
        let dual = Quaternion::from_parts(
            -pitch.clone() * half.clone() * sin.clone(),
            moment * sin + direction * (pitch * half * cos),
        );

        Some(
            self * UnitDualQuaternion::new_unchecked(DualQuaternion::from_real_and_dual(
                real, dual,
            )),
        )
    }

    /// Return the rotation part of this unit dual quaternion.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use nalgebra::{UnitDualQuaternion, UnitQuaternion, Vector3};
    /// let dq = UnitDualQuaternion::from_parts(
    ///     Vector3::new(0.0, 3.0, 0.0).into(),
    ///     UnitQuaternion::from_euler_angles(std::f32::consts::FRAC_PI_4, 0.0, 0.0)
    /// );
    ///
    /// assert_relative_eq!(
    ///     dq.rotation().angle(), std::f32::consts::FRAC_PI_4, epsilon = 1.0e-6
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn rotation(&self) -> UnitQuaternion<T> {
        Unit::new_unchecked(self.as_ref().real.clone())
    }

    /// Return the translation part of this unit dual quaternion.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use nalgebra::{UnitDualQuaternion, UnitQuaternion, Vector3};
    /// let dq = UnitDualQuaternion::from_parts(
    ///     Vector3::new(0.0, 3.0, 0.0).into(),
    ///     UnitQuaternion::from_euler_angles(std::f32::consts::FRAC_PI_4, 0.0, 0.0)
    /// );
    ///
    /// assert_relative_eq!(
    ///     dq.translation().vector, Vector3::new(0.0, 3.0, 0.0), epsilon = 1.0e-6
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn translation(&self) -> Translation3<T> {
        let two = T::one() + T::one();
        Translation3::from(
            ((self.as_ref().dual.clone() * self.as_ref().real.clone().conjugate()) * two)
                .vector()
                .into_owned(),
        )
    }

    /// Builds an isometry from this unit dual quaternion.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use nalgebra::{UnitDualQuaternion, UnitQuaternion, Vector3};
    /// let rotation = UnitQuaternion::from_euler_angles(std::f32::consts::PI, 0.0, 0.0);
    /// let translation = Vector3::new(1.0, 3.0, 2.5);
    /// let dq = UnitDualQuaternion::from_parts(
    ///     translation.into(),
    ///     rotation
    /// );
    /// let iso = dq.to_isometry();
    ///
    /// assert_relative_eq!(iso.rotation.angle(), std::f32::consts::PI, epsilon = 1.0e-6);
    /// assert_relative_eq!(iso.translation.vector, translation, epsilon = 1.0e-6);
    /// ```
    #[inline]
    #[must_use]
    pub fn to_isometry(self) -> Isometry3<T> {
        Isometry3::from_parts(self.translation(), self.rotation())
    }

    /// Rotate and translate a point by this unit dual quaternion interpreted
    /// as an isometry.
    ///
    /// This is the same as the multiplication `self * pt`.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use nalgebra::{UnitDualQuaternion, UnitQuaternion, Vector3, Point3};
    /// let dq = UnitDualQuaternion::from_parts(
    ///     Vector3::new(0.0, 3.0, 0.0).into(),
    ///     UnitQuaternion::from_euler_angles(std::f32::consts::FRAC_PI_2, 0.0, 0.0)
    /// );
    /// let point = Point3::new(1.0, 2.0, 3.0);
    ///
    /// assert_relative_eq!(
    ///     dq.transform_point(&point), Point3::new(1.0, 0.0, 2.0), epsilon = 1.0e-6
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn transform_point(&self, pt: &Point3<T>) -> Point3<T> {
        self * pt
    }

    /// Rotate a vector by this unit dual quaternion, ignoring the translational
    /// component.
    ///
    /// This is the same as the multiplication `self * v`.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use nalgebra::{UnitDualQuaternion, UnitQuaternion, Vector3};
    /// let dq = UnitDualQuaternion::from_parts(
    ///     Vector3::new(0.0, 3.0, 0.0).into(),
    ///     UnitQuaternion::from_euler_angles(std::f32::consts::FRAC_PI_2, 0.0, 0.0)
    /// );
    /// let vector = Vector3::new(1.0, 2.0, 3.0);
    ///
    /// assert_relative_eq!(
    ///     dq.transform_vector(&vector), Vector3::new(1.0, -3.0, 2.0), epsilon = 1.0e-6
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn transform_vector(&self, v: &Vector3<T>) -> Vector3<T> {
        self * v
    }

    /// Rotate and translate a point by the inverse of this unit quaternion.
    ///
    /// This may be cheaper than inverting the unit dual quaternion and
    /// transforming the point.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use nalgebra::{UnitDualQuaternion, UnitQuaternion, Vector3, Point3};
    /// let dq = UnitDualQuaternion::from_parts(
    ///     Vector3::new(0.0, 3.0, 0.0).into(),
    ///     UnitQuaternion::from_euler_angles(std::f32::consts::FRAC_PI_2, 0.0, 0.0)
    /// );
    /// let point = Point3::new(1.0, 2.0, 3.0);
    ///
    /// assert_relative_eq!(
    ///     dq.inverse_transform_point(&point), Point3::new(1.0, 3.0, 1.0), epsilon = 1.0e-6
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn inverse_transform_point(&self, pt: &Point3<T>) -> Point3<T> {
        self.inverse() * pt
    }

    /// Rotate a vector by the inverse of this unit quaternion, ignoring the
    /// translational component.
    ///
    /// This may be cheaper than inverting the unit dual quaternion and
    /// transforming the vector.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use nalgebra::{UnitDualQuaternion, UnitQuaternion, Vector3};
    /// let dq = UnitDualQuaternion::from_parts(
    ///     Vector3::new(0.0, 3.0, 0.0).into(),
    ///     UnitQuaternion::from_euler_angles(std::f32::consts::FRAC_PI_2, 0.0, 0.0)
    /// );
    /// let vector = Vector3::new(1.0, 2.0, 3.0);
    ///
    /// assert_relative_eq!(
    ///     dq.inverse_transform_vector(&vector), Vector3::new(1.0, 3.0, -2.0), epsilon = 1.0e-6
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn inverse_transform_vector(&self, v: &Vector3<T>) -> Vector3<T> {
        self.inverse() * v
    }

    /// Rotate a unit vector by the inverse of this unit quaternion, ignoring
    /// the translational component. This may be
    /// cheaper than inverting the unit dual quaternion and transforming the
    /// vector.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use nalgebra::{UnitDualQuaternion, UnitQuaternion, Unit, Vector3};
    /// let dq = UnitDualQuaternion::from_parts(
    ///     Vector3::new(0.0, 3.0, 0.0).into(),
    ///     UnitQuaternion::from_euler_angles(std::f32::consts::FRAC_PI_2, 0.0, 0.0)
    /// );
    /// let vector = Unit::new_unchecked(Vector3::new(0.0, 1.0, 0.0));
    ///
    /// assert_relative_eq!(
    ///     dq.inverse_transform_unit_vector(&vector),
    ///     Unit::new_unchecked(Vector3::new(0.0, 0.0, -1.0)),
    ///     epsilon = 1.0e-6
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn inverse_transform_unit_vector(&self, v: &Unit<Vector3<T>>) -> Unit<Vector3<T>> {
        self.inverse() * v
    }
}

impl<T: SimdRealField + RealField> UnitDualQuaternion<T>
where
    T::Element: SimdRealField,
{
    /// Converts this unit dual quaternion interpreted as an isometry
    /// into its equivalent homogeneous transformation matrix.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use nalgebra::{Matrix4, UnitDualQuaternion, UnitQuaternion, Vector3};
    /// let dq = UnitDualQuaternion::from_parts(
    ///     Vector3::new(1.0, 3.0, 2.0).into(),
    ///     UnitQuaternion::from_axis_angle(&Vector3::z_axis(), std::f32::consts::FRAC_PI_6)
    /// );
    /// let expected = Matrix4::new(0.8660254, -0.5,      0.0, 1.0,
    ///                             0.5,       0.8660254, 0.0, 3.0,
    ///                             0.0,       0.0,       1.0, 2.0,
    ///                             0.0,       0.0,       0.0, 1.0);
    ///
    /// assert_relative_eq!(dq.to_homogeneous(), expected, epsilon = 1.0e-6);
    /// ```
    #[inline]
    #[must_use]
    pub fn to_homogeneous(self) -> Matrix4<T> {
        self.to_isometry().to_homogeneous()
    }
}

impl<T: RealField> Default for UnitDualQuaternion<T> {
    fn default() -> Self {
        Self::identity()
    }
}

impl<T: RealField + fmt::Display> fmt::Display for UnitDualQuaternion<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.rotation().axis() {
            Some(axis) => {
                let axis = axis.into_inner();
                write!(
                    f,
                    "UnitDualQuaternion translation: {} − angle: {} − axis: ({}, {}, {})",
                    self.translation().vector,
                    self.rotation().angle(),
                    axis[0],
                    axis[1],
                    axis[2]
                )
            }
            None => {
                write!(
                    f,
                    "UnitDualQuaternion translation: {} − angle: {} − axis: (undefined)",
                    self.translation().vector,
                    self.rotation().angle()
                )
            }
        }
    }
}

impl<T: RealField + AbsDiffEq<Epsilon = T>> AbsDiffEq for UnitDualQuaternion<T> {
    type Epsilon = T;

    #[inline]
    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.as_ref().abs_diff_eq(other.as_ref(), epsilon)
    }
}

impl<T: RealField + RelativeEq<Epsilon = T>> RelativeEq for UnitDualQuaternion<T> {
    #[inline]
    fn default_max_relative() -> Self::Epsilon {
        T::default_max_relative()
    }

    #[inline]
    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        self.as_ref()
            .relative_eq(other.as_ref(), epsilon, max_relative)
    }
}

impl<T: RealField + UlpsEq<Epsilon = T>> UlpsEq for UnitDualQuaternion<T> {
    #[inline]
    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
    }

    #[inline]
    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        self.as_ref().ulps_eq(other.as_ref(), epsilon, max_ulps)
    }
}
