/*
 * AAC encoder intensity stereo
 * Copyright (C) 2015 Rostislav Pehlivanov
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
 * AAC encoder Intensity Stereo
 * @author Rostislav Pehlivanov ( atomnuker gmail com )
 */

#ifndef AVCODEC_AACENC_IS_H
#define AVCODEC_AACENC_IS_H

#include "aacenc.h"

/** Frequency in Hz for lower limit of intensity stereo **/
#define INT_STEREO_LOW_LIMIT 6100

struct AACISError {
    int pass;    /* 1 if dist2 <= dist1  */
    int phase;   /* -1 or +1             */
    float error; /* fabs(dist1 - dist2)  */
    float dist1; /* From original coeffs */
    float dist2; /* From IS'd coeffs     */
    float ener01;
};

struct AACISError ff_aac_is_encoding_err(AACEncContext *s, ChannelElement *cpe,
                                         int start, int w, int g, float ener0,
                                         float ener1, float ener01,
                                         int use_pcoeffs, int phase);
void ff_aac_search_for_is(AACEncContext *s, AVCodecContext *avctx, ChannelElement *cpe);

#endif /* AVCODEC_AACENC_IS_H */
