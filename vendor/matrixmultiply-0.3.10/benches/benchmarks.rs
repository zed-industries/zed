extern crate matrixmultiply;
pub use matrixmultiply::dgemm;
pub use matrixmultiply::sgemm;

#[macro_use]
extern crate bencher;

// Compute GFlop/s
// by flop / s = 2 M N K / time

benchmark_main!(mat_mul_f32, mat_mul_f64, layout_f32_032, layout_f64_032);

macro_rules! mat_mul {
    ($modname:ident, $gemm:ident, $(($name:ident, $m:expr, $n:expr, $k:expr))+) => {
        mod $modname {
            use bencher::{Bencher};
            use crate::$gemm;
            $(
            pub fn $name(bench: &mut Bencher)
            {
                let a = vec![0.; $m * $n];
                let b = vec![0.; $n * $k];
                let mut c = vec![0.; $m * $k];
                bench.iter(|| {
                    unsafe {
                        $gemm(
                            $m, $n, $k,
                            1.,
                            a.as_ptr(), $n, 1,
                            b.as_ptr(), $k, 1,
                            0.,
                            c.as_mut_ptr(), $k, 1,
                            )
                    }
                });
            }
            )+
        }
        benchmark_group!{ $modname, $($modname::$name),+ }
    };
}

mat_mul! {mat_mul_f32, sgemm,
    (m004, 4, 4, 4)
    (m006, 6, 6, 6)
    (m008, 8, 8, 8)
    (m012, 12, 12, 12)
    (m016, 16, 16, 16)
    (m032, 32, 32, 32)
    (m064, 64, 64, 64)
    (m127, 127, 127, 127)
    /*
    (m256, 256, 256, 256)
    (m512, 512, 512, 512)
    (mix16x4, 32, 4, 32)
    (mix32x2, 32, 2, 32)
    (mix97, 97, 97, 125)
    (mix128x10000x128, 128, 10000, 128)
    */
}

mat_mul! {mat_mul_f64, dgemm,
    (m004, 4, 4, 4)
    (m006, 6, 6, 6)
    (m008, 8, 8, 8)
    (m012, 12, 12, 12)
    (m016, 16, 16, 16)
    (m032, 32, 32, 32)
    (m064, 64, 64, 64)
    (m127, 127, 127, 127)
    /*
    (m256, 256, 256, 256)
    (m512, 512, 512, 512)
    (mix16x4, 32, 4, 32)
    (mix32x2, 32, 2, 32)
    (mix97, 97, 97, 125)
    (mix128x10000x128, 128, 10000, 128)
    */
}

/// benchmarks combinations of inputs using various layouts
/// row-major ("c") vs column-major ("f") experiments
///
/// These benchmarks give information about
///
/// 1. Matrix packing sensitivity to input layouts (A, B layouts)
/// 2. Microkernel sensitivity to output layouts (C layout)
///    and performance for beta != 0. vs == 0.
///
/// Clike: all elements spaced at least 2 apart
/// Flike: all elements spaced at least 2 apart

enum Layout {
    C,
    F,
    Clike,
    Flike,
}
use self::Layout::*;

impl Layout {
    fn spread(&self) -> usize {
        match *self {
            C | F => 1,
            Clike | Flike => 2,
        }
    }

    fn strides(&self, rs: isize, cs: isize) -> (isize, isize) {
        let spread = self.spread() as isize;
        match *self {
            C | Clike => (rs * spread, cs * spread),
            F | Flike => (cs * spread, rs * spread),
        }
    }
}

macro_rules! gemm_layout {
    ($modname:ident, $gemm:ident, $(($name:ident, $m:expr))+) => {
        mod $modname {
            use bencher::{Bencher};
            use super::Layout::{self, *};
            use crate::$gemm;
            $(

            fn base(bench: &mut Bencher, al: Layout, bl: Layout, cl: Layout, use_beta: bool)
            {
                let a = vec![0.; $m * $m * al.spread()];
                let b = vec![0.; $m * $m * bl.spread()];
                let mut c = vec![0.; $m * $m * cl.spread()];
                let beta = if use_beta { 0.1 } else { 0. };

                let (rsa, csa) = al.strides($m, 1);
                let (rsb, csb) = bl.strides($m, 1);
                let (rsc, csc) = cl.strides($m, 1);

                let max_stride_a = (rsa as usize) * ($m - 1) + (csa as usize) * ($m - 1);
                let max_stride_b = (rsb as usize) * ($m - 1) + (csb as usize) * ($m - 1);
                let max_stride_c = (rsc as usize) * ($m - 1) + (csc as usize) * ($m - 1);

                debug_assert!(max_stride_a < a.len());
                debug_assert!(max_stride_b < b.len());
                debug_assert!(max_stride_c < c.len());
                bench.iter(|| {
                    unsafe {
                        $gemm(
                            $m, $m, $m,
                            1.,
                            a.as_ptr(), rsa, csa,
                            b.as_ptr(), rsb, csb,
                            beta,
                            c.as_mut_ptr(), rsc, csc,
                            )
                    }
                });
            }

            pub fn nobeta_ccc(bench: &mut Bencher) { base(bench, C, C, C, false); }
            pub fn nobeta_ccf(bench: &mut Bencher) { base(bench, C, C, F, false); }
            pub fn nobeta_fcc(bench: &mut Bencher) { base(bench, F, C, C, false); }
            pub fn nobeta_cfc(bench: &mut Bencher) { base(bench, C, F, C, false); }
            pub fn nobeta_ffc(bench: &mut Bencher) { base(bench, F, F, C, false); }
            pub fn nobeta_cff(bench: &mut Bencher) { base(bench, C, F, F, false); }
            pub fn nobeta_fcf(bench: &mut Bencher) { base(bench, F, C, F, false); }
            pub fn nobeta_fff(bench: &mut Bencher) { base(bench, F, F, F, false); }

            pub fn nobeta_cfc_spread_yyn(bench: &mut Bencher) { base(bench, Clike, Flike, C, false); }
            pub fn nobeta_fcc_spread_yyn(bench: &mut Bencher) { base(bench, Flike, Clike, C, false); }
            pub fn nobeta_fcc_spread_nny(bench: &mut Bencher) { base(bench, C, F, Clike, false); }
            pub fn nobeta_fcf_spread_nny(bench: &mut Bencher) { base(bench, C, F, Flike, false); }

            pub fn beta_ccc(bench: &mut Bencher) { base(bench, C, C, C, true); }
            pub fn beta_ccf(bench: &mut Bencher) { base(bench, C, C, F, true); }
            pub fn beta_fcc(bench: &mut Bencher) { base(bench, F, C, C, true); }
            pub fn beta_cfc(bench: &mut Bencher) { base(bench, C, F, C, true); }
            pub fn beta_ffc(bench: &mut Bencher) { base(bench, F, F, C, true); }
            pub fn beta_cff(bench: &mut Bencher) { base(bench, C, F, F, true); }
            pub fn beta_fcf(bench: &mut Bencher) { base(bench, F, C, F, true); }
            pub fn beta_fff(bench: &mut Bencher) { base(bench, F, F, F, true); }

            pub fn beta_fcc_spread_nny(bench: &mut Bencher) { base(bench, C, F, Clike, true); }
            pub fn beta_fcf_spread_nny(bench: &mut Bencher) { base(bench, C, F, Flike, true); }
            )+
        }
        benchmark_group!{ $modname,
            $modname::nobeta_ccc,
            $modname::nobeta_ccf,
            $modname::nobeta_fcc,
            $modname::nobeta_cfc,
            $modname::nobeta_ffc,
            $modname::nobeta_cff,
            $modname::nobeta_fcf,
            $modname::nobeta_fff,

            $modname::nobeta_cfc_spread_yyn,
            $modname::nobeta_fcc_spread_yyn,
            $modname::nobeta_fcc_spread_nny,
            $modname::nobeta_fcf_spread_nny,

            $modname::beta_ccc,
            $modname::beta_ccf,
            $modname::beta_fcc,
            $modname::beta_cfc,
            $modname::beta_ffc,
            $modname::beta_cff,
            $modname::beta_fcf,
            $modname::beta_fff,

            $modname::beta_fcc_spread_nny,
            $modname::beta_fcf_spread_nny
        }
    };
}

gemm_layout! {layout_f32_032, sgemm,
    (m032, 32)
}

gemm_layout! {layout_f64_032, dgemm,
    (m032, 32)
}

use std::ops::{Add, Mul};

trait Z {
    fn zero() -> Self;
}
impl Z for f32 {
    fn zero() -> Self {
        0.
    }
}
impl Z for f64 {
    fn zero() -> Self {
        0.
    }
}

// simple, slow, correct (hopefully) mat mul (Row Major)
#[inline(never)]
fn reference_mat_mul<A>(m: usize, k: usize, n: usize, a: &[A], b: &[A], c: &mut [A])
where
    A: Z + Add<Output = A> + Mul<Output = A> + Copy,
{
    assert!(a.len() >= m * k);
    assert!(b.len() >= k * n);
    assert!(c.len() >= m * n);

    for i in 0..m {
        for j in 0..n {
            unsafe {
                let celt = c.get_unchecked_mut(i * m + j);
                *celt = (0..k).fold(A::zero(), move |s, x| {
                    s + *a.get_unchecked(i * k + x) * *b.get_unchecked(x * n + j)
                });
            }
        }
    }
}

macro_rules! ref_mat_mul {
    ($modname:ident, $ty:ty, $(($name:ident, $m:expr, $n:expr, $k:expr))+) => {
        mod $modname {
            use bencher::{Bencher};
            use super::reference_mat_mul;
            $(
            pub fn $name(bench: &mut Bencher)
            {
                let a = vec![0. as $ty; $m * $n];
                let b = vec![0.; $n * $k];
                let mut c = vec![0.; $m * $k];
                bench.iter(|| {
                    reference_mat_mul($m, $n, $k, &a, &b, &mut c);
                    c[0]
                });
            }
            )+
        }
        benchmark_group!{ $modname, $($modname::$name),+ }
    };
}
ref_mat_mul! {ref_mat_mul_f32, f32,
    (m004, 4, 4, 4)
    (m005, 5, 5, 5)
    (m006, 6, 6, 6)
    (m007, 7, 7, 7)
    (m008, 8, 8, 8)
    (m009, 9, 9, 9)
    (m012, 12, 12, 12)
    (m016, 16, 16, 16)
    (m032, 32, 32, 32)
    (m064, 64, 64, 64)
}
