/*
 * Copyright (c) 2001-2003 BERO <bero@geocities.co.jp>
 * Copyright (c) 2011 Oskar Arvidsson
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

#ifndef AVCODEC_RND_AVG_H
#define AVCODEC_RND_AVG_H

#include <stddef.h>
#include <stdint.h>

#define BYTE_VEC32(c) ((c) * 0x01010101UL)
#define BYTE_VEC64(c) ((c) * 0x0001000100010001UL)

static inline uint32_t rnd_avg32(uint32_t a, uint32_t b)
{
    return (a | b) - (((a ^ b) & ~BYTE_VEC32(0x01)) >> 1);
}

static inline uint32_t no_rnd_avg32(uint32_t a, uint32_t b)
{
    return (a & b) + (((a ^ b) & ~BYTE_VEC32(0x01)) >> 1);
}

static inline uint64_t rnd_avg64(uint64_t a, uint64_t b)
{
    return (a | b) - (((a ^ b) & ~BYTE_VEC64(0x01)) >> 1);
}

static inline uint64_t no_rnd_avg64(uint64_t a, uint64_t b)
{
    return (a & b) + (((a ^ b) & ~BYTE_VEC64(0x01)) >> 1);
}

#endif /* AVCODEC_RND_AVG_H */
