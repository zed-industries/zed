

// Example of using sgemm/dgemm from matrixmultiply,
// we show that we can multiply matrices of differing strides.
//
// Jump down to the next place where it says EXAMPLE.

extern crate core;
extern crate itertools;
extern crate matrixmultiply;

use matrixmultiply::{sgemm, dgemm};

use itertools::Itertools;
use itertools::{
    cloned,
    enumerate,
    repeat_n,
};
use core::fmt::{Display, Debug};

trait Float : Copy + Display + Debug + PartialEq {
    fn zero() -> Self;
    fn from(x: i64) -> Self;
    fn nan() -> Self;
    fn is_nan(self) -> bool;
}

impl Float for f32 {
    fn zero() -> Self { 0. }
    fn from(x: i64) -> Self { x as Self }
    fn nan() -> Self { 0./0. }
    fn is_nan(self) -> bool { self.is_nan() }
}

impl Float for f64 {
    fn zero() -> Self { 0. }
    fn from(x: i64) -> Self { x as Self }
    fn nan() -> Self { 0./0. }
    fn is_nan(self) -> bool { self.is_nan() }
}


trait Gemm : Sized {
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

fn main() {
    test_gemm_strides::<f32>();
    test_gemm_strides::<f64>();
}

fn test_gemm_strides<F>() where F: Gemm + Float {
    let test_sizes = [77];
    for &n in &test_sizes {
        test_strides::<F>(n, n, n);
    }
}

//
// Custom stride tests
//

#[derive(Copy, Clone, Debug)]
enum Layout { C, F }
use self::Layout::*;

impl Layout {
    fn strides_scaled(self, m: usize, n: usize, scale: [usize; 2]) -> (isize, isize) {
        match self {
            C => ((n * scale[0] * scale[1]) as isize, scale[1] as isize),
            F => (scale[0] as isize, (m * scale[1] * scale[0]) as isize),
        }
    }
}

impl Default for Layout {
    fn default() -> Self { C }
}


fn test_strides<F>(m: usize, k: usize, n: usize)
    where F: Gemm + Float
{
    let (m, k, n) = (m, k, n);

    let stride_multipliers = vec![[1, 1], [1, 1], [1, 1], [1, 1], [2, 2]];
    let mut multipliers_iter = cloned(&stride_multipliers).cycle();

    let layout_species = [C, F];
    let layouts_iter = repeat_n(cloned(&layout_species), 4).multi_cartesian_product();

    for elt in layouts_iter {
        let layouts = [elt[0], elt[1], elt[2], elt[3]];
        let (m0, m1, m2, m3) = multipliers_iter.next_tuple().unwrap();
        test_strides_inner::<F>(m, k, n, [m0, m1, m2, m3], layouts);
    }
}


fn test_strides_inner<F>(m: usize, k: usize, n: usize,
                         stride_multipliers: [[usize; 2]; 4],
                         layouts: [Layout; 4])
    where F: Gemm + Float
{
    let (m, k, n) = (m, k, n);

    // stride multipliers
    let mstridea = stride_multipliers[0];
    let mstrideb = stride_multipliers[1];
    let mstridec = stride_multipliers[2];
    let mstridec2 = stride_multipliers[3];

    let mut a = vec![F::zero(); m * k * mstridea[0] * mstridea[1]]; 
    let mut b = vec![F::zero(); k * n * mstrideb[0] * mstrideb[1]];
    let mut c1 = vec![F::nan(); m * n * mstridec[0] * mstridec[1]];
    let mut c2 = vec![F::nan(); m * n * mstridec2[0] * mstridec2[1]];

    for (i, elt) in a.iter_mut().enumerate() {
        *elt = F::from(i as i64);
    }
    for (i, elt) in b.iter_mut().enumerate() {
        *elt = F::from(i as i64);
    }

    let la = layouts[0];
    let lb = layouts[1];
    let lc1 = layouts[2];
    let lc2 = layouts[3];
    let (rs_a, cs_a) = la.strides_scaled(m, k, mstridea);
    let (rs_b, cs_b) = lb.strides_scaled(k, n, mstrideb);
    let (rs_c1, cs_c1) = lc1.strides_scaled(m, n, mstridec);
    let (rs_c2, cs_c2) = lc2.strides_scaled(m, n, mstridec2);

    println!("Test matrix a : {} × {} layout: {:?} strides {}, {}", m, k, la, rs_a, cs_a);
    println!("Test matrix b : {} × {} layout: {:?} strides {}, {}", k, n, lb, rs_b, cs_b);
    println!("Test matrix c1: {} × {} layout: {:?} strides {}, {}", m, n, lc1, rs_c1, cs_c1);
    println!("Test matrix c2: {} × {} layout: {:?} strides {}, {}", m, n, lc2, rs_c2, cs_c2);

    macro_rules! c1 {
        ($i:expr, $j:expr) => (c1[(rs_c1 * $i as isize + cs_c1 * $j as isize) as usize]);
    }

    macro_rules! c2 {
        ($i:expr, $j:expr) => (c2[(rs_c2 * $i as isize + cs_c2 * $j as isize) as usize]);
    }

    unsafe {
        // EXAMPLE: Compute the same result in C1 and C2 in two different ways.
        // We only use whole integer values in the low range of floats here,
        // so we have no loss of precision.

        // C1 = A B
        F::gemm(
            m, k, n,
            F::from(1),
            a.as_ptr(), rs_a, cs_a,
            b.as_ptr(), rs_b, cs_b,
            F::zero(),
            c1.as_mut_ptr(), rs_c1, cs_c1,
        );
        
        // C1 += 2 A B
        F::gemm(
            m, k, n,
            F::from(2),
            a.as_ptr(), rs_a, cs_a,
            b.as_ptr(), rs_b, cs_b,
            F::from(1),
            c1.as_mut_ptr(), rs_c1, cs_c1,
        );

        // C2 = 3 A B 
        F::gemm(
            m, k, n,
            F::from(3),
            a.as_ptr(), rs_a, cs_a,
            b.as_ptr(), rs_b, cs_b,
            F::zero(),
            c2.as_mut_ptr(), rs_c2, cs_c2,
        );
    }
    for i in 0..m {
        for j in 0..n {
            let c1_elt = c1![i, j];
            let c2_elt = c2![i, j];
            assert_eq!(c1_elt, c2_elt,
                       "assertion failed for matrices, mismatch at {},{} \n\
                       a:: {:?}\n\
                       b:: {:?}\n\
                       c1: {:?}\n\
                       c2: {:?}\n",
                       i, j,
                       a, b,
                       c1, c2);
        }
    }
    // check we haven't overwritten the NaN values outside the passed output
    for (index, elt) in enumerate(&c1) {
        let i = index / rs_c1 as usize;
        let j = index / cs_c1 as usize;
        let irem = index % rs_c1 as usize;
        let jrem = index % cs_c1 as usize;
        if irem != 0 && jrem != 0 {
            assert!(elt.is_nan(),
                "Element at index={} ({}, {}) should be NaN, but was {}\n\
                c1: {:?}\n",
            index, i, j, elt,
            c1);
        }
    }
    println!("{}×{}×{} {:?} .. passed.", m, k, n, layouts);
}
