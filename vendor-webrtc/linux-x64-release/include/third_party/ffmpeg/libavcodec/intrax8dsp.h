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

#ifndef AVCODEC_INTRAX8DSP_H
#define AVCODEC_INTRAX8DSP_H

#include <stddef.h>
#include <stdint.h>

typedef struct IntraX8DSPContext {
    void (*v_loop_filter)(uint8_t *src, ptrdiff_t stride, int qscale);
    void (*h_loop_filter)(uint8_t *src, ptrdiff_t stride, int qscale);

    void (*spatial_compensation[12])(const uint8_t *restrict src, uint8_t *restrict dst,
                                     ptrdiff_t stride);
    void (*setup_spatial_compensation)(const uint8_t *restrict src, uint8_t *restrict dst,
                                       ptrdiff_t stride, int *range,
                                       int *sum, int edges);
} IntraX8DSPContext;

void ff_intrax8dsp_init(IntraX8DSPContext *dsp);

#endif /* AVCODEC_INTRAX8DSP_H */
