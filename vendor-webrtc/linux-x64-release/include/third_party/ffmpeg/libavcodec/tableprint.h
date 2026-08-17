/*
 * Generate a file for hardcoded tables
 *
 * Copyright (c) 2009 Reimar Döffinger <Reimar.Doeffinger@gmx.de>
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

#ifndef AVCODEC_TABLEPRINT_H
#define AVCODEC_TABLEPRINT_H

#include <inttypes.h>
#include <stdio.h>

#include "libavutil/common.h"

#define WRITE_1D_FUNC_ARGV(type, linebrk, fmtstr, ...)\
void write_##type##_array(const type *data, int len)\
{\
    int i;\
    printf("   ");\
    for (i = 0; i < len - 1; i++) {\
       printf(" "fmtstr",", __VA_ARGS__);\
       if ((i & linebrk) == linebrk) printf("\n   ");\
    }\
    printf(" "fmtstr"\n", __VA_ARGS__);\
}

#define WRITE_1D_FUNC(type, fmtstr, linebrk)\
    WRITE_1D_FUNC_ARGV(type, linebrk, fmtstr, data[i])

#define WRITE_2D_FUNC(type)\
void write_##type##_2d_array(const void *arg, int len, int len2)\
{\
    const type *data = arg;\
    int i;\
    printf("    {\n");\
    for (i = 0; i < len; i++) {\
        write_##type##_array(data + i * len2, len2);\
        printf(i == len - 1 ? "    }\n" : "    }, {\n");\
    }\
}

/**
 * @name Predefined functions for printing tables
 *
 * @{
 */
void write_int8_t_array     (const int8_t   *, int);
void write_uint8_t_array    (const uint8_t  *, int);
void write_uint16_t_array   (const uint16_t *, int);
void write_uint32_t_array   (const uint32_t *, int);
void write_int32_t_array    (const int32_t  *, int);
void write_float_array      (const float    *, int);
void write_int8_t_2d_array  (const void *, int, int);
void write_uint8_t_2d_array (const void *, int, int);
void write_uint32_t_2d_array(const void *, int, int);
void write_float_2d_array   (const void *, int, int);
/** @} */ // end of printfuncs group

/*
 * MSVC doesn't have %zu, since it was introduced in C99,
 * but has its own %Iu for printing size_t values.
 */
#if defined(_MSC_VER)
#define FMT "Iu"
#else
#define FMT "zu"
#endif

#define WRITE_ARRAY_ALIGNED(prefix, align, type, name)  \
    do {                                                \
        const size_t array_size = FF_ARRAY_ELEMS(name); \
        printf(prefix" DECLARE_ALIGNED("#align", "      \
               #type", "#name")[%"FMT"] = {\n",         \
               array_size);                             \
        write_##type##_array(name, array_size);         \
        printf("};\n");                                 \
    } while(0)

#define WRITE_ARRAY(prefix, type, name)                 \
    do {                                                \
        const size_t array_size = FF_ARRAY_ELEMS(name); \
        printf(prefix" "#type" "#name"[%"FMT"] = {\n",  \
               array_size);                             \
        write_##type##_array(name, array_size);         \
        printf("};\n");                                 \
    } while(0)

#define WRITE_2D_ARRAY(prefix, type, name)                              \
    do {                                                                \
        const size_t array_size1 = FF_ARRAY_ELEMS(name);                \
        const size_t array_size2 = FF_ARRAY_ELEMS(name[0]);             \
        printf(prefix" "#type" "#name"[%"FMT"][%"FMT"] = {\n",          \
               array_size1, array_size2 );                              \
        write_##type##_2d_array(name, array_size1, array_size2);        \
        printf("};\n");                                                 \
    } while(0)


WRITE_1D_FUNC(int8_t,   "%3"PRIi8, 15)
WRITE_1D_FUNC(uint8_t,  "0x%02"PRIx8, 15)
WRITE_1D_FUNC(uint16_t, "0x%08"PRIx16, 7)
WRITE_1D_FUNC(int16_t,  "%5"PRIi16, 7)
WRITE_1D_FUNC(uint32_t, "0x%08"PRIx32, 7)
WRITE_1D_FUNC(int32_t,  "0x%08"PRIx32, 7)
WRITE_1D_FUNC(float,    "%.18e", 3)

WRITE_2D_FUNC(int8_t)
WRITE_2D_FUNC(uint8_t)
WRITE_2D_FUNC(uint32_t)
WRITE_2D_FUNC(float)

static inline void write_fileheader(void)
{
    printf("/* This file was automatically generated. */\n");
    printf("#include <stdint.h>\n");
}

#endif /* AVCODEC_TABLEPRINT_H */
