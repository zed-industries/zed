#[cfg(any(
    all(debug_assertions, feature = "debug-glam-assert"),
    feature = "glam-assert"
))]
macro_rules! glam_assert {
    ($($arg:tt)*) => ( assert!($($arg)*); )
}
#[cfg(not(any(
    all(debug_assertions, feature = "debug-glam-assert"),
    feature = "glam-assert"
)))]
macro_rules! glam_assert {
    ($($arg:tt)*) => {};
}

macro_rules! const_assert {
    ($x:expr $(,)?) => {
        #[allow(unknown_lints, clippy::eq_op)]
        const _: () = assert!($x);
    };
}

macro_rules! const_assert_eq {
    ($x:expr, $y:expr $(,)?) => {
        const_assert!($x == $y);
    };
}

#[deprecated(since = "0.21.0", note = "use sse2::m128_from_f32x4() instead.")]
#[macro_export]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
macro_rules! const_m128 {
    ($fx4:expr) => {{
        $crate::sse2::m128_from_f32x4($fx4)
    }};
}

#[deprecated(since = "0.21.0", note = "use Vec2::from_array() instead.")]
#[macro_export]
macro_rules! const_vec2 {
    ($fx2:expr) => {{
        $crate::Vec2::from_array($fx2)
    }};
}

#[deprecated(since = "0.21.0", note = "use Vec3::from_array() instead.")]
#[macro_export]
macro_rules! const_vec3 {
    ($fx3:expr) => {{
        $crate::Vec3::from_array($fx3)
    }};
}

#[deprecated(since = "0.21.0", note = "use Vec3A::from_array() instead.")]
#[macro_export]
macro_rules! const_vec3a {
    ($fx3:expr) => {{
        $crate::Vec3A::from_array($fx3)
    }};
}

#[deprecated(since = "0.21.0", note = "use Vec4::from_array() instead.")]
#[macro_export]
macro_rules! const_vec4 {
    ($fx4:expr) => {{
        $crate::Vec4::from_array($fx4)
    }};
}

#[deprecated(
    since = "0.21.0",
    note = "use Mat2::from_cols(), Mat2::from_cols_array() or Mat2::from_cols_array_2d() instead."
)]
#[macro_export]
macro_rules! const_mat2 {
    ($col0:expr, $col1:expr) => {{
        $crate::Mat2::from_cols_array_2d(&[$col0, $col1])
    }};
    ($fx4:expr) => {{
        $crate::Mat2::from_cols_array(&$fx4)
    }};
}

#[deprecated(
    since = "0.21.0",
    note = "use Mat3::from_cols(), Mat3::from_cols_array() or Mat3::from_cols_array_2d() instead."
)]
#[macro_export]
macro_rules! const_mat3 {
    ($col0:expr, $col1:expr, $col2:expr) => {{
        $crate::Mat3::from_cols_array_2d(&[$col0, $col1, $col2])
    }};
    ($fx9:expr) => {{
        $crate::Mat3::from_cols_array(&$fx9)
    }};
}

#[deprecated(
    since = "0.21.0",
    note = "use Mat3A::from_cols(), Mat3A::from_cols_array() or Mat3A::from_cols_array_2d() instead."
)]
#[macro_export]
macro_rules! const_mat3a {
    ($col0:expr, $col1:expr, $col2:expr) => {{
        $crate::Mat3A::from_cols_array_2d(&[$col0, $col1, $col2])
    }};
    ($fx9:expr) => {{
        $crate::Mat3A::from_cols_array(&$fx9)
    }};
}

#[deprecated(
    since = "0.21.0",
    note = "use Mat4::from_cols(), Mat4::from_cols_array() or Mat4::from_cols_array_2d() instead."
)]
#[macro_export]
macro_rules! const_mat4 {
    ($col0:expr, $col1:expr, $col2:expr, $col3:expr) => {{
        $crate::Mat4::from_cols_array_2d(&[$col0, $col1, $col2, $col3])
    }};
    ($fx16:expr) => {{
        $crate::Mat4::from_cols_array(&$fx16)
    }};
}

#[deprecated(
    since = "0.21.0",
    note = "use Quat::from_xyzw() or Quat::from_array() instead."
)]
#[macro_export]
macro_rules! const_quat {
    ($fx4:expr) => {{
        $crate::Quat::from_array($fx4)
    }};
}

#[deprecated(since = "0.21.0", note = "use DVec2::from_array() instead.")]
#[macro_export]
macro_rules! const_dvec2 {
    ($fx2:expr) => {{
        $crate::DVec2::from_array($fx2)
    }};
}

#[deprecated(since = "0.21.0", note = "use DVec3::from_array() instead.")]
#[macro_export]
macro_rules! const_dvec3 {
    ($fx3:expr) => {{
        $crate::DVec3::from_array($fx3)
    }};
}

#[deprecated(since = "0.21.0", note = "use DVec4::from_array() instead.")]
#[macro_export]
macro_rules! const_dvec4 {
    ($fx4:expr) => {{
        $crate::DVec4::from_array($fx4)
    }};
}

#[deprecated(
    since = "0.21.0",
    note = "use DMat2::from_cols(), DMat2::from_cols_array() or DMat2::from_cols_array_2d() instead."
)]
#[macro_export]
macro_rules! const_dmat2 {
    ($col0:expr, $col1:expr) => {{
        $crate::DMat2::from_cols_array_2d(&[$col0, $col1])
    }};
    ($fx4:expr) => {{
        $crate::DMat2::from_cols_array(&$fx4)
    }};
}

#[deprecated(
    since = "0.21.0",
    note = "use DMat3::from_cols(), DMat3::from_cols_array() or DMat3::from_cols_array_2d() instead."
)]
#[macro_export]
macro_rules! const_dmat3 {
    ($col0:expr, $col1:expr, $col2:expr) => {{
        $crate::DMat3::from_cols_array_2d(&[$col0, $col1, $col2])
    }};
    ($fx9:expr) => {{
        $crate::DMat3::from_cols_array(&$fx9)
    }};
}

#[deprecated(
    since = "0.21.0",
    note = "use DMat4::from_cols(), DMat4::from_cols_array() or DMat4::from_cols_array_2d() instead."
)]
#[macro_export]
macro_rules! const_dmat4 {
    ($col0:expr, $col1:expr, $col2:expr, $col3:expr) => {{
        $crate::DMat4::from_cols_array_2d(&[$col0, $col1, $col2, $col3])
    }};
    ($fx16:expr) => {{
        $crate::DMat4::from_cols_array(&$fx16)
    }};
}

#[deprecated(
    since = "0.21.0",
    note = "use DQuat::from_xyzw() or DQuat::from_array() instead."
)]
#[macro_export]
macro_rules! const_dquat {
    ($fx4:expr) => {{
        $crate::DQuat::from_array($fx4)
    }};
}

#[deprecated(since = "0.21.0", note = "use IVec2::from_array() instead.")]
#[macro_export]
macro_rules! const_ivec2 {
    ($ix2:expr) => {{
        $crate::IVec2::from_array($ix2)
    }};
}

#[deprecated(since = "0.21.0", note = "use IVec3::from_array() instead.")]
#[macro_export]
macro_rules! const_ivec3 {
    ($ix3:expr) => {{
        $crate::IVec3::from_array($ix3)
    }};
}

#[deprecated(since = "0.21.0", note = "use IVec4::from_array() instead.")]
#[macro_export]
macro_rules! const_ivec4 {
    ($ix4:expr) => {{
        $crate::IVec4::from_array($ix4)
    }};
}

#[deprecated(since = "0.21.0", note = "use UVec2::from_array() instead.")]
#[macro_export]
macro_rules! const_uvec2 {
    ($ux2:expr) => {{
        $crate::UVec2::from_array($ux2)
    }};
}

#[deprecated(since = "0.21.0", note = "use UVec3::from_array() instead.")]
#[macro_export]
macro_rules! const_uvec3 {
    ($ux3:expr) => {{
        $crate::UVec3::from_array($ux3)
    }};
}

#[deprecated(since = "0.21.0", note = "use UVec4::from_array() instead.")]
#[macro_export]
macro_rules! const_uvec4 {
    ($ux4:expr) => {{
        $crate::UVec4::from_array($ux4)
    }};
}
