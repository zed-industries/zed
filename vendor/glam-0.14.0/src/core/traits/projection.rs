use crate::core::traits::{
    matrix::FloatMatrix4x4, quaternion::Quaternion, scalar::FloatEx, vector::*,
};

pub trait ProjectionMatrix<T: FloatEx, V4: FloatVector4<T> + Quaternion<T>>:
    FloatMatrix4x4<T, V4>
{
    /// Creates a right-handed perspective projection matrix with [-1,1] depth range.
    /// This is the same as the OpenGL `gluPerspective` function.
    /// See https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/gluPerspective.xml
    fn perspective_rh_gl(fov_y_radians: T, aspect_ratio: T, z_near: T, z_far: T) -> Self {
        let inv_length = T::ONE / (z_near - z_far);
        let f = T::ONE / (T::HALF * fov_y_radians).tan();
        let a = f / aspect_ratio;
        let b = (z_near + z_far) * inv_length;
        let c = (T::TWO * z_near * z_far) * inv_length;
        Self::from_cols(
            V4::new(a, T::ZERO, T::ZERO, T::ZERO),
            V4::new(T::ZERO, f, T::ZERO, T::ZERO),
            V4::new(T::ZERO, T::ZERO, b, -T::ONE),
            V4::new(T::ZERO, T::ZERO, c, T::ZERO),
        )
    }

    /// Creates a left-handed perspective projection matrix with [0,1] depth range.
    fn perspective_lh(fov_y_radians: T, aspect_ratio: T, z_near: T, z_far: T) -> Self {
        glam_assert!(z_near > T::ZERO && z_far > T::ZERO);
        let (sin_fov, cos_fov) = (T::HALF * fov_y_radians).sin_cos();
        let h = cos_fov / sin_fov;
        let w = h / aspect_ratio;
        let r = z_far / (z_far - z_near);
        Self::from_cols(
            V4::new(w, T::ZERO, T::ZERO, T::ZERO),
            V4::new(T::ZERO, h, T::ZERO, T::ZERO),
            V4::new(T::ZERO, T::ZERO, r, T::ONE),
            V4::new(T::ZERO, T::ZERO, -r * z_near, T::ZERO),
        )
    }

    /// Creates a right-handed perspective projection matrix with [0,1] depth range.
    fn perspective_rh(fov_y_radians: T, aspect_ratio: T, z_near: T, z_far: T) -> Self {
        glam_assert!(z_near > T::ZERO && z_far > T::ZERO);
        let (sin_fov, cos_fov) = (T::HALF * fov_y_radians).sin_cos();
        let h = cos_fov / sin_fov;
        let w = h / aspect_ratio;
        let r = z_far / (z_near - z_far);
        Self::from_cols(
            V4::new(w, T::ZERO, T::ZERO, T::ZERO),
            V4::new(T::ZERO, h, T::ZERO, T::ZERO),
            V4::new(T::ZERO, T::ZERO, r, -T::ONE),
            V4::new(T::ZERO, T::ZERO, r * z_near, T::ZERO),
        )
    }

    /// Creates an infinite left-handed perspective projection matrix with [0,1] depth range.
    fn perspective_infinite_lh(fov_y_radians: T, aspect_ratio: T, z_near: T) -> Self {
        glam_assert!(z_near > T::ZERO);
        let (sin_fov, cos_fov) = (T::HALF * fov_y_radians).sin_cos();
        let h = cos_fov / sin_fov;
        let w = h / aspect_ratio;
        Self::from_cols(
            V4::new(w, T::ZERO, T::ZERO, T::ZERO),
            V4::new(T::ZERO, h, T::ZERO, T::ZERO),
            V4::new(T::ZERO, T::ZERO, T::ONE, T::ONE),
            V4::new(T::ZERO, T::ZERO, -z_near, T::ZERO),
        )
    }

    /// Creates an infinite left-handed perspective projection matrix with [0,1] depth range.
    fn perspective_infinite_reverse_lh(fov_y_radians: T, aspect_ratio: T, z_near: T) -> Self {
        glam_assert!(z_near > T::ZERO);
        let (sin_fov, cos_fov) = (T::HALF * fov_y_radians).sin_cos();
        let h = cos_fov / sin_fov;
        let w = h / aspect_ratio;
        Self::from_cols(
            V4::new(w, T::ZERO, T::ZERO, T::ZERO),
            V4::new(T::ZERO, h, T::ZERO, T::ZERO),
            V4::new(T::ZERO, T::ZERO, T::ZERO, T::ONE),
            V4::new(T::ZERO, T::ZERO, z_near, T::ZERO),
        )
    }

    /// Creates an infinite right-handed perspective projection matrix with
    /// [0,1] depth range.
    fn perspective_infinite_rh(fov_y_radians: T, aspect_ratio: T, z_near: T) -> Self {
        let f = T::ONE / (T::HALF * fov_y_radians).tan();
        Self::from_cols(
            V4::new(f / aspect_ratio, T::ZERO, T::ZERO, T::ZERO),
            V4::new(T::ZERO, f, T::ZERO, T::ZERO),
            V4::new(T::ZERO, T::ZERO, -T::ONE, -T::ONE),
            V4::new(T::ZERO, T::ZERO, -z_near, T::ZERO),
        )
    }

    /// Creates an infinite reverse right-handed perspective projection matrix
    /// with [0,1] depth range.
    fn perspective_infinite_reverse_rh(fov_y_radians: T, aspect_ratio: T, z_near: T) -> Self {
        let f = T::ONE / (T::HALF * fov_y_radians).tan();
        Self::from_cols(
            V4::new(f / aspect_ratio, T::ZERO, T::ZERO, T::ZERO),
            V4::new(T::ZERO, f, T::ZERO, T::ZERO),
            V4::new(T::ZERO, T::ZERO, T::ZERO, -T::ONE),
            V4::new(T::ZERO, T::ZERO, z_near, T::ZERO),
        )
    }

    /// Creates a right-handed orthographic projection matrix with [-1,1] depth
    /// range.  This is the same as the OpenGL `glOrtho` function in OpenGL.
    /// See
    /// https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/glOrtho.xml
    fn orthographic_rh_gl(left: T, right: T, bottom: T, top: T, near: T, far: T) -> Self {
        let a = T::TWO / (right - left);
        let b = T::TWO / (top - bottom);
        let c = -T::TWO / (far - near);
        let tx = -(right + left) / (right - left);
        let ty = -(top + bottom) / (top - bottom);
        let tz = -(far + near) / (far - near);

        Self::from_cols(
            V4::new(a, T::ZERO, T::ZERO, T::ZERO),
            V4::new(T::ZERO, b, T::ZERO, T::ZERO),
            V4::new(T::ZERO, T::ZERO, c, T::ZERO),
            V4::new(tx, ty, tz, T::ONE),
        )
    }

    /// Creates a left-handed orthographic projection matrix with [0,1] depth range.
    fn orthographic_lh(left: T, right: T, bottom: T, top: T, near: T, far: T) -> Self {
        let rcp_width = T::ONE / (right - left);
        let rcp_height = T::ONE / (top - bottom);
        let r = T::ONE / (far - near);
        Self::from_cols(
            V4::new(rcp_width + rcp_width, T::ZERO, T::ZERO, T::ZERO),
            V4::new(T::ZERO, rcp_height + rcp_height, T::ZERO, T::ZERO),
            V4::new(T::ZERO, T::ZERO, r, T::ZERO),
            V4::new(
                -(left + right) * rcp_width,
                -(top + bottom) * rcp_height,
                -r * near,
                T::ONE,
            ),
        )
    }

    /// Creates a right-handed orthographic projection matrix with [0,1] depth range.
    fn orthographic_rh(left: T, right: T, bottom: T, top: T, near: T, far: T) -> Self {
        let rcp_width = T::ONE / (right - left);
        let rcp_height = T::ONE / (top - bottom);
        let r = T::ONE / (near - far);
        Self::from_cols(
            V4::new(rcp_width + rcp_width, T::ZERO, T::ZERO, T::ZERO),
            V4::new(T::ZERO, rcp_height + rcp_height, T::ZERO, T::ZERO),
            V4::new(T::ZERO, T::ZERO, r, T::ZERO),
            V4::new(
                -(left + right) * rcp_width,
                -(top + bottom) * rcp_height,
                r * near,
                T::ONE,
            ),
        )
    }
}
