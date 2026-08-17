/*
 *  Copyright (c) 2025 The WebM project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

// This file is generated. Do not edit.
#ifndef VPX_DSP_RTCD_H_
#define VPX_DSP_RTCD_H_

#ifdef RTCD_C
#define RTCD_EXTERN
#else
#define RTCD_EXTERN extern
#endif

/*
 * DSP
 */

#include "vpx/vpx_integer.h"
#include "vpx_dsp/vpx_dsp_common.h"
#include "vpx_dsp/vpx_filter.h"
#if CONFIG_VP9_ENCODER
 struct macroblock_plane;
 struct ScanOrder;
#endif


#ifdef __cplusplus
extern "C" {
#endif

unsigned int vpx_avg_4x4_c(const uint8_t *, int p);
unsigned int vpx_avg_4x4_neon(const uint8_t *, int p);
#define vpx_avg_4x4 vpx_avg_4x4_neon

unsigned int vpx_avg_8x8_c(const uint8_t *, int p);
unsigned int vpx_avg_8x8_neon(const uint8_t *, int p);
#define vpx_avg_8x8 vpx_avg_8x8_neon

void vpx_comp_avg_pred_c(uint8_t *comp_pred, const uint8_t *pred, int width, int height, const uint8_t *ref, int ref_stride);
void vpx_comp_avg_pred_neon(uint8_t *comp_pred, const uint8_t *pred, int width, int height, const uint8_t *ref, int ref_stride);
#define vpx_comp_avg_pred vpx_comp_avg_pred_neon

void vpx_convolve8_c(const uint8_t *src, ptrdiff_t src_stride, uint8_t *dst, ptrdiff_t dst_stride, const InterpKernel *filter, int x0_q4, int x_step_q4, int y0_q4, int y_step_q4, int w, int h);
void vpx_convolve8_neon(const uint8_t *src, ptrdiff_t src_stride, uint8_t *dst, ptrdiff_t dst_stride, const InterpKernel *filter, int x0_q4, int x_step_q4, int y0_q4, int y_step_q4, int w, int h);
void vpx_convolve8_neon_dotprod(const uint8_t *src, ptrdiff_t src_stride, uint8_t *dst, ptrdiff_t dst_stride, const InterpKernel *filter, int x0_q4, int x_step_q4, int y0_q4, int y_step_q4, int w, int h);
void vpx_convolve8_neon_i8mm(const uint8_t *src, ptrdiff_t src_stride, uint8_t *dst, ptrdiff_t dst_stride, const InterpKernel *filter, int x0_q4, int x_step_q4, int y0_q4, int y_step_q4, int w, int h);
RTCD_EXTERN void (*vpx_convolve8)(const uint8_t *src, ptrdiff_t src_stride, uint8_t *dst, ptrdiff_t dst_stride, const InterpKernel *filter, int x0_q4, int x_step_q4, int y0_q4, int y_step_q4, int w, int h);

void vpx_convolve8_avg_c(const uint8_t *src, ptrdiff_t src_stride, uint8_t *dst, ptrdiff_t dst_stride, const InterpKernel *filter, int x0_q4, int x_step_q4, int y0_q4, int y_step_q4, int w, int h);
void vpx_convolve8_avg_neon(const uint8_t *src, ptrdiff_t src_stride, uint8_t *dst, ptrdiff_t dst_stride, const InterpKernel *filter, int x0_q4, int x_step_q4, int y0_q4, int y_step_q4, int w, int h);
void vpx_convolve8_avg_neon_dotprod(const uint8_t *src, ptrdiff_t src_stride, uint8_t *dst, ptrdiff_t dst_stride, const InterpKernel *filter, int x0_q4, int x_step_q4, int y0_q4, int y_step_q4, int w, int h);
void vpx_convolve8_avg_neon_i8mm(const uint8_t *src, ptrdiff_t src_stride, uint8_t *dst, ptrdiff_t dst_stride, const InterpKernel *filter, int x0_q4, int x_step_q4, int y0_q4, int y_step_q4, int w, int h);
RTCD_EXTERN void (*vpx_convolve8_avg)(const uint8_t *src, ptrdiff_t src_stride, uint8_t *dst, ptrdiff_t dst_stride, const InterpKernel *filter, int x0_q4, int x_step_q4, int y0_q4, int y_step_q4, int w, int h);

void vpx_convolve8_avg_horiz_c(const uint8_t *src, ptrdiff_t src_stride, uint8_t *dst, ptrdiff_t dst_stride, const InterpKernel *filter, int x0_q4, int x_step_q4, int y0_q4, int y_step_q4, int w, int h);
void vpx_convolve8_avg_horiz_neon(const uint8_t *src, ptrdiff_t src_stride, uint8_t *dst, ptrdiff_t dst_stride, const InterpKernel *filter, int x0_q4, int x_step_q4, int y0_q4, int y_step_q4, int w, int h);
void vpx_convolve8_avg_horiz_neon_dotprod(const uint8_t *src, ptrdiff_t src_stride, uint8_t *dst, ptrdiff_t dst_stride, const InterpKernel *filter, int x0_q4, int x_step_q4, int y0_q4, int y_step_q4, int w, int h);
void vpx_convolve8_avg_horiz_neon_i8mm(const uint8_t *src, ptrdiff_t src_stride, uint8_t *dst, ptrdiff_t dst_stride, const InterpKernel *filter, int x0_q4, int x_step_q4, int y0_q4, int y_step_q4, int w, int h);
RTCD_EXTERN void (*vpx_convolve8_avg_horiz)(const uint8_t *src, ptrdiff_t src_stride, uint8_t *dst, ptrdiff_t dst_stride, const InterpKernel *filter, int x0_q4, int x_step_q4, int y0_q4, int y_step_q4, int w, int h);

void vpx_convolve8_avg_vert_c(const uint8_t *src, ptrdiff_t src_stride, uint8_t *dst, ptrdiff_t dst_stride, const InterpKernel *filter, int x0_q4, int x_step_q4, int y0_q4, int y_step_q4, int w, int h);
void vpx_convolve8_avg_vert_neon(const uint8_t *src, ptrdiff_t src_stride, uint8_t *dst, ptrdiff_t dst_stride, const InterpKernel *filter, int x0_q4, int x_step_q4, int y0_q4, int y_step_q4, int w, int h);
void vpx_convolve8_avg_vert_neon_dotprod(const uint8_t *src, ptrdiff_t src_stride, uint8_t *dst, ptrdiff_t dst_stride, const InterpKernel *filter, int x0_q4, int x_step_q4, int y0_q4, int y_step_q4, int w, int h);
void vpx_convolve8_avg_vert_neon_i8mm(const uint8_t *src, ptrdiff_t src_stride, uint8_t *dst, ptrdiff_t dst_stride, const InterpKernel *filter, int x0_q4, int x_step_q4, int y0_q4, int y_step_q4, int w, int h);
RTCD_EXTERN void (*vpx_convolve8_avg_vert)(const uint8_t *src, ptrdiff_t src_stride, uint8_t *dst, ptrdiff_t dst_stride, const InterpKernel *filter, int x0_q4, int x_step_q4, int y0_q4, int y_step_q4, int w, int h);

void vpx_convolve8_horiz_c(const uint8_t *src, ptrdiff_t src_stride, uint8_t *dst, ptrdiff_t dst_stride, const InterpKernel *filter, int x0_q4, int x_step_q4, int y0_q4, int y_step_q4, int w, int h);
void vpx_convolve8_horiz_neon(const uint8_t *src, ptrdiff_t src_stride, uint8_t *dst, ptrdiff_t dst_stride, const InterpKernel *filter, int x0_q4, int x_step_q4, int y0_q4, int y_step_q4, int w, int h);
void vpx_convolve8_horiz_neon_dotprod(const uint8_t *src, ptrdiff_t src_stride, uint8_t *dst, ptrdiff_t dst_stride, const InterpKernel *filter, int x0_q4, int x_step_q4, int y0_q4, int y_step_q4, int w, int h);
void vpx_convolve8_horiz_neon_i8mm(const uint8_t *src, ptrdiff_t src_stride, uint8_t *dst, ptrdiff_t dst_stride, const InterpKernel *filter, int x0_q4, int x_step_q4, int y0_q4, int y_step_q4, int w, int h);
RTCD_EXTERN void (*vpx_convolve8_horiz)(const uint8_t *src, ptrdiff_t src_stride, uint8_t *dst, ptrdiff_t dst_stride, const InterpKernel *filter, int x0_q4, int x_step_q4, int y0_q4, int y_step_q4, int w, int h);

void vpx_convolve8_vert_c(const uint8_t *src, ptrdiff_t src_stride, uint8_t *dst, ptrdiff_t dst_stride, const InterpKernel *filter, int x0_q4, int x_step_q4, int y0_q4, int y_step_q4, int w, int h);
void vpx_convolve8_vert_neon(const uint8_t *src, ptrdiff_t src_stride, uint8_t *dst, ptrdiff_t dst_stride, const InterpKernel *filter, int x0_q4, int x_step_q4, int y0_q4, int y_step_q4, int w, int h);
void vpx_convolve8_vert_neon_dotprod(const uint8_t *src, ptrdiff_t src_stride, uint8_t *dst, ptrdiff_t dst_stride, const InterpKernel *filter, int x0_q4, int x_step_q4, int y0_q4, int y_step_q4, int w, int h);
void vpx_convolve8_vert_neon_i8mm(const uint8_t *src, ptrdiff_t src_stride, uint8_t *dst, ptrdiff_t dst_stride, const InterpKernel *filter, int x0_q4, int x_step_q4, int y0_q4, int y_step_q4, int w, int h);
RTCD_EXTERN void (*vpx_convolve8_vert)(const uint8_t *src, ptrdiff_t src_stride, uint8_t *dst, ptrdiff_t dst_stride, const InterpKernel *filter, int x0_q4, int x_step_q4, int y0_q4, int y_step_q4, int w, int h);

void vpx_convolve_avg_c(const uint8_t *src, ptrdiff_t src_stride, uint8_t *dst, ptrdiff_t dst_stride, const InterpKernel *filter, int x0_q4, int x_step_q4, int y0_q4, int y_step_q4, int w, int h);
void vpx_convolve_avg_neon(const uint8_t *src, ptrdiff_t src_stride, uint8_t *dst, ptrdiff_t dst_stride, const InterpKernel *filter, int x0_q4, int x_step_q4, int y0_q4, int y_step_q4, int w, int h);
#define vpx_convolve_avg vpx_convolve_avg_neon

void vpx_convolve_copy_c(const uint8_t *src, ptrdiff_t src_stride, uint8_t *dst, ptrdiff_t dst_stride, const InterpKernel *filter, int x0_q4, int x_step_q4, int y0_q4, int y_step_q4, int w, int h);
void vpx_convolve_copy_neon(const uint8_t *src, ptrdiff_t src_stride, uint8_t *dst, ptrdiff_t dst_stride, const InterpKernel *filter, int x0_q4, int x_step_q4, int y0_q4, int y_step_q4, int w, int h);
#define vpx_convolve_copy vpx_convolve_copy_neon

void vpx_d117_predictor_16x16_c(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
void vpx_d117_predictor_16x16_neon(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
#define vpx_d117_predictor_16x16 vpx_d117_predictor_16x16_neon

void vpx_d117_predictor_32x32_c(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
void vpx_d117_predictor_32x32_neon(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
#define vpx_d117_predictor_32x32 vpx_d117_predictor_32x32_neon

void vpx_d117_predictor_4x4_c(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
void vpx_d117_predictor_4x4_neon(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
#define vpx_d117_predictor_4x4 vpx_d117_predictor_4x4_neon

void vpx_d117_predictor_8x8_c(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
void vpx_d117_predictor_8x8_neon(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
#define vpx_d117_predictor_8x8 vpx_d117_predictor_8x8_neon

void vpx_d135_predictor_16x16_c(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
void vpx_d135_predictor_16x16_neon(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
#define vpx_d135_predictor_16x16 vpx_d135_predictor_16x16_neon

void vpx_d135_predictor_32x32_c(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
void vpx_d135_predictor_32x32_neon(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
#define vpx_d135_predictor_32x32 vpx_d135_predictor_32x32_neon

void vpx_d135_predictor_4x4_c(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
void vpx_d135_predictor_4x4_neon(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
#define vpx_d135_predictor_4x4 vpx_d135_predictor_4x4_neon

void vpx_d135_predictor_8x8_c(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
void vpx_d135_predictor_8x8_neon(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
#define vpx_d135_predictor_8x8 vpx_d135_predictor_8x8_neon

void vpx_d153_predictor_16x16_c(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
void vpx_d153_predictor_16x16_neon(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
#define vpx_d153_predictor_16x16 vpx_d153_predictor_16x16_neon

void vpx_d153_predictor_32x32_c(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
void vpx_d153_predictor_32x32_neon(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
#define vpx_d153_predictor_32x32 vpx_d153_predictor_32x32_neon

void vpx_d153_predictor_4x4_c(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
void vpx_d153_predictor_4x4_neon(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
#define vpx_d153_predictor_4x4 vpx_d153_predictor_4x4_neon

void vpx_d153_predictor_8x8_c(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
void vpx_d153_predictor_8x8_neon(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
#define vpx_d153_predictor_8x8 vpx_d153_predictor_8x8_neon

void vpx_d207_predictor_16x16_c(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
void vpx_d207_predictor_16x16_neon(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
#define vpx_d207_predictor_16x16 vpx_d207_predictor_16x16_neon

void vpx_d207_predictor_32x32_c(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
void vpx_d207_predictor_32x32_neon(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
#define vpx_d207_predictor_32x32 vpx_d207_predictor_32x32_neon

void vpx_d207_predictor_4x4_c(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
void vpx_d207_predictor_4x4_neon(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
#define vpx_d207_predictor_4x4 vpx_d207_predictor_4x4_neon

void vpx_d207_predictor_8x8_c(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
void vpx_d207_predictor_8x8_neon(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
#define vpx_d207_predictor_8x8 vpx_d207_predictor_8x8_neon

void vpx_d45_predictor_16x16_c(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
void vpx_d45_predictor_16x16_neon(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
#define vpx_d45_predictor_16x16 vpx_d45_predictor_16x16_neon

void vpx_d45_predictor_32x32_c(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
void vpx_d45_predictor_32x32_neon(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
#define vpx_d45_predictor_32x32 vpx_d45_predictor_32x32_neon

void vpx_d45_predictor_4x4_c(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
void vpx_d45_predictor_4x4_neon(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
#define vpx_d45_predictor_4x4 vpx_d45_predictor_4x4_neon

void vpx_d45_predictor_8x8_c(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
void vpx_d45_predictor_8x8_neon(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
#define vpx_d45_predictor_8x8 vpx_d45_predictor_8x8_neon

void vpx_d45e_predictor_4x4_c(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
#define vpx_d45e_predictor_4x4 vpx_d45e_predictor_4x4_c

void vpx_d63_predictor_16x16_c(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
void vpx_d63_predictor_16x16_neon(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
#define vpx_d63_predictor_16x16 vpx_d63_predictor_16x16_neon

void vpx_d63_predictor_32x32_c(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
void vpx_d63_predictor_32x32_neon(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
#define vpx_d63_predictor_32x32 vpx_d63_predictor_32x32_neon

void vpx_d63_predictor_4x4_c(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
void vpx_d63_predictor_4x4_neon(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
#define vpx_d63_predictor_4x4 vpx_d63_predictor_4x4_neon

void vpx_d63_predictor_8x8_c(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
void vpx_d63_predictor_8x8_neon(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
#define vpx_d63_predictor_8x8 vpx_d63_predictor_8x8_neon

void vpx_d63e_predictor_4x4_c(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
#define vpx_d63e_predictor_4x4 vpx_d63e_predictor_4x4_c

void vpx_dc_128_predictor_16x16_c(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
void vpx_dc_128_predictor_16x16_neon(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
#define vpx_dc_128_predictor_16x16 vpx_dc_128_predictor_16x16_neon

void vpx_dc_128_predictor_32x32_c(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
void vpx_dc_128_predictor_32x32_neon(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
#define vpx_dc_128_predictor_32x32 vpx_dc_128_predictor_32x32_neon

void vpx_dc_128_predictor_4x4_c(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
void vpx_dc_128_predictor_4x4_neon(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
#define vpx_dc_128_predictor_4x4 vpx_dc_128_predictor_4x4_neon

void vpx_dc_128_predictor_8x8_c(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
void vpx_dc_128_predictor_8x8_neon(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
#define vpx_dc_128_predictor_8x8 vpx_dc_128_predictor_8x8_neon

void vpx_dc_left_predictor_16x16_c(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
void vpx_dc_left_predictor_16x16_neon(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
#define vpx_dc_left_predictor_16x16 vpx_dc_left_predictor_16x16_neon

void vpx_dc_left_predictor_32x32_c(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
void vpx_dc_left_predictor_32x32_neon(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
#define vpx_dc_left_predictor_32x32 vpx_dc_left_predictor_32x32_neon

void vpx_dc_left_predictor_4x4_c(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
void vpx_dc_left_predictor_4x4_neon(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
#define vpx_dc_left_predictor_4x4 vpx_dc_left_predictor_4x4_neon

void vpx_dc_left_predictor_8x8_c(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
void vpx_dc_left_predictor_8x8_neon(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
#define vpx_dc_left_predictor_8x8 vpx_dc_left_predictor_8x8_neon

void vpx_dc_predictor_16x16_c(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
void vpx_dc_predictor_16x16_neon(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
#define vpx_dc_predictor_16x16 vpx_dc_predictor_16x16_neon

void vpx_dc_predictor_32x32_c(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
void vpx_dc_predictor_32x32_neon(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
#define vpx_dc_predictor_32x32 vpx_dc_predictor_32x32_neon

void vpx_dc_predictor_4x4_c(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
void vpx_dc_predictor_4x4_neon(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
#define vpx_dc_predictor_4x4 vpx_dc_predictor_4x4_neon

void vpx_dc_predictor_8x8_c(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
void vpx_dc_predictor_8x8_neon(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
#define vpx_dc_predictor_8x8 vpx_dc_predictor_8x8_neon

void vpx_dc_top_predictor_16x16_c(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
void vpx_dc_top_predictor_16x16_neon(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
#define vpx_dc_top_predictor_16x16 vpx_dc_top_predictor_16x16_neon

void vpx_dc_top_predictor_32x32_c(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
void vpx_dc_top_predictor_32x32_neon(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
#define vpx_dc_top_predictor_32x32 vpx_dc_top_predictor_32x32_neon

void vpx_dc_top_predictor_4x4_c(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
void vpx_dc_top_predictor_4x4_neon(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
#define vpx_dc_top_predictor_4x4 vpx_dc_top_predictor_4x4_neon

void vpx_dc_top_predictor_8x8_c(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
void vpx_dc_top_predictor_8x8_neon(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
#define vpx_dc_top_predictor_8x8 vpx_dc_top_predictor_8x8_neon

void vpx_fdct16x16_c(const int16_t *input, tran_low_t *output, int stride);
void vpx_fdct16x16_neon(const int16_t *input, tran_low_t *output, int stride);
#define vpx_fdct16x16 vpx_fdct16x16_neon

void vpx_fdct16x16_1_c(const int16_t *input, tran_low_t *output, int stride);
void vpx_fdct16x16_1_neon(const int16_t *input, tran_low_t *output, int stride);
#define vpx_fdct16x16_1 vpx_fdct16x16_1_neon

void vpx_fdct32x32_c(const int16_t *input, tran_low_t *output, int stride);
void vpx_fdct32x32_neon(const int16_t *input, tran_low_t *output, int stride);
#define vpx_fdct32x32 vpx_fdct32x32_neon

void vpx_fdct32x32_1_c(const int16_t *input, tran_low_t *output, int stride);
void vpx_fdct32x32_1_neon(const int16_t *input, tran_low_t *output, int stride);
#define vpx_fdct32x32_1 vpx_fdct32x32_1_neon

void vpx_fdct32x32_rd_c(const int16_t *input, tran_low_t *output, int stride);
void vpx_fdct32x32_rd_neon(const int16_t *input, tran_low_t *output, int stride);
#define vpx_fdct32x32_rd vpx_fdct32x32_rd_neon

void vpx_fdct4x4_c(const int16_t *input, tran_low_t *output, int stride);
void vpx_fdct4x4_neon(const int16_t *input, tran_low_t *output, int stride);
#define vpx_fdct4x4 vpx_fdct4x4_neon

void vpx_fdct4x4_1_c(const int16_t *input, tran_low_t *output, int stride);
void vpx_fdct4x4_1_neon(const int16_t *input, tran_low_t *output, int stride);
#define vpx_fdct4x4_1 vpx_fdct4x4_1_neon

void vpx_fdct8x8_c(const int16_t *input, tran_low_t *output, int stride);
void vpx_fdct8x8_neon(const int16_t *input, tran_low_t *output, int stride);
#define vpx_fdct8x8 vpx_fdct8x8_neon

void vpx_fdct8x8_1_c(const int16_t *input, tran_low_t *output, int stride);
void vpx_fdct8x8_1_neon(const int16_t *input, tran_low_t *output, int stride);
#define vpx_fdct8x8_1 vpx_fdct8x8_1_neon

void vpx_get16x16var_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse, int *sum);
void vpx_get16x16var_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse, int *sum);
void vpx_get16x16var_neon_dotprod(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse, int *sum);
RTCD_EXTERN void (*vpx_get16x16var)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse, int *sum);

unsigned int vpx_get4x4sse_cs_c(const unsigned char *src_ptr, int src_stride, const unsigned char *ref_ptr, int ref_stride);
unsigned int vpx_get4x4sse_cs_neon(const unsigned char *src_ptr, int src_stride, const unsigned char *ref_ptr, int ref_stride);
unsigned int vpx_get4x4sse_cs_neon_dotprod(const unsigned char *src_ptr, int src_stride, const unsigned char *ref_ptr, int ref_stride);
RTCD_EXTERN unsigned int (*vpx_get4x4sse_cs)(const unsigned char *src_ptr, int src_stride, const unsigned char *ref_ptr, int ref_stride);

void vpx_get8x8var_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse, int *sum);
void vpx_get8x8var_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse, int *sum);
void vpx_get8x8var_neon_dotprod(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse, int *sum);
RTCD_EXTERN void (*vpx_get8x8var)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse, int *sum);

unsigned int vpx_get_mb_ss_c(const int16_t *);
#define vpx_get_mb_ss vpx_get_mb_ss_c

void vpx_h_predictor_16x16_c(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
void vpx_h_predictor_16x16_neon(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
#define vpx_h_predictor_16x16 vpx_h_predictor_16x16_neon

void vpx_h_predictor_32x32_c(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
void vpx_h_predictor_32x32_neon(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
#define vpx_h_predictor_32x32 vpx_h_predictor_32x32_neon

void vpx_h_predictor_4x4_c(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
void vpx_h_predictor_4x4_neon(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
#define vpx_h_predictor_4x4 vpx_h_predictor_4x4_neon

void vpx_h_predictor_8x8_c(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
void vpx_h_predictor_8x8_neon(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
#define vpx_h_predictor_8x8 vpx_h_predictor_8x8_neon

void vpx_hadamard_16x16_c(const int16_t *src_diff, ptrdiff_t src_stride, tran_low_t *coeff);
void vpx_hadamard_16x16_neon(const int16_t *src_diff, ptrdiff_t src_stride, tran_low_t *coeff);
#define vpx_hadamard_16x16 vpx_hadamard_16x16_neon

void vpx_hadamard_32x32_c(const int16_t *src_diff, ptrdiff_t src_stride, tran_low_t *coeff);
void vpx_hadamard_32x32_neon(const int16_t *src_diff, ptrdiff_t src_stride, tran_low_t *coeff);
#define vpx_hadamard_32x32 vpx_hadamard_32x32_neon

void vpx_hadamard_8x8_c(const int16_t *src_diff, ptrdiff_t src_stride, tran_low_t *coeff);
void vpx_hadamard_8x8_neon(const int16_t *src_diff, ptrdiff_t src_stride, tran_low_t *coeff);
#define vpx_hadamard_8x8 vpx_hadamard_8x8_neon

void vpx_he_predictor_4x4_c(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
#define vpx_he_predictor_4x4 vpx_he_predictor_4x4_c

void vpx_highbd_10_get16x16var_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse, int *sum);
void vpx_highbd_10_get16x16var_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse, int *sum);
void vpx_highbd_10_get16x16var_sve(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse, int *sum);
RTCD_EXTERN void (*vpx_highbd_10_get16x16var)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse, int *sum);

void vpx_highbd_10_get8x8var_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse, int *sum);
void vpx_highbd_10_get8x8var_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse, int *sum);
void vpx_highbd_10_get8x8var_sve(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse, int *sum);
RTCD_EXTERN void (*vpx_highbd_10_get8x8var)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse, int *sum);

unsigned int vpx_highbd_10_mse16x16_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_10_mse16x16_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_10_mse16x16_sve(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
RTCD_EXTERN unsigned int (*vpx_highbd_10_mse16x16)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);

unsigned int vpx_highbd_10_mse16x8_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_10_mse16x8_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_10_mse16x8_sve(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
RTCD_EXTERN unsigned int (*vpx_highbd_10_mse16x8)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);

unsigned int vpx_highbd_10_mse8x16_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_10_mse8x16_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_10_mse8x16_sve(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
RTCD_EXTERN unsigned int (*vpx_highbd_10_mse8x16)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);

unsigned int vpx_highbd_10_mse8x8_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_10_mse8x8_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_10_mse8x8_sve(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
RTCD_EXTERN unsigned int (*vpx_highbd_10_mse8x8)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);

uint32_t vpx_highbd_10_sub_pixel_avg_variance16x16_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
uint32_t vpx_highbd_10_sub_pixel_avg_variance16x16_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
#define vpx_highbd_10_sub_pixel_avg_variance16x16 vpx_highbd_10_sub_pixel_avg_variance16x16_neon

uint32_t vpx_highbd_10_sub_pixel_avg_variance16x32_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
uint32_t vpx_highbd_10_sub_pixel_avg_variance16x32_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
#define vpx_highbd_10_sub_pixel_avg_variance16x32 vpx_highbd_10_sub_pixel_avg_variance16x32_neon

uint32_t vpx_highbd_10_sub_pixel_avg_variance16x8_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
uint32_t vpx_highbd_10_sub_pixel_avg_variance16x8_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
#define vpx_highbd_10_sub_pixel_avg_variance16x8 vpx_highbd_10_sub_pixel_avg_variance16x8_neon

uint32_t vpx_highbd_10_sub_pixel_avg_variance32x16_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
uint32_t vpx_highbd_10_sub_pixel_avg_variance32x16_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
#define vpx_highbd_10_sub_pixel_avg_variance32x16 vpx_highbd_10_sub_pixel_avg_variance32x16_neon

uint32_t vpx_highbd_10_sub_pixel_avg_variance32x32_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
uint32_t vpx_highbd_10_sub_pixel_avg_variance32x32_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
#define vpx_highbd_10_sub_pixel_avg_variance32x32 vpx_highbd_10_sub_pixel_avg_variance32x32_neon

uint32_t vpx_highbd_10_sub_pixel_avg_variance32x64_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
uint32_t vpx_highbd_10_sub_pixel_avg_variance32x64_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
#define vpx_highbd_10_sub_pixel_avg_variance32x64 vpx_highbd_10_sub_pixel_avg_variance32x64_neon

uint32_t vpx_highbd_10_sub_pixel_avg_variance4x4_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
uint32_t vpx_highbd_10_sub_pixel_avg_variance4x4_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
#define vpx_highbd_10_sub_pixel_avg_variance4x4 vpx_highbd_10_sub_pixel_avg_variance4x4_neon

uint32_t vpx_highbd_10_sub_pixel_avg_variance4x8_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
uint32_t vpx_highbd_10_sub_pixel_avg_variance4x8_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
#define vpx_highbd_10_sub_pixel_avg_variance4x8 vpx_highbd_10_sub_pixel_avg_variance4x8_neon

uint32_t vpx_highbd_10_sub_pixel_avg_variance64x32_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
uint32_t vpx_highbd_10_sub_pixel_avg_variance64x32_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
#define vpx_highbd_10_sub_pixel_avg_variance64x32 vpx_highbd_10_sub_pixel_avg_variance64x32_neon

uint32_t vpx_highbd_10_sub_pixel_avg_variance64x64_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
uint32_t vpx_highbd_10_sub_pixel_avg_variance64x64_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
#define vpx_highbd_10_sub_pixel_avg_variance64x64 vpx_highbd_10_sub_pixel_avg_variance64x64_neon

uint32_t vpx_highbd_10_sub_pixel_avg_variance8x16_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
uint32_t vpx_highbd_10_sub_pixel_avg_variance8x16_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
#define vpx_highbd_10_sub_pixel_avg_variance8x16 vpx_highbd_10_sub_pixel_avg_variance8x16_neon

uint32_t vpx_highbd_10_sub_pixel_avg_variance8x4_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
uint32_t vpx_highbd_10_sub_pixel_avg_variance8x4_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
#define vpx_highbd_10_sub_pixel_avg_variance8x4 vpx_highbd_10_sub_pixel_avg_variance8x4_neon

uint32_t vpx_highbd_10_sub_pixel_avg_variance8x8_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
uint32_t vpx_highbd_10_sub_pixel_avg_variance8x8_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
#define vpx_highbd_10_sub_pixel_avg_variance8x8 vpx_highbd_10_sub_pixel_avg_variance8x8_neon

uint32_t vpx_highbd_10_sub_pixel_variance16x16_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
uint32_t vpx_highbd_10_sub_pixel_variance16x16_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
#define vpx_highbd_10_sub_pixel_variance16x16 vpx_highbd_10_sub_pixel_variance16x16_neon

uint32_t vpx_highbd_10_sub_pixel_variance16x32_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
uint32_t vpx_highbd_10_sub_pixel_variance16x32_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
#define vpx_highbd_10_sub_pixel_variance16x32 vpx_highbd_10_sub_pixel_variance16x32_neon

uint32_t vpx_highbd_10_sub_pixel_variance16x8_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
uint32_t vpx_highbd_10_sub_pixel_variance16x8_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
#define vpx_highbd_10_sub_pixel_variance16x8 vpx_highbd_10_sub_pixel_variance16x8_neon

uint32_t vpx_highbd_10_sub_pixel_variance32x16_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
uint32_t vpx_highbd_10_sub_pixel_variance32x16_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
#define vpx_highbd_10_sub_pixel_variance32x16 vpx_highbd_10_sub_pixel_variance32x16_neon

uint32_t vpx_highbd_10_sub_pixel_variance32x32_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
uint32_t vpx_highbd_10_sub_pixel_variance32x32_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
#define vpx_highbd_10_sub_pixel_variance32x32 vpx_highbd_10_sub_pixel_variance32x32_neon

uint32_t vpx_highbd_10_sub_pixel_variance32x64_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
uint32_t vpx_highbd_10_sub_pixel_variance32x64_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
#define vpx_highbd_10_sub_pixel_variance32x64 vpx_highbd_10_sub_pixel_variance32x64_neon

uint32_t vpx_highbd_10_sub_pixel_variance4x4_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
uint32_t vpx_highbd_10_sub_pixel_variance4x4_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
#define vpx_highbd_10_sub_pixel_variance4x4 vpx_highbd_10_sub_pixel_variance4x4_neon

uint32_t vpx_highbd_10_sub_pixel_variance4x8_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
uint32_t vpx_highbd_10_sub_pixel_variance4x8_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
#define vpx_highbd_10_sub_pixel_variance4x8 vpx_highbd_10_sub_pixel_variance4x8_neon

uint32_t vpx_highbd_10_sub_pixel_variance64x32_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
uint32_t vpx_highbd_10_sub_pixel_variance64x32_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
#define vpx_highbd_10_sub_pixel_variance64x32 vpx_highbd_10_sub_pixel_variance64x32_neon

uint32_t vpx_highbd_10_sub_pixel_variance64x64_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
uint32_t vpx_highbd_10_sub_pixel_variance64x64_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
#define vpx_highbd_10_sub_pixel_variance64x64 vpx_highbd_10_sub_pixel_variance64x64_neon

uint32_t vpx_highbd_10_sub_pixel_variance8x16_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
uint32_t vpx_highbd_10_sub_pixel_variance8x16_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
#define vpx_highbd_10_sub_pixel_variance8x16 vpx_highbd_10_sub_pixel_variance8x16_neon

uint32_t vpx_highbd_10_sub_pixel_variance8x4_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
uint32_t vpx_highbd_10_sub_pixel_variance8x4_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
#define vpx_highbd_10_sub_pixel_variance8x4 vpx_highbd_10_sub_pixel_variance8x4_neon

uint32_t vpx_highbd_10_sub_pixel_variance8x8_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
uint32_t vpx_highbd_10_sub_pixel_variance8x8_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
#define vpx_highbd_10_sub_pixel_variance8x8 vpx_highbd_10_sub_pixel_variance8x8_neon

unsigned int vpx_highbd_10_variance16x16_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_10_variance16x16_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_10_variance16x16_sve(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
RTCD_EXTERN unsigned int (*vpx_highbd_10_variance16x16)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);

unsigned int vpx_highbd_10_variance16x32_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_10_variance16x32_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_10_variance16x32_sve(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
RTCD_EXTERN unsigned int (*vpx_highbd_10_variance16x32)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);

unsigned int vpx_highbd_10_variance16x8_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_10_variance16x8_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_10_variance16x8_sve(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
RTCD_EXTERN unsigned int (*vpx_highbd_10_variance16x8)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);

unsigned int vpx_highbd_10_variance32x16_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_10_variance32x16_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_10_variance32x16_sve(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
RTCD_EXTERN unsigned int (*vpx_highbd_10_variance32x16)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);

unsigned int vpx_highbd_10_variance32x32_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_10_variance32x32_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_10_variance32x32_sve(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
RTCD_EXTERN unsigned int (*vpx_highbd_10_variance32x32)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);

unsigned int vpx_highbd_10_variance32x64_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_10_variance32x64_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_10_variance32x64_sve(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
RTCD_EXTERN unsigned int (*vpx_highbd_10_variance32x64)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);

unsigned int vpx_highbd_10_variance4x4_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_10_variance4x4_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_10_variance4x4_sve(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
RTCD_EXTERN unsigned int (*vpx_highbd_10_variance4x4)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);

unsigned int vpx_highbd_10_variance4x8_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_10_variance4x8_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_10_variance4x8_sve(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
RTCD_EXTERN unsigned int (*vpx_highbd_10_variance4x8)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);

unsigned int vpx_highbd_10_variance64x32_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_10_variance64x32_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_10_variance64x32_sve(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
RTCD_EXTERN unsigned int (*vpx_highbd_10_variance64x32)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);

unsigned int vpx_highbd_10_variance64x64_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_10_variance64x64_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_10_variance64x64_sve(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
RTCD_EXTERN unsigned int (*vpx_highbd_10_variance64x64)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);

unsigned int vpx_highbd_10_variance8x16_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_10_variance8x16_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_10_variance8x16_sve(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
RTCD_EXTERN unsigned int (*vpx_highbd_10_variance8x16)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);

unsigned int vpx_highbd_10_variance8x4_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_10_variance8x4_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_10_variance8x4_sve(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
RTCD_EXTERN unsigned int (*vpx_highbd_10_variance8x4)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);

unsigned int vpx_highbd_10_variance8x8_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_10_variance8x8_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_10_variance8x8_sve(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
RTCD_EXTERN unsigned int (*vpx_highbd_10_variance8x8)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);

void vpx_highbd_12_get16x16var_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse, int *sum);
void vpx_highbd_12_get16x16var_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse, int *sum);
void vpx_highbd_12_get16x16var_sve(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse, int *sum);
RTCD_EXTERN void (*vpx_highbd_12_get16x16var)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse, int *sum);

void vpx_highbd_12_get8x8var_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse, int *sum);
void vpx_highbd_12_get8x8var_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse, int *sum);
void vpx_highbd_12_get8x8var_sve(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse, int *sum);
RTCD_EXTERN void (*vpx_highbd_12_get8x8var)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse, int *sum);

unsigned int vpx_highbd_12_mse16x16_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_12_mse16x16_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_12_mse16x16_sve(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
RTCD_EXTERN unsigned int (*vpx_highbd_12_mse16x16)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);

unsigned int vpx_highbd_12_mse16x8_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_12_mse16x8_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_12_mse16x8_sve(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
RTCD_EXTERN unsigned int (*vpx_highbd_12_mse16x8)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);

unsigned int vpx_highbd_12_mse8x16_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_12_mse8x16_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_12_mse8x16_sve(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
RTCD_EXTERN unsigned int (*vpx_highbd_12_mse8x16)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);

unsigned int vpx_highbd_12_mse8x8_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_12_mse8x8_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_12_mse8x8_sve(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
RTCD_EXTERN unsigned int (*vpx_highbd_12_mse8x8)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);

uint32_t vpx_highbd_12_sub_pixel_avg_variance16x16_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
uint32_t vpx_highbd_12_sub_pixel_avg_variance16x16_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
#define vpx_highbd_12_sub_pixel_avg_variance16x16 vpx_highbd_12_sub_pixel_avg_variance16x16_neon

uint32_t vpx_highbd_12_sub_pixel_avg_variance16x32_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
uint32_t vpx_highbd_12_sub_pixel_avg_variance16x32_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
#define vpx_highbd_12_sub_pixel_avg_variance16x32 vpx_highbd_12_sub_pixel_avg_variance16x32_neon

uint32_t vpx_highbd_12_sub_pixel_avg_variance16x8_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
uint32_t vpx_highbd_12_sub_pixel_avg_variance16x8_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
#define vpx_highbd_12_sub_pixel_avg_variance16x8 vpx_highbd_12_sub_pixel_avg_variance16x8_neon

uint32_t vpx_highbd_12_sub_pixel_avg_variance32x16_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
uint32_t vpx_highbd_12_sub_pixel_avg_variance32x16_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
#define vpx_highbd_12_sub_pixel_avg_variance32x16 vpx_highbd_12_sub_pixel_avg_variance32x16_neon

uint32_t vpx_highbd_12_sub_pixel_avg_variance32x32_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
uint32_t vpx_highbd_12_sub_pixel_avg_variance32x32_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
#define vpx_highbd_12_sub_pixel_avg_variance32x32 vpx_highbd_12_sub_pixel_avg_variance32x32_neon

uint32_t vpx_highbd_12_sub_pixel_avg_variance32x64_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
uint32_t vpx_highbd_12_sub_pixel_avg_variance32x64_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
#define vpx_highbd_12_sub_pixel_avg_variance32x64 vpx_highbd_12_sub_pixel_avg_variance32x64_neon

uint32_t vpx_highbd_12_sub_pixel_avg_variance4x4_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
uint32_t vpx_highbd_12_sub_pixel_avg_variance4x4_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
#define vpx_highbd_12_sub_pixel_avg_variance4x4 vpx_highbd_12_sub_pixel_avg_variance4x4_neon

uint32_t vpx_highbd_12_sub_pixel_avg_variance4x8_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
uint32_t vpx_highbd_12_sub_pixel_avg_variance4x8_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
#define vpx_highbd_12_sub_pixel_avg_variance4x8 vpx_highbd_12_sub_pixel_avg_variance4x8_neon

uint32_t vpx_highbd_12_sub_pixel_avg_variance64x32_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
uint32_t vpx_highbd_12_sub_pixel_avg_variance64x32_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
#define vpx_highbd_12_sub_pixel_avg_variance64x32 vpx_highbd_12_sub_pixel_avg_variance64x32_neon

uint32_t vpx_highbd_12_sub_pixel_avg_variance64x64_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
uint32_t vpx_highbd_12_sub_pixel_avg_variance64x64_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
#define vpx_highbd_12_sub_pixel_avg_variance64x64 vpx_highbd_12_sub_pixel_avg_variance64x64_neon

uint32_t vpx_highbd_12_sub_pixel_avg_variance8x16_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
uint32_t vpx_highbd_12_sub_pixel_avg_variance8x16_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
#define vpx_highbd_12_sub_pixel_avg_variance8x16 vpx_highbd_12_sub_pixel_avg_variance8x16_neon

uint32_t vpx_highbd_12_sub_pixel_avg_variance8x4_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
uint32_t vpx_highbd_12_sub_pixel_avg_variance8x4_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
#define vpx_highbd_12_sub_pixel_avg_variance8x4 vpx_highbd_12_sub_pixel_avg_variance8x4_neon

uint32_t vpx_highbd_12_sub_pixel_avg_variance8x8_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
uint32_t vpx_highbd_12_sub_pixel_avg_variance8x8_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
#define vpx_highbd_12_sub_pixel_avg_variance8x8 vpx_highbd_12_sub_pixel_avg_variance8x8_neon

uint32_t vpx_highbd_12_sub_pixel_variance16x16_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
uint32_t vpx_highbd_12_sub_pixel_variance16x16_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
#define vpx_highbd_12_sub_pixel_variance16x16 vpx_highbd_12_sub_pixel_variance16x16_neon

uint32_t vpx_highbd_12_sub_pixel_variance16x32_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
uint32_t vpx_highbd_12_sub_pixel_variance16x32_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
#define vpx_highbd_12_sub_pixel_variance16x32 vpx_highbd_12_sub_pixel_variance16x32_neon

uint32_t vpx_highbd_12_sub_pixel_variance16x8_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
uint32_t vpx_highbd_12_sub_pixel_variance16x8_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
#define vpx_highbd_12_sub_pixel_variance16x8 vpx_highbd_12_sub_pixel_variance16x8_neon

uint32_t vpx_highbd_12_sub_pixel_variance32x16_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
uint32_t vpx_highbd_12_sub_pixel_variance32x16_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
#define vpx_highbd_12_sub_pixel_variance32x16 vpx_highbd_12_sub_pixel_variance32x16_neon

uint32_t vpx_highbd_12_sub_pixel_variance32x32_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
uint32_t vpx_highbd_12_sub_pixel_variance32x32_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
#define vpx_highbd_12_sub_pixel_variance32x32 vpx_highbd_12_sub_pixel_variance32x32_neon

uint32_t vpx_highbd_12_sub_pixel_variance32x64_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
uint32_t vpx_highbd_12_sub_pixel_variance32x64_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
#define vpx_highbd_12_sub_pixel_variance32x64 vpx_highbd_12_sub_pixel_variance32x64_neon

uint32_t vpx_highbd_12_sub_pixel_variance4x4_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
uint32_t vpx_highbd_12_sub_pixel_variance4x4_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
#define vpx_highbd_12_sub_pixel_variance4x4 vpx_highbd_12_sub_pixel_variance4x4_neon

uint32_t vpx_highbd_12_sub_pixel_variance4x8_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
uint32_t vpx_highbd_12_sub_pixel_variance4x8_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
#define vpx_highbd_12_sub_pixel_variance4x8 vpx_highbd_12_sub_pixel_variance4x8_neon

uint32_t vpx_highbd_12_sub_pixel_variance64x32_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
uint32_t vpx_highbd_12_sub_pixel_variance64x32_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
#define vpx_highbd_12_sub_pixel_variance64x32 vpx_highbd_12_sub_pixel_variance64x32_neon

uint32_t vpx_highbd_12_sub_pixel_variance64x64_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
uint32_t vpx_highbd_12_sub_pixel_variance64x64_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
#define vpx_highbd_12_sub_pixel_variance64x64 vpx_highbd_12_sub_pixel_variance64x64_neon

uint32_t vpx_highbd_12_sub_pixel_variance8x16_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
uint32_t vpx_highbd_12_sub_pixel_variance8x16_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
#define vpx_highbd_12_sub_pixel_variance8x16 vpx_highbd_12_sub_pixel_variance8x16_neon

uint32_t vpx_highbd_12_sub_pixel_variance8x4_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
uint32_t vpx_highbd_12_sub_pixel_variance8x4_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
#define vpx_highbd_12_sub_pixel_variance8x4 vpx_highbd_12_sub_pixel_variance8x4_neon

uint32_t vpx_highbd_12_sub_pixel_variance8x8_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
uint32_t vpx_highbd_12_sub_pixel_variance8x8_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
#define vpx_highbd_12_sub_pixel_variance8x8 vpx_highbd_12_sub_pixel_variance8x8_neon

unsigned int vpx_highbd_12_variance16x16_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_12_variance16x16_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_12_variance16x16_sve(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
RTCD_EXTERN unsigned int (*vpx_highbd_12_variance16x16)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);

unsigned int vpx_highbd_12_variance16x32_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_12_variance16x32_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_12_variance16x32_sve(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
RTCD_EXTERN unsigned int (*vpx_highbd_12_variance16x32)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);

unsigned int vpx_highbd_12_variance16x8_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_12_variance16x8_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_12_variance16x8_sve(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
RTCD_EXTERN unsigned int (*vpx_highbd_12_variance16x8)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);

unsigned int vpx_highbd_12_variance32x16_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_12_variance32x16_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_12_variance32x16_sve(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
RTCD_EXTERN unsigned int (*vpx_highbd_12_variance32x16)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);

unsigned int vpx_highbd_12_variance32x32_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_12_variance32x32_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_12_variance32x32_sve(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
RTCD_EXTERN unsigned int (*vpx_highbd_12_variance32x32)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);

unsigned int vpx_highbd_12_variance32x64_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_12_variance32x64_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_12_variance32x64_sve(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
RTCD_EXTERN unsigned int (*vpx_highbd_12_variance32x64)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);

unsigned int vpx_highbd_12_variance4x4_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_12_variance4x4_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_12_variance4x4_sve(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
RTCD_EXTERN unsigned int (*vpx_highbd_12_variance4x4)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);

unsigned int vpx_highbd_12_variance4x8_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_12_variance4x8_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_12_variance4x8_sve(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
RTCD_EXTERN unsigned int (*vpx_highbd_12_variance4x8)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);

unsigned int vpx_highbd_12_variance64x32_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_12_variance64x32_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_12_variance64x32_sve(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
RTCD_EXTERN unsigned int (*vpx_highbd_12_variance64x32)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);

unsigned int vpx_highbd_12_variance64x64_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_12_variance64x64_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_12_variance64x64_sve(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
RTCD_EXTERN unsigned int (*vpx_highbd_12_variance64x64)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);

unsigned int vpx_highbd_12_variance8x16_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_12_variance8x16_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_12_variance8x16_sve(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
RTCD_EXTERN unsigned int (*vpx_highbd_12_variance8x16)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);

unsigned int vpx_highbd_12_variance8x4_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_12_variance8x4_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_12_variance8x4_sve(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
RTCD_EXTERN unsigned int (*vpx_highbd_12_variance8x4)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);

unsigned int vpx_highbd_12_variance8x8_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_12_variance8x8_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_12_variance8x8_sve(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
RTCD_EXTERN unsigned int (*vpx_highbd_12_variance8x8)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);

void vpx_highbd_8_get16x16var_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse, int *sum);
void vpx_highbd_8_get16x16var_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse, int *sum);
void vpx_highbd_8_get16x16var_sve(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse, int *sum);
RTCD_EXTERN void (*vpx_highbd_8_get16x16var)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse, int *sum);

void vpx_highbd_8_get8x8var_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse, int *sum);
void vpx_highbd_8_get8x8var_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse, int *sum);
void vpx_highbd_8_get8x8var_sve(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse, int *sum);
RTCD_EXTERN void (*vpx_highbd_8_get8x8var)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse, int *sum);

unsigned int vpx_highbd_8_mse16x16_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_8_mse16x16_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_8_mse16x16_neon_dotprod(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
RTCD_EXTERN unsigned int (*vpx_highbd_8_mse16x16)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);

unsigned int vpx_highbd_8_mse16x8_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_8_mse16x8_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_8_mse16x8_neon_dotprod(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
RTCD_EXTERN unsigned int (*vpx_highbd_8_mse16x8)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);

unsigned int vpx_highbd_8_mse8x16_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_8_mse8x16_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_8_mse8x16_neon_dotprod(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
RTCD_EXTERN unsigned int (*vpx_highbd_8_mse8x16)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);

unsigned int vpx_highbd_8_mse8x8_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_8_mse8x8_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_8_mse8x8_neon_dotprod(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
RTCD_EXTERN unsigned int (*vpx_highbd_8_mse8x8)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);

uint32_t vpx_highbd_8_sub_pixel_avg_variance16x16_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
uint32_t vpx_highbd_8_sub_pixel_avg_variance16x16_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
#define vpx_highbd_8_sub_pixel_avg_variance16x16 vpx_highbd_8_sub_pixel_avg_variance16x16_neon

uint32_t vpx_highbd_8_sub_pixel_avg_variance16x32_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
uint32_t vpx_highbd_8_sub_pixel_avg_variance16x32_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
#define vpx_highbd_8_sub_pixel_avg_variance16x32 vpx_highbd_8_sub_pixel_avg_variance16x32_neon

uint32_t vpx_highbd_8_sub_pixel_avg_variance16x8_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
uint32_t vpx_highbd_8_sub_pixel_avg_variance16x8_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
#define vpx_highbd_8_sub_pixel_avg_variance16x8 vpx_highbd_8_sub_pixel_avg_variance16x8_neon

uint32_t vpx_highbd_8_sub_pixel_avg_variance32x16_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
uint32_t vpx_highbd_8_sub_pixel_avg_variance32x16_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
#define vpx_highbd_8_sub_pixel_avg_variance32x16 vpx_highbd_8_sub_pixel_avg_variance32x16_neon

uint32_t vpx_highbd_8_sub_pixel_avg_variance32x32_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
uint32_t vpx_highbd_8_sub_pixel_avg_variance32x32_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
#define vpx_highbd_8_sub_pixel_avg_variance32x32 vpx_highbd_8_sub_pixel_avg_variance32x32_neon

uint32_t vpx_highbd_8_sub_pixel_avg_variance32x64_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
uint32_t vpx_highbd_8_sub_pixel_avg_variance32x64_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
#define vpx_highbd_8_sub_pixel_avg_variance32x64 vpx_highbd_8_sub_pixel_avg_variance32x64_neon

uint32_t vpx_highbd_8_sub_pixel_avg_variance4x4_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
uint32_t vpx_highbd_8_sub_pixel_avg_variance4x4_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
#define vpx_highbd_8_sub_pixel_avg_variance4x4 vpx_highbd_8_sub_pixel_avg_variance4x4_neon

uint32_t vpx_highbd_8_sub_pixel_avg_variance4x8_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
uint32_t vpx_highbd_8_sub_pixel_avg_variance4x8_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
#define vpx_highbd_8_sub_pixel_avg_variance4x8 vpx_highbd_8_sub_pixel_avg_variance4x8_neon

uint32_t vpx_highbd_8_sub_pixel_avg_variance64x32_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
uint32_t vpx_highbd_8_sub_pixel_avg_variance64x32_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
#define vpx_highbd_8_sub_pixel_avg_variance64x32 vpx_highbd_8_sub_pixel_avg_variance64x32_neon

uint32_t vpx_highbd_8_sub_pixel_avg_variance64x64_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
uint32_t vpx_highbd_8_sub_pixel_avg_variance64x64_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
#define vpx_highbd_8_sub_pixel_avg_variance64x64 vpx_highbd_8_sub_pixel_avg_variance64x64_neon

uint32_t vpx_highbd_8_sub_pixel_avg_variance8x16_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
uint32_t vpx_highbd_8_sub_pixel_avg_variance8x16_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
#define vpx_highbd_8_sub_pixel_avg_variance8x16 vpx_highbd_8_sub_pixel_avg_variance8x16_neon

uint32_t vpx_highbd_8_sub_pixel_avg_variance8x4_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
uint32_t vpx_highbd_8_sub_pixel_avg_variance8x4_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
#define vpx_highbd_8_sub_pixel_avg_variance8x4 vpx_highbd_8_sub_pixel_avg_variance8x4_neon

uint32_t vpx_highbd_8_sub_pixel_avg_variance8x8_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
uint32_t vpx_highbd_8_sub_pixel_avg_variance8x8_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
#define vpx_highbd_8_sub_pixel_avg_variance8x8 vpx_highbd_8_sub_pixel_avg_variance8x8_neon

uint32_t vpx_highbd_8_sub_pixel_variance16x16_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
uint32_t vpx_highbd_8_sub_pixel_variance16x16_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
#define vpx_highbd_8_sub_pixel_variance16x16 vpx_highbd_8_sub_pixel_variance16x16_neon

uint32_t vpx_highbd_8_sub_pixel_variance16x32_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
uint32_t vpx_highbd_8_sub_pixel_variance16x32_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
#define vpx_highbd_8_sub_pixel_variance16x32 vpx_highbd_8_sub_pixel_variance16x32_neon

uint32_t vpx_highbd_8_sub_pixel_variance16x8_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
uint32_t vpx_highbd_8_sub_pixel_variance16x8_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
#define vpx_highbd_8_sub_pixel_variance16x8 vpx_highbd_8_sub_pixel_variance16x8_neon

uint32_t vpx_highbd_8_sub_pixel_variance32x16_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
uint32_t vpx_highbd_8_sub_pixel_variance32x16_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
#define vpx_highbd_8_sub_pixel_variance32x16 vpx_highbd_8_sub_pixel_variance32x16_neon

uint32_t vpx_highbd_8_sub_pixel_variance32x32_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
uint32_t vpx_highbd_8_sub_pixel_variance32x32_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
#define vpx_highbd_8_sub_pixel_variance32x32 vpx_highbd_8_sub_pixel_variance32x32_neon

uint32_t vpx_highbd_8_sub_pixel_variance32x64_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
uint32_t vpx_highbd_8_sub_pixel_variance32x64_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
#define vpx_highbd_8_sub_pixel_variance32x64 vpx_highbd_8_sub_pixel_variance32x64_neon

uint32_t vpx_highbd_8_sub_pixel_variance4x4_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
uint32_t vpx_highbd_8_sub_pixel_variance4x4_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
#define vpx_highbd_8_sub_pixel_variance4x4 vpx_highbd_8_sub_pixel_variance4x4_neon

uint32_t vpx_highbd_8_sub_pixel_variance4x8_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
uint32_t vpx_highbd_8_sub_pixel_variance4x8_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
#define vpx_highbd_8_sub_pixel_variance4x8 vpx_highbd_8_sub_pixel_variance4x8_neon

uint32_t vpx_highbd_8_sub_pixel_variance64x32_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
uint32_t vpx_highbd_8_sub_pixel_variance64x32_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
#define vpx_highbd_8_sub_pixel_variance64x32 vpx_highbd_8_sub_pixel_variance64x32_neon

uint32_t vpx_highbd_8_sub_pixel_variance64x64_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
uint32_t vpx_highbd_8_sub_pixel_variance64x64_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
#define vpx_highbd_8_sub_pixel_variance64x64 vpx_highbd_8_sub_pixel_variance64x64_neon

uint32_t vpx_highbd_8_sub_pixel_variance8x16_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
uint32_t vpx_highbd_8_sub_pixel_variance8x16_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
#define vpx_highbd_8_sub_pixel_variance8x16 vpx_highbd_8_sub_pixel_variance8x16_neon

uint32_t vpx_highbd_8_sub_pixel_variance8x4_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
uint32_t vpx_highbd_8_sub_pixel_variance8x4_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
#define vpx_highbd_8_sub_pixel_variance8x4 vpx_highbd_8_sub_pixel_variance8x4_neon

uint32_t vpx_highbd_8_sub_pixel_variance8x8_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
uint32_t vpx_highbd_8_sub_pixel_variance8x8_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
#define vpx_highbd_8_sub_pixel_variance8x8 vpx_highbd_8_sub_pixel_variance8x8_neon

unsigned int vpx_highbd_8_variance16x16_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_8_variance16x16_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_8_variance16x16_sve(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
RTCD_EXTERN unsigned int (*vpx_highbd_8_variance16x16)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);

unsigned int vpx_highbd_8_variance16x32_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_8_variance16x32_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_8_variance16x32_sve(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
RTCD_EXTERN unsigned int (*vpx_highbd_8_variance16x32)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);

unsigned int vpx_highbd_8_variance16x8_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_8_variance16x8_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_8_variance16x8_sve(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
RTCD_EXTERN unsigned int (*vpx_highbd_8_variance16x8)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);

unsigned int vpx_highbd_8_variance32x16_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_8_variance32x16_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_8_variance32x16_sve(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
RTCD_EXTERN unsigned int (*vpx_highbd_8_variance32x16)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);

unsigned int vpx_highbd_8_variance32x32_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_8_variance32x32_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_8_variance32x32_sve(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
RTCD_EXTERN unsigned int (*vpx_highbd_8_variance32x32)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);

unsigned int vpx_highbd_8_variance32x64_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_8_variance32x64_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_8_variance32x64_sve(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
RTCD_EXTERN unsigned int (*vpx_highbd_8_variance32x64)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);

unsigned int vpx_highbd_8_variance4x4_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_8_variance4x4_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_8_variance4x4_sve(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
RTCD_EXTERN unsigned int (*vpx_highbd_8_variance4x4)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);

unsigned int vpx_highbd_8_variance4x8_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_8_variance4x8_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_8_variance4x8_sve(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
RTCD_EXTERN unsigned int (*vpx_highbd_8_variance4x8)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);

unsigned int vpx_highbd_8_variance64x32_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_8_variance64x32_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_8_variance64x32_sve(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
RTCD_EXTERN unsigned int (*vpx_highbd_8_variance64x32)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);

unsigned int vpx_highbd_8_variance64x64_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_8_variance64x64_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_8_variance64x64_sve(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
RTCD_EXTERN unsigned int (*vpx_highbd_8_variance64x64)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);

unsigned int vpx_highbd_8_variance8x16_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_8_variance8x16_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_8_variance8x16_sve(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
RTCD_EXTERN unsigned int (*vpx_highbd_8_variance8x16)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);

unsigned int vpx_highbd_8_variance8x4_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_8_variance8x4_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_8_variance8x4_sve(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
RTCD_EXTERN unsigned int (*vpx_highbd_8_variance8x4)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);

unsigned int vpx_highbd_8_variance8x8_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_8_variance8x8_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_highbd_8_variance8x8_sve(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
RTCD_EXTERN unsigned int (*vpx_highbd_8_variance8x8)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);

unsigned int vpx_highbd_avg_4x4_c(const uint8_t *s8, int p);
unsigned int vpx_highbd_avg_4x4_neon(const uint8_t *s8, int p);
#define vpx_highbd_avg_4x4 vpx_highbd_avg_4x4_neon

unsigned int vpx_highbd_avg_8x8_c(const uint8_t *s8, int p);
unsigned int vpx_highbd_avg_8x8_neon(const uint8_t *s8, int p);
#define vpx_highbd_avg_8x8 vpx_highbd_avg_8x8_neon

void vpx_highbd_comp_avg_pred_c(uint16_t *comp_pred, const uint16_t *pred, int width, int height, const uint16_t *ref, int ref_stride);
void vpx_highbd_comp_avg_pred_neon(uint16_t *comp_pred, const uint16_t *pred, int width, int height, const uint16_t *ref, int ref_stride);
#define vpx_highbd_comp_avg_pred vpx_highbd_comp_avg_pred_neon

void vpx_highbd_convolve8_c(const uint16_t *src, ptrdiff_t src_stride, uint16_t *dst, ptrdiff_t dst_stride, const InterpKernel *filter, int x0_q4, int x_step_q4, int y0_q4, int y_step_q4, int w, int h, int bd);
void vpx_highbd_convolve8_neon(const uint16_t *src, ptrdiff_t src_stride, uint16_t *dst, ptrdiff_t dst_stride, const InterpKernel *filter, int x0_q4, int x_step_q4, int y0_q4, int y_step_q4, int w, int h, int bd);
void vpx_highbd_convolve8_sve2(const uint16_t *src, ptrdiff_t src_stride, uint16_t *dst, ptrdiff_t dst_stride, const InterpKernel *filter, int x0_q4, int x_step_q4, int y0_q4, int y_step_q4, int w, int h, int bd);
RTCD_EXTERN void (*vpx_highbd_convolve8)(const uint16_t *src, ptrdiff_t src_stride, uint16_t *dst, ptrdiff_t dst_stride, const InterpKernel *filter, int x0_q4, int x_step_q4, int y0_q4, int y_step_q4, int w, int h, int bd);

void vpx_highbd_convolve8_avg_c(const uint16_t *src, ptrdiff_t src_stride, uint16_t *dst, ptrdiff_t dst_stride, const InterpKernel *filter, int x0_q4, int x_step_q4, int y0_q4, int y_step_q4, int w, int h, int bd);
void vpx_highbd_convolve8_avg_neon(const uint16_t *src, ptrdiff_t src_stride, uint16_t *dst, ptrdiff_t dst_stride, const InterpKernel *filter, int x0_q4, int x_step_q4, int y0_q4, int y_step_q4, int w, int h, int bd);
void vpx_highbd_convolve8_avg_sve2(const uint16_t *src, ptrdiff_t src_stride, uint16_t *dst, ptrdiff_t dst_stride, const InterpKernel *filter, int x0_q4, int x_step_q4, int y0_q4, int y_step_q4, int w, int h, int bd);
RTCD_EXTERN void (*vpx_highbd_convolve8_avg)(const uint16_t *src, ptrdiff_t src_stride, uint16_t *dst, ptrdiff_t dst_stride, const InterpKernel *filter, int x0_q4, int x_step_q4, int y0_q4, int y_step_q4, int w, int h, int bd);

void vpx_highbd_convolve8_avg_horiz_c(const uint16_t *src, ptrdiff_t src_stride, uint16_t *dst, ptrdiff_t dst_stride, const InterpKernel *filter, int x0_q4, int x_step_q4, int y0_q4, int y_step_q4, int w, int h, int bd);
void vpx_highbd_convolve8_avg_horiz_neon(const uint16_t *src, ptrdiff_t src_stride, uint16_t *dst, ptrdiff_t dst_stride, const InterpKernel *filter, int x0_q4, int x_step_q4, int y0_q4, int y_step_q4, int w, int h, int bd);
void vpx_highbd_convolve8_avg_horiz_sve(const uint16_t *src, ptrdiff_t src_stride, uint16_t *dst, ptrdiff_t dst_stride, const InterpKernel *filter, int x0_q4, int x_step_q4, int y0_q4, int y_step_q4, int w, int h, int bd);
RTCD_EXTERN void (*vpx_highbd_convolve8_avg_horiz)(const uint16_t *src, ptrdiff_t src_stride, uint16_t *dst, ptrdiff_t dst_stride, const InterpKernel *filter, int x0_q4, int x_step_q4, int y0_q4, int y_step_q4, int w, int h, int bd);

void vpx_highbd_convolve8_avg_vert_c(const uint16_t *src, ptrdiff_t src_stride, uint16_t *dst, ptrdiff_t dst_stride, const InterpKernel *filter, int x0_q4, int x_step_q4, int y0_q4, int y_step_q4, int w, int h, int bd);
void vpx_highbd_convolve8_avg_vert_neon(const uint16_t *src, ptrdiff_t src_stride, uint16_t *dst, ptrdiff_t dst_stride, const InterpKernel *filter, int x0_q4, int x_step_q4, int y0_q4, int y_step_q4, int w, int h, int bd);
void vpx_highbd_convolve8_avg_vert_sve2(const uint16_t *src, ptrdiff_t src_stride, uint16_t *dst, ptrdiff_t dst_stride, const InterpKernel *filter, int x0_q4, int x_step_q4, int y0_q4, int y_step_q4, int w, int h, int bd);
RTCD_EXTERN void (*vpx_highbd_convolve8_avg_vert)(const uint16_t *src, ptrdiff_t src_stride, uint16_t *dst, ptrdiff_t dst_stride, const InterpKernel *filter, int x0_q4, int x_step_q4, int y0_q4, int y_step_q4, int w, int h, int bd);

void vpx_highbd_convolve8_horiz_c(const uint16_t *src, ptrdiff_t src_stride, uint16_t *dst, ptrdiff_t dst_stride, const InterpKernel *filter, int x0_q4, int x_step_q4, int y0_q4, int y_step_q4, int w, int h, int bd);
void vpx_highbd_convolve8_horiz_neon(const uint16_t *src, ptrdiff_t src_stride, uint16_t *dst, ptrdiff_t dst_stride, const InterpKernel *filter, int x0_q4, int x_step_q4, int y0_q4, int y_step_q4, int w, int h, int bd);
void vpx_highbd_convolve8_horiz_sve(const uint16_t *src, ptrdiff_t src_stride, uint16_t *dst, ptrdiff_t dst_stride, const InterpKernel *filter, int x0_q4, int x_step_q4, int y0_q4, int y_step_q4, int w, int h, int bd);
RTCD_EXTERN void (*vpx_highbd_convolve8_horiz)(const uint16_t *src, ptrdiff_t src_stride, uint16_t *dst, ptrdiff_t dst_stride, const InterpKernel *filter, int x0_q4, int x_step_q4, int y0_q4, int y_step_q4, int w, int h, int bd);

void vpx_highbd_convolve8_vert_c(const uint16_t *src, ptrdiff_t src_stride, uint16_t *dst, ptrdiff_t dst_stride, const InterpKernel *filter, int x0_q4, int x_step_q4, int y0_q4, int y_step_q4, int w, int h, int bd);
void vpx_highbd_convolve8_vert_neon(const uint16_t *src, ptrdiff_t src_stride, uint16_t *dst, ptrdiff_t dst_stride, const InterpKernel *filter, int x0_q4, int x_step_q4, int y0_q4, int y_step_q4, int w, int h, int bd);
void vpx_highbd_convolve8_vert_sve2(const uint16_t *src, ptrdiff_t src_stride, uint16_t *dst, ptrdiff_t dst_stride, const InterpKernel *filter, int x0_q4, int x_step_q4, int y0_q4, int y_step_q4, int w, int h, int bd);
RTCD_EXTERN void (*vpx_highbd_convolve8_vert)(const uint16_t *src, ptrdiff_t src_stride, uint16_t *dst, ptrdiff_t dst_stride, const InterpKernel *filter, int x0_q4, int x_step_q4, int y0_q4, int y_step_q4, int w, int h, int bd);

void vpx_highbd_convolve_avg_c(const uint16_t *src, ptrdiff_t src_stride, uint16_t *dst, ptrdiff_t dst_stride, const InterpKernel *filter, int x0_q4, int x_step_q4, int y0_q4, int y_step_q4, int w, int h, int bd);
void vpx_highbd_convolve_avg_neon(const uint16_t *src, ptrdiff_t src_stride, uint16_t *dst, ptrdiff_t dst_stride, const InterpKernel *filter, int x0_q4, int x_step_q4, int y0_q4, int y_step_q4, int w, int h, int bd);
#define vpx_highbd_convolve_avg vpx_highbd_convolve_avg_neon

void vpx_highbd_convolve_copy_c(const uint16_t *src, ptrdiff_t src_stride, uint16_t *dst, ptrdiff_t dst_stride, const InterpKernel *filter, int x0_q4, int x_step_q4, int y0_q4, int y_step_q4, int w, int h, int bd);
void vpx_highbd_convolve_copy_neon(const uint16_t *src, ptrdiff_t src_stride, uint16_t *dst, ptrdiff_t dst_stride, const InterpKernel *filter, int x0_q4, int x_step_q4, int y0_q4, int y_step_q4, int w, int h, int bd);
#define vpx_highbd_convolve_copy vpx_highbd_convolve_copy_neon

void vpx_highbd_d117_predictor_16x16_c(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
void vpx_highbd_d117_predictor_16x16_neon(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
#define vpx_highbd_d117_predictor_16x16 vpx_highbd_d117_predictor_16x16_neon

void vpx_highbd_d117_predictor_32x32_c(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
void vpx_highbd_d117_predictor_32x32_neon(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
#define vpx_highbd_d117_predictor_32x32 vpx_highbd_d117_predictor_32x32_neon

void vpx_highbd_d117_predictor_4x4_c(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
void vpx_highbd_d117_predictor_4x4_neon(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
#define vpx_highbd_d117_predictor_4x4 vpx_highbd_d117_predictor_4x4_neon

void vpx_highbd_d117_predictor_8x8_c(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
void vpx_highbd_d117_predictor_8x8_neon(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
#define vpx_highbd_d117_predictor_8x8 vpx_highbd_d117_predictor_8x8_neon

void vpx_highbd_d135_predictor_16x16_c(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
void vpx_highbd_d135_predictor_16x16_neon(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
#define vpx_highbd_d135_predictor_16x16 vpx_highbd_d135_predictor_16x16_neon

void vpx_highbd_d135_predictor_32x32_c(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
void vpx_highbd_d135_predictor_32x32_neon(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
#define vpx_highbd_d135_predictor_32x32 vpx_highbd_d135_predictor_32x32_neon

void vpx_highbd_d135_predictor_4x4_c(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
void vpx_highbd_d135_predictor_4x4_neon(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
#define vpx_highbd_d135_predictor_4x4 vpx_highbd_d135_predictor_4x4_neon

void vpx_highbd_d135_predictor_8x8_c(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
void vpx_highbd_d135_predictor_8x8_neon(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
#define vpx_highbd_d135_predictor_8x8 vpx_highbd_d135_predictor_8x8_neon

void vpx_highbd_d153_predictor_16x16_c(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
void vpx_highbd_d153_predictor_16x16_neon(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
#define vpx_highbd_d153_predictor_16x16 vpx_highbd_d153_predictor_16x16_neon

void vpx_highbd_d153_predictor_32x32_c(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
void vpx_highbd_d153_predictor_32x32_neon(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
#define vpx_highbd_d153_predictor_32x32 vpx_highbd_d153_predictor_32x32_neon

void vpx_highbd_d153_predictor_4x4_c(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
void vpx_highbd_d153_predictor_4x4_neon(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
#define vpx_highbd_d153_predictor_4x4 vpx_highbd_d153_predictor_4x4_neon

void vpx_highbd_d153_predictor_8x8_c(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
void vpx_highbd_d153_predictor_8x8_neon(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
#define vpx_highbd_d153_predictor_8x8 vpx_highbd_d153_predictor_8x8_neon

void vpx_highbd_d207_predictor_16x16_c(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
void vpx_highbd_d207_predictor_16x16_neon(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
#define vpx_highbd_d207_predictor_16x16 vpx_highbd_d207_predictor_16x16_neon

void vpx_highbd_d207_predictor_32x32_c(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
void vpx_highbd_d207_predictor_32x32_neon(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
#define vpx_highbd_d207_predictor_32x32 vpx_highbd_d207_predictor_32x32_neon

void vpx_highbd_d207_predictor_4x4_c(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
void vpx_highbd_d207_predictor_4x4_neon(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
#define vpx_highbd_d207_predictor_4x4 vpx_highbd_d207_predictor_4x4_neon

void vpx_highbd_d207_predictor_8x8_c(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
void vpx_highbd_d207_predictor_8x8_neon(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
#define vpx_highbd_d207_predictor_8x8 vpx_highbd_d207_predictor_8x8_neon

void vpx_highbd_d45_predictor_16x16_c(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
void vpx_highbd_d45_predictor_16x16_neon(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
#define vpx_highbd_d45_predictor_16x16 vpx_highbd_d45_predictor_16x16_neon

void vpx_highbd_d45_predictor_32x32_c(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
void vpx_highbd_d45_predictor_32x32_neon(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
#define vpx_highbd_d45_predictor_32x32 vpx_highbd_d45_predictor_32x32_neon

void vpx_highbd_d45_predictor_4x4_c(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
void vpx_highbd_d45_predictor_4x4_neon(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
#define vpx_highbd_d45_predictor_4x4 vpx_highbd_d45_predictor_4x4_neon

void vpx_highbd_d45_predictor_8x8_c(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
void vpx_highbd_d45_predictor_8x8_neon(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
#define vpx_highbd_d45_predictor_8x8 vpx_highbd_d45_predictor_8x8_neon

void vpx_highbd_d63_predictor_16x16_c(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
void vpx_highbd_d63_predictor_16x16_neon(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
#define vpx_highbd_d63_predictor_16x16 vpx_highbd_d63_predictor_16x16_neon

void vpx_highbd_d63_predictor_32x32_c(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
void vpx_highbd_d63_predictor_32x32_neon(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
#define vpx_highbd_d63_predictor_32x32 vpx_highbd_d63_predictor_32x32_neon

void vpx_highbd_d63_predictor_4x4_c(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
void vpx_highbd_d63_predictor_4x4_neon(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
#define vpx_highbd_d63_predictor_4x4 vpx_highbd_d63_predictor_4x4_neon

void vpx_highbd_d63_predictor_8x8_c(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
void vpx_highbd_d63_predictor_8x8_neon(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
#define vpx_highbd_d63_predictor_8x8 vpx_highbd_d63_predictor_8x8_neon

void vpx_highbd_dc_128_predictor_16x16_c(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
void vpx_highbd_dc_128_predictor_16x16_neon(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
#define vpx_highbd_dc_128_predictor_16x16 vpx_highbd_dc_128_predictor_16x16_neon

void vpx_highbd_dc_128_predictor_32x32_c(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
void vpx_highbd_dc_128_predictor_32x32_neon(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
#define vpx_highbd_dc_128_predictor_32x32 vpx_highbd_dc_128_predictor_32x32_neon

void vpx_highbd_dc_128_predictor_4x4_c(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
void vpx_highbd_dc_128_predictor_4x4_neon(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
#define vpx_highbd_dc_128_predictor_4x4 vpx_highbd_dc_128_predictor_4x4_neon

void vpx_highbd_dc_128_predictor_8x8_c(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
void vpx_highbd_dc_128_predictor_8x8_neon(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
#define vpx_highbd_dc_128_predictor_8x8 vpx_highbd_dc_128_predictor_8x8_neon

void vpx_highbd_dc_left_predictor_16x16_c(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
void vpx_highbd_dc_left_predictor_16x16_neon(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
#define vpx_highbd_dc_left_predictor_16x16 vpx_highbd_dc_left_predictor_16x16_neon

void vpx_highbd_dc_left_predictor_32x32_c(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
void vpx_highbd_dc_left_predictor_32x32_neon(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
#define vpx_highbd_dc_left_predictor_32x32 vpx_highbd_dc_left_predictor_32x32_neon

void vpx_highbd_dc_left_predictor_4x4_c(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
void vpx_highbd_dc_left_predictor_4x4_neon(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
#define vpx_highbd_dc_left_predictor_4x4 vpx_highbd_dc_left_predictor_4x4_neon

void vpx_highbd_dc_left_predictor_8x8_c(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
void vpx_highbd_dc_left_predictor_8x8_neon(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
#define vpx_highbd_dc_left_predictor_8x8 vpx_highbd_dc_left_predictor_8x8_neon

void vpx_highbd_dc_predictor_16x16_c(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
void vpx_highbd_dc_predictor_16x16_neon(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
#define vpx_highbd_dc_predictor_16x16 vpx_highbd_dc_predictor_16x16_neon

void vpx_highbd_dc_predictor_32x32_c(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
void vpx_highbd_dc_predictor_32x32_neon(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
#define vpx_highbd_dc_predictor_32x32 vpx_highbd_dc_predictor_32x32_neon

void vpx_highbd_dc_predictor_4x4_c(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
void vpx_highbd_dc_predictor_4x4_neon(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
#define vpx_highbd_dc_predictor_4x4 vpx_highbd_dc_predictor_4x4_neon

void vpx_highbd_dc_predictor_8x8_c(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
void vpx_highbd_dc_predictor_8x8_neon(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
#define vpx_highbd_dc_predictor_8x8 vpx_highbd_dc_predictor_8x8_neon

void vpx_highbd_dc_top_predictor_16x16_c(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
void vpx_highbd_dc_top_predictor_16x16_neon(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
#define vpx_highbd_dc_top_predictor_16x16 vpx_highbd_dc_top_predictor_16x16_neon

void vpx_highbd_dc_top_predictor_32x32_c(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
void vpx_highbd_dc_top_predictor_32x32_neon(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
#define vpx_highbd_dc_top_predictor_32x32 vpx_highbd_dc_top_predictor_32x32_neon

void vpx_highbd_dc_top_predictor_4x4_c(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
void vpx_highbd_dc_top_predictor_4x4_neon(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
#define vpx_highbd_dc_top_predictor_4x4 vpx_highbd_dc_top_predictor_4x4_neon

void vpx_highbd_dc_top_predictor_8x8_c(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
void vpx_highbd_dc_top_predictor_8x8_neon(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
#define vpx_highbd_dc_top_predictor_8x8 vpx_highbd_dc_top_predictor_8x8_neon

void vpx_highbd_fdct16x16_c(const int16_t *input, tran_low_t *output, int stride);
void vpx_highbd_fdct16x16_neon(const int16_t *input, tran_low_t *output, int stride);
#define vpx_highbd_fdct16x16 vpx_highbd_fdct16x16_neon

void vpx_highbd_fdct16x16_1_c(const int16_t *input, tran_low_t *output, int stride);
void vpx_highbd_fdct16x16_1_neon(const int16_t *input, tran_low_t *output, int stride);
#define vpx_highbd_fdct16x16_1 vpx_highbd_fdct16x16_1_neon

void vpx_highbd_fdct32x32_c(const int16_t *input, tran_low_t *output, int stride);
void vpx_highbd_fdct32x32_neon(const int16_t *input, tran_low_t *output, int stride);
#define vpx_highbd_fdct32x32 vpx_highbd_fdct32x32_neon

void vpx_highbd_fdct32x32_1_c(const int16_t *input, tran_low_t *output, int stride);
void vpx_highbd_fdct32x32_1_neon(const int16_t *input, tran_low_t *output, int stride);
#define vpx_highbd_fdct32x32_1 vpx_highbd_fdct32x32_1_neon

void vpx_highbd_fdct32x32_rd_c(const int16_t *input, tran_low_t *output, int stride);
void vpx_highbd_fdct32x32_rd_neon(const int16_t *input, tran_low_t *output, int stride);
#define vpx_highbd_fdct32x32_rd vpx_highbd_fdct32x32_rd_neon

void vpx_highbd_fdct4x4_c(const int16_t *input, tran_low_t *output, int stride);
void vpx_highbd_fdct4x4_neon(const int16_t *input, tran_low_t *output, int stride);
#define vpx_highbd_fdct4x4 vpx_highbd_fdct4x4_neon

void vpx_highbd_fdct8x8_c(const int16_t *input, tran_low_t *output, int stride);
void vpx_highbd_fdct8x8_neon(const int16_t *input, tran_low_t *output, int stride);
#define vpx_highbd_fdct8x8 vpx_highbd_fdct8x8_neon

void vpx_highbd_fdct8x8_1_c(const int16_t *input, tran_low_t *output, int stride);
void vpx_fdct8x8_1_neon(const int16_t *input, tran_low_t *output, int stride);
#define vpx_highbd_fdct8x8_1 vpx_fdct8x8_1_neon

void vpx_highbd_h_predictor_16x16_c(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
void vpx_highbd_h_predictor_16x16_neon(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
#define vpx_highbd_h_predictor_16x16 vpx_highbd_h_predictor_16x16_neon

void vpx_highbd_h_predictor_32x32_c(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
void vpx_highbd_h_predictor_32x32_neon(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
#define vpx_highbd_h_predictor_32x32 vpx_highbd_h_predictor_32x32_neon

void vpx_highbd_h_predictor_4x4_c(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
void vpx_highbd_h_predictor_4x4_neon(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
#define vpx_highbd_h_predictor_4x4 vpx_highbd_h_predictor_4x4_neon

void vpx_highbd_h_predictor_8x8_c(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
void vpx_highbd_h_predictor_8x8_neon(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
#define vpx_highbd_h_predictor_8x8 vpx_highbd_h_predictor_8x8_neon

void vpx_highbd_hadamard_16x16_c(const int16_t *src_diff, ptrdiff_t src_stride, tran_low_t *coeff);
void vpx_highbd_hadamard_16x16_neon(const int16_t *src_diff, ptrdiff_t src_stride, tran_low_t *coeff);
#define vpx_highbd_hadamard_16x16 vpx_highbd_hadamard_16x16_neon

void vpx_highbd_hadamard_32x32_c(const int16_t *src_diff, ptrdiff_t src_stride, tran_low_t *coeff);
void vpx_highbd_hadamard_32x32_neon(const int16_t *src_diff, ptrdiff_t src_stride, tran_low_t *coeff);
#define vpx_highbd_hadamard_32x32 vpx_highbd_hadamard_32x32_neon

void vpx_highbd_hadamard_8x8_c(const int16_t *src_diff, ptrdiff_t src_stride, tran_low_t *coeff);
void vpx_highbd_hadamard_8x8_neon(const int16_t *src_diff, ptrdiff_t src_stride, tran_low_t *coeff);
#define vpx_highbd_hadamard_8x8 vpx_highbd_hadamard_8x8_neon

void vpx_highbd_idct16x16_10_add_c(const tran_low_t *input, uint16_t *dest, int stride, int bd);
void vpx_highbd_idct16x16_10_add_neon(const tran_low_t *input, uint16_t *dest, int stride, int bd);
#define vpx_highbd_idct16x16_10_add vpx_highbd_idct16x16_10_add_neon

void vpx_highbd_idct16x16_1_add_c(const tran_low_t *input, uint16_t *dest, int stride, int bd);
void vpx_highbd_idct16x16_1_add_neon(const tran_low_t *input, uint16_t *dest, int stride, int bd);
#define vpx_highbd_idct16x16_1_add vpx_highbd_idct16x16_1_add_neon

void vpx_highbd_idct16x16_256_add_c(const tran_low_t *input, uint16_t *dest, int stride, int bd);
void vpx_highbd_idct16x16_256_add_neon(const tran_low_t *input, uint16_t *dest, int stride, int bd);
#define vpx_highbd_idct16x16_256_add vpx_highbd_idct16x16_256_add_neon

void vpx_highbd_idct16x16_38_add_c(const tran_low_t *input, uint16_t *dest, int stride, int bd);
void vpx_highbd_idct16x16_38_add_neon(const tran_low_t *input, uint16_t *dest, int stride, int bd);
#define vpx_highbd_idct16x16_38_add vpx_highbd_idct16x16_38_add_neon

void vpx_highbd_idct32x32_1024_add_c(const tran_low_t *input, uint16_t *dest, int stride, int bd);
void vpx_highbd_idct32x32_1024_add_neon(const tran_low_t *input, uint16_t *dest, int stride, int bd);
#define vpx_highbd_idct32x32_1024_add vpx_highbd_idct32x32_1024_add_neon

void vpx_highbd_idct32x32_135_add_c(const tran_low_t *input, uint16_t *dest, int stride, int bd);
void vpx_highbd_idct32x32_135_add_neon(const tran_low_t *input, uint16_t *dest, int stride, int bd);
#define vpx_highbd_idct32x32_135_add vpx_highbd_idct32x32_135_add_neon

void vpx_highbd_idct32x32_1_add_c(const tran_low_t *input, uint16_t *dest, int stride, int bd);
void vpx_highbd_idct32x32_1_add_neon(const tran_low_t *input, uint16_t *dest, int stride, int bd);
#define vpx_highbd_idct32x32_1_add vpx_highbd_idct32x32_1_add_neon

void vpx_highbd_idct32x32_34_add_c(const tran_low_t *input, uint16_t *dest, int stride, int bd);
void vpx_highbd_idct32x32_34_add_neon(const tran_low_t *input, uint16_t *dest, int stride, int bd);
#define vpx_highbd_idct32x32_34_add vpx_highbd_idct32x32_34_add_neon

void vpx_highbd_idct4x4_16_add_c(const tran_low_t *input, uint16_t *dest, int stride, int bd);
void vpx_highbd_idct4x4_16_add_neon(const tran_low_t *input, uint16_t *dest, int stride, int bd);
#define vpx_highbd_idct4x4_16_add vpx_highbd_idct4x4_16_add_neon

void vpx_highbd_idct4x4_1_add_c(const tran_low_t *input, uint16_t *dest, int stride, int bd);
void vpx_highbd_idct4x4_1_add_neon(const tran_low_t *input, uint16_t *dest, int stride, int bd);
#define vpx_highbd_idct4x4_1_add vpx_highbd_idct4x4_1_add_neon

void vpx_highbd_idct8x8_12_add_c(const tran_low_t *input, uint16_t *dest, int stride, int bd);
void vpx_highbd_idct8x8_12_add_neon(const tran_low_t *input, uint16_t *dest, int stride, int bd);
#define vpx_highbd_idct8x8_12_add vpx_highbd_idct8x8_12_add_neon

void vpx_highbd_idct8x8_1_add_c(const tran_low_t *input, uint16_t *dest, int stride, int bd);
void vpx_highbd_idct8x8_1_add_neon(const tran_low_t *input, uint16_t *dest, int stride, int bd);
#define vpx_highbd_idct8x8_1_add vpx_highbd_idct8x8_1_add_neon

void vpx_highbd_idct8x8_64_add_c(const tran_low_t *input, uint16_t *dest, int stride, int bd);
void vpx_highbd_idct8x8_64_add_neon(const tran_low_t *input, uint16_t *dest, int stride, int bd);
#define vpx_highbd_idct8x8_64_add vpx_highbd_idct8x8_64_add_neon

void vpx_highbd_iwht4x4_16_add_c(const tran_low_t *input, uint16_t *dest, int stride, int bd);
#define vpx_highbd_iwht4x4_16_add vpx_highbd_iwht4x4_16_add_c

void vpx_highbd_iwht4x4_1_add_c(const tran_low_t *input, uint16_t *dest, int stride, int bd);
#define vpx_highbd_iwht4x4_1_add vpx_highbd_iwht4x4_1_add_c

void vpx_highbd_lpf_horizontal_16_c(uint16_t *s, int pitch, const uint8_t *blimit, const uint8_t *limit, const uint8_t *thresh, int bd);
void vpx_highbd_lpf_horizontal_16_neon(uint16_t *s, int pitch, const uint8_t *blimit, const uint8_t *limit, const uint8_t *thresh, int bd);
#define vpx_highbd_lpf_horizontal_16 vpx_highbd_lpf_horizontal_16_neon

void vpx_highbd_lpf_horizontal_16_dual_c(uint16_t *s, int pitch, const uint8_t *blimit, const uint8_t *limit, const uint8_t *thresh, int bd);
void vpx_highbd_lpf_horizontal_16_dual_neon(uint16_t *s, int pitch, const uint8_t *blimit, const uint8_t *limit, const uint8_t *thresh, int bd);
#define vpx_highbd_lpf_horizontal_16_dual vpx_highbd_lpf_horizontal_16_dual_neon

void vpx_highbd_lpf_horizontal_4_c(uint16_t *s, int pitch, const uint8_t *blimit, const uint8_t *limit, const uint8_t *thresh, int bd);
void vpx_highbd_lpf_horizontal_4_neon(uint16_t *s, int pitch, const uint8_t *blimit, const uint8_t *limit, const uint8_t *thresh, int bd);
#define vpx_highbd_lpf_horizontal_4 vpx_highbd_lpf_horizontal_4_neon

void vpx_highbd_lpf_horizontal_4_dual_c(uint16_t *s, int pitch, const uint8_t *blimit0, const uint8_t *limit0, const uint8_t *thresh0, const uint8_t *blimit1, const uint8_t *limit1, const uint8_t *thresh1, int bd);
void vpx_highbd_lpf_horizontal_4_dual_neon(uint16_t *s, int pitch, const uint8_t *blimit0, const uint8_t *limit0, const uint8_t *thresh0, const uint8_t *blimit1, const uint8_t *limit1, const uint8_t *thresh1, int bd);
#define vpx_highbd_lpf_horizontal_4_dual vpx_highbd_lpf_horizontal_4_dual_neon

void vpx_highbd_lpf_horizontal_8_c(uint16_t *s, int pitch, const uint8_t *blimit, const uint8_t *limit, const uint8_t *thresh, int bd);
void vpx_highbd_lpf_horizontal_8_neon(uint16_t *s, int pitch, const uint8_t *blimit, const uint8_t *limit, const uint8_t *thresh, int bd);
#define vpx_highbd_lpf_horizontal_8 vpx_highbd_lpf_horizontal_8_neon

void vpx_highbd_lpf_horizontal_8_dual_c(uint16_t *s, int pitch, const uint8_t *blimit0, const uint8_t *limit0, const uint8_t *thresh0, const uint8_t *blimit1, const uint8_t *limit1, const uint8_t *thresh1, int bd);
void vpx_highbd_lpf_horizontal_8_dual_neon(uint16_t *s, int pitch, const uint8_t *blimit0, const uint8_t *limit0, const uint8_t *thresh0, const uint8_t *blimit1, const uint8_t *limit1, const uint8_t *thresh1, int bd);
#define vpx_highbd_lpf_horizontal_8_dual vpx_highbd_lpf_horizontal_8_dual_neon

void vpx_highbd_lpf_vertical_16_c(uint16_t *s, int pitch, const uint8_t *blimit, const uint8_t *limit, const uint8_t *thresh, int bd);
void vpx_highbd_lpf_vertical_16_neon(uint16_t *s, int pitch, const uint8_t *blimit, const uint8_t *limit, const uint8_t *thresh, int bd);
#define vpx_highbd_lpf_vertical_16 vpx_highbd_lpf_vertical_16_neon

void vpx_highbd_lpf_vertical_16_dual_c(uint16_t *s, int pitch, const uint8_t *blimit, const uint8_t *limit, const uint8_t *thresh, int bd);
void vpx_highbd_lpf_vertical_16_dual_neon(uint16_t *s, int pitch, const uint8_t *blimit, const uint8_t *limit, const uint8_t *thresh, int bd);
#define vpx_highbd_lpf_vertical_16_dual vpx_highbd_lpf_vertical_16_dual_neon

void vpx_highbd_lpf_vertical_4_c(uint16_t *s, int pitch, const uint8_t *blimit, const uint8_t *limit, const uint8_t *thresh, int bd);
void vpx_highbd_lpf_vertical_4_neon(uint16_t *s, int pitch, const uint8_t *blimit, const uint8_t *limit, const uint8_t *thresh, int bd);
#define vpx_highbd_lpf_vertical_4 vpx_highbd_lpf_vertical_4_neon

void vpx_highbd_lpf_vertical_4_dual_c(uint16_t *s, int pitch, const uint8_t *blimit0, const uint8_t *limit0, const uint8_t *thresh0, const uint8_t *blimit1, const uint8_t *limit1, const uint8_t *thresh1, int bd);
void vpx_highbd_lpf_vertical_4_dual_neon(uint16_t *s, int pitch, const uint8_t *blimit0, const uint8_t *limit0, const uint8_t *thresh0, const uint8_t *blimit1, const uint8_t *limit1, const uint8_t *thresh1, int bd);
#define vpx_highbd_lpf_vertical_4_dual vpx_highbd_lpf_vertical_4_dual_neon

void vpx_highbd_lpf_vertical_8_c(uint16_t *s, int pitch, const uint8_t *blimit, const uint8_t *limit, const uint8_t *thresh, int bd);
void vpx_highbd_lpf_vertical_8_neon(uint16_t *s, int pitch, const uint8_t *blimit, const uint8_t *limit, const uint8_t *thresh, int bd);
#define vpx_highbd_lpf_vertical_8 vpx_highbd_lpf_vertical_8_neon

void vpx_highbd_lpf_vertical_8_dual_c(uint16_t *s, int pitch, const uint8_t *blimit0, const uint8_t *limit0, const uint8_t *thresh0, const uint8_t *blimit1, const uint8_t *limit1, const uint8_t *thresh1, int bd);
void vpx_highbd_lpf_vertical_8_dual_neon(uint16_t *s, int pitch, const uint8_t *blimit0, const uint8_t *limit0, const uint8_t *thresh0, const uint8_t *blimit1, const uint8_t *limit1, const uint8_t *thresh1, int bd);
#define vpx_highbd_lpf_vertical_8_dual vpx_highbd_lpf_vertical_8_dual_neon

void vpx_highbd_minmax_8x8_c(const uint8_t *s8, int p, const uint8_t *d8, int dp, int *min, int *max);
void vpx_highbd_minmax_8x8_neon(const uint8_t *s8, int p, const uint8_t *d8, int dp, int *min, int *max);
#define vpx_highbd_minmax_8x8 vpx_highbd_minmax_8x8_neon

void vpx_highbd_quantize_b_c(const tran_low_t *coeff_ptr, intptr_t n_coeffs, const struct macroblock_plane *const mb_plane, tran_low_t *qcoeff_ptr, tran_low_t *dqcoeff_ptr, const int16_t *dequant_ptr, uint16_t *eob_ptr, const struct ScanOrder *const scan_order);
void vpx_highbd_quantize_b_neon(const tran_low_t *coeff_ptr, intptr_t n_coeffs, const struct macroblock_plane *const mb_plane, tran_low_t *qcoeff_ptr, tran_low_t *dqcoeff_ptr, const int16_t *dequant_ptr, uint16_t *eob_ptr, const struct ScanOrder *const scan_order);
#define vpx_highbd_quantize_b vpx_highbd_quantize_b_neon

void vpx_highbd_quantize_b_32x32_c(const tran_low_t *coeff_ptr, const struct macroblock_plane *const mb_plane, tran_low_t *qcoeff_ptr, tran_low_t *dqcoeff_ptr, const int16_t *dequant_ptr, uint16_t *eob_ptr, const struct ScanOrder *const scan_order);
void vpx_highbd_quantize_b_32x32_neon(const tran_low_t *coeff_ptr, const struct macroblock_plane *const mb_plane, tran_low_t *qcoeff_ptr, tran_low_t *dqcoeff_ptr, const int16_t *dequant_ptr, uint16_t *eob_ptr, const struct ScanOrder *const scan_order);
#define vpx_highbd_quantize_b_32x32 vpx_highbd_quantize_b_32x32_neon

unsigned int vpx_highbd_sad16x16_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
unsigned int vpx_highbd_sad16x16_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
#define vpx_highbd_sad16x16 vpx_highbd_sad16x16_neon

unsigned int vpx_highbd_sad16x16_avg_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, const uint8_t *second_pred);
unsigned int vpx_highbd_sad16x16_avg_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, const uint8_t *second_pred);
#define vpx_highbd_sad16x16_avg vpx_highbd_sad16x16_avg_neon

void vpx_highbd_sad16x16x4d_c(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
void vpx_highbd_sad16x16x4d_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
#define vpx_highbd_sad16x16x4d vpx_highbd_sad16x16x4d_neon

unsigned int vpx_highbd_sad16x32_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
unsigned int vpx_highbd_sad16x32_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
#define vpx_highbd_sad16x32 vpx_highbd_sad16x32_neon

unsigned int vpx_highbd_sad16x32_avg_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, const uint8_t *second_pred);
unsigned int vpx_highbd_sad16x32_avg_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, const uint8_t *second_pred);
#define vpx_highbd_sad16x32_avg vpx_highbd_sad16x32_avg_neon

void vpx_highbd_sad16x32x4d_c(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
void vpx_highbd_sad16x32x4d_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
#define vpx_highbd_sad16x32x4d vpx_highbd_sad16x32x4d_neon

unsigned int vpx_highbd_sad16x8_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
unsigned int vpx_highbd_sad16x8_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
#define vpx_highbd_sad16x8 vpx_highbd_sad16x8_neon

unsigned int vpx_highbd_sad16x8_avg_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, const uint8_t *second_pred);
unsigned int vpx_highbd_sad16x8_avg_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, const uint8_t *second_pred);
#define vpx_highbd_sad16x8_avg vpx_highbd_sad16x8_avg_neon

void vpx_highbd_sad16x8x4d_c(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
void vpx_highbd_sad16x8x4d_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
#define vpx_highbd_sad16x8x4d vpx_highbd_sad16x8x4d_neon

unsigned int vpx_highbd_sad32x16_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
unsigned int vpx_highbd_sad32x16_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
#define vpx_highbd_sad32x16 vpx_highbd_sad32x16_neon

unsigned int vpx_highbd_sad32x16_avg_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, const uint8_t *second_pred);
unsigned int vpx_highbd_sad32x16_avg_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, const uint8_t *second_pred);
#define vpx_highbd_sad32x16_avg vpx_highbd_sad32x16_avg_neon

void vpx_highbd_sad32x16x4d_c(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
void vpx_highbd_sad32x16x4d_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
#define vpx_highbd_sad32x16x4d vpx_highbd_sad32x16x4d_neon

unsigned int vpx_highbd_sad32x32_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
unsigned int vpx_highbd_sad32x32_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
#define vpx_highbd_sad32x32 vpx_highbd_sad32x32_neon

unsigned int vpx_highbd_sad32x32_avg_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, const uint8_t *second_pred);
unsigned int vpx_highbd_sad32x32_avg_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, const uint8_t *second_pred);
#define vpx_highbd_sad32x32_avg vpx_highbd_sad32x32_avg_neon

void vpx_highbd_sad32x32x4d_c(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
void vpx_highbd_sad32x32x4d_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
#define vpx_highbd_sad32x32x4d vpx_highbd_sad32x32x4d_neon

unsigned int vpx_highbd_sad32x64_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
unsigned int vpx_highbd_sad32x64_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
#define vpx_highbd_sad32x64 vpx_highbd_sad32x64_neon

unsigned int vpx_highbd_sad32x64_avg_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, const uint8_t *second_pred);
unsigned int vpx_highbd_sad32x64_avg_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, const uint8_t *second_pred);
#define vpx_highbd_sad32x64_avg vpx_highbd_sad32x64_avg_neon

void vpx_highbd_sad32x64x4d_c(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
void vpx_highbd_sad32x64x4d_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
#define vpx_highbd_sad32x64x4d vpx_highbd_sad32x64x4d_neon

unsigned int vpx_highbd_sad4x4_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
unsigned int vpx_highbd_sad4x4_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
#define vpx_highbd_sad4x4 vpx_highbd_sad4x4_neon

unsigned int vpx_highbd_sad4x4_avg_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, const uint8_t *second_pred);
unsigned int vpx_highbd_sad4x4_avg_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, const uint8_t *second_pred);
#define vpx_highbd_sad4x4_avg vpx_highbd_sad4x4_avg_neon

void vpx_highbd_sad4x4x4d_c(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
void vpx_highbd_sad4x4x4d_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
#define vpx_highbd_sad4x4x4d vpx_highbd_sad4x4x4d_neon

unsigned int vpx_highbd_sad4x8_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
unsigned int vpx_highbd_sad4x8_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
#define vpx_highbd_sad4x8 vpx_highbd_sad4x8_neon

unsigned int vpx_highbd_sad4x8_avg_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, const uint8_t *second_pred);
unsigned int vpx_highbd_sad4x8_avg_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, const uint8_t *second_pred);
#define vpx_highbd_sad4x8_avg vpx_highbd_sad4x8_avg_neon

void vpx_highbd_sad4x8x4d_c(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
void vpx_highbd_sad4x8x4d_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
#define vpx_highbd_sad4x8x4d vpx_highbd_sad4x8x4d_neon

unsigned int vpx_highbd_sad64x32_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
unsigned int vpx_highbd_sad64x32_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
#define vpx_highbd_sad64x32 vpx_highbd_sad64x32_neon

unsigned int vpx_highbd_sad64x32_avg_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, const uint8_t *second_pred);
unsigned int vpx_highbd_sad64x32_avg_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, const uint8_t *second_pred);
#define vpx_highbd_sad64x32_avg vpx_highbd_sad64x32_avg_neon

void vpx_highbd_sad64x32x4d_c(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
void vpx_highbd_sad64x32x4d_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
#define vpx_highbd_sad64x32x4d vpx_highbd_sad64x32x4d_neon

unsigned int vpx_highbd_sad64x64_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
unsigned int vpx_highbd_sad64x64_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
#define vpx_highbd_sad64x64 vpx_highbd_sad64x64_neon

unsigned int vpx_highbd_sad64x64_avg_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, const uint8_t *second_pred);
unsigned int vpx_highbd_sad64x64_avg_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, const uint8_t *second_pred);
#define vpx_highbd_sad64x64_avg vpx_highbd_sad64x64_avg_neon

void vpx_highbd_sad64x64x4d_c(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
void vpx_highbd_sad64x64x4d_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
#define vpx_highbd_sad64x64x4d vpx_highbd_sad64x64x4d_neon

unsigned int vpx_highbd_sad8x16_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
unsigned int vpx_highbd_sad8x16_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
#define vpx_highbd_sad8x16 vpx_highbd_sad8x16_neon

unsigned int vpx_highbd_sad8x16_avg_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, const uint8_t *second_pred);
unsigned int vpx_highbd_sad8x16_avg_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, const uint8_t *second_pred);
#define vpx_highbd_sad8x16_avg vpx_highbd_sad8x16_avg_neon

void vpx_highbd_sad8x16x4d_c(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
void vpx_highbd_sad8x16x4d_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
#define vpx_highbd_sad8x16x4d vpx_highbd_sad8x16x4d_neon

unsigned int vpx_highbd_sad8x4_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
unsigned int vpx_highbd_sad8x4_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
#define vpx_highbd_sad8x4 vpx_highbd_sad8x4_neon

unsigned int vpx_highbd_sad8x4_avg_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, const uint8_t *second_pred);
unsigned int vpx_highbd_sad8x4_avg_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, const uint8_t *second_pred);
#define vpx_highbd_sad8x4_avg vpx_highbd_sad8x4_avg_neon

void vpx_highbd_sad8x4x4d_c(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
void vpx_highbd_sad8x4x4d_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
#define vpx_highbd_sad8x4x4d vpx_highbd_sad8x4x4d_neon

unsigned int vpx_highbd_sad8x8_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
unsigned int vpx_highbd_sad8x8_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
#define vpx_highbd_sad8x8 vpx_highbd_sad8x8_neon

unsigned int vpx_highbd_sad8x8_avg_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, const uint8_t *second_pred);
unsigned int vpx_highbd_sad8x8_avg_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, const uint8_t *second_pred);
#define vpx_highbd_sad8x8_avg vpx_highbd_sad8x8_avg_neon

void vpx_highbd_sad8x8x4d_c(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
void vpx_highbd_sad8x8x4d_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
#define vpx_highbd_sad8x8x4d vpx_highbd_sad8x8x4d_neon

unsigned int vpx_highbd_sad_skip_16x16_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
unsigned int vpx_highbd_sad_skip_16x16_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
#define vpx_highbd_sad_skip_16x16 vpx_highbd_sad_skip_16x16_neon

void vpx_highbd_sad_skip_16x16x4d_c(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
void vpx_highbd_sad_skip_16x16x4d_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
#define vpx_highbd_sad_skip_16x16x4d vpx_highbd_sad_skip_16x16x4d_neon

unsigned int vpx_highbd_sad_skip_16x32_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
unsigned int vpx_highbd_sad_skip_16x32_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
#define vpx_highbd_sad_skip_16x32 vpx_highbd_sad_skip_16x32_neon

void vpx_highbd_sad_skip_16x32x4d_c(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
void vpx_highbd_sad_skip_16x32x4d_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
#define vpx_highbd_sad_skip_16x32x4d vpx_highbd_sad_skip_16x32x4d_neon

unsigned int vpx_highbd_sad_skip_16x8_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
unsigned int vpx_highbd_sad_skip_16x8_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
#define vpx_highbd_sad_skip_16x8 vpx_highbd_sad_skip_16x8_neon

void vpx_highbd_sad_skip_16x8x4d_c(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
void vpx_highbd_sad_skip_16x8x4d_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
#define vpx_highbd_sad_skip_16x8x4d vpx_highbd_sad_skip_16x8x4d_neon

unsigned int vpx_highbd_sad_skip_32x16_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
unsigned int vpx_highbd_sad_skip_32x16_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
#define vpx_highbd_sad_skip_32x16 vpx_highbd_sad_skip_32x16_neon

void vpx_highbd_sad_skip_32x16x4d_c(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
void vpx_highbd_sad_skip_32x16x4d_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
#define vpx_highbd_sad_skip_32x16x4d vpx_highbd_sad_skip_32x16x4d_neon

unsigned int vpx_highbd_sad_skip_32x32_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
unsigned int vpx_highbd_sad_skip_32x32_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
#define vpx_highbd_sad_skip_32x32 vpx_highbd_sad_skip_32x32_neon

void vpx_highbd_sad_skip_32x32x4d_c(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
void vpx_highbd_sad_skip_32x32x4d_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
#define vpx_highbd_sad_skip_32x32x4d vpx_highbd_sad_skip_32x32x4d_neon

unsigned int vpx_highbd_sad_skip_32x64_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
unsigned int vpx_highbd_sad_skip_32x64_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
#define vpx_highbd_sad_skip_32x64 vpx_highbd_sad_skip_32x64_neon

void vpx_highbd_sad_skip_32x64x4d_c(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
void vpx_highbd_sad_skip_32x64x4d_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
#define vpx_highbd_sad_skip_32x64x4d vpx_highbd_sad_skip_32x64x4d_neon

unsigned int vpx_highbd_sad_skip_4x4_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
unsigned int vpx_highbd_sad_skip_4x4_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
#define vpx_highbd_sad_skip_4x4 vpx_highbd_sad_skip_4x4_neon

void vpx_highbd_sad_skip_4x4x4d_c(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
void vpx_highbd_sad_skip_4x4x4d_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
#define vpx_highbd_sad_skip_4x4x4d vpx_highbd_sad_skip_4x4x4d_neon

unsigned int vpx_highbd_sad_skip_4x8_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
unsigned int vpx_highbd_sad_skip_4x8_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
#define vpx_highbd_sad_skip_4x8 vpx_highbd_sad_skip_4x8_neon

void vpx_highbd_sad_skip_4x8x4d_c(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
void vpx_highbd_sad_skip_4x8x4d_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
#define vpx_highbd_sad_skip_4x8x4d vpx_highbd_sad_skip_4x8x4d_neon

unsigned int vpx_highbd_sad_skip_64x32_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
unsigned int vpx_highbd_sad_skip_64x32_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
#define vpx_highbd_sad_skip_64x32 vpx_highbd_sad_skip_64x32_neon

void vpx_highbd_sad_skip_64x32x4d_c(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
void vpx_highbd_sad_skip_64x32x4d_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
#define vpx_highbd_sad_skip_64x32x4d vpx_highbd_sad_skip_64x32x4d_neon

unsigned int vpx_highbd_sad_skip_64x64_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
unsigned int vpx_highbd_sad_skip_64x64_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
#define vpx_highbd_sad_skip_64x64 vpx_highbd_sad_skip_64x64_neon

void vpx_highbd_sad_skip_64x64x4d_c(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
void vpx_highbd_sad_skip_64x64x4d_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
#define vpx_highbd_sad_skip_64x64x4d vpx_highbd_sad_skip_64x64x4d_neon

unsigned int vpx_highbd_sad_skip_8x16_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
unsigned int vpx_highbd_sad_skip_8x16_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
#define vpx_highbd_sad_skip_8x16 vpx_highbd_sad_skip_8x16_neon

void vpx_highbd_sad_skip_8x16x4d_c(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
void vpx_highbd_sad_skip_8x16x4d_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
#define vpx_highbd_sad_skip_8x16x4d vpx_highbd_sad_skip_8x16x4d_neon

unsigned int vpx_highbd_sad_skip_8x4_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
unsigned int vpx_highbd_sad_skip_8x4_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
#define vpx_highbd_sad_skip_8x4 vpx_highbd_sad_skip_8x4_neon

void vpx_highbd_sad_skip_8x4x4d_c(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
void vpx_highbd_sad_skip_8x4x4d_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
#define vpx_highbd_sad_skip_8x4x4d vpx_highbd_sad_skip_8x4x4d_neon

unsigned int vpx_highbd_sad_skip_8x8_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
unsigned int vpx_highbd_sad_skip_8x8_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
#define vpx_highbd_sad_skip_8x8 vpx_highbd_sad_skip_8x8_neon

void vpx_highbd_sad_skip_8x8x4d_c(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
void vpx_highbd_sad_skip_8x8x4d_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
#define vpx_highbd_sad_skip_8x8x4d vpx_highbd_sad_skip_8x8x4d_neon

int vpx_highbd_satd_c(const tran_low_t *coeff, int length);
int vpx_highbd_satd_neon(const tran_low_t *coeff, int length);
#define vpx_highbd_satd vpx_highbd_satd_neon

int64_t vpx_highbd_sse_c(const uint8_t *a8, int a_stride, const uint8_t *b8,int b_stride, int width, int height);
int64_t vpx_highbd_sse_neon(const uint8_t *a8, int a_stride, const uint8_t *b8,int b_stride, int width, int height);
#define vpx_highbd_sse vpx_highbd_sse_neon

void vpx_highbd_subtract_block_c(int rows, int cols, int16_t *diff_ptr, ptrdiff_t diff_stride, const uint8_t *src8_ptr, ptrdiff_t src_stride, const uint8_t *pred8_ptr, ptrdiff_t pred_stride, int bd);
void vpx_highbd_subtract_block_neon(int rows, int cols, int16_t *diff_ptr, ptrdiff_t diff_stride, const uint8_t *src8_ptr, ptrdiff_t src_stride, const uint8_t *pred8_ptr, ptrdiff_t pred_stride, int bd);
#define vpx_highbd_subtract_block vpx_highbd_subtract_block_neon

void vpx_highbd_tm_predictor_16x16_c(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
void vpx_highbd_tm_predictor_16x16_neon(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
#define vpx_highbd_tm_predictor_16x16 vpx_highbd_tm_predictor_16x16_neon

void vpx_highbd_tm_predictor_32x32_c(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
void vpx_highbd_tm_predictor_32x32_neon(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
#define vpx_highbd_tm_predictor_32x32 vpx_highbd_tm_predictor_32x32_neon

void vpx_highbd_tm_predictor_4x4_c(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
void vpx_highbd_tm_predictor_4x4_neon(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
#define vpx_highbd_tm_predictor_4x4 vpx_highbd_tm_predictor_4x4_neon

void vpx_highbd_tm_predictor_8x8_c(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
void vpx_highbd_tm_predictor_8x8_neon(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
#define vpx_highbd_tm_predictor_8x8 vpx_highbd_tm_predictor_8x8_neon

void vpx_highbd_v_predictor_16x16_c(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
void vpx_highbd_v_predictor_16x16_neon(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
#define vpx_highbd_v_predictor_16x16 vpx_highbd_v_predictor_16x16_neon

void vpx_highbd_v_predictor_32x32_c(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
void vpx_highbd_v_predictor_32x32_neon(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
#define vpx_highbd_v_predictor_32x32 vpx_highbd_v_predictor_32x32_neon

void vpx_highbd_v_predictor_4x4_c(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
void vpx_highbd_v_predictor_4x4_neon(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
#define vpx_highbd_v_predictor_4x4 vpx_highbd_v_predictor_4x4_neon

void vpx_highbd_v_predictor_8x8_c(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
void vpx_highbd_v_predictor_8x8_neon(uint16_t *dst, ptrdiff_t stride, const uint16_t *above, const uint16_t *left, int bd);
#define vpx_highbd_v_predictor_8x8 vpx_highbd_v_predictor_8x8_neon

void vpx_idct16x16_10_add_c(const tran_low_t *input, uint8_t *dest, int stride);
void vpx_idct16x16_10_add_neon(const tran_low_t *input, uint8_t *dest, int stride);
#define vpx_idct16x16_10_add vpx_idct16x16_10_add_neon

void vpx_idct16x16_1_add_c(const tran_low_t *input, uint8_t *dest, int stride);
void vpx_idct16x16_1_add_neon(const tran_low_t *input, uint8_t *dest, int stride);
#define vpx_idct16x16_1_add vpx_idct16x16_1_add_neon

void vpx_idct16x16_256_add_c(const tran_low_t *input, uint8_t *dest, int stride);
void vpx_idct16x16_256_add_neon(const tran_low_t *input, uint8_t *dest, int stride);
#define vpx_idct16x16_256_add vpx_idct16x16_256_add_neon

void vpx_idct16x16_38_add_c(const tran_low_t *input, uint8_t *dest, int stride);
void vpx_idct16x16_38_add_neon(const tran_low_t *input, uint8_t *dest, int stride);
#define vpx_idct16x16_38_add vpx_idct16x16_38_add_neon

void vpx_idct32x32_1024_add_c(const tran_low_t *input, uint8_t *dest, int stride);
void vpx_idct32x32_1024_add_neon(const tran_low_t *input, uint8_t *dest, int stride);
#define vpx_idct32x32_1024_add vpx_idct32x32_1024_add_neon

void vpx_idct32x32_135_add_c(const tran_low_t *input, uint8_t *dest, int stride);
void vpx_idct32x32_135_add_neon(const tran_low_t *input, uint8_t *dest, int stride);
#define vpx_idct32x32_135_add vpx_idct32x32_135_add_neon

void vpx_idct32x32_1_add_c(const tran_low_t *input, uint8_t *dest, int stride);
void vpx_idct32x32_1_add_neon(const tran_low_t *input, uint8_t *dest, int stride);
#define vpx_idct32x32_1_add vpx_idct32x32_1_add_neon

void vpx_idct32x32_34_add_c(const tran_low_t *input, uint8_t *dest, int stride);
void vpx_idct32x32_34_add_neon(const tran_low_t *input, uint8_t *dest, int stride);
#define vpx_idct32x32_34_add vpx_idct32x32_34_add_neon

void vpx_idct4x4_16_add_c(const tran_low_t *input, uint8_t *dest, int stride);
void vpx_idct4x4_16_add_neon(const tran_low_t *input, uint8_t *dest, int stride);
#define vpx_idct4x4_16_add vpx_idct4x4_16_add_neon

void vpx_idct4x4_1_add_c(const tran_low_t *input, uint8_t *dest, int stride);
void vpx_idct4x4_1_add_neon(const tran_low_t *input, uint8_t *dest, int stride);
#define vpx_idct4x4_1_add vpx_idct4x4_1_add_neon

void vpx_idct8x8_12_add_c(const tran_low_t *input, uint8_t *dest, int stride);
void vpx_idct8x8_12_add_neon(const tran_low_t *input, uint8_t *dest, int stride);
#define vpx_idct8x8_12_add vpx_idct8x8_12_add_neon

void vpx_idct8x8_1_add_c(const tran_low_t *input, uint8_t *dest, int stride);
void vpx_idct8x8_1_add_neon(const tran_low_t *input, uint8_t *dest, int stride);
#define vpx_idct8x8_1_add vpx_idct8x8_1_add_neon

void vpx_idct8x8_64_add_c(const tran_low_t *input, uint8_t *dest, int stride);
void vpx_idct8x8_64_add_neon(const tran_low_t *input, uint8_t *dest, int stride);
#define vpx_idct8x8_64_add vpx_idct8x8_64_add_neon

int16_t vpx_int_pro_col_c(const uint8_t *ref, const int width);
int16_t vpx_int_pro_col_neon(const uint8_t *ref, const int width);
#define vpx_int_pro_col vpx_int_pro_col_neon

void vpx_int_pro_row_c(int16_t hbuf[16], const uint8_t *ref, const int ref_stride, const int height);
void vpx_int_pro_row_neon(int16_t hbuf[16], const uint8_t *ref, const int ref_stride, const int height);
#define vpx_int_pro_row vpx_int_pro_row_neon

void vpx_iwht4x4_16_add_c(const tran_low_t *input, uint8_t *dest, int stride);
#define vpx_iwht4x4_16_add vpx_iwht4x4_16_add_c

void vpx_iwht4x4_1_add_c(const tran_low_t *input, uint8_t *dest, int stride);
#define vpx_iwht4x4_1_add vpx_iwht4x4_1_add_c

void vpx_lpf_horizontal_16_c(uint8_t *s, int pitch, const uint8_t *blimit, const uint8_t *limit, const uint8_t *thresh);
void vpx_lpf_horizontal_16_neon(uint8_t *s, int pitch, const uint8_t *blimit, const uint8_t *limit, const uint8_t *thresh);
#define vpx_lpf_horizontal_16 vpx_lpf_horizontal_16_neon

void vpx_lpf_horizontal_16_dual_c(uint8_t *s, int pitch, const uint8_t *blimit, const uint8_t *limit, const uint8_t *thresh);
void vpx_lpf_horizontal_16_dual_neon(uint8_t *s, int pitch, const uint8_t *blimit, const uint8_t *limit, const uint8_t *thresh);
#define vpx_lpf_horizontal_16_dual vpx_lpf_horizontal_16_dual_neon

void vpx_lpf_horizontal_4_c(uint8_t *s, int pitch, const uint8_t *blimit, const uint8_t *limit, const uint8_t *thresh);
void vpx_lpf_horizontal_4_neon(uint8_t *s, int pitch, const uint8_t *blimit, const uint8_t *limit, const uint8_t *thresh);
#define vpx_lpf_horizontal_4 vpx_lpf_horizontal_4_neon

void vpx_lpf_horizontal_4_dual_c(uint8_t *s, int pitch, const uint8_t *blimit0, const uint8_t *limit0, const uint8_t *thresh0, const uint8_t *blimit1, const uint8_t *limit1, const uint8_t *thresh1);
void vpx_lpf_horizontal_4_dual_neon(uint8_t *s, int pitch, const uint8_t *blimit0, const uint8_t *limit0, const uint8_t *thresh0, const uint8_t *blimit1, const uint8_t *limit1, const uint8_t *thresh1);
#define vpx_lpf_horizontal_4_dual vpx_lpf_horizontal_4_dual_neon

void vpx_lpf_horizontal_8_c(uint8_t *s, int pitch, const uint8_t *blimit, const uint8_t *limit, const uint8_t *thresh);
void vpx_lpf_horizontal_8_neon(uint8_t *s, int pitch, const uint8_t *blimit, const uint8_t *limit, const uint8_t *thresh);
#define vpx_lpf_horizontal_8 vpx_lpf_horizontal_8_neon

void vpx_lpf_horizontal_8_dual_c(uint8_t *s, int pitch, const uint8_t *blimit0, const uint8_t *limit0, const uint8_t *thresh0, const uint8_t *blimit1, const uint8_t *limit1, const uint8_t *thresh1);
void vpx_lpf_horizontal_8_dual_neon(uint8_t *s, int pitch, const uint8_t *blimit0, const uint8_t *limit0, const uint8_t *thresh0, const uint8_t *blimit1, const uint8_t *limit1, const uint8_t *thresh1);
#define vpx_lpf_horizontal_8_dual vpx_lpf_horizontal_8_dual_neon

void vpx_lpf_vertical_16_c(uint8_t *s, int pitch, const uint8_t *blimit, const uint8_t *limit, const uint8_t *thresh);
void vpx_lpf_vertical_16_neon(uint8_t *s, int pitch, const uint8_t *blimit, const uint8_t *limit, const uint8_t *thresh);
#define vpx_lpf_vertical_16 vpx_lpf_vertical_16_neon

void vpx_lpf_vertical_16_dual_c(uint8_t *s, int pitch, const uint8_t *blimit, const uint8_t *limit, const uint8_t *thresh);
void vpx_lpf_vertical_16_dual_neon(uint8_t *s, int pitch, const uint8_t *blimit, const uint8_t *limit, const uint8_t *thresh);
#define vpx_lpf_vertical_16_dual vpx_lpf_vertical_16_dual_neon

void vpx_lpf_vertical_4_c(uint8_t *s, int pitch, const uint8_t *blimit, const uint8_t *limit, const uint8_t *thresh);
void vpx_lpf_vertical_4_neon(uint8_t *s, int pitch, const uint8_t *blimit, const uint8_t *limit, const uint8_t *thresh);
#define vpx_lpf_vertical_4 vpx_lpf_vertical_4_neon

void vpx_lpf_vertical_4_dual_c(uint8_t *s, int pitch, const uint8_t *blimit0, const uint8_t *limit0, const uint8_t *thresh0, const uint8_t *blimit1, const uint8_t *limit1, const uint8_t *thresh1);
void vpx_lpf_vertical_4_dual_neon(uint8_t *s, int pitch, const uint8_t *blimit0, const uint8_t *limit0, const uint8_t *thresh0, const uint8_t *blimit1, const uint8_t *limit1, const uint8_t *thresh1);
#define vpx_lpf_vertical_4_dual vpx_lpf_vertical_4_dual_neon

void vpx_lpf_vertical_8_c(uint8_t *s, int pitch, const uint8_t *blimit, const uint8_t *limit, const uint8_t *thresh);
void vpx_lpf_vertical_8_neon(uint8_t *s, int pitch, const uint8_t *blimit, const uint8_t *limit, const uint8_t *thresh);
#define vpx_lpf_vertical_8 vpx_lpf_vertical_8_neon

void vpx_lpf_vertical_8_dual_c(uint8_t *s, int pitch, const uint8_t *blimit0, const uint8_t *limit0, const uint8_t *thresh0, const uint8_t *blimit1, const uint8_t *limit1, const uint8_t *thresh1);
void vpx_lpf_vertical_8_dual_neon(uint8_t *s, int pitch, const uint8_t *blimit0, const uint8_t *limit0, const uint8_t *thresh0, const uint8_t *blimit1, const uint8_t *limit1, const uint8_t *thresh1);
#define vpx_lpf_vertical_8_dual vpx_lpf_vertical_8_dual_neon

void vpx_mbpost_proc_across_ip_c(unsigned char *src, int pitch, int rows, int cols,int flimit);
void vpx_mbpost_proc_across_ip_neon(unsigned char *src, int pitch, int rows, int cols,int flimit);
#define vpx_mbpost_proc_across_ip vpx_mbpost_proc_across_ip_neon

void vpx_mbpost_proc_down_c(unsigned char *dst, int pitch, int rows, int cols,int flimit);
void vpx_mbpost_proc_down_neon(unsigned char *dst, int pitch, int rows, int cols,int flimit);
#define vpx_mbpost_proc_down vpx_mbpost_proc_down_neon

void vpx_minmax_8x8_c(const uint8_t *s, int p, const uint8_t *d, int dp, int *min, int *max);
void vpx_minmax_8x8_neon(const uint8_t *s, int p, const uint8_t *d, int dp, int *min, int *max);
#define vpx_minmax_8x8 vpx_minmax_8x8_neon

unsigned int vpx_mse16x16_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_mse16x16_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_mse16x16_neon_dotprod(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
RTCD_EXTERN unsigned int (*vpx_mse16x16)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);

unsigned int vpx_mse16x8_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_mse16x8_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_mse16x8_neon_dotprod(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
RTCD_EXTERN unsigned int (*vpx_mse16x8)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);

unsigned int vpx_mse8x16_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_mse8x16_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_mse8x16_neon_dotprod(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
RTCD_EXTERN unsigned int (*vpx_mse8x16)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);

unsigned int vpx_mse8x8_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_mse8x8_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_mse8x8_neon_dotprod(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
RTCD_EXTERN unsigned int (*vpx_mse8x8)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);

void vpx_plane_add_noise_c(uint8_t *start, const int8_t *noise, int blackclamp, int whiteclamp, int width, int height, int pitch);
#define vpx_plane_add_noise vpx_plane_add_noise_c

void vpx_post_proc_down_and_across_mb_row_c(unsigned char *src, unsigned char *dst, int src_pitch, int dst_pitch, int cols, unsigned char *flimits, int size);
void vpx_post_proc_down_and_across_mb_row_neon(unsigned char *src, unsigned char *dst, int src_pitch, int dst_pitch, int cols, unsigned char *flimits, int size);
#define vpx_post_proc_down_and_across_mb_row vpx_post_proc_down_and_across_mb_row_neon

void vpx_quantize_b_c(const tran_low_t *coeff_ptr, intptr_t n_coeffs, const struct macroblock_plane *const mb_plane, tran_low_t *qcoeff_ptr, tran_low_t *dqcoeff_ptr, const int16_t *dequant_ptr, uint16_t *eob_ptr, const struct ScanOrder *const scan_order);
void vpx_quantize_b_neon(const tran_low_t *coeff_ptr, intptr_t n_coeffs, const struct macroblock_plane *const mb_plane, tran_low_t *qcoeff_ptr, tran_low_t *dqcoeff_ptr, const int16_t *dequant_ptr, uint16_t *eob_ptr, const struct ScanOrder *const scan_order);
#define vpx_quantize_b vpx_quantize_b_neon

void vpx_quantize_b_32x32_c(const tran_low_t *coeff_ptr, const struct macroblock_plane *const mb_plane, tran_low_t *qcoeff_ptr, tran_low_t *dqcoeff_ptr, const int16_t *dequant_ptr, uint16_t *eob_ptr, const struct ScanOrder *const scan_order);
void vpx_quantize_b_32x32_neon(const tran_low_t *coeff_ptr, const struct macroblock_plane *const mb_plane, tran_low_t *qcoeff_ptr, tran_low_t *dqcoeff_ptr, const int16_t *dequant_ptr, uint16_t *eob_ptr, const struct ScanOrder *const scan_order);
#define vpx_quantize_b_32x32 vpx_quantize_b_32x32_neon

unsigned int vpx_sad16x16_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
unsigned int vpx_sad16x16_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
unsigned int vpx_sad16x16_neon_dotprod(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
RTCD_EXTERN unsigned int (*vpx_sad16x16)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);

unsigned int vpx_sad16x16_avg_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, const uint8_t *second_pred);
unsigned int vpx_sad16x16_avg_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, const uint8_t *second_pred);
unsigned int vpx_sad16x16_avg_neon_dotprod(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, const uint8_t *second_pred);
RTCD_EXTERN unsigned int (*vpx_sad16x16_avg)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, const uint8_t *second_pred);

void vpx_sad16x16x4d_c(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
void vpx_sad16x16x4d_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
void vpx_sad16x16x4d_neon_dotprod(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
RTCD_EXTERN void (*vpx_sad16x16x4d)(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);

unsigned int vpx_sad16x32_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
unsigned int vpx_sad16x32_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
unsigned int vpx_sad16x32_neon_dotprod(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
RTCD_EXTERN unsigned int (*vpx_sad16x32)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);

unsigned int vpx_sad16x32_avg_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, const uint8_t *second_pred);
unsigned int vpx_sad16x32_avg_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, const uint8_t *second_pred);
unsigned int vpx_sad16x32_avg_neon_dotprod(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, const uint8_t *second_pred);
RTCD_EXTERN unsigned int (*vpx_sad16x32_avg)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, const uint8_t *second_pred);

void vpx_sad16x32x4d_c(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
void vpx_sad16x32x4d_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
void vpx_sad16x32x4d_neon_dotprod(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
RTCD_EXTERN void (*vpx_sad16x32x4d)(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);

unsigned int vpx_sad16x8_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
unsigned int vpx_sad16x8_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
unsigned int vpx_sad16x8_neon_dotprod(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
RTCD_EXTERN unsigned int (*vpx_sad16x8)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);

unsigned int vpx_sad16x8_avg_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, const uint8_t *second_pred);
unsigned int vpx_sad16x8_avg_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, const uint8_t *second_pred);
unsigned int vpx_sad16x8_avg_neon_dotprod(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, const uint8_t *second_pred);
RTCD_EXTERN unsigned int (*vpx_sad16x8_avg)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, const uint8_t *second_pred);

void vpx_sad16x8x4d_c(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
void vpx_sad16x8x4d_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
void vpx_sad16x8x4d_neon_dotprod(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
RTCD_EXTERN void (*vpx_sad16x8x4d)(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);

unsigned int vpx_sad32x16_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
unsigned int vpx_sad32x16_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
unsigned int vpx_sad32x16_neon_dotprod(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
RTCD_EXTERN unsigned int (*vpx_sad32x16)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);

unsigned int vpx_sad32x16_avg_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, const uint8_t *second_pred);
unsigned int vpx_sad32x16_avg_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, const uint8_t *second_pred);
unsigned int vpx_sad32x16_avg_neon_dotprod(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, const uint8_t *second_pred);
RTCD_EXTERN unsigned int (*vpx_sad32x16_avg)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, const uint8_t *second_pred);

void vpx_sad32x16x4d_c(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
void vpx_sad32x16x4d_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
void vpx_sad32x16x4d_neon_dotprod(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
RTCD_EXTERN void (*vpx_sad32x16x4d)(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);

unsigned int vpx_sad32x32_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
unsigned int vpx_sad32x32_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
unsigned int vpx_sad32x32_neon_dotprod(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
RTCD_EXTERN unsigned int (*vpx_sad32x32)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);

unsigned int vpx_sad32x32_avg_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, const uint8_t *second_pred);
unsigned int vpx_sad32x32_avg_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, const uint8_t *second_pred);
unsigned int vpx_sad32x32_avg_neon_dotprod(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, const uint8_t *second_pred);
RTCD_EXTERN unsigned int (*vpx_sad32x32_avg)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, const uint8_t *second_pred);

void vpx_sad32x32x4d_c(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
void vpx_sad32x32x4d_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
void vpx_sad32x32x4d_neon_dotprod(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
RTCD_EXTERN void (*vpx_sad32x32x4d)(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);

unsigned int vpx_sad32x64_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
unsigned int vpx_sad32x64_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
unsigned int vpx_sad32x64_neon_dotprod(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
RTCD_EXTERN unsigned int (*vpx_sad32x64)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);

unsigned int vpx_sad32x64_avg_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, const uint8_t *second_pred);
unsigned int vpx_sad32x64_avg_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, const uint8_t *second_pred);
unsigned int vpx_sad32x64_avg_neon_dotprod(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, const uint8_t *second_pred);
RTCD_EXTERN unsigned int (*vpx_sad32x64_avg)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, const uint8_t *second_pred);

void vpx_sad32x64x4d_c(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
void vpx_sad32x64x4d_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
void vpx_sad32x64x4d_neon_dotprod(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
RTCD_EXTERN void (*vpx_sad32x64x4d)(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);

unsigned int vpx_sad4x4_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
unsigned int vpx_sad4x4_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
#define vpx_sad4x4 vpx_sad4x4_neon

unsigned int vpx_sad4x4_avg_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, const uint8_t *second_pred);
unsigned int vpx_sad4x4_avg_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, const uint8_t *second_pred);
#define vpx_sad4x4_avg vpx_sad4x4_avg_neon

void vpx_sad4x4x4d_c(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
void vpx_sad4x4x4d_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
#define vpx_sad4x4x4d vpx_sad4x4x4d_neon

unsigned int vpx_sad4x8_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
unsigned int vpx_sad4x8_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
#define vpx_sad4x8 vpx_sad4x8_neon

unsigned int vpx_sad4x8_avg_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, const uint8_t *second_pred);
unsigned int vpx_sad4x8_avg_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, const uint8_t *second_pred);
#define vpx_sad4x8_avg vpx_sad4x8_avg_neon

void vpx_sad4x8x4d_c(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
void vpx_sad4x8x4d_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
#define vpx_sad4x8x4d vpx_sad4x8x4d_neon

unsigned int vpx_sad64x32_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
unsigned int vpx_sad64x32_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
unsigned int vpx_sad64x32_neon_dotprod(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
RTCD_EXTERN unsigned int (*vpx_sad64x32)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);

unsigned int vpx_sad64x32_avg_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, const uint8_t *second_pred);
unsigned int vpx_sad64x32_avg_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, const uint8_t *second_pred);
unsigned int vpx_sad64x32_avg_neon_dotprod(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, const uint8_t *second_pred);
RTCD_EXTERN unsigned int (*vpx_sad64x32_avg)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, const uint8_t *second_pred);

void vpx_sad64x32x4d_c(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
void vpx_sad64x32x4d_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
void vpx_sad64x32x4d_neon_dotprod(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
RTCD_EXTERN void (*vpx_sad64x32x4d)(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);

unsigned int vpx_sad64x64_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
unsigned int vpx_sad64x64_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
unsigned int vpx_sad64x64_neon_dotprod(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
RTCD_EXTERN unsigned int (*vpx_sad64x64)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);

unsigned int vpx_sad64x64_avg_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, const uint8_t *second_pred);
unsigned int vpx_sad64x64_avg_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, const uint8_t *second_pred);
unsigned int vpx_sad64x64_avg_neon_dotprod(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, const uint8_t *second_pred);
RTCD_EXTERN unsigned int (*vpx_sad64x64_avg)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, const uint8_t *second_pred);

void vpx_sad64x64x4d_c(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
void vpx_sad64x64x4d_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
void vpx_sad64x64x4d_neon_dotprod(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
RTCD_EXTERN void (*vpx_sad64x64x4d)(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);

unsigned int vpx_sad8x16_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
unsigned int vpx_sad8x16_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
#define vpx_sad8x16 vpx_sad8x16_neon

unsigned int vpx_sad8x16_avg_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, const uint8_t *second_pred);
unsigned int vpx_sad8x16_avg_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, const uint8_t *second_pred);
#define vpx_sad8x16_avg vpx_sad8x16_avg_neon

void vpx_sad8x16x4d_c(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
void vpx_sad8x16x4d_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
#define vpx_sad8x16x4d vpx_sad8x16x4d_neon

unsigned int vpx_sad8x4_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
unsigned int vpx_sad8x4_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
#define vpx_sad8x4 vpx_sad8x4_neon

unsigned int vpx_sad8x4_avg_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, const uint8_t *second_pred);
unsigned int vpx_sad8x4_avg_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, const uint8_t *second_pred);
#define vpx_sad8x4_avg vpx_sad8x4_avg_neon

void vpx_sad8x4x4d_c(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
void vpx_sad8x4x4d_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
#define vpx_sad8x4x4d vpx_sad8x4x4d_neon

unsigned int vpx_sad8x8_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
unsigned int vpx_sad8x8_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
#define vpx_sad8x8 vpx_sad8x8_neon

unsigned int vpx_sad8x8_avg_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, const uint8_t *second_pred);
unsigned int vpx_sad8x8_avg_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, const uint8_t *second_pred);
#define vpx_sad8x8_avg vpx_sad8x8_avg_neon

void vpx_sad8x8x4d_c(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
void vpx_sad8x8x4d_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
#define vpx_sad8x8x4d vpx_sad8x8x4d_neon

unsigned int vpx_sad_skip_16x16_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
unsigned int vpx_sad_skip_16x16_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
unsigned int vpx_sad_skip_16x16_neon_dotprod(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
RTCD_EXTERN unsigned int (*vpx_sad_skip_16x16)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);

void vpx_sad_skip_16x16x4d_c(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
void vpx_sad_skip_16x16x4d_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
void vpx_sad_skip_16x16x4d_neon_dotprod(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
RTCD_EXTERN void (*vpx_sad_skip_16x16x4d)(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);

unsigned int vpx_sad_skip_16x32_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
unsigned int vpx_sad_skip_16x32_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
unsigned int vpx_sad_skip_16x32_neon_dotprod(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
RTCD_EXTERN unsigned int (*vpx_sad_skip_16x32)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);

void vpx_sad_skip_16x32x4d_c(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
void vpx_sad_skip_16x32x4d_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
void vpx_sad_skip_16x32x4d_neon_dotprod(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
RTCD_EXTERN void (*vpx_sad_skip_16x32x4d)(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);

unsigned int vpx_sad_skip_16x8_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
unsigned int vpx_sad_skip_16x8_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
unsigned int vpx_sad_skip_16x8_neon_dotprod(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
RTCD_EXTERN unsigned int (*vpx_sad_skip_16x8)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);

void vpx_sad_skip_16x8x4d_c(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
void vpx_sad_skip_16x8x4d_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
void vpx_sad_skip_16x8x4d_neon_dotprod(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
RTCD_EXTERN void (*vpx_sad_skip_16x8x4d)(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);

unsigned int vpx_sad_skip_32x16_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
unsigned int vpx_sad_skip_32x16_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
unsigned int vpx_sad_skip_32x16_neon_dotprod(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
RTCD_EXTERN unsigned int (*vpx_sad_skip_32x16)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);

void vpx_sad_skip_32x16x4d_c(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
void vpx_sad_skip_32x16x4d_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
void vpx_sad_skip_32x16x4d_neon_dotprod(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
RTCD_EXTERN void (*vpx_sad_skip_32x16x4d)(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);

unsigned int vpx_sad_skip_32x32_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
unsigned int vpx_sad_skip_32x32_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
unsigned int vpx_sad_skip_32x32_neon_dotprod(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
RTCD_EXTERN unsigned int (*vpx_sad_skip_32x32)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);

void vpx_sad_skip_32x32x4d_c(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
void vpx_sad_skip_32x32x4d_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
void vpx_sad_skip_32x32x4d_neon_dotprod(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
RTCD_EXTERN void (*vpx_sad_skip_32x32x4d)(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);

unsigned int vpx_sad_skip_32x64_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
unsigned int vpx_sad_skip_32x64_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
unsigned int vpx_sad_skip_32x64_neon_dotprod(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
RTCD_EXTERN unsigned int (*vpx_sad_skip_32x64)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);

void vpx_sad_skip_32x64x4d_c(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
void vpx_sad_skip_32x64x4d_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
void vpx_sad_skip_32x64x4d_neon_dotprod(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
RTCD_EXTERN void (*vpx_sad_skip_32x64x4d)(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);

unsigned int vpx_sad_skip_4x4_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
unsigned int vpx_sad_skip_4x4_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
#define vpx_sad_skip_4x4 vpx_sad_skip_4x4_neon

void vpx_sad_skip_4x4x4d_c(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
void vpx_sad_skip_4x4x4d_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
#define vpx_sad_skip_4x4x4d vpx_sad_skip_4x4x4d_neon

unsigned int vpx_sad_skip_4x8_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
unsigned int vpx_sad_skip_4x8_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
#define vpx_sad_skip_4x8 vpx_sad_skip_4x8_neon

void vpx_sad_skip_4x8x4d_c(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
void vpx_sad_skip_4x8x4d_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
#define vpx_sad_skip_4x8x4d vpx_sad_skip_4x8x4d_neon

unsigned int vpx_sad_skip_64x32_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
unsigned int vpx_sad_skip_64x32_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
unsigned int vpx_sad_skip_64x32_neon_dotprod(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
RTCD_EXTERN unsigned int (*vpx_sad_skip_64x32)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);

void vpx_sad_skip_64x32x4d_c(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
void vpx_sad_skip_64x32x4d_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
void vpx_sad_skip_64x32x4d_neon_dotprod(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
RTCD_EXTERN void (*vpx_sad_skip_64x32x4d)(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);

unsigned int vpx_sad_skip_64x64_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
unsigned int vpx_sad_skip_64x64_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
unsigned int vpx_sad_skip_64x64_neon_dotprod(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
RTCD_EXTERN unsigned int (*vpx_sad_skip_64x64)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);

void vpx_sad_skip_64x64x4d_c(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
void vpx_sad_skip_64x64x4d_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
void vpx_sad_skip_64x64x4d_neon_dotprod(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
RTCD_EXTERN void (*vpx_sad_skip_64x64x4d)(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);

unsigned int vpx_sad_skip_8x16_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
unsigned int vpx_sad_skip_8x16_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
#define vpx_sad_skip_8x16 vpx_sad_skip_8x16_neon

void vpx_sad_skip_8x16x4d_c(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
void vpx_sad_skip_8x16x4d_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
#define vpx_sad_skip_8x16x4d vpx_sad_skip_8x16x4d_neon

unsigned int vpx_sad_skip_8x4_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
unsigned int vpx_sad_skip_8x4_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
#define vpx_sad_skip_8x4 vpx_sad_skip_8x4_neon

void vpx_sad_skip_8x4x4d_c(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
void vpx_sad_skip_8x4x4d_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
#define vpx_sad_skip_8x4x4d vpx_sad_skip_8x4x4d_neon

unsigned int vpx_sad_skip_8x8_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
unsigned int vpx_sad_skip_8x8_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride);
#define vpx_sad_skip_8x8 vpx_sad_skip_8x8_neon

void vpx_sad_skip_8x8x4d_c(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
void vpx_sad_skip_8x8x4d_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *const ref_array[4], int ref_stride, uint32_t sad_array[4]);
#define vpx_sad_skip_8x8x4d vpx_sad_skip_8x8x4d_neon

int vpx_satd_c(const tran_low_t *coeff, int length);
int vpx_satd_neon(const tran_low_t *coeff, int length);
#define vpx_satd vpx_satd_neon

void vpx_scaled_2d_c(const uint8_t *src, ptrdiff_t src_stride, uint8_t *dst, ptrdiff_t dst_stride, const InterpKernel *filter, int x0_q4, int x_step_q4, int y0_q4, int y_step_q4, int w, int h);
void vpx_scaled_2d_neon(const uint8_t *src, ptrdiff_t src_stride, uint8_t *dst, ptrdiff_t dst_stride, const InterpKernel *filter, int x0_q4, int x_step_q4, int y0_q4, int y_step_q4, int w, int h);
#define vpx_scaled_2d vpx_scaled_2d_neon

void vpx_scaled_avg_2d_c(const uint8_t *src, ptrdiff_t src_stride, uint8_t *dst, ptrdiff_t dst_stride, const InterpKernel *filter, int x0_q4, int x_step_q4, int y0_q4, int y_step_q4, int w, int h);
#define vpx_scaled_avg_2d vpx_scaled_avg_2d_c

void vpx_scaled_avg_horiz_c(const uint8_t *src, ptrdiff_t src_stride, uint8_t *dst, ptrdiff_t dst_stride, const InterpKernel *filter, int x0_q4, int x_step_q4, int y0_q4, int y_step_q4, int w, int h);
#define vpx_scaled_avg_horiz vpx_scaled_avg_horiz_c

void vpx_scaled_avg_vert_c(const uint8_t *src, ptrdiff_t src_stride, uint8_t *dst, ptrdiff_t dst_stride, const InterpKernel *filter, int x0_q4, int x_step_q4, int y0_q4, int y_step_q4, int w, int h);
#define vpx_scaled_avg_vert vpx_scaled_avg_vert_c

void vpx_scaled_horiz_c(const uint8_t *src, ptrdiff_t src_stride, uint8_t *dst, ptrdiff_t dst_stride, const InterpKernel *filter, int x0_q4, int x_step_q4, int y0_q4, int y_step_q4, int w, int h);
#define vpx_scaled_horiz vpx_scaled_horiz_c

void vpx_scaled_vert_c(const uint8_t *src, ptrdiff_t src_stride, uint8_t *dst, ptrdiff_t dst_stride, const InterpKernel *filter, int x0_q4, int x_step_q4, int y0_q4, int y_step_q4, int w, int h);
#define vpx_scaled_vert vpx_scaled_vert_c

int64_t vpx_sse_c(const uint8_t *src, int src_stride, const uint8_t *ref, int ref_stride, int width, int height);
int64_t vpx_sse_neon(const uint8_t *src, int src_stride, const uint8_t *ref, int ref_stride, int width, int height);
int64_t vpx_sse_neon_dotprod(const uint8_t *src, int src_stride, const uint8_t *ref, int ref_stride, int width, int height);
RTCD_EXTERN int64_t (*vpx_sse)(const uint8_t *src, int src_stride, const uint8_t *ref, int ref_stride, int width, int height);

uint32_t vpx_sub_pixel_avg_variance16x16_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
uint32_t vpx_sub_pixel_avg_variance16x16_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
#define vpx_sub_pixel_avg_variance16x16 vpx_sub_pixel_avg_variance16x16_neon

uint32_t vpx_sub_pixel_avg_variance16x32_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
uint32_t vpx_sub_pixel_avg_variance16x32_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
#define vpx_sub_pixel_avg_variance16x32 vpx_sub_pixel_avg_variance16x32_neon

uint32_t vpx_sub_pixel_avg_variance16x8_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
uint32_t vpx_sub_pixel_avg_variance16x8_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
#define vpx_sub_pixel_avg_variance16x8 vpx_sub_pixel_avg_variance16x8_neon

uint32_t vpx_sub_pixel_avg_variance32x16_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
uint32_t vpx_sub_pixel_avg_variance32x16_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
#define vpx_sub_pixel_avg_variance32x16 vpx_sub_pixel_avg_variance32x16_neon

uint32_t vpx_sub_pixel_avg_variance32x32_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
uint32_t vpx_sub_pixel_avg_variance32x32_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
#define vpx_sub_pixel_avg_variance32x32 vpx_sub_pixel_avg_variance32x32_neon

uint32_t vpx_sub_pixel_avg_variance32x64_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
uint32_t vpx_sub_pixel_avg_variance32x64_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
#define vpx_sub_pixel_avg_variance32x64 vpx_sub_pixel_avg_variance32x64_neon

uint32_t vpx_sub_pixel_avg_variance4x4_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
uint32_t vpx_sub_pixel_avg_variance4x4_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
#define vpx_sub_pixel_avg_variance4x4 vpx_sub_pixel_avg_variance4x4_neon

uint32_t vpx_sub_pixel_avg_variance4x8_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
uint32_t vpx_sub_pixel_avg_variance4x8_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
#define vpx_sub_pixel_avg_variance4x8 vpx_sub_pixel_avg_variance4x8_neon

uint32_t vpx_sub_pixel_avg_variance64x32_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
uint32_t vpx_sub_pixel_avg_variance64x32_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
#define vpx_sub_pixel_avg_variance64x32 vpx_sub_pixel_avg_variance64x32_neon

uint32_t vpx_sub_pixel_avg_variance64x64_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
uint32_t vpx_sub_pixel_avg_variance64x64_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
#define vpx_sub_pixel_avg_variance64x64 vpx_sub_pixel_avg_variance64x64_neon

uint32_t vpx_sub_pixel_avg_variance8x16_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
uint32_t vpx_sub_pixel_avg_variance8x16_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
#define vpx_sub_pixel_avg_variance8x16 vpx_sub_pixel_avg_variance8x16_neon

uint32_t vpx_sub_pixel_avg_variance8x4_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
uint32_t vpx_sub_pixel_avg_variance8x4_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
#define vpx_sub_pixel_avg_variance8x4 vpx_sub_pixel_avg_variance8x4_neon

uint32_t vpx_sub_pixel_avg_variance8x8_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
uint32_t vpx_sub_pixel_avg_variance8x8_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse, const uint8_t *second_pred);
#define vpx_sub_pixel_avg_variance8x8 vpx_sub_pixel_avg_variance8x8_neon

uint32_t vpx_sub_pixel_variance16x16_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
uint32_t vpx_sub_pixel_variance16x16_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
#define vpx_sub_pixel_variance16x16 vpx_sub_pixel_variance16x16_neon

uint32_t vpx_sub_pixel_variance16x32_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
uint32_t vpx_sub_pixel_variance16x32_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
#define vpx_sub_pixel_variance16x32 vpx_sub_pixel_variance16x32_neon

uint32_t vpx_sub_pixel_variance16x8_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
uint32_t vpx_sub_pixel_variance16x8_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
#define vpx_sub_pixel_variance16x8 vpx_sub_pixel_variance16x8_neon

uint32_t vpx_sub_pixel_variance32x16_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
uint32_t vpx_sub_pixel_variance32x16_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
#define vpx_sub_pixel_variance32x16 vpx_sub_pixel_variance32x16_neon

uint32_t vpx_sub_pixel_variance32x32_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
uint32_t vpx_sub_pixel_variance32x32_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
#define vpx_sub_pixel_variance32x32 vpx_sub_pixel_variance32x32_neon

uint32_t vpx_sub_pixel_variance32x64_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
uint32_t vpx_sub_pixel_variance32x64_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
#define vpx_sub_pixel_variance32x64 vpx_sub_pixel_variance32x64_neon

uint32_t vpx_sub_pixel_variance4x4_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
uint32_t vpx_sub_pixel_variance4x4_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
#define vpx_sub_pixel_variance4x4 vpx_sub_pixel_variance4x4_neon

uint32_t vpx_sub_pixel_variance4x8_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
uint32_t vpx_sub_pixel_variance4x8_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
#define vpx_sub_pixel_variance4x8 vpx_sub_pixel_variance4x8_neon

uint32_t vpx_sub_pixel_variance64x32_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
uint32_t vpx_sub_pixel_variance64x32_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
#define vpx_sub_pixel_variance64x32 vpx_sub_pixel_variance64x32_neon

uint32_t vpx_sub_pixel_variance64x64_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
uint32_t vpx_sub_pixel_variance64x64_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
#define vpx_sub_pixel_variance64x64 vpx_sub_pixel_variance64x64_neon

uint32_t vpx_sub_pixel_variance8x16_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
uint32_t vpx_sub_pixel_variance8x16_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
#define vpx_sub_pixel_variance8x16 vpx_sub_pixel_variance8x16_neon

uint32_t vpx_sub_pixel_variance8x4_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
uint32_t vpx_sub_pixel_variance8x4_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
#define vpx_sub_pixel_variance8x4 vpx_sub_pixel_variance8x4_neon

uint32_t vpx_sub_pixel_variance8x8_c(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
uint32_t vpx_sub_pixel_variance8x8_neon(const uint8_t *src_ptr, int src_stride, int x_offset, int y_offset, const uint8_t *ref_ptr, int ref_stride, uint32_t *sse);
#define vpx_sub_pixel_variance8x8 vpx_sub_pixel_variance8x8_neon

void vpx_subtract_block_c(int rows, int cols, int16_t *diff_ptr, ptrdiff_t diff_stride, const uint8_t *src_ptr, ptrdiff_t src_stride, const uint8_t *pred_ptr, ptrdiff_t pred_stride);
void vpx_subtract_block_neon(int rows, int cols, int16_t *diff_ptr, ptrdiff_t diff_stride, const uint8_t *src_ptr, ptrdiff_t src_stride, const uint8_t *pred_ptr, ptrdiff_t pred_stride);
#define vpx_subtract_block vpx_subtract_block_neon

uint64_t vpx_sum_squares_2d_i16_c(const int16_t *src, int stride, int size);
uint64_t vpx_sum_squares_2d_i16_neon(const int16_t *src, int stride, int size);
uint64_t vpx_sum_squares_2d_i16_sve(const int16_t *src, int stride, int size);
RTCD_EXTERN uint64_t (*vpx_sum_squares_2d_i16)(const int16_t *src, int stride, int size);

void vpx_tm_predictor_16x16_c(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
void vpx_tm_predictor_16x16_neon(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
#define vpx_tm_predictor_16x16 vpx_tm_predictor_16x16_neon

void vpx_tm_predictor_32x32_c(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
void vpx_tm_predictor_32x32_neon(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
#define vpx_tm_predictor_32x32 vpx_tm_predictor_32x32_neon

void vpx_tm_predictor_4x4_c(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
void vpx_tm_predictor_4x4_neon(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
#define vpx_tm_predictor_4x4 vpx_tm_predictor_4x4_neon

void vpx_tm_predictor_8x8_c(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
void vpx_tm_predictor_8x8_neon(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
#define vpx_tm_predictor_8x8 vpx_tm_predictor_8x8_neon

void vpx_v_predictor_16x16_c(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
void vpx_v_predictor_16x16_neon(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
#define vpx_v_predictor_16x16 vpx_v_predictor_16x16_neon

void vpx_v_predictor_32x32_c(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
void vpx_v_predictor_32x32_neon(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
#define vpx_v_predictor_32x32 vpx_v_predictor_32x32_neon

void vpx_v_predictor_4x4_c(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
void vpx_v_predictor_4x4_neon(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
#define vpx_v_predictor_4x4 vpx_v_predictor_4x4_neon

void vpx_v_predictor_8x8_c(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
void vpx_v_predictor_8x8_neon(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
#define vpx_v_predictor_8x8 vpx_v_predictor_8x8_neon

unsigned int vpx_variance16x16_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_variance16x16_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_variance16x16_neon_dotprod(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
RTCD_EXTERN unsigned int (*vpx_variance16x16)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);

unsigned int vpx_variance16x32_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_variance16x32_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_variance16x32_neon_dotprod(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
RTCD_EXTERN unsigned int (*vpx_variance16x32)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);

unsigned int vpx_variance16x8_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_variance16x8_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_variance16x8_neon_dotprod(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
RTCD_EXTERN unsigned int (*vpx_variance16x8)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);

unsigned int vpx_variance32x16_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_variance32x16_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_variance32x16_neon_dotprod(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
RTCD_EXTERN unsigned int (*vpx_variance32x16)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);

unsigned int vpx_variance32x32_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_variance32x32_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_variance32x32_neon_dotprod(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
RTCD_EXTERN unsigned int (*vpx_variance32x32)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);

unsigned int vpx_variance32x64_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_variance32x64_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_variance32x64_neon_dotprod(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
RTCD_EXTERN unsigned int (*vpx_variance32x64)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);

unsigned int vpx_variance4x4_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_variance4x4_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_variance4x4_neon_dotprod(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
RTCD_EXTERN unsigned int (*vpx_variance4x4)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);

unsigned int vpx_variance4x8_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_variance4x8_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_variance4x8_neon_dotprod(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
RTCD_EXTERN unsigned int (*vpx_variance4x8)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);

unsigned int vpx_variance64x32_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_variance64x32_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_variance64x32_neon_dotprod(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
RTCD_EXTERN unsigned int (*vpx_variance64x32)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);

unsigned int vpx_variance64x64_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_variance64x64_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_variance64x64_neon_dotprod(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
RTCD_EXTERN unsigned int (*vpx_variance64x64)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);

unsigned int vpx_variance8x16_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_variance8x16_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_variance8x16_neon_dotprod(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
RTCD_EXTERN unsigned int (*vpx_variance8x16)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);

unsigned int vpx_variance8x4_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_variance8x4_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_variance8x4_neon_dotprod(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
RTCD_EXTERN unsigned int (*vpx_variance8x4)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);

unsigned int vpx_variance8x8_c(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_variance8x8_neon(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
unsigned int vpx_variance8x8_neon_dotprod(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);
RTCD_EXTERN unsigned int (*vpx_variance8x8)(const uint8_t *src_ptr, int src_stride, const uint8_t *ref_ptr, int ref_stride, unsigned int *sse);

void vpx_ve_predictor_4x4_c(uint8_t *dst, ptrdiff_t stride, const uint8_t *above, const uint8_t *left);
#define vpx_ve_predictor_4x4 vpx_ve_predictor_4x4_c

int vpx_vector_var_c(const int16_t *ref, const int16_t *src, const int bwl);
int vpx_vector_var_neon(const int16_t *ref, const int16_t *src, const int bwl);
#define vpx_vector_var vpx_vector_var_neon

void vpx_dsp_rtcd(void);

#include "vpx_config.h"

#ifdef RTCD_C
#include "vpx_ports/arm.h"
static void setup_rtcd_internal(void)
{
    int flags = arm_cpu_caps();

    (void)flags;

    vpx_convolve8 = vpx_convolve8_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_convolve8 = vpx_convolve8_neon_dotprod;
    if (flags & HAS_NEON_I8MM) vpx_convolve8 = vpx_convolve8_neon_i8mm;
    vpx_convolve8_avg = vpx_convolve8_avg_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_convolve8_avg = vpx_convolve8_avg_neon_dotprod;
    if (flags & HAS_NEON_I8MM) vpx_convolve8_avg = vpx_convolve8_avg_neon_i8mm;
    vpx_convolve8_avg_horiz = vpx_convolve8_avg_horiz_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_convolve8_avg_horiz = vpx_convolve8_avg_horiz_neon_dotprod;
    if (flags & HAS_NEON_I8MM) vpx_convolve8_avg_horiz = vpx_convolve8_avg_horiz_neon_i8mm;
    vpx_convolve8_avg_vert = vpx_convolve8_avg_vert_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_convolve8_avg_vert = vpx_convolve8_avg_vert_neon_dotprod;
    if (flags & HAS_NEON_I8MM) vpx_convolve8_avg_vert = vpx_convolve8_avg_vert_neon_i8mm;
    vpx_convolve8_horiz = vpx_convolve8_horiz_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_convolve8_horiz = vpx_convolve8_horiz_neon_dotprod;
    if (flags & HAS_NEON_I8MM) vpx_convolve8_horiz = vpx_convolve8_horiz_neon_i8mm;
    vpx_convolve8_vert = vpx_convolve8_vert_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_convolve8_vert = vpx_convolve8_vert_neon_dotprod;
    if (flags & HAS_NEON_I8MM) vpx_convolve8_vert = vpx_convolve8_vert_neon_i8mm;
    vpx_get16x16var = vpx_get16x16var_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_get16x16var = vpx_get16x16var_neon_dotprod;
    vpx_get4x4sse_cs = vpx_get4x4sse_cs_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_get4x4sse_cs = vpx_get4x4sse_cs_neon_dotprod;
    vpx_get8x8var = vpx_get8x8var_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_get8x8var = vpx_get8x8var_neon_dotprod;
    vpx_highbd_10_get16x16var = vpx_highbd_10_get16x16var_neon;
    if (flags & HAS_SVE) vpx_highbd_10_get16x16var = vpx_highbd_10_get16x16var_sve;
    vpx_highbd_10_get8x8var = vpx_highbd_10_get8x8var_neon;
    if (flags & HAS_SVE) vpx_highbd_10_get8x8var = vpx_highbd_10_get8x8var_sve;
    vpx_highbd_10_mse16x16 = vpx_highbd_10_mse16x16_neon;
    if (flags & HAS_SVE) vpx_highbd_10_mse16x16 = vpx_highbd_10_mse16x16_sve;
    vpx_highbd_10_mse16x8 = vpx_highbd_10_mse16x8_neon;
    if (flags & HAS_SVE) vpx_highbd_10_mse16x8 = vpx_highbd_10_mse16x8_sve;
    vpx_highbd_10_mse8x16 = vpx_highbd_10_mse8x16_neon;
    if (flags & HAS_SVE) vpx_highbd_10_mse8x16 = vpx_highbd_10_mse8x16_sve;
    vpx_highbd_10_mse8x8 = vpx_highbd_10_mse8x8_neon;
    if (flags & HAS_SVE) vpx_highbd_10_mse8x8 = vpx_highbd_10_mse8x8_sve;
    vpx_highbd_10_variance16x16 = vpx_highbd_10_variance16x16_neon;
    if (flags & HAS_SVE) vpx_highbd_10_variance16x16 = vpx_highbd_10_variance16x16_sve;
    vpx_highbd_10_variance16x32 = vpx_highbd_10_variance16x32_neon;
    if (flags & HAS_SVE) vpx_highbd_10_variance16x32 = vpx_highbd_10_variance16x32_sve;
    vpx_highbd_10_variance16x8 = vpx_highbd_10_variance16x8_neon;
    if (flags & HAS_SVE) vpx_highbd_10_variance16x8 = vpx_highbd_10_variance16x8_sve;
    vpx_highbd_10_variance32x16 = vpx_highbd_10_variance32x16_neon;
    if (flags & HAS_SVE) vpx_highbd_10_variance32x16 = vpx_highbd_10_variance32x16_sve;
    vpx_highbd_10_variance32x32 = vpx_highbd_10_variance32x32_neon;
    if (flags & HAS_SVE) vpx_highbd_10_variance32x32 = vpx_highbd_10_variance32x32_sve;
    vpx_highbd_10_variance32x64 = vpx_highbd_10_variance32x64_neon;
    if (flags & HAS_SVE) vpx_highbd_10_variance32x64 = vpx_highbd_10_variance32x64_sve;
    vpx_highbd_10_variance4x4 = vpx_highbd_10_variance4x4_neon;
    if (flags & HAS_SVE) vpx_highbd_10_variance4x4 = vpx_highbd_10_variance4x4_sve;
    vpx_highbd_10_variance4x8 = vpx_highbd_10_variance4x8_neon;
    if (flags & HAS_SVE) vpx_highbd_10_variance4x8 = vpx_highbd_10_variance4x8_sve;
    vpx_highbd_10_variance64x32 = vpx_highbd_10_variance64x32_neon;
    if (flags & HAS_SVE) vpx_highbd_10_variance64x32 = vpx_highbd_10_variance64x32_sve;
    vpx_highbd_10_variance64x64 = vpx_highbd_10_variance64x64_neon;
    if (flags & HAS_SVE) vpx_highbd_10_variance64x64 = vpx_highbd_10_variance64x64_sve;
    vpx_highbd_10_variance8x16 = vpx_highbd_10_variance8x16_neon;
    if (flags & HAS_SVE) vpx_highbd_10_variance8x16 = vpx_highbd_10_variance8x16_sve;
    vpx_highbd_10_variance8x4 = vpx_highbd_10_variance8x4_neon;
    if (flags & HAS_SVE) vpx_highbd_10_variance8x4 = vpx_highbd_10_variance8x4_sve;
    vpx_highbd_10_variance8x8 = vpx_highbd_10_variance8x8_neon;
    if (flags & HAS_SVE) vpx_highbd_10_variance8x8 = vpx_highbd_10_variance8x8_sve;
    vpx_highbd_12_get16x16var = vpx_highbd_12_get16x16var_neon;
    if (flags & HAS_SVE) vpx_highbd_12_get16x16var = vpx_highbd_12_get16x16var_sve;
    vpx_highbd_12_get8x8var = vpx_highbd_12_get8x8var_neon;
    if (flags & HAS_SVE) vpx_highbd_12_get8x8var = vpx_highbd_12_get8x8var_sve;
    vpx_highbd_12_mse16x16 = vpx_highbd_12_mse16x16_neon;
    if (flags & HAS_SVE) vpx_highbd_12_mse16x16 = vpx_highbd_12_mse16x16_sve;
    vpx_highbd_12_mse16x8 = vpx_highbd_12_mse16x8_neon;
    if (flags & HAS_SVE) vpx_highbd_12_mse16x8 = vpx_highbd_12_mse16x8_sve;
    vpx_highbd_12_mse8x16 = vpx_highbd_12_mse8x16_neon;
    if (flags & HAS_SVE) vpx_highbd_12_mse8x16 = vpx_highbd_12_mse8x16_sve;
    vpx_highbd_12_mse8x8 = vpx_highbd_12_mse8x8_neon;
    if (flags & HAS_SVE) vpx_highbd_12_mse8x8 = vpx_highbd_12_mse8x8_sve;
    vpx_highbd_12_variance16x16 = vpx_highbd_12_variance16x16_neon;
    if (flags & HAS_SVE) vpx_highbd_12_variance16x16 = vpx_highbd_12_variance16x16_sve;
    vpx_highbd_12_variance16x32 = vpx_highbd_12_variance16x32_neon;
    if (flags & HAS_SVE) vpx_highbd_12_variance16x32 = vpx_highbd_12_variance16x32_sve;
    vpx_highbd_12_variance16x8 = vpx_highbd_12_variance16x8_neon;
    if (flags & HAS_SVE) vpx_highbd_12_variance16x8 = vpx_highbd_12_variance16x8_sve;
    vpx_highbd_12_variance32x16 = vpx_highbd_12_variance32x16_neon;
    if (flags & HAS_SVE) vpx_highbd_12_variance32x16 = vpx_highbd_12_variance32x16_sve;
    vpx_highbd_12_variance32x32 = vpx_highbd_12_variance32x32_neon;
    if (flags & HAS_SVE) vpx_highbd_12_variance32x32 = vpx_highbd_12_variance32x32_sve;
    vpx_highbd_12_variance32x64 = vpx_highbd_12_variance32x64_neon;
    if (flags & HAS_SVE) vpx_highbd_12_variance32x64 = vpx_highbd_12_variance32x64_sve;
    vpx_highbd_12_variance4x4 = vpx_highbd_12_variance4x4_neon;
    if (flags & HAS_SVE) vpx_highbd_12_variance4x4 = vpx_highbd_12_variance4x4_sve;
    vpx_highbd_12_variance4x8 = vpx_highbd_12_variance4x8_neon;
    if (flags & HAS_SVE) vpx_highbd_12_variance4x8 = vpx_highbd_12_variance4x8_sve;
    vpx_highbd_12_variance64x32 = vpx_highbd_12_variance64x32_neon;
    if (flags & HAS_SVE) vpx_highbd_12_variance64x32 = vpx_highbd_12_variance64x32_sve;
    vpx_highbd_12_variance64x64 = vpx_highbd_12_variance64x64_neon;
    if (flags & HAS_SVE) vpx_highbd_12_variance64x64 = vpx_highbd_12_variance64x64_sve;
    vpx_highbd_12_variance8x16 = vpx_highbd_12_variance8x16_neon;
    if (flags & HAS_SVE) vpx_highbd_12_variance8x16 = vpx_highbd_12_variance8x16_sve;
    vpx_highbd_12_variance8x4 = vpx_highbd_12_variance8x4_neon;
    if (flags & HAS_SVE) vpx_highbd_12_variance8x4 = vpx_highbd_12_variance8x4_sve;
    vpx_highbd_12_variance8x8 = vpx_highbd_12_variance8x8_neon;
    if (flags & HAS_SVE) vpx_highbd_12_variance8x8 = vpx_highbd_12_variance8x8_sve;
    vpx_highbd_8_get16x16var = vpx_highbd_8_get16x16var_neon;
    if (flags & HAS_SVE) vpx_highbd_8_get16x16var = vpx_highbd_8_get16x16var_sve;
    vpx_highbd_8_get8x8var = vpx_highbd_8_get8x8var_neon;
    if (flags & HAS_SVE) vpx_highbd_8_get8x8var = vpx_highbd_8_get8x8var_sve;
    vpx_highbd_8_mse16x16 = vpx_highbd_8_mse16x16_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_highbd_8_mse16x16 = vpx_highbd_8_mse16x16_neon_dotprod;
    vpx_highbd_8_mse16x8 = vpx_highbd_8_mse16x8_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_highbd_8_mse16x8 = vpx_highbd_8_mse16x8_neon_dotprod;
    vpx_highbd_8_mse8x16 = vpx_highbd_8_mse8x16_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_highbd_8_mse8x16 = vpx_highbd_8_mse8x16_neon_dotprod;
    vpx_highbd_8_mse8x8 = vpx_highbd_8_mse8x8_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_highbd_8_mse8x8 = vpx_highbd_8_mse8x8_neon_dotprod;
    vpx_highbd_8_variance16x16 = vpx_highbd_8_variance16x16_neon;
    if (flags & HAS_SVE) vpx_highbd_8_variance16x16 = vpx_highbd_8_variance16x16_sve;
    vpx_highbd_8_variance16x32 = vpx_highbd_8_variance16x32_neon;
    if (flags & HAS_SVE) vpx_highbd_8_variance16x32 = vpx_highbd_8_variance16x32_sve;
    vpx_highbd_8_variance16x8 = vpx_highbd_8_variance16x8_neon;
    if (flags & HAS_SVE) vpx_highbd_8_variance16x8 = vpx_highbd_8_variance16x8_sve;
    vpx_highbd_8_variance32x16 = vpx_highbd_8_variance32x16_neon;
    if (flags & HAS_SVE) vpx_highbd_8_variance32x16 = vpx_highbd_8_variance32x16_sve;
    vpx_highbd_8_variance32x32 = vpx_highbd_8_variance32x32_neon;
    if (flags & HAS_SVE) vpx_highbd_8_variance32x32 = vpx_highbd_8_variance32x32_sve;
    vpx_highbd_8_variance32x64 = vpx_highbd_8_variance32x64_neon;
    if (flags & HAS_SVE) vpx_highbd_8_variance32x64 = vpx_highbd_8_variance32x64_sve;
    vpx_highbd_8_variance4x4 = vpx_highbd_8_variance4x4_neon;
    if (flags & HAS_SVE) vpx_highbd_8_variance4x4 = vpx_highbd_8_variance4x4_sve;
    vpx_highbd_8_variance4x8 = vpx_highbd_8_variance4x8_neon;
    if (flags & HAS_SVE) vpx_highbd_8_variance4x8 = vpx_highbd_8_variance4x8_sve;
    vpx_highbd_8_variance64x32 = vpx_highbd_8_variance64x32_neon;
    if (flags & HAS_SVE) vpx_highbd_8_variance64x32 = vpx_highbd_8_variance64x32_sve;
    vpx_highbd_8_variance64x64 = vpx_highbd_8_variance64x64_neon;
    if (flags & HAS_SVE) vpx_highbd_8_variance64x64 = vpx_highbd_8_variance64x64_sve;
    vpx_highbd_8_variance8x16 = vpx_highbd_8_variance8x16_neon;
    if (flags & HAS_SVE) vpx_highbd_8_variance8x16 = vpx_highbd_8_variance8x16_sve;
    vpx_highbd_8_variance8x4 = vpx_highbd_8_variance8x4_neon;
    if (flags & HAS_SVE) vpx_highbd_8_variance8x4 = vpx_highbd_8_variance8x4_sve;
    vpx_highbd_8_variance8x8 = vpx_highbd_8_variance8x8_neon;
    if (flags & HAS_SVE) vpx_highbd_8_variance8x8 = vpx_highbd_8_variance8x8_sve;
    vpx_highbd_convolve8 = vpx_highbd_convolve8_neon;
    if (flags & HAS_SVE2) vpx_highbd_convolve8 = vpx_highbd_convolve8_sve2;
    vpx_highbd_convolve8_avg = vpx_highbd_convolve8_avg_neon;
    if (flags & HAS_SVE2) vpx_highbd_convolve8_avg = vpx_highbd_convolve8_avg_sve2;
    vpx_highbd_convolve8_avg_horiz = vpx_highbd_convolve8_avg_horiz_neon;
    if (flags & HAS_SVE) vpx_highbd_convolve8_avg_horiz = vpx_highbd_convolve8_avg_horiz_sve;
    vpx_highbd_convolve8_avg_vert = vpx_highbd_convolve8_avg_vert_neon;
    if (flags & HAS_SVE2) vpx_highbd_convolve8_avg_vert = vpx_highbd_convolve8_avg_vert_sve2;
    vpx_highbd_convolve8_horiz = vpx_highbd_convolve8_horiz_neon;
    if (flags & HAS_SVE) vpx_highbd_convolve8_horiz = vpx_highbd_convolve8_horiz_sve;
    vpx_highbd_convolve8_vert = vpx_highbd_convolve8_vert_neon;
    if (flags & HAS_SVE2) vpx_highbd_convolve8_vert = vpx_highbd_convolve8_vert_sve2;
    vpx_mse16x16 = vpx_mse16x16_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_mse16x16 = vpx_mse16x16_neon_dotprod;
    vpx_mse16x8 = vpx_mse16x8_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_mse16x8 = vpx_mse16x8_neon_dotprod;
    vpx_mse8x16 = vpx_mse8x16_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_mse8x16 = vpx_mse8x16_neon_dotprod;
    vpx_mse8x8 = vpx_mse8x8_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_mse8x8 = vpx_mse8x8_neon_dotprod;
    vpx_sad16x16 = vpx_sad16x16_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_sad16x16 = vpx_sad16x16_neon_dotprod;
    vpx_sad16x16_avg = vpx_sad16x16_avg_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_sad16x16_avg = vpx_sad16x16_avg_neon_dotprod;
    vpx_sad16x16x4d = vpx_sad16x16x4d_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_sad16x16x4d = vpx_sad16x16x4d_neon_dotprod;
    vpx_sad16x32 = vpx_sad16x32_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_sad16x32 = vpx_sad16x32_neon_dotprod;
    vpx_sad16x32_avg = vpx_sad16x32_avg_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_sad16x32_avg = vpx_sad16x32_avg_neon_dotprod;
    vpx_sad16x32x4d = vpx_sad16x32x4d_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_sad16x32x4d = vpx_sad16x32x4d_neon_dotprod;
    vpx_sad16x8 = vpx_sad16x8_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_sad16x8 = vpx_sad16x8_neon_dotprod;
    vpx_sad16x8_avg = vpx_sad16x8_avg_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_sad16x8_avg = vpx_sad16x8_avg_neon_dotprod;
    vpx_sad16x8x4d = vpx_sad16x8x4d_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_sad16x8x4d = vpx_sad16x8x4d_neon_dotprod;
    vpx_sad32x16 = vpx_sad32x16_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_sad32x16 = vpx_sad32x16_neon_dotprod;
    vpx_sad32x16_avg = vpx_sad32x16_avg_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_sad32x16_avg = vpx_sad32x16_avg_neon_dotprod;
    vpx_sad32x16x4d = vpx_sad32x16x4d_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_sad32x16x4d = vpx_sad32x16x4d_neon_dotprod;
    vpx_sad32x32 = vpx_sad32x32_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_sad32x32 = vpx_sad32x32_neon_dotprod;
    vpx_sad32x32_avg = vpx_sad32x32_avg_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_sad32x32_avg = vpx_sad32x32_avg_neon_dotprod;
    vpx_sad32x32x4d = vpx_sad32x32x4d_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_sad32x32x4d = vpx_sad32x32x4d_neon_dotprod;
    vpx_sad32x64 = vpx_sad32x64_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_sad32x64 = vpx_sad32x64_neon_dotprod;
    vpx_sad32x64_avg = vpx_sad32x64_avg_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_sad32x64_avg = vpx_sad32x64_avg_neon_dotprod;
    vpx_sad32x64x4d = vpx_sad32x64x4d_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_sad32x64x4d = vpx_sad32x64x4d_neon_dotprod;
    vpx_sad64x32 = vpx_sad64x32_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_sad64x32 = vpx_sad64x32_neon_dotprod;
    vpx_sad64x32_avg = vpx_sad64x32_avg_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_sad64x32_avg = vpx_sad64x32_avg_neon_dotprod;
    vpx_sad64x32x4d = vpx_sad64x32x4d_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_sad64x32x4d = vpx_sad64x32x4d_neon_dotprod;
    vpx_sad64x64 = vpx_sad64x64_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_sad64x64 = vpx_sad64x64_neon_dotprod;
    vpx_sad64x64_avg = vpx_sad64x64_avg_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_sad64x64_avg = vpx_sad64x64_avg_neon_dotprod;
    vpx_sad64x64x4d = vpx_sad64x64x4d_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_sad64x64x4d = vpx_sad64x64x4d_neon_dotprod;
    vpx_sad_skip_16x16 = vpx_sad_skip_16x16_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_sad_skip_16x16 = vpx_sad_skip_16x16_neon_dotprod;
    vpx_sad_skip_16x16x4d = vpx_sad_skip_16x16x4d_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_sad_skip_16x16x4d = vpx_sad_skip_16x16x4d_neon_dotprod;
    vpx_sad_skip_16x32 = vpx_sad_skip_16x32_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_sad_skip_16x32 = vpx_sad_skip_16x32_neon_dotprod;
    vpx_sad_skip_16x32x4d = vpx_sad_skip_16x32x4d_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_sad_skip_16x32x4d = vpx_sad_skip_16x32x4d_neon_dotprod;
    vpx_sad_skip_16x8 = vpx_sad_skip_16x8_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_sad_skip_16x8 = vpx_sad_skip_16x8_neon_dotprod;
    vpx_sad_skip_16x8x4d = vpx_sad_skip_16x8x4d_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_sad_skip_16x8x4d = vpx_sad_skip_16x8x4d_neon_dotprod;
    vpx_sad_skip_32x16 = vpx_sad_skip_32x16_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_sad_skip_32x16 = vpx_sad_skip_32x16_neon_dotprod;
    vpx_sad_skip_32x16x4d = vpx_sad_skip_32x16x4d_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_sad_skip_32x16x4d = vpx_sad_skip_32x16x4d_neon_dotprod;
    vpx_sad_skip_32x32 = vpx_sad_skip_32x32_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_sad_skip_32x32 = vpx_sad_skip_32x32_neon_dotprod;
    vpx_sad_skip_32x32x4d = vpx_sad_skip_32x32x4d_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_sad_skip_32x32x4d = vpx_sad_skip_32x32x4d_neon_dotprod;
    vpx_sad_skip_32x64 = vpx_sad_skip_32x64_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_sad_skip_32x64 = vpx_sad_skip_32x64_neon_dotprod;
    vpx_sad_skip_32x64x4d = vpx_sad_skip_32x64x4d_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_sad_skip_32x64x4d = vpx_sad_skip_32x64x4d_neon_dotprod;
    vpx_sad_skip_64x32 = vpx_sad_skip_64x32_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_sad_skip_64x32 = vpx_sad_skip_64x32_neon_dotprod;
    vpx_sad_skip_64x32x4d = vpx_sad_skip_64x32x4d_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_sad_skip_64x32x4d = vpx_sad_skip_64x32x4d_neon_dotprod;
    vpx_sad_skip_64x64 = vpx_sad_skip_64x64_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_sad_skip_64x64 = vpx_sad_skip_64x64_neon_dotprod;
    vpx_sad_skip_64x64x4d = vpx_sad_skip_64x64x4d_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_sad_skip_64x64x4d = vpx_sad_skip_64x64x4d_neon_dotprod;
    vpx_sse = vpx_sse_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_sse = vpx_sse_neon_dotprod;
    vpx_sum_squares_2d_i16 = vpx_sum_squares_2d_i16_neon;
    if (flags & HAS_SVE) vpx_sum_squares_2d_i16 = vpx_sum_squares_2d_i16_sve;
    vpx_variance16x16 = vpx_variance16x16_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_variance16x16 = vpx_variance16x16_neon_dotprod;
    vpx_variance16x32 = vpx_variance16x32_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_variance16x32 = vpx_variance16x32_neon_dotprod;
    vpx_variance16x8 = vpx_variance16x8_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_variance16x8 = vpx_variance16x8_neon_dotprod;
    vpx_variance32x16 = vpx_variance32x16_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_variance32x16 = vpx_variance32x16_neon_dotprod;
    vpx_variance32x32 = vpx_variance32x32_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_variance32x32 = vpx_variance32x32_neon_dotprod;
    vpx_variance32x64 = vpx_variance32x64_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_variance32x64 = vpx_variance32x64_neon_dotprod;
    vpx_variance4x4 = vpx_variance4x4_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_variance4x4 = vpx_variance4x4_neon_dotprod;
    vpx_variance4x8 = vpx_variance4x8_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_variance4x8 = vpx_variance4x8_neon_dotprod;
    vpx_variance64x32 = vpx_variance64x32_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_variance64x32 = vpx_variance64x32_neon_dotprod;
    vpx_variance64x64 = vpx_variance64x64_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_variance64x64 = vpx_variance64x64_neon_dotprod;
    vpx_variance8x16 = vpx_variance8x16_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_variance8x16 = vpx_variance8x16_neon_dotprod;
    vpx_variance8x4 = vpx_variance8x4_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_variance8x4 = vpx_variance8x4_neon_dotprod;
    vpx_variance8x8 = vpx_variance8x8_neon;
    if (flags & HAS_NEON_DOTPROD) vpx_variance8x8 = vpx_variance8x8_neon_dotprod;
}
#endif

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // VPX_DSP_RTCD_H_
