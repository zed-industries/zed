/*
 * FITS image format common prototypes and structures
 * Copyright (c) 2017 Paras Chadha
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

#ifndef AVCODEC_FITS_H
#define AVCODEC_FITS_H

#include <inttypes.h>

#include "libavutil/dict.h"

typedef enum FITSHeaderState {
    STATE_SIMPLE,
    STATE_XTENSION,
    STATE_BITPIX,
    STATE_NAXIS,
    STATE_NAXIS_N,
    STATE_PCOUNT,
    STATE_GCOUNT,
    STATE_REST,
} FITSHeaderState;

/**
 * Structure to store the header keywords in FITS file
 */
typedef struct FITSHeader {
    FITSHeaderState state;
    unsigned naxis_index;
    int bitpix;
    int64_t blank;
    int blank_found;
    int naxis;
    int naxisn[999];
    int pcount;
    int gcount;
    int groups;
    int rgb; /**< 1 if file contains RGB image, 0 otherwise */
    int image_extension;
    double bscale;
    double bzero;
    int data_min_found;
    double data_min;
    int data_max_found;
    double data_max;
} FITSHeader;


/**
 * Initialize a single header line
 * @param header pointer to the header
 * @param state current state of parsing the header
 * @return 0 if successful otherwise AVERROR_INVALIDDATA
 */
int avpriv_fits_header_init(FITSHeader *header, FITSHeaderState state);

/**
 * Parse a single header line
 * @param avcl used in av_log
 * @param header pointer to the header
 * @param line one header line
 * @param metadata used to store metadata while decoding
 * @return 0 if successful otherwise AVERROR_INVALIDDATA
 */
int avpriv_fits_header_parse_line(void *avcl, FITSHeader *header, const uint8_t line[80], AVDictionary ***metadata);

#endif /* AVCODEC_FITS_H */
