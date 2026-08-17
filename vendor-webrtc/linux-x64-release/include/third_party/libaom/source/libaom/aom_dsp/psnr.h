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

#ifndef AOM_AOM_DSP_PSNR_H_
#define AOM_AOM_DSP_PSNR_H_

#include "aom_scale/yv12config.h"
#include "config/aom_config.h"

#define MAX_PSNR 100.0

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
  double psnr[4];           // total/y/u/v
  uint64_t sse[4];          // total/y/u/v
  uint32_t samples[4];      // total/y/u/v
  double psnr_hbd[4];       // total/y/u/v when input-bit-depth < bit-depth
  uint64_t sse_hbd[4];      // total/y/u/v when input-bit-depth < bit-depth
  uint32_t samples_hbd[4];  // total/y/u/v when input-bit-depth < bit-depth
} PSNR_STATS;

#if CONFIG_INTERNAL_STATS
/*!\brief Converts SSE to PSNR
 *
 * Converts sum of squared errros (SSE) to peak signal-to-noise ratio (PSNR).
 *
 * \param[in]    samples       Number of samples
 * \param[in]    peak          Max sample value
 * \param[in]    sse           Sum of squared errors
 */
double aom_sse_to_psnr(double samples, double peak, double sse);
#endif  // CONFIG_INTERNAL_STATS
uint64_t aom_get_y_var(const YV12_BUFFER_CONFIG *a, int hstart, int width,
                       int vstart, int height);
uint64_t aom_get_u_var(const YV12_BUFFER_CONFIG *a, int hstart, int width,
                       int vstart, int height);
uint64_t aom_get_v_var(const YV12_BUFFER_CONFIG *a, int hstart, int width,
                       int vstart, int height);
int64_t aom_get_y_sse_part(const YV12_BUFFER_CONFIG *a,
                           const YV12_BUFFER_CONFIG *b, int hstart, int width,
                           int vstart, int height);
int64_t aom_get_y_sse(const YV12_BUFFER_CONFIG *a, const YV12_BUFFER_CONFIG *b);
int64_t aom_get_u_sse_part(const YV12_BUFFER_CONFIG *a,
                           const YV12_BUFFER_CONFIG *b, int hstart, int width,
                           int vstart, int height);
int64_t aom_get_u_sse(const YV12_BUFFER_CONFIG *a, const YV12_BUFFER_CONFIG *b);
int64_t aom_get_v_sse_part(const YV12_BUFFER_CONFIG *a,
                           const YV12_BUFFER_CONFIG *b, int hstart, int width,
                           int vstart, int height);
int64_t aom_get_v_sse(const YV12_BUFFER_CONFIG *a, const YV12_BUFFER_CONFIG *b);
int64_t aom_get_sse_plane(const YV12_BUFFER_CONFIG *a,
                          const YV12_BUFFER_CONFIG *b, int plane, int highbd);
#if CONFIG_AV1_HIGHBITDEPTH
uint64_t aom_highbd_get_y_var(const YV12_BUFFER_CONFIG *a, int hstart,
                              int width, int vstart, int height);
uint64_t aom_highbd_get_u_var(const YV12_BUFFER_CONFIG *a, int hstart,
                              int width, int vstart, int height);
uint64_t aom_highbd_get_v_var(const YV12_BUFFER_CONFIG *a, int hstart,
                              int width, int vstart, int height);
int64_t aom_highbd_get_y_sse_part(const YV12_BUFFER_CONFIG *a,
                                  const YV12_BUFFER_CONFIG *b, int hstart,
                                  int width, int vstart, int height);
int64_t aom_highbd_get_y_sse(const YV12_BUFFER_CONFIG *a,
                             const YV12_BUFFER_CONFIG *b);
int64_t aom_highbd_get_u_sse_part(const YV12_BUFFER_CONFIG *a,
                                  const YV12_BUFFER_CONFIG *b, int hstart,
                                  int width, int vstart, int height);
int64_t aom_highbd_get_u_sse(const YV12_BUFFER_CONFIG *a,
                             const YV12_BUFFER_CONFIG *b);
int64_t aom_highbd_get_v_sse_part(const YV12_BUFFER_CONFIG *a,
                                  const YV12_BUFFER_CONFIG *b, int hstart,
                                  int width, int vstart, int height);
int64_t aom_highbd_get_v_sse(const YV12_BUFFER_CONFIG *a,
                             const YV12_BUFFER_CONFIG *b);
void aom_calc_highbd_psnr(const YV12_BUFFER_CONFIG *a,
                          const YV12_BUFFER_CONFIG *b, PSNR_STATS *psnr,
                          unsigned int bit_depth, unsigned int in_bit_depth);
#endif
void aom_calc_psnr(const YV12_BUFFER_CONFIG *a, const YV12_BUFFER_CONFIG *b,
                   PSNR_STATS *psnr);

double aom_psnrhvs(const YV12_BUFFER_CONFIG *source,
                   const YV12_BUFFER_CONFIG *dest, double *phvs_y,
                   double *phvs_u, double *phvs_v, uint32_t bd, uint32_t in_bd);
#ifdef __cplusplus
}  // extern "C"
#endif
#endif  // AOM_AOM_DSP_PSNR_H_
