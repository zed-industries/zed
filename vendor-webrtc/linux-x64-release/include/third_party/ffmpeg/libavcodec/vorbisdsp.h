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

#ifndef AVCODEC_VORBISDSP_H
#define AVCODEC_VORBISDSP_H

#include <stddef.h>

typedef struct VorbisDSPContext {
    /* assume len is a multiple of 4, and arrays are 16-byte aligned */
    void (*vorbis_inverse_coupling)(float *mag, float *ang,
                                    ptrdiff_t blocksize);
} VorbisDSPContext;

void ff_vorbisdsp_init(VorbisDSPContext *dsp);

/* for internal use only */
void ff_vorbisdsp_init_aarch64(VorbisDSPContext *dsp);
void ff_vorbisdsp_init_x86(VorbisDSPContext *dsp);
void ff_vorbisdsp_init_arm(VorbisDSPContext *dsp);
void ff_vorbisdsp_init_ppc(VorbisDSPContext *dsp);
void ff_vorbisdsp_init_riscv(VorbisDSPContext *dsp);

#endif /* AVCODEC_VORBISDSP_H */
