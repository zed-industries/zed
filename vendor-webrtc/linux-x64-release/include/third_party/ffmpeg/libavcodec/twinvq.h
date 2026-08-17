/*
 * TwinVQ decoder
 * Copyright (c) 2009 Vitor Sessak
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

#ifndef AVCODEC_TWINVQ_H
#define AVCODEC_TWINVQ_H

#include <math.h>
#include <stdint.h>

#include "libavutil/attributes_internal.h"
#include "libavutil/tx.h"
#include "libavutil/common.h"
#include "libavutil/float_dsp.h"
#include "avcodec.h"

enum TwinVQCodec {
    TWINVQ_CODEC_VQF,
    TWINVQ_CODEC_METASOUND,
};

enum TwinVQFrameType {
    TWINVQ_FT_SHORT = 0,  ///< Short frame  (divided in n   sub-blocks)
    TWINVQ_FT_MEDIUM,     ///< Medium frame (divided in m<n sub-blocks)
    TWINVQ_FT_LONG,       ///< Long frame   (single sub-block + PPC)
    TWINVQ_FT_PPC,        ///< Periodic Peak Component (part of the long frame)
};

#define TWINVQ_PPC_SHAPE_CB_SIZE 64
#define TWINVQ_PPC_SHAPE_LEN_MAX 60
#define TWINVQ_SUB_AMP_MAX       4500.0
#define TWINVQ_MULAW_MU          100.0
#define TWINVQ_GAIN_BITS         8
#define TWINVQ_AMP_MAX           13000.0
#define TWINVQ_SUB_GAIN_BITS     5
#define TWINVQ_WINDOW_TYPE_BITS  4
#define TWINVQ_PGAIN_MU          200
#define TWINVQ_LSP_COEFS_MAX     20
#define TWINVQ_LSP_SPLIT_MAX     4
#define TWINVQ_CHANNELS_MAX      2
#define TWINVQ_SUBBLOCKS_MAX     16
#define TWINVQ_BARK_N_COEF_MAX   4

#define TWINVQ_MAX_FRAMES_PER_PACKET 2

/**
 * Parameters and tables that are different for each frame type
 */
struct TwinVQFrameMode {
    uint8_t         sub;      ///< Number subblocks in each frame
    const uint16_t *bark_tab;

    /** number of distinct bark scale envelope values */
    uint8_t         bark_env_size;

    const int16_t  *bark_cb;    ///< codebook for the bark scale envelope (BSE)
    uint8_t         bark_n_coef;///< number of BSE CB coefficients to read
    uint8_t         bark_n_bit; ///< number of bits of the BSE coefs

    //@{
    /** main codebooks for spectrum data */
    const int16_t    *cb0;
    const int16_t    *cb1;
    //@}

    uint8_t         cb_len_read; ///< number of spectrum coefficients to read
};

typedef struct TwinVQFrameData {
    int     window_type;
    enum TwinVQFrameType ftype;

    uint8_t main_coeffs[1024];
    uint8_t ppc_coeffs[TWINVQ_PPC_SHAPE_LEN_MAX];

    uint8_t gain_bits[TWINVQ_CHANNELS_MAX];
    uint8_t sub_gain_bits[TWINVQ_CHANNELS_MAX * TWINVQ_SUBBLOCKS_MAX];

    uint8_t bark1[TWINVQ_CHANNELS_MAX][TWINVQ_SUBBLOCKS_MAX][TWINVQ_BARK_N_COEF_MAX];
    uint8_t bark_use_hist[TWINVQ_CHANNELS_MAX][TWINVQ_SUBBLOCKS_MAX];

    uint8_t lpc_idx1[TWINVQ_CHANNELS_MAX];
    uint8_t lpc_idx2[TWINVQ_CHANNELS_MAX][TWINVQ_LSP_SPLIT_MAX];
    uint8_t lpc_hist_idx[TWINVQ_CHANNELS_MAX];

    int     p_coef[TWINVQ_CHANNELS_MAX];
    int     g_coef[TWINVQ_CHANNELS_MAX];
} TwinVQFrameData;

/**
 * Parameters and tables that are different for every combination of
 * bitrate/sample rate
 */
typedef struct TwinVQModeTab {
    struct TwinVQFrameMode fmode[3]; ///< frame type-dependent parameters

    uint16_t     size;        ///< frame size in samples
    uint8_t      n_lsp;       ///< number of lsp coefficients
    const float *lspcodebook;

    /* number of bits of the different LSP CB coefficients */
    uint8_t      lsp_bit0;
    uint8_t      lsp_bit1;
    uint8_t      lsp_bit2;

    uint8_t      lsp_split;      ///< number of CB entries for the LSP decoding
    const int16_t *ppc_shape_cb; ///< PPC shape CB

    /** number of the bits for the PPC period value */
    uint8_t      ppc_period_bit;

    uint8_t      ppc_shape_bit;  ///< number of bits of the PPC shape CB coeffs
    uint8_t      ppc_shape_len;  ///< size of PPC shape CB
    uint8_t      pgain_bit;      ///< bits for PPC gain

    /** constant for peak period to peak width conversion */
    uint16_t     peak_per2wid;
} TwinVQModeTab;

typedef struct TwinVQContext {
    AVCodecContext *avctx;
    AVFloatDSPContext *fdsp;
    AVTXContext *tx[3];
    av_tx_fn tx_fn[3];

    const TwinVQModeTab *mtab;

    int is_6kbps;

    // history
    float lsp_hist[2][20];           ///< LSP coefficients of the last frame
    float bark_hist[3][2][40];       ///< BSE coefficients of last frame

    // bitstream parameters
    int16_t permut[4][4096];
    uint8_t length[4][2];            ///< main codebook stride
    uint8_t length_change[4];
    uint8_t bits_main_spec[2][4][2]; ///< bits for the main codebook
    int bits_main_spec_change[4];
    int n_div[4];

    float *spectrum;
    float *curr_frame;               ///< non-interleaved output
    float *prev_frame;               ///< non-interleaved previous frame
    int last_block_pos[2];
    int discarded_packets;

    float *cos_tabs[3];

    // scratch buffers
    float *tmp_buf;

    int frame_size, frames_per_packet, cur_frame;
    TwinVQFrameData bits[TWINVQ_MAX_FRAMES_PER_PACKET];

    enum TwinVQCodec codec;

    int (*read_bitstream)(AVCodecContext *avctx, struct TwinVQContext *tctx,
                          const uint8_t *buf, int buf_size);
    void (*dec_bark_env)(struct TwinVQContext *tctx, const uint8_t *in,
                         int use_hist, int ch, float *out, float gain,
                         enum TwinVQFrameType ftype);
    void (*decode_ppc)(struct TwinVQContext *tctx, int period_coef, int g_coef,
                       const float *shape, float *speech);
} TwinVQContext;

FF_VISIBILITY_PUSH_HIDDEN
extern const enum TwinVQFrameType ff_twinvq_wtype_to_ftype_table[];

extern const float ff_metasound_lsp8[];
extern const float ff_metasound_lsp11[];
extern const float ff_metasound_lsp16[];
extern const float ff_metasound_lsp22[];
extern const float ff_metasound_lsp44[];

/** @note not speed critical, hence not optimized */
static inline void twinvq_memset_float(float *buf, float val, int size)
{
    while (size--)
        *buf++ = val;
}

static inline float twinvq_mulawinv(float y, float clip, float mu)
{
    y = av_clipf(y / clip, -1, 1);
    return clip * FFSIGN(y) * (exp(log(1 + mu) * fabs(y)) - 1) / mu;
}

int ff_twinvq_decode_frame(AVCodecContext *avctx, AVFrame *frame,
                           int *got_frame_ptr, AVPacket *avpkt);
int ff_twinvq_decode_close(AVCodecContext *avctx);
/** Requires the caller to call ff_twinvq_decode_close() upon failure. */
int ff_twinvq_decode_init(AVCodecContext *avctx);
FF_VISIBILITY_POP_HIDDEN

#endif /* AVCODEC_TWINVQ_H */
