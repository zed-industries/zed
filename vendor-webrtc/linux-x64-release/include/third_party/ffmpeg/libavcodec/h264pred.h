/*
 * H.26L/H.264/AVC/JVT/14496-10/... encoder/decoder
 * Copyright (c) 2003 Michael Niedermayer <michaelni@gmx.at>
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

/**
 * @file
 * H.264 / AVC / MPEG-4 prediction functions.
 * @author Michael Niedermayer <michaelni@gmx.at>
 */

#ifndef AVCODEC_H264PRED_H
#define AVCODEC_H264PRED_H

#include <stddef.h>
#include <stdint.h>

/**
 * Prediction types
 */
//@{
#define VERT_PRED              0
#define HOR_PRED               1
#define DC_PRED                2
#define DIAG_DOWN_LEFT_PRED    3
#define DIAG_DOWN_RIGHT_PRED   4
#define VERT_RIGHT_PRED        5
#define HOR_DOWN_PRED          6
#define VERT_LEFT_PRED         7
#define HOR_UP_PRED            8

// DC edge (not for VP8)
#define LEFT_DC_PRED           9
#define TOP_DC_PRED           10
#define DC_128_PRED           11

// RV40 specific
#define DIAG_DOWN_LEFT_PRED_RV40_NODOWN   12
#define HOR_UP_PRED_RV40_NODOWN           13
#define VERT_LEFT_PRED_RV40_NODOWN        14

// VP8 specific
#define TM_VP8_PRED            9    ///< "True Motion", used instead of plane
#define VERT_VP8_PRED         10    ///< for VP8, #VERT_PRED is the average of
                                    ///< (left col+cur col x2+right col) / 4;
                                    ///< this is the "unaveraged" one
#define HOR_VP8_PRED          14    ///< unaveraged version of #HOR_PRED, see
                                    ///< #VERT_VP8_PRED for details
#define DC_127_PRED           12
#define DC_129_PRED           13

#define DC_PRED8x8             0
#define HOR_PRED8x8            1
#define VERT_PRED8x8           2
#define PLANE_PRED8x8          3

// DC edge
#define LEFT_DC_PRED8x8        4
#define TOP_DC_PRED8x8         5
#define DC_128_PRED8x8         6

// H.264/SVQ3 (8x8) specific
#define ALZHEIMER_DC_L0T_PRED8x8  7
#define ALZHEIMER_DC_0LT_PRED8x8  8
#define ALZHEIMER_DC_L00_PRED8x8  9
#define ALZHEIMER_DC_0L0_PRED8x8 10

// VP8 specific
#define DC_127_PRED8x8         7
#define DC_129_PRED8x8         8
//@}

#define PART_NOT_AVAILABLE -2

/**
 * Context for storing H.264 prediction functions
 */
typedef struct H264PredContext {
    void(*pred4x4[9 + 3 + 3])(uint8_t *src, const uint8_t *topright,
                              ptrdiff_t stride);
    void(*pred8x8l[9 + 3])(uint8_t *src, int topleft, int topright,
                           ptrdiff_t stride);
    void(*pred8x8[4 + 3 + 4])(uint8_t *src, ptrdiff_t stride);
    void(*pred16x16[4 + 3 + 2])(uint8_t *src, ptrdiff_t stride);

    void(*pred4x4_add[2])(uint8_t *pix /*align  4*/,
                          int16_t *block /*align 16*/, ptrdiff_t stride);
    void(*pred8x8l_add[2])(uint8_t *pix /*align  8*/,
                           int16_t *block /*align 16*/, ptrdiff_t stride);
    void(*pred8x8l_filter_add[2])(uint8_t *pix /*align  8*/,
                           int16_t *block /*align 16*/, int topleft, int topright, ptrdiff_t stride);
    void(*pred8x8_add[3])(uint8_t *pix /*align  8*/,
                          const int *block_offset,
                          int16_t *block /*align 16*/, ptrdiff_t stride);
    void(*pred16x16_add[3])(uint8_t *pix /*align 16*/,
                            const int *block_offset,
                            int16_t *block /*align 16*/, ptrdiff_t stride);
} H264PredContext;

void ff_h264_pred_init(H264PredContext *h, int codec_id,
                       const int bit_depth, const int chroma_format_idc);
void ff_h264_pred_init_aarch64(H264PredContext *h, int codec_id,
                               const int bit_depth,
                               const int chroma_format_idc);
void ff_h264_pred_init_arm(H264PredContext *h, int codec_id,
                           const int bit_depth, const int chroma_format_idc);
void ff_h264_pred_init_x86(H264PredContext *h, int codec_id,
                           const int bit_depth, const int chroma_format_idc);
void ff_h264_pred_init_mips(H264PredContext *h, int codec_id,
                            const int bit_depth, const int chroma_format_idc);
void ff_h264_pred_init_loongarch(H264PredContext *h, int codec_id,
                                 const int bit_depth, const int chroma_format_idc);

#endif /* AVCODEC_H264PRED_H */
