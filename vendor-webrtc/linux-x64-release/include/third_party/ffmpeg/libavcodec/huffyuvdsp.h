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

#ifndef AVCODEC_HUFFYUVDSP_H
#define AVCODEC_HUFFYUVDSP_H

#include <stdint.h>
#include "libavutil/pixfmt.h"

typedef struct HuffYUVDSPContext {
    void (*add_int16)(uint16_t *dst/*align 16*/, const uint16_t *src/*align 16*/,
                      unsigned mask, int w);

    void (*add_hfyu_median_pred_int16)(uint16_t *dst, const uint16_t *top,
                                       const uint16_t *diff, unsigned mask,
                                       int w, int *left, int *left_top);
    void (*add_hfyu_left_pred_bgr32)(uint8_t *dst, const uint8_t *src,
                                     intptr_t w, uint8_t *left);
} HuffYUVDSPContext;

void ff_huffyuvdsp_init(HuffYUVDSPContext *c, enum AVPixelFormat pix_fmt);
void ff_huffyuvdsp_init_riscv(HuffYUVDSPContext *c,
                              enum AVPixelFormat pix_fmt);
void ff_huffyuvdsp_init_x86(HuffYUVDSPContext *c, enum AVPixelFormat pix_fmt);

#endif /* AVCODEC_HUFFYUVDSP_H */
