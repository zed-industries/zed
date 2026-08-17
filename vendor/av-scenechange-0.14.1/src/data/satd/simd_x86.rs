use v_frame::pixel::{Pixel, PixelType};

use super::{to_index, DIST_FNS_LENGTH};
use crate::{
    cpu::CpuFeatureLevel,
    data::{block::BlockSize, plane::PlaneRegion},
};

type SatdFn = unsafe extern "C" fn(
    src: *const u8,
    src_stride: isize,
    dst: *const u8,
    dst_stride: isize,
) -> u32;

type SatdHBDFn = unsafe extern "C" fn(
    src: *const u16,
    src_stride: isize,
    dst: *const u16,
    dst_stride: isize,
    bdmax: u32,
) -> u32;

macro_rules! declare_asm_dist_fn {
        ($(($name: ident, $T: ident)),+) => (
            $(
            extern "C" { fn $name (
                src: *const $T, src_stride: isize, dst: *const $T, dst_stride: isize
            ) -> u32; }
            )+
        )
    }

macro_rules! declare_asm_satd_hbd_fn {
        ($($name: ident),+) => (
            $(
            extern "C" { pub(crate) fn $name (
                src: *const u16, src_stride: isize, dst: *const u16, dst_stride: isize, bdmax: u32
            ) -> u32; }
            )+
        )
    }

declare_asm_dist_fn![
    // SSSE3
    (avsc_satd_8x8_ssse3, u8),
    // SSE4
    (avsc_satd_4x4_sse4, u8),
    // AVX
    (avsc_satd_4x4_avx2, u8),
    (avsc_satd_8x8_avx2, u8),
    (avsc_satd_16x16_avx2, u8),
    (avsc_satd_32x32_avx2, u8),
    (avsc_satd_64x64_avx2, u8),
    (avsc_satd_128x128_avx2, u8),
    (avsc_satd_4x8_avx2, u8),
    (avsc_satd_8x4_avx2, u8),
    (avsc_satd_8x16_avx2, u8),
    (avsc_satd_16x8_avx2, u8),
    (avsc_satd_16x32_avx2, u8),
    (avsc_satd_32x16_avx2, u8),
    (avsc_satd_32x64_avx2, u8),
    (avsc_satd_64x32_avx2, u8),
    (avsc_satd_64x128_avx2, u8),
    (avsc_satd_128x64_avx2, u8),
    (avsc_satd_4x16_avx2, u8),
    (avsc_satd_16x4_avx2, u8),
    (avsc_satd_8x32_avx2, u8),
    (avsc_satd_32x8_avx2, u8),
    (avsc_satd_16x64_avx2, u8),
    (avsc_satd_64x16_avx2, u8)
];

declare_asm_satd_hbd_fn![
    avsc_satd_4x4_hbd_avx2,
    avsc_satd_8x4_hbd_avx2,
    avsc_satd_4x8_hbd_avx2,
    avsc_satd_8x8_hbd_avx2,
    avsc_satd_16x8_hbd_avx2,
    avsc_satd_16x16_hbd_avx2,
    avsc_satd_32x32_hbd_avx2,
    avsc_satd_64x64_hbd_avx2,
    avsc_satd_128x128_hbd_avx2,
    avsc_satd_16x32_hbd_avx2,
    avsc_satd_16x64_hbd_avx2,
    avsc_satd_32x16_hbd_avx2,
    avsc_satd_32x64_hbd_avx2,
    avsc_satd_64x16_hbd_avx2,
    avsc_satd_64x32_hbd_avx2,
    avsc_satd_64x128_hbd_avx2,
    avsc_satd_128x64_hbd_avx2,
    avsc_satd_32x8_hbd_avx2,
    avsc_satd_8x16_hbd_avx2,
    avsc_satd_8x32_hbd_avx2,
    avsc_satd_16x4_hbd_avx2,
    avsc_satd_4x16_hbd_avx2
];

static SATD_FNS_SSSE3: [Option<SatdFn>; DIST_FNS_LENGTH] = {
    let mut out: [Option<SatdFn>; DIST_FNS_LENGTH] = [None; DIST_FNS_LENGTH];

    use BlockSize::*;

    out[BLOCK_8X8 as usize] = Some(avsc_satd_8x8_ssse3);

    out
};

static SATD_FNS_SSE4_1: [Option<SatdFn>; DIST_FNS_LENGTH] = {
    let mut out: [Option<SatdFn>; DIST_FNS_LENGTH] = [None; DIST_FNS_LENGTH];

    use BlockSize::*;

    out[BLOCK_4X4 as usize] = Some(avsc_satd_4x4_sse4);
    out[BLOCK_8X8 as usize] = Some(avsc_satd_8x8_ssse3);

    out
};

static SATD_FNS_AVX2: [Option<SatdFn>; DIST_FNS_LENGTH] = {
    let mut out: [Option<SatdFn>; DIST_FNS_LENGTH] = [None; DIST_FNS_LENGTH];

    use BlockSize::*;

    out[BLOCK_4X4 as usize] = Some(avsc_satd_4x4_avx2);
    out[BLOCK_8X8 as usize] = Some(avsc_satd_8x8_avx2);
    out[BLOCK_16X16 as usize] = Some(avsc_satd_16x16_avx2);
    out[BLOCK_32X32 as usize] = Some(avsc_satd_32x32_avx2);
    out[BLOCK_64X64 as usize] = Some(avsc_satd_64x64_avx2);
    out[BLOCK_128X128 as usize] = Some(avsc_satd_128x128_avx2);

    out[BLOCK_4X8 as usize] = Some(avsc_satd_4x8_avx2);
    out[BLOCK_8X4 as usize] = Some(avsc_satd_8x4_avx2);
    out[BLOCK_8X16 as usize] = Some(avsc_satd_8x16_avx2);
    out[BLOCK_16X8 as usize] = Some(avsc_satd_16x8_avx2);
    out[BLOCK_16X32 as usize] = Some(avsc_satd_16x32_avx2);
    out[BLOCK_32X16 as usize] = Some(avsc_satd_32x16_avx2);
    out[BLOCK_32X64 as usize] = Some(avsc_satd_32x64_avx2);
    out[BLOCK_64X32 as usize] = Some(avsc_satd_64x32_avx2);
    out[BLOCK_64X128 as usize] = Some(avsc_satd_64x128_avx2);
    out[BLOCK_128X64 as usize] = Some(avsc_satd_128x64_avx2);

    out[BLOCK_4X16 as usize] = Some(avsc_satd_4x16_avx2);
    out[BLOCK_16X4 as usize] = Some(avsc_satd_16x4_avx2);
    out[BLOCK_8X32 as usize] = Some(avsc_satd_8x32_avx2);
    out[BLOCK_32X8 as usize] = Some(avsc_satd_32x8_avx2);
    out[BLOCK_16X64 as usize] = Some(avsc_satd_16x64_avx2);
    out[BLOCK_64X16 as usize] = Some(avsc_satd_64x16_avx2);

    out
};

cpu_function_lookup_table!(
  SATD_FNS: [[Option<SatdFn>; DIST_FNS_LENGTH]],
  default: [None; DIST_FNS_LENGTH],
  [SSSE3, SSE4_1, AVX2]
);

static SATD_HBD_FNS_AVX2: [Option<SatdHBDFn>; DIST_FNS_LENGTH] = {
    let mut out: [Option<SatdHBDFn>; DIST_FNS_LENGTH] = [None; DIST_FNS_LENGTH];

    use BlockSize::*;

    out[BLOCK_4X4 as usize] = Some(avsc_satd_4x4_hbd_avx2);
    out[BLOCK_8X8 as usize] = Some(avsc_satd_8x8_hbd_avx2);
    out[BLOCK_16X16 as usize] = Some(avsc_satd_16x16_hbd_avx2);
    out[BLOCK_32X32 as usize] = Some(avsc_satd_32x32_hbd_avx2);
    out[BLOCK_64X64 as usize] = Some(avsc_satd_64x64_hbd_avx2);
    out[BLOCK_128X128 as usize] = Some(avsc_satd_128x128_hbd_avx2);

    out[BLOCK_4X8 as usize] = Some(avsc_satd_4x8_hbd_avx2);
    out[BLOCK_8X4 as usize] = Some(avsc_satd_8x4_hbd_avx2);
    out[BLOCK_8X16 as usize] = Some(avsc_satd_8x16_hbd_avx2);
    out[BLOCK_16X8 as usize] = Some(avsc_satd_16x8_hbd_avx2);
    out[BLOCK_16X32 as usize] = Some(avsc_satd_16x32_hbd_avx2);
    out[BLOCK_32X16 as usize] = Some(avsc_satd_32x16_hbd_avx2);
    out[BLOCK_32X64 as usize] = Some(avsc_satd_32x64_hbd_avx2);
    out[BLOCK_64X32 as usize] = Some(avsc_satd_64x32_hbd_avx2);
    out[BLOCK_64X128 as usize] = Some(avsc_satd_64x128_hbd_avx2);
    out[BLOCK_128X64 as usize] = Some(avsc_satd_128x64_hbd_avx2);

    out[BLOCK_4X16 as usize] = Some(avsc_satd_4x16_hbd_avx2);
    out[BLOCK_16X4 as usize] = Some(avsc_satd_16x4_hbd_avx2);
    out[BLOCK_8X32 as usize] = Some(avsc_satd_8x32_hbd_avx2);
    out[BLOCK_32X8 as usize] = Some(avsc_satd_32x8_hbd_avx2);
    out[BLOCK_16X64 as usize] = Some(avsc_satd_16x64_hbd_avx2);
    out[BLOCK_64X16 as usize] = Some(avsc_satd_64x16_hbd_avx2);

    out
};

cpu_function_lookup_table!(
  SATD_HBD_FNS: [[Option<SatdHBDFn>; DIST_FNS_LENGTH]],
  default: [None; DIST_FNS_LENGTH],
  [AVX2]
);

pub(super) fn get_satd_internal<T: Pixel>(
    src: &PlaneRegion<'_, T>,
    dst: &PlaneRegion<'_, T>,
    w: usize,
    h: usize,
    bit_depth: usize,
    cpu: CpuFeatureLevel,
) -> u32 {
    let bsize_opt = BlockSize::from_width_and_height_opt(w, h);

    let call_rust = || -> u32 { super::rust::get_satd_internal(dst, src, w, h, bit_depth, cpu) };

    match (bsize_opt, T::type_enum()) {
        (Err(_), _) => call_rust(),
        (Ok(bsize), PixelType::U8) => {
            match SATD_FNS[cpu.as_index()][to_index(bsize)] {
                // SAFETY: Calls Assembly code.
                Some(func) => unsafe {
                    func(
                        src.data_ptr() as *const _,
                        T::to_asm_stride(src.plane_cfg.stride),
                        dst.data_ptr() as *const _,
                        T::to_asm_stride(dst.plane_cfg.stride),
                    )
                },
                None => call_rust(),
            }
        }
        (Ok(bsize), PixelType::U16) => {
            match SATD_HBD_FNS[cpu.as_index()][to_index(bsize)] {
                // SAFETY: Calls Assembly code.
                Some(func) => unsafe {
                    func(
                        src.data_ptr() as *const _,
                        T::to_asm_stride(src.plane_cfg.stride),
                        dst.data_ptr() as *const _,
                        T::to_asm_stride(dst.plane_cfg.stride),
                        (1 << bit_depth) - 1,
                    )
                },
                None => call_rust(),
            }
        }
    }
}
