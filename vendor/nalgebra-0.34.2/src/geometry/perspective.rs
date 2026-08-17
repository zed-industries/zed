// Needed otherwise the rkyv macros generate code incompatible with rust-2024
#![cfg_attr(feature = "rkyv-serialize", allow(unsafe_op_in_unsafe_fn))]

#[cfg(feature = "arbitrary")]
use quickcheck::{Arbitrary, Gen};

#[cfg(feature = "rand-no-std")]
use rand::{
    Rng,
    distr::{Distribution, StandardUniform},
};

#[cfg(feature = "serde-serialize-no-std")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

use simba::scalar::RealField;

use crate::base::dimension::U3;
use crate::base::storage::Storage;
use crate::base::{Matrix4, Vector, Vector3};

use crate::geometry::{Point3, Projective3};

#[cfg(feature = "rkyv-serialize")]
use rkyv::bytecheck;

/// A 3D perspective projection stored as a homogeneous 4x4 matrix.
#[repr(C)]
#[cfg_attr(
    feature = "rkyv-serialize-no-std",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize),
    archive(
        as = "Perspective3<T::Archived>",
        bound(archive = "
        T: rkyv::Archive,
        Matrix4<T>: rkyv::Archive<Archived = Matrix4<T::Archived>>
    ")
    )
)]
#[cfg_attr(feature = "rkyv-serialize", derive(bytecheck::CheckBytes))]
#[derive(Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Perspective3<T> {
    matrix: Matrix4<T>,
}

impl<T: RealField> fmt::Debug for Perspective3<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        self.matrix.fmt(f)
    }
}

impl<T: RealField> PartialEq for Perspective3<T> {
    #[inline]
    fn eq(&self, right: &Self) -> bool {
        self.matrix == right.matrix
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl<T> bytemuck::Zeroable for Perspective3<T>
where
    T: RealField + bytemuck::Zeroable,
    Matrix4<T>: bytemuck::Zeroable,
{
}

#[cfg(feature = "bytemuck")]
unsafe impl<T> bytemuck::Pod for Perspective3<T>
where
    T: RealField + bytemuck::Pod,
    Matrix4<T>: bytemuck::Pod,
{
}

#[cfg(feature = "serde-serialize-no-std")]
impl<T: RealField + Serialize> Serialize for Perspective3<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.matrix.serialize(serializer)
    }
}

#[cfg(feature = "serde-serialize-no-std")]
impl<'a, T: RealField + Deserialize<'a>> Deserialize<'a> for Perspective3<T> {
    fn deserialize<Des>(deserializer: Des) -> Result<Self, Des::Error>
    where
        Des: Deserializer<'a>,
    {
        let matrix = Matrix4::<T>::deserialize(deserializer)?;

        Ok(Self::from_matrix_unchecked(matrix))
    }
}

impl<T> Perspective3<T> {
    /// Wraps the given matrix to interpret it as a 3D perspective matrix.
    ///
    /// It is not checked whether or not the given matrix actually represents a perspective
    /// projection.
    #[inline]
    pub const fn from_matrix_unchecked(matrix: Matrix4<T>) -> Self {
        Self { matrix }
    }
}

impl<T: RealField> Perspective3<T> {
    /// Creates a new perspective matrix from the aspect ratio, y field of view, and near/far planes.
    pub fn new(aspect: T, fovy: T, znear: T, zfar: T) -> Self {
        assert!(
            relative_ne!(zfar, znear),
            "The near-plane and far-plane must not be superimposed."
        );
        assert!(
            !relative_eq!(aspect, T::zero()),
            "The aspect ratio must not be zero."
        );

        let matrix = Matrix4::identity();
        let mut res = Self::from_matrix_unchecked(matrix);

        res.set_fovy(fovy);
        res.set_aspect(aspect);
        res.set_znear_and_zfar(znear, zfar);

        res.matrix[(3, 3)] = T::zero();
        res.matrix[(3, 2)] = -T::one();

        res
    }

    /// Retrieves the inverse of the underlying homogeneous matrix.
    #[inline]
    #[must_use]
    pub fn inverse(&self) -> Matrix4<T> {
        let mut res = self.clone().to_homogeneous();

        res[(0, 0)] = T::one() / self.matrix[(0, 0)].clone();
        res[(1, 1)] = T::one() / self.matrix[(1, 1)].clone();
        res[(2, 2)] = T::zero();

        let m23 = self.matrix[(2, 3)].clone();
        let m32 = self.matrix[(3, 2)].clone();

        res[(2, 3)] = T::one() / m32.clone();
        res[(3, 2)] = T::one() / m23.clone();
        res[(3, 3)] = -self.matrix[(2, 2)].clone() / (m23 * m32);

        res
    }

    /// Computes the corresponding homogeneous matrix.
    #[inline]
    #[must_use]
    pub fn to_homogeneous(self) -> Matrix4<T> {
        self.matrix.clone_owned()
    }

    /// A reference to the underlying homogeneous transformation matrix.
    #[inline]
    #[must_use]
    pub const fn as_matrix(&self) -> &Matrix4<T> {
        &self.matrix
    }

    /// A reference to this transformation seen as a `Projective3`.
    #[inline]
    #[must_use]
    pub const fn as_projective(&self) -> &Projective3<T> {
        unsafe { &*(self as *const Perspective3<T> as *const Projective3<T>) }
    }

    /// This transformation seen as a `Projective3`.
    #[inline]
    #[must_use]
    pub fn to_projective(self) -> Projective3<T> {
        Projective3::from_matrix_unchecked(self.matrix)
    }

    /// Retrieves the underlying homogeneous matrix.
    #[inline]
    pub fn into_inner(self) -> Matrix4<T> {
        self.matrix
    }

    /// Retrieves the underlying homogeneous matrix.
    /// Deprecated: Use [`Perspective3::into_inner`] instead.
    #[deprecated(note = "use `.into_inner()` instead")]
    #[inline]
    pub fn unwrap(self) -> Matrix4<T> {
        self.matrix
    }

    /// Gets the `width / height` aspect ratio of the view frustum.
    #[inline]
    #[must_use]
    pub fn aspect(&self) -> T {
        self.matrix[(1, 1)].clone() / self.matrix[(0, 0)].clone()
    }

    /// Gets the y field of view of the view frustum.
    #[inline]
    #[must_use]
    pub fn fovy(&self) -> T {
        (T::one() / self.matrix[(1, 1)].clone()).atan() * crate::convert(2.0)
    }

    /// Gets the near plane offset of the view frustum.
    #[inline]
    #[must_use]
    pub fn znear(&self) -> T {
        let ratio =
            (-self.matrix[(2, 2)].clone() + T::one()) / (-self.matrix[(2, 2)].clone() - T::one());

        self.matrix[(2, 3)].clone() / (ratio * crate::convert(2.0))
            - self.matrix[(2, 3)].clone() / crate::convert(2.0)
    }

    /// Gets the far plane offset of the view frustum.
    #[inline]
    #[must_use]
    pub fn zfar(&self) -> T {
        let ratio =
            (-self.matrix[(2, 2)].clone() + T::one()) / (-self.matrix[(2, 2)].clone() - T::one());

        (self.matrix[(2, 3)].clone() - ratio * self.matrix[(2, 3)].clone()) / crate::convert(2.0)
    }

    // TODO: add a method to retrieve znear and zfar simultaneously?

    // TODO: when we get specialization, specialize the Mul impl instead.
    /// Projects a point. Faster than matrix multiplication.
    #[inline]
    #[must_use]
    pub fn project_point(&self, p: &Point3<T>) -> Point3<T> {
        let inverse_denom = -T::one() / p[2].clone();
        Point3::new(
            self.matrix[(0, 0)].clone() * p[0].clone() * inverse_denom.clone(),
            self.matrix[(1, 1)].clone() * p[1].clone() * inverse_denom.clone(),
            (self.matrix[(2, 2)].clone() * p[2].clone() + self.matrix[(2, 3)].clone())
                * inverse_denom,
        )
    }

    /// Un-projects a point. Faster than multiplication by the matrix inverse.
    #[inline]
    #[must_use]
    pub fn unproject_point(&self, p: &Point3<T>) -> Point3<T> {
        let inverse_denom =
            self.matrix[(2, 3)].clone() / (p[2].clone() + self.matrix[(2, 2)].clone());

        Point3::new(
            p[0].clone() * inverse_denom.clone() / self.matrix[(0, 0)].clone(),
            p[1].clone() * inverse_denom.clone() / self.matrix[(1, 1)].clone(),
            -inverse_denom,
        )
    }

    // TODO: when we get specialization, specialize the Mul impl instead.
    /// Projects a vector. Faster than matrix multiplication.
    #[inline]
    #[must_use]
    pub fn project_vector<SB>(&self, p: &Vector<T, U3, SB>) -> Vector3<T>
    where
        SB: Storage<T, U3>,
    {
        let inverse_denom = -T::one() / p[2].clone();
        Vector3::new(
            self.matrix[(0, 0)].clone() * p[0].clone() * inverse_denom.clone(),
            self.matrix[(1, 1)].clone() * p[1].clone() * inverse_denom,
            self.matrix[(2, 2)].clone(),
        )
    }

    /// Updates this perspective matrix with a new `width / height` aspect ratio of the view
    /// frustum.
    #[inline]
    pub fn set_aspect(&mut self, aspect: T) {
        assert!(
            !relative_eq!(aspect, T::zero()),
            "The aspect ratio must not be zero."
        );
        self.matrix[(0, 0)] = self.matrix[(1, 1)].clone() / aspect;
    }

    /// Updates this perspective with a new y field of view of the view frustum.
    #[inline]
    pub fn set_fovy(&mut self, fovy: T) {
        let old_m22 = self.matrix[(1, 1)].clone();
        let new_m22 = T::one() / (fovy / crate::convert(2.0)).tan();
        self.matrix[(1, 1)] = new_m22.clone();
        self.matrix[(0, 0)] *= new_m22 / old_m22;
    }

    /// Updates this perspective matrix with a new near plane offset of the view frustum.
    #[inline]
    pub fn set_znear(&mut self, znear: T) {
        let zfar = self.zfar();
        self.set_znear_and_zfar(znear, zfar);
    }

    /// Updates this perspective matrix with a new far plane offset of the view frustum.
    #[inline]
    pub fn set_zfar(&mut self, zfar: T) {
        let znear = self.znear();
        self.set_znear_and_zfar(znear, zfar);
    }

    /// Updates this perspective matrix with new near and far plane offsets of the view frustum.
    #[inline]
    pub fn set_znear_and_zfar(&mut self, znear: T, zfar: T) {
        self.matrix[(2, 2)] = (zfar.clone() + znear.clone()) / (znear.clone() - zfar.clone());
        self.matrix[(2, 3)] = zfar.clone() * znear.clone() * crate::convert(2.0) / (znear - zfar);
    }
}

#[cfg(feature = "rand-no-std")]
impl<T: RealField> Distribution<Perspective3<T>> for StandardUniform
where
    StandardUniform: Distribution<T>,
{
    /// Generate an arbitrary random variate for testing purposes.
    fn sample<R: Rng + ?Sized>(&self, r: &mut R) -> Perspective3<T> {
        use crate::base::helper;
        let znear = r.random();
        let zfar = helper::reject_rand(r, |x: &T| !(x.clone() - znear.clone()).is_zero());
        let aspect = helper::reject_rand(r, |x: &T| !x.is_zero());

        Perspective3::new(aspect, r.random(), znear, zfar)
    }
}

#[cfg(feature = "arbitrary")]
impl<T: RealField + Arbitrary> Arbitrary for Perspective3<T> {
    fn arbitrary(g: &mut Gen) -> Self {
        use crate::base::helper;
        let znear: T = Arbitrary::arbitrary(g);
        let zfar = helper::reject(g, |x: &T| !(x.clone() - znear.clone()).is_zero());
        let aspect = helper::reject(g, |x: &T| !x.is_zero());

        Self::new(aspect, Arbitrary::arbitrary(g), znear, zfar)
    }
}

impl<T: RealField> From<Perspective3<T>> for Matrix4<T> {
    #[inline]
    fn from(pers: Perspective3<T>) -> Self {
        pers.into_inner()
    }
}
