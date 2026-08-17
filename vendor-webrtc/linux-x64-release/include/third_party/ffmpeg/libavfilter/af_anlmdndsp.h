/*
 * Copyright (c) 2019 Paul B Mahol
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

#ifndef AVFILTER_ANLMDNDSP_H
#define AVFILTER_ANLMDNDSP_H

#include "libavutil/common.h"

#include "audio.h"
#include "avfilter.h"
#include "formats.h"

typedef struct AudioNLMDNDSPContext {
    float (*compute_distance_ssd)(const float *f1, const float *f2, ptrdiff_t K);
    void (*compute_cache)(float *cache, const float *f, ptrdiff_t S, ptrdiff_t K,
                          ptrdiff_t i, ptrdiff_t jj);
} AudioNLMDNDSPContext;

void ff_anlmdn_init(AudioNLMDNDSPContext *s);
void ff_anlmdn_init_x86(AudioNLMDNDSPContext *s);

#endif /* AVFILTER_ANLMDNDSP_H */
