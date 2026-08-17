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

#ifndef AVCODEC_V210ENC_H
#define AVCODEC_V210ENC_H

#include "libavutil/log.h"
#include "libavutil/opt.h"
#include "libavutil/pixfmt.h"

typedef struct V210EncContext {
    void (*pack_line_8)(const uint8_t *y, const uint8_t *u,
                        const uint8_t *v, uint8_t *dst, ptrdiff_t width);
    void (*pack_line_10)(const uint16_t *y, const uint16_t *u,
                         const uint16_t *v, uint8_t *dst, ptrdiff_t width);
    int sample_factor_8;
    int sample_factor_10;
} V210EncContext;

void ff_v210enc_init_x86(V210EncContext *s);

#endif /* AVCODEC_V210ENC_H */
