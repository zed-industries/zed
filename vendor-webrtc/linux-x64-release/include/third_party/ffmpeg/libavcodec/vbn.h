/*
 * VBN format definitions
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
 * VBN format definitions.
 */

#ifndef AVCODEC_VBN_H
#define AVCODEC_VBN_H

#define VBN_MAGIC          0x900df11e
#define VBN_MAJOR                   3
#define VBN_MINOR                   4

#define VBN_HEADER_SIZE           192

#define VBN_FORMAT_RAW              0
#define VBN_FORMAT_LZ               1
#define VBN_FORMAT_DXT1             2
#define VBN_FORMAT_DXT5             3

#define VBN_COMPRESSION_NONE        0
#define VBN_COMPRESSION_LZW     0x100

#define VBN_PIX_ALPHA               0
#define VBN_PIX_LUMINANCE           1
#define VBN_PIX_LUMINANCE_ALPHA     2
#define VBN_PIX_RGB                 3
#define VBN_PIX_RGBA                5
#define VBN_PIX_INDEX               6

#endif /* AVCODEC_VBN_H */
