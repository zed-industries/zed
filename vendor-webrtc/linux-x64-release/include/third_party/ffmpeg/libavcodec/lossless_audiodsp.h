/*
 * Monkey's Audio lossless audio decoder
 * Copyright (c) 2007 Benjamin Zores <ben@geexbox.org>
 *  based upon libdemac from Dave Chapman.
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

#ifndef AVCODEC_LOSSLESS_AUDIODSP_H
#define AVCODEC_LOSSLESS_AUDIODSP_H

#include <stdint.h>

typedef struct LLAudDSPContext {
    /**
     * Calculate scalar product of v1 and v2,
     * and v1[i] += v3[i] * mul
     * @param len length of vectors, should be multiple of 16,
     *            or padd v3 and v1 or v2 with zeros.
     */
    int32_t (*scalarproduct_and_madd_int16)(int16_t *v1 /* align 16 */,
                                            const int16_t *v2,
                                            const int16_t *v3,
                                            int len, int mul);

    int32_t (*scalarproduct_and_madd_int32)(int16_t *v1 /* align 16 */,
                                            const int32_t *v2,
                                            const int16_t *v3,
                                            int len, int mul);
} LLAudDSPContext;

void ff_llauddsp_init(LLAudDSPContext *c);
void ff_llauddsp_init_arm(LLAudDSPContext *c);
void ff_llauddsp_init_ppc(LLAudDSPContext *c);
void ff_llauddsp_init_riscv(LLAudDSPContext *c);
void ff_llauddsp_init_x86(LLAudDSPContext *c);

#endif /* AVCODEC_LOSSLESS_AUDIODSP_H */
