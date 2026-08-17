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
use test::Bencher;

use cgmath::*;

#[path = "common/macros.rs"]
#[macro_use]
mod macros;

fn bench_from_axis_angle<T: Rotation3<Scalar = f32>>(bh: &mut Bencher) {
    const LEN: usize = 1 << 13;

    let mut rng = SmallRng::from_entropy();

    let axis: Vec<_> = (0..LEN).map(|_| rng.gen::<Vector3<f32>>()).collect();
    let angle: Vec<_> = (0..LEN).map(|_| rng.gen::<Rad<f32>>()).collect();
    let mut i = 0;

    bh.iter(|| {
        i = (i + 1) & (LEN - 1);

        unsafe {
            let res: T =
                Rotation3::from_axis_angle(*axis.get_unchecked(i), *angle.get_unchecked(i));
            test::black_box(res)
        }
    })
}

#[bench]
fn _bench_quat_from_axisangle(bh: &mut Bencher) {
    bench_from_axis_angle::<Quaternion<f32>>(bh)
}

#[bench]
fn _bench_rot3_from_axisangle(bh: &mut Bencher) {
    bench_from_axis_angle::<Basis3<f32>>(bh)
}

bench_construction!(
    _bench_rot2_from_axisangle,
    Basis2<f32>,
    Basis2::from_angle[angle: Rad<f32>]
);

bench_construction!(
    _bench_quat_from_euler_angles,
    Quaternion<f32>,
    Quaternion::from[src: Euler<Rad<f32>>]
);
bench_construction!(
    _bench_rot3_from_euler_angles,
    Basis3<f32>,
    Basis3::from[src: Euler<Rad<f32>>]
);
