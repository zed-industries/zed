/*
 * RLE encoder
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

#ifndef AVCODEC_RLE_H
#define AVCODEC_RLE_H

#include <stdint.h>

/**
 * Count up to 127 consecutive pixels which are either all the same or
 * all differ from the previous and next pixels.
 * @param start Pointer to the first pixel
 * @param len Maximum number of pixels
 * @param bpp Bytes per pixel
 * @param same 1 if searching for identical pixel values, 0 for differing
 * @return Number of matching consecutive pixels found
 */
int ff_rle_count_pixels(const uint8_t *start, int len, int bpp, int same);

/**
 * RLE compress the row, with maximum size of out_size.
 * Value before repeated bytes is (count ^ xor_rep) + add_rep.
 * Value before raw bytes is (count ^ xor_raw) + add_raw.
 * @param outbuf Output buffer
 * @param out_size Maximum output size
 * @param inbuf Input buffer
 * @param bpp Bytes per pixel
 * @param w Image width
 * @return Size of output in bytes, or -1 if larger than out_size
 */
int ff_rle_encode(uint8_t *outbuf, int out_size, const uint8_t *inbuf, int bpp,
                  int w, int add_rep, int xor_rep, int add_raw, int xor_raw);

#endif /* AVCODEC_RLE_H */
