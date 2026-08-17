/*
 * VP8 compatible video decoder
 *
 * Copyright (C) 2010 David Conrad
 * Copyright (C) 2010 Ronald S. Bultje
 * Copyright (C) 2010 Fiona Glaser
 * Copyright (C) 2012 Daniel Kang
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

#ifndef AVCODEC_VP8_H
#define AVCODEC_VP8_H

#include <stdatomic.h>

#include "libavutil/mem_internal.h"
#include "libavutil/thread.h"

#include "avcodec.h"
#include "h264pred.h"
#include "progressframe.h"
#include "videodsp.h"
#include "vp8dsp.h"
#include "vpx_rac.h"

#define VP8_MAX_QUANT 127

typedef enum {
    VP8_FRAME_NONE     = -1,
    VP8_FRAME_CURRENT  =  0,
    VP8_FRAME_PREVIOUS =  1,
    VP8_FRAME_GOLDEN   =  2,
    VP8_FRAME_ALTREF   =  3,
} VP8FrameType;

enum dct_token {
    DCT_0,
    DCT_1,
    DCT_2,
    DCT_3,
    DCT_4,
    DCT_CAT1,
    DCT_CAT2,
    DCT_CAT3,
    DCT_CAT4,
    DCT_CAT5,
    DCT_CAT6,
    DCT_EOB,

    NUM_DCT_TOKENS
};

// used to signal 4x4 intra pred in luma MBs
#define MODE_I4x4 4

enum inter_mvmode {
    VP8_MVMODE_ZERO = MODE_I4x4 + 1,
    VP8_MVMODE_MV,
    VP8_MVMODE_SPLIT
};

enum inter_splitmvmode {
    VP8_SPLITMVMODE_16x8 = 0,    ///< 2 16x8 blocks (vertical)
    VP8_SPLITMVMODE_8x16,        ///< 2 8x16 blocks (horizontal)
    VP8_SPLITMVMODE_8x8,         ///< 2x2 blocks of 8x8px each
    VP8_SPLITMVMODE_4x4,         ///< 4x4 blocks of 4x4px each
    VP8_SPLITMVMODE_NONE,        ///< (only used in prediction) no split MVs
};

typedef struct VP8mv {
    DECLARE_ALIGNED(4, int16_t, x);
    int16_t y;
} VP8mv;

typedef struct VP8FilterStrength {
    uint8_t filter_level;
    uint8_t inner_limit;
    uint8_t inner_filter;
} VP8FilterStrength;

typedef struct VP8Macroblock {
    uint8_t skip;
    // TODO: make it possible to check for at least (i4x4 or split_mv)
    // in one op. are others needed?
    uint8_t mode;
    uint8_t ref_frame;
    uint8_t partitioning;
    uint8_t chroma_pred_mode;
    uint8_t segment;
    uint8_t intra4x4_pred_mode_mb[16];
    DECLARE_ALIGNED(4, uint8_t, intra4x4_pred_mode_top)[4];
    VP8mv mv;
    VP8mv bmv[16];
} VP8Macroblock;

typedef struct VP8intmv {
    int x;
    int y;
} VP8intmv;

typedef struct VP8mvbounds {
    VP8intmv mv_min;
    VP8intmv mv_max;
} VP8mvbounds;

typedef struct VP8ThreadData {
    DECLARE_ALIGNED(16, int16_t, block)[6][4][16];
    DECLARE_ALIGNED(16, int16_t, block_dc)[16];
    /**
     * This is the index plus one of the last non-zero coeff
     * for each of the blocks in the current macroblock.
     * So, 0 -> no coeffs
     *     1 -> dc-only (special transform)
     *     2+-> full transform
     */
    DECLARE_ALIGNED(16, uint8_t, non_zero_count_cache)[6][4];
    /**
     * For coeff decode, we need to know whether the above block had non-zero
     * coefficients. This means for each macroblock, we need data for 4 luma
     * blocks, 2 u blocks, 2 v blocks, and the luma dc block, for a total of 9
     * per macroblock. We keep the last row in top_nnz.
     */
    DECLARE_ALIGNED(8, uint8_t, left_nnz)[9];
    int thread_nr;
#if HAVE_THREADS
    pthread_mutex_t lock;
    pthread_cond_t cond;
#endif
    atomic_int thread_mb_pos; // (mb_y << 16) | (mb_x & 0xFFFF)
    atomic_int wait_mb_pos; // What the current thread is waiting on.

#define EDGE_EMU_LINESIZE 32
    DECLARE_ALIGNED(16, uint8_t, edge_emu_buffer)[21 * EDGE_EMU_LINESIZE];
    VP8FilterStrength *filter_strength;
    VP8mvbounds mv_bounds;
} VP8ThreadData;

typedef struct VP8Frame {
    ProgressFrame tf;
    uint8_t *seg_map; ///< RefStruct reference

    void *hwaccel_picture_private; ///< RefStruct reference
} VP8Frame;

#define MAX_THREADS 8
typedef struct VP8Context {
    VP8ThreadData *thread_data;
    AVCodecContext *avctx;
    enum AVPixelFormat pix_fmt;
    int actually_webp;

    VP8Frame *framep[4];
    VP8Frame *next_framep[4];
    VP8Frame *curframe;
    VP8Frame *prev_frame;

    uint16_t mb_width;   /* number of horizontal MB */
    uint16_t mb_height;  /* number of vertical MB */
    ptrdiff_t linesize;
    ptrdiff_t uvlinesize;

    uint8_t keyframe;
    uint8_t deblock_filter;
    uint8_t mbskip_enabled;
    uint8_t profile;
    VP8mvbounds mv_bounds;

    int8_t sign_bias[4]; ///< one state [0, 1] per ref frame type
    int ref_count[3];

    /**
     * Base parameters for segmentation, i.e. per-macroblock parameters.
     * These must be kept unchanged even if segmentation is not used for
     * a frame, since the values persist between interframes.
     */
    struct {
        uint8_t enabled;
        uint8_t absolute_vals;
        uint8_t update_map;
        uint8_t update_feature_data;
        int8_t base_quant[4];
        int8_t filter_level[4];     ///< base loop filter level
    } segmentation;

    struct {
        uint8_t simple;
        uint8_t level;
        uint8_t sharpness;
    } filter;

    VP8Macroblock *macroblocks;

    uint8_t *intra4x4_pred_mode_top;
    uint8_t intra4x4_pred_mode_left[4];

    /**
     * Macroblocks can have one of 4 different quants in a frame when
     * segmentation is enabled.
     * If segmentation is disabled, only the first segment's values are used.
     */
    struct {
        // [0] - DC qmul  [1] - AC qmul
        int16_t luma_qmul[2];
        int16_t luma_dc_qmul[2];    ///< luma dc-only block quant
        int16_t chroma_qmul[2];
    } qmat[4];

    // Raw quantisation values, which may be needed by hwaccel decode.
    struct {
        int yac_qi;
        int ydc_delta;
        int y2dc_delta;
        int y2ac_delta;
        int uvdc_delta;
        int uvac_delta;
    } quant;

    struct {
        uint8_t enabled;    ///< whether each mb can have a different strength based on mode/ref
        uint8_t update;

        /**
         * filter strength adjustment for the following macroblock modes:
         * [0-3] - i16x16 (always zero)
         * [4]   - i4x4
         * [5]   - zero mv
         * [6]   - inter modes except for zero or split mv
         * [7]   - split mv
         *  i16x16 modes never have any adjustment
         */
        int8_t mode[VP8_MVMODE_SPLIT + 1];

        /**
         * filter strength adjustment for macroblocks that reference:
         * [0] - intra / VP8_FRAME_CURRENT
         * [1] - VP8_FRAME_PREVIOUS
         * [2] - VP8_FRAME_GOLDEN
         * [3] - altref / VP8_FRAME_ALTREF
         */
        int8_t ref[4];
    } lf_delta;

    uint8_t (*top_border)[16 + 8 + 8];
    uint8_t (*top_nnz)[9];

    VPXRangeCoder c;   ///< header context, includes mb modes and motion vectors

    /* This contains the entropy coder state at the end of the header
     * block, in the form specified by the standard.  For use by
     * hwaccels, so that a hardware decoder has the information to
     * start decoding at the macroblock layer.
     */
    struct {
        const uint8_t *input;
        uint32_t range;
        uint32_t value;
        int bit_count;
    } coder_state_at_header_end;

    int header_partition_size;

    /**
     * These are all of the updatable probabilities for binary decisions.
     * They are only implicitly reset on keyframes, making it quite likely
     * for an interframe to desync if a prior frame's header was corrupt
     * or missing outright!
     */
    struct {
        uint8_t segmentid[3];
        uint8_t mbskip;
        uint8_t intra;
        uint8_t last;
        uint8_t golden;
        uint8_t pred16x16[4];
        uint8_t pred8x8c[3];
        uint8_t token[4][16][3][NUM_DCT_TOKENS - 1];
        uint8_t mvc[2][19];
        uint8_t scan[16];
    } prob[2];

    VP8Macroblock *macroblocks_base;
    int invisible;
    int update_last;    ///< update VP8_FRAME_PREVIOUS with the current one
    int update_golden;  ///< VP8_FRAME_NONE if not updated, or which frame to copy if so
    int update_altref;

    /**
     * If this flag is not set, all the probability updates
     * are discarded after this frame is decoded.
     */
    int update_probabilities;

    /**
     * All coefficients are contained in separate arith coding contexts.
     * There can be 1, 2, 4, or 8 of these after the header context.
     */
    int num_coeff_partitions;
    VPXRangeCoder coeff_partition[8];
    int coeff_partition_size[8];
    VideoDSPContext vdsp;
    VP8DSPContext vp8dsp;
    H264PredContext hpc;
    vp8_mc_func put_pixels_tab[3][3][3];
    VP8Frame frames[5];

    uint8_t colorspace; ///< 0 is the only value allowed (meaning bt601)
    uint8_t fullrange;  ///< whether we can skip clamping in dsp functions

    int num_jobs;
    /**
     * This describes the macroblock memory layout.
     * 0 -> Only width+height*2+1 macroblocks allocated (frame/single thread).
     * 1 -> Macroblocks for entire frame allocated (sliced thread).
     */
    int mb_layout;

    int (*decode_mb_row_no_filter)(AVCodecContext *avctx, void *tdata, int jobnr, int threadnr);
    void (*filter_mb_row)(AVCodecContext *avctx, void *tdata, int jobnr, int threadnr);

    /**
     * Interframe DC prediction (VP7)
     * [0] VP8_FRAME_PREVIOUS
     * [1] VP8_FRAME_GOLDEN
     */
    uint16_t inter_dc_pred[2][2];

    /**
     * Macroblock features (VP7)
     */
    uint8_t feature_enabled[4];
    uint8_t feature_present_prob[4];
    uint8_t feature_index_prob[4][3];
    uint8_t feature_value[4][4];
} VP8Context;

int ff_vp8_decode_init(AVCodecContext *avctx);

int ff_vp8_decode_frame(AVCodecContext *avctx, AVFrame *frame,
                        int *got_frame, AVPacket *avpkt);

int ff_vp8_decode_free(AVCodecContext *avctx);

#endif /* AVCODEC_VP8_H */
