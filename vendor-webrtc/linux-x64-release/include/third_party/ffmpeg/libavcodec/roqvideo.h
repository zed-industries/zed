/*
 * Copyright (C) 2003 Mike Melanson
 * Copyright (C) 2003 Dr. Tim Ferguson
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

#ifndef AVCODEC_ROQVIDEO_H
#define AVCODEC_ROQVIDEO_H

#include "libavutil/frame.h"

typedef struct roq_cell {
    unsigned char y[4];
    unsigned char u, v;
} roq_cell;

typedef struct roq_qcell {
    int idx[4];
} roq_qcell;

typedef struct motion_vect {
    int d[2];
} motion_vect;

typedef struct RoqContext {
    const AVClass *class;
    void *logctx;
    AVFrame *last_frame;
    AVFrame *current_frame;
    int width, height;

    roq_cell cb2x2[256];
    roq_qcell cb4x4[256];
} RoqContext;

#define RoQ_INFO              0x1001
#define RoQ_QUAD_CODEBOOK     0x1002
#define RoQ_QUAD_VQ           0x1011
#define RoQ_SOUND_MONO        0x1020
#define RoQ_SOUND_STEREO      0x1021

#define RoQ_ID_MOT              0x00
#define RoQ_ID_FCC              0x01
#define RoQ_ID_SLD              0x02
#define RoQ_ID_CCC              0x03

void ff_apply_vector_2x2(RoqContext *ri, int x, int y, roq_cell *cell);
void ff_apply_vector_4x4(RoqContext *ri, int x, int y, roq_cell *cell);

void ff_apply_motion_4x4(RoqContext *ri, int x, int y, int deltax, int deltay);

void ff_apply_motion_8x8(RoqContext *ri, int x, int y, int deltax, int deltay);

#endif /* AVCODEC_ROQVIDEO_H */
