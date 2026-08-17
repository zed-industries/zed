/*
 * Common stuff for some Microsoft Screen codecs
 * Copyright (C) 2012 Konstantin Shishkov
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

#ifndef AVCODEC_MSS34DSP_H
#define AVCODEC_MSS34DSP_H

#include <stddef.h>
#include <stdint.h>

/**
 * Generate quantisation matrix for given quality.
 *
 * @param qmat    destination matrix
 * @param quality quality setting (1-100)
 * @param luma    generate quantisation matrix for luma or chroma
 */
void ff_mss34_gen_quant_mat(uint16_t *qmat, int quality, int luma);

/**
 * Transform and output DCT block.
 *
 * @param dst     output plane
 * @param stride  output plane stride
 * @param block   block to transform and output
 */
void ff_mss34_dct_put(uint8_t *dst, ptrdiff_t stride, int *block);

#endif /* AVCODEC_MSS34DSP_H */
