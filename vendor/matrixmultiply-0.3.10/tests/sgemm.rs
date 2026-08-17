extern crate core;
extern crate itertools;
extern crate matrixmultiply;

include!("../testdefs/testdefs.rs");

use itertools::Itertools;
use itertools::{
    cloned,
    enumerate,
    repeat_n,
};
use core::fmt::Debug;

const FAST_TEST: Option<&'static str> = option_env!("MMTEST_FAST_TEST");

#[test]
fn test_sgemm() {
    test_gemm::<f32>();
}

#[test]
fn test_dgemm() {
    test_gemm::<f64>();
}

#[cfg(feature="cgemm")]
#[test]
fn test_cgemm() {
    test_gemm::<c32>();
}

#[cfg(feature="cgemm")]
#[test]
fn test_cgemm_complex() {
    test_complex::<c32>(4, 4, 4, true);
    test_complex::<c32>(16, 32, 8, false);
    test_complex::<c32>(63, 65, 67, false);
}

#[cfg(feature="cgemm")]
#[test]
fn test_zgemm() {
    test_gemm::<c64>();
}

#[cfg(feature="cgemm")]
#[test]
fn test_zgemm_complex() {
    test_complex::<c64>(4, 4, 4, true);
    test_complex::<c64>(16, 32, 8, false);
    test_complex::<c64>(63, 65, 67, false);
}


#[test]
fn test_sgemm_strides() {
    test_gemm_strides::<f32>();
}

#[test]
fn test_dgemm_strides() {
    test_gemm_strides::<f64>();
}

#[cfg(feature="cgemm")]
#[test]
fn test_cgemm_strides() {
    test_gemm_strides::<c32>();
}

#[cfg(feature="cgemm")]
#[test]
fn test_zgemm_strides() {
    test_gemm_strides::<c64>();
}

fn test_gemm_strides<F>() where F: Gemm + Float {
    if FAST_TEST.is_some() { return; }

    for n in 0..20 {
        test_strides::<F>(n, n, n);
    }

    for n in (3..12).map(|x| x * 7) {
        test_strides::<F>(n, n, n);
    }

    test_strides::<F>(8, 12, 16);
    test_strides::<F>(8, 0, 10);
}

fn test_gemm<F>() where F: Gemm + Float {
    test_mul_with_id::<F>(4, 4, true);
    test_mul_with_id::<F>(8, 8, true);
    test_mul_with_id::<F>(32, 32, true);

    if FAST_TEST.is_some() { return; }

    test_mul_with_id::<F>(128, 128, false);
    test_mul_with_id::<F>(17, 128, false);
    for i in 0..12 {
        for j in 0..12 {
            test_mul_with_id::<F>(i, j, true);
        }
    }

    test_mul_with_id::<F>(17, 257, false);
    test_mul_with_id::<F>(24, 512, false);

    for i in 0..10 {
        for j in 0..10 {
            test_mul_with_id::<F>(i * 4, j * 4, true);
        }
    }
    
    test_mul_with_id::<F>(266, 265, false);
    test_mul_id_with::<F>(4, 4, true);

    for i in 0..12 {
        for j in 0..12 {
            test_mul_id_with::<F>(i, j, true);
        }
    }
    test_mul_id_with::<F>(266, 265, false);
    test_scale::<F>(0, 4, 4, true);
    test_scale::<F>(4, 0, 4, true);
    test_scale::<F>(4, 4, 0, true);
    test_scale::<F>(4, 4, 4, true);
    test_scale::<F>(19, 20, 16, true);
    test_scale::<F>(150, 140, 128, false);

}

/// multiply a M x N matrix with an N x N id matrix
#[cfg(test)]
fn test_mul_with_id<F>(m: usize, n: usize, small: bool)
    where F: Gemm + Float
{
    if !small && FAST_TEST.is_some() {
        return;
    }

    let (m, k, n) = (m, n, n);
    let mut a = vec![F::zero(); m * k]; 
    let mut b = vec![F::zero(); k * n];
    let mut c = vec![F::zero(); m * n];
    println!("test matrix with id input M={}, N={}", m, n);

    for (i, elt) in a.iter_mut().enumerate() {
        *elt = F::from(i as i64);
    }
    for i in 0..k {
        b[i + i * k] = F::one();
    }

    unsafe {
        F::gemm(
            m, k, n,
            F::one(),
            a.as_ptr(), k as isize, 1,
            b.as_ptr(), n as isize, 1,
            F::zero(),
            c.as_mut_ptr(), n as isize, 1,
            )
    }
    assert_matrix_equal(m, n, &a, k as isize, 1, &c, n as isize, 1, small);
    println!("passed matrix with id input M={}, N={}", m, n);
}

/// multiply a K x K id matrix with an K x N matrix
#[cfg(test)]
fn test_mul_id_with<F>(k: usize, n: usize, small: bool) 
    where F: Gemm + Float
{
    if !small && FAST_TEST.is_some() {
        return;
    }

    let (m, k, n) = (k, k, n);
    let mut a = vec![F::zero(); m * k]; 
    let mut b = vec![F::zero(); k * n];
    let mut c = vec![F::zero(); m * n];

    for i in 0..k {
        a[i + i * k] = F::one();
    }
    for (i, elt) in b.iter_mut().enumerate() {
        *elt = F::from(i as i64);
    }

    unsafe {
        F::gemm(
            m, k, n,
            F::one(),
            a.as_ptr(), k as isize, 1,
            b.as_ptr(), n as isize, 1,
            F::zero(),
            c.as_mut_ptr(), n as isize, 1,
            )
    }
    assert_matrix_equal(m, n, &b, n as isize, 1, &c, n as isize, 1, small);
    println!("passed id with matrix input K={}, N={}", k, n);
}

#[cfg(test)]
fn test_scale<F>(m: usize, k: usize, n: usize, small: bool)
    where F: Gemm + Float
{
    if !small && FAST_TEST.is_some() {
        return;
    }

    let (m, k, n) = (m, k, n);
    let mut a = vec![F::zero(); m * k]; 
    let mut b = vec![F::zero(); k * n];
    let mut c1 = vec![F::one(); m * n];
    let mut c2 = vec![F::nan(); m * n];
    // init c2 with NaN to test the overwriting behavior when beta = 0.

    for (i, elt) in a.iter_mut().enumerate() {
        *elt = F::from2(i as i64, i as i64);
    }
    for (i, elt) in b.iter_mut().enumerate() {
        *elt = F::from2(i as i64, i as i64);
    }

    let alpha1;
    let beta1 = F::zero();
    let alpha21;
    let beta21;
    let alpha22;
    let beta22;

    if !F::is_complex() {
        // 3 A B == C in this way:
        // C <- A B
        // C <- A B + 2 C
        alpha1 = F::from(3);

        alpha21 = F::one();
        beta21 = F::zero();
        alpha22 = F::one();
        beta22 = F::from(2);
    } else {
        // Select constants in a way that makes the complex values
        // significant for the complex case. Using i² = -1 to make sure.
        //
        // (2 + 3i) A B == C in this way:
        // C <- (1 + i) A B
        // C <- A B + (2 + i) C  == (3 + 3i - 1) A B
        alpha1 = F::from2(2, 3);

        alpha21 = F::from2(1, 1);
        beta21 = F::zero();
        alpha22 = F::one();
        beta22 = F::from2(2, 1);
    }

    unsafe {
        // C1 = alpha1 A B
        F::gemm(
            m, k, n,
            alpha1,
            a.as_ptr(), k as isize, 1,
            b.as_ptr(), n as isize, 1,
            beta1,
            c1.as_mut_ptr(), n as isize, 1,
        );

        // C2 = alpha21 A B
        F::gemm(
            m, k, n,
            alpha21,
            a.as_ptr(), k as isize, 1,
            b.as_ptr(), n as isize, 1,
            beta21,
            c2.as_mut_ptr(), n as isize, 1,
        );

        // C2 = A B + beta22 C2
        F::gemm(
            m, k, n,
            alpha22,
            a.as_ptr(), k as isize, 1,
            b.as_ptr(), n as isize, 1,
            beta22,
            c2.as_mut_ptr(), n as isize, 1,
        );
    }

    assert_matrix_equal(m, n, &c1, n as isize, 1, &c2, n as isize, 1, small);
    println!("passed matrix with id input M={}, N={}", m, n);
}


#[cfg(feature="cgemm")]
#[cfg(test)]
fn test_complex<F>(m: usize, k: usize, n: usize, small: bool)
    where F: Gemm + Float
{
    if !small && FAST_TEST.is_some() {
        return;
    }

    let (m, k, n) = (m, k, n);
    let mut a = vec![F::zero(); m * k]; 
    let mut b = vec![F::zero(); k * n];
    let mut c1 = vec![F::zero(); m * n];
    let mut c2 = vec![F::zero(); m * n];

    for (i, elt) in a.iter_mut().enumerate() {
        *elt = F::from2(i as i64, -(i as i64));
    }
    for (i, elt) in b.iter_mut().enumerate() {
        *elt = F::from2(-(i as i64), i as i64);
    }

    let alpha1 = F::from2(3, 2);
    let beta1 = F::zero();

    unsafe {
        // C1 = alpha1 A B
        F::gemm(
            m, k, n,
            alpha1,
            a.as_ptr(), k as isize, 1,
            b.as_ptr(), n as isize, 1,
            beta1,
            c1.as_mut_ptr(), n as isize, 1,
        );
    }
    // reference computation
    // matmul
    for i in 0..m {
        for j in 0..n {
            for ki in 0..k {
                c2[i * n + j].mul_add_assign(a[i * k + ki], b[ki * n + j]);
            }
        }
    }

    // multiply by alpha
    for i in 0..m {
        for j in 0..n {
            let elt = &mut c2[i * n + j];
            let mut scaled = F::zero();
            scaled.mul_add_assign(alpha1, *elt);
            *elt = scaled;
        }
    }

    assert_matrix_equal(m, n, &c1, n as isize, 1, &c2, n as isize, 1, small);
    println!("passed matrix with id input M={}, N={}", m, n);
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


#[cfg(test)]
fn test_strides<F>(m: usize, k: usize, n: usize)
    where F: Gemm + Float
{
    let (m, k, n) = (m, k, n);

    let stride_multipliers = vec![[1, 2], [2, 2], [2, 3], [1, 1], [2, 2], [4, 1], [3, 4]];
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

    let small = m < 8 && k < 8 && n < 8;

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

    unsafe {
        // Compute the same result in C1 and C2 in two different ways.
        // We only use whole integer values in the low range of floats here,
        // so we have no loss of precision.
        //
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

    assert_matrix_equal(m, n, &c1, rs_c1, cs_c1, &c2, rs_c2, cs_c2, small);

    // check we haven't overwritten the NaN values outside the passed output
    for (index, elt) in enumerate(&c1) {
        let i = index / rs_c1 as usize;
        let j = index / cs_c1 as usize;
        let irem = index % rs_c1 as usize;
        let jrem = index % cs_c1 as usize;
        if irem != 0 && jrem != 0 {
            assert!(elt.is_nan(),
                "Element at index={} ({}, {}) should be NaN, but was {:?}\n\
                c1: {:?}\n",
            index, i, j, elt,
            c1);
        }
    }
    println!("{}×{}×{} {:?} .. passed.", m, k, n, layouts);
}


/// Assert that matrix C1 == matrix C2
///
/// m, n: size of matrix C1 and C2
///
/// exact: if true, require == equality
///        if false, use relative difference from zero
fn assert_matrix_equal<F>(m: usize, n: usize, c1: &[F], rs_c1: isize, cs_c1: isize, c2: &[F], rs_c2: isize, cs_c2: isize, exact: bool)
where
    F: Gemm + Float
{
    macro_rules! c1 {
        ($i:expr, $j:expr) => (c1[(rs_c1 * $i as isize + cs_c1 * $j as isize) as usize]);
    }

    macro_rules! c2 {
        ($i:expr, $j:expr) => (c2[(rs_c2 * $i as isize + cs_c2 * $j as isize) as usize]);
    }

    let rel_tolerance = F::relative_error_scale();

    let mut maximal = 0.;
    let mut rel_diff_max = 0.;
    let mut n_diffs = 0;
    let mut first_diff_index = None;

    for i in 0..m {
        for j in 0..n {
            let c1_elt = c1![i, j];
            let c2_elt = c2![i, j];

            let c1norm = c1_elt.abs_f64();
            if c1norm > maximal { maximal = c1norm }
            let c2norm = c2_elt.abs_f64();
            if c2norm > maximal { maximal = c1norm }

            let c_diff = c1_elt.diff(c2_elt);
            if c_diff != F::zero() {
                n_diffs += 1;
                if first_diff_index.is_none() {
                    first_diff_index = Some((i, j));
                }
            }

            let largest_elt = f64::max(c1norm, c2norm);
            let point_diff_rel = c_diff.abs_f64() / largest_elt;
            rel_diff_max = f64::max(point_diff_rel, rel_diff_max);
        }
    }

    if n_diffs > 0 {
        println!("Matrix equality stats: maximal elt: {}, largest relative error={:.4e}, ndiffs={}",
                 maximal, rel_diff_max, n_diffs);
    }

    eprintln!("c1: {:?}, {}, {}", c1, rs_c1, cs_c1);
    eprintln!("c2: {:?}, {}, {}", c2, rs_c2, cs_c2);

    if exact {
        assert_eq!(0, n_diffs,
                   "C1 == C2 assertion failed for matrix of size {}x{} with first failing element at index={:?}",
                   m, n, first_diff_index);
    } else {
        assert!(rel_diff_max < rel_tolerance, "Assertion failed: largest relative diff < {:.2e}, was={:e}", rel_tolerance, rel_diff_max);
    }
}
