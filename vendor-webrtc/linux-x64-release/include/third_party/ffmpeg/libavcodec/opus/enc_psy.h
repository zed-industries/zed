/*
 * Opus encoder
 * Copyright (c) 2017 Rostislav Pehlivanov <atomnuker@gmail.com>
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

#ifndef AVCODEC_OPUS_ENC_PSY_H
#define AVCODEC_OPUS_ENC_PSY_H

#include "libavutil/tx.h"
#include "libavutil/mem_internal.h"

#include "enc.h"
#include "celt.h"
#include "enc_utils.h"

/* Each step is 2.5ms */
typedef struct OpusPsyStep {
    int   index; /* Current index */
    int   silence;
    float energy[OPUS_MAX_CHANNELS][CELT_MAX_BANDS]; /* Masking effects included */
    float tone[OPUS_MAX_CHANNELS][CELT_MAX_BANDS];   /* Tonality */
    float stereo[CELT_MAX_BANDS];                    /* IS/MS compatibility */
    float change_amp[OPUS_MAX_CHANNELS][CELT_MAX_BANDS]; /* Jump over last frame */
    float total_change; /* Total change */

    float *bands[OPUS_MAX_CHANNELS][CELT_MAX_BANDS];
    float coeffs[OPUS_MAX_CHANNELS][OPUS_BLOCK_SIZE(CELT_BLOCK_960)];
} OpusPsyStep;

typedef struct OpusBandExcitation {
    float excitation;
    float excitation_dist;
    float excitation_init;
} OpusBandExcitation;

typedef struct OpusPsyContext {
    AVCodecContext *avctx;
    AVFloatDSPContext *dsp;
    struct FFBufQueue *bufqueue;
    OpusEncOptions *options;

    OpusBandExcitation ex[OPUS_MAX_CHANNELS][CELT_MAX_BANDS];
    FFBesselFilter bfilter_lo[OPUS_MAX_CHANNELS][CELT_MAX_BANDS];
    FFBesselFilter bfilter_hi[OPUS_MAX_CHANNELS][CELT_MAX_BANDS];

    OpusPsyStep *steps[FF_BUFQUEUE_SIZE + 1];
    int max_steps;

    float *window[CELT_BLOCK_NB];
    AVTXContext *mdct[CELT_BLOCK_NB];
    av_tx_fn mdct_fn[CELT_BLOCK_NB];
    int bsize_analysis;

    DECLARE_ALIGNED(32, float, scratch)[2048];

    /* Stats */
    float avg_is_band;
    int64_t dual_stereo_used;
    int64_t total_packets_out;

    /* State */
    OpusPacketInfo p;
    int buffered_steps;
    int steps_to_process;
    int eof;
    float lambda;
    int *inflection_points;
    int inflection_points_count;
} OpusPsyContext;

int  ff_opus_psy_process           (OpusPsyContext *s, OpusPacketInfo *p);
void ff_opus_psy_celt_frame_init   (OpusPsyContext *s, CeltFrame *f, int index);
int  ff_opus_psy_celt_frame_process(OpusPsyContext *s, CeltFrame *f, int index);
void ff_opus_psy_postencode_update (OpusPsyContext *s, CeltFrame *f);

int  ff_opus_psy_init(OpusPsyContext *s, AVCodecContext *avctx,
                      struct FFBufQueue *bufqueue, OpusEncOptions *options);
void ff_opus_psy_signal_eof(OpusPsyContext *s);
int  ff_opus_psy_end(OpusPsyContext *s);

#endif /* AVCODEC_OPUS_ENC_PSY_H */
