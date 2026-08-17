/*
 * XVID MPEG-4 VIDEO CODEC
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
 * header for Xvid IDCT functions
 */

#ifndef AVCODEC_X86_XVIDIDCT_H
#define AVCODEC_X86_XVIDIDCT_H

#include <stddef.h>
#include <stdint.h>

void ff_xvid_idct_sse2(short *block);
void ff_xvid_idct_put_sse2(uint8_t *dest, ptrdiff_t line_size, short *block);
void ff_xvid_idct_add_sse2(uint8_t *dest, ptrdiff_t line_size, short *block);

#endif /* AVCODEC_X86_XVIDIDCT_H */
