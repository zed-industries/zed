/*
 * Copyright (c) 2016 Paul B Mahol
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

#ifndef AVFILTER_THRESHOLD_INIT_H
#define AVFILTER_THRESHOLD_INIT_H

#include <stdint.h>
#include <stddef.h>

#include "config.h"
#include "libavutil/attributes.h"
#include "threshold.h"

static void threshold8(const uint8_t *in, const uint8_t *threshold,
                       const uint8_t *min, const uint8_t *max,
                       uint8_t *out,
                       ptrdiff_t ilinesize, ptrdiff_t tlinesize,
                       ptrdiff_t flinesize, ptrdiff_t slinesize,
                       ptrdiff_t olinesize,
                       int w, int h)
{
    for (int y = 0; y < h; y++) {
        for (int x = 0; x < w; x++)
            out[x] = in[x] <= threshold[x] ? min[x] : max[x];

        in        += ilinesize;
        threshold += tlinesize;
        min       += flinesize;
        max       += slinesize;
        out       += olinesize;
    }
}

static void threshold16(const uint8_t *iin, const uint8_t *tthreshold,
                        const uint8_t *ffirst, const uint8_t *ssecond,
                        uint8_t *oout,
                        ptrdiff_t ilinesize, ptrdiff_t tlinesize,
                        ptrdiff_t flinesize, ptrdiff_t slinesize,
                        ptrdiff_t olinesize,
                        int w, int h)
{
    const uint16_t *in = (const uint16_t *)iin;
    const uint16_t *threshold = (const uint16_t *)tthreshold;
    const uint16_t *min = (const uint16_t *)ffirst;
    const uint16_t *max = (const uint16_t *)ssecond;
    uint16_t *out = (uint16_t *)oout;

    for (int y = 0; y < h; y++) {
        for (int x = 0; x < w; x++)
            out[x] = in[x] <= threshold[x] ? min[x] : max[x];

        in        += ilinesize / 2;
        threshold += tlinesize / 2;
        min       += flinesize / 2;
        max       += slinesize / 2;
        out       += olinesize / 2;
    }
}

static av_unused void ff_threshold_init(ThresholdContext *s)
{
    if (s->depth == 8) {
        s->threshold = threshold8;
        s->bpc = 1;
    } else {
        s->threshold = threshold16;
        s->bpc = 2;
    }

#if ARCH_X86
    ff_threshold_init_x86(s);
#endif
}

#endif /* AVFILTER_THRESHOLD_INIT_H */
