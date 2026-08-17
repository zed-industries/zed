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

#ifndef AVCODEC_UTVIDEODSP_H
#define AVCODEC_UTVIDEODSP_H

#include <stdint.h>
#include <stddef.h>
#include "libavutil/pixfmt.h"
#include "config.h"

typedef struct UTVideoDSPContext {
    void (*restore_rgb_planes)(uint8_t *src_r, uint8_t *src_g, uint8_t *src_b,
                               ptrdiff_t linesize_r, ptrdiff_t linesize_g,
                               ptrdiff_t linesize_b, int width, int height);
    void (*restore_rgb_planes10)(uint16_t *src_r, uint16_t *src_g, uint16_t *src_b,
                                 ptrdiff_t linesize_r, ptrdiff_t linesize_g,
                                 ptrdiff_t linesize_b, int width, int height);
} UTVideoDSPContext;

void ff_utvideodsp_init(UTVideoDSPContext *c);
void ff_utvideodsp_init_riscv(UTVideoDSPContext *c);
void ff_utvideodsp_init_x86(UTVideoDSPContext *c);

#endif /* AVCODEC_UTVIDEODSP_H */
