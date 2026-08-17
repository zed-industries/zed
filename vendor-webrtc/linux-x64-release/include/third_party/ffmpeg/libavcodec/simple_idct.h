/*
 * Simple IDCT
 *
 * Copyright (c) 2001 Michael Niedermayer <michaelni@gmx.at>
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

/**
 * @file
 * simple idct header.
 */

#ifndef AVCODEC_SIMPLE_IDCT_H
#define AVCODEC_SIMPLE_IDCT_H

#include <stddef.h>
#include <stdint.h>

void ff_simple_idct_put_int16_8bit(uint8_t *dest, ptrdiff_t line_size, int16_t *block);
void ff_simple_idct_add_int16_8bit(uint8_t *dest, ptrdiff_t line_size, int16_t *block);
void ff_simple_idct_int16_8bit(int16_t *block);

void ff_simple_idct_put_int16_10bit(uint8_t *dest, ptrdiff_t line_size, int16_t *block);
void ff_simple_idct_add_int16_10bit(uint8_t *dest, ptrdiff_t line_size, int16_t *block);
void ff_simple_idct_int16_10bit(int16_t *block);

void ff_simple_idct_put_int32_10bit(uint8_t *dest, ptrdiff_t line_size, int16_t *block);
void ff_simple_idct_add_int32_10bit(uint8_t *dest, ptrdiff_t line_size, int16_t *block);
void ff_simple_idct_int32_10bit(int16_t *block);

void ff_simple_idct_put_int16_12bit(uint8_t *dest, ptrdiff_t line_size, int16_t *block);
void ff_simple_idct_add_int16_12bit(uint8_t *dest, ptrdiff_t line_size, int16_t *block);
void ff_simple_idct_int16_12bit(int16_t *block);

void ff_simple_idct248_put(uint8_t *dest, ptrdiff_t line_size, int16_t *block);

void ff_simple_idct84_add(uint8_t *dest, ptrdiff_t line_size, int16_t *block);
void ff_simple_idct48_add(uint8_t *dest, ptrdiff_t line_size, int16_t *block);
void ff_simple_idct44_add(uint8_t *dest, ptrdiff_t line_size, int16_t *block);

#endif /* AVCODEC_SIMPLE_IDCT_H */
