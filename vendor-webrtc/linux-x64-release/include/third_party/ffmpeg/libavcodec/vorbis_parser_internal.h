/*
 * Copyright (c) 2012 Justin Ruggles
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
 * Vorbis audio parser
 *
 * Determines the duration for each packet.
 */

#ifndef AVCODEC_VORBIS_PARSER_INTERNAL_H
#define AVCODEC_VORBIS_PARSER_INTERNAL_H

#include "avcodec.h"
#include "vorbis_parser.h"

struct AVVorbisParseContext {
    const AVClass *class;
    int extradata_parsed;       ///< we have attempted to parse extradata
    int valid_extradata;        ///< extradata is valid, so we can calculate duration
    int blocksize[2];           ///< short and long window sizes
    int previous_blocksize;     ///< previous window size
    int mode_blocksize[64];     ///< window size mapping for each mode
    int mode_count;             ///< number of modes
    int mode_mask;              ///< bitmask used to get the mode in each packet
    int prev_mask;              ///< bitmask used to get the previous mode flag in each packet
};

#endif /* AVCODEC_VORBIS_PARSER_INTERNAL_H */
