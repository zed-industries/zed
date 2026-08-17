#[cfg(asm_neon)]
mod simd_neon;
#[cfg(asm_x86_64)]
mod simd_x86;
#[cfg(test)]
mod tests;

#[cfg(not(any(asm_x86_64, asm_neon)))]
use rust::*;
#[cfg(asm_neon)]
use simd_neon::*;
#[cfg(asm_x86_64)]
use simd_x86::*;
use v_frame::pixel::Pixel;

use super::{block::BlockSize, plane::PlaneRegion};
use crate::cpu::CpuFeatureLevel;

mod rust {
    use v_frame::{
        math::msb,
        pixel::{CastFromPrimitive, Pixel},
    };

    use crate::{
        cpu::CpuFeatureLevel,
        data::{
            hadamard::{hadamard4x4, hadamard8x8},
            plane::{Area, PlaneRegion, Rect},
            sad::get_sad,
        },
    };

    /// Sum of absolute transformed differences over a block.
    /// w and h can be at most 128, the size of the largest block.
    /// Use the sum of 4x4 and 8x8 hadamard transforms for the transform, but
    /// revert to sad on edges when these transforms do not fit into w and h.
    /// 4x4 transforms instead of 8x8 transforms when width or height < 8.
    #[cfg_attr(all(asm_x86_64, target_feature = "avx2"), cold)]
    #[cfg_attr(asm_neon, cold)]
    pub(super) fn get_satd_internal<T: Pixel>(
        plane_org: &PlaneRegion<'_, T>,
        plane_ref: &PlaneRegion<'_, T>,
        w: usize,
        h: usize,
        bit_depth: usize,
        cpu: CpuFeatureLevel,
    ) -> u32 {
        assert!(w <= 128 && h <= 128);
        assert!(plane_org.rect().width >= w && plane_org.rect().height >= h);
        assert!(plane_ref.rect().width >= w && plane_ref.rect().height >= h);

        // Size of hadamard transform should be 4x4 or 8x8
        // 4x* and *x4 use 4x4 and all other use 8x8
        let size: usize = w.min(h).min(8);
        let tx2d = if size == 4 { hadamard4x4 } else { hadamard8x8 };

        let mut sum: u64 = 0;

        // Loop over chunks the size of the chosen transform
        for chunk_y in (0..h).step_by(size) {
            let chunk_h = (h - chunk_y).min(size);
            for chunk_x in (0..w).step_by(size) {
                let chunk_w = (w - chunk_x).min(size);
                let chunk_area = Area::Rect(Rect {
                    x: chunk_x as isize,
                    y: chunk_y as isize,
                    width: chunk_w,
                    height: chunk_h,
                });
                let chunk_org = plane_org.subregion(chunk_area);
                let chunk_ref = plane_ref.subregion(chunk_area);

                // Revert to sad on edge blocks (frame edges)
                if chunk_w != size || chunk_h != size {
                    sum += get_sad(&chunk_org, &chunk_ref, chunk_w, chunk_h, bit_depth, cpu) as u64;
                    continue;
                }

                let buf: &mut [i32] = &mut [0; 8 * 8][..size * size];

                // Move the difference of the transforms to a buffer
                for (row_diff, (row_org, row_ref)) in buf
                    .chunks_mut(size)
                    .zip(chunk_org.rows_iter().zip(chunk_ref.rows_iter()))
                {
                    for (diff, (a, b)) in
                        row_diff.iter_mut().zip(row_org.iter().zip(row_ref.iter()))
                    {
                        *diff = i32::cast_from(*a) - i32::cast_from(*b);
                    }
                }

                // Perform the hadamard transform on the differences
                // SAFETY: A sufficient number elements exist for the size of the transform.
                unsafe {
                    tx2d(buf);
                }

                // Sum the absolute values of the transformed differences
                sum += buf.iter().map(|a| a.unsigned_abs() as u64).sum::<u64>();
            }
        }

        // Normalize the results
        let ln = msb(size as i32) as u64;
        ((sum + (1 << ln >> 1)) >> ln) as u32
    }
}

// BlockSize::BLOCK_SIZES.next_power_of_two()
const DIST_FNS_LENGTH: usize = 32;

const fn to_index(bsize: BlockSize) -> usize {
    bsize as usize & (DIST_FNS_LENGTH - 1)
}

pub(crate) fn get_satd<T: Pixel>(
    src: &PlaneRegion<'_, T>,
    dst: &PlaneRegion<'_, T>,
    w: usize,
    h: usize,
    bit_depth: usize,
    cpu: CpuFeatureLevel,
) -> u32 {
    get_satd_internal(src, dst, w, h, bit_depth, cpu)
}
