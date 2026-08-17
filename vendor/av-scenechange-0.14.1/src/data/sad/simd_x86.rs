use v_frame::{
    pixel::{Pixel, PixelType},
    plane::Plane,
};

use crate::CpuFeatureLevel;

macro_rules! decl_sad_plane_fn {
      ($($f:ident),+) => {
        extern "C" {
          $(
            fn $f(
              src: *const u8, dst: *const u8, stride: libc::size_t,
              width: libc::size_t, rows: libc::size_t
            ) -> u64;
          )*
        }
      };
    }

decl_sad_plane_fn!(avsc_sad_plane_8bpc_sse2, avsc_sad_plane_8bpc_avx2);

pub(super) fn sad_plane_internal<T: Pixel>(
    src: &Plane<T>,
    dst: &Plane<T>,
    cpu: CpuFeatureLevel,
) -> u64 {
    use std::mem;

    assert_eq!(src.cfg.width, dst.cfg.width);
    assert_eq!(src.cfg.stride, dst.cfg.stride);
    assert_eq!(src.cfg.height, dst.cfg.height);
    assert!(src.cfg.width <= src.cfg.stride);

    match T::type_enum() {
        PixelType::U8 => {
            // helper macro to reduce boilerplate
            macro_rules! call_asm {
                ($func:ident, $src:expr, $dst:expr, $cpu:expr) => {
                    // SAFETY: Calls Assembly code.
                    unsafe {
                        let result = $func(
                            mem::transmute::<*const T, *const u8>(src.data_origin().as_ptr()),
                            mem::transmute::<*const T, *const u8>(dst.data_origin().as_ptr()),
                            src.cfg.stride,
                            src.cfg.width,
                            src.cfg.height,
                        );

                        result
                    }
                };
            }

            if cpu >= CpuFeatureLevel::AVX2 {
                call_asm!(avsc_sad_plane_8bpc_avx2, src, dst, cpu)
            } else if cpu >= CpuFeatureLevel::SSE2 {
                call_asm!(avsc_sad_plane_8bpc_sse2, src, dst, cpu)
            } else {
                super::rust::sad_plane_internal(src, dst, cpu)
            }
        }
        PixelType::U16 => super::rust::sad_plane_internal(src, dst, cpu),
    }
}
