/*
 * Copyright (c) 2016, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

#ifndef AOM_AV1_COMMON_CDEF_BLOCK_H_
#define AOM_AV1_COMMON_CDEF_BLOCK_H_

#include "aom_dsp/odintrin.h"

#define CDEF_BLOCKSIZE 64
#define CDEF_BLOCKSIZE_LOG2 6
#define CDEF_NBLOCKS ((1 << MAX_SB_SIZE_LOG2) / 8)
#define CDEF_SB_SHIFT (MAX_SB_SIZE_LOG2 - CDEF_BLOCKSIZE_LOG2)

/* We need to buffer two vertical lines. */
#define CDEF_VBORDER (2)
/* We only need to buffer three horizontal pixels too, but let's align to
   16 bytes (8 x 16 bits) to make vectorization easier. */
#define CDEF_HBORDER (8)
#define CDEF_BSTRIDE \
  ALIGN_POWER_OF_TWO((1 << MAX_SB_SIZE_LOG2) + 2 * CDEF_HBORDER, 3)

#define CDEF_VERY_LARGE (0x4000)
#define CDEF_INBUF_SIZE \
  (CDEF_BSTRIDE * ((1 << MAX_SB_SIZE_LOG2) + 2 * CDEF_VBORDER))

extern const int cdef_pri_taps[2][2];
extern const int cdef_sec_taps[2];
extern const int (*const cdef_directions)[2];

typedef struct {
  uint8_t by;
  uint8_t bx;
} cdef_list;

typedef void (*cdef_filter_block_func)(void *dest, int dstride,
                                       const uint16_t *in, int pri_strength,
                                       int sec_strength, int dir,
                                       int pri_damping, int sec_damping,
                                       int coeff_shift, int block_width,
                                       int block_height);

void av1_cdef_filter_fb(uint8_t *dst8, uint16_t *dst16, int dstride,
                        const uint16_t *in, int xdec, int ydec,
                        int dir[CDEF_NBLOCKS][CDEF_NBLOCKS], int *dirinit,
                        int var[CDEF_NBLOCKS][CDEF_NBLOCKS], int pli,
                        cdef_list *dlist, int cdef_count, int level,
                        int sec_strength, int damping, int coeff_shift);

static inline void fill_rect(uint16_t *dst, int dstride, int v, int h,
                             uint16_t x) {
  for (int i = 0; i < v; i++) {
    for (int j = 0; j < h; j++) {
      dst[i * dstride + j] = x;
    }
  }
}
#endif  // AOM_AV1_COMMON_CDEF_BLOCK_H_
