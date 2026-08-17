/*
 * JPEG 2000 DSP functions
 * Copyright (c) 2007 Kamil Nowosad
 * Copyright (c) 2013 Nicolas Bertrand <nicoinattendu@gmail.com>
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

#ifndef AVCODEC_JPEG2000DSP_H
#define AVCODEC_JPEG2000DSP_H

#include <stdint.h>
#include "jpeg2000dwt.h"

typedef struct Jpeg2000DSPContext {
    void (*mct_decode[FF_DWT_NB])(void *src0, void *src1, void *src2, int csize);
} Jpeg2000DSPContext;

extern const float ff_jpeg2000_f_ict_params[4];

void ff_jpeg2000dsp_init(Jpeg2000DSPContext *c);
void ff_jpeg2000dsp_init_riscv(Jpeg2000DSPContext *c);
void ff_jpeg2000dsp_init_x86(Jpeg2000DSPContext *c);

#endif /* AVCODEC_JPEG2000DSP_H */
