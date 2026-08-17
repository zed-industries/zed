#![allow(clippy::all)]

use core::hint::black_box;
use iai_callgrind::{library_benchmark, library_benchmark_group, main};

use glam::{BVec3A, Mat2, Mat3A, Mat4, Quat, Vec2, Vec3A, Vec4};

#[cfg(feature = "scalar-math")]
use glam::BVec4 as BVec4A;

#[cfg(not(feature = "scalar-math"))]
use glam::BVec4A;

#[inline]
fn mat2() -> Mat2 {
    black_box(Mat2::from_cols_array(&[1.0, 2.0, 3.0, 4.0]))
}

#[inline]
fn mat3a() -> Mat3A {
    black_box(Mat3A::from_cols_array(&[
        1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0,
    ]))
}

#[inline]
fn mat4() -> Mat4 {
    black_box(Mat4::from_cols_array(&[
        1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
    ]))
}

#[inline]
fn quat() -> Quat {
    black_box(Quat::from_xyzw(0.0, 0.0, 0.0, 1.0))
}

#[inline]
fn vec2() -> Vec2 {
    black_box(Vec2::new(1.0, 2.0))
}

#[inline]
fn vec3a() -> Vec3A {
    black_box(Vec3A::new(1.0, 2.0, 3.0))
}

#[inline]
fn bvec3a() -> BVec3A {
    black_box(BVec3A::new(true, false, true))
}

#[inline]
fn vec4() -> Vec4 {
    black_box(Vec4::new(1.0, 2.0, 3.0, 4.0))
}

#[inline]
fn bvec4a() -> BVec4A {
    black_box(BVec4A::new(true, false, true, false))
}

#[library_benchmark]
#[bench::args(mat2())]
fn mat2_determinant(m: Mat2) -> f32 {
    black_box(m.determinant())
}

#[library_benchmark]
#[bench::args(mat2())]
fn mat2_inverse(m: Mat2) -> Mat2 {
    black_box(m.inverse())
}

#[library_benchmark]
#[bench::args(mat2())]
fn mat2_transpose(m: Mat2) -> Mat2 {
    black_box(m.transpose())
}

#[library_benchmark]
#[bench::args(mat2(), mat2())]
fn mat2_mul_mat2(m1: Mat2, m2: Mat2) -> Mat2 {
    black_box(m1 * m2)
}

#[library_benchmark]
#[bench::args(mat2(), vec2())]
fn mat2_mul_vec2(m: Mat2, v: Vec2) -> Vec2 {
    black_box(m * v)
}

#[library_benchmark]
#[bench::args(mat3a())]
fn mat3a_determinant(m: Mat3A) -> f32 {
    black_box(m.determinant())
}

#[library_benchmark]
#[bench::args(mat3a())]
fn mat3a_inverse(m: Mat3A) -> Mat3A {
    black_box(m.inverse())
}

#[library_benchmark]
#[bench::args(mat3a())]
fn mat3a_transpose(m: Mat3A) -> Mat3A {
    black_box(m.transpose())
}

#[library_benchmark]
#[bench::args(mat3a(), mat3a())]
fn mat3a_mul_mat3a(m1: Mat3A, m2: Mat3A) -> Mat3A {
    black_box(m1 * m2)
}

#[library_benchmark]
#[bench::args(mat3a(), vec3a())]
fn mat3a_mul_vec3a(m: Mat3A, v: Vec3A) -> Vec3A {
    black_box(m * v)
}

#[library_benchmark]
#[bench::args(mat4())]
fn mat4_determinant(m: Mat4) -> f32 {
    black_box(m.determinant())
}

#[library_benchmark]
#[bench::args(mat4())]
fn mat4_inverse(m: Mat4) -> Mat4 {
    black_box(m.inverse())
}

#[library_benchmark]
#[bench::args(mat4())]
fn mat4_transpose(m: Mat4) -> Mat4 {
    black_box(m.transpose())
}

#[library_benchmark]
#[bench::args(mat4(), mat4())]
fn mat4_mul_mat4(m1: Mat4, m2: Mat4) -> Mat4 {
    black_box(m1 * m2)
}

#[library_benchmark]
#[bench::args(mat4(), vec4())]
fn mat4_mul_vec4(m: Mat4, v: Vec4) -> Vec4 {
    black_box(m * v)
}

#[library_benchmark]
#[bench::args(quat(), quat())]
fn quat_mul_quat(q1: Quat, q2: Quat) -> Quat {
    black_box(q1 * q2)
}

#[library_benchmark]
#[bench::args(quat(), vec3a())]
fn quat_mul_vec3a(q: Quat, v: Vec3A) -> Vec3A {
    black_box(q * v)
}

#[library_benchmark]
#[bench::args(vec3a(), vec3a())]
fn vec3a_dot(v1: Vec3A, v2: Vec3A) -> f32 {
    black_box(v1.dot(v2))
}

#[library_benchmark]
#[bench::args(vec3a(), vec3a())]
fn vec3a_cross(v1: Vec3A, v2: Vec3A) -> Vec3A {
    black_box(v1.cross(v2))
}

#[library_benchmark]
#[bench::args(vec3a())]
fn vec3a_length(v: Vec3A) -> f32 {
    black_box(v.length())
}

#[library_benchmark]
#[bench::args(vec3a())]
fn vec3a_normalize(v: Vec3A) -> Vec3A {
    black_box(v.normalize())
}

#[library_benchmark]
#[bench::args(bvec3a(), vec3a(), vec3a())]
fn vec3a_select(b: BVec3A, v1: Vec3A, v2: Vec3A) -> Vec3A {
    black_box(Vec3A::select(b, v1, v2))
}

#[library_benchmark]
#[bench::args(vec4(), vec4())]
fn vec4_dot(v1: Vec4, v2: Vec4) -> f32 {
    black_box(v1.dot(v2))
}

#[library_benchmark]
#[bench::args(vec4())]
fn vec4_length(v: Vec4) -> f32 {
    black_box(v.length())
}

#[library_benchmark]
#[bench::args(vec4())]
fn vec4_normalize(v: Vec4) -> Vec4 {
    black_box(v.normalize())
}

#[library_benchmark]
#[bench::args(bvec4a(), vec4(), vec4())]
fn vec4_select(b: BVec4A, v1: Vec4, v2: Vec4) -> Vec4 {
    black_box(Vec4::select(b, v1, v2))
}

library_benchmark_group!(
    name = bench_mat2;
    benchmarks =
        mat2_determinant,
        mat2_inverse,
        mat2_mul_mat2,
        mat2_mul_vec2,
        mat2_transpose,
);

library_benchmark_group!(
    name = bench_mat3a;
    benchmarks =
        mat3a_determinant,
        mat3a_inverse,
        mat3a_mul_mat3a,
        mat3a_mul_vec3a,
        mat3a_transpose,
);

library_benchmark_group!(
    name = bench_mat4;
    benchmarks =
        mat4_determinant,
        mat4_inverse,
        mat4_mul_mat4,
        mat4_mul_vec4,
        mat4_transpose,
);

library_benchmark_group!(
    name = bench_quat;
    benchmarks =
        quat_mul_quat,
        quat_mul_vec3a,
);

library_benchmark_group!(
    name = bench_vec3a;
    benchmarks =
        vec3a_dot,
        vec3a_cross,
        vec3a_length,
        vec3a_normalize,
        vec3a_select,
);

library_benchmark_group!(
    name = bench_vec4;
    benchmarks =
        vec4_dot,
        vec4_length,
        vec4_normalize,
        vec4_select,
);

main!(
    library_benchmark_groups = bench_mat2,
    bench_mat3a,
    bench_mat4,
    bench_quat,
    bench_vec3a,
    bench_vec4
);
