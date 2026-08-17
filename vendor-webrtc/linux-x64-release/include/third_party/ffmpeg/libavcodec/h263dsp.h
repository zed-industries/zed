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

#ifndef AVCODEC_H263DSP_H
#define AVCODEC_H263DSP_H

#include <stdint.h>

extern const uint8_t ff_h263_loop_filter_strength[32];

typedef struct H263DSPContext {
    void (*h263_h_loop_filter)(uint8_t *src, int stride, int qscale);
    void (*h263_v_loop_filter)(uint8_t *src, int stride, int qscale);
} H263DSPContext;

void ff_h263dsp_init(H263DSPContext *ctx);
void ff_h263dsp_init_riscv(H263DSPContext *ctx);
void ff_h263dsp_init_x86(H263DSPContext *ctx);
void ff_h263dsp_init_mips(H263DSPContext *ctx);

#endif /* AVCODEC_H263DSP_H */
