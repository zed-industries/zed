// Copyright 2016 - 2023 Ulrik Sverdrup "bluss"
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use rawpointer::PointerExt;

use core::ptr::copy_nonoverlapping;

use crate::kernel::ConstNum;
use crate::kernel::Element;

/// Pack matrix into `pack`
///
/// + kc: length of the micropanel
/// + mc: number of rows/columns in the matrix to be packed
/// + pack: packing buffer
/// + a: matrix,
/// + rsa: row stride
/// + csa: column stride
///
/// + MR: kernel rows/columns that we round up to
// If one of pack and a is of a reference type, it gets a noalias annotation which
// gives benefits to optimization. The packing buffer is contiguous so it can be passed as a slice
// here.
pub(crate) unsafe fn pack<MR, T>(kc: usize, mc: usize, pack: &mut [T],
                                 a: *const T, rsa: isize, csa: isize)
    where T: Element,
          MR: ConstNum,
{
    pack_impl::<MR, T>(kc, mc, pack, a, rsa, csa)
}

/// Specialized for AVX2
/// Safety: Requires AVX2
#[cfg(any(target_arch="x86", target_arch="x86_64"))]
#[target_feature(enable="avx2")]
pub(crate) unsafe fn pack_avx2<MR, T>(kc: usize, mc: usize, pack: &mut [T],
                                     a: *const T, rsa: isize, csa: isize)
    where T: Element,
          MR: ConstNum,
{
    pack_impl::<MR, T>(kc, mc, pack, a, rsa, csa)
}

/// Pack implementation, see pack above for docs.
///
/// Uses inline(always) so that it can be instantiated for different target features.
#[inline(always)]
unsafe fn pack_impl<MR, T>(kc: usize, mc: usize, pack: &mut [T],
                           a: *const T, rsa: isize, csa: isize)
    where T: Element,
          MR: ConstNum,
{
    let pack = pack.as_mut_ptr();
    let mr = MR::VALUE;
    let mut p = 0; // offset into pack

    if rsa == 1 {
        // if the matrix is contiguous in the same direction we are packing,
        // copy a kernel row at a time.
        for ir in 0..mc/mr {
            let row_offset = ir * mr;
            for j in 0..kc {
                let a_row = a.stride_offset(rsa, row_offset)
                             .stride_offset(csa, j);
                copy_nonoverlapping(a_row, pack.add(p), mr);
                p += mr;
            }
        }
    } else {
        // general layout case
        for ir in 0..mc/mr {
            let row_offset = ir * mr;
            for j in 0..kc {
                for i in 0..mr {
                    let a_elt = a.stride_offset(rsa, i + row_offset)
                                 .stride_offset(csa, j);
                    copy_nonoverlapping(a_elt, pack.add(p), 1);
                    p += 1;
                }
            }
        }
    }

    let zero = <_>::zero();

    // Pad with zeros to multiple of kernel size (uneven mc)
    let rest = mc % mr;
    if rest > 0 {
        let row_offset = (mc/mr) * mr;
        for j in 0..kc {
            for i in 0..mr {
                if i < rest {
                    let a_elt = a.stride_offset(rsa, i + row_offset)
                                 .stride_offset(csa, j);
                    copy_nonoverlapping(a_elt, pack.add(p), 1);
                } else {
                    *pack.add(p) = zero;
                }
                p += 1;
            }
        }
    }
}

