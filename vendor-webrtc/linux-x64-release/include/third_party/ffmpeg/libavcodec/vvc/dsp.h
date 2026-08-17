/*
 * VVC DSP
 *
 * Copyright (C) 2021 Nuo Mi
 *
 *
 * This file is part of FFmpeg.
 *
 * FFmpeg is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * FFmpeg is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with FFmpeg; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA
 */

#ifndef AVCODEC_VVC_DSP_H
#define AVCODEC_VVC_DSP_H

#include <stdint.h>
#include <stddef.h>

enum VVCTxType {
    VVC_DCT2,
    VVC_DST7,
    VVC_DCT8,
    VVC_N_TX_TYPE,
};

enum VVCTxSize {
    VVC_TX_SIZE_2,
    VVC_TX_SIZE_4,
    VVC_TX_SIZE_8,
    VVC_TX_SIZE_16,
    VVC_TX_SIZE_32,
    VVC_TX_SIZE_64,
    VVC_N_TX_SIZE,
};

typedef struct VVCInterDSPContext {
    void (*put[2 /* luma, chroma */][7 /* log2(width) - 1 */][2 /* int, frac */][2 /* int, frac */])(
        int16_t *dst, const uint8_t *src, ptrdiff_t src_stride, int height,
        const int8_t *hf, const int8_t *vf, int width);

    void (*put_uni[2 /* luma, chroma */][7 /* log2(width) - 1 */][2 /* int, frac */][2 /* int, frac */])(
        uint8_t *dst, ptrdiff_t dst_stride, const uint8_t *src, ptrdiff_t src_stride, int height,
        const int8_t *hf, const int8_t *vf, int width);

    void (*put_uni_w[2 /* luma, chroma */][7 /* log2(width) - 1 */][2 /* int, frac */][2 /* int, frac */])(
        uint8_t *dst, ptrdiff_t dst_stride, const uint8_t *src, ptrdiff_t src_stride, int height,
        int denom, int wx, int ox, const int8_t *hf, const int8_t *vf, int width);

    void (*put_scaled[2 /* luma, chroma */][7 /* log2(width) - 1 */])(
        int16_t *dst, const uint8_t *src, ptrdiff_t src_stride, int src_height,
        int x, int y, int dx, int dy, int height, const int8_t *hf, const int8_t *vf, int width);

    void (*put_uni_scaled[2 /* luma, chroma */][7 /* log2(width) - 1 */])(
        uint8_t *dst, const ptrdiff_t dst_stride, const uint8_t *src, ptrdiff_t src_stride, int src_height,
        int x, int y, int dx, int dy, int height, const int8_t *hf, const int8_t *vf, int width);

    void (*put_uni_w_scaled[2 /* luma, chroma */][7 /* log2(width) - 1 */])(
        uint8_t *dst, const ptrdiff_t dst_stride, const uint8_t *src, ptrdiff_t src_stride, int src_height,
        int x, int y, int dx, int dy, int height, int denom, int wx, int ox, const int8_t *hf, const int8_t *vf,
        int width);

    void (*avg)(uint8_t *dst, ptrdiff_t dst_stride,
        const int16_t *src0, const int16_t *src1, int width, int height);

    void (*w_avg)(uint8_t *_dst, const ptrdiff_t _dst_stride,
        const int16_t *src0, const int16_t *src1, int width, int height,
        int denom, int w0, int w1, int o0, int o1);

    void (*put_ciip)(uint8_t *dst, ptrdiff_t dst_stride, int width, int height,
        const uint8_t *inter, ptrdiff_t inter_stride, int inter_weight);

    void (*put_gpm)(uint8_t *dst, ptrdiff_t dst_stride, int width, int height,
        const int16_t *src0, const int16_t *src1,
        const uint8_t *weights, int step_x, int step_y);

    void (*fetch_samples)(int16_t *dst, const uint8_t *src, ptrdiff_t src_stride, int x_frac, int y_frac);
    void (*bdof_fetch_samples)(int16_t *dst, const uint8_t *src, ptrdiff_t src_stride, int x_frac, int y_frac,
        int width, int height);

    void (*apply_prof)(int16_t *dst, const int16_t *src, const int16_t *diff_mv_x, const int16_t *diff_mv_y);

    void (*apply_prof_uni)(uint8_t *dst, ptrdiff_t dst_stride, const int16_t *src,
        const int16_t *diff_mv_x, const int16_t *diff_mv_y);
    void (*apply_prof_uni_w)(uint8_t *dst, const ptrdiff_t dst_stride, const int16_t *src,
        const int16_t *diff_mv_x, const int16_t *diff_mv_y, int denom, int wx, int ox);

    void (*apply_bdof)(uint8_t *dst, ptrdiff_t dst_stride, const int16_t *src0, const int16_t *src1, int block_w, int block_h);

    int (*sad)(const int16_t *src0, const int16_t *src1, int dx, int dy, int block_w, int block_h);
    void (*dmvr[2][2])(int16_t *dst, const uint8_t *src, ptrdiff_t src_stride, int height,
        intptr_t mx, intptr_t my, int width);
} VVCInterDSPContext;

struct VVCLocalContext;

typedef struct VVCIntraDSPContext {
    void (*intra_cclm_pred)(const struct VVCLocalContext *lc, int x0, int y0, int w, int h);
    void (*lmcs_scale_chroma)(struct VVCLocalContext *lc, int *dst, const int *coeff, int w, int h, int x0_cu, int y0_cu);
    void (*intra_pred)(const struct VVCLocalContext *lc, int x0, int y0, int w, int h, int c_idx);
    void (*pred_planar)(uint8_t *src, const uint8_t *top, const uint8_t *left, int w, int h, ptrdiff_t stride);
    void (*pred_mip)(uint8_t *src, const uint8_t *top, const uint8_t *left, int w, int h, ptrdiff_t stride,
        int mode_id, int is_transpose);
    void (*pred_dc)(uint8_t *src, const uint8_t *top, const uint8_t *left, int w, int h, ptrdiff_t stride);
    void (*pred_v)(uint8_t *src, const uint8_t *_top, int w, int h, ptrdiff_t stride);
    void (*pred_h)(uint8_t *src, const uint8_t *_left, int w, int h, ptrdiff_t stride);
    void (*pred_angular_v)(uint8_t *src, const uint8_t *_top, const uint8_t *_left,
        int w, int h, ptrdiff_t stride, int c_idx, int mode, int ref_idx, int filter_flag, int need_pdpc);
    void (*pred_angular_h)(uint8_t *src, const uint8_t *_top, const uint8_t *_left, int w, int h, ptrdiff_t stride,
        int c_idx, int mode, int ref_idx, int filter_flag, int need_pdpc);
} VVCIntraDSPContext;

typedef struct VVCItxDSPContext {
    void (*add_residual)(uint8_t *dst, const int *res, int width, int height, ptrdiff_t stride);
    void (*add_residual_joint)(uint8_t *dst, const int *res, int width, int height, ptrdiff_t stride, int c_sign, int shift);
    void (*pred_residual_joint)(int *buf, int width, int height, int c_sign, int shift);

    void (*itx[VVC_N_TX_TYPE][VVC_N_TX_SIZE])(int *coeffs, ptrdiff_t step, size_t nz);
    void (*transform_bdpcm)(int *coeffs, int width, int height, int vertical, int log2_transform_range);
} VVCItxDSPContext;

typedef struct VVCLMCSDSPContext {
    void (*filter)(uint8_t *dst, ptrdiff_t dst_stride, int width, int height, const void *lut);
} VVCLMCSDSPContext;

typedef struct VVCLFDSPContext {
    int (*ladf_level[2 /* h, v */])(const uint8_t *pix, ptrdiff_t stride);

    void (*filter_luma[2 /* h, v */])(uint8_t *pix, ptrdiff_t stride, const int32_t *beta, const int32_t *tc,
        const uint8_t *no_p, const uint8_t *no_q, const uint8_t *max_len_p, const uint8_t *max_len_q, int hor_ctu_edge);
    void (*filter_chroma[2 /* h, v */])(uint8_t *pix, ptrdiff_t stride, const int32_t *beta, const int32_t *tc,
        const uint8_t *no_p, const uint8_t *no_q, const uint8_t *max_len_p, const uint8_t *max_len_q, int shift);
} VVCLFDSPContext;

struct SAOParams;
typedef struct VVCSAODSPContext {
    void (*band_filter[9])(uint8_t *dst, const uint8_t *src, ptrdiff_t dst_stride, ptrdiff_t src_stride,
        const int16_t *sao_offset_val, int sao_left_class, int width, int height);
    /* implicit src_stride parameter has value of 2 * MAX_PB_SIZE + AV_INPUT_BUFFER_PADDING_SIZE */
    void (*edge_filter[9])(uint8_t *dst /* align 16 */, const uint8_t *src /* align 32 */, ptrdiff_t dst_stride,
        const int16_t *sao_offset_val, int sao_eo_class, int width, int height);
    void (*edge_restore[2])(uint8_t *dst, const uint8_t *src, ptrdiff_t dst_stride, ptrdiff_t src_stride,
        const struct SAOParams *sao, const int *borders, int width, int height, int c_idx,
        const uint8_t *vert_edge, const uint8_t *horiz_edge, const uint8_t *diag_edge);
} VVCSAODSPContext;

typedef struct VVCALFDSPContext {
    void (*filter[2 /* luma, chroma */])(uint8_t *dst, ptrdiff_t dst_stride, const uint8_t *src,  ptrdiff_t src_stride,
        int width, int height, const int16_t *filter, const int16_t *clip, int vb_pos);
    void (*filter_cc)(uint8_t *dst, ptrdiff_t dst_stride, const uint8_t *luma, ptrdiff_t luma_stride,
        int width, int height, int hs, int vs, const int16_t *filter, int vb_pos);

    void (*classify)(int *class_idx, int *transpose_idx, const uint8_t *src, ptrdiff_t src_stride, int width, int height,
        int vb_pos, int *gradient_tmp);
    void (*recon_coeff_and_clip)(int16_t *coeff, int16_t *clip, const int *class_idx, const int *transpose_idx, int size,
        const int16_t *coeff_set, const uint8_t *clip_idx_set, const uint8_t *class_to_filt);
} VVCALFDSPContext;

typedef struct VVCDSPContext {
    VVCInterDSPContext inter;
    VVCIntraDSPContext intra;
    VVCItxDSPContext itx;
    VVCLMCSDSPContext lmcs;
    VVCLFDSPContext lf;
    VVCSAODSPContext sao;
    VVCALFDSPContext alf;
} VVCDSPContext;

void ff_vvc_dsp_init(VVCDSPContext *hpc, int bit_depth);

void ff_vvc_dsp_init_aarch64(VVCDSPContext *hpc, const int bit_depth);
void ff_vvc_dsp_init_riscv(VVCDSPContext *hpc, const int bit_depth);
void ff_vvc_dsp_init_x86(VVCDSPContext *hpc, const int bit_depth);

#endif /* AVCODEC_VVC_DSP_H */
