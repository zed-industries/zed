/*
 * Copyright (c) 2022 Caleb Etemesi <etemesicaleb@gmail.com>
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

#ifndef AVCODEC_JPEG2000HTDEC_H
#define AVCODEC_JPEG2000HTDEC_H

#include "jpeg2000dec.h"

/**
 * HT Block decoder as specified in Rec. ITU-T T.814 | ISO/IEC 15444-15
 */

int ff_jpeg2000_decode_htj2k(const Jpeg2000DecoderContext *s, Jpeg2000CodingStyle *codsty,
                            Jpeg2000T1Context *t1, Jpeg2000Cblk *cblk, int width,
                            int height, int M_b, uint8_t roi_shift);

#endif /* AVCODEC_JPEG2000HTDEC_H */
