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

    // SAFETY: The assembly only supports even heights and valid uncropped
    //         widths
    unsafe {
        assert_eq!(height & 1, 0);
        assert!(width.is_power_of_two() && (2..=128).contains(&width));

        // SAFETY: Check bounds of dst
        assert!(dst.rect().width >= width && dst.rect().height >= height);

        // SAFETY: Check bounds of src
        assert!(src.accessible(width + 4, height + 4));
        assert!(src.accessible_neg(3, 3));

        match T::type_enum() {
            PixelType::U8 => match PUT_FNS[cpu.as_index()][get_2d_mode_idx(REGULAR, REGULAR)] {
                Some(func) => func(
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
            PixelType::U16 => {
                match PUT_HBD_FNS[cpu.as_index()][get_2d_mode_idx(REGULAR, REGULAR)] {
                    Some(func) => func(
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
        }
    }
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

// gets an index that can be mapped to a function for a pair of filter modes
const fn get_2d_mode_idx(mode_x: FilterMode, mode_y: FilterMode) -> usize {
    (mode_x as usize + 4 * (mode_y as usize)) & 15
}

macro_rules! decl_mc_fns {
      ($(($mode_x:expr, $mode_y:expr, $func_name:ident)),+) => {
        pastey::item! {
          extern "C" {
            $(
              fn [<$func_name _ssse3>](
                dst: *mut u8, dst_stride: isize, src: *const u8, src_stride: isize,
                w: i32, h: i32, mx: i32, my: i32
              );

              fn [<$func_name _avx2>](
                dst: *mut u8, dst_stride: isize, src: *const u8, src_stride: isize,
                w: i32, h: i32, mx: i32, my: i32
              );

              fn [<$func_name _avx512icl>](
                dst: *mut u8, dst_stride: isize, src: *const u8, src_stride: isize,
                w: i32, h: i32, mx: i32, my: i32
              );
            )*
          }

          static PUT_FNS_SSSE3: [Option<PutFn>; 16] = {
            let mut out: [Option<PutFn>; 16] = [None; 16];
            $(
              out[get_2d_mode_idx($mode_x, $mode_y)] = Some([<$func_name _ssse3>]);
            )*
            out
          };

          static PUT_FNS_AVX2: [Option<PutFn>; 16] = {
            let mut out: [Option<PutFn>; 16] = [None; 16];
            $(
              out[get_2d_mode_idx($mode_x, $mode_y)] = Some([<$func_name _avx2>]);
            )*
            out
          };

          static PUT_FNS_AVX512ICL: [Option<PutFn>; 16] = {
            let mut out: [Option<PutFn>; 16] = [None; 16];
            $(
              out[get_2d_mode_idx($mode_x, $mode_y)] = Some([<$func_name _avx512icl>]);
            )*
            out
          };
        }
      }
    }

decl_mc_fns!(
    (REGULAR, REGULAR, avsc_put_8tap_regular_8bpc),
    (REGULAR, SMOOTH, avsc_put_8tap_regular_smooth_8bpc),
    (REGULAR, SHARP, avsc_put_8tap_regular_sharp_8bpc),
    (SMOOTH, REGULAR, avsc_put_8tap_smooth_regular_8bpc),
    (SMOOTH, SMOOTH, avsc_put_8tap_smooth_8bpc),
    (SMOOTH, SHARP, avsc_put_8tap_smooth_sharp_8bpc),
    (SHARP, REGULAR, avsc_put_8tap_sharp_regular_8bpc),
    (SHARP, SMOOTH, avsc_put_8tap_sharp_smooth_8bpc),
    (SHARP, SHARP, avsc_put_8tap_sharp_8bpc),
    (BILINEAR, BILINEAR, avsc_put_bilin_8bpc)
);

cpu_function_lookup_table!(
  PUT_FNS: [[Option<PutFn>; 16]],
  default: [None; 16],
  [SSSE3, AVX2, AVX512ICL]
);

macro_rules! decl_mc_hbd_fns {
      ($(($mode_x:expr, $mode_y:expr, $func_name:ident)),+) => {
        pastey::item! {
          extern "C" {
            $(
              fn [<$func_name _ssse3>](
                dst: *mut u16, dst_stride: isize, src: *const u16, src_stride: isize,
                w: i32, h: i32, mx: i32, my: i32, bitdepth_max: i32,
              );

              fn [<$func_name _avx2>](
                dst: *mut u16, dst_stride: isize, src: *const u16, src_stride: isize,
                w: i32, h: i32, mx: i32, my: i32, bitdepth_max: i32,
              );
            )*
          }

          static PUT_HBD_FNS_SSSE3: [Option<PutHBDFn>; 16] = {
            let mut out: [Option<PutHBDFn>; 16] = [None; 16];
            $(
              out[get_2d_mode_idx($mode_x, $mode_y)] = Some([<$func_name _ssse3>]);
            )*
            out
          };

          static PUT_HBD_FNS_AVX2: [Option<PutHBDFn>; 16] = {
            let mut out: [Option<PutHBDFn>; 16] = [None; 16];
            $(
              out[get_2d_mode_idx($mode_x, $mode_y)] = Some([<$func_name _avx2>]);
            )*
            out
          };
        }
      }
    }

decl_mc_hbd_fns!(
    (REGULAR, REGULAR, avsc_put_8tap_regular_16bpc),
    (REGULAR, SMOOTH, avsc_put_8tap_regular_smooth_16bpc),
    (REGULAR, SHARP, avsc_put_8tap_regular_sharp_16bpc),
    (SMOOTH, REGULAR, avsc_put_8tap_smooth_regular_16bpc),
    (SMOOTH, SMOOTH, avsc_put_8tap_smooth_16bpc),
    (SMOOTH, SHARP, avsc_put_8tap_smooth_sharp_16bpc),
    (SHARP, REGULAR, avsc_put_8tap_sharp_regular_16bpc),
    (SHARP, SMOOTH, avsc_put_8tap_sharp_smooth_16bpc),
    (SHARP, SHARP, avsc_put_8tap_sharp_16bpc),
    (BILINEAR, BILINEAR, avsc_put_bilin_16bpc)
);

cpu_function_lookup_table!(
  PUT_HBD_FNS: [[Option<PutHBDFn>; 16]],
  default: [None; 16],
  [SSSE3, AVX2]
);
