/*
 * Copyright (c) 2021, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */
#ifndef AOM_AOM_DSP_X86_INTRAPRED_UTILS_H_
#define AOM_AOM_DSP_X86_INTRAPRED_UTILS_H_

#include <emmintrin.h>  // SSE2
#include "aom/aom_integer.h"
#include "config/aom_config.h"
#include "config/aom_dsp_rtcd.h"

static DECLARE_ALIGNED(16, uint8_t, EvenOddMaskx[8][16]) = {
  { 0, 2, 4, 6, 8, 10, 12, 14, 1, 3, 5, 7, 9, 11, 13, 15 },
  { 0, 1, 3, 5, 7, 9, 11, 13, 0, 2, 4, 6, 8, 10, 12, 14 },
  { 0, 0, 2, 4, 6, 8, 10, 12, 0, 0, 3, 5, 7, 9, 11, 13 },
  { 0, 0, 0, 3, 5, 7, 9, 11, 0, 0, 0, 4, 6, 8, 10, 12 },
  { 0, 0, 0, 0, 4, 6, 8, 10, 0, 0, 0, 0, 5, 7, 9, 11 },
  { 0, 0, 0, 0, 0, 5, 7, 9, 0, 0, 0, 0, 0, 6, 8, 10 },
  { 0, 0, 0, 0, 0, 0, 6, 8, 0, 0, 0, 0, 0, 0, 7, 9 },
  { 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 8 }
};

static DECLARE_ALIGNED(16, uint8_t, LoadMaskx[16][16]) = {
  { 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15 },
  { 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14 },
  { 0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13 },
  { 0, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12 },
  { 0, 0, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11 },
  { 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10 },
  { 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9 },
  { 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8 },
  { 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7 },
  { 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 5, 6 },
  { 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 5 },
  { 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4 },
  { 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3 },
  { 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2 },
  { 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1 },
  { 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 },
};

static DECLARE_ALIGNED(32, int, LoadMaskz2[8][8]) = {
  { -1, 0, 0, 0, 0, 0, 0, 0 },       { -1, -1, 0, 0, 0, 0, 0, 0 },
  { -1, -1, -1, 0, 0, 0, 0, 0 },     { -1, -1, -1, -1, 0, 0, 0, 0 },
  { -1, -1, -1, -1, -1, 0, 0, 0 },   { -1, -1, -1, -1, -1, -1, 0, 0 },
  { -1, -1, -1, -1, -1, -1, -1, 0 }, { -1, -1, -1, -1, -1, -1, -1, -1 },
};

static inline void transpose4x16_sse2(__m128i *x, __m128i *d) {
  __m128i w0, w1, w2, w3, ww0, ww1, ww2, ww3;
  w0 = _mm_unpacklo_epi8(x[0], x[1]);
  w1 = _mm_unpacklo_epi8(x[2], x[3]);
  w2 = _mm_unpackhi_epi8(x[0], x[1]);
  w3 = _mm_unpackhi_epi8(x[2], x[3]);

  ww0 = _mm_unpacklo_epi16(w0, w1);
  ww1 = _mm_unpacklo_epi16(w2, w3);
  ww2 = _mm_unpackhi_epi16(w0, w1);
  ww3 = _mm_unpackhi_epi16(w2, w3);

  w0 = _mm_unpacklo_epi32(ww0, ww1);
  w2 = _mm_unpacklo_epi32(ww2, ww3);
  w1 = _mm_unpackhi_epi32(ww0, ww1);
  w3 = _mm_unpackhi_epi32(ww2, ww3);

  d[0] = _mm_unpacklo_epi64(w0, w2);
  d[1] = _mm_unpackhi_epi64(w0, w2);
  d[2] = _mm_unpacklo_epi64(w1, w3);
  d[3] = _mm_unpackhi_epi64(w1, w3);

  d[4] = _mm_srli_si128(d[0], 8);
  d[5] = _mm_srli_si128(d[1], 8);
  d[6] = _mm_srli_si128(d[2], 8);
  d[7] = _mm_srli_si128(d[3], 8);

  d[8] = _mm_srli_si128(d[0], 4);
  d[9] = _mm_srli_si128(d[1], 4);
  d[10] = _mm_srli_si128(d[2], 4);
  d[11] = _mm_srli_si128(d[3], 4);

  d[12] = _mm_srli_si128(d[0], 12);
  d[13] = _mm_srli_si128(d[1], 12);
  d[14] = _mm_srli_si128(d[2], 12);
  d[15] = _mm_srli_si128(d[3], 12);
}

static inline void transpose16x16_sse2(__m128i *x, __m128i *d) {
  __m128i w0, w1, w2, w3, w4, w5, w6, w7, w8, w9;
  __m128i w10, w11, w12, w13, w14, w15;

  w0 = _mm_unpacklo_epi8(x[0], x[1]);
  w1 = _mm_unpacklo_epi8(x[2], x[3]);
  w2 = _mm_unpacklo_epi8(x[4], x[5]);
  w3 = _mm_unpacklo_epi8(x[6], x[7]);

  w8 = _mm_unpacklo_epi8(x[8], x[9]);
  w9 = _mm_unpacklo_epi8(x[10], x[11]);
  w10 = _mm_unpacklo_epi8(x[12], x[13]);
  w11 = _mm_unpacklo_epi8(x[14], x[15]);

  w4 = _mm_unpacklo_epi16(w0, w1);
  w5 = _mm_unpacklo_epi16(w2, w3);
  w12 = _mm_unpacklo_epi16(w8, w9);
  w13 = _mm_unpacklo_epi16(w10, w11);

  w6 = _mm_unpacklo_epi32(w4, w5);
  w7 = _mm_unpackhi_epi32(w4, w5);
  w14 = _mm_unpacklo_epi32(w12, w13);
  w15 = _mm_unpackhi_epi32(w12, w13);

  // Store first 4-line result
  d[0] = _mm_unpacklo_epi64(w6, w14);
  d[1] = _mm_unpackhi_epi64(w6, w14);
  d[2] = _mm_unpacklo_epi64(w7, w15);
  d[3] = _mm_unpackhi_epi64(w7, w15);

  w4 = _mm_unpackhi_epi16(w0, w1);
  w5 = _mm_unpackhi_epi16(w2, w3);
  w12 = _mm_unpackhi_epi16(w8, w9);
  w13 = _mm_unpackhi_epi16(w10, w11);

  w6 = _mm_unpacklo_epi32(w4, w5);
  w7 = _mm_unpackhi_epi32(w4, w5);
  w14 = _mm_unpacklo_epi32(w12, w13);
  w15 = _mm_unpackhi_epi32(w12, w13);

  // Store second 4-line result
  d[4] = _mm_unpacklo_epi64(w6, w14);
  d[5] = _mm_unpackhi_epi64(w6, w14);
  d[6] = _mm_unpacklo_epi64(w7, w15);
  d[7] = _mm_unpackhi_epi64(w7, w15);

  // upper half
  w0 = _mm_unpackhi_epi8(x[0], x[1]);
  w1 = _mm_unpackhi_epi8(x[2], x[3]);
  w2 = _mm_unpackhi_epi8(x[4], x[5]);
  w3 = _mm_unpackhi_epi8(x[6], x[7]);

  w8 = _mm_unpackhi_epi8(x[8], x[9]);
  w9 = _mm_unpackhi_epi8(x[10], x[11]);
  w10 = _mm_unpackhi_epi8(x[12], x[13]);
  w11 = _mm_unpackhi_epi8(x[14], x[15]);

  w4 = _mm_unpacklo_epi16(w0, w1);
  w5 = _mm_unpacklo_epi16(w2, w3);
  w12 = _mm_unpacklo_epi16(w8, w9);
  w13 = _mm_unpacklo_epi16(w10, w11);

  w6 = _mm_unpacklo_epi32(w4, w5);
  w7 = _mm_unpackhi_epi32(w4, w5);
  w14 = _mm_unpacklo_epi32(w12, w13);
  w15 = _mm_unpackhi_epi32(w12, w13);

  // Store first 4-line result
  d[8] = _mm_unpacklo_epi64(w6, w14);
  d[9] = _mm_unpackhi_epi64(w6, w14);
  d[10] = _mm_unpacklo_epi64(w7, w15);
  d[11] = _mm_unpackhi_epi64(w7, w15);

  w4 = _mm_unpackhi_epi16(w0, w1);
  w5 = _mm_unpackhi_epi16(w2, w3);
  w12 = _mm_unpackhi_epi16(w8, w9);
  w13 = _mm_unpackhi_epi16(w10, w11);

  w6 = _mm_unpacklo_epi32(w4, w5);
  w7 = _mm_unpackhi_epi32(w4, w5);
  w14 = _mm_unpacklo_epi32(w12, w13);
  w15 = _mm_unpackhi_epi32(w12, w13);

  // Store second 4-line result
  d[12] = _mm_unpacklo_epi64(w6, w14);
  d[13] = _mm_unpackhi_epi64(w6, w14);
  d[14] = _mm_unpacklo_epi64(w7, w15);
  d[15] = _mm_unpackhi_epi64(w7, w15);
}

static void transpose_TX_16X16(const uint8_t *src, ptrdiff_t pitchSrc,
                               uint8_t *dst, ptrdiff_t pitchDst) {
  __m128i r[16];
  __m128i d[16];
  for (int j = 0; j < 16; j++) {
    r[j] = _mm_loadu_si128((__m128i *)(src + j * pitchSrc));
  }
  transpose16x16_sse2(r, d);
  for (int j = 0; j < 16; j++) {
    _mm_storeu_si128((__m128i *)(dst + j * pitchDst), d[j]);
  }
}

static void transpose(const uint8_t *src, ptrdiff_t pitchSrc, uint8_t *dst,
                      ptrdiff_t pitchDst, int width, int height) {
  for (int j = 0; j < height; j += 16)
    for (int i = 0; i < width; i += 16)
      transpose_TX_16X16(src + i * pitchSrc + j, pitchSrc,
                         dst + j * pitchDst + i, pitchDst);
}

#endif  // AOM_AOM_DSP_X86_INTRAPRED_UTILS_H_
