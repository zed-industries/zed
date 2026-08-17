/*
 * VP9 compatible video decoder
 *
 * Copyright (C) 2013 Ronald S. Bultje <rsbultje gmail com>
 * Copyright (C) 2013 Clément Bœsch <u pkh me>
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

#ifndef AVCODEC_VP9DEC_H
#define AVCODEC_VP9DEC_H

#include <stddef.h>
#include <stdint.h>
#include <stdatomic.h>

#include "libavutil/mem_internal.h"
#include "libavutil/pixfmt.h"
#include "libavutil/thread.h"

#include "get_bits.h"
#include "videodsp.h"
#include "vp9.h"
#include "vp9dsp.h"
#include "vp9shared.h"
#include "vpx_rac.h"

#define REF_INVALID_SCALE 0xFFFF

enum MVJoint {
    MV_JOINT_ZERO,
    MV_JOINT_H,
    MV_JOINT_V,
    MV_JOINT_HV,
};

typedef struct ProbContext {
    uint8_t y_mode[4][9];
    uint8_t uv_mode[10][9];
    uint8_t filter[4][2];
    uint8_t mv_mode[7][3];
    uint8_t intra[4];
    uint8_t comp[5];
    uint8_t single_ref[5][2];
    uint8_t comp_ref[5];
    uint8_t tx32p[2][3];
    uint8_t tx16p[2][2];
    uint8_t tx8p[2];
    uint8_t skip[3];
    uint8_t mv_joint[3];
    struct {
        uint8_t sign;
        uint8_t classes[10];
        uint8_t class0;
        uint8_t bits[10];
        uint8_t class0_fp[2][3];
        uint8_t fp[3];
        uint8_t class0_hp;
        uint8_t hp;
    } mv_comp[2];
    uint8_t partition[4][4][3];
} ProbContext;

typedef struct VP9Filter {
    uint8_t level[8 * 8];
    uint8_t /* bit=col */ mask[2 /* 0=y, 1=uv */][2 /* 0=col, 1=row */]
                              [8 /* rows */][4 /* 0=16, 1=8, 2=4, 3=inner4 */];
} VP9Filter;

typedef struct VP9Block {
    uint8_t seg_id, intra, comp, ref[2], mode[4], uvmode, skip;
    enum FilterMode filter;
    VP9mv mv[4 /* b_idx */][2 /* ref */];
    enum BlockSize bs;
    enum TxfmMode tx, uvtx;
    enum BlockLevel bl;
    enum BlockPartition bp;
} VP9Block;

typedef struct VP9TileData VP9TileData;

typedef struct VP9Context {
    VP9SharedContext s;
    VP9TileData *td;

    VP9DSPContext dsp;
    VideoDSPContext vdsp;
    GetBitContext gb;
    VPXRangeCoder c;
    int pass, active_tile_cols;

#if HAVE_THREADS
    pthread_mutex_t progress_mutex;
    pthread_cond_t progress_cond;
    atomic_int *entries;
    unsigned pthread_init_cnt;
#endif

    uint8_t ss_h, ss_v;
    uint8_t last_bpp, bpp_index, bytesperpixel;
    uint8_t last_keyframe;
    // sb_cols/rows, rows/cols and last_fmt are used for allocating all internal
    // arrays, and are thus per-thread. w/h and gf_fmt are synced between threads
    // and are therefore per-stream. pix_fmt represents the value in the header
    // of the currently processed frame.
    int w, h;
    enum AVPixelFormat pix_fmt, last_fmt, gf_fmt;
    unsigned sb_cols, sb_rows, rows, cols;
    ProgressFrame next_refs[8];

    struct {
        uint8_t lim_lut[64];
        uint8_t mblim_lut[64];
    } filter_lut;
    struct {
        ProbContext p;
        uint8_t coef[4][2][2][6][6][3];
    } prob_ctx[4];
    struct {
        ProbContext p;
        uint8_t coef[4][2][2][6][6][11];
    } prob;

    // contextual (above) cache
    uint8_t *above_partition_ctx;
    uint8_t *above_mode_ctx;
    // FIXME maybe merge some of the below in a flags field?
    uint8_t *above_y_nnz_ctx;
    uint8_t *above_uv_nnz_ctx[2];
    uint8_t *above_skip_ctx; // 1bit
    uint8_t *above_txfm_ctx; // 2bit
    uint8_t *above_segpred_ctx; // 1bit
    uint8_t *above_intra_ctx; // 1bit
    uint8_t *above_comp_ctx; // 1bit
    uint8_t *above_ref_ctx; // 2bit
    uint8_t *above_filter_ctx;
    VP9mv (*above_mv_ctx)[2];

    // whole-frame cache
    uint8_t *intra_pred_data[3];
    VP9Filter *lflvl;

    // block reconstruction intermediates
    int block_alloc_using_2pass;
    uint16_t mvscale[3][2];
    uint8_t mvstep[3][2];

    // frame specific buffer pools
    struct AVRefStructPool *frame_extradata_pool;
    int frame_extradata_pool_size;
} VP9Context;

struct VP9TileData {
    const VP9Context *s;
    VPXRangeCoder *c_b;
    VPXRangeCoder *c;
    int row, row7, col, col7;
    uint8_t *dst[3];
    ptrdiff_t y_stride, uv_stride;
    VP9Block *b_base, *b;
    unsigned tile_col_start;

    struct {
        unsigned y_mode[4][10];
        unsigned uv_mode[10][10];
        unsigned filter[4][3];
        unsigned mv_mode[7][4];
        unsigned intra[4][2];
        unsigned comp[5][2];
        unsigned single_ref[5][2][2];
        unsigned comp_ref[5][2];
        unsigned tx32p[2][4];
        unsigned tx16p[2][3];
        unsigned tx8p[2][2];
        unsigned skip[3][2];
        unsigned mv_joint[4];
        struct {
            unsigned sign[2];
            unsigned classes[11];
            unsigned class0[2];
            unsigned bits[10][2];
            unsigned class0_fp[2][4];
            unsigned fp[4];
            unsigned class0_hp[2];
            unsigned hp[2];
        } mv_comp[2];
        unsigned partition[4][4][4];
        unsigned coef[4][2][2][6][6][3];
        unsigned eob[4][2][2][6][6][2];
    } counts;

    // whole-frame cache
    DECLARE_ALIGNED(32, uint8_t, edge_emu_buffer)[135 * 144 * 2];

    // contextual (left) cache
    DECLARE_ALIGNED(16, uint8_t, left_y_nnz_ctx)[16];
    DECLARE_ALIGNED(16, uint8_t, left_mode_ctx)[16];
    DECLARE_ALIGNED(16, VP9mv, left_mv_ctx)[16][2];
    DECLARE_ALIGNED(16, uint8_t, left_uv_nnz_ctx)[2][16];
    DECLARE_ALIGNED(8, uint8_t, left_partition_ctx)[8];
    DECLARE_ALIGNED(8, uint8_t, left_skip_ctx)[8];
    DECLARE_ALIGNED(8, uint8_t, left_txfm_ctx)[8];
    DECLARE_ALIGNED(8, uint8_t, left_segpred_ctx)[8];
    DECLARE_ALIGNED(8, uint8_t, left_intra_ctx)[8];
    DECLARE_ALIGNED(8, uint8_t, left_comp_ctx)[8];
    DECLARE_ALIGNED(8, uint8_t, left_ref_ctx)[8];
    DECLARE_ALIGNED(8, uint8_t, left_filter_ctx)[8];
    // block reconstruction intermediates
    DECLARE_ALIGNED(32, uint8_t, tmp_y)[64 * 64 * 2];
    DECLARE_ALIGNED(32, uint8_t, tmp_uv)[2][64 * 64 * 2];
    struct { int x, y; } min_mv, max_mv;
    int16_t *block_base, *block, *uvblock_base[2], *uvblock[2];
    uint8_t *eob_base, *uveob_base[2], *eob, *uveob[2];

    // error message
    int error_info;
    struct {
        unsigned int row:13;
        unsigned int col:13;
        unsigned int block_size_idx_x:2;
        unsigned int block_size_idx_y:2;
    } *block_structure;
    unsigned int nb_block_structure;
};

void ff_vp9_fill_mv(VP9TileData *td, VP9mv *mv, int mode, int sb);

void ff_vp9_adapt_probs(VP9Context *s);

void ff_vp9_decode_block(VP9TileData *td, int row, int col,
                         VP9Filter *lflvl, ptrdiff_t yoff, ptrdiff_t uvoff,
                         enum BlockLevel bl, enum BlockPartition bp);

void ff_vp9_loopfilter_sb(struct AVCodecContext *avctx, VP9Filter *lflvl,
                          int row, int col, ptrdiff_t yoff, ptrdiff_t uvoff);

void ff_vp9_intra_recon_8bpp(VP9TileData *td,
                             ptrdiff_t y_off, ptrdiff_t uv_off);
void ff_vp9_intra_recon_16bpp(VP9TileData *td,
                              ptrdiff_t y_off, ptrdiff_t uv_off);
void ff_vp9_inter_recon_8bpp(VP9TileData *td);
void ff_vp9_inter_recon_16bpp(VP9TileData *td);

#endif /* AVCODEC_VP9DEC_H */
