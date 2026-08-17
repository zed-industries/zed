/*
 * Copyright (c) 2012 Clément Bœsch
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
 * JACOsub shared utils
 */

#ifndef AVCODEC_JACOSUB_H
#define AVCODEC_JACOSUB_H

#include "libavutil/common.h"

#define JSS_MAX_LINESIZE 512

static av_always_inline int jss_whitespace(char c)
{
    return c == ' ' || (c >= '\t' && c <= '\r');
}

static av_always_inline const char *jss_skip_whitespace(const char *p)
{
    while (jss_whitespace(*p))
        p++;
    return p;
}

#endif /* AVCODEC_JACOSUB_H */
