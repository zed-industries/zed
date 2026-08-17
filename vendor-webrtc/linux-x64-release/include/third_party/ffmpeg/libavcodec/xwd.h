/*
 * XWD image format
 *
 * Copyright (c) 2012 Paul B Mahol
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

#ifndef AVCODEC_XWD_H
#define AVCODEC_XWD_H

#define XWD_VERSION         7
#define XWD_HEADER_SIZE     100
#define XWD_CMAP_SIZE       12

#define XWD_XY_BITMAP       0
#define XWD_XY_PIXMAP       1
#define XWD_Z_PIXMAP        2

#define XWD_STATIC_GRAY     0
#define XWD_GRAY_SCALE      1
#define XWD_STATIC_COLOR    2
#define XWD_PSEUDO_COLOR    3
#define XWD_TRUE_COLOR      4
#define XWD_DIRECT_COLOR    5

#endif /* AVCODEC_XWD_H */
