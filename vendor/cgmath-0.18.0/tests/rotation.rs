// Copyright 2015 The CGMath Developers. For a full listing of the authors,
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

extern crate cgmath;

use cgmath::*;

mod rotation {
    use super::cgmath::*;

    pub fn a2<R: Rotation2<Scalar = f64>>() -> R {
        Rotation2::from_angle(Deg(30.0))
    }

    pub fn a3<R: Rotation3<Scalar = f64>>() -> R {
        let axis = Vector3::new(1.0, 1.0, 0.0).normalize();
        Rotation3::from_axis_angle(axis, Deg(30.0))
    }
}

#[test]
fn test_invert_basis2() {
    let a: Basis2<_> = rotation::a2();
    let a = a * a.invert();
    let a: &Matrix2<_> = a.as_ref();
    assert!(a.is_identity());
}

#[test]
fn test_invert_basis3() {
    let a: Basis3<_> = rotation::a3();
    let a = a * a.invert();
    let a: &Matrix3<_> = a.as_ref();
    assert!(a.is_identity());
}
