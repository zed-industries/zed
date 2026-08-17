/*
 * SVQ1 encoder DSP
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

#ifndef AVCODEC_SVQ1ENCDSP_H
#define AVCODEC_SVQ1ENCDSP_H

#include <stdint.h>

#include "config.h"

typedef struct SVQ1EncDSPContext {
    int (*ssd_int8_vs_int16)(const int8_t *pix1, const int16_t *pix2,
                             intptr_t size);
} SVQ1EncDSPContext;

void ff_svq1enc_init_ppc(SVQ1EncDSPContext *c);
void ff_svq1enc_init_riscv(SVQ1EncDSPContext *c);
void ff_svq1enc_init_x86(SVQ1EncDSPContext *c);

static int ssd_int8_vs_int16_c(const int8_t *pix1, const int16_t *pix2,
                               intptr_t size)
{
    int score = 0;

    for (intptr_t i = 0; i < size; i++)
        score += (pix1[i] - pix2[i]) * (pix1[i] - pix2[i]);
    return score;
}

static inline void ff_svq1enc_init(SVQ1EncDSPContext *c)
{
    c->ssd_int8_vs_int16 = ssd_int8_vs_int16_c;

#if ARCH_PPC
    ff_svq1enc_init_ppc(c);
#elif ARCH_RISCV
    ff_svq1enc_init_riscv(c);
#elif ARCH_X86
    ff_svq1enc_init_x86(c);
#endif
}

#endif /* AVCODEC_SVQ1ENCDSP_H */
