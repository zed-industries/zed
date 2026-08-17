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

#ifndef AVCODEC_FLACDSP_H
#define AVCODEC_FLACDSP_H

#include <stdint.h>

#include "libavutil/samplefmt.h"

typedef struct FLACDSPContext {
    void (*decorrelate[4])(uint8_t **out, int32_t **in, int channels,
                           int len, int shift);
    void (*lpc16)(int32_t *samples, const int coeffs[32], int order,
                  int qlevel, int len);
    void (*lpc32)(int32_t *samples, const int coeffs[32], int order,
                  int qlevel, int len);
    void (*lpc33)(int64_t *samples, const int32_t *residual, const int coeffs[32],
                  int pred_order, int qlevel, int len);
    void (*wasted32)(int32_t *decoded, int wasted, int len);
    void (*wasted33)(int64_t *decoded, const int32_t *residual,
                     int wasted, int len);
} FLACDSPContext;

void ff_flacdsp_init(FLACDSPContext *c, enum AVSampleFormat fmt, int channels);
void ff_flacdsp_init_arm(FLACDSPContext *c, enum AVSampleFormat fmt, int channels);
void ff_flacdsp_init_riscv(FLACDSPContext *c, enum AVSampleFormat fmt, int channels);
void ff_flacdsp_init_x86(FLACDSPContext *c, enum AVSampleFormat fmt, int channels);

#endif /* AVCODEC_FLACDSP_H */
