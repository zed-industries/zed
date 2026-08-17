use v_frame::{
    pixel::{Pixel, PixelType},
    plane::PlaneSlice,
};

use crate::{
    cpu::CpuFeatureLevel,
    data::{
        mc::{FilterMode, FilterMode::*},
        plane::PlaneRegionMut,
    },
};

#[allow(clippy::too_many_arguments)]
pub fn put_8tap_internal<T: Pixel>(
    dst: &mut PlaneRegionMut<'_, T>,
    src: PlaneSlice<'_, T>,
    width: usize,
    height: usize,
    col_frac: i32,
    row_frac: i32,
    bit_depth: usize,
    cpu: CpuFeatureLevel,
) {
    let call_rust = |dst: &mut PlaneRegionMut<'_, T>| {
        super::rust::put_8tap_internal(dst, src, width, height, col_frac, row_frac, bit_depth, cpu);
    };

    unsafe {
        // SAFETY: The assembly only supports even heights and valid uncropped
        //         widths
        assert_eq!(height & 1, 0);
        assert!(width.is_power_of_two() && (2..=128).contains(&width));

        // SAFETY: Check bounds of dst
        assert!(dst.rect().width >= width && dst.rect().height >= height);

        // SAFETY: Check bounds of src
        assert!(src.accessible(width + 4, height + 4));
        assert!(src.accessible_neg(3, 3));

        match T::type_enum() {
            PixelType::U8 => match PUT_FNS[cpu.as_index()]
                [get_2d_mode_idx(FilterMode::REGULAR, FilterMode::REGULAR)]
            {
                Some(func) => (func)(
                    dst.data_ptr_mut() as *mut _,
                    T::to_asm_stride(dst.plane_cfg.stride),
                    src.as_ptr() as *const _,
                    T::to_asm_stride(src.plane.cfg.stride),
                    width as i32,
                    height as i32,
                    col_frac,
                    row_frac,
                ),
                None => call_rust(dst),
            },
            PixelType::U16 if bit_depth > 8 => {
                match PUT_HBD_FNS[cpu.as_index()]
                    [get_2d_mode_idx(FilterMode::REGULAR, FilterMode::REGULAR)]
                {
                    Some(func) => (func)(
                        dst.data_ptr_mut() as *mut _,
                        T::to_asm_stride(dst.plane_cfg.stride),
                        src.as_ptr() as *const _,
                        T::to_asm_stride(src.plane.cfg.stride),
                        width as i32,
                        height as i32,
                        col_frac,
                        row_frac,
                        (1 << bit_depth) - 1,
                    ),
                    None => call_rust(dst),
                }
            }
            _ => call_rust(dst),
        }
    }
}

// gets an index that can be mapped to a function for a pair of filter modes
const fn get_2d_mode_idx(mode_x: FilterMode, mode_y: FilterMode) -> usize {
    (mode_x as usize + 4 * (mode_y as usize)) & 15
}

type PutFn = unsafe extern "C" fn(
    dst: *mut u8,
    dst_stride: isize,
    src: *const u8,
    src_stride: isize,
    width: i32,
    height: i32,
    col_frac: i32,
    row_frac: i32,
);

type PutHBDFn = unsafe extern "C" fn(
    dst: *mut u16,
    dst_stride: isize,
    src: *const u16,
    src_stride: isize,
    width: i32,
    height: i32,
    col_frac: i32,
    row_frac: i32,
    bitdepth_max: i32,
);

macro_rules! decl_mc_fns {
      ($(($mode_x:expr, $mode_y:expr, $func_name:ident)),+) => {
        extern "C" {
          $(
            fn $func_name(
              dst: *mut u8, dst_stride: isize, src: *const u8, src_stride: isize,
              w: i32, h: i32, mx: i32, my: i32,
            );
          )*
        }

        static PUT_FNS_NEON: [Option<PutFn>; 16] = {
          let mut out: [Option<PutFn>; 16] = [None; 16];
          $(
            out[get_2d_mode_idx($mode_x, $mode_y)] = Some($func_name);
          )*
          out
        };
      }
    }

decl_mc_fns!(
    (REGULAR, REGULAR, avsc_put_8tap_regular_8bpc_neon),
    (REGULAR, SMOOTH, avsc_put_8tap_regular_smooth_8bpc_neon),
    (REGULAR, SHARP, avsc_put_8tap_regular_sharp_8bpc_neon),
    (SMOOTH, REGULAR, avsc_put_8tap_smooth_regular_8bpc_neon),
    (SMOOTH, SMOOTH, avsc_put_8tap_smooth_8bpc_neon),
    (SMOOTH, SHARP, avsc_put_8tap_smooth_sharp_8bpc_neon),
    (SHARP, REGULAR, avsc_put_8tap_sharp_regular_8bpc_neon),
    (SHARP, SMOOTH, avsc_put_8tap_sharp_smooth_8bpc_neon),
    (SHARP, SHARP, avsc_put_8tap_sharp_8bpc_neon),
    (BILINEAR, BILINEAR, avsc_put_bilin_8bpc_neon)
);

cpu_function_lookup_table!(
  PUT_FNS: [[Option<PutFn>; 16]],
  default: [None; 16],
  [NEON]
);

macro_rules! decl_mc_hbd_fns {
      ($(($mode_x:expr, $mode_y:expr, $func_name:ident)),+) => {
        extern "C" {
          $(
            fn $func_name(
              dst: *mut u16, dst_stride: isize, src: *const u16, src_stride: isize,
              w: i32, h: i32, mx: i32, my: i32, bitdepth_max: i32,
            );
          )*
        }

        static PUT_HBD_FNS_NEON: [Option<PutHBDFn>; 16] = {
          let mut out: [Option<PutHBDFn>; 16] = [None; 16];
          $(
            out[get_2d_mode_idx($mode_x, $mode_y)] = Some($func_name);
          )*
          out
        };
      }
    }

decl_mc_hbd_fns!(
    (REGULAR, REGULAR, avsc_put_8tap_regular_16bpc_neon),
    (REGULAR, SMOOTH, avsc_put_8tap_regular_smooth_16bpc_neon),
    (REGULAR, SHARP, avsc_put_8tap_regular_sharp_16bpc_neon),
    (SMOOTH, REGULAR, avsc_put_8tap_smooth_regular_16bpc_neon),
    (SMOOTH, SMOOTH, avsc_put_8tap_smooth_16bpc_neon),
    (SMOOTH, SHARP, avsc_put_8tap_smooth_sharp_16bpc_neon),
    (SHARP, REGULAR, avsc_put_8tap_sharp_regular_16bpc_neon),
    (SHARP, SMOOTH, avsc_put_8tap_sharp_smooth_16bpc_neon),
    (SHARP, SHARP, avsc_put_8tap_sharp_16bpc_neon),
    (BILINEAR, BILINEAR, avsc_put_bilin_16bpc_neon)
);

cpu_function_lookup_table!(
  PUT_HBD_FNS: [[Option<PutHBDFn>; 16]],
  default: [None; 16],
  [NEON]
);
