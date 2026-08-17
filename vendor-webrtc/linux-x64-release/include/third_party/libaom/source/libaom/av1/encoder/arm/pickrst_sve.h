/*
 * Copyright (c) 2024, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

#ifndef AOM_AV1_ENCODER_ARM_PICKRST_SVE_H_
#define AOM_AV1_ENCODER_ARM_PICKRST_SVE_H_

#include <arm_neon.h>
#include <arm_sve.h>

#include "aom_dsp/arm/aom_neon_sve_bridge.h"
#include "av1/encoder/arm/pickrst_neon.h"

// Swap each half of the dgd vectors so that we can accumulate the result of
// the dot-products directly in the destination matrix.
static inline int16x8x2_t transpose_dgd(int16x8_t dgd0, int16x8_t dgd1) {
  int16x8_t dgd_trn0 = vreinterpretq_s16_s64(
      vzip1q_s64(vreinterpretq_s64_s16(dgd0), vreinterpretq_s64_s16(dgd1)));
  int16x8_t dgd_trn1 = vreinterpretq_s16_s64(
      vzip2q_s64(vreinterpretq_s64_s16(dgd0), vreinterpretq_s64_s16(dgd1)));

  return (struct int16x8x2_t){ dgd_trn0, dgd_trn1 };
}

static inline void compute_M_one_row_win5(int16x8_t src, int16x8_t dgd[5],
                                          int64_t *M, int row) {
  const int wiener_win = 5;

  int64x2_t m01 = vld1q_s64(M + row * wiener_win + 0);
  int16x8x2_t dgd01 = transpose_dgd(dgd[0], dgd[1]);

  int64x2_t cross_corr01 = aom_svdot_lane_s16(m01, dgd01.val[0], src, 0);
  cross_corr01 = aom_svdot_lane_s16(cross_corr01, dgd01.val[1], src, 1);
  vst1q_s64(M + row * wiener_win + 0, cross_corr01);

  int64x2_t m23 = vld1q_s64(M + row * wiener_win + 2);
  int16x8x2_t dgd23 = transpose_dgd(dgd[2], dgd[3]);

  int64x2_t cross_corr23 = aom_svdot_lane_s16(m23, dgd23.val[0], src, 0);
  cross_corr23 = aom_svdot_lane_s16(cross_corr23, dgd23.val[1], src, 1);
  vst1q_s64(M + row * wiener_win + 2, cross_corr23);

  int64x2_t m4 = aom_sdotq_s16(vdupq_n_s64(0), src, dgd[4]);
  M[row * wiener_win + 4] += vaddvq_s64(m4);
}

static inline void compute_M_one_row_win7(int16x8_t src, int16x8_t dgd[7],
                                          int64_t *M, int row) {
  const int wiener_win = 7;

  int64x2_t m01 = vld1q_s64(M + row * wiener_win + 0);
  int16x8x2_t dgd01 = transpose_dgd(dgd[0], dgd[1]);

  int64x2_t cross_corr01 = aom_svdot_lane_s16(m01, dgd01.val[0], src, 0);
  cross_corr01 = aom_svdot_lane_s16(cross_corr01, dgd01.val[1], src, 1);
  vst1q_s64(M + row * wiener_win + 0, cross_corr01);

  int64x2_t m23 = vld1q_s64(M + row * wiener_win + 2);
  int16x8x2_t dgd23 = transpose_dgd(dgd[2], dgd[3]);

  int64x2_t cross_corr23 = aom_svdot_lane_s16(m23, dgd23.val[0], src, 0);
  cross_corr23 = aom_svdot_lane_s16(cross_corr23, dgd23.val[1], src, 1);
  vst1q_s64(M + row * wiener_win + 2, cross_corr23);

  int64x2_t m45 = vld1q_s64(M + row * wiener_win + 4);
  int16x8x2_t dgd45 = transpose_dgd(dgd[4], dgd[5]);

  int64x2_t cross_corr45 = aom_svdot_lane_s16(m45, dgd45.val[0], src, 0);
  cross_corr45 = aom_svdot_lane_s16(cross_corr45, dgd45.val[1], src, 1);
  vst1q_s64(M + row * wiener_win + 4, cross_corr45);

  int64x2_t m6 = aom_sdotq_s16(vdupq_n_s64(0), src, dgd[6]);
  M[row * wiener_win + 6] += vaddvq_s64(m6);
}

static inline void compute_H_one_col(int16x8_t *dgd, int col, int64_t *H,
                                     const int wiener_win,
                                     const int wiener_win2) {
  for (int row0 = 0; row0 < wiener_win; row0++) {
    for (int row1 = row0; row1 < wiener_win; row1++) {
      int auto_cov_idx =
          (col * wiener_win + row0) * wiener_win2 + (col * wiener_win) + row1;

      int64x2_t auto_cov = aom_sdotq_s16(vdupq_n_s64(0), dgd[row0], dgd[row1]);
      H[auto_cov_idx] += vaddvq_s64(auto_cov);
    }
  }
}

static inline void compute_H_two_rows_win5(int16x8_t *dgd0, int16x8_t *dgd1,
                                           int row0, int row1, int64_t *H) {
  for (int col0 = 0; col0 < 5; col0++) {
    int auto_cov_idx = (row0 * 5 + col0) * 25 + (row1 * 5);

    int64x2_t h01 = vld1q_s64(H + auto_cov_idx);
    int16x8x2_t dgd01 = transpose_dgd(dgd1[0], dgd1[1]);

    int64x2_t auto_cov01 = aom_svdot_lane_s16(h01, dgd01.val[0], dgd0[col0], 0);
    auto_cov01 = aom_svdot_lane_s16(auto_cov01, dgd01.val[1], dgd0[col0], 1);
    vst1q_s64(H + auto_cov_idx, auto_cov01);

    int64x2_t h23 = vld1q_s64(H + auto_cov_idx + 2);
    int16x8x2_t dgd23 = transpose_dgd(dgd1[2], dgd1[3]);

    int64x2_t auto_cov23 = aom_svdot_lane_s16(h23, dgd23.val[0], dgd0[col0], 0);
    auto_cov23 = aom_svdot_lane_s16(auto_cov23, dgd23.val[1], dgd0[col0], 1);
    vst1q_s64(H + auto_cov_idx + 2, auto_cov23);

    int64x2_t auto_cov4 = aom_sdotq_s16(vdupq_n_s64(0), dgd0[col0], dgd1[4]);
    H[auto_cov_idx + 4] += vaddvq_s64(auto_cov4);
  }
}

static inline void compute_H_two_rows_win7(int16x8_t *dgd0, int16x8_t *dgd1,
                                           int row0, int row1, int64_t *H) {
  for (int col0 = 0; col0 < 7; col0++) {
    int auto_cov_idx = (row0 * 7 + col0) * 49 + (row1 * 7);

    int64x2_t h01 = vld1q_s64(H + auto_cov_idx);
    int16x8x2_t dgd01 = transpose_dgd(dgd1[0], dgd1[1]);

    int64x2_t auto_cov01 = aom_svdot_lane_s16(h01, dgd01.val[0], dgd0[col0], 0);
    auto_cov01 = aom_svdot_lane_s16(auto_cov01, dgd01.val[1], dgd0[col0], 1);
    vst1q_s64(H + auto_cov_idx, auto_cov01);

    int64x2_t h23 = vld1q_s64(H + auto_cov_idx + 2);
    int16x8x2_t dgd23 = transpose_dgd(dgd1[2], dgd1[3]);

    int64x2_t auto_cov23 = aom_svdot_lane_s16(h23, dgd23.val[0], dgd0[col0], 0);
    auto_cov23 = aom_svdot_lane_s16(auto_cov23, dgd23.val[1], dgd0[col0], 1);
    vst1q_s64(H + auto_cov_idx + 2, auto_cov23);

    int64x2_t h45 = vld1q_s64(H + auto_cov_idx + 4);
    int16x8x2_t dgd45 = transpose_dgd(dgd1[4], dgd1[5]);

    int64x2_t auto_cov45 = aom_svdot_lane_s16(h45, dgd45.val[0], dgd0[col0], 0);
    auto_cov45 = aom_svdot_lane_s16(auto_cov45, dgd45.val[1], dgd0[col0], 1);
    vst1q_s64(H + auto_cov_idx + 4, auto_cov45);

    int64x2_t auto_cov6 = aom_sdotq_s16(vdupq_n_s64(0), dgd0[col0], dgd1[6]);
    H[auto_cov_idx + 6] += vaddvq_s64(auto_cov6);
  }
}

static inline void stats_top_win5_sve(const int16x8_t src[2],
                                      const int16x8_t dgd[2],
                                      const int16_t *const d,
                                      const int32_t d_stride, int64x2_t *sum_m,
                                      int64x2_t *sum_h) {
  int16x8_t dgds[WIENER_WIN_CHROMA * 2];

  load_s16_8x5(d + 0, d_stride, &dgds[0], &dgds[2], &dgds[4], &dgds[6],
               &dgds[8]);
  load_s16_8x5(d + 8, d_stride, &dgds[1], &dgds[3], &dgds[5], &dgds[7],
               &dgds[9]);

  sum_m[0] = aom_sdotq_s16(sum_m[0], src[0], dgds[0]);
  sum_m[0] = aom_sdotq_s16(sum_m[0], src[1], dgds[1]);
  sum_m[1] = aom_sdotq_s16(sum_m[1], src[0], dgds[2]);
  sum_m[1] = aom_sdotq_s16(sum_m[1], src[1], dgds[3]);
  sum_m[2] = aom_sdotq_s16(sum_m[2], src[0], dgds[4]);
  sum_m[2] = aom_sdotq_s16(sum_m[2], src[1], dgds[5]);
  sum_m[3] = aom_sdotq_s16(sum_m[3], src[0], dgds[6]);
  sum_m[3] = aom_sdotq_s16(sum_m[3], src[1], dgds[7]);
  sum_m[4] = aom_sdotq_s16(sum_m[4], src[0], dgds[8]);
  sum_m[4] = aom_sdotq_s16(sum_m[4], src[1], dgds[9]);

  sum_h[0] = aom_sdotq_s16(sum_h[0], dgd[0], dgds[0]);
  sum_h[0] = aom_sdotq_s16(sum_h[0], dgd[1], dgds[1]);
  sum_h[1] = aom_sdotq_s16(sum_h[1], dgd[0], dgds[2]);
  sum_h[1] = aom_sdotq_s16(sum_h[1], dgd[1], dgds[3]);
  sum_h[2] = aom_sdotq_s16(sum_h[2], dgd[0], dgds[4]);
  sum_h[2] = aom_sdotq_s16(sum_h[2], dgd[1], dgds[5]);
  sum_h[3] = aom_sdotq_s16(sum_h[3], dgd[0], dgds[6]);
  sum_h[3] = aom_sdotq_s16(sum_h[3], dgd[1], dgds[7]);
  sum_h[4] = aom_sdotq_s16(sum_h[4], dgd[0], dgds[8]);
  sum_h[4] = aom_sdotq_s16(sum_h[4], dgd[1], dgds[9]);
}

static inline void stats_left_win5_sve(const int16x8_t src[2], const int16_t *d,
                                       const int32_t d_stride, int64x2_t *sum) {
  int16x8_t dgds[WIN_CHROMA];

  load_s16_8x4(d + d_stride + 0, d_stride, &dgds[0], &dgds[2], &dgds[4],
               &dgds[6]);
  load_s16_8x4(d + d_stride + 8, d_stride, &dgds[1], &dgds[3], &dgds[5],
               &dgds[7]);

  sum[0] = aom_sdotq_s16(sum[0], src[0], dgds[0]);
  sum[0] = aom_sdotq_s16(sum[0], src[1], dgds[1]);
  sum[1] = aom_sdotq_s16(sum[1], src[0], dgds[2]);
  sum[1] = aom_sdotq_s16(sum[1], src[1], dgds[3]);
  sum[2] = aom_sdotq_s16(sum[2], src[0], dgds[4]);
  sum[2] = aom_sdotq_s16(sum[2], src[1], dgds[5]);
  sum[3] = aom_sdotq_s16(sum[3], src[0], dgds[6]);
  sum[3] = aom_sdotq_s16(sum[3], src[1], dgds[7]);
}

static inline void sub_deltas_step4_sve(int16x8_t *A, int16x8_t *B,
                                        int64x2_t *deltas) {
  deltas[0] = aom_sdotq_s16(deltas[0], vnegq_s16(A[0]), B[0]);
  deltas[1] = aom_sdotq_s16(deltas[1], vnegq_s16(A[0]), B[1]);
  deltas[2] = aom_sdotq_s16(deltas[2], vnegq_s16(A[0]), B[2]);
  deltas[3] = aom_sdotq_s16(deltas[3], vnegq_s16(A[0]), B[3]);
  deltas[4] = aom_sdotq_s16(deltas[4], vnegq_s16(A[0]), B[4]);
  deltas[5] = aom_sdotq_s16(deltas[5], vnegq_s16(A[1]), B[0]);
  deltas[6] = aom_sdotq_s16(deltas[6], vnegq_s16(A[2]), B[0]);
  deltas[7] = aom_sdotq_s16(deltas[7], vnegq_s16(A[3]), B[0]);
  deltas[8] = aom_sdotq_s16(deltas[8], vnegq_s16(A[4]), B[0]);
}

static inline void add_deltas_step4_sve(int16x8_t *A, int16x8_t *B,
                                        int64x2_t *deltas) {
  deltas[0] = aom_sdotq_s16(deltas[0], A[0], B[0]);
  deltas[1] = aom_sdotq_s16(deltas[1], A[0], B[1]);
  deltas[2] = aom_sdotq_s16(deltas[2], A[0], B[2]);
  deltas[3] = aom_sdotq_s16(deltas[3], A[0], B[3]);
  deltas[4] = aom_sdotq_s16(deltas[4], A[0], B[4]);
  deltas[5] = aom_sdotq_s16(deltas[5], A[1], B[0]);
  deltas[6] = aom_sdotq_s16(deltas[6], A[2], B[0]);
  deltas[7] = aom_sdotq_s16(deltas[7], A[3], B[0]);
  deltas[8] = aom_sdotq_s16(deltas[8], A[4], B[0]);
}

static inline void load_square_win5_sve(
    const int16_t *const di, const int16_t *const dj, const int32_t d_stride,
    const int32_t height, int16x8_t *d_is, int16x8_t *d_ie, int16x8_t *d_js,
    int16x8_t *d_je, svbool_t p0, svbool_t p1) {
  d_is[0] = svget_neonq_s16(svld1_s16(p0, di + 0 * d_stride + 0));
  d_is[1] = svget_neonq_s16(svld1_s16(p1, di + 0 * d_stride + 8));
  d_is[2] = svget_neonq_s16(svld1_s16(p0, di + 1 * d_stride + 0));
  d_is[3] = svget_neonq_s16(svld1_s16(p1, di + 1 * d_stride + 8));
  d_is[4] = svget_neonq_s16(svld1_s16(p0, di + 2 * d_stride + 0));
  d_is[5] = svget_neonq_s16(svld1_s16(p1, di + 2 * d_stride + 8));
  d_is[6] = svget_neonq_s16(svld1_s16(p0, di + 3 * d_stride + 0));
  d_is[7] = svget_neonq_s16(svld1_s16(p1, di + 3 * d_stride + 8));

  d_ie[0] = svget_neonq_s16(svld1_s16(p0, di + (height + 0) * d_stride + 0));
  d_ie[1] = svget_neonq_s16(svld1_s16(p1, di + (height + 0) * d_stride + 8));
  d_ie[2] = svget_neonq_s16(svld1_s16(p0, di + (height + 1) * d_stride + 0));
  d_ie[3] = svget_neonq_s16(svld1_s16(p1, di + (height + 1) * d_stride + 8));
  d_ie[4] = svget_neonq_s16(svld1_s16(p0, di + (height + 2) * d_stride + 0));
  d_ie[5] = svget_neonq_s16(svld1_s16(p1, di + (height + 2) * d_stride + 8));
  d_ie[6] = svget_neonq_s16(svld1_s16(p0, di + (height + 3) * d_stride + 0));
  d_ie[7] = svget_neonq_s16(svld1_s16(p1, di + (height + 3) * d_stride + 8));

  load_s16_8x4(dj + 0, d_stride, &d_js[0], &d_js[2], &d_js[4], &d_js[6]);
  load_s16_8x4(dj + 8, d_stride, &d_js[1], &d_js[3], &d_js[5], &d_js[7]);
  load_s16_8x4(dj + height * d_stride + 0, d_stride, &d_je[0], &d_je[2],
               &d_je[4], &d_je[6]);
  load_s16_8x4(dj + height * d_stride + 8, d_stride, &d_je[1], &d_je[3],
               &d_je[5], &d_je[7]);
}

static inline void update_4_stats_sve(const int64_t *const src,
                                      const int64x2_t *delta,
                                      int64_t *const dst) {
  const int64x2_t s1 = vld1q_s64(src);
  const int64x2_t s2 = vld1q_s64(src + 2);

  vst1q_s64(dst + 0, vaddq_s64(s1, delta[0]));
  vst1q_s64(dst + 2, vaddq_s64(s2, delta[1]));
}

static inline void derive_square_win5_sve(
    int16x8_t *d_is, const int16x8_t *d_ie, const int16x8_t *d_js,
    const int16x8_t *d_je,
    int64x2_t deltas[WIENER_WIN_CHROMA - 1][WIENER_WIN_CHROMA - 1]) {
  d_is[0] = vnegq_s16(d_is[0]);
  d_is[1] = vnegq_s16(d_is[1]);
  d_is[2] = vnegq_s16(d_is[2]);
  d_is[3] = vnegq_s16(d_is[3]);
  d_is[4] = vnegq_s16(d_is[4]);
  d_is[5] = vnegq_s16(d_is[5]);
  d_is[6] = vnegq_s16(d_is[6]);
  d_is[7] = vnegq_s16(d_is[7]);

  deltas[0][0] = aom_sdotq_s16(deltas[0][0], d_is[0], d_js[0]);
  deltas[0][0] = aom_sdotq_s16(deltas[0][0], d_is[1], d_js[1]);
  deltas[0][1] = aom_sdotq_s16(deltas[0][1], d_is[0], d_js[2]);
  deltas[0][1] = aom_sdotq_s16(deltas[0][1], d_is[1], d_js[3]);
  deltas[0][2] = aom_sdotq_s16(deltas[0][2], d_is[0], d_js[4]);
  deltas[0][2] = aom_sdotq_s16(deltas[0][2], d_is[1], d_js[5]);
  deltas[0][3] = aom_sdotq_s16(deltas[0][3], d_is[0], d_js[6]);
  deltas[0][3] = aom_sdotq_s16(deltas[0][3], d_is[1], d_js[7]);

  deltas[1][0] = aom_sdotq_s16(deltas[1][0], d_is[2], d_js[0]);
  deltas[1][0] = aom_sdotq_s16(deltas[1][0], d_is[3], d_js[1]);
  deltas[1][1] = aom_sdotq_s16(deltas[1][1], d_is[2], d_js[2]);
  deltas[1][1] = aom_sdotq_s16(deltas[1][1], d_is[3], d_js[3]);
  deltas[1][2] = aom_sdotq_s16(deltas[1][2], d_is[2], d_js[4]);
  deltas[1][2] = aom_sdotq_s16(deltas[1][2], d_is[3], d_js[5]);
  deltas[1][3] = aom_sdotq_s16(deltas[1][3], d_is[2], d_js[6]);
  deltas[1][3] = aom_sdotq_s16(deltas[1][3], d_is[3], d_js[7]);

  deltas[2][0] = aom_sdotq_s16(deltas[2][0], d_is[4], d_js[0]);
  deltas[2][0] = aom_sdotq_s16(deltas[2][0], d_is[5], d_js[1]);
  deltas[2][1] = aom_sdotq_s16(deltas[2][1], d_is[4], d_js[2]);
  deltas[2][1] = aom_sdotq_s16(deltas[2][1], d_is[5], d_js[3]);
  deltas[2][2] = aom_sdotq_s16(deltas[2][2], d_is[4], d_js[4]);
  deltas[2][2] = aom_sdotq_s16(deltas[2][2], d_is[5], d_js[5]);
  deltas[2][3] = aom_sdotq_s16(deltas[2][3], d_is[4], d_js[6]);
  deltas[2][3] = aom_sdotq_s16(deltas[2][3], d_is[5], d_js[7]);

  deltas[3][0] = aom_sdotq_s16(deltas[3][0], d_is[6], d_js[0]);
  deltas[3][0] = aom_sdotq_s16(deltas[3][0], d_is[7], d_js[1]);
  deltas[3][1] = aom_sdotq_s16(deltas[3][1], d_is[6], d_js[2]);
  deltas[3][1] = aom_sdotq_s16(deltas[3][1], d_is[7], d_js[3]);
  deltas[3][2] = aom_sdotq_s16(deltas[3][2], d_is[6], d_js[4]);
  deltas[3][2] = aom_sdotq_s16(deltas[3][2], d_is[7], d_js[5]);
  deltas[3][3] = aom_sdotq_s16(deltas[3][3], d_is[6], d_js[6]);
  deltas[3][3] = aom_sdotq_s16(deltas[3][3], d_is[7], d_js[7]);

  deltas[0][0] = aom_sdotq_s16(deltas[0][0], d_ie[0], d_je[0]);
  deltas[0][0] = aom_sdotq_s16(deltas[0][0], d_ie[1], d_je[1]);
  deltas[0][1] = aom_sdotq_s16(deltas[0][1], d_ie[0], d_je[2]);
  deltas[0][1] = aom_sdotq_s16(deltas[0][1], d_ie[1], d_je[3]);
  deltas[0][2] = aom_sdotq_s16(deltas[0][2], d_ie[0], d_je[4]);
  deltas[0][2] = aom_sdotq_s16(deltas[0][2], d_ie[1], d_je[5]);
  deltas[0][3] = aom_sdotq_s16(deltas[0][3], d_ie[0], d_je[6]);
  deltas[0][3] = aom_sdotq_s16(deltas[0][3], d_ie[1], d_je[7]);

  deltas[1][0] = aom_sdotq_s16(deltas[1][0], d_ie[2], d_je[0]);
  deltas[1][0] = aom_sdotq_s16(deltas[1][0], d_ie[3], d_je[1]);
  deltas[1][1] = aom_sdotq_s16(deltas[1][1], d_ie[2], d_je[2]);
  deltas[1][1] = aom_sdotq_s16(deltas[1][1], d_ie[3], d_je[3]);
  deltas[1][2] = aom_sdotq_s16(deltas[1][2], d_ie[2], d_je[4]);
  deltas[1][2] = aom_sdotq_s16(deltas[1][2], d_ie[3], d_je[5]);
  deltas[1][3] = aom_sdotq_s16(deltas[1][3], d_ie[2], d_je[6]);
  deltas[1][3] = aom_sdotq_s16(deltas[1][3], d_ie[3], d_je[7]);

  deltas[2][0] = aom_sdotq_s16(deltas[2][0], d_ie[4], d_je[0]);
  deltas[2][0] = aom_sdotq_s16(deltas[2][0], d_ie[5], d_je[1]);
  deltas[2][1] = aom_sdotq_s16(deltas[2][1], d_ie[4], d_je[2]);
  deltas[2][1] = aom_sdotq_s16(deltas[2][1], d_ie[5], d_je[3]);
  deltas[2][2] = aom_sdotq_s16(deltas[2][2], d_ie[4], d_je[4]);
  deltas[2][2] = aom_sdotq_s16(deltas[2][2], d_ie[5], d_je[5]);
  deltas[2][3] = aom_sdotq_s16(deltas[2][3], d_ie[4], d_je[6]);
  deltas[2][3] = aom_sdotq_s16(deltas[2][3], d_ie[5], d_je[7]);

  deltas[3][0] = aom_sdotq_s16(deltas[3][0], d_ie[6], d_je[0]);
  deltas[3][0] = aom_sdotq_s16(deltas[3][0], d_ie[7], d_je[1]);
  deltas[3][1] = aom_sdotq_s16(deltas[3][1], d_ie[6], d_je[2]);
  deltas[3][1] = aom_sdotq_s16(deltas[3][1], d_ie[7], d_je[3]);
  deltas[3][2] = aom_sdotq_s16(deltas[3][2], d_ie[6], d_je[4]);
  deltas[3][2] = aom_sdotq_s16(deltas[3][2], d_ie[7], d_je[5]);
  deltas[3][3] = aom_sdotq_s16(deltas[3][3], d_ie[6], d_je[6]);
  deltas[3][3] = aom_sdotq_s16(deltas[3][3], d_ie[7], d_je[7]);
}

static inline void hadd_update_4_stats_sve(const int64_t *const src,
                                           const int64x2_t *deltas,
                                           int64_t *const dst) {
  int64x2_t src0 = vld1q_s64(src);
  int64x2_t src1 = vld1q_s64(src + 2);
  vst1q_s64(dst + 0, vaddq_s64(src0, vpaddq_s64(deltas[0], deltas[1])));
  vst1q_s64(dst + 2, vaddq_s64(src1, vpaddq_s64(deltas[2], deltas[3])));
}

static inline void load_triangle_win5_sve(const int16_t *const di,
                                          const int32_t d_stride,
                                          const int32_t height, int16x8_t *d_is,
                                          int16x8_t *d_ie, svbool_t p0,
                                          svbool_t p1) {
  d_is[0] = svget_neonq_s16(svld1_s16(p0, di + 0 * d_stride + 0));
  d_is[1] = svget_neonq_s16(svld1_s16(p1, di + 0 * d_stride + 8));
  d_is[2] = svget_neonq_s16(svld1_s16(p0, di + 1 * d_stride + 0));
  d_is[3] = svget_neonq_s16(svld1_s16(p1, di + 1 * d_stride + 8));
  d_is[4] = svget_neonq_s16(svld1_s16(p0, di + 2 * d_stride + 0));
  d_is[5] = svget_neonq_s16(svld1_s16(p1, di + 2 * d_stride + 8));
  d_is[6] = svget_neonq_s16(svld1_s16(p0, di + 3 * d_stride + 0));
  d_is[7] = svget_neonq_s16(svld1_s16(p1, di + 3 * d_stride + 8));
  d_ie[0] = svget_neonq_s16(svld1_s16(p0, di + (height + 0) * d_stride + 0));
  d_ie[1] = svget_neonq_s16(svld1_s16(p1, di + (height + 0) * d_stride + 8));
  d_ie[2] = svget_neonq_s16(svld1_s16(p0, di + (height + 1) * d_stride + 0));
  d_ie[3] = svget_neonq_s16(svld1_s16(p1, di + (height + 1) * d_stride + 8));
  d_ie[4] = svget_neonq_s16(svld1_s16(p0, di + (height + 2) * d_stride + 0));
  d_ie[5] = svget_neonq_s16(svld1_s16(p1, di + (height + 2) * d_stride + 8));
  d_ie[6] = svget_neonq_s16(svld1_s16(p0, di + (height + 3) * d_stride + 0));
  d_ie[7] = svget_neonq_s16(svld1_s16(p1, di + (height + 3) * d_stride + 8));
}

static inline void derive_triangle_win5_sve(const int16x8_t *d_is,
                                            const int16x8_t *d_ie,
                                            int64x2_t *deltas) {
  deltas[0] = aom_sdotq_s16(deltas[0], vnegq_s16(d_is[0]), d_is[0]);
  deltas[0] = aom_sdotq_s16(deltas[0], vnegq_s16(d_is[1]), d_is[1]);
  deltas[1] = aom_sdotq_s16(deltas[1], vnegq_s16(d_is[0]), d_is[2]);
  deltas[1] = aom_sdotq_s16(deltas[1], vnegq_s16(d_is[1]), d_is[3]);
  deltas[2] = aom_sdotq_s16(deltas[2], vnegq_s16(d_is[0]), d_is[4]);
  deltas[2] = aom_sdotq_s16(deltas[2], vnegq_s16(d_is[1]), d_is[5]);
  deltas[3] = aom_sdotq_s16(deltas[3], vnegq_s16(d_is[0]), d_is[6]);
  deltas[3] = aom_sdotq_s16(deltas[3], vnegq_s16(d_is[1]), d_is[7]);
  deltas[4] = aom_sdotq_s16(deltas[4], vnegq_s16(d_is[2]), d_is[2]);
  deltas[4] = aom_sdotq_s16(deltas[4], vnegq_s16(d_is[3]), d_is[3]);
  deltas[5] = aom_sdotq_s16(deltas[5], vnegq_s16(d_is[2]), d_is[4]);
  deltas[5] = aom_sdotq_s16(deltas[5], vnegq_s16(d_is[3]), d_is[5]);
  deltas[6] = aom_sdotq_s16(deltas[6], vnegq_s16(d_is[2]), d_is[6]);
  deltas[6] = aom_sdotq_s16(deltas[6], vnegq_s16(d_is[3]), d_is[7]);
  deltas[7] = aom_sdotq_s16(deltas[7], vnegq_s16(d_is[4]), d_is[4]);
  deltas[7] = aom_sdotq_s16(deltas[7], vnegq_s16(d_is[5]), d_is[5]);
  deltas[8] = aom_sdotq_s16(deltas[8], vnegq_s16(d_is[4]), d_is[6]);
  deltas[8] = aom_sdotq_s16(deltas[8], vnegq_s16(d_is[5]), d_is[7]);
  deltas[9] = aom_sdotq_s16(deltas[9], vnegq_s16(d_is[6]), d_is[6]);
  deltas[9] = aom_sdotq_s16(deltas[9], vnegq_s16(d_is[7]), d_is[7]);

  deltas[0] = aom_sdotq_s16(deltas[0], d_ie[0], d_ie[0]);
  deltas[0] = aom_sdotq_s16(deltas[0], d_ie[1], d_ie[1]);
  deltas[1] = aom_sdotq_s16(deltas[1], d_ie[0], d_ie[2]);
  deltas[1] = aom_sdotq_s16(deltas[1], d_ie[1], d_ie[3]);
  deltas[2] = aom_sdotq_s16(deltas[2], d_ie[0], d_ie[4]);
  deltas[2] = aom_sdotq_s16(deltas[2], d_ie[1], d_ie[5]);
  deltas[3] = aom_sdotq_s16(deltas[3], d_ie[0], d_ie[6]);
  deltas[3] = aom_sdotq_s16(deltas[3], d_ie[1], d_ie[7]);
  deltas[4] = aom_sdotq_s16(deltas[4], d_ie[2], d_ie[2]);
  deltas[4] = aom_sdotq_s16(deltas[4], d_ie[3], d_ie[3]);
  deltas[5] = aom_sdotq_s16(deltas[5], d_ie[2], d_ie[4]);
  deltas[5] = aom_sdotq_s16(deltas[5], d_ie[3], d_ie[5]);
  deltas[6] = aom_sdotq_s16(deltas[6], d_ie[2], d_ie[6]);
  deltas[6] = aom_sdotq_s16(deltas[6], d_ie[3], d_ie[7]);
  deltas[7] = aom_sdotq_s16(deltas[7], d_ie[4], d_ie[4]);
  deltas[7] = aom_sdotq_s16(deltas[7], d_ie[5], d_ie[5]);
  deltas[8] = aom_sdotq_s16(deltas[8], d_ie[4], d_ie[6]);
  deltas[8] = aom_sdotq_s16(deltas[8], d_ie[5], d_ie[7]);
  deltas[9] = aom_sdotq_s16(deltas[9], d_ie[6], d_ie[6]);
  deltas[9] = aom_sdotq_s16(deltas[9], d_ie[7], d_ie[7]);
}

static inline void compute_stats_win5_sve(
    const int16_t *const d, const int32_t d_stride, const int16_t *const s,
    const int32_t s_stride, const int32_t width, const int32_t height,
    int64_t *const M, int64_t *const H) {
  const int32_t wiener_win = WIENER_WIN_CHROMA;
  const int32_t wiener_win2 = wiener_win * wiener_win;
  const int32_t h8 = height & ~7;
  int32_t i, j, x, y;

  // Use a predicate to compute the last columns.
  svbool_t p0 = svwhilelt_b16_u32(0, width % 16 == 0 ? 16 : width % 16);
  svbool_t p1 = svwhilelt_b16_u32(8, width % 16 == 0 ? 16 : width % 16);

  // Step 1: Calculate the top edge of the whole matrix, i.e., the top
  // edge of each triangle and square on the top row.
  j = 0;
  do {
    const int16_t *s_t = s;
    const int16_t *d_t = d;
    int64x2_t sum_m[WIENER_WIN_CHROMA] = { vdupq_n_s64(0) };
    int64x2_t sum_h[WIENER_WIN_CHROMA] = { vdupq_n_s64(0) };
    int16x8_t src[2], dgd[2];

    y = height;
    do {
      x = 0;
      while (x < width - 16) {
        src[0] = vld1q_s16(s_t + x + 0);
        src[1] = vld1q_s16(s_t + x + 8);
        dgd[0] = vld1q_s16(d_t + x + 0);
        dgd[1] = vld1q_s16(d_t + x + 8);
        stats_top_win5_sve(src, dgd, d_t + j + x, d_stride, sum_m, sum_h);
        x += 16;
      }

      src[0] = svget_neonq_s16(svld1_s16(p0, s_t + x + 0));
      src[1] = svget_neonq_s16(svld1_s16(p1, s_t + x + 8));
      dgd[0] = svget_neonq_s16(svld1_s16(p0, d_t + x + 0));
      dgd[1] = svget_neonq_s16(svld1_s16(p1, d_t + x + 8));

      stats_top_win5_sve(src, dgd, d_t + j + x, d_stride, sum_m, sum_h);

      s_t += s_stride;
      d_t += d_stride;
    } while (--y);

    vst1q_s64(&M[wiener_win * j + 0], vpaddq_s64(sum_m[0], sum_m[1]));
    vst1q_s64(&M[wiener_win * j + 2], vpaddq_s64(sum_m[2], sum_m[3]));
    M[wiener_win * j + 4] = vaddvq_s64(sum_m[4]);

    vst1q_s64(&H[wiener_win * j + 0], vpaddq_s64(sum_h[0], sum_h[1]));
    vst1q_s64(&H[wiener_win * j + 2], vpaddq_s64(sum_h[2], sum_h[3]));
    H[wiener_win * j + 4] = vaddvq_s64(sum_h[4]);
  } while (++j < wiener_win);

  // Step 2: Calculate the left edge of each square on the top row.
  j = 1;
  do {
    const int16_t *d_t = d;
    int64x2_t sum_h[WIENER_WIN_CHROMA - 1] = { vdupq_n_s64(0) };
    int16x8_t dgd[2];

    y = height;
    do {
      x = 0;
      while (x < width - 16) {
        dgd[0] = vld1q_s16(d_t + j + x + 0);
        dgd[1] = vld1q_s16(d_t + j + x + 8);
        stats_left_win5_sve(dgd, d_t + x, d_stride, sum_h);
        x += 16;
      }

      dgd[0] = svget_neonq_s16(svld1_s16(p0, d_t + j + x + 0));
      dgd[1] = svget_neonq_s16(svld1_s16(p1, d_t + j + x + 8));

      stats_left_win5_sve(dgd, d_t + x, d_stride, sum_h);

      d_t += d_stride;
    } while (--y);

    int64x2_t sum_h01 = vpaddq_s64(sum_h[0], sum_h[1]);
    int64x2_t sum_h23 = vpaddq_s64(sum_h[2], sum_h[3]);
    vst1_s64(&H[1 * wiener_win2 + j * wiener_win], vget_low_s64(sum_h01));
    vst1_s64(&H[2 * wiener_win2 + j * wiener_win], vget_high_s64(sum_h01));
    vst1_s64(&H[3 * wiener_win2 + j * wiener_win], vget_low_s64(sum_h23));
    vst1_s64(&H[4 * wiener_win2 + j * wiener_win], vget_high_s64(sum_h23));

  } while (++j < wiener_win);

  // Step 3: Derive the top edge of each triangle along the diagonal. No
  // triangle in top row.
  {
    const int16_t *d_t = d;

    if (height % 2) {
      int32x4_t deltas[(WIENER_WIN + 1) * 2] = { vdupq_n_s32(0) };
      int32x4_t deltas_tr[(WIENER_WIN + 1) * 2] = { vdupq_n_s32(0) };
      int16x8_t ds[WIENER_WIN * 2];

      load_s16_8x4(d_t, d_stride, &ds[0], &ds[2], &ds[4], &ds[6]);
      load_s16_8x4(d_t + width, d_stride, &ds[1], &ds[3], &ds[5], &ds[7]);
      d_t += 4 * d_stride;

      step3_win5_oneline_neon(&d_t, d_stride, width, height, ds, deltas);
      transpose_arrays_s32_8x8(deltas, deltas_tr);

      update_5_stats_neon(H + 0 * wiener_win * wiener_win2 + 0 * wiener_win,
                          deltas_tr[0], vgetq_lane_s32(deltas_tr[4], 0),
                          H + 1 * wiener_win * wiener_win2 + 1 * wiener_win);

      update_5_stats_neon(H + 1 * wiener_win * wiener_win2 + 1 * wiener_win,
                          deltas_tr[1], vgetq_lane_s32(deltas_tr[5], 0),
                          H + 2 * wiener_win * wiener_win2 + 2 * wiener_win);

      update_5_stats_neon(H + 2 * wiener_win * wiener_win2 + 2 * wiener_win,
                          deltas_tr[2], vgetq_lane_s32(deltas_tr[6], 0),
                          H + 3 * wiener_win * wiener_win2 + 3 * wiener_win);

      update_5_stats_neon(H + 3 * wiener_win * wiener_win2 + 3 * wiener_win,
                          deltas_tr[3], vgetq_lane_s32(deltas_tr[7], 0),
                          H + 4 * wiener_win * wiener_win2 + 4 * wiener_win);

    } else {
      int32x4_t deltas[WIENER_WIN_CHROMA * 2] = { vdupq_n_s32(0) };
      int16x8_t ds[WIENER_WIN_CHROMA * 2];

      ds[0] = load_unaligned_s16_4x2(d_t + 0 * d_stride, width);
      ds[1] = load_unaligned_s16_4x2(d_t + 1 * d_stride, width);
      ds[2] = load_unaligned_s16_4x2(d_t + 2 * d_stride, width);
      ds[3] = load_unaligned_s16_4x2(d_t + 3 * d_stride, width);

      step3_win5_neon(d_t + 4 * d_stride, d_stride, width, height, ds, deltas);

      transpose_elems_inplace_s32_4x4(&deltas[0], &deltas[1], &deltas[2],
                                      &deltas[3]);

      update_5_stats_neon(H + 0 * wiener_win * wiener_win2 + 0 * wiener_win,
                          deltas[0], vgetq_lane_s32(deltas[4], 0),
                          H + 1 * wiener_win * wiener_win2 + 1 * wiener_win);

      update_5_stats_neon(H + 1 * wiener_win * wiener_win2 + 1 * wiener_win,
                          deltas[1], vgetq_lane_s32(deltas[4], 1),
                          H + 2 * wiener_win * wiener_win2 + 2 * wiener_win);

      update_5_stats_neon(H + 2 * wiener_win * wiener_win2 + 2 * wiener_win,
                          deltas[2], vgetq_lane_s32(deltas[4], 2),
                          H + 3 * wiener_win * wiener_win2 + 3 * wiener_win);

      update_5_stats_neon(H + 3 * wiener_win * wiener_win2 + 3 * wiener_win,
                          deltas[3], vgetq_lane_s32(deltas[4], 3),
                          H + 4 * wiener_win * wiener_win2 + 4 * wiener_win);
    }
  }

  // Step 4: Derive the top and left edge of each square. No square in top and
  // bottom row.
  {
    y = h8;

    int16x4_t d_s[12];
    int16x4_t d_e[12];
    const int16_t *d_t = d;
    int16x4_t zeros = vdup_n_s16(0);
    load_s16_4x4(d_t, d_stride, &d_s[0], &d_s[1], &d_s[2], &d_s[3]);
    load_s16_4x4(d_t + width, d_stride, &d_e[0], &d_e[1], &d_e[2], &d_e[3]);
    int64x2_t deltas[6][18] = { { vdupq_n_s64(0) }, { vdupq_n_s64(0) } };

    while (y >= 8) {
      load_s16_4x8(d_t + 4 * d_stride, d_stride, &d_s[4], &d_s[5], &d_s[6],
                   &d_s[7], &d_s[8], &d_s[9], &d_s[10], &d_s[11]);
      load_s16_4x8(d_t + width + 4 * d_stride, d_stride, &d_e[4], &d_e[5],
                   &d_e[6], &d_e[7], &d_e[8], &d_e[9], &d_e[10], &d_e[11]);

      int16x8_t s_tr[8], e_tr[8];
      transpose_elems_s16_4x8(d_s[0], d_s[1], d_s[2], d_s[3], d_s[4], d_s[5],
                              d_s[6], d_s[7], &s_tr[0], &s_tr[1], &s_tr[2],
                              &s_tr[3]);
      transpose_elems_s16_4x8(d_s[8], d_s[9], d_s[10], d_s[11], zeros, zeros,
                              zeros, zeros, &s_tr[4], &s_tr[5], &s_tr[6],
                              &s_tr[7]);

      transpose_elems_s16_4x8(d_e[0], d_e[1], d_e[2], d_e[3], d_e[4], d_e[5],
                              d_e[6], d_e[7], &e_tr[0], &e_tr[1], &e_tr[2],
                              &e_tr[3]);
      transpose_elems_s16_4x8(d_e[8], d_e[9], d_e[10], d_e[11], zeros, zeros,
                              zeros, zeros, &e_tr[4], &e_tr[5], &e_tr[6],
                              &e_tr[7]);

      int16x8_t start_col0[5], start_col1[5], start_col2[5], start_col3[5];
      start_col0[0] = s_tr[0];
      start_col0[1] = vextq_s16(s_tr[0], s_tr[4], 1);
      start_col0[2] = vextq_s16(s_tr[0], s_tr[4], 2);
      start_col0[3] = vextq_s16(s_tr[0], s_tr[4], 3);
      start_col0[4] = vextq_s16(s_tr[0], s_tr[4], 4);

      start_col1[0] = s_tr[1];
      start_col1[1] = vextq_s16(s_tr[1], s_tr[5], 1);
      start_col1[2] = vextq_s16(s_tr[1], s_tr[5], 2);
      start_col1[3] = vextq_s16(s_tr[1], s_tr[5], 3);
      start_col1[4] = vextq_s16(s_tr[1], s_tr[5], 4);

      start_col2[0] = s_tr[2];
      start_col2[1] = vextq_s16(s_tr[2], s_tr[6], 1);
      start_col2[2] = vextq_s16(s_tr[2], s_tr[6], 2);
      start_col2[3] = vextq_s16(s_tr[2], s_tr[6], 3);
      start_col2[4] = vextq_s16(s_tr[2], s_tr[6], 4);

      start_col3[0] = s_tr[3];
      start_col3[1] = vextq_s16(s_tr[3], s_tr[7], 1);
      start_col3[2] = vextq_s16(s_tr[3], s_tr[7], 2);
      start_col3[3] = vextq_s16(s_tr[3], s_tr[7], 3);
      start_col3[4] = vextq_s16(s_tr[3], s_tr[7], 4);

      // i = 1, j = 2;
      sub_deltas_step4_sve(start_col0, start_col1, deltas[0]);

      // i = 1, j = 3;
      sub_deltas_step4_sve(start_col0, start_col2, deltas[1]);

      // i = 1, j = 4
      sub_deltas_step4_sve(start_col0, start_col3, deltas[2]);

      // i = 2, j =3
      sub_deltas_step4_sve(start_col1, start_col2, deltas[3]);

      // i = 2, j = 4
      sub_deltas_step4_sve(start_col1, start_col3, deltas[4]);

      // i = 3, j = 4
      sub_deltas_step4_sve(start_col2, start_col3, deltas[5]);

      int16x8_t end_col0[5], end_col1[5], end_col2[5], end_col3[5];
      end_col0[0] = e_tr[0];
      end_col0[1] = vextq_s16(e_tr[0], e_tr[4], 1);
      end_col0[2] = vextq_s16(e_tr[0], e_tr[4], 2);
      end_col0[3] = vextq_s16(e_tr[0], e_tr[4], 3);
      end_col0[4] = vextq_s16(e_tr[0], e_tr[4], 4);

      end_col1[0] = e_tr[1];
      end_col1[1] = vextq_s16(e_tr[1], e_tr[5], 1);
      end_col1[2] = vextq_s16(e_tr[1], e_tr[5], 2);
      end_col1[3] = vextq_s16(e_tr[1], e_tr[5], 3);
      end_col1[4] = vextq_s16(e_tr[1], e_tr[5], 4);

      end_col2[0] = e_tr[2];
      end_col2[1] = vextq_s16(e_tr[2], e_tr[6], 1);
      end_col2[2] = vextq_s16(e_tr[2], e_tr[6], 2);
      end_col2[3] = vextq_s16(e_tr[2], e_tr[6], 3);
      end_col2[4] = vextq_s16(e_tr[2], e_tr[6], 4);

      end_col3[0] = e_tr[3];
      end_col3[1] = vextq_s16(e_tr[3], e_tr[7], 1);
      end_col3[2] = vextq_s16(e_tr[3], e_tr[7], 2);
      end_col3[3] = vextq_s16(e_tr[3], e_tr[7], 3);
      end_col3[4] = vextq_s16(e_tr[3], e_tr[7], 4);

      // i = 1, j = 2;
      add_deltas_step4_sve(end_col0, end_col1, deltas[0]);

      // i = 1, j = 3;
      add_deltas_step4_sve(end_col0, end_col2, deltas[1]);

      // i = 1, j = 4
      add_deltas_step4_sve(end_col0, end_col3, deltas[2]);

      // i = 2, j =3
      add_deltas_step4_sve(end_col1, end_col2, deltas[3]);

      // i = 2, j = 4
      add_deltas_step4_sve(end_col1, end_col3, deltas[4]);

      // i = 3, j = 4
      add_deltas_step4_sve(end_col2, end_col3, deltas[5]);

      d_s[0] = d_s[8];
      d_s[1] = d_s[9];
      d_s[2] = d_s[10];
      d_s[3] = d_s[11];
      d_e[0] = d_e[8];
      d_e[1] = d_e[9];
      d_e[2] = d_e[10];
      d_e[3] = d_e[11];

      d_t += 8 * d_stride;
      y -= 8;
    }

    if (h8 != height) {
      const int16x8_t mask_h = vld1q_s16(&mask_16bit[16] - (height % 8));

      load_s16_4x8(d_t + 4 * d_stride, d_stride, &d_s[4], &d_s[5], &d_s[6],
                   &d_s[7], &d_s[8], &d_s[9], &d_s[10], &d_s[11]);
      load_s16_4x8(d_t + width + 4 * d_stride, d_stride, &d_e[4], &d_e[5],
                   &d_e[6], &d_e[7], &d_e[8], &d_e[9], &d_e[10], &d_e[11]);
      int16x8_t s_tr[8], e_tr[8];
      transpose_elems_s16_4x8(d_s[0], d_s[1], d_s[2], d_s[3], d_s[4], d_s[5],
                              d_s[6], d_s[7], &s_tr[0], &s_tr[1], &s_tr[2],
                              &s_tr[3]);
      transpose_elems_s16_4x8(d_s[8], d_s[9], d_s[10], d_s[11], zeros, zeros,
                              zeros, zeros, &s_tr[4], &s_tr[5], &s_tr[6],
                              &s_tr[7]);
      transpose_elems_s16_4x8(d_e[0], d_e[1], d_e[2], d_e[3], d_e[4], d_e[5],
                              d_e[6], d_e[7], &e_tr[0], &e_tr[1], &e_tr[2],
                              &e_tr[3]);
      transpose_elems_s16_4x8(d_e[8], d_e[9], d_e[10], d_e[11], zeros, zeros,
                              zeros, zeros, &e_tr[4], &e_tr[5], &e_tr[6],
                              &e_tr[7]);

      int16x8_t start_col0[5], start_col1[5], start_col2[5], start_col3[5];
      start_col0[0] = vandq_s16(s_tr[0], mask_h);
      start_col0[1] = vandq_s16(vextq_s16(s_tr[0], s_tr[4], 1), mask_h);
      start_col0[2] = vandq_s16(vextq_s16(s_tr[0], s_tr[4], 2), mask_h);
      start_col0[3] = vandq_s16(vextq_s16(s_tr[0], s_tr[4], 3), mask_h);
      start_col0[4] = vandq_s16(vextq_s16(s_tr[0], s_tr[4], 4), mask_h);

      start_col1[0] = vandq_s16(s_tr[1], mask_h);
      start_col1[1] = vandq_s16(vextq_s16(s_tr[1], s_tr[5], 1), mask_h);
      start_col1[2] = vandq_s16(vextq_s16(s_tr[1], s_tr[5], 2), mask_h);
      start_col1[3] = vandq_s16(vextq_s16(s_tr[1], s_tr[5], 3), mask_h);
      start_col1[4] = vandq_s16(vextq_s16(s_tr[1], s_tr[5], 4), mask_h);

      start_col2[0] = vandq_s16(s_tr[2], mask_h);
      start_col2[1] = vandq_s16(vextq_s16(s_tr[2], s_tr[6], 1), mask_h);
      start_col2[2] = vandq_s16(vextq_s16(s_tr[2], s_tr[6], 2), mask_h);
      start_col2[3] = vandq_s16(vextq_s16(s_tr[2], s_tr[6], 3), mask_h);
      start_col2[4] = vandq_s16(vextq_s16(s_tr[2], s_tr[6], 4), mask_h);

      start_col3[0] = vandq_s16(s_tr[3], mask_h);
      start_col3[1] = vandq_s16(vextq_s16(s_tr[3], s_tr[7], 1), mask_h);
      start_col3[2] = vandq_s16(vextq_s16(s_tr[3], s_tr[7], 2), mask_h);
      start_col3[3] = vandq_s16(vextq_s16(s_tr[3], s_tr[7], 3), mask_h);
      start_col3[4] = vandq_s16(vextq_s16(s_tr[3], s_tr[7], 4), mask_h);

      // i = 1, j = 2;
      sub_deltas_step4_sve(start_col0, start_col1, deltas[0]);

      // i = 1, j = 3;
      sub_deltas_step4_sve(start_col0, start_col2, deltas[1]);

      // i = 1, j = 4
      sub_deltas_step4_sve(start_col0, start_col3, deltas[2]);

      // i = 2, j = 3
      sub_deltas_step4_sve(start_col1, start_col2, deltas[3]);

      // i = 2, j = 4
      sub_deltas_step4_sve(start_col1, start_col3, deltas[4]);

      // i = 3, j = 4
      sub_deltas_step4_sve(start_col2, start_col3, deltas[5]);

      int16x8_t end_col0[5], end_col1[5], end_col2[5], end_col3[5];
      end_col0[0] = vandq_s16(e_tr[0], mask_h);
      end_col0[1] = vandq_s16(vextq_s16(e_tr[0], e_tr[4], 1), mask_h);
      end_col0[2] = vandq_s16(vextq_s16(e_tr[0], e_tr[4], 2), mask_h);
      end_col0[3] = vandq_s16(vextq_s16(e_tr[0], e_tr[4], 3), mask_h);
      end_col0[4] = vandq_s16(vextq_s16(e_tr[0], e_tr[4], 4), mask_h);

      end_col1[0] = vandq_s16(e_tr[1], mask_h);
      end_col1[1] = vandq_s16(vextq_s16(e_tr[1], e_tr[5], 1), mask_h);
      end_col1[2] = vandq_s16(vextq_s16(e_tr[1], e_tr[5], 2), mask_h);
      end_col1[3] = vandq_s16(vextq_s16(e_tr[1], e_tr[5], 3), mask_h);
      end_col1[4] = vandq_s16(vextq_s16(e_tr[1], e_tr[5], 4), mask_h);

      end_col2[0] = vandq_s16(e_tr[2], mask_h);
      end_col2[1] = vandq_s16(vextq_s16(e_tr[2], e_tr[6], 1), mask_h);
      end_col2[2] = vandq_s16(vextq_s16(e_tr[2], e_tr[6], 2), mask_h);
      end_col2[3] = vandq_s16(vextq_s16(e_tr[2], e_tr[6], 3), mask_h);
      end_col2[4] = vandq_s16(vextq_s16(e_tr[2], e_tr[6], 4), mask_h);

      end_col3[0] = vandq_s16(e_tr[3], mask_h);
      end_col3[1] = vandq_s16(vextq_s16(e_tr[3], e_tr[7], 1), mask_h);
      end_col3[2] = vandq_s16(vextq_s16(e_tr[3], e_tr[7], 2), mask_h);
      end_col3[3] = vandq_s16(vextq_s16(e_tr[3], e_tr[7], 3), mask_h);
      end_col3[4] = vandq_s16(vextq_s16(e_tr[3], e_tr[7], 4), mask_h);

      // i = 1, j = 2;
      add_deltas_step4_sve(end_col0, end_col1, deltas[0]);

      // i = 1, j = 3;
      add_deltas_step4_sve(end_col0, end_col2, deltas[1]);

      // i = 1, j = 4
      add_deltas_step4_sve(end_col0, end_col3, deltas[2]);

      // i = 2, j =3
      add_deltas_step4_sve(end_col1, end_col2, deltas[3]);

      // i = 2, j = 4
      add_deltas_step4_sve(end_col1, end_col3, deltas[4]);

      // i = 3, j = 4
      add_deltas_step4_sve(end_col2, end_col3, deltas[5]);
    }

    int64_t single_delta[6];

    deltas[0][0] = vpaddq_s64(deltas[0][0], deltas[0][1]);
    deltas[0][1] = vpaddq_s64(deltas[0][2], deltas[0][3]);
    deltas[1][0] = vpaddq_s64(deltas[1][0], deltas[1][1]);
    deltas[1][1] = vpaddq_s64(deltas[1][2], deltas[1][3]);
    deltas[2][0] = vpaddq_s64(deltas[2][0], deltas[2][1]);
    deltas[2][1] = vpaddq_s64(deltas[2][2], deltas[2][3]);
    deltas[3][0] = vpaddq_s64(deltas[3][0], deltas[3][1]);
    deltas[3][1] = vpaddq_s64(deltas[3][2], deltas[3][3]);
    deltas[4][0] = vpaddq_s64(deltas[4][0], deltas[4][1]);
    deltas[4][1] = vpaddq_s64(deltas[4][2], deltas[4][3]);
    deltas[5][0] = vpaddq_s64(deltas[5][0], deltas[5][1]);
    deltas[5][1] = vpaddq_s64(deltas[5][2], deltas[5][3]);

    deltas[0][5] = vpaddq_s64(deltas[0][5], deltas[0][6]);
    deltas[0][7] = vpaddq_s64(deltas[0][7], deltas[0][8]);
    deltas[1][5] = vpaddq_s64(deltas[1][5], deltas[1][6]);
    deltas[1][7] = vpaddq_s64(deltas[1][7], deltas[1][8]);
    deltas[2][5] = vpaddq_s64(deltas[2][5], deltas[2][6]);
    deltas[2][7] = vpaddq_s64(deltas[2][7], deltas[2][8]);
    deltas[3][5] = vpaddq_s64(deltas[3][5], deltas[3][6]);
    deltas[3][7] = vpaddq_s64(deltas[3][7], deltas[3][8]);
    deltas[4][5] = vpaddq_s64(deltas[4][5], deltas[4][6]);
    deltas[4][7] = vpaddq_s64(deltas[4][7], deltas[4][8]);
    deltas[5][5] = vpaddq_s64(deltas[5][5], deltas[5][6]);
    deltas[5][7] = vpaddq_s64(deltas[5][7], deltas[5][8]);

    vst1q_s64(single_delta + 0, vpaddq_s64(deltas[0][4], deltas[1][4]));
    vst1q_s64(single_delta + 2, vpaddq_s64(deltas[2][4], deltas[3][4]));
    vst1q_s64(single_delta + 4, vpaddq_s64(deltas[4][4], deltas[5][4]));

    int idx = 0;
    for (i = 1; i < wiener_win - 1; i++) {
      for (j = i + 1; j < wiener_win; j++) {
        update_4_stats_sve(
            H + (i - 1) * wiener_win * wiener_win2 + (j - 1) * wiener_win,
            deltas[idx], H + i * wiener_win * wiener_win2 + j * wiener_win);
        H[i * wiener_win * wiener_win2 + j * wiener_win + 4] =
            H[(i - 1) * wiener_win * wiener_win2 + (j - 1) * wiener_win + 4] +
            single_delta[idx];

        H[(i * wiener_win + 1) * wiener_win2 + j * wiener_win] =
            H[((i - 1) * wiener_win + 1) * wiener_win2 + (j - 1) * wiener_win] +
            vgetq_lane_s64(deltas[idx][5], 0);
        H[(i * wiener_win + 2) * wiener_win2 + j * wiener_win] =
            H[((i - 1) * wiener_win + 2) * wiener_win2 + (j - 1) * wiener_win] +
            vgetq_lane_s64(deltas[idx][5], 1);
        H[(i * wiener_win + 3) * wiener_win2 + j * wiener_win] =
            H[((i - 1) * wiener_win + 3) * wiener_win2 + (j - 1) * wiener_win] +
            vgetq_lane_s64(deltas[idx][7], 0);
        H[(i * wiener_win + 4) * wiener_win2 + j * wiener_win] =
            H[((i - 1) * wiener_win + 4) * wiener_win2 + (j - 1) * wiener_win] +
            vgetq_lane_s64(deltas[idx][7], 1);

        idx++;
      }
    }
  }

  // Step 5: Derive other points of each square. No square in bottom row.
  i = 0;
  do {
    const int16_t *const di = d + i;

    j = i + 1;
    do {
      const int16_t *const dj = d + j;
      int64x2_t deltas[WIENER_WIN_CHROMA - 1][WIENER_WIN_CHROMA - 1] = {
        { vdupq_n_s64(0) }, { vdupq_n_s64(0) }
      };
      int16x8_t d_is[WIN_CHROMA], d_ie[WIN_CHROMA];
      int16x8_t d_js[WIN_CHROMA], d_je[WIN_CHROMA];

      x = 0;
      while (x < width - 16) {
        load_square_win5_neon(di + x, dj + x, d_stride, height, d_is, d_ie,
                              d_js, d_je);
        derive_square_win5_sve(d_is, d_ie, d_js, d_je, deltas);
        x += 16;
      }

      load_square_win5_sve(di + x, dj + x, d_stride, height, d_is, d_ie, d_js,
                           d_je, p0, p1);
      derive_square_win5_sve(d_is, d_ie, d_js, d_je, deltas);

      hadd_update_4_stats_sve(
          H + (i * wiener_win + 0) * wiener_win2 + j * wiener_win, deltas[0],
          H + (i * wiener_win + 1) * wiener_win2 + j * wiener_win + 1);
      hadd_update_4_stats_sve(
          H + (i * wiener_win + 1) * wiener_win2 + j * wiener_win, deltas[1],
          H + (i * wiener_win + 2) * wiener_win2 + j * wiener_win + 1);
      hadd_update_4_stats_sve(
          H + (i * wiener_win + 2) * wiener_win2 + j * wiener_win, deltas[2],
          H + (i * wiener_win + 3) * wiener_win2 + j * wiener_win + 1);
      hadd_update_4_stats_sve(
          H + (i * wiener_win + 3) * wiener_win2 + j * wiener_win, deltas[3],
          H + (i * wiener_win + 4) * wiener_win2 + j * wiener_win + 1);
    } while (++j < wiener_win);
  } while (++i < wiener_win - 1);

  // Step 6: Derive other points of each upper triangle along the diagonal.
  i = 0;
  do {
    const int16_t *const di = d + i;
    int64x2_t deltas[WIENER_WIN_CHROMA * 2 + 1] = { vdupq_n_s64(0) };
    int16x8_t d_is[WIN_CHROMA], d_ie[WIN_CHROMA];

    x = 0;
    while (x < width - 16) {
      load_triangle_win5_neon(di + x, d_stride, height, d_is, d_ie);
      derive_triangle_win5_sve(d_is, d_ie, deltas);
      x += 16;
    }

    load_triangle_win5_sve(di + x, d_stride, height, d_is, d_ie, p0, p1);
    derive_triangle_win5_sve(d_is, d_ie, deltas);

    // Row 1: 4 points
    hadd_update_4_stats_sve(
        H + (i * wiener_win + 0) * wiener_win2 + i * wiener_win, deltas,
        H + (i * wiener_win + 1) * wiener_win2 + i * wiener_win + 1);

    // Row 2: 3 points
    int64x2_t src0 =
        vld1q_s64(H + (i * wiener_win + 1) * wiener_win2 + i * wiener_win + 1);
    vst1q_s64(H + (i * wiener_win + 2) * wiener_win2 + i * wiener_win + 2,
              vaddq_s64(src0, vpaddq_s64(deltas[4], deltas[5])));

    int64x2_t deltas69 = vpaddq_s64(deltas[6], deltas[9]);

    H[(i * wiener_win + 2) * wiener_win2 + i * wiener_win + 4] =
        H[(i * wiener_win + 1) * wiener_win2 + i * wiener_win + 3] +
        vgetq_lane_s64(deltas69, 0);

    // Row 3: 2 points
    int64x2_t src1 =
        vld1q_s64(H + (i * wiener_win + 2) * wiener_win2 + i * wiener_win + 2);
    vst1q_s64(H + (i * wiener_win + 3) * wiener_win2 + i * wiener_win + 3,
              vaddq_s64(src1, vpaddq_s64(deltas[7], deltas[8])));

    // Row 4: 1 point
    H[(i * wiener_win + 4) * wiener_win2 + i * wiener_win + 4] =
        H[(i * wiener_win + 3) * wiener_win2 + i * wiener_win + 3] +
        vgetq_lane_s64(deltas69, 1);
  } while (++i < wiener_win);
}

static inline void stats_top_win7_sve(const int16x8_t src[2],
                                      const int16x8_t dgd[2],
                                      const int16_t *const d,
                                      const int32_t d_stride, int64x2_t *sum_m,
                                      int64x2_t *sum_h) {
  int16x8_t dgds[WIENER_WIN * 2];

  load_s16_8x7(d + 0, d_stride, &dgds[0], &dgds[2], &dgds[4], &dgds[6],
               &dgds[8], &dgds[10], &dgds[12]);
  load_s16_8x7(d + 8, d_stride, &dgds[1], &dgds[3], &dgds[5], &dgds[7],
               &dgds[9], &dgds[11], &dgds[13]);

  sum_m[0] = aom_sdotq_s16(sum_m[0], src[0], dgds[0]);
  sum_m[0] = aom_sdotq_s16(sum_m[0], src[1], dgds[1]);
  sum_m[1] = aom_sdotq_s16(sum_m[1], src[0], dgds[2]);
  sum_m[1] = aom_sdotq_s16(sum_m[1], src[1], dgds[3]);
  sum_m[2] = aom_sdotq_s16(sum_m[2], src[0], dgds[4]);
  sum_m[2] = aom_sdotq_s16(sum_m[2], src[1], dgds[5]);
  sum_m[3] = aom_sdotq_s16(sum_m[3], src[0], dgds[6]);
  sum_m[3] = aom_sdotq_s16(sum_m[3], src[1], dgds[7]);
  sum_m[4] = aom_sdotq_s16(sum_m[4], src[0], dgds[8]);
  sum_m[4] = aom_sdotq_s16(sum_m[4], src[1], dgds[9]);
  sum_m[5] = aom_sdotq_s16(sum_m[5], src[0], dgds[10]);
  sum_m[5] = aom_sdotq_s16(sum_m[5], src[1], dgds[11]);
  sum_m[6] = aom_sdotq_s16(sum_m[6], src[0], dgds[12]);
  sum_m[6] = aom_sdotq_s16(sum_m[6], src[1], dgds[13]);

  sum_h[0] = aom_sdotq_s16(sum_h[0], dgd[0], dgds[0]);
  sum_h[0] = aom_sdotq_s16(sum_h[0], dgd[1], dgds[1]);
  sum_h[1] = aom_sdotq_s16(sum_h[1], dgd[0], dgds[2]);
  sum_h[1] = aom_sdotq_s16(sum_h[1], dgd[1], dgds[3]);
  sum_h[2] = aom_sdotq_s16(sum_h[2], dgd[0], dgds[4]);
  sum_h[2] = aom_sdotq_s16(sum_h[2], dgd[1], dgds[5]);
  sum_h[3] = aom_sdotq_s16(sum_h[3], dgd[0], dgds[6]);
  sum_h[3] = aom_sdotq_s16(sum_h[3], dgd[1], dgds[7]);
  sum_h[4] = aom_sdotq_s16(sum_h[4], dgd[0], dgds[8]);
  sum_h[4] = aom_sdotq_s16(sum_h[4], dgd[1], dgds[9]);
  sum_h[5] = aom_sdotq_s16(sum_h[5], dgd[0], dgds[10]);
  sum_h[5] = aom_sdotq_s16(sum_h[5], dgd[1], dgds[11]);
  sum_h[6] = aom_sdotq_s16(sum_h[6], dgd[0], dgds[12]);
  sum_h[6] = aom_sdotq_s16(sum_h[6], dgd[1], dgds[13]);
}

static inline void stats_left_win7_sve(const int16x8_t src[2], const int16_t *d,
                                       const int32_t d_stride, int64x2_t *sum) {
  int16x8_t dgds[WIN_7];

  load_s16_8x6(d + d_stride + 0, d_stride, &dgds[0], &dgds[2], &dgds[4],
               &dgds[6], &dgds[8], &dgds[10]);
  load_s16_8x6(d + d_stride + 8, d_stride, &dgds[1], &dgds[3], &dgds[5],
               &dgds[7], &dgds[9], &dgds[11]);

  sum[0] = aom_sdotq_s16(sum[0], src[0], dgds[0]);
  sum[0] = aom_sdotq_s16(sum[0], src[1], dgds[1]);
  sum[1] = aom_sdotq_s16(sum[1], src[0], dgds[2]);
  sum[1] = aom_sdotq_s16(sum[1], src[1], dgds[3]);
  sum[2] = aom_sdotq_s16(sum[2], src[0], dgds[4]);
  sum[2] = aom_sdotq_s16(sum[2], src[1], dgds[5]);
  sum[3] = aom_sdotq_s16(sum[3], src[0], dgds[6]);
  sum[3] = aom_sdotq_s16(sum[3], src[1], dgds[7]);
  sum[4] = aom_sdotq_s16(sum[4], src[0], dgds[8]);
  sum[4] = aom_sdotq_s16(sum[4], src[1], dgds[9]);
  sum[5] = aom_sdotq_s16(sum[5], src[0], dgds[10]);
  sum[5] = aom_sdotq_s16(sum[5], src[1], dgds[11]);
}

static inline void load_square_win7_sve(
    const int16_t *const di, const int16_t *const dj, const int32_t d_stride,
    const int32_t height, int16x8_t *d_is, int16x8_t *d_ie, int16x8_t *d_js,
    int16x8_t *d_je, svbool_t p0, svbool_t p1) {
  d_is[0] = svget_neonq_s16(svld1_s16(p0, di + 0 * d_stride + 0));
  d_is[1] = svget_neonq_s16(svld1_s16(p1, di + 0 * d_stride + 8));
  d_is[2] = svget_neonq_s16(svld1_s16(p0, di + 1 * d_stride + 0));
  d_is[3] = svget_neonq_s16(svld1_s16(p1, di + 1 * d_stride + 8));
  d_is[4] = svget_neonq_s16(svld1_s16(p0, di + 2 * d_stride + 0));
  d_is[5] = svget_neonq_s16(svld1_s16(p1, di + 2 * d_stride + 8));
  d_is[6] = svget_neonq_s16(svld1_s16(p0, di + 3 * d_stride + 0));
  d_is[7] = svget_neonq_s16(svld1_s16(p1, di + 3 * d_stride + 8));
  d_is[8] = svget_neonq_s16(svld1_s16(p0, di + 4 * d_stride + 0));
  d_is[9] = svget_neonq_s16(svld1_s16(p1, di + 4 * d_stride + 8));
  d_is[10] = svget_neonq_s16(svld1_s16(p0, di + 5 * d_stride + 0));
  d_is[11] = svget_neonq_s16(svld1_s16(p1, di + 5 * d_stride + 8));

  d_ie[0] = svget_neonq_s16(svld1_s16(p0, di + (height + 0) * d_stride + 0));
  d_ie[1] = svget_neonq_s16(svld1_s16(p1, di + (height + 0) * d_stride + 8));
  d_ie[2] = svget_neonq_s16(svld1_s16(p0, di + (height + 1) * d_stride + 0));
  d_ie[3] = svget_neonq_s16(svld1_s16(p1, di + (height + 1) * d_stride + 8));
  d_ie[4] = svget_neonq_s16(svld1_s16(p0, di + (height + 2) * d_stride + 0));
  d_ie[5] = svget_neonq_s16(svld1_s16(p1, di + (height + 2) * d_stride + 8));
  d_ie[6] = svget_neonq_s16(svld1_s16(p0, di + (height + 3) * d_stride + 0));
  d_ie[7] = svget_neonq_s16(svld1_s16(p1, di + (height + 3) * d_stride + 8));
  d_ie[8] = svget_neonq_s16(svld1_s16(p0, di + (height + 4) * d_stride + 0));
  d_ie[9] = svget_neonq_s16(svld1_s16(p1, di + (height + 4) * d_stride + 8));
  d_ie[10] = svget_neonq_s16(svld1_s16(p0, di + (height + 5) * d_stride + 0));
  d_ie[11] = svget_neonq_s16(svld1_s16(p1, di + (height + 5) * d_stride + 8));

  load_s16_8x6(dj + 0, d_stride, &d_js[0], &d_js[2], &d_js[4], &d_js[6],
               &d_js[8], &d_js[10]);
  load_s16_8x6(dj + 8, d_stride, &d_js[1], &d_js[3], &d_js[5], &d_js[7],
               &d_js[9], &d_js[11]);
  load_s16_8x6(dj + height * d_stride + 0, d_stride, &d_je[0], &d_je[2],
               &d_je[4], &d_je[6], &d_je[8], &d_je[10]);
  load_s16_8x6(dj + height * d_stride + 8, d_stride, &d_je[1], &d_je[3],
               &d_je[5], &d_je[7], &d_je[9], &d_je[11]);
}

static inline void derive_square_win7_sve(int16x8_t *d_is,
                                          const int16x8_t *d_ie,
                                          const int16x8_t *d_js,
                                          const int16x8_t *d_je,
                                          int64x2_t deltas[][WIN_7]) {
  d_is[0] = vnegq_s16(d_is[0]);
  d_is[1] = vnegq_s16(d_is[1]);
  d_is[2] = vnegq_s16(d_is[2]);
  d_is[3] = vnegq_s16(d_is[3]);
  d_is[4] = vnegq_s16(d_is[4]);
  d_is[5] = vnegq_s16(d_is[5]);
  d_is[6] = vnegq_s16(d_is[6]);
  d_is[7] = vnegq_s16(d_is[7]);
  d_is[8] = vnegq_s16(d_is[8]);
  d_is[9] = vnegq_s16(d_is[9]);
  d_is[10] = vnegq_s16(d_is[10]);
  d_is[11] = vnegq_s16(d_is[11]);

  deltas[0][0] = aom_sdotq_s16(deltas[0][0], d_is[0], d_js[0]);
  deltas[0][0] = aom_sdotq_s16(deltas[0][0], d_is[1], d_js[1]);
  deltas[0][1] = aom_sdotq_s16(deltas[0][1], d_is[0], d_js[2]);
  deltas[0][1] = aom_sdotq_s16(deltas[0][1], d_is[1], d_js[3]);
  deltas[0][2] = aom_sdotq_s16(deltas[0][2], d_is[0], d_js[4]);
  deltas[0][2] = aom_sdotq_s16(deltas[0][2], d_is[1], d_js[5]);
  deltas[0][3] = aom_sdotq_s16(deltas[0][3], d_is[0], d_js[6]);
  deltas[0][3] = aom_sdotq_s16(deltas[0][3], d_is[1], d_js[7]);
  deltas[0][4] = aom_sdotq_s16(deltas[0][4], d_is[0], d_js[8]);
  deltas[0][4] = aom_sdotq_s16(deltas[0][4], d_is[1], d_js[9]);
  deltas[0][5] = aom_sdotq_s16(deltas[0][5], d_is[0], d_js[10]);
  deltas[0][5] = aom_sdotq_s16(deltas[0][5], d_is[1], d_js[11]);

  deltas[1][0] = aom_sdotq_s16(deltas[1][0], d_is[2], d_js[0]);
  deltas[1][0] = aom_sdotq_s16(deltas[1][0], d_is[3], d_js[1]);
  deltas[1][1] = aom_sdotq_s16(deltas[1][1], d_is[2], d_js[2]);
  deltas[1][1] = aom_sdotq_s16(deltas[1][1], d_is[3], d_js[3]);
  deltas[1][2] = aom_sdotq_s16(deltas[1][2], d_is[2], d_js[4]);
  deltas[1][2] = aom_sdotq_s16(deltas[1][2], d_is[3], d_js[5]);
  deltas[1][3] = aom_sdotq_s16(deltas[1][3], d_is[2], d_js[6]);
  deltas[1][3] = aom_sdotq_s16(deltas[1][3], d_is[3], d_js[7]);
  deltas[1][4] = aom_sdotq_s16(deltas[1][4], d_is[2], d_js[8]);
  deltas[1][4] = aom_sdotq_s16(deltas[1][4], d_is[3], d_js[9]);
  deltas[1][5] = aom_sdotq_s16(deltas[1][5], d_is[2], d_js[10]);
  deltas[1][5] = aom_sdotq_s16(deltas[1][5], d_is[3], d_js[11]);

  deltas[2][0] = aom_sdotq_s16(deltas[2][0], d_is[4], d_js[0]);
  deltas[2][0] = aom_sdotq_s16(deltas[2][0], d_is[5], d_js[1]);
  deltas[2][1] = aom_sdotq_s16(deltas[2][1], d_is[4], d_js[2]);
  deltas[2][1] = aom_sdotq_s16(deltas[2][1], d_is[5], d_js[3]);
  deltas[2][2] = aom_sdotq_s16(deltas[2][2], d_is[4], d_js[4]);
  deltas[2][2] = aom_sdotq_s16(deltas[2][2], d_is[5], d_js[5]);
  deltas[2][3] = aom_sdotq_s16(deltas[2][3], d_is[4], d_js[6]);
  deltas[2][3] = aom_sdotq_s16(deltas[2][3], d_is[5], d_js[7]);
  deltas[2][4] = aom_sdotq_s16(deltas[2][4], d_is[4], d_js[8]);
  deltas[2][4] = aom_sdotq_s16(deltas[2][4], d_is[5], d_js[9]);
  deltas[2][5] = aom_sdotq_s16(deltas[2][5], d_is[4], d_js[10]);
  deltas[2][5] = aom_sdotq_s16(deltas[2][5], d_is[5], d_js[11]);

  deltas[3][0] = aom_sdotq_s16(deltas[3][0], d_is[6], d_js[0]);
  deltas[3][0] = aom_sdotq_s16(deltas[3][0], d_is[7], d_js[1]);
  deltas[3][1] = aom_sdotq_s16(deltas[3][1], d_is[6], d_js[2]);
  deltas[3][1] = aom_sdotq_s16(deltas[3][1], d_is[7], d_js[3]);
  deltas[3][2] = aom_sdotq_s16(deltas[3][2], d_is[6], d_js[4]);
  deltas[3][2] = aom_sdotq_s16(deltas[3][2], d_is[7], d_js[5]);
  deltas[3][3] = aom_sdotq_s16(deltas[3][3], d_is[6], d_js[6]);
  deltas[3][3] = aom_sdotq_s16(deltas[3][3], d_is[7], d_js[7]);
  deltas[3][4] = aom_sdotq_s16(deltas[3][4], d_is[6], d_js[8]);
  deltas[3][4] = aom_sdotq_s16(deltas[3][4], d_is[7], d_js[9]);
  deltas[3][5] = aom_sdotq_s16(deltas[3][5], d_is[6], d_js[10]);
  deltas[3][5] = aom_sdotq_s16(deltas[3][5], d_is[7], d_js[11]);

  deltas[4][0] = aom_sdotq_s16(deltas[4][0], d_is[8], d_js[0]);
  deltas[4][0] = aom_sdotq_s16(deltas[4][0], d_is[9], d_js[1]);
  deltas[4][1] = aom_sdotq_s16(deltas[4][1], d_is[8], d_js[2]);
  deltas[4][1] = aom_sdotq_s16(deltas[4][1], d_is[9], d_js[3]);
  deltas[4][2] = aom_sdotq_s16(deltas[4][2], d_is[8], d_js[4]);
  deltas[4][2] = aom_sdotq_s16(deltas[4][2], d_is[9], d_js[5]);
  deltas[4][3] = aom_sdotq_s16(deltas[4][3], d_is[8], d_js[6]);
  deltas[4][3] = aom_sdotq_s16(deltas[4][3], d_is[9], d_js[7]);
  deltas[4][4] = aom_sdotq_s16(deltas[4][4], d_is[8], d_js[8]);
  deltas[4][4] = aom_sdotq_s16(deltas[4][4], d_is[9], d_js[9]);
  deltas[4][5] = aom_sdotq_s16(deltas[4][5], d_is[8], d_js[10]);
  deltas[4][5] = aom_sdotq_s16(deltas[4][5], d_is[9], d_js[11]);

  deltas[5][0] = aom_sdotq_s16(deltas[5][0], d_is[10], d_js[0]);
  deltas[5][0] = aom_sdotq_s16(deltas[5][0], d_is[11], d_js[1]);
  deltas[5][1] = aom_sdotq_s16(deltas[5][1], d_is[10], d_js[2]);
  deltas[5][1] = aom_sdotq_s16(deltas[5][1], d_is[11], d_js[3]);
  deltas[5][2] = aom_sdotq_s16(deltas[5][2], d_is[10], d_js[4]);
  deltas[5][2] = aom_sdotq_s16(deltas[5][2], d_is[11], d_js[5]);
  deltas[5][3] = aom_sdotq_s16(deltas[5][3], d_is[10], d_js[6]);
  deltas[5][3] = aom_sdotq_s16(deltas[5][3], d_is[11], d_js[7]);
  deltas[5][4] = aom_sdotq_s16(deltas[5][4], d_is[10], d_js[8]);
  deltas[5][4] = aom_sdotq_s16(deltas[5][4], d_is[11], d_js[9]);
  deltas[5][5] = aom_sdotq_s16(deltas[5][5], d_is[10], d_js[10]);
  deltas[5][5] = aom_sdotq_s16(deltas[5][5], d_is[11], d_js[11]);

  deltas[0][0] = aom_sdotq_s16(deltas[0][0], d_ie[0], d_je[0]);
  deltas[0][0] = aom_sdotq_s16(deltas[0][0], d_ie[1], d_je[1]);
  deltas[0][1] = aom_sdotq_s16(deltas[0][1], d_ie[0], d_je[2]);
  deltas[0][1] = aom_sdotq_s16(deltas[0][1], d_ie[1], d_je[3]);
  deltas[0][2] = aom_sdotq_s16(deltas[0][2], d_ie[0], d_je[4]);
  deltas[0][2] = aom_sdotq_s16(deltas[0][2], d_ie[1], d_je[5]);
  deltas[0][3] = aom_sdotq_s16(deltas[0][3], d_ie[0], d_je[6]);
  deltas[0][3] = aom_sdotq_s16(deltas[0][3], d_ie[1], d_je[7]);
  deltas[0][4] = aom_sdotq_s16(deltas[0][4], d_ie[0], d_je[8]);
  deltas[0][4] = aom_sdotq_s16(deltas[0][4], d_ie[1], d_je[9]);
  deltas[0][5] = aom_sdotq_s16(deltas[0][5], d_ie[0], d_je[10]);
  deltas[0][5] = aom_sdotq_s16(deltas[0][5], d_ie[1], d_je[11]);

  deltas[1][0] = aom_sdotq_s16(deltas[1][0], d_ie[2], d_je[0]);
  deltas[1][0] = aom_sdotq_s16(deltas[1][0], d_ie[3], d_je[1]);
  deltas[1][1] = aom_sdotq_s16(deltas[1][1], d_ie[2], d_je[2]);
  deltas[1][1] = aom_sdotq_s16(deltas[1][1], d_ie[3], d_je[3]);
  deltas[1][2] = aom_sdotq_s16(deltas[1][2], d_ie[2], d_je[4]);
  deltas[1][2] = aom_sdotq_s16(deltas[1][2], d_ie[3], d_je[5]);
  deltas[1][3] = aom_sdotq_s16(deltas[1][3], d_ie[2], d_je[6]);
  deltas[1][3] = aom_sdotq_s16(deltas[1][3], d_ie[3], d_je[7]);
  deltas[1][4] = aom_sdotq_s16(deltas[1][4], d_ie[2], d_je[8]);
  deltas[1][4] = aom_sdotq_s16(deltas[1][4], d_ie[3], d_je[9]);
  deltas[1][5] = aom_sdotq_s16(deltas[1][5], d_ie[2], d_je[10]);
  deltas[1][5] = aom_sdotq_s16(deltas[1][5], d_ie[3], d_je[11]);

  deltas[2][0] = aom_sdotq_s16(deltas[2][0], d_ie[4], d_je[0]);
  deltas[2][0] = aom_sdotq_s16(deltas[2][0], d_ie[5], d_je[1]);
  deltas[2][1] = aom_sdotq_s16(deltas[2][1], d_ie[4], d_je[2]);
  deltas[2][1] = aom_sdotq_s16(deltas[2][1], d_ie[5], d_je[3]);
  deltas[2][2] = aom_sdotq_s16(deltas[2][2], d_ie[4], d_je[4]);
  deltas[2][2] = aom_sdotq_s16(deltas[2][2], d_ie[5], d_je[5]);
  deltas[2][3] = aom_sdotq_s16(deltas[2][3], d_ie[4], d_je[6]);
  deltas[2][3] = aom_sdotq_s16(deltas[2][3], d_ie[5], d_je[7]);
  deltas[2][4] = aom_sdotq_s16(deltas[2][4], d_ie[4], d_je[8]);
  deltas[2][4] = aom_sdotq_s16(deltas[2][4], d_ie[5], d_je[9]);
  deltas[2][5] = aom_sdotq_s16(deltas[2][5], d_ie[4], d_je[10]);
  deltas[2][5] = aom_sdotq_s16(deltas[2][5], d_ie[5], d_je[11]);

  deltas[3][0] = aom_sdotq_s16(deltas[3][0], d_ie[6], d_je[0]);
  deltas[3][0] = aom_sdotq_s16(deltas[3][0], d_ie[7], d_je[1]);
  deltas[3][1] = aom_sdotq_s16(deltas[3][1], d_ie[6], d_je[2]);
  deltas[3][1] = aom_sdotq_s16(deltas[3][1], d_ie[7], d_je[3]);
  deltas[3][2] = aom_sdotq_s16(deltas[3][2], d_ie[6], d_je[4]);
  deltas[3][2] = aom_sdotq_s16(deltas[3][2], d_ie[7], d_je[5]);
  deltas[3][3] = aom_sdotq_s16(deltas[3][3], d_ie[6], d_je[6]);
  deltas[3][3] = aom_sdotq_s16(deltas[3][3], d_ie[7], d_je[7]);
  deltas[3][4] = aom_sdotq_s16(deltas[3][4], d_ie[6], d_je[8]);
  deltas[3][4] = aom_sdotq_s16(deltas[3][4], d_ie[7], d_je[9]);
  deltas[3][5] = aom_sdotq_s16(deltas[3][5], d_ie[6], d_je[10]);
  deltas[3][5] = aom_sdotq_s16(deltas[3][5], d_ie[7], d_je[11]);

  deltas[4][0] = aom_sdotq_s16(deltas[4][0], d_ie[8], d_je[0]);
  deltas[4][0] = aom_sdotq_s16(deltas[4][0], d_ie[9], d_je[1]);
  deltas[4][1] = aom_sdotq_s16(deltas[4][1], d_ie[8], d_je[2]);
  deltas[4][1] = aom_sdotq_s16(deltas[4][1], d_ie[9], d_je[3]);
  deltas[4][2] = aom_sdotq_s16(deltas[4][2], d_ie[8], d_je[4]);
  deltas[4][2] = aom_sdotq_s16(deltas[4][2], d_ie[9], d_je[5]);
  deltas[4][3] = aom_sdotq_s16(deltas[4][3], d_ie[8], d_je[6]);
  deltas[4][3] = aom_sdotq_s16(deltas[4][3], d_ie[9], d_je[7]);
  deltas[4][4] = aom_sdotq_s16(deltas[4][4], d_ie[8], d_je[8]);
  deltas[4][4] = aom_sdotq_s16(deltas[4][4], d_ie[9], d_je[9]);
  deltas[4][5] = aom_sdotq_s16(deltas[4][5], d_ie[8], d_je[10]);
  deltas[4][5] = aom_sdotq_s16(deltas[4][5], d_ie[9], d_je[11]);

  deltas[5][0] = aom_sdotq_s16(deltas[5][0], d_ie[10], d_je[0]);
  deltas[5][0] = aom_sdotq_s16(deltas[5][0], d_ie[11], d_je[1]);
  deltas[5][1] = aom_sdotq_s16(deltas[5][1], d_ie[10], d_je[2]);
  deltas[5][1] = aom_sdotq_s16(deltas[5][1], d_ie[11], d_je[3]);
  deltas[5][2] = aom_sdotq_s16(deltas[5][2], d_ie[10], d_je[4]);
  deltas[5][2] = aom_sdotq_s16(deltas[5][2], d_ie[11], d_je[5]);
  deltas[5][3] = aom_sdotq_s16(deltas[5][3], d_ie[10], d_je[6]);
  deltas[5][3] = aom_sdotq_s16(deltas[5][3], d_ie[11], d_je[7]);
  deltas[5][4] = aom_sdotq_s16(deltas[5][4], d_ie[10], d_je[8]);
  deltas[5][4] = aom_sdotq_s16(deltas[5][4], d_ie[11], d_je[9]);
  deltas[5][5] = aom_sdotq_s16(deltas[5][5], d_ie[10], d_je[10]);
  deltas[5][5] = aom_sdotq_s16(deltas[5][5], d_ie[11], d_je[11]);
}

static inline void hadd_update_6_stats_sve(const int64_t *const src,
                                           const int64x2_t *deltas,
                                           int64_t *const dst) {
  int64x2_t src0 = vld1q_s64(src + 0);
  int64x2_t src1 = vld1q_s64(src + 2);
  int64x2_t src2 = vld1q_s64(src + 4);

  int64x2_t deltas01 = vpaddq_s64(deltas[0], deltas[1]);
  int64x2_t deltas23 = vpaddq_s64(deltas[2], deltas[3]);
  int64x2_t deltas45 = vpaddq_s64(deltas[4], deltas[5]);

  vst1q_s64(dst + 0, vaddq_s64(src0, deltas01));
  vst1q_s64(dst + 2, vaddq_s64(src1, deltas23));
  vst1q_s64(dst + 4, vaddq_s64(src2, deltas45));
}

static inline void load_triangle_win7_sve(const int16_t *const di,
                                          const int32_t d_stride,
                                          const int32_t height, int16x8_t *d_is,
                                          int16x8_t *d_ie, svbool_t p0,
                                          svbool_t p1) {
  d_is[0] = svget_neonq_s16(svld1_s16(p0, di + 0 * d_stride + 0));
  d_is[1] = svget_neonq_s16(svld1_s16(p1, di + 0 * d_stride + 8));
  d_is[2] = svget_neonq_s16(svld1_s16(p0, di + 1 * d_stride + 0));
  d_is[3] = svget_neonq_s16(svld1_s16(p1, di + 1 * d_stride + 8));
  d_is[4] = svget_neonq_s16(svld1_s16(p0, di + 2 * d_stride + 0));
  d_is[5] = svget_neonq_s16(svld1_s16(p1, di + 2 * d_stride + 8));
  d_is[6] = svget_neonq_s16(svld1_s16(p0, di + 3 * d_stride + 0));
  d_is[7] = svget_neonq_s16(svld1_s16(p1, di + 3 * d_stride + 8));
  d_is[8] = svget_neonq_s16(svld1_s16(p0, di + 4 * d_stride + 0));
  d_is[9] = svget_neonq_s16(svld1_s16(p1, di + 4 * d_stride + 8));
  d_is[10] = svget_neonq_s16(svld1_s16(p0, di + 5 * d_stride + 0));
  d_is[11] = svget_neonq_s16(svld1_s16(p1, di + 5 * d_stride + 8));

  d_ie[0] = svget_neonq_s16(svld1_s16(p0, di + (height + 0) * d_stride + 0));
  d_ie[1] = svget_neonq_s16(svld1_s16(p1, di + (height + 0) * d_stride + 8));
  d_ie[2] = svget_neonq_s16(svld1_s16(p0, di + (height + 1) * d_stride + 0));
  d_ie[3] = svget_neonq_s16(svld1_s16(p1, di + (height + 1) * d_stride + 8));
  d_ie[4] = svget_neonq_s16(svld1_s16(p0, di + (height + 2) * d_stride + 0));
  d_ie[5] = svget_neonq_s16(svld1_s16(p1, di + (height + 2) * d_stride + 8));
  d_ie[6] = svget_neonq_s16(svld1_s16(p0, di + (height + 3) * d_stride + 0));
  d_ie[7] = svget_neonq_s16(svld1_s16(p1, di + (height + 3) * d_stride + 8));
  d_ie[8] = svget_neonq_s16(svld1_s16(p0, di + (height + 4) * d_stride + 0));
  d_ie[9] = svget_neonq_s16(svld1_s16(p1, di + (height + 4) * d_stride + 8));
  d_ie[10] = svget_neonq_s16(svld1_s16(p0, di + (height + 5) * d_stride + 0));
  d_ie[11] = svget_neonq_s16(svld1_s16(p1, di + (height + 5) * d_stride + 8));
}

static inline void derive_triangle_win7_sve(const int16x8_t *d_is,
                                            const int16x8_t *d_ie,
                                            int64x2_t *deltas) {
  deltas[0] = aom_sdotq_s16(deltas[0], vnegq_s16(d_is[0]), d_is[0]);
  deltas[0] = aom_sdotq_s16(deltas[0], vnegq_s16(d_is[1]), d_is[1]);
  deltas[1] = aom_sdotq_s16(deltas[1], vnegq_s16(d_is[0]), d_is[2]);
  deltas[1] = aom_sdotq_s16(deltas[1], vnegq_s16(d_is[1]), d_is[3]);
  deltas[2] = aom_sdotq_s16(deltas[2], vnegq_s16(d_is[0]), d_is[4]);
  deltas[2] = aom_sdotq_s16(deltas[2], vnegq_s16(d_is[1]), d_is[5]);
  deltas[3] = aom_sdotq_s16(deltas[3], vnegq_s16(d_is[0]), d_is[6]);
  deltas[3] = aom_sdotq_s16(deltas[3], vnegq_s16(d_is[1]), d_is[7]);
  deltas[4] = aom_sdotq_s16(deltas[4], vnegq_s16(d_is[0]), d_is[8]);
  deltas[4] = aom_sdotq_s16(deltas[4], vnegq_s16(d_is[1]), d_is[9]);
  deltas[5] = aom_sdotq_s16(deltas[5], vnegq_s16(d_is[0]), d_is[10]);
  deltas[5] = aom_sdotq_s16(deltas[5], vnegq_s16(d_is[1]), d_is[11]);

  deltas[6] = aom_sdotq_s16(deltas[6], vnegq_s16(d_is[2]), d_is[2]);
  deltas[6] = aom_sdotq_s16(deltas[6], vnegq_s16(d_is[3]), d_is[3]);
  deltas[7] = aom_sdotq_s16(deltas[7], vnegq_s16(d_is[2]), d_is[4]);
  deltas[7] = aom_sdotq_s16(deltas[7], vnegq_s16(d_is[3]), d_is[5]);
  deltas[8] = aom_sdotq_s16(deltas[8], vnegq_s16(d_is[2]), d_is[6]);
  deltas[8] = aom_sdotq_s16(deltas[8], vnegq_s16(d_is[3]), d_is[7]);
  deltas[9] = aom_sdotq_s16(deltas[9], vnegq_s16(d_is[2]), d_is[8]);
  deltas[9] = aom_sdotq_s16(deltas[9], vnegq_s16(d_is[3]), d_is[9]);
  deltas[10] = aom_sdotq_s16(deltas[10], vnegq_s16(d_is[2]), d_is[10]);
  deltas[10] = aom_sdotq_s16(deltas[10], vnegq_s16(d_is[3]), d_is[11]);

  deltas[11] = aom_sdotq_s16(deltas[11], vnegq_s16(d_is[4]), d_is[4]);
  deltas[11] = aom_sdotq_s16(deltas[11], vnegq_s16(d_is[5]), d_is[5]);
  deltas[12] = aom_sdotq_s16(deltas[12], vnegq_s16(d_is[4]), d_is[6]);
  deltas[12] = aom_sdotq_s16(deltas[12], vnegq_s16(d_is[5]), d_is[7]);
  deltas[13] = aom_sdotq_s16(deltas[13], vnegq_s16(d_is[4]), d_is[8]);
  deltas[13] = aom_sdotq_s16(deltas[13], vnegq_s16(d_is[5]), d_is[9]);
  deltas[14] = aom_sdotq_s16(deltas[14], vnegq_s16(d_is[4]), d_is[10]);
  deltas[14] = aom_sdotq_s16(deltas[14], vnegq_s16(d_is[5]), d_is[11]);

  deltas[15] = aom_sdotq_s16(deltas[15], vnegq_s16(d_is[6]), d_is[6]);
  deltas[15] = aom_sdotq_s16(deltas[15], vnegq_s16(d_is[7]), d_is[7]);
  deltas[16] = aom_sdotq_s16(deltas[16], vnegq_s16(d_is[6]), d_is[8]);
  deltas[16] = aom_sdotq_s16(deltas[16], vnegq_s16(d_is[7]), d_is[9]);
  deltas[17] = aom_sdotq_s16(deltas[17], vnegq_s16(d_is[6]), d_is[10]);
  deltas[17] = aom_sdotq_s16(deltas[17], vnegq_s16(d_is[7]), d_is[11]);

  deltas[18] = aom_sdotq_s16(deltas[18], vnegq_s16(d_is[8]), d_is[8]);
  deltas[18] = aom_sdotq_s16(deltas[18], vnegq_s16(d_is[9]), d_is[9]);
  deltas[19] = aom_sdotq_s16(deltas[19], vnegq_s16(d_is[8]), d_is[10]);
  deltas[19] = aom_sdotq_s16(deltas[19], vnegq_s16(d_is[9]), d_is[11]);

  deltas[20] = aom_sdotq_s16(deltas[20], vnegq_s16(d_is[10]), d_is[10]);
  deltas[20] = aom_sdotq_s16(deltas[20], vnegq_s16(d_is[11]), d_is[11]);

  deltas[0] = aom_sdotq_s16(deltas[0], d_ie[0], d_ie[0]);
  deltas[0] = aom_sdotq_s16(deltas[0], d_ie[1], d_ie[1]);
  deltas[1] = aom_sdotq_s16(deltas[1], d_ie[0], d_ie[2]);
  deltas[1] = aom_sdotq_s16(deltas[1], d_ie[1], d_ie[3]);
  deltas[2] = aom_sdotq_s16(deltas[2], d_ie[0], d_ie[4]);
  deltas[2] = aom_sdotq_s16(deltas[2], d_ie[1], d_ie[5]);
  deltas[3] = aom_sdotq_s16(deltas[3], d_ie[0], d_ie[6]);
  deltas[3] = aom_sdotq_s16(deltas[3], d_ie[1], d_ie[7]);
  deltas[4] = aom_sdotq_s16(deltas[4], d_ie[0], d_ie[8]);
  deltas[4] = aom_sdotq_s16(deltas[4], d_ie[1], d_ie[9]);
  deltas[5] = aom_sdotq_s16(deltas[5], d_ie[0], d_ie[10]);
  deltas[5] = aom_sdotq_s16(deltas[5], d_ie[1], d_ie[11]);

  deltas[6] = aom_sdotq_s16(deltas[6], d_ie[2], d_ie[2]);
  deltas[6] = aom_sdotq_s16(deltas[6], d_ie[3], d_ie[3]);
  deltas[7] = aom_sdotq_s16(deltas[7], d_ie[2], d_ie[4]);
  deltas[7] = aom_sdotq_s16(deltas[7], d_ie[3], d_ie[5]);
  deltas[8] = aom_sdotq_s16(deltas[8], d_ie[2], d_ie[6]);
  deltas[8] = aom_sdotq_s16(deltas[8], d_ie[3], d_ie[7]);
  deltas[9] = aom_sdotq_s16(deltas[9], d_ie[2], d_ie[8]);
  deltas[9] = aom_sdotq_s16(deltas[9], d_ie[3], d_ie[9]);
  deltas[10] = aom_sdotq_s16(deltas[10], d_ie[2], d_ie[10]);
  deltas[10] = aom_sdotq_s16(deltas[10], d_ie[3], d_ie[11]);

  deltas[11] = aom_sdotq_s16(deltas[11], d_ie[4], d_ie[4]);
  deltas[11] = aom_sdotq_s16(deltas[11], d_ie[5], d_ie[5]);
  deltas[12] = aom_sdotq_s16(deltas[12], d_ie[4], d_ie[6]);
  deltas[12] = aom_sdotq_s16(deltas[12], d_ie[5], d_ie[7]);
  deltas[13] = aom_sdotq_s16(deltas[13], d_ie[4], d_ie[8]);
  deltas[13] = aom_sdotq_s16(deltas[13], d_ie[5], d_ie[9]);
  deltas[14] = aom_sdotq_s16(deltas[14], d_ie[4], d_ie[10]);
  deltas[14] = aom_sdotq_s16(deltas[14], d_ie[5], d_ie[11]);

  deltas[15] = aom_sdotq_s16(deltas[15], d_ie[6], d_ie[6]);
  deltas[15] = aom_sdotq_s16(deltas[15], d_ie[7], d_ie[7]);
  deltas[16] = aom_sdotq_s16(deltas[16], d_ie[6], d_ie[8]);
  deltas[16] = aom_sdotq_s16(deltas[16], d_ie[7], d_ie[9]);
  deltas[17] = aom_sdotq_s16(deltas[17], d_ie[6], d_ie[10]);
  deltas[17] = aom_sdotq_s16(deltas[17], d_ie[7], d_ie[11]);

  deltas[18] = aom_sdotq_s16(deltas[18], d_ie[8], d_ie[8]);
  deltas[18] = aom_sdotq_s16(deltas[18], d_ie[9], d_ie[9]);
  deltas[19] = aom_sdotq_s16(deltas[19], d_ie[8], d_ie[10]);
  deltas[19] = aom_sdotq_s16(deltas[19], d_ie[9], d_ie[11]);

  deltas[20] = aom_sdotq_s16(deltas[20], d_ie[10], d_ie[10]);
  deltas[20] = aom_sdotq_s16(deltas[20], d_ie[11], d_ie[11]);
}

static inline void compute_stats_win7_sve(
    const int16_t *const d, const int32_t d_stride, const int16_t *const s,
    const int32_t s_stride, const int32_t width, const int32_t height,
    int64_t *const M, int64_t *const H) {
  const int32_t wiener_win = WIENER_WIN;
  const int32_t wiener_win2 = wiener_win * wiener_win;
  const int32_t h8 = height & ~7;
  int32_t i, j, x, y;

  // Use a predicate to compute the last columns.
  svbool_t p0 = svwhilelt_b16_u32(0, width % 16 == 0 ? 16 : width % 16);
  svbool_t p1 = svwhilelt_b16_u32(8, width % 16 == 0 ? 16 : width % 16);

  // Step 1: Calculate the top edge of the whole matrix, i.e., the top
  // edge of each triangle and square on the top row.
  j = 0;
  do {
    const int16_t *s_t = s;
    const int16_t *d_t = d;
    int64x2_t sum_m[WIENER_WIN] = { vdupq_n_s64(0) };
    int64x2_t sum_h[WIENER_WIN] = { vdupq_n_s64(0) };
    int16x8_t src[2], dgd[2];

    y = height;
    do {
      x = 0;
      while (x < width - 16) {
        src[0] = vld1q_s16(s_t + x + 0);
        src[1] = vld1q_s16(s_t + x + 8);
        dgd[0] = vld1q_s16(d_t + x + 0);
        dgd[1] = vld1q_s16(d_t + x + 8);
        stats_top_win7_sve(src, dgd, d_t + j + x, d_stride, sum_m, sum_h);
        x += 16;
      }

      src[0] = svget_neonq_s16(svld1_s16(p0, s_t + x + 0));
      src[1] = svget_neonq_s16(svld1_s16(p1, s_t + x + 8));
      dgd[0] = svget_neonq_s16(svld1_s16(p0, d_t + x + 0));
      dgd[1] = svget_neonq_s16(svld1_s16(p1, d_t + x + 8));
      stats_top_win7_sve(src, dgd, d_t + j + x, d_stride, sum_m, sum_h);

      s_t += s_stride;
      d_t += d_stride;
    } while (--y);

    vst1q_s64(M + wiener_win * j + 0, vpaddq_s64(sum_m[0], sum_m[1]));
    vst1q_s64(M + wiener_win * j + 2, vpaddq_s64(sum_m[2], sum_m[3]));
    vst1q_s64(M + wiener_win * j + 4, vpaddq_s64(sum_m[4], sum_m[5]));
    M[wiener_win * j + 6] = vaddvq_s64(sum_m[6]);

    vst1q_s64(H + wiener_win * j + 0, vpaddq_s64(sum_h[0], sum_h[1]));
    vst1q_s64(H + wiener_win * j + 2, vpaddq_s64(sum_h[2], sum_h[3]));
    vst1q_s64(H + wiener_win * j + 4, vpaddq_s64(sum_h[4], sum_h[5]));
    H[wiener_win * j + 6] = vaddvq_s64(sum_h[6]);
  } while (++j < wiener_win);

  // Step 2: Calculate the left edge of each square on the top row.
  j = 1;
  do {
    const int16_t *d_t = d;
    int64x2_t sum_h[WIENER_WIN - 1] = { vdupq_n_s64(0) };
    int16x8_t dgd[2];

    y = height;
    do {
      x = 0;
      while (x < width - 16) {
        dgd[0] = vld1q_s16(d_t + j + x + 0);
        dgd[1] = vld1q_s16(d_t + j + x + 8);
        stats_left_win7_sve(dgd, d_t + x, d_stride, sum_h);
        x += 16;
      }

      dgd[0] = svget_neonq_s16(svld1_s16(p0, d_t + j + x + 0));
      dgd[1] = svget_neonq_s16(svld1_s16(p1, d_t + j + x + 8));
      stats_left_win7_sve(dgd, d_t + x, d_stride, sum_h);

      d_t += d_stride;
    } while (--y);

    int64x2_t sum_h01 = vpaddq_s64(sum_h[0], sum_h[1]);
    int64x2_t sum_h23 = vpaddq_s64(sum_h[2], sum_h[3]);
    int64x2_t sum_h45 = vpaddq_s64(sum_h[4], sum_h[5]);
    vst1_s64(&H[1 * wiener_win2 + j * wiener_win], vget_low_s64(sum_h01));
    vst1_s64(&H[2 * wiener_win2 + j * wiener_win], vget_high_s64(sum_h01));
    vst1_s64(&H[3 * wiener_win2 + j * wiener_win], vget_low_s64(sum_h23));
    vst1_s64(&H[4 * wiener_win2 + j * wiener_win], vget_high_s64(sum_h23));
    vst1_s64(&H[5 * wiener_win2 + j * wiener_win], vget_low_s64(sum_h45));
    vst1_s64(&H[6 * wiener_win2 + j * wiener_win], vget_high_s64(sum_h45));
  } while (++j < wiener_win);

  // Step 3: Derive the top edge of each triangle along the diagonal. No
  // triangle in top row.
  {
    const int16_t *d_t = d;
    // Pad to call transpose function.
    int32x4_t deltas[(WIENER_WIN + 1) * 2] = { vdupq_n_s32(0) };
    int32x4_t deltas_tr[(WIENER_WIN + 1) * 2] = { vdupq_n_s32(0) };
    int16x8_t ds[WIENER_WIN * 2];

    load_s16_8x6(d_t, d_stride, &ds[0], &ds[2], &ds[4], &ds[6], &ds[8],
                 &ds[10]);
    load_s16_8x6(d_t + width, d_stride, &ds[1], &ds[3], &ds[5], &ds[7], &ds[9],
                 &ds[11]);

    d_t += 6 * d_stride;

    step3_win7_neon(d_t, d_stride, width, height, ds, deltas);
    transpose_arrays_s32_8x8(deltas, deltas_tr);

    update_8_stats_neon(H + 0 * wiener_win * wiener_win2 + 0 * wiener_win,
                        deltas_tr[0], deltas_tr[4],
                        H + 1 * wiener_win * wiener_win2 + 1 * wiener_win);
    update_8_stats_neon(H + 1 * wiener_win * wiener_win2 + 1 * wiener_win,
                        deltas_tr[1], deltas_tr[5],
                        H + 2 * wiener_win * wiener_win2 + 2 * wiener_win);
    update_8_stats_neon(H + 2 * wiener_win * wiener_win2 + 2 * wiener_win,
                        deltas_tr[2], deltas_tr[6],
                        H + 3 * wiener_win * wiener_win2 + 3 * wiener_win);
    update_8_stats_neon(H + 3 * wiener_win * wiener_win2 + 3 * wiener_win,
                        deltas_tr[3], deltas_tr[7],
                        H + 4 * wiener_win * wiener_win2 + 4 * wiener_win);
    update_8_stats_neon(H + 4 * wiener_win * wiener_win2 + 4 * wiener_win,
                        deltas_tr[8], deltas_tr[12],
                        H + 5 * wiener_win * wiener_win2 + 5 * wiener_win);
    update_8_stats_neon(H + 5 * wiener_win * wiener_win2 + 5 * wiener_win,
                        deltas_tr[9], deltas_tr[13],
                        H + 6 * wiener_win * wiener_win2 + 6 * wiener_win);
  }

  // Step 4: Derive the top and left edge of each square. No square in top and
  // bottom row.

  i = 1;
  do {
    j = i + 1;
    do {
      const int16_t *di = d + i - 1;
      const int16_t *dj = d + j - 1;
      int64x2_t deltas[(2 * WIENER_WIN - 1) * 2] = { vdupq_n_s64(0) };
      int16x8_t dd[WIENER_WIN * 2], ds[WIENER_WIN * 2];

      dd[5] = vdupq_n_s16(0);  // Initialize to avoid warning.
      const int16_t dd0_values[] = { di[0 * d_stride],
                                     di[1 * d_stride],
                                     di[2 * d_stride],
                                     di[3 * d_stride],
                                     di[4 * d_stride],
                                     di[5 * d_stride],
                                     0,
                                     0 };
      dd[0] = vld1q_s16(dd0_values);
      const int16_t dd1_values[] = { di[0 * d_stride + width],
                                     di[1 * d_stride + width],
                                     di[2 * d_stride + width],
                                     di[3 * d_stride + width],
                                     di[4 * d_stride + width],
                                     di[5 * d_stride + width],
                                     0,
                                     0 };
      dd[1] = vld1q_s16(dd1_values);
      const int16_t ds0_values[] = { dj[0 * d_stride],
                                     dj[1 * d_stride],
                                     dj[2 * d_stride],
                                     dj[3 * d_stride],
                                     dj[4 * d_stride],
                                     dj[5 * d_stride],
                                     0,
                                     0 };
      ds[0] = vld1q_s16(ds0_values);
      int16_t ds1_values[] = { dj[0 * d_stride + width],
                               dj[1 * d_stride + width],
                               dj[2 * d_stride + width],
                               dj[3 * d_stride + width],
                               dj[4 * d_stride + width],
                               dj[5 * d_stride + width],
                               0,
                               0 };
      ds[1] = vld1q_s16(ds1_values);

      y = 0;
      while (y < h8) {
        // 00s 10s 20s 30s 40s 50s 60s 70s  00e 10e 20e 30e 40e 50e 60e 70e
        dd[0] = vsetq_lane_s16(di[6 * d_stride], dd[0], 6);
        dd[0] = vsetq_lane_s16(di[7 * d_stride], dd[0], 7);
        dd[1] = vsetq_lane_s16(di[6 * d_stride + width], dd[1], 6);
        dd[1] = vsetq_lane_s16(di[7 * d_stride + width], dd[1], 7);

        // 00s 10s 20s 30s 40s 50s 60s 70s  00e 10e 20e 30e 40e 50e 60e 70e
        // 01s 11s 21s 31s 41s 51s 61s 71s  01e 11e 21e 31e 41e 51e 61e 71e
        ds[0] = vsetq_lane_s16(dj[6 * d_stride], ds[0], 6);
        ds[0] = vsetq_lane_s16(dj[7 * d_stride], ds[0], 7);
        ds[1] = vsetq_lane_s16(dj[6 * d_stride + width], ds[1], 6);
        ds[1] = vsetq_lane_s16(dj[7 * d_stride + width], ds[1], 7);

        load_more_16_neon(di + 8 * d_stride, width, &dd[0], &dd[2]);
        load_more_16_neon(dj + 8 * d_stride, width, &ds[0], &ds[2]);
        load_more_16_neon(di + 9 * d_stride, width, &dd[2], &dd[4]);
        load_more_16_neon(dj + 9 * d_stride, width, &ds[2], &ds[4]);
        load_more_16_neon(di + 10 * d_stride, width, &dd[4], &dd[6]);
        load_more_16_neon(dj + 10 * d_stride, width, &ds[4], &ds[6]);
        load_more_16_neon(di + 11 * d_stride, width, &dd[6], &dd[8]);
        load_more_16_neon(dj + 11 * d_stride, width, &ds[6], &ds[8]);
        load_more_16_neon(di + 12 * d_stride, width, &dd[8], &dd[10]);
        load_more_16_neon(dj + 12 * d_stride, width, &ds[8], &ds[10]);
        load_more_16_neon(di + 13 * d_stride, width, &dd[10], &dd[12]);
        load_more_16_neon(dj + 13 * d_stride, width, &ds[10], &ds[12]);

        deltas[0] = aom_sdotq_s16(deltas[0], dd[0], ds[0]);
        deltas[1] = aom_sdotq_s16(deltas[1], dd[1], ds[1]);
        deltas[2] = aom_sdotq_s16(deltas[2], dd[0], ds[2]);
        deltas[3] = aom_sdotq_s16(deltas[3], dd[1], ds[3]);
        deltas[4] = aom_sdotq_s16(deltas[4], dd[0], ds[4]);
        deltas[5] = aom_sdotq_s16(deltas[5], dd[1], ds[5]);
        deltas[6] = aom_sdotq_s16(deltas[6], dd[0], ds[6]);
        deltas[7] = aom_sdotq_s16(deltas[7], dd[1], ds[7]);
        deltas[8] = aom_sdotq_s16(deltas[8], dd[0], ds[8]);
        deltas[9] = aom_sdotq_s16(deltas[9], dd[1], ds[9]);
        deltas[10] = aom_sdotq_s16(deltas[10], dd[0], ds[10]);
        deltas[11] = aom_sdotq_s16(deltas[11], dd[1], ds[11]);
        deltas[12] = aom_sdotq_s16(deltas[12], dd[0], ds[12]);
        deltas[13] = aom_sdotq_s16(deltas[13], dd[1], ds[13]);
        deltas[14] = aom_sdotq_s16(deltas[14], dd[2], ds[0]);
        deltas[15] = aom_sdotq_s16(deltas[15], dd[3], ds[1]);
        deltas[16] = aom_sdotq_s16(deltas[16], dd[4], ds[0]);
        deltas[17] = aom_sdotq_s16(deltas[17], dd[5], ds[1]);
        deltas[18] = aom_sdotq_s16(deltas[18], dd[6], ds[0]);
        deltas[19] = aom_sdotq_s16(deltas[19], dd[7], ds[1]);
        deltas[20] = aom_sdotq_s16(deltas[20], dd[8], ds[0]);
        deltas[21] = aom_sdotq_s16(deltas[21], dd[9], ds[1]);
        deltas[22] = aom_sdotq_s16(deltas[22], dd[10], ds[0]);
        deltas[23] = aom_sdotq_s16(deltas[23], dd[11], ds[1]);
        deltas[24] = aom_sdotq_s16(deltas[24], dd[12], ds[0]);
        deltas[25] = aom_sdotq_s16(deltas[25], dd[13], ds[1]);

        dd[0] = vextq_s16(dd[12], vdupq_n_s16(0), 2);
        dd[1] = vextq_s16(dd[13], vdupq_n_s16(0), 2);
        ds[0] = vextq_s16(ds[12], vdupq_n_s16(0), 2);
        ds[1] = vextq_s16(ds[13], vdupq_n_s16(0), 2);

        di += 8 * d_stride;
        dj += 8 * d_stride;
        y += 8;
      }

      int64x2_t deltas02 = vpaddq_s64(deltas[0], deltas[2]);
      int64x2_t deltas13 = vpaddq_s64(deltas[1], deltas[3]);
      int64x2_t deltas46 = vpaddq_s64(deltas[4], deltas[6]);
      int64x2_t deltas57 = vpaddq_s64(deltas[5], deltas[7]);
      int64x2_t deltas810 = vpaddq_s64(deltas[8], deltas[10]);
      int64x2_t deltas911 = vpaddq_s64(deltas[9], deltas[11]);
      int64x2_t deltas1212 = vpaddq_s64(deltas[12], deltas[12]);
      int64x2_t deltas1313 = vpaddq_s64(deltas[13], deltas[13]);
      int64x2_t deltas1416 = vpaddq_s64(deltas[14], deltas[16]);
      int64x2_t deltas1820 = vpaddq_s64(deltas[18], deltas[20]);
      int64x2_t deltas1517 = vpaddq_s64(deltas[15], deltas[17]);
      int64x2_t deltas1921 = vpaddq_s64(deltas[19], deltas[21]);
      int64x2_t deltas2224 = vpaddq_s64(deltas[22], deltas[24]);
      int64x2_t deltas2325 = vpaddq_s64(deltas[23], deltas[25]);
      deltas02 = vsubq_s64(deltas13, deltas02);
      deltas46 = vsubq_s64(deltas57, deltas46);
      deltas810 = vsubq_s64(deltas911, deltas810);
      deltas1212 = vsubq_s64(deltas1313, deltas1212);
      deltas1416 = vsubq_s64(deltas1517, deltas1416);
      deltas1820 = vsubq_s64(deltas1921, deltas1820);
      deltas2224 = vsubq_s64(deltas2325, deltas2224);

      if (h8 != height) {
        const int16_t ds0_vals[] = {
          dj[0 * d_stride], dj[0 * d_stride + width],
          dj[1 * d_stride], dj[1 * d_stride + width],
          dj[2 * d_stride], dj[2 * d_stride + width],
          dj[3 * d_stride], dj[3 * d_stride + width]
        };
        ds[0] = vld1q_s16(ds0_vals);

        ds[1] = vsetq_lane_s16(dj[4 * d_stride], ds[1], 0);
        ds[1] = vsetq_lane_s16(dj[4 * d_stride + width], ds[1], 1);
        ds[1] = vsetq_lane_s16(dj[5 * d_stride], ds[1], 2);
        ds[1] = vsetq_lane_s16(dj[5 * d_stride + width], ds[1], 3);
        const int16_t dd4_vals[] = {
          -di[1 * d_stride], di[1 * d_stride + width],
          -di[2 * d_stride], di[2 * d_stride + width],
          -di[3 * d_stride], di[3 * d_stride + width],
          -di[4 * d_stride], di[4 * d_stride + width]
        };
        dd[4] = vld1q_s16(dd4_vals);

        dd[5] = vsetq_lane_s16(-di[5 * d_stride], dd[5], 0);
        dd[5] = vsetq_lane_s16(di[5 * d_stride + width], dd[5], 1);
        do {
          dd[0] = vdupq_n_s16(-di[0 * d_stride]);
          dd[2] = dd[3] = vdupq_n_s16(di[0 * d_stride + width]);
          dd[0] = dd[1] = vzip1q_s16(dd[0], dd[2]);

          ds[4] = vdupq_n_s16(dj[0 * d_stride]);
          ds[6] = ds[7] = vdupq_n_s16(dj[0 * d_stride + width]);
          ds[4] = ds[5] = vzip1q_s16(ds[4], ds[6]);

          dd[5] = vsetq_lane_s16(-di[6 * d_stride], dd[5], 2);
          dd[5] = vsetq_lane_s16(di[6 * d_stride + width], dd[5], 3);
          ds[1] = vsetq_lane_s16(dj[6 * d_stride], ds[1], 4);
          ds[1] = vsetq_lane_s16(dj[6 * d_stride + width], ds[1], 5);

          const int32x4_t res0 =
              vpaddq_s32(vmull_s16(vget_low_s16(dd[0]), vget_low_s16(ds[0])),
                         vmull_s16(vget_high_s16(dd[0]), vget_high_s16(ds[0])));
          deltas02 = vaddw_s32(deltas02, vget_low_s32(res0));
          deltas46 = vaddw_s32(deltas46, vget_high_s32(res0));
          const int32x4_t res1 =
              vpaddq_s32(vmull_s16(vget_low_s16(dd[1]), vget_low_s16(ds[1])),
                         vmull_s16(vget_high_s16(dd[1]), vget_high_s16(ds[1])));
          deltas810 = vaddw_s32(deltas810, vget_low_s32(res1));
          deltas1212 = vaddw_s32(deltas1212, vget_high_s32(res1));
          const int32x4_t res2 =
              vpaddq_s32(vmull_s16(vget_low_s16(dd[4]), vget_low_s16(ds[4])),
                         vmull_s16(vget_high_s16(dd[4]), vget_high_s16(ds[4])));
          deltas1416 = vaddw_s32(deltas1416, vget_low_s32(res2));
          deltas1820 = vaddw_s32(deltas1820, vget_high_s32(res2));
          const int32x4_t res3 =
              vpaddq_s32(vmull_s16(vget_low_s16(dd[5]), vget_low_s16(ds[5])),
                         vmull_s16(vget_high_s16(dd[5]), vget_high_s16(ds[5])));
          deltas2224 = vaddw_s32(deltas2224, vget_low_s32(res3));

          int32_t tmp0 = vgetq_lane_s32(vreinterpretq_s32_s16(ds[0]), 0);
          ds[0] = vextq_s16(ds[0], ds[1], 2);
          ds[1] = vextq_s16(ds[1], ds[0], 2);
          ds[1] = vreinterpretq_s16_s32(
              vsetq_lane_s32(tmp0, vreinterpretq_s32_s16(ds[1]), 3));
          int32_t tmp1 = vgetq_lane_s32(vreinterpretq_s32_s16(dd[4]), 0);
          dd[4] = vextq_s16(dd[4], dd[5], 2);
          dd[5] = vextq_s16(dd[5], dd[4], 2);
          dd[5] = vreinterpretq_s16_s32(
              vsetq_lane_s32(tmp1, vreinterpretq_s32_s16(dd[5]), 3));
          di += d_stride;
          dj += d_stride;
        } while (++y < height);
      }

      // Writing one more element on the top edge of a square falls to
      // the next square in the same row or the first element in the next
      // row, which will just be overwritten later.
      int64x2_t s0 = vld1q_s64(H + (i - 1) * wiener_win * wiener_win2 +
                               (j - 1) * wiener_win + 0);
      int64x2_t s1 = vld1q_s64(H + (i - 1) * wiener_win * wiener_win2 +
                               (j - 1) * wiener_win + 2);
      int64x2_t s2 = vld1q_s64(H + (i - 1) * wiener_win * wiener_win2 +
                               (j - 1) * wiener_win + 4);
      int64x2_t s3 = vld1q_s64(H + (i - 1) * wiener_win * wiener_win2 +
                               (j - 1) * wiener_win + 6);

      vst1q_s64(H + i * wiener_win * wiener_win2 + j * wiener_win + 0,
                vaddq_s64(s0, deltas02));
      vst1q_s64(H + i * wiener_win * wiener_win2 + j * wiener_win + 2,
                vaddq_s64(s1, deltas46));
      vst1q_s64(H + i * wiener_win * wiener_win2 + j * wiener_win + 4,
                vaddq_s64(s2, deltas810));
      vst1q_s64(H + i * wiener_win * wiener_win2 + j * wiener_win + 6,
                vaddq_s64(s3, deltas1212));

      H[(i * wiener_win + 1) * wiener_win2 + j * wiener_win] =
          H[((i - 1) * wiener_win + 1) * wiener_win2 + (j - 1) * wiener_win] +
          vgetq_lane_s64(deltas1416, 0);
      H[(i * wiener_win + 2) * wiener_win2 + j * wiener_win] =
          H[((i - 1) * wiener_win + 2) * wiener_win2 + (j - 1) * wiener_win] +
          vgetq_lane_s64(deltas1416, 1);
      H[(i * wiener_win + 3) * wiener_win2 + j * wiener_win] =
          H[((i - 1) * wiener_win + 3) * wiener_win2 + (j - 1) * wiener_win] +
          vgetq_lane_s64(deltas1820, 0);
      H[(i * wiener_win + 4) * wiener_win2 + j * wiener_win] =
          H[((i - 1) * wiener_win + 4) * wiener_win2 + (j - 1) * wiener_win] +
          vgetq_lane_s64(deltas1820, 1);
      H[(i * wiener_win + 5) * wiener_win2 + j * wiener_win] =
          H[((i - 1) * wiener_win + 5) * wiener_win2 + (j - 1) * wiener_win] +
          vgetq_lane_s64(deltas2224, 0);
      H[(i * wiener_win + 6) * wiener_win2 + j * wiener_win] =
          H[((i - 1) * wiener_win + 6) * wiener_win2 + (j - 1) * wiener_win] +
          vgetq_lane_s64(deltas2224, 1);
    } while (++j < wiener_win);
  } while (++i < wiener_win - 1);

  // Step 5: Derive other points of each square. No square in bottom row.
  i = 0;
  do {
    const int16_t *const di = d + i;

    j = i + 1;
    do {
      const int16_t *const dj = d + j;
      int64x2_t deltas[WIENER_WIN - 1][WIN_7] = { { vdupq_n_s64(0) },
                                                  { vdupq_n_s64(0) } };
      int16x8_t d_is[WIN_7];
      int16x8_t d_ie[WIN_7];
      int16x8_t d_js[WIN_7];
      int16x8_t d_je[WIN_7];

      x = 0;
      while (x < width - 16) {
        load_square_win7_neon(di + x, dj + x, d_stride, height, d_is, d_ie,
                              d_js, d_je);
        derive_square_win7_sve(d_is, d_ie, d_js, d_je, deltas);
        x += 16;
      }

      load_square_win7_sve(di + x, dj + x, d_stride, height, d_is, d_ie, d_js,
                           d_je, p0, p1);
      derive_square_win7_sve(d_is, d_ie, d_js, d_je, deltas);

      hadd_update_6_stats_sve(
          H + (i * wiener_win + 0) * wiener_win2 + j * wiener_win, deltas[0],
          H + (i * wiener_win + 1) * wiener_win2 + j * wiener_win + 1);
      hadd_update_6_stats_sve(
          H + (i * wiener_win + 1) * wiener_win2 + j * wiener_win, deltas[1],
          H + (i * wiener_win + 2) * wiener_win2 + j * wiener_win + 1);
      hadd_update_6_stats_sve(
          H + (i * wiener_win + 2) * wiener_win2 + j * wiener_win, deltas[2],
          H + (i * wiener_win + 3) * wiener_win2 + j * wiener_win + 1);
      hadd_update_6_stats_sve(
          H + (i * wiener_win + 3) * wiener_win2 + j * wiener_win, deltas[3],
          H + (i * wiener_win + 4) * wiener_win2 + j * wiener_win + 1);
      hadd_update_6_stats_sve(
          H + (i * wiener_win + 4) * wiener_win2 + j * wiener_win, deltas[4],
          H + (i * wiener_win + 5) * wiener_win2 + j * wiener_win + 1);
      hadd_update_6_stats_sve(
          H + (i * wiener_win + 5) * wiener_win2 + j * wiener_win, deltas[5],
          H + (i * wiener_win + 6) * wiener_win2 + j * wiener_win + 1);
    } while (++j < wiener_win);
  } while (++i < wiener_win - 1);

  // Step 6: Derive other points of each upper triangle along the diagonal.
  i = 0;
  do {
    const int16_t *const di = d + i;
    int64x2_t deltas[3 * WIENER_WIN] = { vdupq_n_s64(0) };
    int16x8_t d_is[WIN_7], d_ie[WIN_7];

    x = 0;
    while (x < width - 16) {
      load_triangle_win7_neon(di + x, d_stride, height, d_is, d_ie);
      derive_triangle_win7_sve(d_is, d_ie, deltas);
      x += 16;
    }

    load_triangle_win7_sve(di + x, d_stride, height, d_is, d_ie, p0, p1);
    derive_triangle_win7_sve(d_is, d_ie, deltas);

    // Row 1: 6 points
    hadd_update_6_stats_sve(
        H + (i * wiener_win + 0) * wiener_win2 + i * wiener_win, deltas,
        H + (i * wiener_win + 1) * wiener_win2 + i * wiener_win + 1);

    int64x2_t deltas1017 = vpaddq_s64(deltas[10], deltas[17]);

    // Row 2: 5 points
    hadd_update_4_stats_sve(
        H + (i * wiener_win + 1) * wiener_win2 + i * wiener_win + 1, deltas + 6,
        H + (i * wiener_win + 2) * wiener_win2 + i * wiener_win + 2);
    H[(i * wiener_win + 2) * wiener_win2 + i * wiener_win + 6] =
        H[(i * wiener_win + 1) * wiener_win2 + i * wiener_win + 5] +
        vgetq_lane_s64(deltas1017, 0);

    // Row 3: 4 points
    hadd_update_4_stats_sve(
        H + (i * wiener_win + 2) * wiener_win2 + i * wiener_win + 2,
        deltas + 11,
        H + (i * wiener_win + 3) * wiener_win2 + i * wiener_win + 3);

    // Row 4: 3 points
    int64x2_t h0 =
        vld1q_s64(H + (i * wiener_win + 3) * wiener_win2 + i * wiener_win + 3);
    vst1q_s64(H + (i * wiener_win + 4) * wiener_win2 + i * wiener_win + 4,
              vaddq_s64(h0, vpaddq_s64(deltas[15], deltas[16])));
    H[(i * wiener_win + 4) * wiener_win2 + i * wiener_win + 6] =
        H[(i * wiener_win + 3) * wiener_win2 + i * wiener_win + 5] +
        vgetq_lane_s64(deltas1017, 1);

    // Row 5: 2 points
    int64x2_t h1 =
        vld1q_s64(H + (i * wiener_win + 4) * wiener_win2 + i * wiener_win + 4);
    vst1q_s64(H + (i * wiener_win + 5) * wiener_win2 + i * wiener_win + 5,
              vaddq_s64(h1, vpaddq_s64(deltas[18], deltas[19])));

    // Row 6: 1 points
    H[(i * wiener_win + 6) * wiener_win2 + i * wiener_win + 6] =
        H[(i * wiener_win + 5) * wiener_win2 + i * wiener_win + 5] +
        vaddvq_s64(deltas[20]);
  } while (++i < wiener_win);
}

#endif  // AOM_AV1_ENCODER_ARM_PICKRST_SVE_H_
