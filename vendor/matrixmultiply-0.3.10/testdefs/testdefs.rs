
use matrixmultiply::{sgemm, dgemm};
#[cfg(feature="cgemm")]
use matrixmultiply::{cgemm, zgemm, CGemmOption};

// Common code for tests - generic treatment of f32, f64, c32, c64 and Gemm

pub trait Float : Copy + std::fmt::Debug + PartialEq {
    fn zero() -> Self;
    fn one() -> Self;
    // construct as number x
    fn from(x: i64) -> Self;
    // construct as number x + yi, but ignore y if not complex
    fn from2(x: i64, _y: i64) -> Self { Self::from(x) }
    fn nan() -> Self;
    fn real(self) -> Self { self }
    fn imag(self) -> Self;
    fn is_nan(self) -> bool;
    fn is_complex() -> bool { false }
    fn diff(self, rhs: Self) -> Self;
    // absolute value as f64
    fn abs_f64(self) -> f64;
    fn relative_error_scale() -> f64;
    fn mul_add_assign(&mut self, x: Self, y: Self);
}

impl Float for f32 {
    fn zero() -> Self { 0. }
    fn one() -> Self { 1. }
    fn from(x: i64) -> Self { x as Self }
    fn nan() -> Self { 0./0. }
    fn imag(self) -> Self { 0. }
    fn is_nan(self) -> bool { self.is_nan() }
    fn diff(self, rhs: Self) -> Self { self - rhs }
    fn abs_f64(self) -> f64 { self.abs() as f64 }
    fn relative_error_scale() -> f64 { 1e-6 }
    fn mul_add_assign(&mut self, x: Self, y: Self) {
        *self += x * y;
    }
}

impl Float for f64 {
    fn zero() -> Self { 0. }
    fn one() -> Self { 1. }
    fn from(x: i64) -> Self { x as Self }
    fn nan() -> Self { 0./0. }
    fn imag(self) -> Self { 0. }
    fn is_nan(self) -> bool { self.is_nan() }
    fn diff(self, rhs: Self) -> Self { self - rhs }
    fn abs_f64(self) -> f64 { self.abs() }
    fn relative_error_scale() -> f64 { 1e-12 }
    fn mul_add_assign(&mut self, x: Self, y: Self) {
        *self += x * y;
    }
}

#[allow(non_camel_case_types)]
#[cfg(feature="cgemm")]
pub type c32 = [f32; 2];
#[allow(non_camel_case_types)]
#[cfg(feature="cgemm")]
pub type c64 = [f64; 2];

#[cfg(feature="cgemm")]
impl Float for c32 {
    fn zero() -> Self { [0., 0.] }
    fn one() -> Self { [1., 0.] }
    fn from(x: i64) -> Self { [x as _, 0.] }
    fn from2(x: i64, y: i64) -> Self { [x as _, y as _] }
    fn nan() -> Self { [0./0., 0./0.] }
    fn real(self) -> Self { [self[0], 0.] }
    fn imag(self) -> Self { [self[1], 0.] }
    fn is_nan(self) -> bool { self[0].is_nan() || self[1].is_nan() }
    fn is_complex() -> bool { true }
    fn diff(self, rhs: Self) -> Self {
        [self[0] - rhs[0], self[1] - rhs[1]]
    }
    fn abs_f64(self) -> f64 {
        (self[0].powi(2) + self[1].powi(2)).sqrt() as f64
    }
    fn relative_error_scale() -> f64 { 1e-6 }
    fn mul_add_assign(&mut self, x: Self, y: Self) {
        let [re, im] = c32_mul(x, y);
        self[0] += re;
        self[1] += im;
    }
}

#[cfg(feature="cgemm")]
impl Float for c64 {
    fn zero() -> Self { [0., 0.] }
    fn one() -> Self { [1., 0.] }
    fn from(x: i64) -> Self { [x as _, 0.] }
    fn from2(x: i64, y: i64) -> Self { [x as _, y as _] }
    fn nan() -> Self { [0./0., 0./0.] }
    fn real(self) -> Self { [self[0], 0.] }
    fn imag(self) -> Self { [self[1], 0.] }
    fn is_nan(self) -> bool { self[0].is_nan() || self[1].is_nan() }
    fn is_complex() -> bool { true }
    fn diff(self, rhs: Self) -> Self {
        [self[0] - rhs[0], self[1] - rhs[1]]
    }
    fn abs_f64(self) -> f64 {
        (self[0].powi(2) + self[1].powi(2)).sqrt()
    }
    fn relative_error_scale() -> f64 { 1e-12 }
    fn mul_add_assign(&mut self, x: Self, y: Self) {
        let [re, im] = c64_mul(x, y);
        self[0] += re;
        self[1] += im;
    }
}


#[cfg(feature = "cgemm")]
#[inline(always)]
pub(crate) fn c32_mul(x: c32, y: c32) -> c32 {
    let [a, b] = x;
    let [c, d] = y;
    [a * c - b * d, b * c + a * d]
}

#[cfg(feature = "cgemm")]
#[inline(always)]
pub(crate) fn c64_mul(x: c64, y: c64) -> c64 {
    let [a, b] = x;
    let [c, d] = y;
    [a * c - b * d, b * c + a * d]
}


pub trait Gemm : Sized {
    unsafe fn gemm(
        m: usize, k: usize, n: usize,
        alpha: Self,
        a: *const Self, rsa: isize, csa: isize,
        b: *const Self, rsb: isize, csb: isize,
        beta: Self,
        c: *mut Self, rsc: isize, csc: isize);
}

impl Gemm for f32 {
    unsafe fn gemm(
        m: usize, k: usize, n: usize,
        alpha: Self,
        a: *const Self, rsa: isize, csa: isize,
        b: *const Self, rsb: isize, csb: isize,
        beta: Self,
        c: *mut Self, rsc: isize, csc: isize) {
        sgemm(
            m, k, n,
            alpha,
            a, rsa, csa,
            b, rsb, csb,
            beta,
            c, rsc, csc)
    }
}

impl Gemm for f64 {
    unsafe fn gemm(
        m: usize, k: usize, n: usize,
        alpha: Self,
        a: *const Self, rsa: isize, csa: isize,
        b: *const Self, rsb: isize, csb: isize,
        beta: Self,
        c: *mut Self, rsc: isize, csc: isize) {
        dgemm(
            m, k, n,
            alpha,
            a, rsa, csa,
            b, rsb, csb,
            beta,
            c, rsc, csc)
    }
}

#[cfg(feature="cgemm")]
impl Gemm for c32 {
    unsafe fn gemm(
        m: usize, k: usize, n: usize,
        alpha: Self,
        a: *const Self, rsa: isize, csa: isize,
        b: *const Self, rsb: isize, csb: isize,
        beta: Self,
        c: *mut Self, rsc: isize, csc: isize) {
        cgemm(
            CGemmOption::Standard,
            CGemmOption::Standard,
            m, k, n,
            alpha,
            a, rsa, csa,
            b, rsb, csb,
            beta,
            c, rsc, csc)
    }
}

#[cfg(feature="cgemm")]
impl Gemm for c64 {
    unsafe fn gemm(
        m: usize, k: usize, n: usize,
        alpha: Self,
        a: *const Self, rsa: isize, csa: isize,
        b: *const Self, rsb: isize, csb: isize,
        beta: Self,
        c: *mut Self, rsc: isize, csc: isize) {
        zgemm(
            CGemmOption::Standard,
            CGemmOption::Standard,
            m, k, n,
            alpha,
            a, rsa, csa,
            b, rsb, csb,
            beta,
            c, rsc, csc)
    }
}

