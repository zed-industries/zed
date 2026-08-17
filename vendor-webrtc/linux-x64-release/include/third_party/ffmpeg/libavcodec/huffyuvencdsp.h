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

#ifndef AVCODEC_HUFFYUVENCDSP_H
#define AVCODEC_HUFFYUVENCDSP_H

#include <stdint.h>

#include "libavutil/pixfmt.h"

typedef struct HuffYUVEncDSPContext {
    void (*diff_int16)(uint16_t *dst /* align 16 */,
                       const uint16_t *src1 /* align 16 */,
                       const uint16_t *src2 /* align 1 */,
                       unsigned mask, int w);

    void (*sub_hfyu_median_pred_int16)(uint16_t *dst, const uint16_t *src1,
                                       const uint16_t *src2, unsigned mask,
                                       int w, int *left, int *left_top);
} HuffYUVEncDSPContext;

void ff_huffyuvencdsp_init(HuffYUVEncDSPContext *c, enum AVPixelFormat pix_fmt);
void ff_huffyuvencdsp_init_x86(HuffYUVEncDSPContext *c, enum AVPixelFormat pix_fmt);

#endif /* AVCODEC_HUFFYUVENCDSP_H */
