/*
 * JPEG-LS decoder
 * Copyright (c) 2003 Michael Niedermayer
 * Copyright (c) 2006 Konstantin Shishkov
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
 * JPEG-LS decoder.
 */

#ifndef AVCODEC_JPEGLSDEC_H
#define AVCODEC_JPEGLSDEC_H

#include "mjpeg.h"
#include "mjpegdec.h"

/**
 * Decode LSE block with initialization parameters
 */
int ff_jpegls_decode_lse(MJpegDecodeContext *s);

int ff_jpegls_decode_picture(MJpegDecodeContext *s, int near,
                             int point_transform, int ilv);

#endif /* AVCODEC_JPEGLSDEC_H */
