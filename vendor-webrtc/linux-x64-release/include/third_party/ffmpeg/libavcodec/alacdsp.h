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

#ifndef AVCODEC_ALACDSP_H
#define AVCODEC_ALACDSP_H

#include <stdint.h>

typedef struct ALACDSPContext {
    void (*decorrelate_stereo)(int32_t *buffer[2], int nb_samples,
                               int decorr_shift, int decorr_left_weight);
    void (*append_extra_bits[2])(int32_t *buffer[2], int32_t *extra_bits_buffer[2],
                                 int extra_bits, int channels, int nb_samples);
} ALACDSPContext;

void ff_alacdsp_init(ALACDSPContext *c);
void ff_alacdsp_init_riscv(ALACDSPContext *c);
void ff_alacdsp_init_x86(ALACDSPContext *c);

#endif /* AVCODEC_ALACDSP_H */
