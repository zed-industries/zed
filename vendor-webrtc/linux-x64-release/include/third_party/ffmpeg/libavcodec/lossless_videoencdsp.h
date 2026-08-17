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

#ifndef AVCODEC_LOSSLESS_VIDEOENCDSP_H
#define AVCODEC_LOSSLESS_VIDEOENCDSP_H

#include <stddef.h>
#include <stdint.h>

typedef struct LLVidEncDSPContext {
    void (*diff_bytes)(uint8_t *dst /* align 1 */,
                       const uint8_t *src1 /* align 1 */,
                       const uint8_t *src2 /* align 1 */,
                       intptr_t w);
    /**
     * Subtract HuffYUV's variant of median prediction.
     * Note, this might read from src1[-1], src2[-1].
     */
    void (*sub_median_pred)(uint8_t *dst, const uint8_t *src1,
                            const uint8_t *src2, intptr_t w,
                            int *left, int *left_top);

    void (*sub_left_predict)(uint8_t *dst, const uint8_t *src,
                          ptrdiff_t stride, ptrdiff_t width, int height);
} LLVidEncDSPContext;

void ff_llvidencdsp_init(LLVidEncDSPContext *c);
void ff_llvidencdsp_init_riscv(LLVidEncDSPContext *c);
void ff_llvidencdsp_init_x86(LLVidEncDSPContext *c);

#endif /* AVCODEC_LOSSLESS_VIDEOENCDSP_H */
