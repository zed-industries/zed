// Copyright 2021-2023 Ulrik Sverdrup "bluss"
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use core::mem;
use core::ptr::copy_nonoverlapping;

use rawpointer::PointerExt;

use crate::kernel::Element;
use crate::kernel::ConstNum;

#[cfg(feature = "std")]
macro_rules! fmuladd {
    // conceptually $dst += $a * $b, optionally use fused multiply-add
    (fma_yes, $dst:expr, $a:expr, $b:expr) => {
        {
            $dst = $a.mul_add($b, $dst);
        }
    };
    (fma_no, $dst:expr, $a:expr, $b:expr) => {
        {
            $dst += $a * $b;
        }
    };
}

#[cfg(not(feature = "std"))]
macro_rules! fmuladd {
    ($any:tt, $dst:expr, $a:expr, $b:expr) => {
        {
            $dst += $a * $b;
        }
    };
}


// kernel fallback impl macro
// Depends on a couple of macro and function defitions to be in scope - loop_m/_n, at, etc.
// $fma_opt: fma_yes or fma_no to use f32::mul_add etc or not
macro_rules! kernel_fallback_impl_complex {
    ([$($attr:meta)*] [$fma_opt:tt] $name:ident, $elem_ty:ty, $real_ty:ty, $mr:expr, $nr:expr, $unroll:tt) => {
    $(#[$attr])*
    unsafe fn $name(k: usize, alpha: $elem_ty, a: *const $elem_ty, b: *const $elem_ty,
                    beta: $elem_ty, c: *mut $elem_ty, rsc: isize, csc: isize)
    {
        const MR: usize = $mr;
        const NR: usize = $nr;

        debug_assert_eq!(beta, <$elem_ty>::zero(), "Beta must be 0 or is not masked");

        let mut pp  = [<$real_ty>::zero(); MR];
        let mut qq  = [<$real_ty>::zero(); MR];
        let mut rr  = [<$real_ty>::zero(); NR];
        let mut ss  = [<$real_ty>::zero(); NR];

        let mut ab: [[$elem_ty; NR]; MR] = [[<$elem_ty>::zero(); NR]; MR];
        let mut areal = a as *const $real_ty;
        let mut breal = b as *const $real_ty;

        unroll_by!($unroll => k, {
            // We set:
            // P + Q i = A
            // R + S i = B
            //
            // see pack_complex for how data is packed
            let aimag = areal.add(MR);
            let bimag = breal.add(NR);

            // AB = PR - QS + i (QR + PS)
            loop_m!(i, {
                pp[i] = at(areal, i);
                qq[i] = at(aimag, i);
            });
            loop_n!(j, {
                rr[j] = at(breal, j);
                ss[j] = at(bimag, j);
            });
            loop_m!(i, {
                loop_n!(j, {
                    // optionally use fma
                    fmuladd!($fma_opt, ab[i][j][0], pp[i], rr[j]);
                    fmuladd!($fma_opt, ab[i][j][1], pp[i], ss[j]);
                    fmuladd!($fma_opt, ab[i][j][0], -qq[i], ss[j]);
                    fmuladd!($fma_opt, ab[i][j][1], qq[i], rr[j]);
                })
            });

            areal = aimag.add(MR);
            breal = bimag.add(NR);
        });

        macro_rules! c {
            ($i:expr, $j:expr) => (c.offset(rsc * $i as isize + csc * $j as isize));
        }

        // set C = α A B
        loop_n!(j, loop_m!(i, *c![i, j] = mul(alpha, ab[i][j])));
    }
    };
}

/// GemmKernel packing trait methods
macro_rules! pack_methods {
    () => {
        #[inline]
        unsafe fn pack_mr(kc: usize, mc: usize, pack: &mut [Self::Elem],
                          a: *const Self::Elem, rsa: isize, csa: isize)
        {
            pack_complex::<Self::MRTy, T, TReal>(kc, mc, pack, a, rsa, csa)
        }

        #[inline]
        unsafe fn pack_nr(kc: usize, mc: usize, pack: &mut [Self::Elem],
                        a: *const Self::Elem, rsa: isize, csa: isize)
        {
            pack_complex::<Self::NRTy, T, TReal>(kc, mc, pack, a, rsa, csa)
        }
    }
}


/// Pack complex: similar to general packing but separate rows for real and imag parts.
///
/// Source matrix contains [p0 + q0i, p1 + q1i, p2 + q2i, ..] and it's packed into
/// alternate rows of real and imaginary parts.
///
/// [ p0 p1 p2 p3 .. (MR repeats)
///   q0 q1 q2 q3 .. (MR repeats)
///   px p_ p_ p_ .. (x = MR)
///   qx q_ q_ q_ .. (x = MR)
///   py p_ p_ p_ .. (y = 2 * MR)
///   qy q_ q_ q_ .. (y = 2 * MR)
///   ...
/// ]
pub(crate) unsafe fn pack_complex<MR, T, TReal>(kc: usize, mc: usize, pack: &mut [T],
                                                a: *const T, rsa: isize, csa: isize)
    where MR: ConstNum,
          T: Element,
          TReal: Element,
{
    // use pointers as pointer to TReal
    let pack = pack.as_mut_ptr() as *mut TReal;
    let areal = a as *const TReal;
    let aimag = areal.add(1);

    assert_eq!(mem::size_of::<T>(), 2 * mem::size_of::<TReal>());

    let mr = MR::VALUE;
    let mut p = 0; // offset into pack

    // general layout case (no contig case when stride != 1)
    for ir in 0..mc/mr {
        let row_offset = ir * mr;
        for j in 0..kc {
            // real row
            for i in 0..mr {
                let a_elt = areal.stride_offset(2 * rsa, i + row_offset)
                                 .stride_offset(2 * csa, j);
                copy_nonoverlapping(a_elt, pack.add(p), 1);
                p += 1;
            }
            // imag row
            for i in 0..mr {
                let a_elt = aimag.stride_offset(2 * rsa, i + row_offset)
                                 .stride_offset(2 * csa, j);
                copy_nonoverlapping(a_elt, pack.add(p), 1);
                p += 1;
            }
        }
    }

    let zero = TReal::zero();

    // Pad with zeros to multiple of kernel size (uneven mc)
    let rest = mc % mr;
    if rest > 0 {
        let row_offset = (mc/mr) * mr;
        for j in 0..kc {
            // real row
            for i in 0..mr {
                if i < rest {
                    let a_elt = areal.stride_offset(2 * rsa, i + row_offset)
                                     .stride_offset(2 * csa, j);
                    copy_nonoverlapping(a_elt, pack.add(p), 1);
                } else {
                    *pack.add(p) = zero;
                }
                p += 1;
            }
            // imag row
            for i in 0..mr {
                if i < rest {
                    let a_elt = aimag.stride_offset(2 * rsa, i + row_offset)
                                     .stride_offset(2 * csa, j);
                    copy_nonoverlapping(a_elt, pack.add(p), 1);
                } else {
                    *pack.add(p) = zero;
                }
                p += 1;
            }
        }
    }
}
