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

#ifndef AVCODEC_EXRDSP_H
#define AVCODEC_EXRDSP_H

#include <stddef.h>
#include <stdint.h>

typedef struct ExrDSPContext {
    void (*reorder_pixels)(uint8_t *dst, const uint8_t *src, ptrdiff_t size);
    void (*predictor)(uint8_t *src, ptrdiff_t size);
} ExrDSPContext;

void ff_exrdsp_init(ExrDSPContext *c);
void ff_exrdsp_init_riscv(ExrDSPContext *c);
void ff_exrdsp_init_x86(ExrDSPContext *c);

#endif /* AVCODEC_EXRDSP_H */
