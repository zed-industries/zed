// Copyright 2013-2014 The CGMath Developers. For a full listing of the authors,
// refer to the Cargo.toml file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate approx;
extern crate cgmath;

use cgmath::*;
use std::f64;
use std::iter;

#[test]
fn test_constructor() {
    assert_eq!(vec1(1f32), Vector1::new(1f32));
    assert_eq!(vec2(1f32, 2f32), Vector2::new(1f32, 2f32));
    assert_eq!(vec3(1f64, 2f64, 3f64), Vector3::new(1f64, 2f64, 3f64));
    assert_eq!(
        vec4(1isize, 2isize, 3isize, 4isize),
        Vector4::new(1isize, 2isize, 3isize, 4isize)
    );
}

#[test]
fn test_from_value() {
    assert_eq!(
        Vector2::from_value(102isize),
        Vector2::new(102isize, 102isize)
    );
    assert_eq!(
        Vector3::from_value(22isize),
        Vector3::new(22isize, 22isize, 22isize)
    );
    assert_eq!(
        Vector4::from_value(76.5f64),
        Vector4::new(76.5f64, 76.5f64, 76.5f64, 76.5f64)
    );
}

macro_rules! impl_test_add {
    ($VectorN:ident { $($field:ident),+ }, $s:expr, $v:expr) => (
        // vector + vector ops
        assert_eq!($v + $v, $VectorN::new($($v.$field + $v.$field),+));
        assert_eq!(&$v + &$v, $v + $v);
        assert_eq!(&$v + $v, $v + $v);
        assert_eq!($v + &$v, $v + $v);
    )
}

macro_rules! impl_test_sub {
    ($VectorN:ident { $($field:ident),+ }, $s:expr, $v:expr) => (
        // vector - vector ops
        assert_eq!($v - $v, $VectorN::new($($v.$field - $v.$field),+));
        assert_eq!(&$v - &$v, $v - $v);
        assert_eq!(&$v - $v, $v - $v);
        assert_eq!($v - &$v, $v - $v);
    )
}

macro_rules! impl_test_mul {
    ($VectorN:ident { $($field:ident),+ }, $s:expr, $v:expr) => (
        // vector * scalar ops
        assert_eq!($v * $s, $VectorN::new($($v.$field * $s),+));
        assert_eq!($s * $v, $VectorN::new($($s * $v.$field),+));
        assert_eq!(&$v * $s, $v * $s);
        assert_eq!($s * &$v, $s * $v);
        // commutativity
        assert_eq!($v * $s, $s * $v);
    )
}

macro_rules! impl_test_div {
    ($VectorN:ident { $($field:ident),+ }, $s:expr, $v:expr) => (
        // vector / scalar ops
        assert_eq!($v / $s, $VectorN::new($($v.$field / $s),+));
        assert_eq!($s / $v, $VectorN::new($($s / $v.$field),+));
        assert_eq!(&$v / $s, $v / $s);
        assert_eq!($s / &$v, $s / $v);
    )
}

macro_rules! impl_test_rem {
    ($VectorN:ident { $($field:ident),+ }, $s:expr, $v:expr) => (
        // vector % scalar ops
        assert_eq!($v % $s, $VectorN::new($($v.$field % $s),+));
        assert_eq!($s % $v, $VectorN::new($($s % $v.$field),+));
        assert_eq!(&$v % $s, $v % $s);
        assert_eq!($s % &$v, $s % $v);
    )
}

macro_rules! impl_test_iter_sum {
    ($VectorN:ident { $($field:ident),+ }, $ty:ty, $s:expr, $v:expr) => (
        assert_eq!($VectorN::new($($v.$field * $s),+),
                   iter::repeat($v).take($s as usize).sum());
    )
}

#[test]
fn test_add() {
    impl_test_add!(Vector4 { x, y, z, w }, 2.0f32, vec4(2.0f32, 4.0, 6.0, 8.0));
    impl_test_add!(Vector3 { x, y, z }, 2.0f32, vec3(2.0f32, 4.0, 6.0));
    impl_test_add!(Vector2 { x, y }, 2.0f32, vec2(2.0f32, 4.0));
}

#[test]
fn test_sub() {
    impl_test_sub!(Vector4 { x, y, z, w }, 2.0f32, vec4(2.0f32, 4.0, 6.0, 8.0));
    impl_test_sub!(Vector3 { x, y, z }, 2.0f32, vec3(2.0f32, 4.0, 6.0));
    impl_test_sub!(Vector2 { x, y }, 2.0f32, vec2(2.0f32, 4.0));
}

#[test]
fn test_mul() {
    impl_test_mul!(Vector4 { x, y, z, w }, 2.0f32, vec4(2.0f32, 4.0, 6.0, 8.0));
    impl_test_mul!(Vector3 { x, y, z }, 2.0f32, vec3(2.0f32, 4.0, 6.0));
    impl_test_mul!(Vector2 { x, y }, 2.0f32, vec2(2.0f32, 4.0));
}

#[test]
fn test_div() {
    impl_test_div!(Vector4 { x, y, z, w }, 2.0f32, vec4(2.0f32, 4.0, 6.0, 8.0));
    impl_test_div!(Vector3 { x, y, z }, 2.0f32, vec3(2.0f32, 4.0, 6.0));
    impl_test_div!(Vector2 { x, y }, 2.0f32, vec2(2.0f32, 4.0));
}

#[test]
fn test_rem() {
    impl_test_rem!(Vector4 { x, y, z, w }, 2.0f32, vec4(2.0f32, 4.0, 6.0, 8.0));
    impl_test_rem!(Vector3 { x, y, z }, 2.0f32, vec3(2.0f32, 4.0, 6.0));
    impl_test_rem!(Vector2 { x, y }, 2.0f32, vec2(2.0f32, 4.0));
}

#[test]
fn test_dot() {
    assert_eq!(Vector2::new(1.0, 2.0).dot(Vector2::new(3.0, 4.0)), 11.0);
    assert_eq!(
        Vector3::new(1.0, 2.0, 3.0).dot(Vector3::new(4.0, 5.0, 6.0)),
        32.0
    );
    assert_eq!(
        Vector4::new(1.0, 2.0, 3.0, 4.0).dot(Vector4::new(5.0, 6.0, 7.0, 8.0)),
        70.0
    );
}

#[test]
fn test_sum() {
    assert_eq!(Vector2::new(1isize, 2isize).sum(), 3isize);
    assert_eq!(Vector3::new(1isize, 2isize, 3isize).sum(), 6isize);
    assert_eq!(Vector4::new(1isize, 2isize, 3isize, 4isize).sum(), 10isize);

    assert_eq!(Vector2::new(3.0f64, 4.0f64).sum(), 7.0f64);
    assert_eq!(Vector3::new(4.0f64, 5.0f64, 6.0f64).sum(), 15.0f64);
    assert_eq!(Vector4::new(5.0f64, 6.0f64, 7.0f64, 8.0f64).sum(), 26.0f64);
}

#[test]
fn test_iter_sum() {
    impl_test_iter_sum!(
        Vector4 { x, y, z, w },
        f32,
        2.0f32,
        vec4(2.0f32, 4.0, 6.0, 8.0)
    );
    impl_test_iter_sum!(Vector3 { x, y, z }, f32, 2.0f32, vec3(2.0f32, 4.0, 6.0));
    impl_test_iter_sum!(Vector2 { x, y }, f32, 2.0f32, vec2(2.0f32, 4.0));

    impl_test_iter_sum!(Vector4 { x, y, z, w }, usize, 2usize, vec4(2usize, 4, 6, 8));
    impl_test_iter_sum!(Vector3 { x, y, z }, usize, 2usize, vec3(2usize, 4, 6));
    impl_test_iter_sum!(Vector2 { x, y }, usize, 2usize, vec2(2usize, 4));
}

#[test]
fn test_product() {
    assert_eq!(Vector2::new(1isize, 2isize).product(), 2isize);
    assert_eq!(Vector3::new(1isize, 2isize, 3isize).product(), 6isize);
    assert_eq!(
        Vector4::new(1isize, 2isize, 3isize, 4isize).product(),
        24isize
    );

    assert_eq!(Vector2::new(3.0f64, 4.0f64).product(), 12.0f64);
    assert_eq!(Vector3::new(4.0f64, 5.0f64, 6.0f64).product(), 120.0f64);
    assert_eq!(
        Vector4::new(5.0f64, 6.0f64, 7.0f64, 8.0f64).product(),
        1680.0f64
    );
}

#[test]
fn test_cross() {
    let a = Vector3::new(1isize, 2isize, 3isize);
    let b = Vector3::new(4isize, 5isize, 6isize);
    let r = Vector3::new(-3isize, 6isize, -3isize);
    assert_eq!(a.cross(b), r);
}

#[test]
fn test_is_perpendicular() {
    assert!(Vector2::new(1.0f64, 0.0f64).is_perpendicular(Vector2::new(0.0f64, 1.0f64)));
    assert!(
        Vector3::new(0.0f64, 1.0f64, 0.0f64).is_perpendicular(Vector3::new(0.0f64, 0.0f64, 1.0f64))
    );
    assert!(Vector4::new(1.0f64, 0.0f64, 0.0f64, 0.0f64)
        .is_perpendicular(Vector4::new(0.0f64, 0.0f64, 0.0f64, 1.0f64)));
}

#[cfg(test)]
mod test_magnitude {
    use cgmath::*;

    #[test]
    fn test_vector2() {
        let (a, a_res) = (Vector2::new(3.0f64, 4.0f64), 5.0f64); // (3, 4, 5) Pythagorean triple
        let (b, b_res) = (Vector2::new(5.0f64, 12.0f64), 13.0f64); // (5, 12, 13) Pythagorean triple

        assert_eq!(a.magnitude2(), a_res * a_res);
        assert_eq!(b.magnitude2(), b_res * b_res);

        assert_eq!(a.magnitude(), a_res);
        assert_eq!(b.magnitude(), b_res);
    }

    #[test]
    fn test_vector3() {
        let (a, a_res) = (Vector3::new(2.0f64, 3.0f64, 6.0f64), 7.0f64); // (2, 3, 6, 7) Pythagorean quadruple
        let (b, b_res) = (Vector3::new(1.0f64, 4.0f64, 8.0f64), 9.0f64); // (1, 4, 8, 9) Pythagorean quadruple

        assert_eq!(a.magnitude2(), a_res * a_res);
        assert_eq!(b.magnitude2(), b_res * b_res);

        assert_eq!(a.magnitude(), a_res);
        assert_eq!(b.magnitude(), b_res);
    }

    #[test]
    fn test_vector4() {
        let (a, a_res) = (Vector4::new(1.0f64, 2.0f64, 4.0f64, 10.0f64), 11.0f64); // (1, 2, 4, 10, 11) Pythagorean quintuple
        let (b, b_res) = (Vector4::new(1.0f64, 2.0f64, 8.0f64, 10.0f64), 13.0f64); // (1, 2, 8, 10, 13) Pythagorean quintuple

        assert_eq!(a.magnitude2(), a_res * a_res);
        assert_eq!(b.magnitude2(), b_res * b_res);

        assert_eq!(a.magnitude(), a_res);
        assert_eq!(b.magnitude(), b_res);
    }
}

#[test]
fn test_angle() {
    assert_ulps_eq!(
        Vector2::new(1.0f64, 0.0f64).angle(Vector2::new(0.0f64, 1.0f64)),
        &Rad(f64::consts::FRAC_PI_2)
    );
    assert_ulps_eq!(
        Vector2::new(10.0f64, 0.0f64).angle(Vector2::new(0.0f64, 5.0f64)),
        &Rad(f64::consts::FRAC_PI_2)
    );
    assert_ulps_eq!(
        Vector2::new(-1.0f64, 0.0f64).angle(Vector2::new(0.0f64, 1.0f64)),
        &-Rad(f64::consts::FRAC_PI_2)
    );

    assert_ulps_eq!(
        Vector3::new(1.0f64, 0.0f64, 1.0f64).angle(Vector3::new(1.0f64, 1.0f64, 0.0f64)),
        &Rad(f64::consts::FRAC_PI_3)
    );
    assert_ulps_eq!(
        Vector3::new(10.0f64, 0.0f64, 10.0f64).angle(Vector3::new(5.0f64, 5.0f64, 0.0f64)),
        &Rad(f64::consts::FRAC_PI_3)
    );
    assert_ulps_eq!(
        Vector3::new(-1.0f64, 0.0f64, -1.0f64).angle(Vector3::new(1.0f64, -1.0f64, 0.0f64)),
        &Rad(2.0f64 * f64::consts::FRAC_PI_3)
    );

    assert_ulps_eq!(
        Vector4::new(1.0f64, 0.0f64, 1.0f64, 0.0f64)
            .angle(Vector4::new(0.0f64, 1.0f64, 0.0f64, 1.0f64)),
        &Rad(f64::consts::FRAC_PI_2)
    );
    assert_ulps_eq!(
        Vector4::new(10.0f64, 0.0f64, 10.0f64, 0.0f64)
            .angle(Vector4::new(0.0f64, 5.0f64, 0.0f64, 5.0f64)),
        &Rad(f64::consts::FRAC_PI_2)
    );
    assert_ulps_eq!(
        Vector4::new(-1.0f64, 0.0f64, -1.0f64, 0.0f64)
            .angle(Vector4::new(0.0f64, 1.0f64, 0.0f64, 1.0f64)),
        &Rad(f64::consts::FRAC_PI_2)
    );
}

#[test]
fn test_normalize() {
    // TODO: test normalize_to, normalize_sel.0, and normalize_self_to
    assert_ulps_eq!(
        Vector2::new(3.0f64, 4.0f64).normalize(),
        &Vector2::new(3.0 / 5.0, 4.0 / 5.0)
    );
    assert_ulps_eq!(
        Vector3::new(2.0f64, 3.0f64, 6.0f64).normalize(),
        &Vector3::new(2.0 / 7.0, 3.0 / 7.0, 6.0 / 7.0)
    );
    assert_ulps_eq!(
        Vector4::new(1.0f64, 2.0f64, 4.0f64, 10.0f64).normalize(),
        &Vector4::new(1.0 / 11.0, 2.0 / 11.0, 4.0 / 11.0, 10.0 / 11.0)
    );
}

#[test]
fn test_project_on() {
    assert_ulps_eq!(
        Vector2::new(-1.0f64, 5.0).project_on(Vector2::new(2.0, 4.0)),
        &Vector2::new(9.0 / 5.0, 18.0 / 5.0)
    );
    assert_ulps_eq!(
        Vector3::new(5.0f64, 6.0, 7.0).project_on(Vector3::new(1.0, 1.0, 1.0)),
        &Vector3::new(6.0, 6.0, 6.0)
    );
    assert_ulps_eq!(
        Vector4::new(0.0f64, -5.0, 5.0, 5.0).project_on(Vector4::new(0.0, 1.0, 0.0, 0.5)),
        &Vector4::new(0.0, -2.0, 0.0, -1.0)
    );
}

#[test]
fn test_cast() {
    assert_ulps_eq!(
        Vector2::new(0.9f64, 1.5).cast().unwrap(),
        Vector2::new(0.9f32, 1.5)
    );
    assert_ulps_eq!(
        Vector3::new(1.0f64, 2.4, -3.13).cast().unwrap(),
        Vector3::new(1.0f32, 2.4, -3.13)
    );
    assert_ulps_eq!(
        Vector4::new(13.5f64, -4.6, -8.3, 2.41).cast().unwrap(),
        Vector4::new(13.5f32, -4.6, -8.3, 2.41)
    );
}
