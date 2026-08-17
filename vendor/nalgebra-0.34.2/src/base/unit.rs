// Needed otherwise the rkyv macros generate code incompatible with rust-2024
#![cfg_attr(feature = "rkyv-serialize", allow(unsafe_op_in_unsafe_fn))]

use std::fmt;
use std::ops::Deref;

#[cfg(feature = "serde-serialize-no-std")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::allocator::Allocator;
use crate::base::DefaultAllocator;
use crate::storage::RawStorage;
use crate::{Dim, Matrix, OMatrix, RealField, Scalar, SimdComplexField, SimdRealField};

#[cfg(feature = "rkyv-serialize")]
use rkyv::bytecheck;

/// A wrapper that ensures the underlying algebraic entity has a unit norm.
///
/// **It is likely that the only piece of documentation that you need in this page are:**
/// - **[The construction with normalization](#construction-with-normalization)**
/// - **[Data extraction and construction without normalization](#data-extraction-and-construction-without-normalization)**
/// - **[Interpolation between two unit vectors](#interpolation-between-two-unit-vectors)**
///
/// All the other impl blocks you will see in this page are about [`UnitComplex`](crate::UnitComplex)
/// and [`UnitQuaternion`](crate::UnitQuaternion); both built on top of `Unit`.  If you are interested
/// in their documentation, read their dedicated pages directly.
#[repr(transparent)]
#[derive(Clone, Hash, Copy)]
#[cfg_attr(
    feature = "rkyv-serialize-no-std",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize),
    archive(
        as = "Unit<T::Archived>",
        bound(archive = "
        T: rkyv::Archive,
    ")
    )
)]
#[cfg_attr(feature = "rkyv-serialize", derive(bytecheck::CheckBytes))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Unit<T> {
    pub(crate) value: T,
}

impl<T: fmt::Debug> fmt::Debug for Unit<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        self.value.fmt(formatter)
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl<T> bytemuck::Zeroable for Unit<T> where T: bytemuck::Zeroable {}

#[cfg(feature = "bytemuck")]
unsafe impl<T> bytemuck::Pod for Unit<T> where T: bytemuck::Pod {}

#[cfg(feature = "serde-serialize-no-std")]
impl<T: Serialize> Serialize for Unit<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.value.serialize(serializer)
    }
}

#[cfg(feature = "serde-serialize-no-std")]
impl<'de, T: Deserialize<'de>> Deserialize<'de> for Unit<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        T::deserialize(deserializer).map(|x| Unit { value: x })
    }
}

impl<T, R, C, S> PartialEq for Unit<Matrix<T, R, C, S>>
where
    T: Scalar + PartialEq,
    R: Dim,
    C: Dim,
    S: RawStorage<T, R, C>,
{
    #[inline]
    fn eq(&self, rhs: &Self) -> bool {
        self.value.eq(&rhs.value)
    }
}

impl<T, R, C, S> Eq for Unit<Matrix<T, R, C, S>>
where
    T: Scalar + Eq,
    R: Dim,
    C: Dim,
    S: RawStorage<T, R, C>,
{
}

/// Trait implemented by entities scan be be normalized and put in an `Unit` struct.
pub trait Normed {
    /// The type of the norm.
    type Norm: SimdRealField;
    /// Computes the norm.
    fn norm(&self) -> Self::Norm;
    /// Computes the squared norm.
    fn norm_squared(&self) -> Self::Norm;
    /// Multiply `self` by n.
    fn scale_mut(&mut self, n: Self::Norm);
    /// Divides `self` by n.
    fn unscale_mut(&mut self, n: Self::Norm);
}

/// # Construction with normalization
impl<T: Normed> Unit<T> {
    /// Normalize the given vector and return it wrapped on a `Unit` structure.
    #[inline]
    pub fn new_normalize(value: T) -> Self {
        Self::new_and_get(value).0
    }

    /// Attempts to normalize the given vector and return it wrapped on a `Unit` structure.
    ///
    /// Returns `None` if the norm was smaller or equal to `min_norm`.
    #[inline]
    pub fn try_new(value: T, min_norm: T::Norm) -> Option<Self>
    where
        T::Norm: RealField,
    {
        Self::try_new_and_get(value, min_norm).map(|res| res.0)
    }

    /// Normalize the given vector and return it wrapped on a `Unit` structure and its norm.
    #[inline]
    pub fn new_and_get(mut value: T) -> (Self, T::Norm) {
        let n = value.norm();
        value.unscale_mut(n.clone());
        (Unit { value }, n)
    }

    /// Normalize the given vector and return it wrapped on a `Unit` structure and its norm.
    ///
    /// Returns `None` if the norm was smaller or equal to `min_norm`.
    #[inline]
    pub fn try_new_and_get(mut value: T, min_norm: T::Norm) -> Option<(Self, T::Norm)>
    where
        T::Norm: RealField,
    {
        let sq_norm = value.norm_squared();

        if sq_norm > min_norm.clone() * min_norm {
            let n = sq_norm.simd_sqrt();
            value.unscale_mut(n.clone());
            Some((Unit { value }, n))
        } else {
            None
        }
    }

    /// Normalizes this vector again. This is useful when repeated computations
    /// might cause a drift in the norm because of float inaccuracies.
    ///
    /// Returns the norm before re-normalization. See `.renormalize_fast` for a faster alternative
    /// that may be slightly less accurate if `self` drifted significantly from having a unit length.
    #[inline]
    pub fn renormalize(&mut self) -> T::Norm {
        let n = self.norm();
        self.value.unscale_mut(n.clone());
        n
    }

    /// Normalizes this vector again using a first-order Taylor approximation.
    /// This is useful when repeated computations might cause a drift in the norm
    /// because of float inaccuracies.
    #[inline]
    pub fn renormalize_fast(&mut self) {
        let sq_norm = self.value.norm_squared();
        let three: T::Norm = crate::convert(3.0);
        let half: T::Norm = crate::convert(0.5);
        self.value.scale_mut(half * (three - sq_norm));
    }
}

/// # Data extraction and construction without normalization
impl<T> Unit<T> {
    /// Wraps the given value, assuming it is already normalized.
    #[inline]
    pub const fn new_unchecked(value: T) -> Self {
        Unit { value }
    }

    /// Wraps the given reference, assuming it is already normalized.
    #[inline]
    pub const fn from_ref_unchecked(value: &T) -> &Self {
        unsafe { &*(value as *const T as *const Self) }
    }

    /// Retrieves the underlying value.
    #[inline]
    pub fn into_inner(self) -> T {
        self.value
    }

    /// Retrieves the underlying value.
    /// Deprecated: use [`Unit::into_inner`] instead.
    #[deprecated(note = "use `.into_inner()` instead")]
    #[inline]
    pub fn unwrap(self) -> T {
        self.value
    }

    /// Returns a mutable reference to the underlying value. This is `_unchecked` because modifying
    /// the underlying value in such a way that it no longer has unit length may lead to unexpected
    /// results.
    #[inline]
    pub const fn as_mut_unchecked(&mut self) -> &mut T {
        &mut self.value
    }
}

impl<T> AsRef<T> for Unit<T> {
    #[inline]
    fn as_ref(&self) -> &T {
        &self.value
    }
}

/*
/*
 *
 * Conversions.
 *
 */
impl<T: NormedSpace> SubsetOf<T> for Unit<T>
where T::RealField: RelativeEq
{
    #[inline]
    fn to_superset(&self) -> T {
        self.clone().into_inner()
    }

    #[inline]
    fn is_in_subset(value: &T) -> bool {
        relative_eq!(value.norm_squared(), crate::one())
    }

    #[inline]
    fn from_superset_unchecked(value: &T) -> Self {
        Unit::new_normalize(value.clone()) // We still need to re-normalize because the condition is inexact.
    }
}

// impl<T: RelativeEq> RelativeEq for Unit<T> {
//     type Epsilon = T::Epsilon;
//
//     #[inline]
//     fn default_epsilon() -> Self::Epsilon {
//         T::default_epsilon()
//     }
//
//     #[inline]
//     fn default_max_relative() -> Self::Epsilon {
//         T::default_max_relative()
//     }
//
//     #[inline]
//     fn default_max_ulps() -> u32 {
//         T::default_max_ulps()
//     }
//
//     #[inline]
//     fn relative_eq(&self, other: &Self, epsilon: Self::Epsilon, max_relative: Self::Epsilon) -> bool {
//         self.value.relative_eq(&other.value, epsilon, max_relative)
//     }
//
//     #[inline]
//     fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
//         self.value.ulps_eq(&other.value, epsilon, max_ulps)
//     }
// }
*/
// TODO:re-enable this impl when specialization is possible.
// Currently, it is disabled so that we can have a nice output for the `UnitQuaternion` display.
/*
impl<T: fmt::Display> fmt::Display for Unit<T> {
    // XXX: will not always work correctly due to rounding errors.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.value.fmt(f)
    }
}
*/

impl<T> Deref for Unit<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        unsafe { &*(self as *const Self as *const T) }
    }
}

// NOTE: we can't use a generic implementation for `Unit<T>` because
// num_complex::Complex does not implement `From[Complex<...>...]` (and can't
// because of the orphan rules).
impl<T: Scalar + simba::simd::PrimitiveSimdValue, R: Dim, C: Dim>
    From<[Unit<OMatrix<T::Element, R, C>>; 2]> for Unit<OMatrix<T, R, C>>
where
    T: From<[<T as simba::simd::SimdValue>::Element; 2]>,
    T::Element: Scalar,
    DefaultAllocator: Allocator<R, C>,
{
    #[inline]
    fn from(arr: [Unit<OMatrix<T::Element, R, C>>; 2]) -> Self {
        Self::new_unchecked(OMatrix::from([
            arr[0].clone().into_inner(),
            arr[1].clone().into_inner(),
        ]))
    }
}

impl<T: Scalar + simba::simd::PrimitiveSimdValue, R: Dim, C: Dim>
    From<[Unit<OMatrix<T::Element, R, C>>; 4]> for Unit<OMatrix<T, R, C>>
where
    T: From<[<T as simba::simd::SimdValue>::Element; 4]>,
    T::Element: Scalar,
    DefaultAllocator: Allocator<R, C>,
{
    #[inline]
    fn from(arr: [Unit<OMatrix<T::Element, R, C>>; 4]) -> Self {
        Self::new_unchecked(OMatrix::from([
            arr[0].clone().into_inner(),
            arr[1].clone().into_inner(),
            arr[2].clone().into_inner(),
            arr[3].clone().into_inner(),
        ]))
    }
}

impl<T: Scalar + simba::simd::PrimitiveSimdValue, R: Dim, C: Dim>
    From<[Unit<OMatrix<T::Element, R, C>>; 8]> for Unit<OMatrix<T, R, C>>
where
    T: From<[<T as simba::simd::SimdValue>::Element; 8]>,
    T::Element: Scalar,
    DefaultAllocator: Allocator<R, C>,
{
    #[inline]
    fn from(arr: [Unit<OMatrix<T::Element, R, C>>; 8]) -> Self {
        Self::new_unchecked(OMatrix::from([
            arr[0].clone().into_inner(),
            arr[1].clone().into_inner(),
            arr[2].clone().into_inner(),
            arr[3].clone().into_inner(),
            arr[4].clone().into_inner(),
            arr[5].clone().into_inner(),
            arr[6].clone().into_inner(),
            arr[7].clone().into_inner(),
        ]))
    }
}

impl<T: Scalar + simba::simd::PrimitiveSimdValue, R: Dim, C: Dim>
    From<[Unit<OMatrix<T::Element, R, C>>; 16]> for Unit<OMatrix<T, R, C>>
where
    T: From<[<T as simba::simd::SimdValue>::Element; 16]>,
    T::Element: Scalar,
    DefaultAllocator: Allocator<R, C>,
{
    #[inline]
    fn from(arr: [Unit<OMatrix<T::Element, R, C>>; 16]) -> Self {
        Self::new_unchecked(OMatrix::from([
            arr[0].clone().into_inner(),
            arr[1].clone().into_inner(),
            arr[2].clone().into_inner(),
            arr[3].clone().into_inner(),
            arr[4].clone().into_inner(),
            arr[5].clone().into_inner(),
            arr[6].clone().into_inner(),
            arr[7].clone().into_inner(),
            arr[8].clone().into_inner(),
            arr[9].clone().into_inner(),
            arr[10].clone().into_inner(),
            arr[11].clone().into_inner(),
            arr[12].clone().into_inner(),
            arr[13].clone().into_inner(),
            arr[14].clone().into_inner(),
            arr[15].clone().into_inner(),
        ]))
    }
}
