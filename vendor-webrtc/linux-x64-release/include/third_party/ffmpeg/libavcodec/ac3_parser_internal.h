/*
 * AC-3 parser internal code
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

#ifndef AVCODEC_AC3_PARSER_INTERNAL_H
#define AVCODEC_AC3_PARSER_INTERNAL_H

#include <stddef.h>
#include <stdint.h>

#include "ac3defs.h"
#include "get_bits.h"

/**
 * @struct AC3HeaderInfo
 * Coded AC-3 header values up to the lfeon element, plus derived values.
 */
typedef struct AC3HeaderInfo {
    /** @name Coded elements
     * @{
     */
    uint16_t sync_word;
    uint16_t crc1;
    uint8_t sr_code;
    uint8_t bitstream_id;
    uint8_t bitstream_mode;
    uint8_t channel_mode;
    uint8_t lfe_on;
    uint8_t frame_type;
    int substreamid;                        ///< substream identification
    int center_mix_level;                   ///< Center mix level index
    int surround_mix_level;                 ///< Surround mix level index
    uint16_t channel_map;
    int num_blocks;                         ///< number of audio blocks
    int dolby_surround_mode;
    /** @} */

    /** @name Derived values
     * @{
     */
    uint8_t sr_shift;
    uint16_t sample_rate;
    uint32_t bit_rate;
    uint8_t channels;
    uint16_t frame_size;
    uint64_t channel_layout;
    int8_t ac3_bit_rate_code;
    /** @} */
} AC3HeaderInfo;

typedef enum {
    AC3_PARSE_ERROR_SYNC        = -0x1030c0a,
    AC3_PARSE_ERROR_BSID        = -0x2030c0a,
    AC3_PARSE_ERROR_SAMPLE_RATE = -0x3030c0a,
    AC3_PARSE_ERROR_FRAME_SIZE  = -0x4030c0a,
    AC3_PARSE_ERROR_FRAME_TYPE  = -0x5030c0a,
    AC3_PARSE_ERROR_CRC         = -0x6030c0a,
} AC3ParseError;

/**
 * Parse AC-3 frame header.
 * Parse the header up to the lfeon element, which is the first 52 or 54 bits
 * depending on the audio coding mode.
 * @param[in]  gbc BitContext containing the first 54 bits of the frame.
 * @param[out] hdr Pointer to struct where header info is written.
 * @return 0 on success and AC3_PARSE_ERROR_* values otherwise.
 */
int ff_ac3_parse_header(GetBitContext *gbc, AC3HeaderInfo *hdr);

int avpriv_ac3_parse_header(AC3HeaderInfo **hdr, const uint8_t *buf,
                            size_t size);

int ff_ac3_find_syncword(const uint8_t *buf, int buf_size);

#endif /* AVCODEC_AC3_PARSER_INTERNAL_H */
