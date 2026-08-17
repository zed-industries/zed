/*
 * JPEG XL Header Parser
 * Copyright (c) 2023 Leo Izen <leo.izen@gmail.com>
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

#ifndef AVCODEC_JPEGXL_PARSE_H
#define AVCODEC_JPEGXL_PARSE_H

#include <stdint.h>

#include "libavutil/rational.h"

#include "jpegxl.h"

typedef struct FFJXLMetadata {
    uint32_t width;
    uint32_t height;
    uint32_t coded_width;
    uint32_t coded_height;
    int bit_depth;
    int have_alpha;
    /*
     * offset, in bits, of the animation header
     * zero if not animated
     */
    int animation_offset;
    AVRational timebase;
    FFJXLColorSpace csp;
    FFJXLWhitePoint wp;
    FFJXLPrimaries primaries;
    FFJXLTransferCharacteristic trc;

    /* used by the parser */
    int xyb_encoded;
    int have_icc_profile;
    int have_timecodes;
    uint32_t num_extra_channels;
} FFJXLMetadata;

/*
 * copies as much of the codestream into the buffer as possible
 * pass a shorter buflen to request less
 * returns the number of bytes consumed from input, may be greater than input_len
 * if the input doesn't end on an ISOBMFF-box boundary
 */
int ff_jpegxl_collect_codestream_header(const uint8_t *input_buffer, int input_len,
                                        uint8_t *buffer, int buflen, int *copied);

/*
 * Parse the codestream header with the provided buffer. Returns negative upon failure,
 * or the number of bits consumed upon success.
 * The FFJXLMetadata parameter may be NULL, in which case it's ignored.
 */
int ff_jpegxl_parse_codestream_header(const uint8_t *buf, int buflen, FFJXLMetadata *meta, int validate);

#endif /* AVCODEC_JPEGXL_PARSE_H */
