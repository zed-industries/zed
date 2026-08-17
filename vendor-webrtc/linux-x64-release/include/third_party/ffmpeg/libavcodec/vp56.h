/*
 * Copyright (C) 2006  Aurelien Jacobs <aurel@gnuage.org>
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
 * VP5 and VP6 compatible video decoder (common features)
 */

#ifndef AVCODEC_VP56_H
#define AVCODEC_VP56_H

#include "libavutil/mem_internal.h"

#include "avcodec.h"
#include "get_bits.h"
#include "hpeldsp.h"
#include "h264chroma.h"
#include "videodsp.h"
#include "vp3dsp.h"
#include "vp56dsp.h"
#include "vpx_rac.h"

typedef struct vp56_context VP56Context;

typedef enum {
    VP56_FRAME_NONE     =-1,
    VP56_FRAME_CURRENT  = 0,
    VP56_FRAME_PREVIOUS = 1,
    VP56_FRAME_GOLDEN   = 2,
} VP56Frame;

typedef enum {
    VP56_MB_INTER_NOVEC_PF = 0,  /**< Inter MB, no vector, from previous frame */
    VP56_MB_INTRA          = 1,  /**< Intra MB */
    VP56_MB_INTER_DELTA_PF = 2,  /**< Inter MB, above/left vector + delta, from previous frame */
    VP56_MB_INTER_V1_PF    = 3,  /**< Inter MB, first vector, from previous frame */
    VP56_MB_INTER_V2_PF    = 4,  /**< Inter MB, second vector, from previous frame */
    VP56_MB_INTER_NOVEC_GF = 5,  /**< Inter MB, no vector, from golden frame */
    VP56_MB_INTER_DELTA_GF = 6,  /**< Inter MB, above/left vector + delta, from golden frame */
    VP56_MB_INTER_4V       = 7,  /**< Inter MB, 4 vectors, from previous frame */
    VP56_MB_INTER_V1_GF    = 8,  /**< Inter MB, first vector, from golden frame */
    VP56_MB_INTER_V2_GF    = 9,  /**< Inter MB, second vector, from golden frame */
} VP56mb;

typedef struct VP56Tree {
  int8_t val;
  int8_t prob_idx;
} VP56Tree;

typedef struct VP56mv {
    DECLARE_ALIGNED(4, int16_t, x);
    int16_t y;
} VP56mv;

#define VP56_SIZE_CHANGE 1

typedef void (*VP56ParseVectorAdjustment)(VP56Context *s,
                                          VP56mv *vect);
typedef void (*VP56Filter)(VP56Context *s, uint8_t *dst, uint8_t *src,
                           int offset1, int offset2, ptrdiff_t stride,
                           VP56mv mv, int mask, int select, int luma);
typedef int  (*VP56ParseCoeff)(VP56Context *s);
typedef void (*VP56DefaultModelsInit)(VP56Context *s);
typedef void (*VP56ParseVectorModels)(VP56Context *s);
typedef int  (*VP56ParseCoeffModels)(VP56Context *s);
typedef int  (*VP56ParseHeader)(VP56Context *s, const uint8_t *buf,
                                int buf_size);

typedef struct VP56RefDc {
    uint8_t not_null_dc;
    VP56Frame ref_frame;
    int16_t dc_coeff;
} VP56RefDc;

typedef struct VP56Macroblock {
    uint8_t type;
    VP56mv mv;
} VP56Macroblock;

typedef struct VP56Model {
    uint8_t coeff_reorder[64];       /* used in vp6 only */
    uint8_t coeff_index_to_pos[64];  /* used in vp6 only */
    uint8_t coeff_index_to_idct_selector[64]; /* used in vp6 only */
    uint8_t vector_sig[2];           /* delta sign */
    uint8_t vector_dct[2];           /* delta coding types */
    uint8_t vector_pdi[2][2];        /* predefined delta init */
    uint8_t vector_pdv[2][7];        /* predefined delta values */
    uint8_t vector_fdv[2][8];        /* 8 bit delta value definition */
    uint8_t coeff_dccv[2][11];       /* DC coeff value */
    uint8_t coeff_ract[2][3][6][11]; /* Run/AC coding type and AC coeff value */
    uint8_t coeff_acct[2][3][3][6][5];/* vp5 only AC coding type for coding group < 3 */
    uint8_t coeff_dcct[2][36][5];    /* DC coeff coding type */
    uint8_t coeff_runv[2][14];       /* run value (vp6 only) */
    uint8_t mb_type[3][10][10];      /* model for decoding MB type */
    uint8_t mb_types_stats[3][10][2];/* contextual, next MB type stats */
} VP56Model;

struct vp56_context {
    AVCodecContext *avctx;
    H264ChromaContext h264chroma;
    HpelDSPContext hdsp;
    VideoDSPContext vdsp;
    VP3DSPContext vp3dsp;
    VP56DSPContext vp56dsp;
    uint8_t idct_scantable[64];
    AVFrame *frames[4];
    uint8_t *edge_emu_buffer_alloc;
    uint8_t *edge_emu_buffer;
    VPXRangeCoder c;
    VPXRangeCoder cc;
    VPXRangeCoder *ccp;
    int sub_version;

    /* frame info */
    int golden_frame;
    int plane_width[4];
    int plane_height[4];
    int mb_width;   /* number of horizontal MB */
    int mb_height;  /* number of vertical MB */
    int block_offset[6];

    int quantizer;
    uint16_t dequant_dc;
    uint16_t dequant_ac;

    /* DC predictors management */
    VP56RefDc *above_blocks;
    VP56RefDc left_block[4];
    int above_block_idx[6];
    int16_t prev_dc[3][3];    /* [plan][ref_frame] */

    /* blocks / macroblock */
    VP56mb mb_type;
    VP56Macroblock *macroblocks;
    DECLARE_ALIGNED(16, int16_t, block_coeff)[6][64];
    int idct_selector[6];
    const uint8_t *def_coeff_reorder;/* used in vp6 only */

    /* motion vectors */
    VP56mv mv[6];  /* vectors for each block in MB */
    VP56mv vector_candidate[2];
    int vector_candidate_pos;

    /* filtering hints */
    int filter_header;               /* used in vp6 only */
    int deblock_filtering;
    int filter_selection;
    int filter_mode;
    int max_vector_length;
    int sample_variance_threshold;
    DECLARE_ALIGNED(8, int, bounding_values_array)[256];

    uint8_t coeff_ctx[4][64];              /* used in vp5 only */
    uint8_t coeff_ctx_last[4];             /* used in vp5 only */

    int has_alpha;

    /* interlacing params */
    int interlaced;
    int il_prob;
    int il_block;

    /* upside-down flipping hints */
    int flip;  /* are we flipping ? */
    int frbi;  /* first row block index in MB */
    int srbi;  /* second row block index in MB */
    ptrdiff_t stride[4];  /* stride for each plan */

    const uint8_t *vp56_coord_div;
    VP56ParseVectorAdjustment parse_vector_adjustment;
    VP56Filter filter;
    VP56ParseCoeff parse_coeff;
    VP56DefaultModelsInit default_models_init;
    VP56ParseVectorModels parse_vector_models;
    VP56ParseCoeffModels parse_coeff_models;
    VP56ParseHeader parse_header;

    /* for "slice" parallelism between YUV and A */
    VP56Context *alpha_context;

    VP56Model *modelp;
    VP56Model model;

    /* huffman decoding */
    int use_huffman;
    GetBitContext gb;
    VLC dccv_vlc[2];
    VLC runv_vlc[2];
    VLC ract_vlc[2][3][6];
    unsigned int nb_null[2][2];       /* number of consecutive NULL DC/AC */

    int have_undamaged_frame;
    int discard_frame;
};


/**
 * Initializes an VP56Context. Expects its caller to clean up
 * in case of error.
 */
int ff_vp56_init_context(AVCodecContext *avctx, VP56Context *s,
                          int flip, int has_alpha);
int ff_vp56_free_context(VP56Context *s);
void ff_vp56_init_dequant(VP56Context *s, int quantizer);
int ff_vp56_decode_frame(AVCodecContext *avctx, AVFrame *frame,
                         int *got_frame, AVPacket *avpkt);


/**
 * vp56 specific range coder implementation
 */

static int vp56_rac_gets(VPXRangeCoder *c, int bits)
{
    int value = 0;

    while (bits--) {
        value = (value << 1) | vpx_rac_get(c);
    }

    return value;
}

// P(7)
static av_unused int vp56_rac_gets_nn(VPXRangeCoder *c, int bits)
{
    int v = vp56_rac_gets(c, 7) << 1;
    return v + !v;
}

static av_always_inline
int vp56_rac_get_tree(VPXRangeCoder *c,
                      const VP56Tree *tree,
                      const uint8_t *probs)
{
    while (tree->val > 0) {
        if (vpx_rac_get_prob_branchy(c, probs[tree->prob_idx]))
            tree += tree->val;
        else
            tree++;
    }
    return -tree->val;
}

#endif /* AVCODEC_VP56_H */
