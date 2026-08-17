/*
 * TIFF Common Routines
 * Copyright (c) 2013 Thilo Borgmann <thilo.borgmann _at_ mail.de>
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
 * TIFF Common Routines
 * @author Thilo Borgmann <thilo.borgmann _at_ mail.de>
 */

#ifndef AVCODEC_TIFF_COMMON_H
#define AVCODEC_TIFF_COMMON_H

#include <stdint.h>
#include "libavutil/dict.h"
#include "bytestream.h"

/** data type identifiers for TIFF tags */
enum TiffTypes {
    TIFF_BYTE = 1,
    TIFF_STRING,
    TIFF_SHORT,
    TIFF_LONG,
    TIFF_RATIONAL,
    TIFF_SBYTE,
    TIFF_UNDEFINED,
    TIFF_SSHORT,
    TIFF_SLONG,
    TIFF_SRATIONAL,
    TIFF_FLOAT,
    TIFF_DOUBLE,
    TIFF_IFD
};

/** sizes of various TIFF field types (string size = 100)*/
static const uint8_t type_sizes[14] = {
    0, 1, 100, 2, 4, 8, 1, 1, 2, 4, 8, 4, 8, 4
};

static const uint16_t ifd_tags[] = {
    0x8769, // EXIF IFD
    0x8825, // GPS IFD
    0xA005  // Interoperability IFD
};


/** Returns a value > 0 if the tag is a known IFD-tag.
 *  The return value is the array index + 1 within ifd_tags[].
 */
int ff_tis_ifd(unsigned tag);

/** Reads a short from the bytestream using given endianness. */
unsigned ff_tget_short(GetByteContext *gb, int le);

/** Reads a long from the bytestream using given endianness. */
unsigned ff_tget_long(GetByteContext *gb, int le);

/** Reads a double from the bytestream using given endianness. */
double   ff_tget_double(GetByteContext *gb, int le);

/** Reads a byte from the bytestream using given endianness. */
unsigned ff_tget(GetByteContext *gb, int type, int le);

/** Adds count rationals converted to a string
 *  into the metadata dictionary.
 */
int ff_tadd_rational_metadata(int count, const char *name, const char *sep,
                              GetByteContext *gb, int le, AVDictionary **metadata);

/** Adds count longs converted to a string
 *  into the metadata dictionary.
 */
int ff_tadd_long_metadata(int count, const char *name, const char *sep,
                          GetByteContext *gb, int le, AVDictionary **metadata);

/** Adds count doubles converted to a string
 *  into the metadata dictionary.
 */
int ff_tadd_doubles_metadata(int count, const char *name, const char *sep,
                             GetByteContext *gb, int le, AVDictionary **metadata);

/** Adds count shorts converted to a string
 *  into the metadata dictionary.
 */
int ff_tadd_shorts_metadata(int count, const char *name, const char *sep,
                            GetByteContext *gb, int le, int is_signed, AVDictionary **metadata);

/** Adds count bytes converted to a string
 *  into the metadata dictionary.
 */
int ff_tadd_bytes_metadata(int count, const char *name, const char *sep,
                           GetByteContext *gb, int le, int is_signed, AVDictionary **metadata);

/** Adds a string of count characters
 *  into the metadata dictionary.
 */
int ff_tadd_string_metadata(int count, const char *name,
                            GetByteContext *gb, int le, AVDictionary **metadata);

/** Decodes a TIFF header from the input bytestream
 *  and sets the endianness in *le and the offset to
 *  the first IFD in *ifd_offset accordingly.
 */
int ff_tdecode_header(GetByteContext *gb, int *le, int *ifd_offset);

/** Reads the first 3 fields of a TIFF tag, which are
 *  the tag id, the tag type and the count of values for that tag.
 *  Afterwards the bytestream is located at the first value to read and
 *  *next holds the bytestream offset of the following tag.
 */
int ff_tread_tag(GetByteContext *gb, int le, unsigned *tag, unsigned *type,
                 unsigned *count, int *next);

#endif /* AVCODEC_TIFF_COMMON_H */
