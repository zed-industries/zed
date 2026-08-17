#include "src/cpu.h"
#include "src/cdef.h"

extern void BF(dav1d_cdef_filter_block_4x4, rvv)(pixel *dst, const ptrdiff_t dst_stride,
                                    const pixel (*left)[2],
                                    const pixel *const top, const pixel *const bottom,
                                    const int pri_strength, const int sec_strength, const int dir,
                                    const int damping, const enum CdefEdgeFlags edges HIGHBD_DECL_SUFFIX);

extern void BF(dav1d_cdef_filter_block_4x8, rvv)(pixel *dst, const ptrdiff_t dst_stride,
                                    const pixel (*left)[2],
                                    const pixel *const top, const pixel *const bottom,
                                    const int pri_strength, const int sec_strength, const int dir,
                                    const int damping, const enum CdefEdgeFlags edges HIGHBD_DECL_SUFFIX);

extern void BF(dav1d_cdef_filter_block_8x8, rvv)(pixel *dst, const ptrdiff_t dst_stride,
                                    const pixel (*left)[2],
                                    const pixel *const top, const pixel *const bottom,
                                    const int pri_strength, const int sec_strength, const int dir,
                                    const int damping, const enum CdefEdgeFlags edges HIGHBD_DECL_SUFFIX);


static ALWAYS_INLINE void cdef_dsp_init_riscv(Dav1dCdefDSPContext *const c) {
    const unsigned flags = dav1d_get_cpu_flags();
    if (!(flags & DAV1D_RISCV_CPU_FLAG_V)) return;

    // c->dir = BF(dav1d_cdef_dir, rvv);
    c->fb[0] = BF(dav1d_cdef_filter_block_8x8, rvv);
    c->fb[1] = BF(dav1d_cdef_filter_block_4x8, rvv);
    c->fb[2] = BF(dav1d_cdef_filter_block_4x4, rvv);
}
