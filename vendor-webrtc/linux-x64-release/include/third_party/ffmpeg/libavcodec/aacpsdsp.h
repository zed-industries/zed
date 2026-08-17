/*
 * Copyright (c) 2012 Mans Rullgard
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

#ifndef AVCODEC_AACPSDSP_H
#define AVCODEC_AACPSDSP_H

#include <stddef.h>

#include "aac_defines.h"

#define PS_QMF_TIME_SLOTS 32
#define PS_AP_LINKS 3
#define PS_MAX_AP_DELAY 5

typedef struct PSDSPContext {
    void (*add_squares)(INTFLOAT *dst, const INTFLOAT (*src)[2], int n);
    void (*mul_pair_single)(INTFLOAT (*dst)[2], INTFLOAT (*src0)[2], INTFLOAT *src1,
                            int n);
    void (*hybrid_analysis)(INTFLOAT (*out)[2], INTFLOAT (*in)[2],
                            const INTFLOAT (*filter)[8][2],
                            ptrdiff_t stride, int n);
    void (*hybrid_analysis_ileave)(INTFLOAT (*out)[32][2], INTFLOAT L[2][38][64],
                                   int i, int len);
    void (*hybrid_synthesis_deint)(INTFLOAT out[2][38][64], INTFLOAT (*in)[32][2],
                                   int i, int len);
    void (*decorrelate)(INTFLOAT (*out)[2], INTFLOAT (*delay)[2],
                        INTFLOAT (*ap_delay)[PS_QMF_TIME_SLOTS+PS_MAX_AP_DELAY][2],
                        const INTFLOAT phi_fract[2], const INTFLOAT (*Q_fract)[2],
                        const INTFLOAT *transient_gain,
                        INTFLOAT g_decay_slope,
                        int len);
    void (*stereo_interpolate[2])(INTFLOAT (*l)[2], INTFLOAT (*r)[2],
                                  INTFLOAT h[2][4], INTFLOAT h_step[2][4],
                                  int len);
} PSDSPContext;

void AAC_RENAME(ff_psdsp_init)(PSDSPContext *s);
void ff_psdsp_init_arm(PSDSPContext *s);
void ff_psdsp_init_aarch64(PSDSPContext *s);
void ff_psdsp_init_riscv(PSDSPContext *s);
void ff_psdsp_init_x86(PSDSPContext *s);

#endif /* AVCODEC_AACPSDSP_H */
