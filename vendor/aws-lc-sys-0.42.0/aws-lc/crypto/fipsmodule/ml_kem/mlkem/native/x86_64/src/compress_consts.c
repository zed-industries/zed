/*
 * Copyright (c) The mlkem-native project authors
 * SPDX-License-Identifier: Apache-2.0 OR ISC OR MIT
 */

/*
 * WARNING: This file is auto-generated from scripts/autogen
 *          in the mlkem-native repository.
 *          Do not modify it directly.
 */

#include "../../../common.h"

#if defined(MLK_ARITH_BACKEND_X86_64_DEFAULT)

#include "compress_consts.h"

#if !defined(MLK_CONFIG_MULTILEVEL_NO_SHARED) &&                   \
    (defined(MLK_CONFIG_MULTILEVEL_WITH_SHARED) || MLKEM_K == 2 || \
     MLKEM_K == 3)

MLK_ALIGN const uint8_t mlk_compress_d4_data[32] = {
    0, 0, 0, 0, 4, 0, 0, 0, 1, 0, 0, 0, 5, 0, 0, 0,
    2, 0, 0, 0, 6, 0, 0, 0, 3, 0, 0, 0, 7, 0, 0, 0, /* permdidx */
};

MLK_ALIGN const uint8_t mlk_decompress_d4_data[32] = {
    0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3,
    4, 4, 4, 4, 5, 5, 5, 5, 6, 6, 6, 6, 7, 7, 7, 7, /* shufbidx */
};

MLK_ALIGN const uint8_t mlk_compress_d10_data[32] = {
    0,   1,   2,   3,   4,   8, 9,  10, 11, 12,  255,
    255, 255, 255, 255, 255, 9, 10, 11, 12, 255, 255,
    255, 255, 255, 255, 0,   1, 2,  3,  4,  8, /* shufbidx */
};

MLK_ALIGN const uint8_t mlk_decompress_d10_data[32] = {
    0, 1, 1, 2, 2, 3, 3, 4, 5, 6, 6, 7, 7, 8,  8,  9,
    2, 3, 3, 4, 4, 5, 5, 6, 7, 8, 8, 9, 9, 10, 10, 11, /* shufbidx */
};

#endif /* !MLK_CONFIG_MULTILEVEL_NO_SHARED &&                                 \
          (MLK_CONFIG_MULTILEVEL_WITH_SHARED || MLKEM_K == 2 || MLKEM_K == 3) \
        */

#if !defined(MLK_CONFIG_MULTILEVEL_NO_SHARED) && \
    (defined(MLK_CONFIG_MULTILEVEL_WITH_SHARED) || MLKEM_K == 4)

MLK_ALIGN const uint8_t
    mlk_compress_d5_data[32] = {
        0, 1,  2,  3,  4,   255, 255, 255, 255, 255, 8,
        9, 10, 11, 12, 255, 9,   10,  11,  12,  255, 0,
        1, 2,  3,  4,  255, 255, 255, 255, 255, 8, /* shufbidx */
};

/* shufbidx[0:32], mask[32:64], shift[64:96] */
MLK_ALIGN const uint8_t mlk_decompress_d5_data[96] = {
    0,   0, 0,   1, 1,   1,  1,   2,  2,   3, 3,   3, 3,   4, 4,   4, 5,  5,
    5,   6, 6,   6, 6,   7,  7,   8,  8,   8, 8,   9, 9,   9, /* shufbidx */
    31,  0, 224, 3, 124, 0,  128, 15, 240, 1, 62,  0, 192, 7, 248, 0, 31, 0,
    224, 3, 124, 0, 128, 15, 240, 1,  62,  0, 192, 7, 248, 0, /* mask */
    0,   4, 32,  0, 0,   1,  8,   0,  64,  0, 0,   2, 16,  0, 128, 0, 0,  4,
    32,  0, 0,   1, 8,   0,  64,  0,  0,   2, 16,  0, 128, 0, /* shift */
};

/* srlvqidx[0:32], shufbidx[32:64] */
MLK_ALIGN const uint8_t mlk_compress_d11_data[64] = {
    10, 0, 0, 0, 0,  0,   0,   0,   30,  0, 0,  0,   0,   0,   0,   0,   10,
    0,  0, 0, 0, 0,  0,   0,   30,  0,   0, 0,  0,   0,   0,   0, /* srlvqidx */
    0,  1, 2, 3, 4,  5,   6,   7,   8,   9, 10, 255, 255, 255, 255, 255, 5,
    6,  7, 8, 9, 10, 255, 255, 255, 255, 0, 0,  1,   2,   3,   4, /* shufbidx */
};

/* shufbidx[0:32], srlvdidx[32:64], srlvqidx[64:96], shift[96:128] */
MLK_ALIGN const uint8_t mlk_decompress_d11_data[128] = {
    0,  1, 1, 2, 2, 3, 4,  5, 5, 6, 6, 7,  8,  9,  9,  10,
    3,  4, 4, 5, 5, 6, 7,  8, 8, 9, 9, 10, 11, 12, 12, 13, /* shufbidx */
    0,  0, 0, 0, 1, 0, 0,  0, 0, 0, 0, 0,  0,  0,  0,  0,
    0,  0, 0, 0, 1, 0, 0,  0, 0, 0, 0, 0,  0,  0,  0,  0, /* srlvdidx */
    0,  0, 0, 0, 0, 0, 0,  0, 2, 0, 0, 0,  0,  0,  0,  0,
    0,  0, 0, 0, 0, 0, 0,  0, 2, 0, 0, 0,  0,  0,  0,  0, /* srlvqidx */
    32, 0, 4, 0, 1, 0, 32, 0, 8, 0, 1, 0,  32, 0,  4,  0,
    32, 0, 4, 0, 1, 0, 32, 0, 8, 0, 1, 0,  32, 0,  4,  0, /* shift */
};

#endif /* !MLK_CONFIG_MULTILEVEL_NO_SHARED && \
          (MLK_CONFIG_MULTILEVEL_WITH_SHARED || MLKEM_K == 4) */

#else /* MLK_ARITH_BACKEND_X86_64_DEFAULT */

MLK_EMPTY_CU(avx2_compress_consts)

#endif /* !MLK_ARITH_BACKEND_X86_64_DEFAULT */
