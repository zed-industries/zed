/*
 * Lossless video DSP utils
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


#ifndef AVCODEC_LOSSLESS_VIDEODSP_H
#define AVCODEC_LOSSLESS_VIDEODSP_H

#include <stdint.h>
#include <stddef.h>

typedef struct LLVidDSPContext {
    void (*add_bytes)(uint8_t *dst /* align 32 */, uint8_t *src /* align 32 */,
                      ptrdiff_t w);
    void (*add_median_pred)(uint8_t *dst, const uint8_t *top,
                            const uint8_t *diff, ptrdiff_t w,
                            int *left, int *left_top);
    int (*add_left_pred)(uint8_t *dst, const uint8_t *src,
                         ptrdiff_t w, int left);

    int  (*add_left_pred_int16)(uint16_t *dst, const uint16_t *src,
                                unsigned mask, ptrdiff_t w, unsigned left);
    void (*add_gradient_pred)(uint8_t *src /* align 32 */, const ptrdiff_t stride, const ptrdiff_t width);
} LLVidDSPContext;

void ff_llviddsp_init(LLVidDSPContext *llviddsp);
void ff_llviddsp_init_riscv(LLVidDSPContext *llviddsp);
void ff_llviddsp_init_x86(LLVidDSPContext *llviddsp);
void ff_llviddsp_init_ppc(LLVidDSPContext *llviddsp);

#endif //AVCODEC_LOSSLESS_VIDEODSP_H
