/*
 * Copyright (C) 2016 foo86
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

#ifndef AVCODEC_DCAMATH_H
#define AVCODEC_DCAMATH_H

#include "libavutil/common.h"
#include "libavutil/intmath.h"

static inline int32_t norm__(int64_t a, int bits)
{
    if (bits > 0)
        return (int32_t)((a + (INT64_C(1) << (bits - 1))) >> bits);
    else
        return (int32_t)a;
}

static inline int32_t mul__(int32_t a, int32_t b, int bits)
{
    return norm__((int64_t)a * b, bits);
}

static inline int32_t norm13(int64_t a) { return norm__(a, 13); }
static inline int32_t norm16(int64_t a) { return norm__(a, 16); }
static inline int32_t norm20(int64_t a) { return norm__(a, 20); }
static inline int32_t norm21(int64_t a) { return norm__(a, 21); }
static inline int32_t norm23(int64_t a) { return norm__(a, 23); }

static inline int32_t mul15(int32_t a, int32_t b) { return mul__(a, b, 15); }
static inline int32_t mul16(int32_t a, int32_t b) { return mul__(a, b, 16); }
static inline int32_t mul17(int32_t a, int32_t b) { return mul__(a, b, 17); }
static inline int32_t mul22(int32_t a, int32_t b) { return mul__(a, b, 22); }
static inline int32_t mul23(int32_t a, int32_t b) { return mul__(a, b, 23); }
static inline int32_t mul31(int32_t a, int32_t b) { return mul__(a, b, 31); }
static inline int32_t mul32(int32_t a, int32_t b) { return mul__(a, b, 32); }

static inline int32_t clip23(int32_t a) { return av_clip_intp2(a, 23); }

#endif
