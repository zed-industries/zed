/*
 * Copyright (c) 2015 Ronald S. Bultje <rsbultje@gmail.com>
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

#ifndef AVFILTER_SSIM_H
#define AVFILTER_SSIM_H

#include <stddef.h>
#include <stdint.h>

typedef struct SSIMDSPContext {
    void (*ssim_4x4_line)(const uint8_t *buf, ptrdiff_t buf_stride,
                          const uint8_t *ref, ptrdiff_t ref_stride,
                          int (*sums)[4], int w);
    double (*ssim_end_line)(const int (*sum0)[4], const int (*sum1)[4], int w);
} SSIMDSPContext;

void ff_ssim_init_x86(SSIMDSPContext *dsp);

#endif /* AVFILTER_SSIM_H */
