/*
 * G.723.1 common header and data tables
 * Copyright (c) 2006 Benjamin Larsson
 * Copyright (c) 2010 Mohamed Naufal Basheer
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
 * G.723.1 types, functions and data tables
 */

#ifndef AVCODEC_G723_1_H
#define AVCODEC_G723_1_H

#include <stdint.h>

#include "libavutil/log.h"

#define SUBFRAMES       4
#define SUBFRAME_LEN    60
#define FRAME_LEN       (SUBFRAME_LEN << 2)
#define HALF_FRAME_LEN  (FRAME_LEN / 2)
#define LPC_FRAME       (HALF_FRAME_LEN + SUBFRAME_LEN)
#define LPC_ORDER       10
#define LSP_BANDS       3
#define LSP_CB_SIZE     256
#define PITCH_MIN       18
#define PITCH_MAX       (PITCH_MIN + 127)
#define PITCH_ORDER     5
#define GRID_SIZE       2
#define PULSE_MAX       6
#define GAIN_LEVELS     24
#define COS_TBL_SIZE    512

/**
 * Bitexact implementation of 2ab scaled by 1/2^16.
 *
 * @param a 32 bit multiplicand
 * @param b 16 bit multiplier
 */
#define MULL2(a, b) \
        ((((a) >> 16) * (b) * 2) + (((a) & 0xffff) * (b) >> 15))

/**
 * G723.1 frame types
 */
enum FrameType {
    ACTIVE_FRAME,        ///< Active speech
    SID_FRAME,           ///< Silence Insertion Descriptor frame
    UNTRANSMITTED_FRAME
};

/**
 * G723.1 rate values
 */
enum Rate {
    RATE_6300,
    RATE_5300
};

/**
 * G723.1 unpacked data subframe
 */
typedef struct G723_1_Subframe {
    int ad_cb_lag;     ///< adaptive codebook lag
    int ad_cb_gain;
    int dirac_train;
    int pulse_sign;
    int grid_index;
    int amp_index;
    int pulse_pos;
} G723_1_Subframe;

/**
 * Pitch postfilter parameters
 */
typedef struct PPFParam {
    int     index;    ///< postfilter backward/forward lag
    int16_t opt_gain; ///< optimal gain
    int16_t sc_gain;  ///< scaling gain
} PPFParam;

/**
 * Harmonic filter parameters
 */
typedef struct HFParam {
    int index;
    int gain;
} HFParam;

/**
 * Optimized fixed codebook excitation parameters
 */
typedef struct FCBParam {
    int min_err;
    int amp_index;
    int grid_index;
    int dirac_train;
    int pulse_pos[PULSE_MAX];
    int pulse_sign[PULSE_MAX];
} FCBParam;

typedef struct G723_1_ChannelContext {
    G723_1_Subframe subframe[4];
    enum FrameType cur_frame_type;
    enum FrameType past_frame_type;
    enum Rate cur_rate;
    uint8_t lsp_index[LSP_BANDS];
    int pitch_lag[2];
    int erased_frames;

    int16_t prev_lsp[LPC_ORDER];
    int16_t sid_lsp[LPC_ORDER];
    int16_t prev_excitation[PITCH_MAX];
    int16_t excitation[PITCH_MAX + FRAME_LEN + 4];
    int16_t synth_mem[LPC_ORDER];
    int16_t fir_mem[LPC_ORDER];
    int     iir_mem[LPC_ORDER];

    int random_seed;
    int cng_random_seed;
    int interp_index;
    int interp_gain;
    int sid_gain;
    int cur_gain;
    int reflection_coef;
    int pf_gain;                 ///< formant postfilter
                                 ///< gain scaling unit memory
    int16_t audio[FRAME_LEN + LPC_ORDER + PITCH_MAX + 4];

    /* encoder */
    int16_t prev_data[HALF_FRAME_LEN];
    int16_t prev_weight_sig[PITCH_MAX];

    int16_t hpf_fir_mem;                   ///< highpass filter fir
    int     hpf_iir_mem;                   ///< and iir memories
    int16_t perf_fir_mem[LPC_ORDER];       ///< perceptual filter fir
    int16_t perf_iir_mem[LPC_ORDER];       ///< and iir memories

    int16_t harmonic_mem[PITCH_MAX];
} G723_1_ChannelContext;

typedef struct G723_1_Context {
    AVClass *class;
    int postfilter;

    G723_1_ChannelContext ch[2];
} G723_1_Context;


/**
 * Scale vector contents based on the largest of their absolutes.
 */
int ff_g723_1_scale_vector(int16_t *dst, const int16_t *vector, int length);

/**
 * Calculate the number of left-shifts required for normalizing the input.
 *
 * @param num   input number
 * @param width width of the input, 16 bits(0) / 32 bits(1)
 */
int ff_g723_1_normalize_bits(int num, int width);

int ff_g723_1_dot_product(const int16_t *a, const int16_t *b, int length);

/**
 * Get delayed contribution from the previous excitation vector.
 */
void ff_g723_1_get_residual(int16_t *residual, int16_t *prev_excitation,
                            int lag);

/**
 * Generate a train of dirac functions with period as pitch lag.
 */
void ff_g723_1_gen_dirac_train(int16_t *buf, int pitch_lag);


/**
 * Generate adaptive codebook excitation.
 */
void ff_g723_1_gen_acb_excitation(int16_t *vector, int16_t *prev_excitation,
                                  int pitch_lag, G723_1_Subframe *subfrm,
                                  enum Rate cur_rate);
/**
 * Quantize LSP frequencies by interpolation and convert them to
 * the corresponding LPC coefficients.
 *
 * @param lpc      buffer for LPC coefficients
 * @param cur_lsp  the current LSP vector
 * @param prev_lsp the previous LSP vector
 */
void ff_g723_1_lsp_interpolate(int16_t *lpc, int16_t *cur_lsp,
                               int16_t *prev_lsp);

/**
 * Perform inverse quantization of LSP frequencies.
 *
 * @param cur_lsp    the current LSP vector
 * @param prev_lsp   the previous LSP vector
 * @param lsp_index  VQ indices
 * @param bad_frame  bad frame flag
 */
void ff_g723_1_inverse_quant(int16_t *cur_lsp, int16_t *prev_lsp,
                             uint8_t *lsp_index, int bad_frame);

static const uint8_t frame_size[4] = { 24, 20, 4, 1 };

/**
 * LSP DC component
 */
static const int16_t dc_lsp[LPC_ORDER] = {
    0x0c3b,
    0x1271,
    0x1e0a,
    0x2a36,
    0x3630,
    0x406f,
    0x4d28,
    0x56f4,
    0x638c,
    0x6c46
};

/* Cosine table scaled by 2^14 */
extern const int16_t ff_g723_1_cos_tab[COS_TBL_SIZE + 1];
#define G723_1_COS_TAB_FIRST_ELEMENT 16384

/**
 *  LSP VQ tables
 */
extern const int16_t ff_g723_1_lsp_band0[LSP_CB_SIZE][3];
extern const int16_t ff_g723_1_lsp_band1[LSP_CB_SIZE][3];
extern const int16_t ff_g723_1_lsp_band2[LSP_CB_SIZE][4];

/**
 * Used for the coding/decoding of the pulses positions
 * for the MP-MLQ codebook
 */
extern const int32_t ff_g723_1_combinatorial_table[PULSE_MAX][SUBFRAME_LEN/GRID_SIZE];

/**
 * Number of non-zero pulses in the MP-MLQ excitation
 */
static const int8_t pulses[4] = {6, 5, 6, 5};

extern const int16_t ff_g723_1_fixed_cb_gain[GAIN_LEVELS];

extern const int16_t ff_g723_1_adaptive_cb_gain85 [ 85 * 20];
extern const int16_t ff_g723_1_adaptive_cb_gain170[170 * 20];

#endif /* AVCODEC_G723_1_H */
