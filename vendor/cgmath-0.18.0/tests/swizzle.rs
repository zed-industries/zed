// Copyright 2013-2017 The CGMath Developers. For a full listing of the authors,
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

#![cfg(feature = "swizzle")]

extern crate cgmath;

use cgmath::{Point1, Point2, Point3, Vector1, Vector2, Vector3, Vector4};

// Sanity checks
#[test]
fn test_point_swizzle() {
    let p1 = Point1::new(1.0);
    let p2 = Point2::new(1.0, 2.0);
    let p3 = Point3::new(1.0, 2.0, 3.0);
    assert_eq!(p1.x(), p1);
    assert_eq!(p2.x(), p1);
    assert_eq!(p2.y(), Point1::new(2.0));
    assert_eq!(p2.xx(), Point2::new(1.0, 1.0));
    assert_eq!(p2.xy(), p2);
    assert_eq!(p2.yx(), Point2::new(2.0, 1.0));
    assert_eq!(p2.yy(), Point2::new(2.0, 2.0));
    assert_eq!(p3.x(), p1);
    assert_eq!(p3.y(), Point1::new(2.0));
    assert_eq!(p3.xy(), p2);
    assert_eq!(p3.zy(), Point2::new(3.0, 2.0));
    assert_eq!(p3.yyx(), Point3::new(2.0, 2.0, 1.0));
}

#[test]
fn test_vector_swizzle() {
    let p1 = Vector1::new(1.0);
    let p2 = Vector2::new(1.0, 2.0);
    let p3 = Vector3::new(1.0, 2.0, 3.0);
    let p4 = Vector4::new(1.0, 2.0, 3.0, 4.0);
    assert_eq!(p1.x(), p1);
    assert_eq!(p2.x(), p1);
    assert_eq!(p2.y(), Vector1::new(2.0));
    assert_eq!(p2.xx(), Vector2::new(1.0, 1.0));
    assert_eq!(p2.xy(), p2);
    assert_eq!(p2.yx(), Vector2::new(2.0, 1.0));
    assert_eq!(p2.yy(), Vector2::new(2.0, 2.0));
    assert_eq!(p3.x(), p1);
    assert_eq!(p3.y(), Vector1::new(2.0));
    assert_eq!(p3.xy(), p2);
    assert_eq!(p3.zy(), Vector2::new(3.0, 2.0));
    assert_eq!(p3.yyx(), Vector3::new(2.0, 2.0, 1.0));
    assert_eq!(p4.xyxy(), Vector4::new(1.0, 2.0, 1.0, 2.0));
}
