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

#ifndef AVFILTER_ATADENOISE_H
#define AVFILTER_ATADENOISE_H

#include <stddef.h>
#include <stdint.h>

enum ATAAlgorithm {
    PARALLEL,
    SERIAL,
    NB_ATAA
};

typedef struct ATADenoiseDSPContext {
    void (*filter_row[4])(const uint8_t *src, uint8_t *dst,
                          const uint8_t **srcf,
                          int w, int mid, int size,
                          int thra, int thrb, const float *weight);
} ATADenoiseDSPContext;

void ff_atadenoise_init_x86(ATADenoiseDSPContext *dsp, int depth, int algorithm, const float *sigma);

#endif /* AVFILTER_ATADENOISE_H */
