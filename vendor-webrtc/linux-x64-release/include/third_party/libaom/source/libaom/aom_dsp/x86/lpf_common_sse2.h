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

#ifndef AOM_AOM_DSP_X86_LPF_COMMON_SSE2_H_
#define AOM_AOM_DSP_X86_LPF_COMMON_SSE2_H_

#include <emmintrin.h>  // SSE2

#include "config/aom_config.h"

#define mm_storelu(dst, v) memcpy((dst), (const char *)&(v), 8)
#define mm_storehu(dst, v) memcpy((dst), (const char *)&(v) + 8, 8)

static inline void highbd_transpose6x6_sse2(__m128i *x0, __m128i *x1,
                                            __m128i *x2, __m128i *x3,
                                            __m128i *x4, __m128i *x5,
                                            __m128i *d0, __m128i *d1,
                                            __m128i *d2, __m128i *d3,
                                            __m128i *d4, __m128i *d5) {
  __m128i w0, w1, w2, w3, w4, w5, ww0;

  // 00 01 02 03 04 05 xx xx
  // 10 11 12 13 14 15 xx xx
  // 20 21 22 23 24 25 xx xx
  // 30 31 32 33 34 35 xx xx
  // 40 41 42 43 44 45 xx xx
  // 50 51 52 53 54 55 xx xx

  w0 = _mm_unpacklo_epi16(*x0, *x1);  // 00 10 01 11 02 12 03 13
  w1 = _mm_unpacklo_epi16(*x2, *x3);  // 20 30 21 31 22 32 23 33
  w2 = _mm_unpacklo_epi16(*x4, *x5);  // 40 50 41 51 42 52 43 53

  ww0 = _mm_unpacklo_epi32(w0, w1);   // 00 10 20 30 01 11 21 31
  *d0 = _mm_unpacklo_epi64(ww0, w2);  // 00 10 20 30 40 50 41 51
  *d1 = _mm_unpackhi_epi64(ww0,
                           _mm_srli_si128(w2, 4));  // 01 11 21 31 41 51 xx xx

  ww0 = _mm_unpackhi_epi32(w0, w1);  // 02 12 22 32 03 13 23 33
  *d2 = _mm_unpacklo_epi64(ww0,
                           _mm_srli_si128(w2, 8));  // 02 12 22 32 42 52 xx xx

  w3 = _mm_unpackhi_epi16(*x0, *x1);  // 04 14 05 15 xx xx xx xx
  w4 = _mm_unpackhi_epi16(*x2, *x3);  // 24 34 25 35 xx xx xx xx
  w5 = _mm_unpackhi_epi16(*x4, *x5);  // 44 54 45 55 xx xx xx xx

  *d3 = _mm_unpackhi_epi64(ww0, _mm_srli_si128(w2, 4));  // 03 13 23 33 43 53

  ww0 = _mm_unpacklo_epi32(w3, w4);   //  04 14 24 34 05 15 25 35
  *d4 = _mm_unpacklo_epi64(ww0, w5);  //  04 14 24 34 44 54 45 55
  *d5 = _mm_unpackhi_epi64(ww0,
                           _mm_slli_si128(w5, 4));  // 05 15 25 35 45 55 xx xx
}

static inline void highbd_transpose4x8_8x4_low_sse2(__m128i *x0, __m128i *x1,
                                                    __m128i *x2, __m128i *x3,
                                                    __m128i *d0, __m128i *d1,
                                                    __m128i *d2, __m128i *d3) {
  __m128i zero = _mm_setzero_si128();
  __m128i w0, w1, ww0, ww1;

  w0 = _mm_unpacklo_epi16(*x0, *x1);  // 00 10 01 11 02 12 03 13
  w1 = _mm_unpacklo_epi16(*x2, *x3);  // 20 30 21 31 22 32 23 33

  ww0 = _mm_unpacklo_epi32(w0, w1);  // 00 10 20 30 01 11 21 31
  ww1 = _mm_unpackhi_epi32(w0, w1);  // 02 12 22 32 03 13 23 33

  *d0 = _mm_unpacklo_epi64(ww0, zero);  // 00 10 20 30 xx xx xx xx
  *d1 = _mm_unpackhi_epi64(ww0, zero);  // 01 11 21 31 xx xx xx xx
  *d2 = _mm_unpacklo_epi64(ww1, zero);  // 02 12 22 32 xx xx xx xx
  *d3 = _mm_unpackhi_epi64(ww1, zero);  // 03 13 23 33 xx xx xx xx
}

static inline void highbd_transpose4x8_8x4_high_sse2(__m128i *x0, __m128i *x1,
                                                     __m128i *x2, __m128i *x3,
                                                     __m128i *d4, __m128i *d5,
                                                     __m128i *d6, __m128i *d7) {
  __m128i w0, w1, ww2, ww3;
  __m128i zero = _mm_setzero_si128();

  w0 = _mm_unpackhi_epi16(*x0, *x1);  // 04 14 05 15 06 16 07 17
  w1 = _mm_unpackhi_epi16(*x2, *x3);  // 24 34 25 35 26 36 27 37

  ww2 = _mm_unpacklo_epi32(w0, w1);  //  04 14 24 34 05 15 25 35
  ww3 = _mm_unpackhi_epi32(w0, w1);  //  06 16 26 36 07 17 27 37

  *d4 = _mm_unpacklo_epi64(ww2, zero);  // 04 14 24 34 xx xx xx xx
  *d5 = _mm_unpackhi_epi64(ww2, zero);  // 05 15 25 35 xx xx xx xx
  *d6 = _mm_unpacklo_epi64(ww3, zero);  // 06 16 26 36 xx xx xx xx
  *d7 = _mm_unpackhi_epi64(ww3, zero);  // 07 17 27 37 xx xx xx xx
}

// here in and out pointers (x and d) should be different! we don't store their
// values inside
static inline void highbd_transpose4x8_8x4_sse2(__m128i *x0, __m128i *x1,
                                                __m128i *x2, __m128i *x3,
                                                __m128i *d0, __m128i *d1,
                                                __m128i *d2, __m128i *d3,
                                                __m128i *d4, __m128i *d5,
                                                __m128i *d6, __m128i *d7) {
  // input
  // x0 00 01 02 03 04 05 06 07
  // x1 10 11 12 13 14 15 16 17
  // x2 20 21 22 23 24 25 26 27
  // x3 30 31 32 33 34 35 36 37
  // output
  // 00 10 20 30 xx xx xx xx
  // 01 11 21 31 xx xx xx xx
  // 02 12 22 32 xx xx xx xx
  // 03 13 23 33 xx xx xx xx
  // 04 14 24 34 xx xx xx xx
  // 05 15 25 35 xx xx xx xx
  // 06 16 26 36 xx xx xx xx
  // 07 17 27 37 xx xx xx xx
  highbd_transpose4x8_8x4_low_sse2(x0, x1, x2, x3, d0, d1, d2, d3);
  highbd_transpose4x8_8x4_high_sse2(x0, x1, x2, x3, d4, d5, d6, d7);
}

static inline void highbd_transpose8x8_low_sse2(__m128i *x0, __m128i *x1,
                                                __m128i *x2, __m128i *x3,
                                                __m128i *x4, __m128i *x5,
                                                __m128i *x6, __m128i *x7,
                                                __m128i *d0, __m128i *d1,
                                                __m128i *d2, __m128i *d3) {
  __m128i w0, w1, w2, w3, ww0, ww1;
  // x0 00 01 02 03 04 05 06 07
  // x1 10 11 12 13 14 15 16 17
  // x2 20 21 22 23 24 25 26 27
  // x3 30 31 32 33 34 35 36 37
  // x4 40 41 42 43 44 45 46 47
  // x5 50 51 52 53 54 55 56 57
  // x6 60 61 62 63 64 65 66 67
  // x7 70 71 72 73 74 75 76 77

  w0 = _mm_unpacklo_epi16(*x0, *x1);  // 00 10 01 11 02 12 03 13
  w1 = _mm_unpacklo_epi16(*x2, *x3);  // 20 30 21 31 22 32 23 33
  w2 = _mm_unpacklo_epi16(*x4, *x5);  // 40 50 41 51 42 52 43 53
  w3 = _mm_unpacklo_epi16(*x6, *x7);  // 60 70 61 71 62 72 63 73

  ww0 = _mm_unpacklo_epi32(w0, w1);  // 00 10 20 30 01 11 21 31
  ww1 = _mm_unpacklo_epi32(w2, w3);  // 40 50 60 70 41 51 61 71

  *d0 = _mm_unpacklo_epi64(ww0, ww1);  // 00 10 20 30 40 50 60 70
  *d1 = _mm_unpackhi_epi64(ww0, ww1);  // 01 11 21 31 41 51 61 71

  ww0 = _mm_unpackhi_epi32(w0, w1);  // 02 12 22 32 03 13 23 33
  ww1 = _mm_unpackhi_epi32(w2, w3);  // 42 52 62 72 43 53 63 73

  *d2 = _mm_unpacklo_epi64(ww0, ww1);  // 02 12 22 32 42 52 62 72
  *d3 = _mm_unpackhi_epi64(ww0, ww1);  // 03 13 23 33 43 53 63 73
}

static inline void highbd_transpose8x8_high_sse2(__m128i *x0, __m128i *x1,
                                                 __m128i *x2, __m128i *x3,
                                                 __m128i *x4, __m128i *x5,
                                                 __m128i *x6, __m128i *x7,
                                                 __m128i *d4, __m128i *d5,
                                                 __m128i *d6, __m128i *d7) {
  __m128i w0, w1, w2, w3, ww0, ww1;
  // x0 00 01 02 03 04 05 06 07
  // x1 10 11 12 13 14 15 16 17
  // x2 20 21 22 23 24 25 26 27
  // x3 30 31 32 33 34 35 36 37
  // x4 40 41 42 43 44 45 46 47
  // x5 50 51 52 53 54 55 56 57
  // x6 60 61 62 63 64 65 66 67
  // x7 70 71 72 73 74 75 76 77
  w0 = _mm_unpackhi_epi16(*x0, *x1);  // 04 14 05 15 06 16 07 17
  w1 = _mm_unpackhi_epi16(*x2, *x3);  // 24 34 25 35 26 36 27 37
  w2 = _mm_unpackhi_epi16(*x4, *x5);  // 44 54 45 55 46 56 47 57
  w3 = _mm_unpackhi_epi16(*x6, *x7);  // 64 74 65 75 66 76 67 77

  ww0 = _mm_unpacklo_epi32(w0, w1);  // 04 14 24 34 05 15 25 35
  ww1 = _mm_unpacklo_epi32(w2, w3);  // 44 54 64 74 45 55 65 75

  *d4 = _mm_unpacklo_epi64(ww0, ww1);  // 04 14 24 34 44 54 64 74
  *d5 = _mm_unpackhi_epi64(ww0, ww1);  // 05 15 25 35 45 55 65 75

  ww0 = _mm_unpackhi_epi32(w0, w1);  // 06 16 26 36 07 17 27 37
  ww1 = _mm_unpackhi_epi32(w2, w3);  // 46 56 66 76 47 57 67 77

  *d6 = _mm_unpacklo_epi64(ww0, ww1);  // 06 16 26 36 46 56 66 76
  *d7 = _mm_unpackhi_epi64(ww0, ww1);  // 07 17 27 37 47 57 67 77
}

// here in and out pointers (x and d) should be different! we don't store their
// values inside
static inline void highbd_transpose8x8_sse2(
    __m128i *x0, __m128i *x1, __m128i *x2, __m128i *x3, __m128i *x4,
    __m128i *x5, __m128i *x6, __m128i *x7, __m128i *d0, __m128i *d1,
    __m128i *d2, __m128i *d3, __m128i *d4, __m128i *d5, __m128i *d6,
    __m128i *d7) {
  highbd_transpose8x8_low_sse2(x0, x1, x2, x3, x4, x5, x6, x7, d0, d1, d2, d3);
  highbd_transpose8x8_high_sse2(x0, x1, x2, x3, x4, x5, x6, x7, d4, d5, d6, d7);
}

// here in and out pointers (x and d arrays) should be different! we don't store
// their values inside
static inline void highbd_transpose8x16_sse2(
    __m128i *x0, __m128i *x1, __m128i *x2, __m128i *x3, __m128i *x4,
    __m128i *x5, __m128i *x6, __m128i *x7, __m128i *d0, __m128i *d1,
    __m128i *d2, __m128i *d3, __m128i *d4, __m128i *d5, __m128i *d6,
    __m128i *d7) {
  highbd_transpose8x8_sse2(x0, x1, x2, x3, x4, x5, x6, x7, d0, d1, d2, d3, d4,
                           d5, d6, d7);
  highbd_transpose8x8_sse2(x0 + 1, x1 + 1, x2 + 1, x3 + 1, x4 + 1, x5 + 1,
                           x6 + 1, x7 + 1, d0 + 1, d1 + 1, d2 + 1, d3 + 1,
                           d4 + 1, d5 + 1, d6 + 1, d7 + 1);
}

// Low bit depth functions
static inline void transpose4x8_8x4_low_sse2(__m128i *x0, __m128i *x1,
                                             __m128i *x2, __m128i *x3,
                                             __m128i *d0, __m128i *d1,
                                             __m128i *d2, __m128i *d3) {
  // input
  // x0   00 01 02 03 04 05 06 07 xx xx xx xx xx xx xx xx
  // x1   10 11 12 13 14 15 16 17 xx xx xx xx xx xx xx xx
  // x2   20 21 22 23 24 25 26 27 xx xx xx xx xx xx xx xx
  // x3   30 31 32 33 34 35 36 37 xx xx xx xx xx xx xx xx
  // output
  // 00 10 20 30 xx xx xx xx xx xx xx xx xx xx xx xx
  // 01 11 21 31 xx xx xx xx xx xx xx xx xx xx xx xx
  // 02 12 22 32 xx xx xx xx xx xx xx xx xx xx xx xx
  // 03 13 23 33 xx xx xx xx xx xx xx xx xx xx xx xx

  __m128i w0, w1;

  w0 = _mm_unpacklo_epi8(
      *x0, *x1);  // 00 10 01 11 02 12 03 13 04 14 05 15 06 16 07 17
  w1 = _mm_unpacklo_epi8(
      *x2, *x3);  // 20 30 21 31 22 32 23 33 24 34 25 35 26 36 27 37

  *d0 = _mm_unpacklo_epi16(
      w0, w1);  // 00 10 20 30 01 11 21 31 02 12 22 32 03 13 23 33

  *d1 = _mm_srli_si128(*d0,
                       4);  // 01 11 21 31 xx xx xx xx xx xx xx xx xx xx xx xx
  *d2 = _mm_srli_si128(*d0,
                       8);  // 02 12 22 32 xx xx xx xx xx xx xx xx xx xx xx xx
  *d3 = _mm_srli_si128(*d0,
                       12);  // 03 13 23 33 xx xx xx xx xx xx xx xx xx xx xx xx
}

static inline void transpose4x8_8x4_sse2(__m128i *x0, __m128i *x1, __m128i *x2,
                                         __m128i *x3, __m128i *d0, __m128i *d1,
                                         __m128i *d2, __m128i *d3, __m128i *d4,
                                         __m128i *d5, __m128i *d6,
                                         __m128i *d7) {
  // input
  // x0   00 01 02 03 04 05 06 07 xx xx xx xx xx xx xx xx
  // x1   10 11 12 13 14 15 16 17 xx xx xx xx xx xx xx xx
  // x2   20 21 22 23 24 25 26 27 xx xx xx xx xx xx xx xx
  // x3   30 31 32 33 34 35 36 37 xx xx xx xx xx xx xx xx
  // output
  // 00 10 20 30 xx xx xx xx xx xx xx xx xx xx xx xx
  // 01 11 21 31 xx xx xx xx xx xx xx xx xx xx xx xx
  // 02 12 22 32 xx xx xx xx xx xx xx xx xx xx xx xx
  // 03 13 23 33 xx xx xx xx xx xx xx xx xx xx xx xx
  // 04 14 24 34 xx xx xx xx xx xx xx xx xx xx xx xx
  // 05 15 25 35 xx xx xx xx xx xx xx xx xx xx xx xx
  // 06 16 26 36 xx xx xx xx xx xx xx xx xx xx xx xx
  // 07 17 27 37 xx xx xx xx xx xx xx xx xx xx xx xx

  __m128i w0, w1, ww0, ww1;

  w0 = _mm_unpacklo_epi8(
      *x0, *x1);  // 00 10 01 11 02 12 03 13 04 14 05 15 06 16 07 17
  w1 = _mm_unpacklo_epi8(
      *x2, *x3);  // 20 30 21 31 22 32 23 33 24 34 25 35 26 36 27 37

  ww0 = _mm_unpacklo_epi16(
      w0, w1);  // 00 10 20 30 01 11 21 31 02 12 22 32 03 13 23 33
  ww1 = _mm_unpackhi_epi16(
      w0, w1);  // 04 14 24 34 05 15 25 35 06 16 26 36 07 17 27 37

  *d0 = ww0;  // 00 10 20 30 xx xx xx xx xx xx xx xx xx xx xx xx
  *d1 = _mm_srli_si128(ww0,
                       4);  // 01 11 21 31 xx xx xx xx xx xx xx xx xx xx xx xx
  *d2 = _mm_srli_si128(ww0,
                       8);  // 02 12 22 32 xx xx xx xx xx xx xx xx xx xx xx xx
  *d3 = _mm_srli_si128(ww0,
                       12);  // 03 13 23 33 xx xx xx xx xx xx xx xx xx xx xx xx

  *d4 = ww1;  // 04 14 24 34 xx xx xx xx xx xx xx xx xx xx xx xx
  *d5 = _mm_srli_si128(ww1,
                       4);  // 05 15 25 35 xx xx xx xx xx xx xx xx xx xx xx xx
  *d6 = _mm_srli_si128(ww1,
                       8);  // 06 16 26 36 xx xx xx xx xx xx xx xx xx xx xx xx
  *d7 = _mm_srli_si128(ww1,
                       12);  // 07 17 27 37 xx xx xx xx xx xx xx xx xx xx xx xx
}

static inline void transpose8x8_low_sse2(__m128i *x0, __m128i *x1, __m128i *x2,
                                         __m128i *x3, __m128i *x4, __m128i *x5,
                                         __m128i *x6, __m128i *x7, __m128i *d0,
                                         __m128i *d1, __m128i *d2,
                                         __m128i *d3) {
  // input
  // x0 00 01 02 03 04 05 06 07
  // x1 10 11 12 13 14 15 16 17
  // x2 20 21 22 23 24 25 26 27
  // x3 30 31 32 33 34 35 36 37
  // x4 40 41 42 43 44 45 46 47
  // x5  50 51 52 53 54 55 56 57
  // x6  60 61 62 63 64 65 66 67
  // x7 70 71 72 73 74 75 76 77
  // output
  // d0 00 10 20 30 40 50 60 70 xx xx xx xx xx xx xx
  // d1 01 11 21 31 41 51 61 71 xx xx xx xx xx xx xx xx
  // d2 02 12 22 32 42 52 62 72 xx xx xx xx xx xx xx xx
  // d3 03 13 23 33 43 53 63 73 xx xx xx xx xx xx xx xx

  __m128i w0, w1, w2, w3, w4, w5;

  w0 = _mm_unpacklo_epi8(
      *x0, *x1);  // 00 10 01 11 02 12 03 13 04 14 05 15 06 16 07 17

  w1 = _mm_unpacklo_epi8(
      *x2, *x3);  // 20 30 21 31 22 32 23 33 24 34 25 35 26 36 27 37

  w2 = _mm_unpacklo_epi8(
      *x4, *x5);  // 40 50 41 51 42 52 43 53 44 54 45 55 46 56 47 57

  w3 = _mm_unpacklo_epi8(
      *x6, *x7);  // 60 70 61 71 62 72 63 73 64 74 65 75 66 76 67 77

  w4 = _mm_unpacklo_epi16(
      w0, w1);  // 00 10 20 30 01 11 21 31 02 12 22 32 03 13 23 33
  w5 = _mm_unpacklo_epi16(
      w2, w3);  // 40 50 60 70 41 51 61 71 42 52 62 72 43 53 63 73

  *d0 = _mm_unpacklo_epi32(
      w4, w5);  // 00 10 20 30 40 50 60 70 01 11 21 31 41 51 61 71
  *d1 = _mm_srli_si128(*d0, 8);
  *d2 = _mm_unpackhi_epi32(
      w4, w5);  // 02 12 22 32 42 52 62 72 03 13 23 33 43 53 63 73
  *d3 = _mm_srli_si128(*d2, 8);
}

static inline void transpose8x8_sse2(__m128i *x0, __m128i *x1, __m128i *x2,
                                     __m128i *x3, __m128i *x4, __m128i *x5,
                                     __m128i *x6, __m128i *x7, __m128i *d0d1,
                                     __m128i *d2d3, __m128i *d4d5,
                                     __m128i *d6d7) {
  __m128i w0, w1, w2, w3, w4, w5, w6, w7;
  // x0 00 01 02 03 04 05 06 07
  // x1 10 11 12 13 14 15 16 17
  w0 = _mm_unpacklo_epi8(
      *x0, *x1);  // 00 10 01 11 02 12 03 13 04 14 05 15 06 16 07 17

  // x2 20 21 22 23 24 25 26 27
  // x3 30 31 32 33 34 35 36 37
  w1 = _mm_unpacklo_epi8(
      *x2, *x3);  // 20 30 21 31 22 32 23 33 24 34 25 35 26 36 27 37

  // x4 40 41 42 43 44 45 46 47
  // x5  50 51 52 53 54 55 56 57
  w2 = _mm_unpacklo_epi8(
      *x4, *x5);  // 40 50 41 51 42 52 43 53 44 54 45 55 46 56 47 57

  // x6  60 61 62 63 64 65 66 67
  // x7 70 71 72 73 74 75 76 77
  w3 = _mm_unpacklo_epi8(
      *x6, *x7);  // 60 70 61 71 62 72 63 73 64 74 65 75 66 76 67 77

  w4 = _mm_unpacklo_epi16(
      w0, w1);  // 00 10 20 30 01 11 21 31 02 12 22 32 03 13 23 33
  w5 = _mm_unpacklo_epi16(
      w2, w3);  // 40 50 60 70 41 51 61 71 42 52 62 72 43 53 63 73

  *d0d1 = _mm_unpacklo_epi32(
      w4, w5);  // 00 10 20 30 40 50 60 70 01 11 21 31 41 51 61 71
  *d2d3 = _mm_unpackhi_epi32(
      w4, w5);  // 02 12 22 32 42 52 62 72 03 13 23 33 43 53 63 73

  w6 = _mm_unpackhi_epi16(
      w0, w1);  // 04 14 24 34 05 15 25 35 06 16 26 36 07 17 27 37
  w7 = _mm_unpackhi_epi16(
      w2, w3);  // 44 54 64 74 45 55 65 75 46 56 66 76 47 57 67 77

  *d4d5 = _mm_unpacklo_epi32(
      w6, w7);  // 04 14 24 34 44 54 64 74 05 15 25 35 45 55 65 75
  *d6d7 = _mm_unpackhi_epi32(
      w6, w7);  // 06 16 26 36 46 56 66 76 07 17 27 37 47 57 67 77
}

static inline void transpose16x8_8x16_sse2(
    __m128i *x0, __m128i *x1, __m128i *x2, __m128i *x3, __m128i *x4,
    __m128i *x5, __m128i *x6, __m128i *x7, __m128i *x8, __m128i *x9,
    __m128i *x10, __m128i *x11, __m128i *x12, __m128i *x13, __m128i *x14,
    __m128i *x15, __m128i *d0, __m128i *d1, __m128i *d2, __m128i *d3,
    __m128i *d4, __m128i *d5, __m128i *d6, __m128i *d7) {
  __m128i w0, w1, w2, w3, w4, w5, w6, w7, w8, w9;
  __m128i w10, w11, w12, w13, w14, w15;

  w0 = _mm_unpacklo_epi8(*x0, *x1);
  w1 = _mm_unpacklo_epi8(*x2, *x3);
  w2 = _mm_unpacklo_epi8(*x4, *x5);
  w3 = _mm_unpacklo_epi8(*x6, *x7);

  w8 = _mm_unpacklo_epi8(*x8, *x9);
  w9 = _mm_unpacklo_epi8(*x10, *x11);
  w10 = _mm_unpacklo_epi8(*x12, *x13);
  w11 = _mm_unpacklo_epi8(*x14, *x15);

  w4 = _mm_unpacklo_epi16(w0, w1);
  w5 = _mm_unpacklo_epi16(w2, w3);
  w12 = _mm_unpacklo_epi16(w8, w9);
  w13 = _mm_unpacklo_epi16(w10, w11);

  w6 = _mm_unpacklo_epi32(w4, w5);
  w7 = _mm_unpackhi_epi32(w4, w5);
  w14 = _mm_unpacklo_epi32(w12, w13);
  w15 = _mm_unpackhi_epi32(w12, w13);

  // Store first 4-line result
  *d0 = _mm_unpacklo_epi64(w6, w14);
  *d1 = _mm_unpackhi_epi64(w6, w14);
  *d2 = _mm_unpacklo_epi64(w7, w15);
  *d3 = _mm_unpackhi_epi64(w7, w15);

  w4 = _mm_unpackhi_epi16(w0, w1);
  w5 = _mm_unpackhi_epi16(w2, w3);
  w12 = _mm_unpackhi_epi16(w8, w9);
  w13 = _mm_unpackhi_epi16(w10, w11);

  w6 = _mm_unpacklo_epi32(w4, w5);
  w7 = _mm_unpackhi_epi32(w4, w5);
  w14 = _mm_unpacklo_epi32(w12, w13);
  w15 = _mm_unpackhi_epi32(w12, w13);

  // Store second 4-line result
  *d4 = _mm_unpacklo_epi64(w6, w14);
  *d5 = _mm_unpackhi_epi64(w6, w14);
  *d6 = _mm_unpacklo_epi64(w7, w15);
  *d7 = _mm_unpackhi_epi64(w7, w15);
}

static inline void transpose8x16_16x8_sse2(
    __m128i *x0, __m128i *x1, __m128i *x2, __m128i *x3, __m128i *x4,
    __m128i *x5, __m128i *x6, __m128i *x7, __m128i *d0d1, __m128i *d2d3,
    __m128i *d4d5, __m128i *d6d7, __m128i *d8d9, __m128i *d10d11,
    __m128i *d12d13, __m128i *d14d15) {
  __m128i w0, w1, w2, w3, w4, w5, w6, w7, w8, w9;
  __m128i w10, w11, w12, w13, w14, w15;

  w0 = _mm_unpacklo_epi8(*x0, *x1);
  w1 = _mm_unpacklo_epi8(*x2, *x3);
  w2 = _mm_unpacklo_epi8(*x4, *x5);
  w3 = _mm_unpacklo_epi8(*x6, *x7);

  w8 = _mm_unpackhi_epi8(*x0, *x1);
  w9 = _mm_unpackhi_epi8(*x2, *x3);
  w10 = _mm_unpackhi_epi8(*x4, *x5);
  w11 = _mm_unpackhi_epi8(*x6, *x7);

  w4 = _mm_unpacklo_epi16(w0, w1);
  w5 = _mm_unpacklo_epi16(w2, w3);
  w12 = _mm_unpacklo_epi16(w8, w9);
  w13 = _mm_unpacklo_epi16(w10, w11);

  w6 = _mm_unpacklo_epi32(w4, w5);
  w7 = _mm_unpackhi_epi32(w4, w5);
  w14 = _mm_unpacklo_epi32(w12, w13);
  w15 = _mm_unpackhi_epi32(w12, w13);

  // Store first 4-line result
  *d0d1 = _mm_unpacklo_epi64(w6, w14);
  *d2d3 = _mm_unpackhi_epi64(w6, w14);
  *d4d5 = _mm_unpacklo_epi64(w7, w15);
  *d6d7 = _mm_unpackhi_epi64(w7, w15);

  w4 = _mm_unpackhi_epi16(w0, w1);
  w5 = _mm_unpackhi_epi16(w2, w3);
  w12 = _mm_unpackhi_epi16(w8, w9);
  w13 = _mm_unpackhi_epi16(w10, w11);

  w6 = _mm_unpacklo_epi32(w4, w5);
  w7 = _mm_unpackhi_epi32(w4, w5);
  w14 = _mm_unpacklo_epi32(w12, w13);
  w15 = _mm_unpackhi_epi32(w12, w13);

  // Store second 4-line result
  *d8d9 = _mm_unpacklo_epi64(w6, w14);
  *d10d11 = _mm_unpackhi_epi64(w6, w14);
  *d12d13 = _mm_unpacklo_epi64(w7, w15);
  *d14d15 = _mm_unpackhi_epi64(w7, w15);
}

static inline void transpose_16x8(unsigned char *in0, unsigned char *in1,
                                  int in_p, unsigned char *out, int out_p) {
  __m128i x0, x1, x2, x3, x4, x5, x6, x7;
  __m128i x8, x9, x10, x11, x12, x13, x14, x15;

  x0 = _mm_loadl_epi64((__m128i *)in0);
  x1 = _mm_loadl_epi64((__m128i *)(in0 + in_p));
  x0 = _mm_unpacklo_epi8(x0, x1);

  x2 = _mm_loadl_epi64((__m128i *)(in0 + 2 * in_p));
  x3 = _mm_loadl_epi64((__m128i *)(in0 + 3 * in_p));
  x1 = _mm_unpacklo_epi8(x2, x3);

  x4 = _mm_loadl_epi64((__m128i *)(in0 + 4 * in_p));
  x5 = _mm_loadl_epi64((__m128i *)(in0 + 5 * in_p));
  x2 = _mm_unpacklo_epi8(x4, x5);

  x6 = _mm_loadl_epi64((__m128i *)(in0 + 6 * in_p));
  x7 = _mm_loadl_epi64((__m128i *)(in0 + 7 * in_p));
  x3 = _mm_unpacklo_epi8(x6, x7);
  x4 = _mm_unpacklo_epi16(x0, x1);

  x8 = _mm_loadl_epi64((__m128i *)in1);
  x9 = _mm_loadl_epi64((__m128i *)(in1 + in_p));
  x8 = _mm_unpacklo_epi8(x8, x9);
  x5 = _mm_unpacklo_epi16(x2, x3);

  x10 = _mm_loadl_epi64((__m128i *)(in1 + 2 * in_p));
  x11 = _mm_loadl_epi64((__m128i *)(in1 + 3 * in_p));
  x9 = _mm_unpacklo_epi8(x10, x11);

  x12 = _mm_loadl_epi64((__m128i *)(in1 + 4 * in_p));
  x13 = _mm_loadl_epi64((__m128i *)(in1 + 5 * in_p));
  x10 = _mm_unpacklo_epi8(x12, x13);
  x12 = _mm_unpacklo_epi16(x8, x9);

  x14 = _mm_loadl_epi64((__m128i *)(in1 + 6 * in_p));
  x15 = _mm_loadl_epi64((__m128i *)(in1 + 7 * in_p));
  x11 = _mm_unpacklo_epi8(x14, x15);
  x13 = _mm_unpacklo_epi16(x10, x11);

  x6 = _mm_unpacklo_epi32(x4, x5);
  x7 = _mm_unpackhi_epi32(x4, x5);
  x14 = _mm_unpacklo_epi32(x12, x13);
  x15 = _mm_unpackhi_epi32(x12, x13);

  // Store first 4-line result
  _mm_storeu_si128((__m128i *)out, _mm_unpacklo_epi64(x6, x14));
  _mm_storeu_si128((__m128i *)(out + out_p), _mm_unpackhi_epi64(x6, x14));
  _mm_storeu_si128((__m128i *)(out + 2 * out_p), _mm_unpacklo_epi64(x7, x15));
  _mm_storeu_si128((__m128i *)(out + 3 * out_p), _mm_unpackhi_epi64(x7, x15));

  x4 = _mm_unpackhi_epi16(x0, x1);
  x5 = _mm_unpackhi_epi16(x2, x3);
  x12 = _mm_unpackhi_epi16(x8, x9);
  x13 = _mm_unpackhi_epi16(x10, x11);

  x6 = _mm_unpacklo_epi32(x4, x5);
  x7 = _mm_unpackhi_epi32(x4, x5);
  x14 = _mm_unpacklo_epi32(x12, x13);
  x15 = _mm_unpackhi_epi32(x12, x13);

  // Store second 4-line result
  _mm_storeu_si128((__m128i *)(out + 4 * out_p), _mm_unpacklo_epi64(x6, x14));
  _mm_storeu_si128((__m128i *)(out + 5 * out_p), _mm_unpackhi_epi64(x6, x14));
  _mm_storeu_si128((__m128i *)(out + 6 * out_p), _mm_unpacklo_epi64(x7, x15));
  _mm_storeu_si128((__m128i *)(out + 7 * out_p), _mm_unpackhi_epi64(x7, x15));
}

static inline void transpose_16x8_to_8x16(unsigned char *src, int in_p,
                                          unsigned char *dst, int out_p) {
  // a0 b0 c0 d0 e0 f0 g0 h0 A0 B0 C0 D0 E0 F0 G0 H0
  // a1 b1 c1 d1 e1 f1 g1 h1 A1 B1 C1 D1 E1 F1 G1 H1
  // a2 b2 c2 d2 e2 f2 g2 h2 A2 B2 C2 D2 E2 F2 G2 H2
  // a3 b3 c3 d3 e3 f3 g3 h3 A3 B3 C3 D3 E3 F3 G3 H3
  // a4 b4 c4 d4 e4 f4 g4 h4 A4 B4 C4 D4 E4 F4 G4 H4
  // a5 b5 c5 d5 e5 f5 g5 h5 A5 B5 C5 D5 E5 F5 G5 H5
  // a6 b6 c6 d6 e6 f6 g6 h6 A6 B6 C6 D6 E6 F6 G6 H6
  // a7 b7 c7 d7 e7 f7 g7 h7 A7 B7 C7 D7 E7 F7 G7 H7
  const __m128i x0 = _mm_loadu_si128((__m128i *)(src));
  const __m128i x1 = _mm_loadu_si128((__m128i *)(src + (1 * in_p)));
  const __m128i x2 = _mm_loadu_si128((__m128i *)(src + (2 * in_p)));
  const __m128i x3 = _mm_loadu_si128((__m128i *)(src + (3 * in_p)));
  const __m128i x4 = _mm_loadu_si128((__m128i *)(src + (4 * in_p)));
  const __m128i x5 = _mm_loadu_si128((__m128i *)(src + (5 * in_p)));
  const __m128i x6 = _mm_loadu_si128((__m128i *)(src + (6 * in_p)));
  const __m128i x7 = _mm_loadu_si128((__m128i *)(src + (7 * in_p)));

  // a0 a1 b0 b1 c0 c1 d0 d1 A0 A1 B0 B1 C0 C1 D0 D1
  // e0 e1 f0 f1 g0 g1 h0 h1 E0 E1 F0 F1 G0 G1 H0 H1
  // a2 a3 b2 b3 c2 c3 d2 d3 A2 A3 B2 B3 C2 C3 D2 D3
  // e2 e3 f2 f3 g2 g3 h2 h3 E2 E3 F2 F3 G2 G3 H2 H3
  // a4 a5 b4 b5 c4 c5 d4 d5 A4 A5 B4 B5 C4 C5 D4 D5
  // e4 e5 f4 f5 g4 g5 h4 h5 E4 E5 F4 F5 G4 G5 H4 H5
  // a6 a7 b6 b7 c6 c7 d6 d7 A6 A7 B6 B7 C6 C7 D6 D7
  // e6 e7 f6 f7 g6 g7 h6 h7 E6 E7 F6 F7 G6 G7 H6 H7
  const __m128i x_s10 = _mm_unpacklo_epi8(x0, x1);
  const __m128i x_s11 = _mm_unpackhi_epi8(x0, x1);
  const __m128i x_s12 = _mm_unpacklo_epi8(x2, x3);
  const __m128i x_s13 = _mm_unpackhi_epi8(x2, x3);
  const __m128i x_s14 = _mm_unpacklo_epi8(x4, x5);
  const __m128i x_s15 = _mm_unpackhi_epi8(x4, x5);
  const __m128i x_s16 = _mm_unpacklo_epi8(x6, x7);
  const __m128i x_s17 = _mm_unpackhi_epi8(x6, x7);

  // a0 a1 a2 a3 b0 b1 b2 b3 | A0 A1 A2 A3 B0 B1 B2 B3
  // c0 c1 c2 c3 d0 d1 d2 d3 | C0 C1 C2 C3 D0 D1 D2 D3
  // e0 e1 e2 e3 f0 f1 f2 f3 | E0 E1 E2 E3 F0 F1 F2 F3
  // g0 g1 g2 g3 h0 h1 h2 h3 | G0 G1 G2 G3 H0 H1 H2 H3
  // a4 a5 a6 a7 b4 b5 b6 b7 | A4 A5 A6 A7 B4 B5 B6 B7
  // c4 c5 c6 c7 d4 d5 d6 d7 | C4 C5 C6 C7 D4 D5 D6 D7
  // e4 e5 e6 e7 f4 f5 f6 f7 | E4 E5 E6 E7 F4 F5 F6 F7
  // g4 g5 g6 g7 h4 h5 h6 h7 | G4 G5 G6 G7 H4 H5 H6 H7
  const __m128i x_s20 = _mm_unpacklo_epi16(x_s10, x_s12);
  const __m128i x_s21 = _mm_unpackhi_epi16(x_s10, x_s12);
  const __m128i x_s22 = _mm_unpacklo_epi16(x_s11, x_s13);
  const __m128i x_s23 = _mm_unpackhi_epi16(x_s11, x_s13);
  const __m128i x_s24 = _mm_unpacklo_epi16(x_s14, x_s16);
  const __m128i x_s25 = _mm_unpackhi_epi16(x_s14, x_s16);
  const __m128i x_s26 = _mm_unpacklo_epi16(x_s15, x_s17);
  const __m128i x_s27 = _mm_unpackhi_epi16(x_s15, x_s17);

  // a0 a1 a2 a3 a4 a5 a6 a7 | A0 A1 A2 A3 A4 A5 A6 A7
  // b0 b1 b2 b3 b4 b5 b6 b7 | B0 B1 B2 B3 B4 B5 B6 B7
  // c0 c1 c2 c3 c4 c5 c6 c7 | C0 C1 C2 C3 C4 C5 C6 C7
  // d0 d1 d2 d3 d4 d5 d6 d7 | D0 D1 D2 D3 D4 D5 D6 D7
  // e0 e1 e2 e3 e4 e5 e6 e7 | E0 E1 E2 E3 E4 E5 E6 E7
  // f0 f1 f2 f3 f4 f5 f6 f7 | F0 F1 F2 F3 F4 F5 F6 F7
  // g0 g1 g2 g3 g4 g5 g6 g7 | G0 G1 G2 G3 G4 G5 G6 G7
  // h0 h1 h2 h3 h4 h5 h6 h7 | H0 H1 H2 H3 H4 H5 H6 H7
  const __m128i x_s30 = _mm_unpacklo_epi32(x_s20, x_s24);
  const __m128i x_s31 = _mm_unpackhi_epi32(x_s20, x_s24);
  const __m128i x_s32 = _mm_unpacklo_epi32(x_s21, x_s25);
  const __m128i x_s33 = _mm_unpackhi_epi32(x_s21, x_s25);
  const __m128i x_s34 = _mm_unpacklo_epi32(x_s22, x_s26);
  const __m128i x_s35 = _mm_unpackhi_epi32(x_s22, x_s26);
  const __m128i x_s36 = _mm_unpacklo_epi32(x_s23, x_s27);
  const __m128i x_s37 = _mm_unpackhi_epi32(x_s23, x_s27);

  mm_storelu(dst, x_s30);
  mm_storehu(dst + (1 * out_p), x_s30);
  mm_storelu(dst + (2 * out_p), x_s31);
  mm_storehu(dst + (3 * out_p), x_s31);
  mm_storelu(dst + (4 * out_p), x_s32);
  mm_storehu(dst + (5 * out_p), x_s32);
  mm_storelu(dst + (6 * out_p), x_s33);
  mm_storehu(dst + (7 * out_p), x_s33);
  mm_storelu(dst + (8 * out_p), x_s34);
  mm_storehu(dst + (9 * out_p), x_s34);
  mm_storelu(dst + (10 * out_p), x_s35);
  mm_storehu(dst + (11 * out_p), x_s35);
  mm_storelu(dst + (12 * out_p), x_s36);
  mm_storehu(dst + (13 * out_p), x_s36);
  mm_storelu(dst + (14 * out_p), x_s37);
  mm_storehu(dst + (15 * out_p), x_s37);
}

static inline void transpose_8xn(unsigned char *src[], int in_p,
                                 unsigned char *dst[], int out_p,
                                 int num_8x8_to_transpose) {
  int idx8x8 = 0;
  __m128i x0, x1, x2, x3, x4, x5, x6, x7;
  do {
    unsigned char *in = src[idx8x8];
    unsigned char *out = dst[idx8x8];

    x0 =
        _mm_loadl_epi64((__m128i *)(in + 0 * in_p));  // 00 01 02 03 04 05 06 07
    x1 =
        _mm_loadl_epi64((__m128i *)(in + 1 * in_p));  // 10 11 12 13 14 15 16 17
    // 00 10 01 11 02 12 03 13 04 14 05 15 06 16 07 17
    x0 = _mm_unpacklo_epi8(x0, x1);

    x2 =
        _mm_loadl_epi64((__m128i *)(in + 2 * in_p));  // 20 21 22 23 24 25 26 27
    x3 =
        _mm_loadl_epi64((__m128i *)(in + 3 * in_p));  // 30 31 32 33 34 35 36 37
    // 20 30 21 31 22 32 23 33 24 34 25 35 26 36 27 37
    x1 = _mm_unpacklo_epi8(x2, x3);

    x4 =
        _mm_loadl_epi64((__m128i *)(in + 4 * in_p));  // 40 41 42 43 44 45 46 47
    x5 =
        _mm_loadl_epi64((__m128i *)(in + 5 * in_p));  // 50 51 52 53 54 55 56 57
    // 40 50 41 51 42 52 43 53 44 54 45 55 46 56 47 57
    x2 = _mm_unpacklo_epi8(x4, x5);

    x6 =
        _mm_loadl_epi64((__m128i *)(in + 6 * in_p));  // 60 61 62 63 64 65 66 67
    x7 =
        _mm_loadl_epi64((__m128i *)(in + 7 * in_p));  // 70 71 72 73 74 75 76 77
    // 60 70 61 71 62 72 63 73 64 74 65 75 66 76 67 77
    x3 = _mm_unpacklo_epi8(x6, x7);

    // 00 10 20 30 01 11 21 31 02 12 22 32 03 13 23 33
    x4 = _mm_unpacklo_epi16(x0, x1);
    // 40 50 60 70 41 51 61 71 42 52 62 72 43 53 63 73
    x5 = _mm_unpacklo_epi16(x2, x3);
    // 00 10 20 30 40 50 60 70 01 11 21 31 41 51 61 71
    x6 = _mm_unpacklo_epi32(x4, x5);
    mm_storelu(out + 0 * out_p, x6);  // 00 10 20 30 40 50 60 70
    mm_storehu(out + 1 * out_p, x6);  // 01 11 21 31 41 51 61 71
    // 02 12 22 32 42 52 62 72 03 13 23 33 43 53 63 73
    x7 = _mm_unpackhi_epi32(x4, x5);
    mm_storelu(out + 2 * out_p, x7);  // 02 12 22 32 42 52 62 72
    mm_storehu(out + 3 * out_p, x7);  // 03 13 23 33 43 53 63 73

    // 04 14 24 34 05 15 25 35 06 16 26 36 07 17 27 37
    x4 = _mm_unpackhi_epi16(x0, x1);
    // 44 54 64 74 45 55 65 75 46 56 66 76 47 57 67 77
    x5 = _mm_unpackhi_epi16(x2, x3);
    // 04 14 24 34 44 54 64 74 05 15 25 35 45 55 65 75
    x6 = _mm_unpacklo_epi32(x4, x5);
    mm_storelu(out + 4 * out_p, x6);  // 04 14 24 34 44 54 64 74
    mm_storehu(out + 5 * out_p, x6);  // 05 15 25 35 45 55 65 75
    // 06 16 26 36 46 56 66 76 07 17 27 37 47 57 67 77
    x7 = _mm_unpackhi_epi32(x4, x5);

    mm_storelu(out + 6 * out_p, x7);  // 06 16 26 36 46 56 66 76
    mm_storehu(out + 7 * out_p, x7);  // 07 17 27 37 47 57 67 77
  } while (++idx8x8 < num_8x8_to_transpose);
}

#endif  // AOM_AOM_DSP_X86_LPF_COMMON_SSE2_H_
