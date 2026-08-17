/*
 * Copyright (c) 2024 Institue of Software Chinese Academy of Sciences (ISCAS).
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

#ifndef AVCODEC_RISCV_H26X_H2656DSP_H
#define AVCODEC_RISCV_H26X_H2656DSP_H

void ff_h2656_put_pixels_8_rvv_256(int16_t *dst, const uint8_t *src, ptrdiff_t srcstride, int height, intptr_t mx, intptr_t my, int width);
void ff_h2656_put_pixels_8_rvv_128(int16_t *dst, const uint8_t *src, ptrdiff_t srcstride, int height, intptr_t mx, intptr_t my, int width);

#endif
