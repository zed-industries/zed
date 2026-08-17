/*
 * codec2 utility functions
 * Copyright (c) 2017 Tomas Härdin
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

#ifndef AVCODEC_CODEC2UTILS_H
#define AVCODEC_CODEC2UTILS_H

#include <stdint.h>

//Highest mode we're willing to use.
//Don't want to let users accidentally produce files that can't be decoded in the future.
//CODEC2_MODE_WB (9) is experimental/unstable as of 2017-11-23.
#define CODEC2_MODE_MAX 8 //CODEC2_MODE_700C

//Used by both codec2raw demuxer and libcodec2 encoder.
//The integers match the values in codec2.h, so "3200" -> CODEC2_MODE_3000 = 0 and so on.
//It is possible that we're linked to a version of libcodec2 that lacks some of these modes.
//For example Debian stretch ships with libcodec2.so.0.4 which lacks CODEC2_MODE_700C.
#define CODEC2_AVOPTIONS(desc, classname, min_val, default_val, option_flags) \
    { "mode", desc, offsetof(classname, mode), AV_OPT_TYPE_INT, {.i64 = default_val}, min_val, CODEC2_MODE_MAX, .flags=option_flags, .unit="codec2_mode"},\
    { "3200", "3200", 0, AV_OPT_TYPE_CONST, {.i64 = 0}, .flags=option_flags, .unit="codec2_mode"},\
    { "2400", "2400", 0, AV_OPT_TYPE_CONST, {.i64 = 1}, .flags=option_flags, .unit="codec2_mode"},\
    { "1600", "1600", 0, AV_OPT_TYPE_CONST, {.i64 = 2}, .flags=option_flags, .unit="codec2_mode"},\
    { "1400", "1400", 0, AV_OPT_TYPE_CONST, {.i64 = 3}, .flags=option_flags, .unit="codec2_mode"},\
    { "1300", "1300", 0, AV_OPT_TYPE_CONST, {.i64 = 4}, .flags=option_flags, .unit="codec2_mode"},\
    { "1200", "1200", 0, AV_OPT_TYPE_CONST, {.i64 = 5}, .flags=option_flags, .unit="codec2_mode"},\
    { "700",  "700",  0, AV_OPT_TYPE_CONST, {.i64 = 6}, .flags=option_flags, .unit="codec2_mode"},\
    { "700B", "700B", 0, AV_OPT_TYPE_CONST, {.i64 = 7}, .flags=option_flags, .unit="codec2_mode"},\
    { "700C", "700C", 0, AV_OPT_TYPE_CONST, {.i64 = 8}, .flags=option_flags, .unit="codec2_mode"}

#define CODEC2_EXTRADATA_SIZE 4

//Used in codec2raw demuxer and libcodec2 encoder
static inline void codec2_make_extradata(uint8_t *ptr, int mode) {
    //version 0.8 as of 2017-12-23 (r3386)
    ptr[0] = 0;     //major
    ptr[1] = 8;     //minor
    ptr[2] = mode;  //mode
    ptr[3] = 0;     //flags
}

static inline uint8_t codec2_mode_from_extradata(uint8_t *ptr) {
    return ptr[2];
}

#endif /* AVCODEC_CODEC2UTILS_H */
