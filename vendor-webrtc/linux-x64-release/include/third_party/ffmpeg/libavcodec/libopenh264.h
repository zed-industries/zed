/*
 * OpenH264 shared utils
 * Copyright (C) 2014 Martin Storsjo
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

#ifndef AVCODEC_LIBOPENH264_H
#define AVCODEC_LIBOPENH264_H

#define OPENH264_VER_AT_LEAST(maj, min) \
    ((OPENH264_MAJOR  > (maj)) || \
     (OPENH264_MAJOR == (maj) && OPENH264_MINOR >= (min)))

// This function will be provided to the libopenh264 library.  The function will be called
// when libopenh264 wants to log a message (error, warning, info, etc.).  The signature for
// this function (defined in .../codec/api/svc/codec_api.h) is:
//
//        typedef void (*WelsTraceCallback) (void* ctx, int level, const char* string);

void ff_libopenh264_trace_callback(void *ctx, int level, const char *msg);

#endif /* AVCODEC_LIBOPENH264_H */
