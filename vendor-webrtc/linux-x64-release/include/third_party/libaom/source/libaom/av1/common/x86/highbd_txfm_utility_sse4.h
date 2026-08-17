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

#ifndef AOM_AV1_COMMON_X86_HIGHBD_TXFM_UTILITY_SSE4_H_
#define AOM_AV1_COMMON_X86_HIGHBD_TXFM_UTILITY_SSE4_H_

#include <smmintrin.h> /* SSE4.1 */

#define TRANSPOSE_4X4(x0, x1, x2, x3, y0, y1, y2, y3) \
  do {                                                \
    __m128i u0, u1, u2, u3;                           \
    u0 = _mm_unpacklo_epi32(x0, x1);                  \
    u1 = _mm_unpackhi_epi32(x0, x1);                  \
    u2 = _mm_unpacklo_epi32(x2, x3);                  \
    u3 = _mm_unpackhi_epi32(x2, x3);                  \
    y0 = _mm_unpacklo_epi64(u0, u2);                  \
    y1 = _mm_unpackhi_epi64(u0, u2);                  \
    y2 = _mm_unpacklo_epi64(u1, u3);                  \
    y3 = _mm_unpackhi_epi64(u1, u3);                  \
  } while (0)

static inline void transpose_8x8(const __m128i *in, __m128i *out) {
  TRANSPOSE_4X4(in[0], in[2], in[4], in[6], out[0], out[2], out[4], out[6]);
  TRANSPOSE_4X4(in[1], in[3], in[5], in[7], out[8], out[10], out[12], out[14]);
  TRANSPOSE_4X4(in[8], in[10], in[12], in[14], out[1], out[3], out[5], out[7]);
  TRANSPOSE_4X4(in[9], in[11], in[13], in[15], out[9], out[11], out[13],
                out[15]);
}

static inline void transpose_16x16(const __m128i *in, __m128i *out) {
  // Upper left 8x8
  TRANSPOSE_4X4(in[0], in[4], in[8], in[12], out[0], out[4], out[8], out[12]);
  TRANSPOSE_4X4(in[1], in[5], in[9], in[13], out[16], out[20], out[24],
                out[28]);
  TRANSPOSE_4X4(in[16], in[20], in[24], in[28], out[1], out[5], out[9],
                out[13]);
  TRANSPOSE_4X4(in[17], in[21], in[25], in[29], out[17], out[21], out[25],
                out[29]);

  // Upper right 8x8
  TRANSPOSE_4X4(in[2], in[6], in[10], in[14], out[32], out[36], out[40],
                out[44]);
  TRANSPOSE_4X4(in[3], in[7], in[11], in[15], out[48], out[52], out[56],
                out[60]);
  TRANSPOSE_4X4(in[18], in[22], in[26], in[30], out[33], out[37], out[41],
                out[45]);
  TRANSPOSE_4X4(in[19], in[23], in[27], in[31], out[49], out[53], out[57],
                out[61]);

  // Lower left 8x8
  TRANSPOSE_4X4(in[32], in[36], in[40], in[44], out[2], out[6], out[10],
                out[14]);
  TRANSPOSE_4X4(in[33], in[37], in[41], in[45], out[18], out[22], out[26],
                out[30]);
  TRANSPOSE_4X4(in[48], in[52], in[56], in[60], out[3], out[7], out[11],
                out[15]);
  TRANSPOSE_4X4(in[49], in[53], in[57], in[61], out[19], out[23], out[27],
                out[31]);
  // Lower right 8x8
  TRANSPOSE_4X4(in[34], in[38], in[42], in[46], out[34], out[38], out[42],
                out[46]);
  TRANSPOSE_4X4(in[35], in[39], in[43], in[47], out[50], out[54], out[58],
                out[62]);
  TRANSPOSE_4X4(in[50], in[54], in[58], in[62], out[35], out[39], out[43],
                out[47]);
  TRANSPOSE_4X4(in[51], in[55], in[59], in[63], out[51], out[55], out[59],
                out[63]);
}

static inline void transpose_8nx8n(const __m128i *input, __m128i *output,
                                   const int width, const int height) {
  const int numcol = height >> 2;
  const int numrow = width >> 2;
  for (int j = 0; j < numrow; j++) {
    for (int i = 0; i < numcol; i++) {
      TRANSPOSE_4X4(input[i * width + j + (numrow * 0)],
                    input[i * width + j + (numrow * 1)],
                    input[i * width + j + (numrow * 2)],
                    input[i * width + j + (numrow * 3)],
                    output[j * height + i + (numcol * 0)],
                    output[j * height + i + (numcol * 1)],
                    output[j * height + i + (numcol * 2)],
                    output[j * height + i + (numcol * 3)]);
    }
  }
}

// Note:
//  rounding = 1 << (bit - 1)
static inline __m128i half_btf_sse4_1(const __m128i *w0, const __m128i *n0,
                                      const __m128i *w1, const __m128i *n1,
                                      const __m128i *rounding, int bit) {
  __m128i x, y;

  x = _mm_mullo_epi32(*w0, *n0);
  y = _mm_mullo_epi32(*w1, *n1);
  x = _mm_add_epi32(x, y);
  x = _mm_add_epi32(x, *rounding);
  x = _mm_srai_epi32(x, bit);
  return x;
}

static inline __m128i half_btf_0_sse4_1(const __m128i *w0, const __m128i *n0,
                                        const __m128i *rounding, int bit) {
  __m128i x;

  x = _mm_mullo_epi32(*w0, *n0);
  x = _mm_add_epi32(x, *rounding);
  x = _mm_srai_epi32(x, bit);
  return x;
}

typedef void (*transform_1d_sse4_1)(__m128i *in, __m128i *out, int bit,
                                    int do_cols, int bd, int out_shift);

typedef void (*fwd_transform_1d_sse4_1)(__m128i *in, __m128i *out, int bit,
                                        const int num_cols);

void av1_highbd_inv_txfm2d_add_universe_sse4_1(const int32_t *input,
                                               uint8_t *output, int stride,
                                               TX_TYPE tx_type, TX_SIZE tx_size,
                                               int eob, const int bd);

#endif  // AOM_AV1_COMMON_X86_HIGHBD_TXFM_UTILITY_SSE4_H_
