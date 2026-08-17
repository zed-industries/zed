/*
 * AAC ADTS header decoding prototypes and structures
 * Copyright (c) 2003 Fabrice Bellard
 * Copyright (c) 2003 Michael Niedermayer
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

#ifndef AVCODEC_ADTS_HEADER_H
#define AVCODEC_ADTS_HEADER_H

#include "adts_parser.h"
#include "defs.h"

typedef enum {
    AAC_PARSE_ERROR_SYNC        = -0x1030c0a,
    AAC_PARSE_ERROR_SAMPLE_RATE = -0x3030c0a,
    AAC_PARSE_ERROR_FRAME_SIZE  = -0x4030c0a,
} AACParseError;

typedef struct AACADTSHeaderInfo {
    uint32_t sample_rate;
    uint32_t samples;
    uint32_t bit_rate;
    uint8_t  crc_absent;
    uint8_t  object_type;
    uint8_t  sampling_index;
    uint8_t  chan_config;
    uint8_t  num_aac_frames;
    uint32_t frame_length;
} AACADTSHeaderInfo;

struct GetBitContext;

/**
 * Parse the ADTS frame header to the end of the variable header, which is
 * the first 54 bits.
 * @param[in]  gbc BitContext containing the first 54 bits of the frame.
 * @param[out] hdr Pointer to struct where header info is written.
 * @return the size in bytes of the header parsed on success and
 *         AAC_PARSE_ERROR_* values otherwise.
 */
int ff_adts_header_parse(struct GetBitContext *gbc, AACADTSHeaderInfo *hdr);

/**
 * Wrapper around ff_adts_header_parse() for users that don't already have
 * a suitable GetBitContext.
 */
int ff_adts_header_parse_buf(const uint8_t buf[AV_AAC_ADTS_HEADER_SIZE + AV_INPUT_BUFFER_PADDING_SIZE],
                             AACADTSHeaderInfo *hdr);

/**
 * Parse the ADTS frame header contained in the buffer, which is
 * the first 54 bits.
 * @param[in]  buf  Pointer to buffer containing the first 54 bits of the frame.
 * @param[in]  size Size of buffer containing the first 54 bits of the frame.
 * @param[out] phdr Pointer to pointer to struct AACADTSHeaderInfo for which
 * memory is allocated and header info is written into it. After using the header
 * information, the allocated memory must be freed by using av_free.
 * @return 0 on success, AAC_PARSE_ERROR_* values on invalid input and
 *         ordinary AVERROR codes otherwise.
 */
int avpriv_adts_header_parse(AACADTSHeaderInfo **phdr, const uint8_t *buf, size_t size);

#endif /* AVCODEC_ADTS_HEADER_H */
