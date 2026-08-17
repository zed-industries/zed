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
#ifndef VP8_RTCD_H_
#define VP8_RTCD_H_

#ifdef RTCD_C
#define RTCD_EXTERN
#else
#define RTCD_EXTERN extern
#endif

/*
 * VP8
 */

struct blockd;
struct macroblockd;
struct loop_filter_info;

/* Encoder forward decls */
struct block;
struct macroblock;
struct variance_vtable;
union int_mv;
struct yv12_buffer_config;

#ifdef __cplusplus
extern "C" {
#endif

void vp8_bilinear_predict16x16_c(unsigned char *src_ptr, int src_pixels_per_line, int xoffset, int yoffset, unsigned char *dst_ptr, int dst_pitch);
void vp8_bilinear_predict16x16_neon(unsigned char *src_ptr, int src_pixels_per_line, int xoffset, int yoffset, unsigned char *dst_ptr, int dst_pitch);
RTCD_EXTERN void (*vp8_bilinear_predict16x16)(unsigned char *src_ptr, int src_pixels_per_line, int xoffset, int yoffset, unsigned char *dst_ptr, int dst_pitch);

void vp8_bilinear_predict4x4_c(unsigned char *src_ptr, int src_pixels_per_line, int xoffset, int yoffset, unsigned char *dst_ptr, int dst_pitch);
void vp8_bilinear_predict4x4_neon(unsigned char *src_ptr, int src_pixels_per_line, int xoffset, int yoffset, unsigned char *dst_ptr, int dst_pitch);
RTCD_EXTERN void (*vp8_bilinear_predict4x4)(unsigned char *src_ptr, int src_pixels_per_line, int xoffset, int yoffset, unsigned char *dst_ptr, int dst_pitch);

void vp8_bilinear_predict8x4_c(unsigned char *src_ptr, int src_pixels_per_line, int xoffset, int yoffset, unsigned char *dst_ptr, int dst_pitch);
void vp8_bilinear_predict8x4_neon(unsigned char *src_ptr, int src_pixels_per_line, int xoffset, int yoffset, unsigned char *dst_ptr, int dst_pitch);
RTCD_EXTERN void (*vp8_bilinear_predict8x4)(unsigned char *src_ptr, int src_pixels_per_line, int xoffset, int yoffset, unsigned char *dst_ptr, int dst_pitch);

void vp8_bilinear_predict8x8_c(unsigned char *src_ptr, int src_pixels_per_line, int xoffset, int yoffset, unsigned char *dst_ptr, int dst_pitch);
void vp8_bilinear_predict8x8_neon(unsigned char *src_ptr, int src_pixels_per_line, int xoffset, int yoffset, unsigned char *dst_ptr, int dst_pitch);
RTCD_EXTERN void (*vp8_bilinear_predict8x8)(unsigned char *src_ptr, int src_pixels_per_line, int xoffset, int yoffset, unsigned char *dst_ptr, int dst_pitch);

int vp8_block_error_c(short *coeff, short *dqcoeff);
#define vp8_block_error vp8_block_error_c

void vp8_copy32xn_c(const unsigned char *src_ptr, int src_stride, unsigned char *dst_ptr, int dst_stride, int height);
#define vp8_copy32xn vp8_copy32xn_c

void vp8_copy_mem16x16_c(unsigned char *src, int src_stride, unsigned char *dst, int dst_stride);
void vp8_copy_mem16x16_neon(unsigned char *src, int src_stride, unsigned char *dst, int dst_stride);
RTCD_EXTERN void (*vp8_copy_mem16x16)(unsigned char *src, int src_stride, unsigned char *dst, int dst_stride);

void vp8_copy_mem8x4_c(unsigned char *src, int src_stride, unsigned char *dst, int dst_stride);
void vp8_copy_mem8x4_neon(unsigned char *src, int src_stride, unsigned char *dst, int dst_stride);
RTCD_EXTERN void (*vp8_copy_mem8x4)(unsigned char *src, int src_stride, unsigned char *dst, int dst_stride);

void vp8_copy_mem8x8_c(unsigned char *src, int src_stride, unsigned char *dst, int dst_stride);
void vp8_copy_mem8x8_neon(unsigned char *src, int src_stride, unsigned char *dst, int dst_stride);
RTCD_EXTERN void (*vp8_copy_mem8x8)(unsigned char *src, int src_stride, unsigned char *dst, int dst_stride);

void vp8_dc_only_idct_add_c(short input_dc, unsigned char *pred_ptr, int pred_stride, unsigned char *dst_ptr, int dst_stride);
void vp8_dc_only_idct_add_neon(short input_dc, unsigned char *pred_ptr, int pred_stride, unsigned char *dst_ptr, int dst_stride);
RTCD_EXTERN void (*vp8_dc_only_idct_add)(short input_dc, unsigned char *pred_ptr, int pred_stride, unsigned char *dst_ptr, int dst_stride);

int vp8_denoiser_filter_c(unsigned char *mc_running_avg_y, int mc_avg_y_stride, unsigned char *running_avg_y, int avg_y_stride, unsigned char *sig, int sig_stride, unsigned int motion_magnitude, int increase_denoising);
int vp8_denoiser_filter_neon(unsigned char *mc_running_avg_y, int mc_avg_y_stride, unsigned char *running_avg_y, int avg_y_stride, unsigned char *sig, int sig_stride, unsigned int motion_magnitude, int increase_denoising);
RTCD_EXTERN int (*vp8_denoiser_filter)(unsigned char *mc_running_avg_y, int mc_avg_y_stride, unsigned char *running_avg_y, int avg_y_stride, unsigned char *sig, int sig_stride, unsigned int motion_magnitude, int increase_denoising);

int vp8_denoiser_filter_uv_c(unsigned char *mc_running_avg, int mc_avg_stride, unsigned char *running_avg, int avg_stride, unsigned char *sig, int sig_stride, unsigned int motion_magnitude, int increase_denoising);
int vp8_denoiser_filter_uv_neon(unsigned char *mc_running_avg, int mc_avg_stride, unsigned char *running_avg, int avg_stride, unsigned char *sig, int sig_stride, unsigned int motion_magnitude, int increase_denoising);
RTCD_EXTERN int (*vp8_denoiser_filter_uv)(unsigned char *mc_running_avg, int mc_avg_stride, unsigned char *running_avg, int avg_stride, unsigned char *sig, int sig_stride, unsigned int motion_magnitude, int increase_denoising);

void vp8_dequant_idct_add_c(short *input, short *dq, unsigned char *dest, int stride);
void vp8_dequant_idct_add_neon(short *input, short *dq, unsigned char *dest, int stride);
RTCD_EXTERN void (*vp8_dequant_idct_add)(short *input, short *dq, unsigned char *dest, int stride);

void vp8_dequant_idct_add_uv_block_c(short *q, short *dq, unsigned char *dst_u, unsigned char *dst_v, int stride, char *eobs);
void vp8_dequant_idct_add_uv_block_neon(short *q, short *dq, unsigned char *dst_u, unsigned char *dst_v, int stride, char *eobs);
RTCD_EXTERN void (*vp8_dequant_idct_add_uv_block)(short *q, short *dq, unsigned char *dst_u, unsigned char *dst_v, int stride, char *eobs);

void vp8_dequant_idct_add_y_block_c(short *q, short *dq, unsigned char *dst, int stride, char *eobs);
void vp8_dequant_idct_add_y_block_neon(short *q, short *dq, unsigned char *dst, int stride, char *eobs);
RTCD_EXTERN void (*vp8_dequant_idct_add_y_block)(short *q, short *dq, unsigned char *dst, int stride, char *eobs);

void vp8_dequantize_b_c(struct blockd*, short *DQC);
void vp8_dequantize_b_neon(struct blockd*, short *DQC);
RTCD_EXTERN void (*vp8_dequantize_b)(struct blockd*, short *DQC);

int vp8_diamond_search_sad_c(struct macroblock *x, struct block *b, struct blockd *d, union int_mv *ref_mv, union int_mv *best_mv, int search_param, int sad_per_bit, int *num00, struct variance_vtable *fn_ptr, int *mvcost[2], union int_mv *center_mv);
#define vp8_diamond_search_sad vp8_diamond_search_sad_c

void vp8_fast_quantize_b_c(struct block *, struct blockd *);
void vp8_fast_quantize_b_neon(struct block *, struct blockd *);
RTCD_EXTERN void (*vp8_fast_quantize_b)(struct block *, struct blockd *);

void vp8_filter_by_weight16x16_c(unsigned char *src, int src_stride, unsigned char *dst, int dst_stride, int src_weight);
#define vp8_filter_by_weight16x16 vp8_filter_by_weight16x16_c

void vp8_filter_by_weight4x4_c(unsigned char *src, int src_stride, unsigned char *dst, int dst_stride, int src_weight);
#define vp8_filter_by_weight4x4 vp8_filter_by_weight4x4_c

void vp8_filter_by_weight8x8_c(unsigned char *src, int src_stride, unsigned char *dst, int dst_stride, int src_weight);
#define vp8_filter_by_weight8x8 vp8_filter_by_weight8x8_c

void vp8_loop_filter_bh_c(unsigned char *y_ptr, unsigned char *u_ptr, unsigned char *v_ptr, int y_stride, int uv_stride, struct loop_filter_info *lfi);
void vp8_loop_filter_bh_neon(unsigned char *y_ptr, unsigned char *u_ptr, unsigned char *v_ptr, int y_stride, int uv_stride, struct loop_filter_info *lfi);
RTCD_EXTERN void (*vp8_loop_filter_bh)(unsigned char *y_ptr, unsigned char *u_ptr, unsigned char *v_ptr, int y_stride, int uv_stride, struct loop_filter_info *lfi);

void vp8_loop_filter_bv_c(unsigned char *y_ptr, unsigned char *u_ptr, unsigned char *v_ptr, int y_stride, int uv_stride, struct loop_filter_info *lfi);
void vp8_loop_filter_bv_neon(unsigned char *y_ptr, unsigned char *u_ptr, unsigned char *v_ptr, int y_stride, int uv_stride, struct loop_filter_info *lfi);
RTCD_EXTERN void (*vp8_loop_filter_bv)(unsigned char *y_ptr, unsigned char *u_ptr, unsigned char *v_ptr, int y_stride, int uv_stride, struct loop_filter_info *lfi);

void vp8_loop_filter_mbh_c(unsigned char *y_ptr, unsigned char *u_ptr, unsigned char *v_ptr, int y_stride, int uv_stride, struct loop_filter_info *lfi);
void vp8_loop_filter_mbh_neon(unsigned char *y_ptr, unsigned char *u_ptr, unsigned char *v_ptr, int y_stride, int uv_stride, struct loop_filter_info *lfi);
RTCD_EXTERN void (*vp8_loop_filter_mbh)(unsigned char *y_ptr, unsigned char *u_ptr, unsigned char *v_ptr, int y_stride, int uv_stride, struct loop_filter_info *lfi);

void vp8_loop_filter_mbv_c(unsigned char *y_ptr, unsigned char *u_ptr, unsigned char *v_ptr, int y_stride, int uv_stride, struct loop_filter_info *lfi);
void vp8_loop_filter_mbv_neon(unsigned char *y_ptr, unsigned char *u_ptr, unsigned char *v_ptr, int y_stride, int uv_stride, struct loop_filter_info *lfi);
RTCD_EXTERN void (*vp8_loop_filter_mbv)(unsigned char *y_ptr, unsigned char *u_ptr, unsigned char *v_ptr, int y_stride, int uv_stride, struct loop_filter_info *lfi);

void vp8_loop_filter_bhs_c(unsigned char *y_ptr, int y_stride, const unsigned char *blimit);
void vp8_loop_filter_bhs_neon(unsigned char *y_ptr, int y_stride, const unsigned char *blimit);
RTCD_EXTERN void (*vp8_loop_filter_simple_bh)(unsigned char *y_ptr, int y_stride, const unsigned char *blimit);

void vp8_loop_filter_bvs_c(unsigned char *y_ptr, int y_stride, const unsigned char *blimit);
void vp8_loop_filter_bvs_neon(unsigned char *y_ptr, int y_stride, const unsigned char *blimit);
RTCD_EXTERN void (*vp8_loop_filter_simple_bv)(unsigned char *y_ptr, int y_stride, const unsigned char *blimit);

void vp8_loop_filter_simple_horizontal_edge_c(unsigned char *y_ptr, int y_stride, const unsigned char *blimit);
void vp8_loop_filter_mbhs_neon(unsigned char *y_ptr, int y_stride, const unsigned char *blimit);
RTCD_EXTERN void (*vp8_loop_filter_simple_mbh)(unsigned char *y_ptr, int y_stride, const unsigned char *blimit);

void vp8_loop_filter_simple_vertical_edge_c(unsigned char *y_ptr, int y_stride, const unsigned char *blimit);
void vp8_loop_filter_mbvs_neon(unsigned char *y_ptr, int y_stride, const unsigned char *blimit);
RTCD_EXTERN void (*vp8_loop_filter_simple_mbv)(unsigned char *y_ptr, int y_stride, const unsigned char *blimit);

int vp8_mbblock_error_c(struct macroblock *mb, int dc);
#define vp8_mbblock_error vp8_mbblock_error_c

int vp8_mbuverror_c(struct macroblock *mb);
#define vp8_mbuverror vp8_mbuverror_c

int vp8_refining_search_sad_c(struct macroblock *x, struct block *b, struct blockd *d, union int_mv *ref_mv, int error_per_bit, int search_range, struct variance_vtable *fn_ptr, int *mvcost[2], union int_mv *center_mv);
#define vp8_refining_search_sad vp8_refining_search_sad_c

void vp8_regular_quantize_b_c(struct block *, struct blockd *);
#define vp8_regular_quantize_b vp8_regular_quantize_b_c

void vp8_short_fdct4x4_c(short *input, short *output, int pitch);
void vp8_short_fdct4x4_neon(short *input, short *output, int pitch);
RTCD_EXTERN void (*vp8_short_fdct4x4)(short *input, short *output, int pitch);

void vp8_short_fdct8x4_c(short *input, short *output, int pitch);
void vp8_short_fdct8x4_neon(short *input, short *output, int pitch);
RTCD_EXTERN void (*vp8_short_fdct8x4)(short *input, short *output, int pitch);

void vp8_short_idct4x4llm_c(short *input, unsigned char *pred_ptr, int pred_stride, unsigned char *dst_ptr, int dst_stride);
void vp8_short_idct4x4llm_neon(short *input, unsigned char *pred_ptr, int pred_stride, unsigned char *dst_ptr, int dst_stride);
RTCD_EXTERN void (*vp8_short_idct4x4llm)(short *input, unsigned char *pred_ptr, int pred_stride, unsigned char *dst_ptr, int dst_stride);

void vp8_short_inv_walsh4x4_c(short *input, short *mb_dqcoeff);
void vp8_short_inv_walsh4x4_neon(short *input, short *mb_dqcoeff);
RTCD_EXTERN void (*vp8_short_inv_walsh4x4)(short *input, short *mb_dqcoeff);

void vp8_short_inv_walsh4x4_1_c(short *input, short *mb_dqcoeff);
#define vp8_short_inv_walsh4x4_1 vp8_short_inv_walsh4x4_1_c

void vp8_short_walsh4x4_c(short *input, short *output, int pitch);
void vp8_short_walsh4x4_neon(short *input, short *output, int pitch);
RTCD_EXTERN void (*vp8_short_walsh4x4)(short *input, short *output, int pitch);

void vp8_sixtap_predict16x16_c(unsigned char *src_ptr, int src_pixels_per_line, int xoffset, int yoffset, unsigned char *dst_ptr, int dst_pitch);
void vp8_sixtap_predict16x16_neon(unsigned char *src_ptr, int src_pixels_per_line, int xoffset, int yoffset, unsigned char *dst_ptr, int dst_pitch);
RTCD_EXTERN void (*vp8_sixtap_predict16x16)(unsigned char *src_ptr, int src_pixels_per_line, int xoffset, int yoffset, unsigned char *dst_ptr, int dst_pitch);

void vp8_sixtap_predict4x4_c(unsigned char *src_ptr, int src_pixels_per_line, int xoffset, int yoffset, unsigned char *dst_ptr, int dst_pitch);
void vp8_sixtap_predict4x4_neon(unsigned char *src_ptr, int src_pixels_per_line, int xoffset, int yoffset, unsigned char *dst_ptr, int dst_pitch);
RTCD_EXTERN void (*vp8_sixtap_predict4x4)(unsigned char *src_ptr, int src_pixels_per_line, int xoffset, int yoffset, unsigned char *dst_ptr, int dst_pitch);

void vp8_sixtap_predict8x4_c(unsigned char *src_ptr, int src_pixels_per_line, int xoffset, int yoffset, unsigned char *dst_ptr, int dst_pitch);
void vp8_sixtap_predict8x4_neon(unsigned char *src_ptr, int src_pixels_per_line, int xoffset, int yoffset, unsigned char *dst_ptr, int dst_pitch);
RTCD_EXTERN void (*vp8_sixtap_predict8x4)(unsigned char *src_ptr, int src_pixels_per_line, int xoffset, int yoffset, unsigned char *dst_ptr, int dst_pitch);

void vp8_sixtap_predict8x8_c(unsigned char *src_ptr, int src_pixels_per_line, int xoffset, int yoffset, unsigned char *dst_ptr, int dst_pitch);
void vp8_sixtap_predict8x8_neon(unsigned char *src_ptr, int src_pixels_per_line, int xoffset, int yoffset, unsigned char *dst_ptr, int dst_pitch);
RTCD_EXTERN void (*vp8_sixtap_predict8x8)(unsigned char *src_ptr, int src_pixels_per_line, int xoffset, int yoffset, unsigned char *dst_ptr, int dst_pitch);

void vp8_rtcd(void);

#include "vpx_config.h"

#ifdef RTCD_C
#include "vpx_ports/arm.h"
static void setup_rtcd_internal(void)
{
    int flags = arm_cpu_caps();

    (void)flags;

    vp8_bilinear_predict16x16 = vp8_bilinear_predict16x16_c;
    if (flags & HAS_NEON) vp8_bilinear_predict16x16 = vp8_bilinear_predict16x16_neon;
    vp8_bilinear_predict4x4 = vp8_bilinear_predict4x4_c;
    if (flags & HAS_NEON) vp8_bilinear_predict4x4 = vp8_bilinear_predict4x4_neon;
    vp8_bilinear_predict8x4 = vp8_bilinear_predict8x4_c;
    if (flags & HAS_NEON) vp8_bilinear_predict8x4 = vp8_bilinear_predict8x4_neon;
    vp8_bilinear_predict8x8 = vp8_bilinear_predict8x8_c;
    if (flags & HAS_NEON) vp8_bilinear_predict8x8 = vp8_bilinear_predict8x8_neon;
    vp8_copy_mem16x16 = vp8_copy_mem16x16_c;
    if (flags & HAS_NEON) vp8_copy_mem16x16 = vp8_copy_mem16x16_neon;
    vp8_copy_mem8x4 = vp8_copy_mem8x4_c;
    if (flags & HAS_NEON) vp8_copy_mem8x4 = vp8_copy_mem8x4_neon;
    vp8_copy_mem8x8 = vp8_copy_mem8x8_c;
    if (flags & HAS_NEON) vp8_copy_mem8x8 = vp8_copy_mem8x8_neon;
    vp8_dc_only_idct_add = vp8_dc_only_idct_add_c;
    if (flags & HAS_NEON) vp8_dc_only_idct_add = vp8_dc_only_idct_add_neon;
    vp8_denoiser_filter = vp8_denoiser_filter_c;
    if (flags & HAS_NEON) vp8_denoiser_filter = vp8_denoiser_filter_neon;
    vp8_denoiser_filter_uv = vp8_denoiser_filter_uv_c;
    if (flags & HAS_NEON) vp8_denoiser_filter_uv = vp8_denoiser_filter_uv_neon;
    vp8_dequant_idct_add = vp8_dequant_idct_add_c;
    if (flags & HAS_NEON) vp8_dequant_idct_add = vp8_dequant_idct_add_neon;
    vp8_dequant_idct_add_uv_block = vp8_dequant_idct_add_uv_block_c;
    if (flags & HAS_NEON) vp8_dequant_idct_add_uv_block = vp8_dequant_idct_add_uv_block_neon;
    vp8_dequant_idct_add_y_block = vp8_dequant_idct_add_y_block_c;
    if (flags & HAS_NEON) vp8_dequant_idct_add_y_block = vp8_dequant_idct_add_y_block_neon;
    vp8_dequantize_b = vp8_dequantize_b_c;
    if (flags & HAS_NEON) vp8_dequantize_b = vp8_dequantize_b_neon;
    vp8_fast_quantize_b = vp8_fast_quantize_b_c;
    if (flags & HAS_NEON) vp8_fast_quantize_b = vp8_fast_quantize_b_neon;
    vp8_loop_filter_bh = vp8_loop_filter_bh_c;
    if (flags & HAS_NEON) vp8_loop_filter_bh = vp8_loop_filter_bh_neon;
    vp8_loop_filter_bv = vp8_loop_filter_bv_c;
    if (flags & HAS_NEON) vp8_loop_filter_bv = vp8_loop_filter_bv_neon;
    vp8_loop_filter_mbh = vp8_loop_filter_mbh_c;
    if (flags & HAS_NEON) vp8_loop_filter_mbh = vp8_loop_filter_mbh_neon;
    vp8_loop_filter_mbv = vp8_loop_filter_mbv_c;
    if (flags & HAS_NEON) vp8_loop_filter_mbv = vp8_loop_filter_mbv_neon;
    vp8_loop_filter_simple_bh = vp8_loop_filter_bhs_c;
    if (flags & HAS_NEON) vp8_loop_filter_simple_bh = vp8_loop_filter_bhs_neon;
    vp8_loop_filter_simple_bv = vp8_loop_filter_bvs_c;
    if (flags & HAS_NEON) vp8_loop_filter_simple_bv = vp8_loop_filter_bvs_neon;
    vp8_loop_filter_simple_mbh = vp8_loop_filter_simple_horizontal_edge_c;
    if (flags & HAS_NEON) vp8_loop_filter_simple_mbh = vp8_loop_filter_mbhs_neon;
    vp8_loop_filter_simple_mbv = vp8_loop_filter_simple_vertical_edge_c;
    if (flags & HAS_NEON) vp8_loop_filter_simple_mbv = vp8_loop_filter_mbvs_neon;
    vp8_short_fdct4x4 = vp8_short_fdct4x4_c;
    if (flags & HAS_NEON) vp8_short_fdct4x4 = vp8_short_fdct4x4_neon;
    vp8_short_fdct8x4 = vp8_short_fdct8x4_c;
    if (flags & HAS_NEON) vp8_short_fdct8x4 = vp8_short_fdct8x4_neon;
    vp8_short_idct4x4llm = vp8_short_idct4x4llm_c;
    if (flags & HAS_NEON) vp8_short_idct4x4llm = vp8_short_idct4x4llm_neon;
    vp8_short_inv_walsh4x4 = vp8_short_inv_walsh4x4_c;
    if (flags & HAS_NEON) vp8_short_inv_walsh4x4 = vp8_short_inv_walsh4x4_neon;
    vp8_short_walsh4x4 = vp8_short_walsh4x4_c;
    if (flags & HAS_NEON) vp8_short_walsh4x4 = vp8_short_walsh4x4_neon;
    vp8_sixtap_predict16x16 = vp8_sixtap_predict16x16_c;
    if (flags & HAS_NEON) vp8_sixtap_predict16x16 = vp8_sixtap_predict16x16_neon;
    vp8_sixtap_predict4x4 = vp8_sixtap_predict4x4_c;
    if (flags & HAS_NEON) vp8_sixtap_predict4x4 = vp8_sixtap_predict4x4_neon;
    vp8_sixtap_predict8x4 = vp8_sixtap_predict8x4_c;
    if (flags & HAS_NEON) vp8_sixtap_predict8x4 = vp8_sixtap_predict8x4_neon;
    vp8_sixtap_predict8x8 = vp8_sixtap_predict8x8_c;
    if (flags & HAS_NEON) vp8_sixtap_predict8x8 = vp8_sixtap_predict8x8_neon;
}
#endif

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // VP8_RTCD_H_
