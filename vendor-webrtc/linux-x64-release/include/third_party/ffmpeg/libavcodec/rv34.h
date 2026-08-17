/*
 * RV30/40 decoder common data declarations
 * Copyright (c) 2007 Mike Melanson, Konstantin Shishkov
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
 * RV30 and RV40 decoder common data declarations
 */

#ifndef AVCODEC_RV34_H
#define AVCODEC_RV34_H

#include "libavutil/mem_internal.h"

#include "avcodec.h"
#include "mpegvideo.h"

#include "h264pred.h"
#include "rv34dsp.h"

#define MB_TYPE_SEPARATE_DC 0x01000000
#define IS_SEPARATE_DC(a)   ((a) & MB_TYPE_SEPARATE_DC)

/**
 * RV30 and RV40 Macroblock types
 */
enum RV40BlockTypes{
    RV34_MB_TYPE_INTRA,      ///< Intra macroblock
    RV34_MB_TYPE_INTRA16x16, ///< Intra macroblock with DCs in a separate 4x4 block
    RV34_MB_P_16x16,         ///< P-frame macroblock, one motion frame
    RV34_MB_P_8x8,           ///< P-frame macroblock, 8x8 motion compensation partitions
    RV34_MB_B_FORWARD,       ///< B-frame macroblock, forward prediction
    RV34_MB_B_BACKWARD,      ///< B-frame macroblock, backward prediction
    RV34_MB_SKIP,            ///< Skipped block
    RV34_MB_B_DIRECT,        ///< Bidirectionally predicted B-frame macroblock, no motion vectors
    RV34_MB_P_16x8,          ///< P-frame macroblock, 16x8 motion compensation partitions
    RV34_MB_P_8x16,          ///< P-frame macroblock, 8x16 motion compensation partitions
    RV34_MB_B_BIDIR,         ///< Bidirectionally predicted B-frame macroblock, two motion vectors
    RV34_MB_P_MIX16x16,      ///< P-frame macroblock with DCs in a separate 4x4 block, one motion vector
    RV34_MB_TYPES
};

/**
 * VLC tables used by the decoder
 *
 * Intra frame VLC sets do not contain some of those tables.
 */
typedef struct RV34VLC{
    const VLCElem *cbppattern[2];     ///< VLCs used for pattern of coded block patterns decoding
    VLC cbp[2][4];                    ///< VLCs used for coded block patterns decoding
    const VLCElem *first_pattern[4];  ///< VLCs used for decoding coefficients in the first subblock
    const VLCElem *second_pattern[2]; ///< VLCs used for decoding coefficients in the subblocks 2 and 3
    const VLCElem *third_pattern[2];  ///< VLCs used for decoding coefficients in the last subblock
    const VLCElem *coefficient;       ///< VLCs used for decoding big coefficients
}RV34VLC;

/** essential slice information */
typedef struct SliceInfo{
    int type;              ///< slice type (intra, inter)
    int quant;             ///< quantizer used for this slice
    int vlc_set;           ///< VLCs used for this slice
    int start, end;        ///< start and end macroblocks of the slice
    int width;             ///< coded width
    int height;            ///< coded height
    int pts;               ///< frame timestamp
}SliceInfo;

/** decoder context */
typedef struct RV34DecContext{
    MpegEncContext s;
    RV34DSPContext rdsp;
    int8_t *intra_types_hist;///< old block types, used for prediction
    int8_t *intra_types;     ///< block types
    int    intra_types_stride;///< block types array stride
    const uint8_t *luma_dc_quant_i;///< luma subblock DC quantizer for intraframes
    const uint8_t *luma_dc_quant_p;///< luma subblock DC quantizer for interframes

    const RV34VLC *cur_vlcs; ///< VLC set used for current frame decoding
    H264PredContext h;       ///< functions for 4x4 and 16x16 intra block prediction
    SliceInfo si;            ///< current slice information

    int *mb_type;            ///< internal macroblock types
    int block_type;          ///< current block type
    int luma_vlc;            ///< which VLC set will be used for decoding of luma blocks
    int chroma_vlc;          ///< which VLC set will be used for decoding of chroma blocks
    int is16;                ///< current block has additional 16x16 specific features or not
    int dmv[4][2];           ///< differential motion vectors for the current macroblock

    int rv30;                ///< indicates which RV variant is currently decoded
    int max_rpr;

    int cur_pts, last_pts, next_pts;
    int scaled_weight;
    int weight1, weight2;    ///< B-frame distance fractions (0.14) used in motion compensation
    int mv_weight1, mv_weight2;

    int orig_width, orig_height;

    uint16_t *cbp_luma;      ///< CBP values for luma subblocks
    uint8_t  *cbp_chroma;    ///< CBP values for chroma subblocks
    uint16_t *deblock_coefs; ///< deblock coefficients for each macroblock

    /** 8x8 block available flags (for MV prediction) */
    DECLARE_ALIGNED(8, uint32_t, avail_cache)[3*4];

    /** temporary blocks for RV4 weighted MC */
    uint8_t *tmp_b_block_y[2];
    uint8_t *tmp_b_block_uv[4];
    uint8_t *tmp_b_block_base;

    int (*parse_slice_header)(struct RV34DecContext *r, GetBitContext *gb, SliceInfo *si);
    int (*decode_mb_info)(struct RV34DecContext *r);
    int (*decode_intra_types)(struct RV34DecContext *r, GetBitContext *gb, int8_t *dst);
    void (*loop_filter)(struct RV34DecContext *r, int row);
}RV34DecContext;

/**
 * common decoding functions
 */
int ff_rv34_get_start_offset(GetBitContext *gb, int blocks);
int ff_rv34_decode_init(AVCodecContext *avctx);
int ff_rv34_decode_frame(AVCodecContext *avctx, AVFrame *frame,
                         int *got_frame, AVPacket *avpkt);
int ff_rv34_decode_end(AVCodecContext *avctx);
int ff_rv34_decode_update_thread_context(AVCodecContext *dst, const AVCodecContext *src);

#endif /* AVCODEC_RV34_H */
