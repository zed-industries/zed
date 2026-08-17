/*
 * MSMPEG4 encoder header
 * copyright (c) 2007 Aurelien Jacobs <aurel@gnuage.org>
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

#ifndef AVCODEC_MSMPEG4ENC_H
#define AVCODEC_MSMPEG4ENC_H

#include "mpegvideo.h"
#include "put_bits.h"
#include "rl.h"

typedef struct MSMPEG4EncContext {
    MpegEncContext s;

    /** [mb_intra][isChroma][level][run][last] */
    unsigned ac_stats[2][2][MAX_LEVEL + 1][MAX_RUN + 1][2];
} MSMPEG4EncContext;

void ff_msmpeg4_encode_init(MpegEncContext *s);
void ff_msmpeg4_encode_picture_header(MpegEncContext *s);
void ff_msmpeg4_encode_ext_header(MpegEncContext *s);
void ff_msmpeg4_encode_mb(MpegEncContext *s, int16_t block[6][64],
                          int motion_x, int motion_y);
void ff_msmpeg4_encode_block(MpegEncContext * s, int16_t * block, int n);
void ff_msmpeg4_handle_slices(MpegEncContext *s);
void ff_msmpeg4_encode_motion(MpegEncContext * s, int mx, int my);

void ff_msmpeg4_code012(PutBitContext *pb, int n);

#endif
