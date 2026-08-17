/*
 * Converting FOURCCs to uppercase
 * Copyright (c) 2001 Fabrice Bellard
 * Copyright (c) 2002-2004 Michael Niedermayer <michaelni@gmx.at>
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

#ifndef AVCODEC_TO_UPPER4_H
#define AVCODEC_TO_UPPER4_H

#include "libavutil/avstring.h"
#include "internal.h"

unsigned int ff_toupper4(unsigned int x)
{
    return av_toupper(x & 0xFF) |
          (av_toupper((x >>  8) & 0xFF) << 8)  |
          (av_toupper((x >> 16) & 0xFF) << 16) |
((unsigned)av_toupper((x >> 24) & 0xFF) << 24);
}

#endif
