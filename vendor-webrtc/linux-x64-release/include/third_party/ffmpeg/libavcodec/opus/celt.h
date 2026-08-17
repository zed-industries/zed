/*
 * Opus decoder/encoder CELT functions
 * Copyright (c) 2012 Andrew D'Addesio
 * Copyright (c) 2013-2014 Mozilla Corporation
 * Copyright (c) 2016 Rostislav Pehlivanov <atomnuker@gmail.com>
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

#ifndef AVCODEC_OPUS_CELT_H
#define AVCODEC_OPUS_CELT_H

#include <stdint.h>

#include "libavcodec/avcodec.h"

#include "dsp.h"
#include "rc.h"

#include "libavutil/float_dsp.h"
#include "libavutil/libm.h"
#include "libavutil/mem_internal.h"
#include "libavutil/tx.h"

#define CELT_SHORT_BLOCKSIZE         120
#define CELT_OVERLAP                 CELT_SHORT_BLOCKSIZE
#define CELT_MAX_LOG_BLOCKS          3
#define CELT_MAX_FRAME_SIZE          (CELT_SHORT_BLOCKSIZE * (1 << CELT_MAX_LOG_BLOCKS))
#define CELT_MAX_BANDS               21

#define CELT_VECTORS                 11
#define CELT_ALLOC_STEPS             6
#define CELT_FINE_OFFSET             21
#define CELT_MAX_FINE_BITS           8
#define CELT_NORM_SCALE              16384
#define CELT_QTHETA_OFFSET           4
#define CELT_QTHETA_OFFSET_TWOPHASE  16
#define CELT_POSTFILTER_MINPERIOD    15
#define CELT_ENERGY_SILENCE          (-28.0f)

enum CeltSpread {
    CELT_SPREAD_NONE,
    CELT_SPREAD_LIGHT,
    CELT_SPREAD_NORMAL,
    CELT_SPREAD_AGGRESSIVE
};

enum CeltBlockSize {
    CELT_BLOCK_120,
    CELT_BLOCK_240,
    CELT_BLOCK_480,
    CELT_BLOCK_960,

    CELT_BLOCK_NB
};

typedef struct CeltBlock {
    float energy[CELT_MAX_BANDS];
    float lin_energy[CELT_MAX_BANDS];
    float error_energy[CELT_MAX_BANDS];
    float prev_energy[2][CELT_MAX_BANDS];

    uint8_t collapse_masks[CELT_MAX_BANDS];

    /* buffer for mdct output + postfilter */
    DECLARE_ALIGNED(32, float, buf)[2048];
    DECLARE_ALIGNED(32, float, coeffs)[CELT_MAX_FRAME_SIZE];

    /* Used by the encoder */
    DECLARE_ALIGNED(32, float, overlap)[FFALIGN(CELT_OVERLAP, 16)];
    DECLARE_ALIGNED(32, float, samples)[FFALIGN(CELT_MAX_FRAME_SIZE, 16)];

    /* postfilter parameters */
    int   pf_period_new;
    float pf_gains_new[3];
    int   pf_period;
    float pf_gains[3];
    int   pf_period_old;
    float pf_gains_old[3];

    float emph_coeff;
} CeltBlock;

typedef struct CeltFrame {
    // constant values that do not change during context lifetime
    AVCodecContext      *avctx;
    AVTXContext        *tx[4];
    av_tx_fn            tx_fn[4];
    AVFloatDSPContext   *dsp;
    CeltBlock           block[2];
    struct CeltPVQ      *pvq;
    OpusDSP             opusdsp;
    int channels;
    int output_channels;
    int apply_phase_inv;

    enum CeltBlockSize size;
    int start_band;
    int end_band;
    int coded_bands;
    int transient;
    int pfilter;
    int skip_band_floor;
    int tf_select;
    int alloc_trim;
    int alloc_boost[CELT_MAX_BANDS];
    int blocks;        /* number of iMDCT blocks in the frame, depends on transient */
    int blocksize;     /* size of each block */
    int silence;       /* Frame is filled with silence */
    int anticollapse_needed; /* Whether to expect an anticollapse bit */
    int anticollapse;  /* Encoded anticollapse bit */
    int intensity_stereo;
    int dual_stereo;
    int flushed;
    uint32_t seed;
    enum CeltSpread spread;

    /* Encoder PF coeffs */
    int pf_octave;
    int pf_period;
    int pf_tapset;
    float pf_gain;

    /* Bit allocation */
    int framebits;
    int remaining;
    int remaining2;
    int caps         [CELT_MAX_BANDS];
    int fine_bits    [CELT_MAX_BANDS];
    int fine_priority[CELT_MAX_BANDS];
    int pulses       [CELT_MAX_BANDS];
    int tf_change    [CELT_MAX_BANDS];
} CeltFrame;

/* LCG for noise generation */
static av_always_inline uint32_t celt_rng(CeltFrame *f)
{
    f->seed = 1664525 * f->seed + 1013904223;
    return f->seed;
}

static av_always_inline void celt_renormalize_vector(float *X, int N, float gain)
{
    int i;
    float g = 1e-15f;
    for (i = 0; i < N; i++)
        g += X[i] * X[i];
    g = gain / sqrtf(g);

    for (i = 0; i < N; i++)
        X[i] *= g;
}

int ff_celt_init(AVCodecContext *avctx, CeltFrame **f, int output_channels,
                 int apply_phase_inv);

void ff_celt_free(CeltFrame **f);

void ff_celt_flush(CeltFrame *f);

int ff_celt_decode_frame(CeltFrame *f, OpusRangeCoder *rc, float **output,
                         int coded_channels, int frame_size, int startband, int endband);

/* Encode or decode CELT bands */
void ff_celt_quant_bands(CeltFrame *f, OpusRangeCoder *rc);

/* Encode or decode CELT bitallocation */
void ff_celt_bitalloc(CeltFrame *f, OpusRangeCoder *rc, int encode);

#endif /* AVCODEC_OPUS_CELT_H */
