/*
 * H.261 encoder
 * Copyright (c) 2002-2004 Michael Niedermayer <michaelni@gmx.at>
 * Copyright (c) 2004 Maarten Daniels
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
 * H.261 encoder header.
 */

#ifndef AVCODEC_H261ENC_H
#define AVCODEC_H261ENC_H

#include "mpegvideo.h"

void ff_h261_reorder_mb_index(MpegEncContext *s);
void ff_h261_encode_mb(MpegEncContext *s, int16_t block[6][64],
                       int motion_x, int motion_y);
void ff_h261_encode_picture_header(MpegEncContext *s);
int ff_h261_encode_init(MpegEncContext *s);

#endif
