/*
 * Floating point AAN IDCT
 * Copyright (c) 2008 Michael Niedermayer <michaelni@gmx.at>
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

#ifndef AVCODEC_FAANIDCT_H
#define AVCODEC_FAANIDCT_H

#include <stddef.h>
#include <stdint.h>

void ff_faanidct(int16_t block[64]);
void ff_faanidct_add(uint8_t *dest, ptrdiff_t line_size, int16_t block[64]);
void ff_faanidct_put(uint8_t *dest, ptrdiff_t line_size, int16_t block[64]);

#endif /* AVCODEC_FAANIDCT_H */
