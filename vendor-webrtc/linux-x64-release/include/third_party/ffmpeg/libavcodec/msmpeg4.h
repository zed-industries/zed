/*
 * MSMPEG4 backend for encoder and decoder
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

#ifndef AVCODEC_MSMPEG4_H
#define AVCODEC_MSMPEG4_H

#include <stdint.h>

#include "mpegvideo.h"

#define II_BITRATE 128*1024
#define MBAC_BITRATE 50*1024

#define DC_MAX 119

void ff_msmpeg4_common_init(MpegEncContext *s);
int ff_msmpeg4_coded_block_pred(MpegEncContext * s, int n,
                                uint8_t **coded_block_ptr);

int ff_msmpeg4_pred_dc(MpegEncContext *s, int n,
                       int16_t **dc_val_ptr, int *dir_ptr);

#endif /* AVCODEC_MSMPEG4_H */
