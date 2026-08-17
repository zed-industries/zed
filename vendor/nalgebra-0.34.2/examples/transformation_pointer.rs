extern crate nalgebra as na;

use na::{Isometry3, Vector3};

fn main() {
    let iso = Isometry3::new(Vector3::new(1.0f32, 0.0, 1.0), na::zero());

    // Compute the homogeneous coordinates first.
    let iso_matrix = iso.to_homogeneous();
    let iso_array = iso_matrix.as_slice();
    let iso_pointer = iso_array.as_ptr();

    /* Then pass the raw pointer to some graphics API. */

    #[allow(clippy::float_cmp)]
    unsafe {
        assert_eq!(*iso_pointer, 1.0);
        assert_eq!(*iso_pointer.offset(5), 1.0);
        assert_eq!(*iso_pointer.offset(10), 1.0);
        assert_eq!(*iso_pointer.offset(15), 1.0);

        assert_eq!(*iso_pointer.offset(12), 1.0);
        assert_eq!(*iso_pointer.offset(13), 0.0);
        assert_eq!(*iso_pointer.offset(14), 1.0);
    }
}
