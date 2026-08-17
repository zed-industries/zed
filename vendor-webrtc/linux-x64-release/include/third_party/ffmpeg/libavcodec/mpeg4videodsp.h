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

#ifndef AVCODEC_MPEG4VIDEODSP_H
#define AVCODEC_MPEG4VIDEODSP_H

#include <stdint.h>

void ff_gmc_c(uint8_t *dst, const uint8_t *src, int stride, int h, int ox, int oy,
              int dxx, int dxy, int dyx, int dyy, int shift, int r,
              int width, int height);

typedef struct Mpeg4VideoDSPContext {
    /**
     * translational global motion compensation.
     */
    void (*gmc1)(uint8_t *dst /* align 8 */, const uint8_t *src /* align 1 */,
                 int srcStride, int h, int x16, int y16, int rounder);
    /**
     * global motion compensation.
     */
    void (*gmc)(uint8_t *dst /* align 8 */, const uint8_t *src /* align 1 */,
                int stride, int h, int ox, int oy,
                int dxx, int dxy, int dyx, int dyy,
                int shift, int r, int width, int height);
} Mpeg4VideoDSPContext;

void ff_mpeg4videodsp_init(Mpeg4VideoDSPContext *c);
void ff_mpeg4videodsp_init_ppc(Mpeg4VideoDSPContext *c);
void ff_mpeg4videodsp_init_x86(Mpeg4VideoDSPContext *c);

#endif /* AVCODEC_MPEG4VIDEODSP_H */
