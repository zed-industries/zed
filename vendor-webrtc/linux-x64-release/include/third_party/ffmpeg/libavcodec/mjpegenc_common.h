/*
 * lossless JPEG shared bits
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

#ifndef AVCODEC_MJPEGENC_COMMON_H
#define AVCODEC_MJPEGENC_COMMON_H

#include <stdint.h>

#include "avcodec.h"
#include "put_bits.h"

struct MJpegContext;

int ff_mjpeg_add_icc_profile_size(AVCodecContext *avctx, const AVFrame *frame,
                                  size_t *max_pkt_size);
void ff_mjpeg_encode_picture_header(AVCodecContext *avctx, PutBitContext *pb,
                                    const AVFrame *frame, const struct MJpegContext *m,
                                    const uint8_t intra_matrix_permutation[64],
                                    int pred,
                                    const uint16_t luma_intra_matrix[64],
                                    const uint16_t chroma_intra_matrix[64],
                                    int use_slices);
void ff_mjpeg_encode_picture_trailer(PutBitContext *pb, int header_bits);
void ff_mjpeg_escape_FF(PutBitContext *pb, int start);
void ff_mjpeg_build_huffman_codes(uint8_t *huff_size, uint16_t *huff_code,
                                  const uint8_t *bits_table,
                                  const uint8_t *val_table);
void ff_mjpeg_init_hvsample(const AVCodecContext *avctx, int hsample[4], int vsample[4]);

void ff_mjpeg_encode_dc(PutBitContext *pb, int val,
                        const uint8_t huff_size[], const uint16_t huff_code[]);

int ff_mjpeg_encode_check_pix_fmt(AVCodecContext *avctx);

#endif /* AVCODEC_MJPEGENC_COMMON_H */
