/*
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
 * audio volume filter
 */

#ifndef AVFILTER_VOLUME_H
#define AVFILTER_VOLUME_H

#include <stdint.h>
#include "libavutil/eval.h"
#include "libavutil/float_dsp.h"
#include "libavutil/log.h"
#include "libavutil/samplefmt.h"

enum PrecisionType {
    PRECISION_FIXED = 0,
    PRECISION_FLOAT,
    PRECISION_DOUBLE,
};

enum EvalMode {
    EVAL_MODE_ONCE,
    EVAL_MODE_FRAME,
    EVAL_MODE_NB
};

enum VolumeVarName {
    VAR_N,
    VAR_NB_CHANNELS,
    VAR_NB_CONSUMED_SAMPLES,
    VAR_NB_SAMPLES,
#if FF_API_FRAME_PKT
    VAR_POS,
#endif
    VAR_PTS,
    VAR_SAMPLE_RATE,
    VAR_STARTPTS,
    VAR_STARTT,
    VAR_T,
    VAR_TB,
    VAR_VOLUME,
    VAR_VARS_NB
};

enum ReplayGainType {
    REPLAYGAIN_DROP,
    REPLAYGAIN_IGNORE,
    REPLAYGAIN_TRACK,
    REPLAYGAIN_ALBUM,
};

typedef struct VolumeContext {
    const AVClass *class;
    AVFloatDSPContext *fdsp;
    int precision;
    int eval_mode;
    const char *volume_expr;
    AVExpr *volume_pexpr;
    double var_values[VAR_VARS_NB];

    int replaygain;
    double replaygain_preamp;
    int    replaygain_noclip;
    double volume;
    int    volume_i;
    int    channels;
    int    planes;
    enum AVSampleFormat sample_fmt;

    void (*scale_samples)(uint8_t *dst, const uint8_t *src, int nb_samples,
                          int volume);
    int samples_align;
} VolumeContext;

void ff_volume_init_x86(VolumeContext *vol);

#endif /* AVFILTER_VOLUME_H */
