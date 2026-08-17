/*
 * TTA (The Lossless True Audio) data
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

#ifndef AVCODEC_TTADATA_H
#define AVCODEC_TTADATA_H

#include <stdint.h>

#define MAX_ORDER 16
typedef struct TTAFilter {
    int32_t shift, round, error;
    int32_t qm[MAX_ORDER];
    int32_t dx[MAX_ORDER];
    int32_t dl[MAX_ORDER];
} TTAFilter;

typedef struct TTARice {
    uint32_t k0, k1, sum0, sum1;
} TTARice;

typedef struct TTAChannel {
    int32_t predictor;
    TTAFilter filter;
    TTARice rice;
} TTAChannel;

extern const uint32_t ff_tta_shift_1[];
extern const uint32_t * const ff_tta_shift_16;
extern const uint8_t ff_tta_filter_configs[];

void ff_tta_rice_init(TTARice *c, uint32_t k0, uint32_t k1);
void ff_tta_filter_init(TTAFilter *c, int32_t shift);
#endif /* AVCODEC_TTADATA_H */
