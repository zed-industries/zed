/*
 *
 * Computer-graphics specific implementations.
 * Currently, it is mostly implemented for homogeneous matrices in 2- and 3-space.
 *
 */

use num::{One, Zero};

use crate::base::allocator::Allocator;
use crate::base::dimension::{DimName, DimNameDiff, DimNameSub, U1};
use crate::base::storage::{Storage, StorageMut};
use crate::base::{
    Const, DefaultAllocator, Matrix3, Matrix4, OMatrix, OVector, Scalar, SquareMatrix, Unit,
    Vector, Vector2, Vector3,
};
use crate::geometry::{
    Isometry, IsometryMatrix3, Orthographic3, Perspective3, Point, Point2, Point3, Rotation2,
    Rotation3,
};

use simba::scalar::{ClosedAddAssign, ClosedMulAssign, RealField};

/// # Translation and scaling in any dimension
impl<T, D: DimName> OMatrix<T, D, D>
where
    T: Scalar + Zero + One,
    DefaultAllocator: Allocator<D, D>,
{
    /// Creates a new homogeneous matrix that applies the same scaling factor on each dimension.
    #[inline]
    pub fn new_scaling(scaling: T) -> Self {
        let mut res = Self::from_diagonal_element(scaling);
        res[(D::DIM - 1, D::DIM - 1)] = T::one();

        res
    }

    /// Creates a new homogeneous matrix that applies a distinct scaling factor for each dimension.
    #[inline]
    pub fn new_nonuniform_scaling<SB>(scaling: &Vector<T, DimNameDiff<D, U1>, SB>) -> Self
    where
        D: DimNameSub<U1>,
        SB: Storage<T, DimNameDiff<D, U1>>,
    {
        let mut res = Self::identity();
        for i in 0..scaling.len() {
            res[(i, i)] = scaling[i].clone();
        }

        res
    }

    /// Creates a new homogeneous matrix that applies a pure translation.
    #[inline]
    pub fn new_translation<SB>(translation: &Vector<T, DimNameDiff<D, U1>, SB>) -> Self
    where
        D: DimNameSub<U1>,
        SB: Storage<T, DimNameDiff<D, U1>>,
    {
        let mut res = Self::identity();
        res.generic_view_mut((0, D::DIM - 1), (DimNameDiff::<D, U1>::name(), Const::<1>))
            .copy_from(translation);

        res
    }
}

/// # 2D transformations as a Matrix3
impl<T: RealField> Matrix3<T> {
    /// Builds a 2 dimensional homogeneous rotation matrix from an angle in radian.
    #[inline]
    pub fn new_rotation(angle: T) -> Self {
        Rotation2::new(angle).to_homogeneous()
    }

    /// Creates a new homogeneous matrix that applies a scaling factor for each dimension with respect to point.
    ///
    /// Can be used to implement `zoom_to` functionality.
    #[inline]
    pub fn new_nonuniform_scaling_wrt_point(scaling: &Vector2<T>, pt: &Point2<T>) -> Self {
        let zero = T::zero();
        let one = T::one();
        Matrix3::new(
            scaling.x.clone(),
            zero.clone(),
            pt.x.clone() - pt.x.clone() * scaling.x.clone(),
            zero.clone(),
            scaling.y.clone(),
            pt.y.clone() - pt.y.clone() * scaling.y.clone(),
            zero.clone(),
            zero,
            one,
        )
    }
}

/// # 3D transformations as a Matrix4
impl<T: RealField> Matrix4<T> {
    /// Builds a 3D homogeneous rotation matrix from an axis and an angle (multiplied together).
    ///
    /// Returns the identity matrix if the given argument is zero.
    #[inline]
    pub fn new_rotation(axisangle: Vector3<T>) -> Self {
        Rotation3::new(axisangle).to_homogeneous()
    }

    /// Builds a 3D homogeneous rotation matrix from an axis and an angle (multiplied together).
    ///
    /// Returns the identity matrix if the given argument is zero.
    #[inline]
    pub fn new_rotation_wrt_point(axisangle: Vector3<T>, pt: Point3<T>) -> Self {
        let rot = Rotation3::from_scaled_axis(axisangle);
        Isometry::rotation_wrt_point(rot, pt).to_homogeneous()
    }

    /// Creates a new homogeneous matrix that applies a scaling factor for each dimension with respect to point.
    ///
    /// Can be used to implement `zoom_to` functionality.
    #[inline]
    pub fn new_nonuniform_scaling_wrt_point(scaling: &Vector3<T>, pt: &Point3<T>) -> Self {
        let zero = T::zero();
        let one = T::one();
        Matrix4::new(
            scaling.x.clone(),
            zero.clone(),
            zero.clone(),
            pt.x.clone() - pt.x.clone() * scaling.x.clone(),
            zero.clone(),
            scaling.y.clone(),
            zero.clone(),
            pt.y.clone() - pt.y.clone() * scaling.y.clone(),
            zero.clone(),
            zero.clone(),
            scaling.z.clone(),
            pt.z.clone() - pt.z.clone() * scaling.z.clone(),
            zero.clone(),
            zero.clone(),
            zero,
            one,
        )
    }

    /// Builds a 3D homogeneous rotation matrix from an axis and an angle (multiplied together).
    ///
    /// Returns the identity matrix if the given argument is zero.
    /// This is identical to `Self::new_rotation`.
    #[inline]
    pub fn from_scaled_axis(axisangle: Vector3<T>) -> Self {
        Rotation3::from_scaled_axis(axisangle).to_homogeneous()
    }

    /// Creates a new rotation from Euler angles.
    ///
    /// The primitive rotations are applied in order: 1 roll − 2 pitch − 3 yaw.
    pub fn from_euler_angles(roll: T, pitch: T, yaw: T) -> Self {
        Rotation3::from_euler_angles(roll, pitch, yaw).to_homogeneous()
    }

    /// Builds a 3D homogeneous rotation matrix from an axis and a rotation angle.
    pub fn from_axis_angle(axis: &Unit<Vector3<T>>, angle: T) -> Self {
        Rotation3::from_axis_angle(axis, angle).to_homogeneous()
    }

    /// Creates a new homogeneous matrix for an orthographic projection.
    #[inline]
    pub fn new_orthographic(left: T, right: T, bottom: T, top: T, znear: T, zfar: T) -> Self {
        Orthographic3::new(left, right, bottom, top, znear, zfar).into_inner()
    }

    /// Creates a new homogeneous matrix for a perspective projection.
    #[inline]
    pub fn new_perspective(aspect: T, fovy: T, znear: T, zfar: T) -> Self {
        Perspective3::new(aspect, fovy, znear, zfar).into_inner()
    }

    /// Creates an isometry that corresponds to the local frame of an observer standing at the
    /// point `eye` and looking toward `target`.
    ///
    /// It maps the view direction `target - eye` to the positive `z` axis and the origin to the
    /// `eye`.
    #[inline]
    pub fn face_towards(eye: &Point3<T>, target: &Point3<T>, up: &Vector3<T>) -> Self {
        IsometryMatrix3::face_towards(eye, target, up).to_homogeneous()
    }

    /// Deprecated: Use [`Matrix4::face_towards`] instead.
    #[deprecated(note = "renamed to `face_towards`")]
    pub fn new_observer_frame(eye: &Point3<T>, target: &Point3<T>, up: &Vector3<T>) -> Self {
        Matrix4::face_towards(eye, target, up)
    }

    /// Builds a right-handed look-at view matrix.
    #[inline]
    pub fn look_at_rh(eye: &Point3<T>, target: &Point3<T>, up: &Vector3<T>) -> Self {
        IsometryMatrix3::look_at_rh(eye, target, up).to_homogeneous()
    }

    /// Builds a left-handed look-at view matrix.
    #[inline]
    pub fn look_at_lh(eye: &Point3<T>, target: &Point3<T>, up: &Vector3<T>) -> Self {
        IsometryMatrix3::look_at_lh(eye, target, up).to_homogeneous()
    }
}

/// # Append/prepend translation and scaling
impl<T: Scalar + Zero + One + ClosedMulAssign + ClosedAddAssign, D: DimName, S: Storage<T, D, D>>
    SquareMatrix<T, D, S>
{
    /// Computes the transformation equal to `self` followed by an uniform scaling factor.
    #[inline]
    #[must_use = "Did you mean to use append_scaling_mut()?"]
    pub fn append_scaling(&self, scaling: T) -> OMatrix<T, D, D>
    where
        D: DimNameSub<U1>,
        DefaultAllocator: Allocator<D, D>,
    {
        let mut res = self.clone_owned();
        res.append_scaling_mut(scaling);
        res
    }

    /// Computes the transformation equal to an uniform scaling factor followed by `self`.
    #[inline]
    #[must_use = "Did you mean to use prepend_scaling_mut()?"]
    pub fn prepend_scaling(&self, scaling: T) -> OMatrix<T, D, D>
    where
        D: DimNameSub<U1>,
        DefaultAllocator: Allocator<D, D>,
    {
        let mut res = self.clone_owned();
        res.prepend_scaling_mut(scaling);
        res
    }

    /// Computes the transformation equal to `self` followed by a non-uniform scaling factor.
    #[inline]
    #[must_use = "Did you mean to use append_nonuniform_scaling_mut()?"]
    pub fn append_nonuniform_scaling<SB>(
        &self,
        scaling: &Vector<T, DimNameDiff<D, U1>, SB>,
    ) -> OMatrix<T, D, D>
    where
        D: DimNameSub<U1>,
        SB: Storage<T, DimNameDiff<D, U1>>,
        DefaultAllocator: Allocator<D, D>,
    {
        let mut res = self.clone_owned();
        res.append_nonuniform_scaling_mut(scaling);
        res
    }

    /// Computes the transformation equal to a non-uniform scaling factor followed by `self`.
    #[inline]
    #[must_use = "Did you mean to use prepend_nonuniform_scaling_mut()?"]
    pub fn prepend_nonuniform_scaling<SB>(
        &self,
        scaling: &Vector<T, DimNameDiff<D, U1>, SB>,
    ) -> OMatrix<T, D, D>
    where
        D: DimNameSub<U1>,
        SB: Storage<T, DimNameDiff<D, U1>>,
        DefaultAllocator: Allocator<D, D>,
    {
        let mut res = self.clone_owned();
        res.prepend_nonuniform_scaling_mut(scaling);
        res
    }

    /// Computes the transformation equal to `self` followed by a translation.
    #[inline]
    #[must_use = "Did you mean to use append_translation_mut()?"]
    pub fn append_translation<SB>(
        &self,
        shift: &Vector<T, DimNameDiff<D, U1>, SB>,
    ) -> OMatrix<T, D, D>
    where
        D: DimNameSub<U1>,
        SB: Storage<T, DimNameDiff<D, U1>>,
        DefaultAllocator: Allocator<D, D>,
    {
        let mut res = self.clone_owned();
        res.append_translation_mut(shift);
        res
    }

    /// Computes the transformation equal to a translation followed by `self`.
    #[inline]
    #[must_use = "Did you mean to use prepend_translation_mut()?"]
    pub fn prepend_translation<SB>(
        &self,
        shift: &Vector<T, DimNameDiff<D, U1>, SB>,
    ) -> OMatrix<T, D, D>
    where
        D: DimNameSub<U1>,
        SB: Storage<T, DimNameDiff<D, U1>>,
        DefaultAllocator: Allocator<D, D> + Allocator<DimNameDiff<D, U1>>,
    {
        let mut res = self.clone_owned();
        res.prepend_translation_mut(shift);
        res
    }

    /// Computes in-place the transformation equal to `self` followed by an uniform scaling factor.
    #[inline]
    pub fn append_scaling_mut(&mut self, scaling: T)
    where
        S: StorageMut<T, D, D>,
        D: DimNameSub<U1>,
    {
        let mut to_scale = self.rows_generic_mut(0, DimNameDiff::<D, U1>::name());
        to_scale *= scaling;
    }

    /// Computes in-place the transformation equal to an uniform scaling factor followed by `self`.
    #[inline]
    pub fn prepend_scaling_mut(&mut self, scaling: T)
    where
        S: StorageMut<T, D, D>,
        D: DimNameSub<U1>,
    {
        let mut to_scale = self.columns_generic_mut(0, DimNameDiff::<D, U1>::name());
        to_scale *= scaling;
    }

    /// Computes in-place the transformation equal to `self` followed by a non-uniform scaling factor.
    #[inline]
    pub fn append_nonuniform_scaling_mut<SB>(&mut self, scaling: &Vector<T, DimNameDiff<D, U1>, SB>)
    where
        S: StorageMut<T, D, D>,
        D: DimNameSub<U1>,
        SB: Storage<T, DimNameDiff<D, U1>>,
    {
        for i in 0..scaling.len() {
            let mut to_scale = self.fixed_rows_mut::<1>(i);
            to_scale *= scaling[i].clone();
        }
    }

    /// Computes in-place the transformation equal to a non-uniform scaling factor followed by `self`.
    #[inline]
    pub fn prepend_nonuniform_scaling_mut<SB>(
        &mut self,
        scaling: &Vector<T, DimNameDiff<D, U1>, SB>,
    ) where
        S: StorageMut<T, D, D>,
        D: DimNameSub<U1>,
        SB: Storage<T, DimNameDiff<D, U1>>,
    {
        for i in 0..scaling.len() {
            let mut to_scale = self.fixed_columns_mut::<1>(i);
            to_scale *= scaling[i].clone();
        }
    }

    /// Computes the transformation equal to `self` followed by a translation.
    #[inline]
    pub fn append_translation_mut<SB>(&mut self, shift: &Vector<T, DimNameDiff<D, U1>, SB>)
    where
        S: StorageMut<T, D, D>,
        D: DimNameSub<U1>,
        SB: Storage<T, DimNameDiff<D, U1>>,
    {
        for i in 0..D::DIM {
            for j in 0..D::DIM - 1 {
                let add = shift[j].clone() * self[(D::DIM - 1, i)].clone();
                self[(j, i)] += add;
            }
        }
    }

    /// Computes the transformation equal to a translation followed by `self`.
    #[inline]
    pub fn prepend_translation_mut<SB>(&mut self, shift: &Vector<T, DimNameDiff<D, U1>, SB>)
    where
        D: DimNameSub<U1>,
        S: StorageMut<T, D, D>,
        SB: Storage<T, DimNameDiff<D, U1>>,
        DefaultAllocator: Allocator<DimNameDiff<D, U1>>,
    {
        let scale = self
            .generic_view((D::DIM - 1, 0), (Const::<1>, DimNameDiff::<D, U1>::name()))
            .tr_dot(shift);
        let post_translation = self.generic_view(
            (0, 0),
            (DimNameDiff::<D, U1>::name(), DimNameDiff::<D, U1>::name()),
        ) * shift;

        self[(D::DIM - 1, D::DIM - 1)] += scale;

        let mut translation =
            self.generic_view_mut((0, D::DIM - 1), (DimNameDiff::<D, U1>::name(), Const::<1>));
        translation += post_translation;
    }
}

/// # Transformation of vectors and points
impl<T: RealField, D: DimNameSub<U1>, S: Storage<T, D, D>> SquareMatrix<T, D, S>
where
    DefaultAllocator: Allocator<D, D>
        + Allocator<DimNameDiff<D, U1>>
        + Allocator<DimNameDiff<D, U1>, DimNameDiff<D, U1>>,
{
    /// Transforms the given vector, assuming the matrix `self` uses homogeneous coordinates.
    #[inline]
    pub fn transform_vector(
        &self,
        v: &OVector<T, DimNameDiff<D, U1>>,
    ) -> OVector<T, DimNameDiff<D, U1>> {
        let transform = self.generic_view(
            (0, 0),
            (DimNameDiff::<D, U1>::name(), DimNameDiff::<D, U1>::name()),
        );
        let normalizer =
            self.generic_view((D::DIM - 1, 0), (Const::<1>, DimNameDiff::<D, U1>::name()));
        let n = normalizer.tr_dot(v);

        if !n.is_zero() {
            return transform * (v / n);
        }

        transform * v
    }
}

impl<T: RealField, S: Storage<T, Const<3>, Const<3>>> SquareMatrix<T, Const<3>, S> {
    /// Transforms the given point, assuming the matrix `self` uses homogeneous coordinates.
    #[inline]
    pub fn transform_point(&self, pt: &Point<T, 2>) -> Point<T, 2> {
        let transform = self.fixed_view::<2, 2>(0, 0);
        let translation = self.fixed_view::<2, 1>(0, 2);
        let normalizer = self.fixed_view::<1, 2>(2, 0);
        let n = normalizer.tr_dot(&pt.coords) + unsafe { self.get_unchecked((2, 2)).clone() };

        if !n.is_zero() {
            (transform * pt + translation) / n
        } else {
            transform * pt + translation
        }
    }
}

impl<T: RealField, S: Storage<T, Const<4>, Const<4>>> SquareMatrix<T, Const<4>, S> {
    /// Transforms the given point, assuming the matrix `self` uses homogeneous coordinates.
    #[inline]
    pub fn transform_point(&self, pt: &Point<T, 3>) -> Point<T, 3> {
        let transform = self.fixed_view::<3, 3>(0, 0);
        let translation = self.fixed_view::<3, 1>(0, 3);
        let normalizer = self.fixed_view::<1, 3>(3, 0);
        let n = normalizer.tr_dot(&pt.coords) + unsafe { self.get_unchecked((3, 3)).clone() };

        if !n.is_zero() {
            (transform * pt + translation) / n
        } else {
            transform * pt + translation
        }
    }
}
