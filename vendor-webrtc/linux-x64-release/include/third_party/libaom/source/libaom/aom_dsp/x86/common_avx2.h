/*
 * Copyright (c) 2017, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

#ifndef AOM_AOM_DSP_X86_COMMON_AVX2_H_
#define AOM_AOM_DSP_X86_COMMON_AVX2_H_

#include <immintrin.h>

#include "config/aom_config.h"

// Note: in and out could have the same value
static inline void mm256_transpose_16x16(const __m256i *in, __m256i *out) {
  __m256i tr0_0 = _mm256_unpacklo_epi16(in[0], in[1]);
  __m256i tr0_1 = _mm256_unpackhi_epi16(in[0], in[1]);
  __m256i tr0_2 = _mm256_unpacklo_epi16(in[2], in[3]);
  __m256i tr0_3 = _mm256_unpackhi_epi16(in[2], in[3]);
  __m256i tr0_4 = _mm256_unpacklo_epi16(in[4], in[5]);
  __m256i tr0_5 = _mm256_unpackhi_epi16(in[4], in[5]);
  __m256i tr0_6 = _mm256_unpacklo_epi16(in[6], in[7]);
  __m256i tr0_7 = _mm256_unpackhi_epi16(in[6], in[7]);

  __m256i tr0_8 = _mm256_unpacklo_epi16(in[8], in[9]);
  __m256i tr0_9 = _mm256_unpackhi_epi16(in[8], in[9]);
  __m256i tr0_a = _mm256_unpacklo_epi16(in[10], in[11]);
  __m256i tr0_b = _mm256_unpackhi_epi16(in[10], in[11]);
  __m256i tr0_c = _mm256_unpacklo_epi16(in[12], in[13]);
  __m256i tr0_d = _mm256_unpackhi_epi16(in[12], in[13]);
  __m256i tr0_e = _mm256_unpacklo_epi16(in[14], in[15]);
  __m256i tr0_f = _mm256_unpackhi_epi16(in[14], in[15]);

  // 00 10 01 11 02 12 03 13  08 18 09 19 0a 1a 0b 1b
  // 04 14 05 15 06 16 07 17  0c 1c 0d 1d 0e 1e 0f 1f
  // 20 30 21 31 22 32 23 33  28 38 29 39 2a 3a 2b 3b
  // 24 34 25 35 26 36 27 37  2c 3c 2d 3d 2e 3e 2f 3f
  // 40 50 41 51 42 52 43 53  48 58 49 59 4a 5a 4b 5b
  // 44 54 45 55 46 56 47 57  4c 5c 4d 5d 4e 5e 4f 5f
  // 60 70 61 71 62 72 63 73  68 78 69 79 6a 7a 6b 7b
  // 64 74 65 75 66 76 67 77  6c 7c 6d 7d 6e 7e 6f 7f

  // 80 90 81 91 82 92 83 93  88 98 89 99 8a 9a 8b 9b
  // 84 94 85 95 86 96 87 97  8c 9c 8d 9d 8e 9e 8f 9f
  // a0 b0 a1 b1 a2 b2 a3 b3  a8 b8 a9 b9 aa ba ab bb
  // a4 b4 a5 b5 a6 b6 a7 b7  ac bc ad bd ae be af bf
  // c0 d0 c1 d1 c2 d2 c3 d3  c8 d8 c9 d9 ca da cb db
  // c4 d4 c5 d5 c6 d6 c7 d7  cc dc cd dd ce de cf df
  // e0 f0 e1 f1 e2 f2 e3 f3  e8 f8 e9 f9 ea fa eb fb
  // e4 f4 e5 f5 e6 f6 e7 f7  ec fc ed fd ee fe ef ff

  __m256i tr1_0 = _mm256_unpacklo_epi32(tr0_0, tr0_2);
  __m256i tr1_1 = _mm256_unpackhi_epi32(tr0_0, tr0_2);
  __m256i tr1_2 = _mm256_unpacklo_epi32(tr0_1, tr0_3);
  __m256i tr1_3 = _mm256_unpackhi_epi32(tr0_1, tr0_3);
  __m256i tr1_4 = _mm256_unpacklo_epi32(tr0_4, tr0_6);
  __m256i tr1_5 = _mm256_unpackhi_epi32(tr0_4, tr0_6);
  __m256i tr1_6 = _mm256_unpacklo_epi32(tr0_5, tr0_7);
  __m256i tr1_7 = _mm256_unpackhi_epi32(tr0_5, tr0_7);

  __m256i tr1_8 = _mm256_unpacklo_epi32(tr0_8, tr0_a);
  __m256i tr1_9 = _mm256_unpackhi_epi32(tr0_8, tr0_a);
  __m256i tr1_a = _mm256_unpacklo_epi32(tr0_9, tr0_b);
  __m256i tr1_b = _mm256_unpackhi_epi32(tr0_9, tr0_b);
  __m256i tr1_c = _mm256_unpacklo_epi32(tr0_c, tr0_e);
  __m256i tr1_d = _mm256_unpackhi_epi32(tr0_c, tr0_e);
  __m256i tr1_e = _mm256_unpacklo_epi32(tr0_d, tr0_f);
  __m256i tr1_f = _mm256_unpackhi_epi32(tr0_d, tr0_f);

  // 00 10 20 30 01 11 21 31  08 18 28 38 09 19 29 39
  // 02 12 22 32 03 13 23 33  0a 1a 2a 3a 0b 1b 2b 3b
  // 04 14 24 34 05 15 25 35  0c 1c 2c 3c 0d 1d 2d 3d
  // 06 16 26 36 07 17 27 37  0e 1e 2e 3e 0f 1f 2f 3f
  // 40 50 60 70 41 51 61 71  48 58 68 78 49 59 69 79
  // 42 52 62 72 43 53 63 73  4a 5a 6a 7a 4b 5b 6b 7b
  // 44 54 64 74 45 55 65 75  4c 5c 6c 7c 4d 5d 6d 7d
  // 46 56 66 76 47 57 67 77  4e 5e 6e 7e 4f 5f 6f 7f

  // 80 90 a0 b0 81 91 a1 b1  88 98 a8 b8 89 99 a9 b9
  // 82 92 a2 b2 83 93 a3 b3  8a 9a aa ba 8b 9b ab bb
  // 84 94 a4 b4 85 95 a5 b5  8c 9c ac bc 8d 9d ad bd
  // 86 96 a6 b6 87 97 a7 b7  8e ae 9e be 8f 9f af bf
  // c0 d0 e0 f0 c1 d1 e1 f1  c8 d8 e8 f8 c9 d9 e9 f9
  // c2 d2 e2 f2 c3 d3 e3 f3  ca da ea fa cb db eb fb
  // c4 d4 e4 f4 c5 d5 e5 f5  cc dc ef fc cd dd ed fd
  // c6 d6 e6 f6 c7 d7 e7 f7  ce de ee fe cf df ef ff

  tr0_0 = _mm256_unpacklo_epi64(tr1_0, tr1_4);
  tr0_1 = _mm256_unpackhi_epi64(tr1_0, tr1_4);
  tr0_2 = _mm256_unpacklo_epi64(tr1_1, tr1_5);
  tr0_3 = _mm256_unpackhi_epi64(tr1_1, tr1_5);
  tr0_4 = _mm256_unpacklo_epi64(tr1_2, tr1_6);
  tr0_5 = _mm256_unpackhi_epi64(tr1_2, tr1_6);
  tr0_6 = _mm256_unpacklo_epi64(tr1_3, tr1_7);
  tr0_7 = _mm256_unpackhi_epi64(tr1_3, tr1_7);

  tr0_8 = _mm256_unpacklo_epi64(tr1_8, tr1_c);
  tr0_9 = _mm256_unpackhi_epi64(tr1_8, tr1_c);
  tr0_a = _mm256_unpacklo_epi64(tr1_9, tr1_d);
  tr0_b = _mm256_unpackhi_epi64(tr1_9, tr1_d);
  tr0_c = _mm256_unpacklo_epi64(tr1_a, tr1_e);
  tr0_d = _mm256_unpackhi_epi64(tr1_a, tr1_e);
  tr0_e = _mm256_unpacklo_epi64(tr1_b, tr1_f);
  tr0_f = _mm256_unpackhi_epi64(tr1_b, tr1_f);

  // 00 10 20 30 40 50 60 70  08 18 28 38 48 58 68 78
  // 01 11 21 31 41 51 61 71  09 19 29 39 49 59 69 79
  // 02 12 22 32 42 52 62 72  0a 1a 2a 3a 4a 5a 6a 7a
  // 03 13 23 33 43 53 63 73  0b 1b 2b 3b 4b 5b 6b 7b
  // 04 14 24 34 44 54 64 74  0c 1c 2c 3c 4c 5c 6c 7c
  // 05 15 25 35 45 55 65 75  0d 1d 2d 3d 4d 5d 6d 7d
  // 06 16 26 36 46 56 66 76  0e 1e 2e 3e 4e 5e 6e 7e
  // 07 17 27 37 47 57 67 77  0f 1f 2f 3f 4f 5f 6f 7f

  // 80 90 a0 b0 c0 d0 e0 f0  88 98 a8 b8 c8 d8 e8 f8
  // 81 91 a1 b1 c1 d1 e1 f1  89 99 a9 b9 c9 d9 e9 f9
  // 82 92 a2 b2 c2 d2 e2 f2  8a 9a aa ba ca da ea fa
  // 83 93 a3 b3 c3 d3 e3 f3  8b 9b ab bb cb db eb fb
  // 84 94 a4 b4 c4 d4 e4 f4  8c 9c ac bc cc dc ef fc
  // 85 95 a5 b5 c5 d5 e5 f5  8d 9d ad bd cd dd ed fd
  // 86 96 a6 b6 c6 d6 e6 f6  8e ae 9e be ce de ee fe
  // 87 97 a7 b7 c7 d7 e7 f7  8f 9f af bf cf df ef ff

  out[0] = _mm256_permute2x128_si256(tr0_0, tr0_8, 0x20);  // 0010 0000
  out[8] = _mm256_permute2x128_si256(tr0_0, tr0_8, 0x31);  // 0011 0001
  out[1] = _mm256_permute2x128_si256(tr0_1, tr0_9, 0x20);
  out[9] = _mm256_permute2x128_si256(tr0_1, tr0_9, 0x31);
  out[2] = _mm256_permute2x128_si256(tr0_2, tr0_a, 0x20);
  out[10] = _mm256_permute2x128_si256(tr0_2, tr0_a, 0x31);
  out[3] = _mm256_permute2x128_si256(tr0_3, tr0_b, 0x20);
  out[11] = _mm256_permute2x128_si256(tr0_3, tr0_b, 0x31);

  out[4] = _mm256_permute2x128_si256(tr0_4, tr0_c, 0x20);
  out[12] = _mm256_permute2x128_si256(tr0_4, tr0_c, 0x31);
  out[5] = _mm256_permute2x128_si256(tr0_5, tr0_d, 0x20);
  out[13] = _mm256_permute2x128_si256(tr0_5, tr0_d, 0x31);
  out[6] = _mm256_permute2x128_si256(tr0_6, tr0_e, 0x20);
  out[14] = _mm256_permute2x128_si256(tr0_6, tr0_e, 0x31);
  out[7] = _mm256_permute2x128_si256(tr0_7, tr0_f, 0x20);
  out[15] = _mm256_permute2x128_si256(tr0_7, tr0_f, 0x31);
}
#endif  // AOM_AOM_DSP_X86_COMMON_AVX2_H_
