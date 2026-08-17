extern crate nalgebra as na;

use na::{Matrix3, Point3, Vector3};

fn main() {
    let v = Vector3::new(1.0f32, 0.0, 1.0);
    let p = Point3::new(1.0f32, 0.0, 1.0);
    let m = na::one::<Matrix3<f32>>();

    // Convert to arrays.
    let v_array = v.as_slice();
    let p_array = p.coords.as_slice();
    let m_array = m.as_slice();

    // Get data pointers.
    let v_pointer = v_array.as_ptr();
    let p_pointer = p_array.as_ptr();
    let m_pointer = m_array.as_ptr();

    /* Then pass the raw pointers to some graphics API. */

    #[allow(clippy::float_cmp)]
    unsafe {
        assert_eq!(*v_pointer, 1.0);
        assert_eq!(*v_pointer.offset(1), 0.0);
        assert_eq!(*v_pointer.offset(2), 1.0);

        assert_eq!(*p_pointer, 1.0);
        assert_eq!(*p_pointer.offset(1), 0.0);
        assert_eq!(*p_pointer.offset(2), 1.0);

        assert_eq!(*m_pointer, 1.0);
        assert_eq!(*m_pointer.offset(4), 1.0);
        assert_eq!(*m_pointer.offset(8), 1.0);
    }
}
