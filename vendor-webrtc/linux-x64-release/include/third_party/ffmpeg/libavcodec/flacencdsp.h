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

#ifndef AVCODEC_FLACENCDSP_H
#define AVCODEC_FLACENCDSP_H

#include <stdint.h>

typedef struct FLACEncDSPContext {
    void (*lpc16_encode)(int32_t *res, const int32_t *smp, int len, int order,
                         const int32_t coefs[32], int shift);
    void (*lpc32_encode)(int32_t *res, const int32_t *smp, int len, int order,
                         const int32_t coefs[32], int shift);
} FLACEncDSPContext;

void ff_flacencdsp_init(FLACEncDSPContext *c);
void ff_flacencdsp_init_x86(FLACEncDSPContext *c);

#endif /* AVCODEC_FLACDSP_H */
