/*
 * Apple ProRes compatible decoder
 *
 * Copyright (c) 2010-2011 Maxim Poliakovski
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

#ifndef AVCODEC_PRORESDSP_H
#define AVCODEC_PRORESDSP_H

#include <stddef.h>
#include <stdint.h>

typedef struct ProresDSPContext {
    int idct_permutation_type;
    uint8_t idct_permutation[64];
    void (*idct_put)(uint16_t *out, ptrdiff_t linesize, int16_t *block, const int16_t *qmat);
} ProresDSPContext;

void ff_proresdsp_init(ProresDSPContext *dsp, int bits_per_raw_sample);

void ff_proresdsp_init_x86(ProresDSPContext *dsp, int bits_per_raw_sample);

#endif /* AVCODEC_PRORESDSP_H */
