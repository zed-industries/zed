/*
 * WMA compatible codec
 * Copyright (c) 2002-2007 The FFmpeg Project
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

#ifndef AVCODEC_WMA_H
#define AVCODEC_WMA_H

#include "libavutil/float_dsp.h"
#include "libavutil/mem_internal.h"
#include "libavutil/tx.h"

#include "avcodec.h"
#include "get_bits.h"
#include "put_bits.h"

/* size of blocks */
#define BLOCK_MIN_BITS 7
#define BLOCK_MAX_BITS 11
#define BLOCK_MAX_SIZE (1 << BLOCK_MAX_BITS)

#define BLOCK_NB_SIZES (BLOCK_MAX_BITS - BLOCK_MIN_BITS + 1)

/* XXX: find exact max size */
#define HIGH_BAND_MAX_SIZE 16

#define NB_LSP_COEFS 10

/* XXX: is it a suitable value ? */
#define MAX_CODED_SUPERFRAME_SIZE 32768

#define MAX_CHANNELS 2

#define NOISE_TAB_SIZE 8192

#define LSP_POW_BITS 7

// FIXME should be in wmadec
#define VLCBITS 9
#define VLCMAX ((22 + VLCBITS - 1) / VLCBITS)

typedef float WMACoef;          ///< type for decoded coefficients, int16_t would be enough for wma 1/2

typedef struct CoefVLCTable {
    int n;                      ///< total number of codes
    int max_level;
    const uint32_t *huffcodes;  ///< VLC bit values
    const uint8_t *huffbits;    ///< VLC bit size
    const uint16_t *levels;     ///< table to build run/level tables
} CoefVLCTable;

typedef struct WMACodecContext {
    AVCodecContext *avctx;
    GetBitContext gb;
    PutBitContext pb;
    int version;                            ///< 1 = 0x160 (WMAV1), 2 = 0x161 (WMAV2)
    int use_bit_reservoir;
    int use_variable_block_len;
    int use_exp_vlc;                        ///< exponent coding: 0 = lsp, 1 = vlc + delta
    int use_noise_coding;                   ///< true if perceptual noise is added
    int byte_offset_bits;
    VLC exp_vlc;
    int exponent_sizes[BLOCK_NB_SIZES];
    uint16_t exponent_bands[BLOCK_NB_SIZES][25];
    int high_band_start[BLOCK_NB_SIZES];    ///< index of first coef in high band
    int coefs_start;                        ///< first coded coef
    int coefs_end[BLOCK_NB_SIZES];          ///< max number of coded coefficients
    int exponent_high_sizes[BLOCK_NB_SIZES];
    int exponent_high_bands[BLOCK_NB_SIZES][HIGH_BAND_MAX_SIZE];
    VLC hgain_vlc;

    /* coded values in high bands */
    int high_band_coded[MAX_CHANNELS][HIGH_BAND_MAX_SIZE];
    int high_band_values[MAX_CHANNELS][HIGH_BAND_MAX_SIZE];

    /* there are two possible tables for spectral coefficients */
// FIXME the following 3 tables should be shared between decoders
    VLC coef_vlc[2];
    uint16_t *run_table[2];
    float *level_table[2];
    uint16_t *int_table[2];
    const CoefVLCTable *coef_vlcs[2];
    /* frame info */
    int frame_len;                          ///< frame length in samples
    int frame_len_bits;                     ///< frame_len = 1 << frame_len_bits
    int nb_block_sizes;                     ///< number of block sizes
    /* block info */
    int reset_block_lengths;
    int block_len_bits;                     ///< log2 of current block length
    int next_block_len_bits;                ///< log2 of next block length
    int prev_block_len_bits;                ///< log2 of prev block length
    int block_len;                          ///< block length in samples
    int block_num;                          ///< block number in current frame
    int block_pos;                          ///< current position in frame
    uint8_t ms_stereo;                      ///< true if mid/side stereo mode
    uint8_t channel_coded[MAX_CHANNELS];    ///< true if channel is coded
    int exponents_bsize[MAX_CHANNELS];      ///< log2 ratio frame/exp. length
    DECLARE_ALIGNED(32, float, exponents)[MAX_CHANNELS][BLOCK_MAX_SIZE];
    float max_exponent[MAX_CHANNELS];
    WMACoef coefs1[MAX_CHANNELS][BLOCK_MAX_SIZE];
    DECLARE_ALIGNED(32, float, coefs)[MAX_CHANNELS][BLOCK_MAX_SIZE];
    DECLARE_ALIGNED(32, float, output)[BLOCK_MAX_SIZE * 2];
    AVTXContext *mdct_ctx[BLOCK_NB_SIZES];
    av_tx_fn mdct_fn[BLOCK_NB_SIZES];
    const float *windows[BLOCK_NB_SIZES];
    /* output buffer for one frame and the last for IMDCT windowing */
    DECLARE_ALIGNED(32, float, frame_out)[MAX_CHANNELS][BLOCK_MAX_SIZE * 2];
    /* last frame info */
    uint8_t last_superframe[MAX_CODED_SUPERFRAME_SIZE + AV_INPUT_BUFFER_PADDING_SIZE]; /* padding added */
    int last_bitoffset;
    int last_superframe_len;
    int exponents_initialized[MAX_CHANNELS];
    float noise_table[NOISE_TAB_SIZE];
    int noise_index;
    float noise_mult; /* XXX: suppress that and integrate it in the noise array */
    /* lsp_to_curve tables */
    float lsp_cos_table[BLOCK_MAX_SIZE];
    float lsp_pow_e_table[256];
    float lsp_pow_m_table1[(1 << LSP_POW_BITS)];
    float lsp_pow_m_table2[(1 << LSP_POW_BITS)];
    AVFloatDSPContext *fdsp;

    int eof_done; /* decode flag to output remaining samples after EOF */

#ifdef TRACE
    int frame_count;
#endif /* TRACE */
} WMACodecContext;

extern const uint8_t ff_wma_hgain_hufftab[37][2];
extern const float ff_wma_lsp_codebook[NB_LSP_COEFS][16];
extern const uint32_t ff_aac_scalefactor_code[121];
extern const uint8_t  ff_aac_scalefactor_bits[121];

av_warn_unused_result
int ff_wma_init(AVCodecContext *avctx, int flags2);

int ff_wma_total_gain_to_bits(int total_gain);
int ff_wma_end(AVCodecContext *avctx);
unsigned int ff_wma_get_large_val(GetBitContext *gb);
int ff_wma_run_level_decode(AVCodecContext *avctx, GetBitContext *gb,
                            const VLCElem *vlc, const float *level_table,
                            const uint16_t *run_table, int version,
                            WMACoef *ptr, int offset, int num_coefs,
                            int block_len, int frame_len_bits,
                            int coef_nb_bits);

#endif /* AVCODEC_WMA_H */
