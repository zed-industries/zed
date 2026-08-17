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
type SatdHbdFn = unsafe extern "C" fn(
    src: *const u16,
    src_stride: isize,
    dst: *const u16,
    dst_stride: isize,
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

declare_asm_dist_fn![
    // SATD
    (avsc_satd4x4_neon, u8),
    (avsc_satd4x8_neon, u8),
    (avsc_satd4x16_neon, u8),
    (avsc_satd8x4_neon, u8),
    (avsc_satd8x8_neon, u8),
    (avsc_satd8x16_neon, u8),
    (avsc_satd8x32_neon, u8),
    (avsc_satd16x4_neon, u8),
    (avsc_satd16x8_neon, u8),
    (avsc_satd16x16_neon, u8),
    (avsc_satd16x32_neon, u8),
    (avsc_satd16x64_neon, u8),
    (avsc_satd32x8_neon, u8),
    (avsc_satd32x16_neon, u8),
    (avsc_satd32x32_neon, u8),
    (avsc_satd32x64_neon, u8),
    (avsc_satd64x16_neon, u8),
    (avsc_satd64x32_neon, u8),
    (avsc_satd64x64_neon, u8),
    (avsc_satd64x128_neon, u8),
    (avsc_satd128x64_neon, u8),
    (avsc_satd128x128_neon, u8),
    // SATD HBD
    (avsc_satd4x4_hbd_neon, u16),
    (avsc_satd4x8_hbd_neon, u16),
    (avsc_satd4x16_hbd_neon, u16),
    (avsc_satd8x4_hbd_neon, u16),
    (avsc_satd8x8_hbd_neon, u16),
    (avsc_satd8x16_hbd_neon, u16),
    (avsc_satd8x32_hbd_neon, u16),
    (avsc_satd16x4_hbd_neon, u16),
    (avsc_satd16x8_hbd_neon, u16),
    (avsc_satd16x16_hbd_neon, u16),
    (avsc_satd16x32_hbd_neon, u16),
    (avsc_satd16x64_hbd_neon, u16),
    (avsc_satd32x8_hbd_neon, u16),
    (avsc_satd32x16_hbd_neon, u16),
    (avsc_satd32x32_hbd_neon, u16),
    (avsc_satd32x64_hbd_neon, u16),
    (avsc_satd64x16_hbd_neon, u16),
    (avsc_satd64x32_hbd_neon, u16),
    (avsc_satd64x64_hbd_neon, u16),
    (avsc_satd64x128_hbd_neon, u16),
    (avsc_satd128x64_hbd_neon, u16),
    (avsc_satd128x128_hbd_neon, u16)
];

static SATD_FNS_NEON: [Option<SatdFn>; DIST_FNS_LENGTH] = {
    let mut out: [Option<SatdFn>; DIST_FNS_LENGTH] = [None; DIST_FNS_LENGTH];

    use crate::data::block::BlockSize::*;

    out[BLOCK_4X4 as usize] = Some(avsc_satd4x4_neon);
    out[BLOCK_4X8 as usize] = Some(avsc_satd4x8_neon);
    out[BLOCK_4X16 as usize] = Some(avsc_satd4x16_neon);
    out[BLOCK_8X4 as usize] = Some(avsc_satd8x4_neon);
    out[BLOCK_16X4 as usize] = Some(avsc_satd16x4_neon);

    out[BLOCK_8X8 as usize] = Some(avsc_satd8x8_neon);
    out[BLOCK_8X16 as usize] = Some(avsc_satd8x16_neon);
    out[BLOCK_8X32 as usize] = Some(avsc_satd8x32_neon);
    out[BLOCK_16X8 as usize] = Some(avsc_satd16x8_neon);
    out[BLOCK_16X16 as usize] = Some(avsc_satd16x16_neon);
    out[BLOCK_16X32 as usize] = Some(avsc_satd16x32_neon);
    out[BLOCK_16X64 as usize] = Some(avsc_satd16x64_neon);
    out[BLOCK_32X8 as usize] = Some(avsc_satd32x8_neon);
    out[BLOCK_32X16 as usize] = Some(avsc_satd32x16_neon);
    out[BLOCK_32X32 as usize] = Some(avsc_satd32x32_neon);
    out[BLOCK_32X64 as usize] = Some(avsc_satd32x64_neon);
    out[BLOCK_64X16 as usize] = Some(avsc_satd64x16_neon);
    out[BLOCK_64X32 as usize] = Some(avsc_satd64x32_neon);
    out[BLOCK_64X64 as usize] = Some(avsc_satd64x64_neon);
    out[BLOCK_64X128 as usize] = Some(avsc_satd64x128_neon);
    out[BLOCK_128X64 as usize] = Some(avsc_satd128x64_neon);
    out[BLOCK_128X128 as usize] = Some(avsc_satd128x128_neon);

    out
};

static SATD_HBD_FNS_NEON: [Option<SatdHbdFn>; DIST_FNS_LENGTH] = {
    let mut out: [Option<SatdHbdFn>; DIST_FNS_LENGTH] = [None; DIST_FNS_LENGTH];

    use crate::data::block::BlockSize::*;

    out[BLOCK_4X4 as usize] = Some(avsc_satd4x4_hbd_neon);
    out[BLOCK_4X8 as usize] = Some(avsc_satd4x8_hbd_neon);
    out[BLOCK_4X16 as usize] = Some(avsc_satd4x16_hbd_neon);
    out[BLOCK_8X4 as usize] = Some(avsc_satd8x4_hbd_neon);
    out[BLOCK_16X4 as usize] = Some(avsc_satd16x4_hbd_neon);

    out[BLOCK_8X8 as usize] = Some(avsc_satd8x8_hbd_neon);
    out[BLOCK_8X16 as usize] = Some(avsc_satd8x16_hbd_neon);
    out[BLOCK_8X32 as usize] = Some(avsc_satd8x32_hbd_neon);
    out[BLOCK_16X8 as usize] = Some(avsc_satd16x8_hbd_neon);
    out[BLOCK_16X16 as usize] = Some(avsc_satd16x16_hbd_neon);
    out[BLOCK_16X32 as usize] = Some(avsc_satd16x32_hbd_neon);
    out[BLOCK_16X64 as usize] = Some(avsc_satd16x64_hbd_neon);
    out[BLOCK_32X8 as usize] = Some(avsc_satd32x8_hbd_neon);
    out[BLOCK_32X16 as usize] = Some(avsc_satd32x16_hbd_neon);
    out[BLOCK_32X32 as usize] = Some(avsc_satd32x32_hbd_neon);
    out[BLOCK_32X64 as usize] = Some(avsc_satd32x64_hbd_neon);
    out[BLOCK_64X16 as usize] = Some(avsc_satd64x16_hbd_neon);
    out[BLOCK_64X32 as usize] = Some(avsc_satd64x32_hbd_neon);
    out[BLOCK_64X64 as usize] = Some(avsc_satd64x64_hbd_neon);
    out[BLOCK_64X128 as usize] = Some(avsc_satd64x128_hbd_neon);
    out[BLOCK_128X64 as usize] = Some(avsc_satd128x64_hbd_neon);
    out[BLOCK_128X128 as usize] = Some(avsc_satd128x128_hbd_neon);

    out
};

cpu_function_lookup_table!(
  SATD_FNS: [[Option<SatdFn>; DIST_FNS_LENGTH]],
  default: [None; DIST_FNS_LENGTH],
  [NEON]
);

cpu_function_lookup_table!(
  SATD_HBD_FNS: [[Option<SatdHbdFn>; DIST_FNS_LENGTH]],
  default: [None; DIST_FNS_LENGTH],
  [NEON]
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

    let call_rust = || -> u32 { super::rust::get_satd_internal(src, dst, w, h, bit_depth, cpu) };

    match (bsize_opt, T::type_enum()) {
        (Err(_), _) => call_rust(),
        (Ok(bsize), PixelType::U8) => {
            match SATD_FNS[cpu.as_index()][to_index(bsize)] {
                // SAFETY: Calls Assembly code.
                Some(func) => unsafe {
                    (func)(
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
                    (func)(
                        src.data_ptr() as *const _,
                        T::to_asm_stride(src.plane_cfg.stride),
                        dst.data_ptr() as *const _,
                        T::to_asm_stride(dst.plane_cfg.stride),
                    )
                },
                None => call_rust(),
            }
        }
    }
}
