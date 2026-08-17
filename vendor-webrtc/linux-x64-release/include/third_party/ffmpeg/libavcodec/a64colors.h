/*
 * a64 video encoder - c64 colors in rgb (Pepto)
 * Copyright (c) 2009 Tobias Bindhammer
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
 * a64 video encoder - c64 colors in rgb
 */

#ifndef AVCODEC_A64COLORS_H
#define AVCODEC_A64COLORS_H

#include <stdint.h>

/* c64 palette in RGB */
static const uint8_t a64_palette[16][3] = {
    {0x00, 0x00, 0x00},
    {0xff, 0xff, 0xff},
    {0x68, 0x37, 0x2b},
    {0x70, 0xa4, 0xb2},
    {0x6f, 0x3d, 0x86},
    {0x58, 0x8d, 0x43},
    {0x35, 0x28, 0x79},
    {0xb8, 0xc7, 0x6f},
    {0x6f, 0x4f, 0x25},
    {0x43, 0x39, 0x00},
    {0x9a, 0x67, 0x59},
    {0x44, 0x44, 0x44},
    {0x6c, 0x6c, 0x6c},
    {0x9a, 0xd2, 0x84},
    {0x6c, 0x5e, 0xb5},
    {0x95, 0x95, 0x95},
};

#endif /* AVCODEC_A64COLORS_H */
