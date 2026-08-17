/*
 * Motion estimation
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

#ifndef AVCODEC_MOTION_EST_H
#define AVCODEC_MOTION_EST_H

#include <stdint.h>

#include "avcodec.h"
#include "hpeldsp.h"
#include "me_cmp.h"
#include "qpeldsp.h"

struct MpegEncContext;

#if ARCH_IA64 // Limit static arrays to avoid gcc failing "short data segment overflowed"
#define MAX_MV 1024
#else
#define MAX_MV 4096
#endif
#define MAX_DMV (2*MAX_MV)
#define ME_MAP_SIZE 64

#define FF_ME_ZERO 0
#define FF_ME_EPZS 1
#define FF_ME_XONE 2

/**
 * Motion estimation context.
 */
typedef struct MotionEstContext {
    AVCodecContext *avctx;
    int skip;                       ///< set if ME is skipped for the current MB
    int co_located_mv[4][2];        ///< mv from last P-frame for direct mode ME
    int direct_basis_mv[4][2];
    uint8_t *scratchpad;            /**< data area for the ME algo, so that
                                     * the ME does not need to malloc/free. */
    uint8_t *temp;
    uint32_t *map;                  ///< map to avoid duplicate evaluations
    uint32_t *score_map;            ///< map to store the scores
    unsigned map_generation;
    int pre_penalty_factor;
    int penalty_factor;             /**< an estimate of the bits required to
                                     * code a given mv value, e.g. (1,0) takes
                                     * more bits than (0,0). We have to
                                     * estimate whether any reduction in
                                     * residual is worth the extra bits. */
    int sub_penalty_factor;
    int mb_penalty_factor;
    int flags;
    int sub_flags;
    int mb_flags;
    int pre_pass;                   ///< = 1 for the pre pass
    int dia_size;
    int xmin;
    int xmax;
    int ymin;
    int ymax;
    int pred_x;
    int pred_y;
    const uint8_t *src[4][4];
    const uint8_t *ref[4][4];
    int stride;
    int uvstride;
    /* temp variables for picture complexity calculation */
    int64_t mc_mb_var_sum_temp;
    int64_t mb_var_sum_temp;
    int scene_change_score;

    me_cmp_func me_pre_cmp[6];
    me_cmp_func me_cmp[6];
    me_cmp_func me_sub_cmp[6];
    me_cmp_func mb_cmp[6];

    me_cmp_func pix_abs[2][4];
    me_cmp_func sse;

    op_pixels_func(*hpel_put)[4];
    op_pixels_func(*hpel_avg)[4];
    qpel_mc_func(*qpel_put)[16];
    qpel_mc_func(*qpel_avg)[16];
    const uint8_t (*mv_penalty)[MAX_DMV * 2 + 1]; ///< bit amount needed to encode a MV
    const uint8_t *current_mv_penalty;
    int (*sub_motion_search)(struct MpegEncContext *s,
                             int *mx_ptr, int *my_ptr, int dmin,
                             int src_index, int ref_index,
                             int size, int h);
} MotionEstContext;

static inline int ff_h263_round_chroma(int x)
{
    //FIXME static or not?
    static const uint8_t h263_chroma_roundtab[16] = {
    //  0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15
        0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1,
    };
    return h263_chroma_roundtab[x & 0xf] + (x >> 3);
}

/**
 * Performs one-time initialization of the MotionEstContext.
 */
int ff_me_init(MotionEstContext *c, struct AVCodecContext *avctx,
               const struct MECmpContext *mecc, int mpvenc);

void ff_me_init_pic(struct MpegEncContext *s);

void ff_estimate_p_frame_motion(struct MpegEncContext *s, int mb_x, int mb_y);
void ff_estimate_b_frame_motion(struct MpegEncContext *s, int mb_x, int mb_y);

int ff_pre_estimate_p_frame_motion(struct MpegEncContext *s,
                                   int mb_x, int mb_y);

int ff_epzs_motion_search(struct MpegEncContext *s, int *mx_ptr, int *my_ptr,
                          int P[10][2], int src_index, int ref_index,
                          const int16_t (*last_mv)[2], int ref_mv_scale,
                          int size, int h);

int ff_get_mb_score(struct MpegEncContext *s, int mx, int my, int src_index,
                    int ref_index, int size, int h, int add_rate);

int ff_get_best_fcode(struct MpegEncContext *s,
                      const int16_t (*mv_table)[2], int type);

void ff_fix_long_p_mvs(struct MpegEncContext *s, int type);
void ff_fix_long_mvs(struct MpegEncContext *s, uint8_t *field_select_table,
                     int field_select, int16_t (*mv_table)[2], int f_code,
                     int type, int truncate);

#endif /* AVCODEC_MOTION_EST_H */
