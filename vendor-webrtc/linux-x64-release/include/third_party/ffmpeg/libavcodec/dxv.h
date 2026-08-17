/*
 * Resolume DXV common
 * Copyright (C) 2024 Connor Worley <connorbworley@gmail.com>
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

#ifndef AVCODEC_DXV_H
#define AVCODEC_DXV_H

#include "libavutil/macros.h"

typedef enum DXVTextureFormat {
    DXV_FMT_DXT1 = MKBETAG('D', 'X', 'T', '1'),
    DXV_FMT_DXT5 = MKBETAG('D', 'X', 'T', '5'),
    DXV_FMT_YCG6 = MKBETAG('Y', 'C', 'G', '6'),
    DXV_FMT_YG10 = MKBETAG('Y', 'G', '1', '0'),
} DXVTextureFormat;

#endif /* AVCODEC_DXV_H */
