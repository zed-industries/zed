/*
 * Copyright (c) 2018, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */
#ifndef AOM_AV1_COMMON_X86_AV1_TXFM_SSE2_H_
#define AOM_AV1_COMMON_X86_AV1_TXFM_SSE2_H_

#include <emmintrin.h>  // SSE2

#include "config/aom_config.h"
#include "config/av1_rtcd.h"

#include "aom/aom_integer.h"
#include "aom_dsp/x86/transpose_sse2.h"
#include "aom_dsp/x86/txfm_common_sse2.h"
#include "av1/common/av1_txfm.h"

#ifdef __cplusplus
extern "C" {
#endif

static inline void btf_16_w4_sse2(
    const __m128i *const w0, const __m128i *const w1, const __m128i __rounding,
    const int8_t cos_bit, const __m128i *const in0, const __m128i *const in1,
    __m128i *const out0, __m128i *const out1) {
  const __m128i t0 = _mm_unpacklo_epi16(*in0, *in1);
  const __m128i u0 = _mm_madd_epi16(t0, *w0);
  const __m128i v0 = _mm_madd_epi16(t0, *w1);
  const __m128i a0 = _mm_add_epi32(u0, __rounding);
  const __m128i b0 = _mm_add_epi32(v0, __rounding);
  const __m128i c0 = _mm_srai_epi32(a0, cos_bit);
  const __m128i d0 = _mm_srai_epi32(b0, cos_bit);

  *out0 = _mm_packs_epi32(c0, c0);
  *out1 = _mm_packs_epi32(d0, c0);
}

#define btf_16_4p_sse2(w0, w1, in0, in1, out0, out1) \
  do {                                               \
    __m128i t0 = _mm_unpacklo_epi16(in0, in1);       \
    __m128i u0 = _mm_madd_epi16(t0, w0);             \
    __m128i v0 = _mm_madd_epi16(t0, w1);             \
                                                     \
    __m128i a0 = _mm_add_epi32(u0, __rounding);      \
    __m128i b0 = _mm_add_epi32(v0, __rounding);      \
                                                     \
    __m128i c0 = _mm_srai_epi32(a0, cos_bit);        \
    __m128i d0 = _mm_srai_epi32(b0, cos_bit);        \
                                                     \
    out0 = _mm_packs_epi32(c0, c0);                  \
    out1 = _mm_packs_epi32(d0, d0);                  \
  } while (0)

#define btf_16_sse2(w0, w1, in0, in1, out0, out1) \
  do {                                            \
    __m128i t0 = _mm_unpacklo_epi16(in0, in1);    \
    __m128i t1 = _mm_unpackhi_epi16(in0, in1);    \
    __m128i u0 = _mm_madd_epi16(t0, w0);          \
    __m128i u1 = _mm_madd_epi16(t1, w0);          \
    __m128i v0 = _mm_madd_epi16(t0, w1);          \
    __m128i v1 = _mm_madd_epi16(t1, w1);          \
                                                  \
    __m128i a0 = _mm_add_epi32(u0, __rounding);   \
    __m128i a1 = _mm_add_epi32(u1, __rounding);   \
    __m128i b0 = _mm_add_epi32(v0, __rounding);   \
    __m128i b1 = _mm_add_epi32(v1, __rounding);   \
                                                  \
    __m128i c0 = _mm_srai_epi32(a0, cos_bit);     \
    __m128i c1 = _mm_srai_epi32(a1, cos_bit);     \
    __m128i d0 = _mm_srai_epi32(b0, cos_bit);     \
    __m128i d1 = _mm_srai_epi32(b1, cos_bit);     \
                                                  \
    out0 = _mm_packs_epi32(c0, c1);               \
    out1 = _mm_packs_epi32(d0, d1);               \
  } while (0)

static inline __m128i load_16bit_to_16bit(const int16_t *a) {
  return _mm_load_si128((const __m128i *)a);
}

static inline __m128i load_32bit_to_16bit(const int32_t *a) {
  const __m128i a_low = _mm_load_si128((const __m128i *)a);
  return _mm_packs_epi32(a_low, *(const __m128i *)(a + 4));
}

static inline __m128i load_32bit_to_16bit_w4(const int32_t *a) {
  const __m128i a_low = _mm_load_si128((const __m128i *)a);
  return _mm_packs_epi32(a_low, a_low);
}

// Store 4 16 bit values. Sign extend the values.
static inline void store_16bit_to_32bit_w4(const __m128i a, int32_t *const b) {
  const __m128i a_lo = _mm_unpacklo_epi16(a, a);
  const __m128i a_1 = _mm_srai_epi32(a_lo, 16);
  _mm_store_si128((__m128i *)b, a_1);
}

// Store 8 16 bit values. Sign extend the values.
static inline void store_16bit_to_32bit(__m128i a, int32_t *b) {
  const __m128i a_lo = _mm_unpacklo_epi16(a, a);
  const __m128i a_hi = _mm_unpackhi_epi16(a, a);
  const __m128i a_1 = _mm_srai_epi32(a_lo, 16);
  const __m128i a_2 = _mm_srai_epi32(a_hi, 16);
  _mm_store_si128((__m128i *)b, a_1);
  _mm_store_si128((__m128i *)(b + 4), a_2);
}

static inline __m128i scale_round_sse2(const __m128i a, const int scale) {
  const __m128i scale_rounding = pair_set_epi16(scale, 1 << (NewSqrt2Bits - 1));
  const __m128i b = _mm_madd_epi16(a, scale_rounding);
  return _mm_srai_epi32(b, NewSqrt2Bits);
}

static inline void store_rect_16bit_to_32bit_w4(const __m128i a,
                                                int32_t *const b) {
  const __m128i one = _mm_set1_epi16(1);
  const __m128i a_lo = _mm_unpacklo_epi16(a, one);
  const __m128i b_lo = scale_round_sse2(a_lo, NewSqrt2);
  _mm_store_si128((__m128i *)b, b_lo);
}

static inline void store_rect_16bit_to_32bit(const __m128i a,
                                             int32_t *const b) {
  const __m128i one = _mm_set1_epi16(1);
  const __m128i a_lo = _mm_unpacklo_epi16(a, one);
  const __m128i a_hi = _mm_unpackhi_epi16(a, one);
  const __m128i b_lo = scale_round_sse2(a_lo, NewSqrt2);
  const __m128i b_hi = scale_round_sse2(a_hi, NewSqrt2);
  _mm_store_si128((__m128i *)b, b_lo);
  _mm_store_si128((__m128i *)(b + 4), b_hi);
}

static inline void load_buffer_16bit_to_16bit_w4(const int16_t *const in,
                                                 const int stride,
                                                 __m128i *const out,
                                                 const int out_size) {
  for (int i = 0; i < out_size; ++i) {
    out[i] = _mm_loadl_epi64((const __m128i *)(in + i * stride));
  }
}

static inline void load_buffer_16bit_to_16bit_w4_flip(const int16_t *const in,
                                                      const int stride,
                                                      __m128i *const out,
                                                      const int out_size) {
  for (int i = 0; i < out_size; ++i) {
    out[out_size - i - 1] = _mm_loadl_epi64((const __m128i *)(in + i * stride));
  }
}

static inline void load_buffer_16bit_to_16bit(const int16_t *in, int stride,
                                              __m128i *out, int out_size) {
  for (int i = 0; i < out_size; ++i) {
    out[i] = load_16bit_to_16bit(in + i * stride);
  }
}

static inline void load_buffer_16bit_to_16bit_flip(const int16_t *in,
                                                   int stride, __m128i *out,
                                                   int out_size) {
  for (int i = 0; i < out_size; ++i) {
    out[out_size - i - 1] = load_16bit_to_16bit(in + i * stride);
  }
}

static inline void load_buffer_32bit_to_16bit(const int32_t *in, int stride,
                                              __m128i *out, int out_size) {
  for (int i = 0; i < out_size; ++i) {
    out[i] = load_32bit_to_16bit(in + i * stride);
  }
}

static inline void load_buffer_32bit_to_16bit_w4(const int32_t *in, int stride,
                                                 __m128i *out, int out_size) {
  for (int i = 0; i < out_size; ++i) {
    out[i] = load_32bit_to_16bit_w4(in + i * stride);
  }
}

static inline void load_buffer_32bit_to_16bit_flip(const int32_t *in,
                                                   int stride, __m128i *out,
                                                   int out_size) {
  for (int i = 0; i < out_size; ++i) {
    out[out_size - i - 1] = load_32bit_to_16bit(in + i * stride);
  }
}

static inline void store_buffer_16bit_to_32bit_w4(const __m128i *const in,
                                                  int32_t *const out,
                                                  const int stride,
                                                  const int out_size) {
  for (int i = 0; i < out_size; ++i) {
    store_16bit_to_32bit_w4(in[i], out + i * stride);
  }
}

static inline void store_buffer_16bit_to_32bit_w8(const __m128i *const in,
                                                  int32_t *const out,
                                                  const int stride,
                                                  const int out_size) {
  for (int i = 0; i < out_size; ++i) {
    store_16bit_to_32bit(in[i], out + i * stride);
  }
}

static inline void store_rect_buffer_16bit_to_32bit_w4(const __m128i *const in,
                                                       int32_t *const out,
                                                       const int stride,
                                                       const int out_size) {
  for (int i = 0; i < out_size; ++i) {
    store_rect_16bit_to_32bit_w4(in[i], out + i * stride);
  }
}

static inline void store_rect_buffer_16bit_to_32bit_w8(const __m128i *const in,
                                                       int32_t *const out,
                                                       const int stride,
                                                       const int out_size) {
  for (int i = 0; i < out_size; ++i) {
    store_rect_16bit_to_32bit(in[i], out + i * stride);
  }
}

static inline void store_buffer_16bit_to_16bit_8x8(const __m128i *in,
                                                   uint16_t *out,
                                                   const int stride) {
  for (int i = 0; i < 8; ++i) {
    _mm_store_si128((__m128i *)(out + i * stride), in[i]);
  }
}

static inline void round_shift_16bit(__m128i *in, int size, int bit) {
  if (bit < 0) {
    bit = -bit;
    __m128i rounding = _mm_set1_epi16(1 << (bit - 1));
    for (int i = 0; i < size; ++i) {
      in[i] = _mm_adds_epi16(in[i], rounding);
      in[i] = _mm_srai_epi16(in[i], bit);
    }
  } else if (bit > 0) {
    for (int i = 0; i < size; ++i) {
      in[i] = _mm_slli_epi16(in[i], bit);
    }
  }
}

static inline void flip_buf_sse2(__m128i *in, __m128i *out, int size) {
  for (int i = 0; i < size; ++i) {
    out[size - i - 1] = in[i];
  }
}

void av1_lowbd_fwd_txfm2d_4x4_sse2(const int16_t *input, int32_t *output,
                                   int stride, TX_TYPE tx_type, int bd);

void av1_lowbd_fwd_txfm2d_4x8_sse2(const int16_t *input, int32_t *output,
                                   int stride, TX_TYPE tx_type, int bd);

void av1_lowbd_fwd_txfm2d_4x16_sse2(const int16_t *input, int32_t *output,
                                    int stride, TX_TYPE tx_type, int bd);

void av1_lowbd_fwd_txfm2d_8x4_sse2(const int16_t *input, int32_t *output,
                                   int stride, TX_TYPE tx_type, int bd);

void av1_lowbd_fwd_txfm2d_8x8_sse2(const int16_t *input, int32_t *output,
                                   int stride, TX_TYPE tx_type, int bd);

void av1_lowbd_fwd_txfm2d_8x16_sse2(const int16_t *input, int32_t *output,
                                    int stride, TX_TYPE tx_type, int bd);

void av1_lowbd_fwd_txfm2d_8x32_sse2(const int16_t *input, int32_t *output,
                                    int stride, TX_TYPE tx_type, int bd);

void av1_lowbd_fwd_txfm2d_16x4_sse2(const int16_t *input, int32_t *output,
                                    int stride, TX_TYPE tx_type, int bd);

void av1_lowbd_fwd_txfm2d_16x8_sse2(const int16_t *input, int32_t *output,
                                    int stride, TX_TYPE tx_type, int bd);

void av1_lowbd_fwd_txfm2d_16x16_sse2(const int16_t *input, int32_t *output,
                                     int stride, TX_TYPE tx_type, int bd);

void av1_lowbd_fwd_txfm2d_16x32_sse2(const int16_t *input, int32_t *output,
                                     int stride, TX_TYPE tx_type, int bd);

void av1_lowbd_fwd_txfm2d_32x8_sse2(const int16_t *input, int32_t *output,
                                    int stride, TX_TYPE tx_type, int bd);

void av1_lowbd_fwd_txfm2d_32x16_sse2(const int16_t *input, int32_t *output,
                                     int stride, TX_TYPE tx_type, int bd);

void av1_lowbd_fwd_txfm2d_32x32_sse2(const int16_t *input, int32_t *output,
                                     int stride, TX_TYPE tx_type, int bd);

void av1_lowbd_fwd_txfm2d_16x64_sse2(const int16_t *input, int32_t *output,
                                     int stride, TX_TYPE tx_type, int bd);

void av1_lowbd_fwd_txfm2d_64x16_sse2(const int16_t *input, int32_t *output,
                                     int stride, TX_TYPE tx_type, int bd);

typedef void (*transform_1d_sse2)(const __m128i *input, __m128i *output,
                                  int8_t cos_bit);

void av1_iadst8_sse2(const __m128i *input, __m128i *output);

void av1_idct8_sse2(const __m128i *input, __m128i *output);

typedef struct {
  transform_1d_sse2 col, row;  // vertical and horizontal
} transform_2d_sse2;

#ifdef __cplusplus
}
#endif  // __cplusplus
#endif  // AOM_AV1_COMMON_X86_AV1_TXFM_SSE2_H_
