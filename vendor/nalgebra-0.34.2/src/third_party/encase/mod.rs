use crate::{
    ArrayStorage, Const, Matrix, Matrix2, Matrix2x3, Matrix2x4, Matrix3, Matrix3x2, Matrix3x4,
    Matrix4, Matrix4x2, Matrix4x3, MatrixView2, MatrixView2x3, MatrixView2x4, MatrixView3,
    MatrixView3x2, MatrixView3x4, MatrixView4, MatrixView4x2, MatrixView4x3, MatrixViewMut2,
    MatrixViewMut2x3, MatrixViewMut2x4, MatrixViewMut3, MatrixViewMut3x2, MatrixViewMut3x4,
    MatrixViewMut4, MatrixViewMut4x2, MatrixViewMut4x3, OVector, Point, Point2, Point3, Point4,
    SMatrix, Scalar, Vector2, Vector3, Vector4, VectorView2, VectorView3, VectorView4,
    VectorViewMut2, VectorViewMut3, VectorViewMut4,
};
use encase::{
    matrix::{AsMutMatrixParts, AsRefMatrixParts, FromMatrixParts, MatrixScalar, impl_matrix},
    vector::{AsMutVectorParts, AsRefVectorParts, FromVectorParts, VectorScalar, impl_vector},
};

impl_vector!(2, VectorView2<'_, T>);
impl_vector!(2, VectorViewMut2<'_, T>);
impl_vector!(2, Vector2<T>);

impl_vector!(3, VectorView3<'_, T>);
impl_vector!(3, VectorViewMut3<'_, T>);
impl_vector!(3, Vector3<T>);

impl_vector!(4, VectorView4<'_, T>);
impl_vector!(4, VectorViewMut4<'_, T>);
impl_vector!(4, Vector4<T>);

impl_vector!(2, Point2<T>; (T: Scalar); using From);
impl_vector!(3, Point3<T>; (T: Scalar); using From);
impl_vector!(4, Point4<T>; (T: Scalar); using From);

impl_matrix!(2, 2, MatrixView2<'_, T>);
impl_matrix!(2, 2, MatrixViewMut2<'_, T>);
impl_matrix!(2, 2, Matrix2<T>);

impl_matrix!(3, 2, MatrixView2x3<'_, T>);
impl_matrix!(4, 2, MatrixView2x4<'_, T>);
impl_matrix!(2, 3, MatrixView3x2<'_, T>);
impl_matrix!(3, 2, MatrixViewMut2x3<'_, T>);
impl_matrix!(4, 2, MatrixViewMut2x4<'_, T>);
impl_matrix!(2, 3, MatrixViewMut3x2<'_, T>);
impl_matrix!(3, 2, Matrix2x3<T>);
impl_matrix!(4, 2, Matrix2x4<T>);
impl_matrix!(2, 3, Matrix3x2<T>);

impl_matrix!(3, 3, MatrixView3<'_, T>);
impl_matrix!(3, 3, MatrixViewMut3<'_, T>);
impl_matrix!(3, 3, Matrix3<T>);

impl_matrix!(4, 3, MatrixView3x4<'_, T>);
impl_matrix!(2, 4, MatrixView4x2<'_, T>);
impl_matrix!(3, 4, MatrixView4x3<'_, T>);
impl_matrix!(4, 3, MatrixViewMut3x4<'_, T>);
impl_matrix!(2, 4, MatrixViewMut4x2<'_, T>);
impl_matrix!(3, 4, MatrixViewMut4x3<'_, T>);
impl_matrix!(4, 3, Matrix3x4<T>);
impl_matrix!(2, 4, Matrix4x2<T>);
impl_matrix!(3, 4, Matrix4x3<T>);

impl_matrix!(4, 4, MatrixView4<'_, T>);
impl_matrix!(4, 4, MatrixViewMut4<'_, T>);
impl_matrix!(4, 4, Matrix4<T>);

impl<T: VectorScalar, S, const N: usize> AsRefVectorParts<T, N> for Matrix<T, Const<N>, Const<1>, S>
where
    Self: AsRef<[T; N]>,
{
    fn as_ref_parts(&self) -> &[T; N] {
        self.as_ref()
    }
}

impl<T: VectorScalar, S, const N: usize> AsMutVectorParts<T, N> for Matrix<T, Const<N>, Const<1>, S>
where
    Self: AsMut<[T; N]>,
{
    fn as_mut_parts(&mut self) -> &mut [T; N] {
        self.as_mut()
    }
}

impl<T: VectorScalar, const N: usize> FromVectorParts<T, N> for SMatrix<T, N, 1> {
    fn from_parts(parts: [T; N]) -> Self {
        Self::from_array_storage(ArrayStorage([parts]))
    }
}

impl<T: VectorScalar + Scalar, const N: usize> AsRefVectorParts<T, N> for Point<T, N>
where
    OVector<T, Const<N>>: AsRef<[T; N]>,
{
    fn as_ref_parts(&self) -> &[T; N] {
        self.coords.as_ref()
    }
}

impl<T: VectorScalar + Scalar, const N: usize> AsMutVectorParts<T, N> for Point<T, N>
where
    OVector<T, Const<N>>: AsMut<[T; N]>,
{
    fn as_mut_parts(&mut self) -> &mut [T; N] {
        self.coords.as_mut()
    }
}

impl<T: MatrixScalar, S, const C: usize, const R: usize> AsRefMatrixParts<T, C, R>
    for Matrix<T, Const<R>, Const<C>, S>
where
    Self: AsRef<[[T; R]; C]>,
{
    fn as_ref_parts(&self) -> &[[T; R]; C] {
        self.as_ref()
    }
}

impl<T: MatrixScalar, S, const C: usize, const R: usize> AsMutMatrixParts<T, C, R>
    for Matrix<T, Const<R>, Const<C>, S>
where
    Self: AsMut<[[T; R]; C]>,
{
    fn as_mut_parts(&mut self) -> &mut [[T; R]; C] {
        self.as_mut()
    }
}

impl<T: MatrixScalar, const C: usize, const R: usize> FromMatrixParts<T, C, R>
    for SMatrix<T, R, C>
{
    fn from_parts(parts: [[T; R]; C]) -> Self {
        Self::from_array_storage(ArrayStorage(parts))
    }
}
