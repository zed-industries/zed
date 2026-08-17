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

#ifndef AVCODEC_FDCTDSP_H
#define AVCODEC_FDCTDSP_H

#include <stdint.h>

#include "libavutil/attributes_internal.h"

struct AVCodecContext;

typedef struct FDCTDSPContext {
    void (*fdct)(int16_t *block /* align 16 */);
    void (*fdct248)(int16_t *block /* align 16 */);
} FDCTDSPContext;

FF_VISIBILITY_PUSH_HIDDEN
void ff_fdctdsp_init(FDCTDSPContext *c, struct AVCodecContext *avctx);
void ff_fdctdsp_init_aarch64(FDCTDSPContext *c, struct AVCodecContext *avctx,
                             unsigned high_bit_depth);
void ff_fdctdsp_init_ppc(FDCTDSPContext *c, struct AVCodecContext *avctx,
                         unsigned high_bit_depth);
void ff_fdctdsp_init_x86(FDCTDSPContext *c, struct AVCodecContext *avctx,
                         unsigned high_bit_depth);

void ff_fdct_ifast(int16_t *data);
void ff_fdct_ifast248(int16_t *data);
void ff_jpeg_fdct_islow_8(int16_t *data);
void ff_jpeg_fdct_islow_10(int16_t *data);
void ff_fdct248_islow_8(int16_t *data);
void ff_fdct248_islow_10(int16_t *data);
FF_VISIBILITY_POP_HIDDEN

#endif /* AVCODEC_FDCTDSP_H */
