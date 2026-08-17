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

extern crate cgmath;

use cgmath::{ortho, Matrix4, Vector4};

#[test]
fn test_ortho_scale() {
    // An orthographic projection can be used to scale points
    // but this will result in the clip space (Z) being swapped (-1 -> 1)
    // this test asserts that this property is true of our ortho projection

    let vec_near: Vector4<f32> = Vector4::new(-1., -1., -1., 1.);
    let vec_orig: Vector4<f32> = Vector4::new(0., 0., 0., 1.);
    let vec_far: Vector4<f32> = Vector4::new(1., 1., 1., 1.);

    let o: Matrix4<f32> = ortho(-1., 1., -1., 1., -1., 1.);
    let near = o * vec_near;
    let orig = o * vec_orig;
    let far = o * vec_far;

    assert_eq!(near, Vector4::new(-1f32, -1., 1., 1.));
    assert_eq!(orig, Vector4::new(0f32, 0., 0., 1.));
    assert_eq!(far, Vector4::new(1f32, 1., -1., 1.));

    let o: Matrix4<f32> = ortho(-2., 2., -2., 2., -2., 2.);
    let near = o * vec_near;
    let orig = o * vec_orig;
    let far = o * vec_far;

    assert_eq!(near, Vector4::new(-0.5f32, -0.5, 0.5, 1.));
    assert_eq!(orig, Vector4::new(0f32, 0., 0., 1.));
    assert_eq!(far, Vector4::new(0.5f32, 0.5, -0.5, 1.));
}

#[test]
fn test_ortho_translate() {
    // An orthographic projection can be used to translate a point
    // but this will result in the clip space (Z) being swapped (-1 -> 1)
    // this test asserts that this property is true of our ortho projection

    let vec_orig: Vector4<f32> = Vector4::new(0., 0., 0., 1.);

    let o: Matrix4<f32> = ortho(-1., 1., -1., 1., -1., 1.);
    let orig = o * vec_orig;
    assert_eq!(orig, Vector4::new(0., 0., 0., 1.));

    let o: Matrix4<f32> = ortho(0., 2., 0., 2., 0., 2.);
    let orig = o * vec_orig;
    assert_eq!(orig, Vector4::new(-1., -1., -1., 1.));

    let o: Matrix4<f32> = ortho(-2., 0., -2., 0., -2., 0.);
    let orig = o * vec_orig;
    assert_eq!(orig, Vector4::new(1., 1., 1., 1.));
}
