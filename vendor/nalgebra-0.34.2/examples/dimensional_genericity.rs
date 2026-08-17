extern crate nalgebra as na;

use na::allocator::Allocator;
use na::dimension::Dim;
use na::{DefaultAllocator, OVector, RealField, Unit, Vector2, Vector3};

/// Reflects a vector wrt. the hyperplane with normal `plane_normal`.
fn reflect_wrt_hyperplane_with_dimensional_genericity<T: RealField, D: Dim>(
    plane_normal: &Unit<OVector<T, D>>,
    vector: &OVector<T, D>,
) -> OVector<T, D>
where
    T: RealField,
    D: Dim,
    DefaultAllocator: Allocator<D>,
{
    let n = plane_normal.as_ref(); // Get the underlying V.
    vector - n * (n.dot(vector) * na::convert(2.0))
}

/// Reflects a 2D vector wrt. the 2D line with normal `plane_normal`.
fn reflect_wrt_hyperplane2<T>(plane_normal: &Unit<Vector2<T>>, vector: &Vector2<T>) -> Vector2<T>
where
    T: RealField,
{
    let n = plane_normal.as_ref(); // Get the underlying Vector2
    vector - n * (n.dot(vector) * na::convert(2.0))
}

/// Reflects a 3D vector wrt. the 3D plane with normal `plane_normal`.
/// /!\ This is an exact replicate of `reflect_wrt_hyperplane2`, but for 3D.
fn reflect_wrt_hyperplane3<T>(plane_normal: &Unit<Vector3<T>>, vector: &Vector3<T>) -> Vector3<T>
where
    T: RealField,
{
    let n = plane_normal.as_ref(); // Get the underlying Vector3
    vector - n * (n.dot(vector) * na::convert(2.0))
}

fn main() {
    let plane2 = Vector2::y_axis(); // 2D plane normal.
    let plane3 = Vector3::y_axis(); // 3D plane normal.

    let v2 = Vector2::new(1.0, 2.0); // 2D vector to be reflected.
    let v3 = Vector3::new(1.0, 2.0, 3.0); // 3D vector to be reflected.

    // We can call the same function for 2D and 3D.
    assert_eq!(
        reflect_wrt_hyperplane_with_dimensional_genericity(&plane2, &v2).y,
        -2.0
    );
    assert_eq!(
        reflect_wrt_hyperplane_with_dimensional_genericity(&plane3, &v3).y,
        -2.0
    );

    // Call each specific implementation depending on the dimension.
    assert_eq!(reflect_wrt_hyperplane2(&plane2, &v2).y, -2.0);
    assert_eq!(reflect_wrt_hyperplane3(&plane3, &v3).y, -2.0);
}
