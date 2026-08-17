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

#ifndef AVCODEC_AUDIODSP_H
#define AVCODEC_AUDIODSP_H

#include <stdint.h>

typedef struct AudioDSPContext {
    /**
     * Calculate scalar product of two vectors.
     * @param len length of vectors, should be multiple of 16
     */
    int32_t (*scalarproduct_int16)(const int16_t *v1,
                                   const int16_t *v2 /* align 16 */, int len);

    /**
     * Clip each element in an array of int32_t to a given minimum and
     * maximum value.
     * @param dst  destination array
     *             constraints: 16-byte aligned
     * @param src  source array
     *             constraints: 16-byte aligned
     * @param min  minimum value
     *             constraints: must be in the range [-(1 << 24), 1 << 24]
     * @param max  maximum value
     *             constraints: must be in the range [-(1 << 24), 1 << 24]
     * @param len  number of elements in the array
     *             constraints: multiple of 32 greater than zero
     */
    void (*vector_clip_int32)(int32_t *dst, const int32_t *src, int32_t min,
                              int32_t max, unsigned int len);
    /* assume len is a multiple of 16, and arrays are 16-byte aligned */
    void (*vector_clipf)(float *dst /* align 16 */,
                         const float *src /* align 16 */,
                         int len /* align 16 */,
                         float min, float max);
} AudioDSPContext;

void ff_audiodsp_init(AudioDSPContext *c);
void ff_audiodsp_init_arm(AudioDSPContext *c);
void ff_audiodsp_init_ppc(AudioDSPContext *c);
void ff_audiodsp_init_riscv(AudioDSPContext *c);
void ff_audiodsp_init_x86(AudioDSPContext *c);

#endif /* AVCODEC_AUDIODSP_H */
