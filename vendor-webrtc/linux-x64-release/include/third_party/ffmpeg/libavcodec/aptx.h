/*
 * Audio Processing Technology codec for Bluetooth (aptX)
 *
 * Copyright (C) 2017  Aurelien Jacobs <aurel@gnuage.org>
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

#ifndef AVCODEC_APTX_H
#define AVCODEC_APTX_H

#include "libavutil/intreadwrite.h"
#include "avcodec.h"
#include "mathops.h"


enum channels {
    LEFT,
    RIGHT,
    NB_CHANNELS
};

enum subbands {
    LF,  // Low Frequency (0-5.5 kHz)
    MLF, // Medium-Low Frequency (5.5-11kHz)
    MHF, // Medium-High Frequency (11-16.5kHz)
    HF,  // High Frequency (16.5-22kHz)
    NB_SUBBANDS
};

#define NB_FILTERS 2
#define FILTER_TAPS 16

typedef struct {
    int pos;
    int32_t buffer[2*FILTER_TAPS];
} FilterSignal;

typedef struct {
    FilterSignal outer_filter_signal[NB_FILTERS];
    FilterSignal inner_filter_signal[NB_FILTERS][NB_FILTERS];
} QMFAnalysis;

typedef struct {
    int32_t quantized_sample;
    int32_t quantized_sample_parity_change;
    int32_t error;
} Quantize;

typedef struct {
    int32_t quantization_factor;
    int32_t factor_select;
    int32_t reconstructed_difference;
} InvertQuantize;

typedef struct {
    int32_t prev_sign[2];
    int32_t s_weight[2];
    int32_t d_weight[24];
    int32_t pos;
    int32_t reconstructed_differences[48];
    int32_t previous_reconstructed_sample;
    int32_t predicted_difference;
    int32_t predicted_sample;
} Prediction;

typedef struct {
    int32_t codeword_history;
    int32_t dither_parity;
    int32_t dither[NB_SUBBANDS];

    QMFAnalysis qmf;
    Quantize quantize[NB_SUBBANDS];
    InvertQuantize invert_quantize[NB_SUBBANDS];
    Prediction prediction[NB_SUBBANDS];
} Channel;

typedef struct {
    int hd;
    int block_size;
    int32_t sync_idx;
    Channel channels[NB_CHANNELS];
} AptXContext;

typedef const struct {
    const int32_t *quantize_intervals;
    const int32_t *invert_quantize_dither_factors;
    const int32_t *quantize_dither_factors;
    const int16_t *quantize_factor_select_offset;
    int tables_size;
    int32_t factor_max;
    int32_t prediction_order;
} ConstTables;

extern ConstTables ff_aptx_quant_tables[2][NB_SUBBANDS];

/* Rounded right shift with optionnal clipping */
#define RSHIFT_SIZE(size)                                                     \
av_always_inline                                                              \
static int##size##_t rshift##size(int##size##_t value, int shift)             \
{                                                                             \
    int##size##_t rounding = (int##size##_t)1 << (shift - 1);                 \
    int##size##_t mask = ((int##size##_t)1 << (shift + 1)) - 1;               \
    return ((value + rounding) >> shift) - ((value & mask) == rounding);      \
}                                                                             \
av_always_inline                                                              \
static int##size##_t rshift##size##_clip24(int##size##_t value, int shift)    \
{                                                                             \
    return av_clip_intp2(rshift##size(value, shift), 23);                     \
}
RSHIFT_SIZE(32)
RSHIFT_SIZE(64)

/*
 * Convolution filter coefficients for the outer QMF of the QMF tree.
 * The 2 sets are a mirror of each other.
 */
static const int32_t aptx_qmf_outer_coeffs[NB_FILTERS][FILTER_TAPS] = {
    {
        730, -413, -9611, 43626, -121026, 269973, -585547, 2801966,
        697128, -160481, 27611, 8478, -10043, 3511, 688, -897,
    },
    {
        -897, 688, 3511, -10043, 8478, 27611, -160481, 697128,
        2801966, -585547, 269973, -121026, 43626, -9611, -413, 730,
    },
};

/*
 * Convolution filter coefficients for the inner QMF of the QMF tree.
 * The 2 sets are a mirror of each other.
 */
static const int32_t aptx_qmf_inner_coeffs[NB_FILTERS][FILTER_TAPS] = {
    {
       1033, -584, -13592, 61697, -171156, 381799, -828088, 3962579,
       985888, -226954, 39048, 11990, -14203, 4966, 973, -1268,
    },
    {
      -1268, 973, 4966, -14203, 11990, 39048, -226954, 985888,
      3962579, -828088, 381799, -171156, 61697, -13592, -584, 1033,
    },
};

/*
 * Push one sample into a circular signal buffer.
 */
av_always_inline
static void aptx_qmf_filter_signal_push(FilterSignal *signal, int32_t sample)
{
    signal->buffer[signal->pos            ] = sample;
    signal->buffer[signal->pos+FILTER_TAPS] = sample;
    signal->pos = (signal->pos + 1) & (FILTER_TAPS - 1);
}

/*
 * Compute the convolution of the signal with the coefficients, and reduce
 * to 24 bits by applying the specified right shifting.
 */
av_always_inline
static int32_t aptx_qmf_convolution(FilterSignal *signal,
                                    const int32_t coeffs[FILTER_TAPS],
                                    int shift)
{
    int32_t *sig = &signal->buffer[signal->pos];
    int64_t e = 0;
    int i;

    for (i = 0; i < FILTER_TAPS; i++)
        e += MUL64(sig[i], coeffs[i]);

    return rshift64_clip24(e, shift);
}

static inline int32_t aptx_quantized_parity(Channel *channel)
{
    int32_t parity = channel->dither_parity;
    int subband;

    for (subband = 0; subband < NB_SUBBANDS; subband++)
        parity ^= channel->quantize[subband].quantized_sample;

    return parity & 1;
}

/* For each sample, ensure that the parity of all subbands of all channels
 * is 0 except once every 8 samples where the parity is forced to 1. */
static inline int aptx_check_parity(Channel channels[NB_CHANNELS], int32_t *idx)
{
    int32_t parity = aptx_quantized_parity(&channels[LEFT])
                   ^ aptx_quantized_parity(&channels[RIGHT]);

    int eighth = *idx == 7;
    *idx = (*idx + 1) & 7;

    return parity ^ eighth;
}

void ff_aptx_invert_quantize_and_prediction(Channel *channel, int hd);
void ff_aptx_generate_dither(Channel *channel);

int ff_aptx_init(AVCodecContext *avctx);

#endif /* AVCODEC_APTX_H */
