/*
 * Snappy module
 * Copyright (c) Luca Barbato
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
 * Snappy decompression
 *
 * Snappy is a compression/decompression algorithm that does not aim for
 * maximum compression, but rather for very high speeds and reasonable
 * compression.
 *
 * http://en.wikipedia.org/wiki/Snappy_%28software%29
 */

#ifndef AVCODEC_SNAPPY_H
#define AVCODEC_SNAPPY_H

#include <stdint.h>

#include "bytestream.h"

/**
 * Get the uncompressed length of an input buffer compressed using the Snappy
 * algorithm. The GetByteContext is not advanced.
 *
 * @param gb    input GetByteContext.
 * @return      A positive length on success, AVERROR otherwise.
 */
 int64_t ff_snappy_peek_uncompressed_length(GetByteContext *gb);

/**
 * Decompress an input buffer using Snappy algorithm.
 *
 * @param gb    input GetByteContext.
 * @param buf   input buffer pointer.
 * @param size  input/output on input, the size of buffer, on output, the size
 *              of the uncompressed data.
 * @return      0 if success, AVERROR otherwise.
 */
int ff_snappy_uncompress(GetByteContext *gb, uint8_t *buf, int64_t *size);

#endif /* AVCODEC_SNAPPY_H */
