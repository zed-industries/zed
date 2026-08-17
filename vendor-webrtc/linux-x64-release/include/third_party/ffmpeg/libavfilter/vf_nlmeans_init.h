/*
 * Copyright (c) 2016 Clément Bœsch <u pkh me>
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

#ifndef AVFILTER_NLMEANS_INIT_H
#define AVFILTER_NLMEANS_INIT_H

#include <stddef.h>
#include <stdint.h>

#include "config.h"
#include "libavutil/avassert.h"
#include "libavutil/macros.h"
#include "vf_nlmeans.h"

/**
 * Compute squared difference of the safe area (the zone where s1 and s2
 * overlap). It is likely the largest integral zone, so it is interesting to do
 * as little checks as possible; contrary to the unsafe version of this
 * function, we do not need any clipping here.
 *
 * The line above dst and the column to its left are always readable.
 */
static void compute_safe_ssd_integral_image_c(uint32_t *dst, ptrdiff_t dst_linesize_32,
                                              const uint8_t *s1, ptrdiff_t linesize1,
                                              const uint8_t *s2, ptrdiff_t linesize2,
                                              int w, int h)
{
    const uint32_t *dst_top = dst - dst_linesize_32;

    /* SIMD-friendly assumptions allowed here */
    av_assert2(!(w & 0xf) && w >= 16 && h >= 1);

    for (int y = 0; y < h; y++) {
        for (int x = 0; x < w; x += 4) {
            const int d0 = s1[x    ] - s2[x    ];
            const int d1 = s1[x + 1] - s2[x + 1];
            const int d2 = s1[x + 2] - s2[x + 2];
            const int d3 = s1[x + 3] - s2[x + 3];

            dst[x    ] = dst_top[x    ] - dst_top[x - 1] + d0*d0;
            dst[x + 1] = dst_top[x + 1] - dst_top[x    ] + d1*d1;
            dst[x + 2] = dst_top[x + 2] - dst_top[x + 1] + d2*d2;
            dst[x + 3] = dst_top[x + 3] - dst_top[x + 2] + d3*d3;

            dst[x    ] += dst[x - 1];
            dst[x + 1] += dst[x    ];
            dst[x + 2] += dst[x + 1];
            dst[x + 3] += dst[x + 2];
        }
        s1  += linesize1;
        s2  += linesize2;
        dst += dst_linesize_32;
        dst_top += dst_linesize_32;
    }
}

static void compute_weights_line_c(const uint32_t *const iia,
                                   const uint32_t *const iib,
                                   const uint32_t *const iid,
                                   const uint32_t *const iie,
                                   const uint8_t *const src,
                                   float *total_weight,
                                   float *sum,
                                   const float *const weight_lut,
                                   int max_meaningful_diff,
                                   int startx, int endx)
{
    for (int x = startx; x < endx; x++) {
        /*
         * M is a discrete map where every entry contains the sum of all the entries
         * in the rectangle from the top-left origin of M to its coordinate. In the
         * following schema, "i" contains the sum of the whole map:
         *
         * M = +----------+-----------------+----+
         *     |          |                 |    |
         *     |          |                 |    |
         *     |         a|                b|   c|
         *     +----------+-----------------+----+
         *     |          |                 |    |
         *     |          |                 |    |
         *     |          |        X        |    |
         *     |          |                 |    |
         *     |         d|                e|   f|
         *     +----------+-----------------+----+
         *     |          |                 |    |
         *     |         g|                h|   i|
         *     +----------+-----------------+----+
         *
         * The sum of the X box can be calculated with:
         *    X = e-d-b+a
         *
         * See https://en.wikipedia.org/wiki/Summed_area_table
         *
         * The compute*_ssd functions compute the integral image M where every entry
         * contains the sum of the squared difference of every corresponding pixels of
         * two input planes of the same size as M.
         */
        const uint32_t a = iia[x];
        const uint32_t b = iib[x];
        const uint32_t d = iid[x];
        const uint32_t e = iie[x];
        const uint32_t patch_diff_sq = FFMIN(e - d - b + a, max_meaningful_diff);
        const float weight = weight_lut[patch_diff_sq]; // exp(-patch_diff_sq * s->pdiff_scale)

        total_weight[x] += weight;
        sum[x] += weight * src[x];
    }
}

static av_unused void ff_nlmeans_init(NLMeansDSPContext *dsp)
{
    dsp->compute_safe_ssd_integral_image = compute_safe_ssd_integral_image_c;
    dsp->compute_weights_line = compute_weights_line_c;

#if ARCH_AARCH64
    ff_nlmeans_init_aarch64(dsp);
#elif ARCH_X86
    ff_nlmeans_init_x86(dsp);
#endif
}

#endif /* AVFILTER_NLMEANS_INIT_H */
