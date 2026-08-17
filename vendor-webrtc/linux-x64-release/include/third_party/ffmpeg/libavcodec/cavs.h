/*
 * Chinese AVS video (AVS1-P2, JiZhun profile) decoder.
 * Copyright (c) 2006  Stefan Gehrer <stefan.gehrer@gmx.de>
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

#ifndef AVCODEC_CAVS_H
#define AVCODEC_CAVS_H

#include <stddef.h>
#include <stdint.h>

#include "libavutil/frame.h"
#include "libavutil/mem_internal.h"

#include "avcodec.h"
#include "cavsdsp.h"
#include "blockdsp.h"
#include "h264chroma.h"
#include "get_bits.h"
#include "videodsp.h"

#define SLICE_MAX_START_CODE    0x000001af
#define EXT_START_CODE          0x000001b5
#define USER_START_CODE         0x000001b2
#define CAVS_START_CODE         0x000001b0
#define PIC_I_START_CODE        0x000001b3
#define PIC_PB_START_CODE       0x000001b6

#define A_AVAIL                          1
#define B_AVAIL                          2
#define C_AVAIL                          4
#define D_AVAIL                          8
#define NOT_AVAIL                       -1
#define REF_INTRA                       -2
#define REF_DIR                         -3

#define ESCAPE_CODE                     59

#define FWD0                          0x01
#define FWD1                          0x02
#define BWD0                          0x04
#define BWD1                          0x08
#define SYM0                          0x10
#define SYM1                          0x20
#define SPLITH                        0x40
#define SPLITV                        0x80

#define MV_BWD_OFFS                     12
#define MV_STRIDE                        4

enum cavs_mb {
  I_8X8 = 0,
  P_SKIP,
  P_16X16,
  P_16X8,
  P_8X16,
  P_8X8,
  B_SKIP,
  B_DIRECT,
  B_FWD_16X16,
  B_BWD_16X16,
  B_SYM_16X16,
  B_8X8 = 29
};

enum cavs_sub_mb {
  B_SUB_DIRECT,
  B_SUB_FWD,
  B_SUB_BWD,
  B_SUB_SYM
};

enum cavs_intra_luma {
  INTRA_L_VERT,
  INTRA_L_HORIZ,
  INTRA_L_LP,
  INTRA_L_DOWN_LEFT,
  INTRA_L_DOWN_RIGHT,
  INTRA_L_LP_LEFT,
  INTRA_L_LP_TOP,
  INTRA_L_DC_128
};

enum cavs_intra_chroma {
  INTRA_C_LP,
  INTRA_C_HORIZ,
  INTRA_C_VERT,
  INTRA_C_PLANE,
  INTRA_C_LP_LEFT,
  INTRA_C_LP_TOP,
  INTRA_C_DC_128,
};

enum cavs_mv_pred {
  MV_PRED_MEDIAN,
  MV_PRED_LEFT,
  MV_PRED_TOP,
  MV_PRED_TOPRIGHT,
  MV_PRED_PSKIP,
  MV_PRED_BSKIP
};

enum cavs_block {
  BLK_16X16,
  BLK_16X8,
  BLK_8X16,
  BLK_8X8
};

enum cavs_mv_loc {
  MV_FWD_D3 = 0,
  MV_FWD_B2,
  MV_FWD_B3,
  MV_FWD_C2,
  MV_FWD_A1,
  MV_FWD_X0,
  MV_FWD_X1,
  MV_FWD_A3 = 8,
  MV_FWD_X2,
  MV_FWD_X3,
  MV_BWD_D3 = MV_BWD_OFFS,
  MV_BWD_B2,
  MV_BWD_B3,
  MV_BWD_C2,
  MV_BWD_A1,
  MV_BWD_X0,
  MV_BWD_X1,
  MV_BWD_A3 = MV_BWD_OFFS+8,
  MV_BWD_X2,
  MV_BWD_X3
};

typedef struct cavs_vector {
    int16_t x;
    int16_t y;
    int16_t dist;
    int16_t ref;
} cavs_vector;

struct dec_2dvlc {
  int8_t rltab[59][3];
  int8_t level_add[27];
  int8_t golomb_order;
  int inc_limit;
  int8_t max_run;
};

typedef struct AVSFrame {
    AVFrame *f;
    int poc;
} AVSFrame;

typedef struct AVSContext {
    AVCodecContext *avctx;
    BlockDSPContext bdsp;
    H264ChromaContext h264chroma;
    VideoDSPContext vdsp;
    CAVSDSPContext  cdsp;
    GetBitContext gb;
    AVSFrame cur;     ///< currently decoded frame
    AVSFrame DPB[2];  ///< reference frames
    int dist[2];     ///< temporal distances from current frame to ref frames
    int low_delay;
    int profile, level;
    int aspect_ratio;
    int mb_width, mb_height;
    int width, height;
    int stream_revision; ///<0 for samples from 2006, 1 for rm52j encoder
    int progressive;
    int pic_structure;
    int skip_mode_flag; ///< select between skip_count or one skip_flag per MB
    int loop_filter_disable;
    int alpha_offset, beta_offset;
    int ref_flag;
    int mbx, mby, mbidx; ///< macroblock coordinates
    int flags;         ///< availability flags of neighbouring macroblocks
    int stc;           ///< last start code
    uint8_t *cy, *cu, *cv; ///< current MB sample pointers
    int left_qp;
    uint8_t *top_qp;

    /** mv motion vector cache
       0:    D3  B2  B3  C2
       4:    A1  X0  X1   -
       8:    A3  X2  X3   -

       X are the vectors in the current macroblock (5,6,9,10)
       A is the macroblock to the left (4,8)
       B is the macroblock to the top (1,2)
       C is the macroblock to the top-right (3)
       D is the macroblock to the top-left (0)

       the same is repeated for backward motion vectors */
    DECLARE_ALIGNED(8, cavs_vector, mv)[2*4*3];
    cavs_vector *top_mv[2];
    cavs_vector *col_mv;

    /** luma pred mode cache
       0:    --  B2  B3
       3:    A1  X0  X1
       6:    A3  X2  X3   */
    int pred_mode_Y[3*3];
    int *top_pred_Y;
    ptrdiff_t l_stride, c_stride;
    int luma_scan[4];
    int qp;
    int qp_fixed;
    int pic_qp_fixed;
    int cbp;
    uint8_t permutated_scantable[64];

    /** intra prediction is done with un-deblocked samples
     they are saved here before deblocking the MB  */
    uint8_t *top_border_y, *top_border_u, *top_border_v;
    uint8_t left_border_y[26], left_border_u[10], left_border_v[10];
    uint8_t intern_border_y[26];
    uint8_t topleft_border_y, topleft_border_u, topleft_border_v;

    void (*intra_pred_l[8])(uint8_t *d, uint8_t *top, uint8_t *left, ptrdiff_t stride);
    void (*intra_pred_c[7])(uint8_t *d, uint8_t *top, uint8_t *left, ptrdiff_t stride);
    uint8_t *col_type_base;

    /* scaling factors for MV prediction */
    int sym_factor;    ///< for scaling in symmetrical B block
    int direct_den[2]; ///< for scaling in direct B block
    int scale_den[2];  ///< for scaling neighbouring MVs

    uint8_t *edge_emu_buffer;

    int got_keyframe;
    int16_t *block;
} AVSContext;

extern const uint8_t     ff_cavs_chroma_qp[64];
extern const uint8_t     ff_cavs_partition_flags[30];
extern const cavs_vector ff_cavs_intra_mv;
extern const cavs_vector ff_cavs_dir_mv;

static inline void set_mvs(cavs_vector *mv, enum cavs_block size) {
    switch(size) {
    case BLK_16X16:
        mv[MV_STRIDE  ] = mv[0];
        mv[MV_STRIDE+1] = mv[0];
    case BLK_16X8:
        mv[1] = mv[0];
        break;
    case BLK_8X16:
        mv[MV_STRIDE] = mv[0];
        break;
    }
}

void ff_cavs_filter(AVSContext *h, enum cavs_mb mb_type);
void ff_cavs_load_intra_pred_luma(AVSContext *h, uint8_t *top, uint8_t **left,
                                  int block);
void ff_cavs_load_intra_pred_chroma(AVSContext *h);
void ff_cavs_modify_mb_i(AVSContext *h, int *pred_mode_uv);
void ff_cavs_inter(AVSContext *h, enum cavs_mb mb_type);
void ff_cavs_mv(AVSContext *h, enum cavs_mv_loc nP, enum cavs_mv_loc nC,
                enum cavs_mv_pred mode, enum cavs_block size, int ref);
void ff_cavs_init_mb(AVSContext *h);
int  ff_cavs_next_mb(AVSContext *h);
int ff_cavs_init_pic(AVSContext *h);
int ff_cavs_init_top_lines(AVSContext *h);
int ff_cavs_init(AVCodecContext *avctx);
int ff_cavs_end (AVCodecContext *avctx);

#endif /* AVCODEC_CAVS_H */
