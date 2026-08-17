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

#ifndef AVCODEC_X86_SIMPLE_IDCT_H
#define AVCODEC_X86_SIMPLE_IDCT_H

#include <stddef.h>
#include <stdint.h>

void ff_simple_idct_mmx(int16_t *block);
void ff_simple_idct_add_mmx(uint8_t *dest, ptrdiff_t line_size, int16_t *block);
void ff_simple_idct_put_mmx(uint8_t *dest, ptrdiff_t line_size, int16_t *block);

void ff_simple_idct_add_sse2(uint8_t *dest, ptrdiff_t line_size, int16_t *block);
void ff_simple_idct_put_sse2(uint8_t *dest, ptrdiff_t line_size, int16_t *block);

void ff_simple_idct8_sse2(int16_t *block);
void ff_simple_idct8_avx(int16_t *block);

void ff_simple_idct8_put_sse2(uint8_t *dest, ptrdiff_t line_size, int16_t *block);
void ff_simple_idct8_put_avx(uint8_t *dest, ptrdiff_t line_size, int16_t *block);

void ff_simple_idct8_add_sse2(uint8_t *dest, ptrdiff_t line_size, int16_t *block);
void ff_simple_idct8_add_avx(uint8_t *dest, ptrdiff_t line_size, int16_t *block);

void ff_simple_idct10_sse2(int16_t *block);
void ff_simple_idct10_avx(int16_t *block);

void ff_simple_idct10_put_sse2(uint8_t *dest, ptrdiff_t line_size, int16_t *block);
void ff_simple_idct10_put_avx(uint8_t *dest, ptrdiff_t line_size, int16_t *block);

void ff_simple_idct12_sse2(int16_t *block);
void ff_simple_idct12_avx(int16_t *block);

void ff_simple_idct12_put_sse2(uint8_t *dest, ptrdiff_t line_size, int16_t *block);
void ff_simple_idct12_put_avx(uint8_t *dest, ptrdiff_t line_size, int16_t *block);

#endif /* AVCODEC_X86_SIMPLE_IDCT_H */
