/*
 * (I)DCT Transforms
 * Copyright (c) 2009 Peter Ross <pross@xvid.org>
 * Copyright (c) 2010 Alex Converse <alex.converse@gmail.com>
 * Copyright (c) 2010 Vitor Sessak
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
 * Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
 */

#ifndef AVCODEC_DCT_H
#define AVCODEC_DCT_H

#include <stddef.h>
#include <stdint.h>

void ff_j_rev_dct(int16_t *data);
void ff_j_rev_dct4(int16_t *data);
void ff_j_rev_dct2(int16_t *data);
void ff_j_rev_dct1(int16_t *data);
void ff_jref_idct_put(uint8_t *dest, ptrdiff_t line_size, int16_t *block);
void ff_jref_idct_add(uint8_t *dest, ptrdiff_t line_size, int16_t *block);

#endif /* AVCODEC_DCT_H */
