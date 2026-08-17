/*
 * CCITT Fax Group 3 and 4 decompression
 * Copyright (c) 2008 Konstantin Shishkov
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
 * CCITT Fax Group 3 and 4 decompression
 * @author Konstantin Shishkov
 */
#ifndef AVCODEC_FAXCOMPR_H
#define AVCODEC_FAXCOMPR_H

#include "avcodec.h"
#include "tiff.h"

/**
 * initialize unpacker code
 */
void ff_ccitt_unpack_init(void);

/**
 * unpack data compressed with CCITT Group 3 1/2-D or Group 4 method
 */
int ff_ccitt_unpack(AVCodecContext *avctx,
                    const uint8_t *src, int srcsize,
                    uint8_t *dst, int height, int stride,
                    enum TiffCompr compr, int opts);

#endif /* AVCODEC_FAXCOMPR_H */
