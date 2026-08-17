// Copyright 2016 - 2021 Ulrik Sverdrup "bluss"
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::archparam;
use crate::packing::pack;

/// General matrix multiply kernel
pub(crate) trait GemmKernel {
    type Elem: Element;

    /// Kernel rows
    const MR: usize = Self::MRTy::VALUE;
    /// Kernel cols
    const NR: usize = Self::NRTy::VALUE;
    /// Kernel rows as const num type
    type MRTy: ConstNum;
    /// Kernel cols as const num type
    type NRTy: ConstNum;

    /// align inputs to this
    fn align_to() -> usize;

    /// Whether to always use the masked wrapper around the kernel.
    fn always_masked() -> bool;

    // These should ideally be tuned per kernel and per microarch
    #[inline(always)]
    fn nc() -> usize { archparam::S_NC }
    #[inline(always)]
    fn kc() -> usize { archparam::S_KC }
    #[inline(always)]
    fn mc() -> usize { archparam::S_MC }

    /// Pack matrix A into its packing buffer.
    ///
    /// See pack for more documentation.
    ///
    /// Override only if the default packing function does not
    /// use the right layout.
    #[inline]
    unsafe fn pack_mr(kc: usize, mc: usize, pack_buf: &mut [Self::Elem],
                      a: *const Self::Elem, rsa: isize, csa: isize)
    {
        pack::<Self::MRTy, _>(kc, mc, pack_buf, a, rsa, csa)
    }

    /// Pack matrix B into its packing buffer
    ///
    /// See pack for more documentation.
    ///
    /// Override only if the default packing function does not
    /// use the right layout.
    #[inline]
    unsafe fn pack_nr(kc: usize, mc: usize, pack_buf: &mut [Self::Elem],
                      a: *const Self::Elem, rsa: isize, csa: isize)
    {
        pack::<Self::NRTy, _>(kc, mc, pack_buf, a, rsa, csa)
    }


    /// Matrix multiplication kernel
    ///
    /// This does the matrix multiplication:
    ///
    /// C ← α A B + β C
    ///
    /// + `k`: length of data in a, b
    /// + a, b are packed
    /// + c has general strides
    /// + rsc: row stride of c
    /// + csc: col stride of c
    /// + `alpha`: scaling factor for A B product
    /// + `beta`: scaling factor for c.
    ///   Note: if `beta` is `0.`, the kernel should not (and must not)
    ///   read from c, its value is to be treated as if it was zero.
    ///
    /// When masked, the kernel is always called with β=0 but α is passed
    /// as usual. (This is only useful information if you return `true` from
    /// `always_masked`.)
    unsafe fn kernel(
        k: usize,
        alpha: Self::Elem,
        a: *const Self::Elem,
        b: *const Self::Elem,
        beta: Self::Elem,
        c: *mut Self::Elem, rsc: isize, csc: isize);
}

pub(crate) trait Element : Copy + Send + Sync {
    fn zero() -> Self;
    fn one() -> Self;
    #[cfg_attr(not(test), allow(unused))]
    fn test_value() -> Self;
    fn is_zero(&self) -> bool;
    fn add_assign(&mut self, rhs: Self);
    fn mul_assign(&mut self, rhs: Self);
}

impl Element for f32 {
    fn zero() -> Self { 0. }
    fn one() -> Self { 1. }
    fn test_value() -> Self { 1. }
    fn is_zero(&self) -> bool { *self == 0. }
    fn add_assign(&mut self, rhs: Self) { *self += rhs; }
    fn mul_assign(&mut self, rhs: Self) { *self *= rhs; }
}

impl Element for f64 {
    fn zero() -> Self { 0. }
    fn one() -> Self { 1. }
    fn test_value() -> Self { 1. }
    fn is_zero(&self) -> bool { *self == 0. }
    fn add_assign(&mut self, rhs: Self) { *self += rhs; }
    fn mul_assign(&mut self, rhs: Self) { *self *= rhs; }
}

/// Kernel selector
pub(crate) trait GemmSelect<T> {
    /// Call `select` with the selected kernel for this configuration
    fn select<K>(self, kernel: K)
        where K: GemmKernel<Elem=T>,
              T: Element;
}

#[cfg(feature = "cgemm")]
#[allow(non_camel_case_types)]
pub(crate) type c32 = [f32; 2];

#[cfg(feature = "cgemm")]
#[allow(non_camel_case_types)]
pub(crate) type c64 = [f64; 2];

#[cfg(feature = "cgemm")]
impl Element for c32 {
    fn zero() -> Self { [0., 0.] }
    fn one() -> Self { [1., 0.] }
    fn test_value() -> Self { [2., 1.] }
    fn is_zero(&self) -> bool { *self == [0., 0.] }

    #[inline(always)]
    fn add_assign(&mut self, y: Self) {
        self[0] += y[0];
        self[1] += y[1];
    }

    #[inline(always)]
    fn mul_assign(&mut self, rhs: Self) {
        *self = c32_mul(*self, rhs);
    }
}

#[cfg(feature = "cgemm")]
impl Element for c64 {
    fn zero() -> Self { [0., 0.] }
    fn one() -> Self { [1., 0.] }
    fn test_value() -> Self { [2., 1.] }
    fn is_zero(&self) -> bool { *self == [0., 0.] }

    #[inline(always)]
    fn add_assign(&mut self, y: Self) {
        self[0] += y[0];
        self[1] += y[1];
    }

    #[inline(always)]
    fn mul_assign(&mut self, rhs: Self) {
        *self = c64_mul(*self, rhs);
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


pub(crate) trait ConstNum {
    const VALUE: usize;
}

#[cfg(feature = "cgemm")]
pub(crate) struct U2;
pub(crate) struct U4;
pub(crate) struct U8;

#[cfg(feature = "cgemm")]
impl ConstNum for U2 { const VALUE: usize = 2; }
impl ConstNum for U4 { const VALUE: usize = 4; }
impl ConstNum for U8 { const VALUE: usize = 8; }


#[cfg(test)]
pub(crate) mod test {
    use std::fmt;

    use super::GemmKernel;
    use super::Element;
    use crate::aligned_alloc::Alloc;

    pub(crate) fn aligned_alloc<K>(elt: K::Elem, n: usize) -> Alloc<K::Elem>
        where K: GemmKernel,
              K::Elem: Copy,
    {
        unsafe {
            Alloc::new(n, K::align_to()).init_with(elt)
        }
    }

    /// Assert that we can compute A I == A and I B == B for the kernel (truncated, if needed)
    ///
    /// Tests C col major and row major
    /// Tests beta == 0 (and no other option)
    pub(crate) fn test_a_kernel<K, T>(_name: &str)
    where
        K: GemmKernel<Elem = T>,
        T: Element + fmt::Debug + PartialEq,
    {
        const K: usize = 16;
        let mr = K::MR;
        let nr = K::NR;

        // 1. Test A I == A (variables a, b, c)
        // b looks like an identity matrix (truncated, depending on MR/NR)

        let mut a = aligned_alloc::<K>(T::zero(), mr * K);
        let mut b = aligned_alloc::<K>(T::zero(), nr * K);

        let mut count = 1;
        for i in 0..mr {
            for j in 0..K {
                for _ in 0..count {
                    a[i * K + j].add_assign(T::test_value());
                }
                count += 1;
            }
        }

        for i in 0..Ord::min(K, nr) {
            b[i + i * nr] = T::one();
        }

        let mut c = vec![T::zero(); mr * nr];
        unsafe {
            // col major C
            K::kernel(K, T::one(), a.as_ptr(), b.as_ptr(), T::zero(), c.as_mut_ptr(), 1, mr as isize);
        }
        let common_len = Ord::min(a.len(), c.len());
        assert_eq!(&a[..common_len], &c[..common_len]);

        // 2. Test I B == B (variables a, b, c)
        // a looks like an identity matrix (truncated, depending on MR/NR)

        let mut a = aligned_alloc::<K>(T::zero(), mr * K);
        let mut b = aligned_alloc::<K>(T::zero(), nr * K);

        for i in 0..Ord::min(K, mr) {
            a[i + i * mr] = T::one();
        }

        let mut count = 1;
        for i in 0..K {
            for j in 0..nr {
                for _ in 0..count {
                    b[i * nr + j].add_assign(T::test_value());
                }
                count += 1;
            }
        }

        let mut c = vec![T::zero(); mr * nr];
        unsafe {
            // row major C
            K::kernel(K, T::one(), a.as_ptr(), b.as_ptr(), T::zero(), c.as_mut_ptr(), nr as isize, 1);
        }
        let common_len = Ord::min(b.len(), c.len());
        assert_eq!(&b[..common_len], &c[..common_len]);
    }

    #[cfg(feature="cgemm")]
    /// Assert that we can compute A I == A for the kernel (truncated, if needed)
    ///
    /// Tests C col major and row major
    /// Tests beta == 0 (and no other option)
    pub(crate) fn test_complex_packed_kernel<K, T, TReal>(_name: &str)
    where
        K: GemmKernel<Elem = T>,
        T: Element + fmt::Debug + PartialEq,
        TReal: Element + fmt::Debug + PartialEq,
    {
        use crate::cgemm_common::pack_complex;

        const K: usize = 16;
        let mr = K::MR;
        let nr = K::NR;

        // 1. Test A I == A (variables a, b, c)
        // b looks like an identity matrix (truncated, depending on MR/NR)

        let mut a = aligned_alloc::<K>(T::zero(), mr * K);
        let mut apack = aligned_alloc::<K>(T::zero(), mr * K);
        let mut b = aligned_alloc::<K>(T::zero(), nr * K);
        let mut bpack = aligned_alloc::<K>(T::zero(), nr * K);

        let mut count = 1;
        for i in 0..mr {
            for j in 0..K {
                for _ in 0..count {
                    a[i * K + j].add_assign(T::test_value());
                }
                count += 1;
            }
        }

        for i in 0..Ord::min(K, nr) {
            b[i + i * nr] = T::one();
        }

        // unlike test_a_kernel, we need custom packing for these kernels
        unsafe {
            pack_complex::<K::MRTy, T, TReal>(K, mr, &mut apack[..], a.ptr_mut(), 1, mr as isize);
            pack_complex::<K::NRTy, T, TReal>(nr, K, &mut bpack[..], b.ptr_mut(), nr as isize, 1);
        }

        let mut c = vec![T::zero(); mr * nr];
        unsafe {
            // col major C
            K::kernel(K, T::one(), apack.as_ptr(), bpack.as_ptr(), T::zero(), c.as_mut_ptr(), 1, mr as isize);
        }
        let common_len = Ord::min(a.len(), c.len());
        assert_eq!(&a[..common_len], &c[..common_len]);
    }

}
