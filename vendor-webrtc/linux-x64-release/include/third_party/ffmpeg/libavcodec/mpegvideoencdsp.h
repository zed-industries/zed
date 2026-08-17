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

#ifndef AVCODEC_MPEGVIDEOENCDSP_H
#define AVCODEC_MPEGVIDEOENCDSP_H

#include <stdint.h>

#include "avcodec.h"

#define BASIS_SHIFT 16
#define RECON_SHIFT 6

#define EDGE_TOP    1
#define EDGE_BOTTOM 2

typedef struct MpegvideoEncDSPContext {
    int (*try_8x8basis)(const int16_t rem[64], const int16_t weight[64],
                        const int16_t basis[64], int scale);
    void (*add_8x8basis)(int16_t rem[64], const int16_t basis[64], int scale);

    int (*pix_sum)(const uint8_t *pix, ptrdiff_t line_size);
    int (*pix_norm1)(const uint8_t *pix, ptrdiff_t line_size);

    void (*shrink[4])(uint8_t *dst, ptrdiff_t dst_wrap, const uint8_t *src,
                      ptrdiff_t src_wrap, int width, int height);

    void (*draw_edges)(uint8_t *buf, ptrdiff_t wrap, int width, int height,
                       int w, int h, int sides);
} MpegvideoEncDSPContext;

void ff_mpegvideoencdsp_init(MpegvideoEncDSPContext *c,
                             AVCodecContext *avctx);
void ff_mpegvideoencdsp_init_aarch64(MpegvideoEncDSPContext *c,
                                     AVCodecContext *avctx);
void ff_mpegvideoencdsp_init_arm(MpegvideoEncDSPContext *c,
                                 AVCodecContext *avctx);
void ff_mpegvideoencdsp_init_ppc(MpegvideoEncDSPContext *c,
                                 AVCodecContext *avctx);
void ff_mpegvideoencdsp_init_riscv(MpegvideoEncDSPContext *c,
                                   AVCodecContext *avctx);
void ff_mpegvideoencdsp_init_x86(MpegvideoEncDSPContext *c,
                                 AVCodecContext *avctx);
void ff_mpegvideoencdsp_init_mips(MpegvideoEncDSPContext *c,
                                  AVCodecContext *avctx);

#endif /* AVCODEC_MPEGVIDEOENCDSP_H */
