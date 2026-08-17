/*
 * AOM film grain synthesis
 * Copyright (c) 2021 Niklas Haas <ffmpeg@haasn.xyz>
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
 * AOM film grain synthesis.
 * @author Niklas Haas <ffmpeg@haasn.xyz>
 */

#ifndef AVCODEC_AOM_FILM_GRAIN_H
#define AVCODEC_AOM_FILM_GRAIN_H

#include "libavutil/buffer.h"
#include "libavutil/film_grain_params.h"

typedef struct AVFilmGrainAFGS1Params {
    int enable;
    AVBufferRef *sets[8];
} AVFilmGrainAFGS1Params;

// Synthesizes film grain on top of `in` and stores the result to `out`. `out`
// must already have been allocated and set to the same size and format as `in`.
int ff_aom_apply_film_grain(AVFrame *out, const AVFrame *in,
                            const AVFilmGrainParams *params);

// Parse AFGS1 parameter sets from an ITU-T T.35 payload. Returns 0 on success,
// or a negative error code.
int ff_aom_parse_film_grain_sets(AVFilmGrainAFGS1Params *s,
                                 const uint8_t *payload, int payload_size);

// Attach all valid film grain param sets to `frame`.
int ff_aom_attach_film_grain_sets(const AVFilmGrainAFGS1Params *s, AVFrame *frame);

// Free all allocations in `s` and zero the entire struct.
void ff_aom_uninit_film_grain_params(AVFilmGrainAFGS1Params *s);

#endif /* AVCODEC_AOM_FILM_GRAIN_H */
