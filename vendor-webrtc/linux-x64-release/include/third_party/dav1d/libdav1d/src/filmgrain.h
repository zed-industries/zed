/*
 * Copyright © 2018-2021, VideoLAN and dav1d authors
 * Copyright © 2018, Two Orioles, LLC
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

#ifndef DAV1D_SRC_FILM_GRAIN_H
#define DAV1D_SRC_FILM_GRAIN_H

#include "common/bitdepth.h"

#include "src/levels.h"

#define GRAIN_WIDTH 82
#define GRAIN_HEIGHT 73
#define FG_BLOCK_SIZE 32
#if !defined(BITDEPTH) || BITDEPTH == 8
#define SCALING_SIZE 256
typedef int8_t entry;
#else
#define SCALING_SIZE 4096
typedef int16_t entry;
#endif

#define decl_generate_grain_y_fn(name) \
void (name)(entry buf[][GRAIN_WIDTH], \
            const Dav1dFilmGrainData *const data HIGHBD_DECL_SUFFIX)
typedef decl_generate_grain_y_fn(*generate_grain_y_fn);

#define decl_generate_grain_uv_fn(name) \
void (name)(entry buf[][GRAIN_WIDTH], \
            const entry buf_y[][GRAIN_WIDTH], \
            const Dav1dFilmGrainData *const data, const intptr_t uv HIGHBD_DECL_SUFFIX)
typedef decl_generate_grain_uv_fn(*generate_grain_uv_fn);

#define decl_fgy_32x32xn_fn(name) \
void (name)(pixel *dst_row, const pixel *src_row, ptrdiff_t stride, \
            const Dav1dFilmGrainData *data, \
            size_t pw, const uint8_t scaling[SCALING_SIZE], \
            const entry grain_lut[][GRAIN_WIDTH], \
            int bh, int row_num HIGHBD_DECL_SUFFIX)
typedef decl_fgy_32x32xn_fn(*fgy_32x32xn_fn);

#define decl_fguv_32x32xn_fn(name) \
void (name)(pixel *dst_row, const pixel *src_row, ptrdiff_t stride, \
            const Dav1dFilmGrainData *data, size_t pw, \
            const uint8_t scaling[SCALING_SIZE], \
            const entry grain_lut[][GRAIN_WIDTH], int bh, int row_num, \
            const pixel *luma_row, ptrdiff_t luma_stride, \
            int uv_pl, int is_id HIGHBD_DECL_SUFFIX)
typedef decl_fguv_32x32xn_fn(*fguv_32x32xn_fn);

typedef struct Dav1dFilmGrainDSPContext {
    generate_grain_y_fn generate_grain_y;
    generate_grain_uv_fn generate_grain_uv[3];

    fgy_32x32xn_fn fgy_32x32xn;
    fguv_32x32xn_fn fguv_32x32xn[3];
} Dav1dFilmGrainDSPContext;

bitfn_decls(void dav1d_film_grain_dsp_init, Dav1dFilmGrainDSPContext *c);

#endif /* DAV1D_SRC_FILM_GRAIN_H */
