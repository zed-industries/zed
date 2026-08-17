/*
 * Sun Rasterfile Image Format
 * Copyright (c) 2007, 2008 Ivo van Poorten
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

#ifndef AVCODEC_SUNRAST_H
#define AVCODEC_SUNRAST_H

#define RAS_MAGIC 0x59a66a95

#define RMT_NONE      0
#define RMT_EQUAL_RGB 1
#define RMT_RAW       2 ///< the data layout of this map type is unknown

/* The Old and Standard format types indicate that the image data is
 * uncompressed. There is no difference between the two formats. */
#define RT_OLD          0
#define RT_STANDARD     1

/* The Byte-Encoded format type indicates that the image data is compressed
 * using a run-length encoding scheme. */
#define RT_BYTE_ENCODED 2
#define RLE_TRIGGER 0x80

/* The RGB format type indicates that the image is uncompressed with reverse
 * component order from Old and Standard (RGB vs BGR). */
#define RT_FORMAT_RGB   3

/* The TIFF and IFF format types indicate that the raster file was originally
 * converted from either of these file formats. We do not have any samples or
 * documentation of the format details. */
#define RT_FORMAT_TIFF  4
#define RT_FORMAT_IFF   5

/* The Experimental format type is implementation-specific and is generally an
 * indication that the image file does not conform to the Sun Raster file
 * format specification. */
#define RT_EXPERIMENTAL 0xffff

#endif /* AVCODEC_SUNRAST_H */
