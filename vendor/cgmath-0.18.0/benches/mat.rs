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

#![feature(test)]
#![allow(unused_macros)]

extern crate cgmath;
extern crate rand;
extern crate test;

use rand::{rngs::SmallRng, Rng, SeedableRng};
use std::ops::*;
use test::Bencher;

use cgmath::*;

#[path = "common/macros.rs"]
#[macro_use]
mod macros;

bench_binop!(_bench_matrix2_mul_m, Matrix2<f32>, Matrix2<f32>, mul);
bench_binop!(_bench_matrix3_mul_m, Matrix3<f32>, Matrix3<f32>, mul);
bench_binop!(_bench_matrix4_mul_m, Matrix4<f32>, Matrix4<f32>, mul);

bench_binop!(_bench_matrix2_add_m, Matrix2<f32>, Matrix2<f32>, add);
bench_binop!(_bench_matrix3_add_m, Matrix3<f32>, Matrix3<f32>, add);
bench_binop!(_bench_matrix4_add_m, Matrix4<f32>, Matrix4<f32>, add);

bench_binop!(_bench_matrix2_sub_m, Matrix2<f32>, Matrix2<f32>, sub);
bench_binop!(_bench_matrix3_sub_m, Matrix3<f32>, Matrix3<f32>, sub);
bench_binop!(_bench_matrix4_sub_m, Matrix4<f32>, Matrix4<f32>, sub);

bench_binop!(_bench_matrix2_mul_v, Matrix2<f32>, Vector2<f32>, mul);
bench_binop!(_bench_matrix3_mul_v, Matrix3<f32>, Vector3<f32>, mul);
bench_binop!(_bench_matrix4_mul_v, Matrix4<f32>, Vector4<f32>, mul);

bench_binop!(_bench_matrix2_mul_s, Matrix2<f32>, f32, mul);
bench_binop!(_bench_matrix3_mul_s, Matrix3<f32>, f32, mul);
bench_binop!(_bench_matrix4_mul_s, Matrix4<f32>, f32, mul);

bench_binop!(_bench_matrix2_div_s, Matrix2<f32>, f32, div);
bench_binop!(_bench_matrix3_div_s, Matrix3<f32>, f32, div);
bench_binop!(_bench_matrix4_div_s, Matrix4<f32>, f32, div);

bench_unop!(_bench_matrix2_invert, Matrix2<f32>, invert);
bench_unop!(_bench_matrix3_invert, Matrix3<f32>, invert);
bench_unop!(_bench_matrix4_invert, Matrix4<f32>, invert);

bench_unop!(_bench_matrix2_transpose, Matrix2<f32>, transpose);
bench_unop!(_bench_matrix3_transpose, Matrix3<f32>, transpose);
bench_unop!(_bench_matrix4_transpose, Matrix4<f32>, transpose);

bench_unop!(_bench_matrix4_determinant, Matrix4<f32>, determinant);
