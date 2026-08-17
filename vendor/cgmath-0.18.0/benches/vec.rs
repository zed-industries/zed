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

bench_binop!(_bench_vector2_add_v, Vector2<f32>, Vector2<f32>, add);
bench_binop!(_bench_vector3_add_v, Vector3<f32>, Vector3<f32>, add);
bench_binop!(_bench_vector4_add_v, Vector4<f32>, Vector4<f32>, add);

bench_binop!(_bench_vector2_sub_v, Vector2<f32>, Vector2<f32>, sub);
bench_binop!(_bench_vector3_sub_v, Vector3<f32>, Vector3<f32>, sub);
bench_binop!(_bench_vector4_sub_v, Vector4<f32>, Vector4<f32>, sub);

bench_binop!(_bench_vector2_mul_s, Vector2<f32>, f32, mul);
bench_binop!(_bench_vector3_mul_s, Vector3<f32>, f32, mul);
bench_binop!(_bench_vector4_mul_s, Vector4<f32>, f32, mul);

bench_binop!(_bench_vector2_div_s, Vector2<f32>, f32, div);
bench_binop!(_bench_vector3_div_s, Vector3<f32>, f32, div);
bench_binop!(_bench_vector4_div_s, Vector4<f32>, f32, div);

bench_binop!(_bench_vector2_rem_s, Vector2<f32>, f32, rem);
bench_binop!(_bench_vector3_rem_s, Vector3<f32>, f32, rem);
bench_binop!(_bench_vector4_rem_s, Vector4<f32>, f32, rem);

bench_binop!(_bench_vector2_dot, Vector2<f32>, Vector2<f32>, dot);
bench_binop!(_bench_vector3_dot, Vector3<f32>, Vector3<f32>, dot);
bench_binop!(_bench_vector4_dot, Vector4<f32>, Vector4<f32>, dot);

bench_binop!(_bench_vector3_cross, Vector3<f32>, Vector3<f32>, cross);

bench_unop!(_bench_vector2_magnitude, Vector2<f32>, magnitude);
bench_unop!(_bench_vector3_magnitude, Vector3<f32>, magnitude);
bench_unop!(_bench_vector4_magnitude, Vector4<f32>, magnitude);

bench_unop!(_bench_vector2_normalize, Vector2<f32>, normalize);
bench_unop!(_bench_vector3_normalize, Vector3<f32>, normalize);
bench_unop!(_bench_vector4_normalize, Vector4<f32>, normalize);
