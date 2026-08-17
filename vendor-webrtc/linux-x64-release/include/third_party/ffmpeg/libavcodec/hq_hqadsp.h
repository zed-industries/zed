/*
 * Canopus HQ/HQA decoder
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

/**
 * @file
 * HQ/HQA variant of AAN IDCT
 * It differs from the standard AAN IDCT in precision and in the second stage.
 */

#ifndef AVCODEC_HQ_HQADSP_H
#define AVCODEC_HQ_HQADSP_H

#include <stdint.h>

typedef struct HQDSPContext {
    void (*idct_put)(uint8_t *dst, int stride, int16_t *block);
} HQDSPContext;

void ff_hqdsp_init(HQDSPContext *c);

#endif /* AVCODEC_HQ_HQADSP_H */
