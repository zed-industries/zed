/*
 * Copyright (C) 2012 Ronald S. Bultje
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
 * Core video DSP helper functions
 */

#ifndef AVCODEC_VIDEODSP_H
#define AVCODEC_VIDEODSP_H

#include <stddef.h>
#include <stdint.h>

#define EMULATED_EDGE(depth) \
void ff_emulated_edge_mc_ ## depth(uint8_t *dst, const uint8_t *src, \
                                   ptrdiff_t dst_stride, ptrdiff_t src_stride, \
                                   int block_w, int block_h,\
                                   int src_x, int src_y, int w, int h);

EMULATED_EDGE(8)

typedef struct VideoDSPContext {
    /**
     * Copy a rectangular area of samples to a temporary buffer and replicate
     * the border samples.
     *
     * @param dst destination buffer
     * @param dst_stride number of bytes between 2 vertically adjacent samples
     *                   in destination buffer
     * @param src source buffer
     * @param dst_linesize number of bytes between 2 vertically adjacent
     *                     samples in the destination buffer
     * @param src_linesize number of bytes between 2 vertically adjacent
     *                     samples in both the source buffer
     * @param block_w width of block
     * @param block_h height of block
     * @param src_x x coordinate of the top left sample of the block in the
     *                source buffer
     * @param src_y y coordinate of the top left sample of the block in the
     *                source buffer
     * @param w width of the source buffer
     * @param h height of the source buffer
     */
    void (*emulated_edge_mc)(uint8_t *dst, const uint8_t *src,
                             ptrdiff_t dst_linesize,
                             ptrdiff_t src_linesize,
                             int block_w, int block_h,
                             int src_x, int src_y, int w, int h);

    /**
     * Prefetch memory into cache (if supported by hardware).
     *
     * @param buf    pointer to buffer to prefetch memory from
     * @param stride distance between two lines of buf (in bytes)
     * @param h      number of lines to prefetch
     */
    void (*prefetch)(const uint8_t *buf, ptrdiff_t stride, int h);
} VideoDSPContext;

void ff_videodsp_init(VideoDSPContext *ctx, int bpc);

/* for internal use only (i.e. called by ff_videodsp_init() */
void ff_videodsp_init_aarch64(VideoDSPContext *ctx, int bpc);
void ff_videodsp_init_arm(VideoDSPContext *ctx, int bpc);
void ff_videodsp_init_ppc(VideoDSPContext *ctx, int bpc);
void ff_videodsp_init_riscv(VideoDSPContext *ctx, int bpc);
void ff_videodsp_init_x86(VideoDSPContext *ctx, int bpc);
void ff_videodsp_init_mips(VideoDSPContext *ctx, int bpc);
void ff_videodsp_init_loongarch(VideoDSPContext *ctx, int bpc);

#endif /* AVCODEC_VIDEODSP_H */
