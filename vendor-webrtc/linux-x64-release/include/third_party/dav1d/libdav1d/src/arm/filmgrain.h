/*
 * Copyright © 2018, Niklas Haas
 * Copyright © 2018, VideoLAN and dav1d authors
 * Copyright © 2018, Two Orioles, LLC
 * Copyright © 2021, Martin Storsjo
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright notice, this
 *    list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright notice,
 *    this list of conditions and the following disclaimer in the documentation
 *    and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR
 * ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#include "src/cpu.h"
#include "src/filmgrain.h"
#include "asm-offsets.h"

CHECK_OFFSET(Dav1dFilmGrainData, seed, FGD_SEED);
CHECK_OFFSET(Dav1dFilmGrainData, ar_coeff_lag, FGD_AR_COEFF_LAG);
CHECK_OFFSET(Dav1dFilmGrainData, ar_coeffs_y, FGD_AR_COEFFS_Y);
CHECK_OFFSET(Dav1dFilmGrainData, ar_coeffs_uv, FGD_AR_COEFFS_UV);
CHECK_OFFSET(Dav1dFilmGrainData, ar_coeff_shift, FGD_AR_COEFF_SHIFT);
CHECK_OFFSET(Dav1dFilmGrainData, grain_scale_shift, FGD_GRAIN_SCALE_SHIFT);

CHECK_OFFSET(Dav1dFilmGrainData, scaling_shift, FGD_SCALING_SHIFT);
CHECK_OFFSET(Dav1dFilmGrainData, uv_mult, FGD_UV_MULT);
CHECK_OFFSET(Dav1dFilmGrainData, uv_luma_mult, FGD_UV_LUMA_MULT);
CHECK_OFFSET(Dav1dFilmGrainData, uv_offset, FGD_UV_OFFSET);
CHECK_OFFSET(Dav1dFilmGrainData, clip_to_restricted_range, FGD_CLIP_TO_RESTRICTED_RANGE);

void BF(dav1d_generate_grain_y, neon)(entry buf[][GRAIN_WIDTH],
                                      const Dav1dFilmGrainData *const data
                                      HIGHBD_DECL_SUFFIX);

#define GEN_GRAIN_UV(suff) \
void BF(dav1d_generate_grain_uv_ ## suff, neon)(entry buf[][GRAIN_WIDTH], \
                                                const entry buf_y[][GRAIN_WIDTH], \
                                                const Dav1dFilmGrainData *const data, \
                                                const intptr_t uv \
                                                HIGHBD_DECL_SUFFIX)

GEN_GRAIN_UV(420);
GEN_GRAIN_UV(422);
GEN_GRAIN_UV(444);

// Use ptrdiff_t instead of int for the last few parameters, to get the
// same layout of parameters on the stack across platforms.
void BF(dav1d_fgy_32x32, neon)(pixel *const dst,
                               const pixel *const src,
                               const ptrdiff_t stride,
                               const uint8_t scaling[SCALING_SIZE],
                               const int scaling_shift,
                               const entry grain_lut[][GRAIN_WIDTH],
                               const int offsets[][2],
                               const int h, const ptrdiff_t clip,
                               const ptrdiff_t type
                               HIGHBD_DECL_SUFFIX);

static void fgy_32x32xn_neon(pixel *const dst_row, const pixel *const src_row,
                             const ptrdiff_t stride,
                             const Dav1dFilmGrainData *const data, const size_t pw,
                             const uint8_t scaling[SCALING_SIZE],
                             const entry grain_lut[][GRAIN_WIDTH],
                             const int bh, const int row_num HIGHBD_DECL_SUFFIX)
{
    const int rows = 1 + (data->overlap_flag && row_num > 0);

    // seed[0] contains the current row, seed[1] contains the previous
    unsigned seed[2];
    for (int i = 0; i < rows; i++) {
        seed[i] = data->seed;
        seed[i] ^= (((row_num - i) * 37  + 178) & 0xFF) << 8;
        seed[i] ^= (((row_num - i) * 173 + 105) & 0xFF);
    }

    int offsets[2 /* col offset */][2 /* row offset */];

    // process this row in FG_BLOCK_SIZE^2 blocks
    for (unsigned bx = 0; bx < pw; bx += FG_BLOCK_SIZE) {

        if (data->overlap_flag && bx) {
            // shift previous offsets left
            for (int i = 0; i < rows; i++)
                offsets[1][i] = offsets[0][i];
        }

        // update current offsets
        for (int i = 0; i < rows; i++)
            offsets[0][i] = get_random_number(8, &seed[i]);

        int type = 0;
        if (data->overlap_flag && row_num)
            type |= 1; /* overlap y */
        if (data->overlap_flag && bx)
            type |= 2; /* overlap x */

        BF(dav1d_fgy_32x32, neon)(dst_row + bx, src_row + bx, stride,
                                  scaling, data->scaling_shift,
                                  grain_lut, offsets, bh,
                                  data->clip_to_restricted_range, type
                                  HIGHBD_TAIL_SUFFIX);
    }
}

// Use ptrdiff_t instead of int for the last few parameters, to get the
// parameters on the stack with the same layout across platforms.
#define FGUV(nm, sx, sy) \
void BF(dav1d_fguv_32x32_##nm, neon)(pixel *const dst, \
                                     const pixel *const src, \
                                     const ptrdiff_t stride, \
                                     const uint8_t scaling[SCALING_SIZE], \
                                     const Dav1dFilmGrainData *const data, \
                                     const entry grain_lut[][GRAIN_WIDTH], \
                                     const pixel *const luma_row, \
                                     const ptrdiff_t luma_stride, \
                                     const int offsets[][2], \
                                     const ptrdiff_t h, const ptrdiff_t uv, \
                                     const ptrdiff_t is_id, \
                                     const ptrdiff_t type \
                                     HIGHBD_DECL_SUFFIX); \
static void \
fguv_32x32xn_##nm##_neon(pixel *const dst_row, const pixel *const src_row, \
                  const ptrdiff_t stride, const Dav1dFilmGrainData *const data, \
                  const size_t pw, const uint8_t scaling[SCALING_SIZE], \
                  const entry grain_lut[][GRAIN_WIDTH], const int bh, \
                  const int row_num, const pixel *const luma_row, \
                  const ptrdiff_t luma_stride, const int uv, const int is_id \
                  HIGHBD_DECL_SUFFIX) \
{ \
    const int rows = 1 + (data->overlap_flag && row_num > 0); \
 \
    /* seed[0] contains the current row, seed[1] contains the previous */ \
    unsigned seed[2]; \
    for (int i = 0; i < rows; i++) { \
        seed[i] = data->seed; \
        seed[i] ^= (((row_num - i) * 37  + 178) & 0xFF) << 8; \
        seed[i] ^= (((row_num - i) * 173 + 105) & 0xFF); \
    } \
 \
    int offsets[2 /* col offset */][2 /* row offset */]; \
 \
    /* process this row in FG_BLOCK_SIZE^2 blocks (subsampled) */ \
    for (unsigned bx = 0; bx < pw; bx += FG_BLOCK_SIZE >> sx) { \
        if (data->overlap_flag && bx) { \
            /* shift previous offsets left */ \
            for (int i = 0; i < rows; i++) \
                offsets[1][i] = offsets[0][i]; \
        } \
 \
        /* update current offsets */ \
        for (int i = 0; i < rows; i++) \
            offsets[0][i] = get_random_number(8, &seed[i]); \
 \
        int type = 0; \
        if (data->overlap_flag && row_num) \
            type |= 1; /* overlap y */ \
        if (data->overlap_flag && bx) \
            type |= 2; /* overlap x */ \
        if (data->chroma_scaling_from_luma) \
            type |= 4; \
 \
        BF(dav1d_fguv_32x32_##nm, neon)(dst_row + bx, src_row + bx, stride, \
                                        scaling, data, grain_lut, \
                                        luma_row + (bx << sx), luma_stride, \
                                        offsets, bh, uv, is_id, type \
                                        HIGHBD_TAIL_SUFFIX); \
    } \
}

FGUV(420, 1, 1);
FGUV(422, 1, 0);
FGUV(444, 0, 0);

static ALWAYS_INLINE void film_grain_dsp_init_arm(Dav1dFilmGrainDSPContext *const c) {
    const unsigned flags = dav1d_get_cpu_flags();

    if (!(flags & DAV1D_ARM_CPU_FLAG_NEON)) return;

    c->generate_grain_y = BF(dav1d_generate_grain_y, neon);
    c->generate_grain_uv[DAV1D_PIXEL_LAYOUT_I420 - 1] = BF(dav1d_generate_grain_uv_420, neon);
    c->generate_grain_uv[DAV1D_PIXEL_LAYOUT_I422 - 1] = BF(dav1d_generate_grain_uv_422, neon);
    c->generate_grain_uv[DAV1D_PIXEL_LAYOUT_I444 - 1] = BF(dav1d_generate_grain_uv_444, neon);

    c->fgy_32x32xn = fgy_32x32xn_neon;
    c->fguv_32x32xn[DAV1D_PIXEL_LAYOUT_I420 - 1] = fguv_32x32xn_420_neon;
    c->fguv_32x32xn[DAV1D_PIXEL_LAYOUT_I422 - 1] = fguv_32x32xn_422_neon;
    c->fguv_32x32xn[DAV1D_PIXEL_LAYOUT_I444 - 1] = fguv_32x32xn_444_neon;
}
