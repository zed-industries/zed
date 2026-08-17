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

#[test]
fn test_constructor() {
    assert_eq!(point1(1f32), Point1::new(1f32));
    assert_eq!(point2(1f32, 2f32), Point2::new(1f32, 2f32));
    assert_eq!(point3(1f64, 2f64, 3f64), Point3::new(1f64, 2f64, 3f64));
}

macro_rules! impl_test_mul {
    ($PointN:ident { $($field:ident),+ }, $s:expr, $v:expr) => (
        // point * scalar ops
        assert_eq!($v * $s, $PointN::new($($v.$field * $s),+));
        assert_eq!($s * $v, $PointN::new($($s * $v.$field),+));
        assert_eq!(&$v * $s, $v * $s);
        assert_eq!($s * &$v, $s * $v);
        // commutativity
        assert_eq!($v * $s, $s * $v);
    )
}

macro_rules! impl_test_div {
    ($PointN:ident { $($field:ident),+ }, $s:expr, $v:expr) => (
        // point / scalar ops
        assert_eq!($v / $s, $PointN::new($($v.$field / $s),+));
        assert_eq!($s / $v, $PointN::new($($s / $v.$field),+));
        assert_eq!(&$v / $s, $v / $s);
        assert_eq!($s / &$v, $s / $v);
    )
}

macro_rules! impl_test_rem {
    ($PointN:ident { $($field:ident),+ }, $s:expr, $v:expr) => (
        // point % scalar ops
        assert_eq!($v % $s, $PointN::new($($v.$field % $s),+));
        assert_eq!($s % $v, $PointN::new($($s % $v.$field),+));
        assert_eq!(&$v % $s, $v % $s);
        assert_eq!($s % &$v, $s % $v);
    )
}

#[test]
fn test_homogeneous() {
    let p = Point3::new(1.0f64, 2.0f64, 3.0f64);
    assert_ulps_eq!(&p, &Point3::from_homogeneous(p.to_homogeneous()));
}

#[test]
fn test_mul() {
    impl_test_mul!(Point3 { x, y, z }, 2.0f32, Point3::new(2.0f32, 4.0, 6.0));
    impl_test_mul!(Point2 { x, y }, 2.0f32, Point2::new(2.0f32, 4.0));
}

#[test]
fn test_div() {
    impl_test_div!(Point3 { x, y, z }, 2.0f32, Point3::new(2.0f32, 4.0, 6.0));
    impl_test_div!(Point2 { x, y }, 2.0f32, Point2::new(2.0f32, 4.0));
}

#[test]
fn test_rem() {
    impl_test_rem!(Point3 { x, y, z }, 2.0f32, Point3::new(2.0f32, 4.0, 6.0));
    impl_test_rem!(Point2 { x, y }, 2.0f32, Point2::new(2.0f32, 4.0));
}

#[test]
fn test_cast() {
    assert_ulps_eq!(Point1::new(0.9f64).cast().unwrap(), Point1::new(0.9f32));
    assert_ulps_eq!(
        Point2::new(0.9f64, 1.5).cast().unwrap(),
        Point2::new(0.9f32, 1.5)
    );
    assert_ulps_eq!(
        Point3::new(1.0f64, 2.4, -3.13).cast().unwrap(),
        Point3::new(1.0f32, 2.4, -3.13)
    );
}
