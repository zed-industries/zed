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

#ifndef AVCODEC_H264IDCT_H
#define AVCODEC_H264IDCT_H

#include <stdint.h>

#define H264_IDCT(depth) \
void ff_h264_idct8_add_ ## depth ## _c(uint8_t *dst, int16_t *block, int stride);\
void ff_h264_idct_add_ ## depth ## _c(uint8_t *dst, int16_t *block, int stride);\
void ff_h264_idct8_dc_add_ ## depth ## _c(uint8_t *dst, int16_t *block, int stride);\
void ff_h264_idct_dc_add_ ## depth ## _c(uint8_t *dst, int16_t *block, int stride);\
void ff_h264_idct_add16_ ## depth ## _c(uint8_t *dst, const int *blockoffset, int16_t *block, int stride, const uint8_t nnzc[5 * 8]);\
void ff_h264_idct_add16intra_ ## depth ## _c(uint8_t *dst, const int *blockoffset, int16_t *block, int stride, const uint8_t nnzc[5 * 8]);\
void ff_h264_idct8_add4_ ## depth ## _c(uint8_t *dst, const int *blockoffset, int16_t *block, int stride, const uint8_t nnzc[5 * 8]);\
void ff_h264_idct_add8_422_ ## depth ## _c(uint8_t **dest, const int *blockoffset, int16_t *block, int stride, const uint8_t nnzc[15 * 8]);\
void ff_h264_idct_add8_ ## depth ## _c(uint8_t **dest, const int *blockoffset, int16_t *block, int stride, const uint8_t nnzc[15 * 8]);\
void ff_h264_luma_dc_dequant_idct_ ## depth ## _c(int16_t *output, int16_t *input, int qmul);\
void ff_h264_chroma422_dc_dequant_idct_ ## depth ## _c(int16_t *block, int qmul);\
void ff_h264_chroma_dc_dequant_idct_ ## depth ## _c(int16_t *block, int qmul);

H264_IDCT( 8)
H264_IDCT( 9)
H264_IDCT(10)
H264_IDCT(12)
H264_IDCT(14)

#endif /* AVCODEC_H264IDCT_H */
