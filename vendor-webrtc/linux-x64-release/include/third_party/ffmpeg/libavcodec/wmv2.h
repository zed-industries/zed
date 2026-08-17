/*
 * Copyright (c) 2002 The FFmpeg Project
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

#ifndef AVCODEC_WMV2_H
#define AVCODEC_WMV2_H

#include "mpegvideo.h"
#include "wmv2dsp.h"

#define SKIP_TYPE_NONE 0
#define SKIP_TYPE_MPEG 1
#define SKIP_TYPE_ROW  2
#define SKIP_TYPE_COL  3


typedef struct WMV2Context {
    WMV2DSPContext wdsp;
    int hshift;
} WMV2Context;

void ff_wmv2_common_init(MpegEncContext *s);

void ff_mspel_motion(MpegEncContext *s,
                     uint8_t *dest_y, uint8_t *dest_cb, uint8_t *dest_cr,
                     uint8_t *const *ref_picture,
                     const op_pixels_func (*pix_op)[4],
                     int motion_x, int motion_y, int h);


static av_always_inline int wmv2_get_cbp_table_index(MpegEncContext *s, int cbp_index)
{
    static const uint8_t map[3][3] = {
        { 0, 2, 1 },
        { 1, 0, 2 },
        { 2, 1, 0 },
    };

    return map[(s->qscale > 10) + (s->qscale > 20)][cbp_index];
}

#endif /* AVCODEC_WMV2_H */
