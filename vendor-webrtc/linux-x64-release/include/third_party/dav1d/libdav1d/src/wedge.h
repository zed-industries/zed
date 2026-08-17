/*
 * Copyright © 2018, VideoLAN and dav1d authors
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

#ifndef DAV1D_SRC_WEDGE_H
#define DAV1D_SRC_WEDGE_H

#include "src/levels.h"

typedef struct {
    /* Offsets, in units of 8 bytes, relative to the start of the struct. */
    struct {
        uint16_t wedge[2 /* sign */][16 /* wedge_idx */];
        uint16_t ii[N_INTER_INTRA_PRED_MODES];
    } offsets[3 /* 444, 422, 420 */][BS_8x8 - BS_32x32 + 1];

    uint8_t ALIGN(wedge_444_32x32[    16 * 32 * 32], 64);
    uint8_t ALIGN(wedge_444_32x16[    16 * 32 * 16], 64);
    uint8_t ALIGN(wedge_444_32x8 [    16 * 32 *  8], 64);
    uint8_t ALIGN(wedge_444_16x32[    16 * 16 * 32], 64);
    uint8_t ALIGN(wedge_444_16x16[    16 * 16 * 16], 64);
    uint8_t ALIGN(wedge_444_16x8 [    16 * 16 *  8], 64);
    uint8_t ALIGN(wedge_444_8x32 [    16 *  8 * 32], 64);
    uint8_t ALIGN(wedge_444_8x16 [    16 *  8 * 16], 64);
    uint8_t ALIGN(wedge_444_8x8  [    16 *  8 *  8], 64);

    uint8_t ALIGN(wedge_422_16x32[2 * 16 * 16 * 32], 64);
    uint8_t ALIGN(wedge_422_16x16[2 * 16 * 16 * 16], 64);
    uint8_t ALIGN(wedge_422_16x8 [2 * 16 * 16 *  8], 64);
    uint8_t ALIGN(wedge_422_8x32 [2 * 16 *  8 * 32], 64);
    uint8_t ALIGN(wedge_422_8x16 [2 * 16 *  8 * 16], 64);
    uint8_t ALIGN(wedge_422_8x8  [2 * 16 *  8 *  8], 64);
    uint8_t ALIGN(wedge_422_4x32 [2 * 16 *  4 * 32], 64);
    uint8_t ALIGN(wedge_422_4x16 [2 * 16 *  4 * 16], 64);
    uint8_t ALIGN(wedge_422_4x8  [2 * 16 *  4 *  8], 64);

    uint8_t ALIGN(wedge_420_16x16[2 * 16 * 16 * 16], 64);
    uint8_t ALIGN(wedge_420_16x8 [2 * 16 * 16 *  8], 64);
    uint8_t ALIGN(wedge_420_16x4 [2 * 16 * 16 *  4], 64);
    uint8_t ALIGN(wedge_420_8x16 [2 * 16 *  8 * 16], 64);
    uint8_t ALIGN(wedge_420_8x8  [2 * 16 *  8 *  8], 64);
    uint8_t ALIGN(wedge_420_8x4  [2 * 16 *  8 *  4], 64);
    uint8_t ALIGN(wedge_420_4x16 [2 * 16 *  4 * 16], 64);
    uint8_t ALIGN(wedge_420_4x8  [2 * 16 *  4 *  8], 64);
    uint8_t ALIGN(wedge_420_4x4  [2 * 16 *  4 *  4], 64);

    uint8_t ALIGN(ii_dc         [    32 * 32], 64);
    uint8_t ALIGN(ii_nondc_32x32[3 * 32 * 32], 64);
    uint8_t ALIGN(ii_nondc_16x32[3 * 16 * 32], 64);
    uint8_t ALIGN(ii_nondc_16x16[3 * 16 * 16], 64);
    uint8_t ALIGN(ii_nondc_8x32 [3 *  8 * 32], 64);
    uint8_t ALIGN(ii_nondc_8x16 [3 *  8 * 16], 64);
    uint8_t ALIGN(ii_nondc_8x8  [3 *  8 *  8], 64);
    uint8_t ALIGN(ii_nondc_4x16 [3 *  4 * 16], 64);
    uint8_t ALIGN(ii_nondc_4x8  [3 *  4 *  8], 32);
    uint8_t ALIGN(ii_nondc_4x4  [3 *  4 *  4], 16);
} Dav1dMasks;

#define II_MASK(c, bs, b) \
    ((const uint8_t*)((uintptr_t)&dav1d_masks + \
    (size_t)((b)->interintra_type == INTER_INTRA_BLEND ? \
    dav1d_masks.offsets[c][(bs)-BS_32x32].ii[(b)->interintra_mode] : \
    dav1d_masks.offsets[c][(bs)-BS_32x32].wedge[0][(b)->wedge_idx]) * 8))

#define WEDGE_MASK(c, bs, sign, idx) \
    ((const uint8_t*)((uintptr_t)&dav1d_masks + \
    (size_t)dav1d_masks.offsets[c][(bs)-BS_32x32].wedge[sign][idx] * 8))

EXTERN Dav1dMasks dav1d_masks;

void dav1d_init_ii_wedge_masks(void);

#endif /* DAV1D_SRC_WEDGE_H */
