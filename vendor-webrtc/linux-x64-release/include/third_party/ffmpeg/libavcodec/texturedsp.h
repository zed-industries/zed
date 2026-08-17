/*
 * Texture block module
 * Copyright (C) 2015 Vittorio Giovara <vittorio.giovara@gmail.com>
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
 * Texture block (4x4) module
 *
 * References:
 *   https://www.opengl.org/wiki/S3_Texture_Compression
 *   https://www.opengl.org/wiki/Red_Green_Texture_Compression
 *   https://msdn.microsoft.com/en-us/library/bb694531%28v=vs.85%29.aspx
 *
 * All functions return how much data has been written or read.
 *
 * Pixel input or output format is always AV_PIX_FMT_RGBA.
 */

#ifndef AVCODEC_TEXTUREDSP_H
#define AVCODEC_TEXTUREDSP_H

#include <stddef.h>
#include <stdint.h>

#define TEXTURE_BLOCK_W 4
#define TEXTURE_BLOCK_H 4

typedef struct TextureDSPContext {
    int (*dxt1_block)        (uint8_t *dst, ptrdiff_t stride, const uint8_t *block);
    int (*dxt1a_block)       (uint8_t *dst, ptrdiff_t stride, const uint8_t *block);
    int (*dxt2_block)        (uint8_t *dst, ptrdiff_t stride, const uint8_t *block);
    int (*dxt3_block)        (uint8_t *dst, ptrdiff_t stride, const uint8_t *block);
    int (*dxt4_block)        (uint8_t *dst, ptrdiff_t stride, const uint8_t *block);
    int (*dxt5_block)        (uint8_t *dst, ptrdiff_t stride, const uint8_t *block);
    int (*dxt5y_block)       (uint8_t *dst, ptrdiff_t stride, const uint8_t *block);
    int (*dxt5ys_block)      (uint8_t *dst, ptrdiff_t stride, const uint8_t *block);
    int (*rgtc1s_block)      (uint8_t *dst, ptrdiff_t stride, const uint8_t *block);
    int (*rgtc1u_block)      (uint8_t *dst, ptrdiff_t stride, const uint8_t *block);
    int (*rgtc1u_gray_block) (uint8_t *dst, ptrdiff_t stride, const uint8_t *block);
    int (*rgtc1u_alpha_block)(uint8_t *dst, ptrdiff_t stride, const uint8_t *block);
    int (*rgtc2s_block)      (uint8_t *dst, ptrdiff_t stride, const uint8_t *block);
    int (*rgtc2u_block)      (uint8_t *dst, ptrdiff_t stride, const uint8_t *block);
    int (*dxn3dc_block)      (uint8_t *dst, ptrdiff_t stride, const uint8_t *block);
} TextureDSPContext;

typedef struct TextureDSPEncContext {
    int (*dxt1_block)        (uint8_t *dst, ptrdiff_t stride, const uint8_t *block);
    int (*dxt5_block)        (uint8_t *dst, ptrdiff_t stride, const uint8_t *block);
    int (*dxt5ys_block)      (uint8_t *dst, ptrdiff_t stride, const uint8_t *block);
} TextureDSPEncContext;

typedef struct TextureDSPThreadContext {
    union {
        const uint8_t *in;       // Input frame data
        uint8_t *out;            // Output frame data
    } frame_data;
    ptrdiff_t stride;            // Frame linesize
    int width, height;           // Frame width / height
    union {
        const uint8_t *in;       // Compressed texture for decompression
        uint8_t *out;            // Compressed texture of compression
    } tex_data;
    int tex_ratio;               // Number of compressed bytes in a texture block
    int raw_ratio;               // Number bytes in a line of a raw block
    int slice_count;             // Number of slices for threaded operations

    /* Pointer to the selected compress or decompress function. */
    int (*tex_funct)(uint8_t *dst, ptrdiff_t stride, const uint8_t *block);
} TextureDSPThreadContext;

void ff_texturedsp_init(TextureDSPContext *c);
void ff_texturedspenc_init(TextureDSPEncContext *c);

struct AVCodecContext;
int ff_texturedsp_exec_decompress_threads(struct AVCodecContext *avctx,
                                          TextureDSPThreadContext *ctx);
int ff_texturedsp_exec_compress_threads(struct AVCodecContext *avctx,
                                        TextureDSPThreadContext *ctx);

#endif /* AVCODEC_TEXTUREDSP_H */
