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

#ifndef AVCODEC_VP3DSP_H
#define AVCODEC_VP3DSP_H

#include <stddef.h>
#include <stdint.h>

typedef struct VP3DSPContext {
    /**
     * Copy 8xH pixels from source to destination buffer using a bilinear
     * filter with no rounding (i.e. *dst = (*a + *b) >> 1).
     *
     * @param dst destination buffer, aligned by 8
     * @param a first source buffer, no alignment
     * @param b second source buffer, no alignment
     * @param stride distance between two lines in source/dest buffers
     * @param h height
     */
    void (*put_no_rnd_pixels_l2)(uint8_t *dst,
                                 const uint8_t *a,
                                 const uint8_t *b,
                                 ptrdiff_t stride, int h);

    void (*idct_put)(uint8_t *dest, ptrdiff_t stride, int16_t *block);
    void (*idct_add)(uint8_t *dest, ptrdiff_t stride, int16_t *block);
    void (*idct_dc_add)(uint8_t *dest, ptrdiff_t stride, int16_t *block);
    void (*v_loop_filter)(uint8_t *src, ptrdiff_t stride, int *bounding_values);
    void (*h_loop_filter)(uint8_t *src, ptrdiff_t stride, int *bounding_values);
    void (*v_loop_filter_unaligned)(uint8_t *src, ptrdiff_t stride, int *bounding_values);
    void (*h_loop_filter_unaligned)(uint8_t *src, ptrdiff_t stride, int *bounding_values);
} VP3DSPContext;

void ff_vp3dsp_v_loop_filter_12(uint8_t *first_pixel, ptrdiff_t stride, int *bounding_values);
void ff_vp3dsp_h_loop_filter_12(uint8_t *first_pixel, ptrdiff_t stride, int *bounding_values);

void ff_vp3dsp_idct10_put(uint8_t *dest, ptrdiff_t stride, int16_t *block);
void ff_vp3dsp_idct10_add(uint8_t *dest, ptrdiff_t stride, int16_t *block);

void ff_vp3dsp_init(VP3DSPContext *c, int flags);
void ff_vp3dsp_init_arm(VP3DSPContext *c, int flags);
void ff_vp3dsp_init_ppc(VP3DSPContext *c, int flags);
void ff_vp3dsp_init_x86(VP3DSPContext *c, int flags);
void ff_vp3dsp_init_mips(VP3DSPContext *c, int flags);

void ff_vp3dsp_set_bounding_values(int * bound_values_array, int filter_limit);

#endif /* AVCODEC_VP3DSP_H */
