/*
 * Shared definitions and helper functions for
 * AV1 (de)packetization.
 * Copyright (c) 2024 Axis Communications
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
 * @brief shared defines and functions for AV1 RTP dec/enc
 * @author Chris Hodges <chris.hodges@axis.com>
 */

#ifndef AVFORMAT_RTP_AV1_H
#define AVFORMAT_RTP_AV1_H

#include <stdint.h>

#include "libavutil/log.h"

// define a couple of flags and bit fields
#define AV1B_OBU_FORBIDDEN      7
#define AV1F_OBU_FORBIDDEN      (1u << AV1B_OBU_FORBIDDEN)
#define AV1S_OBU_TYPE           3
#define AV1M_OBU_TYPE           15
#define AV1B_OBU_EXTENSION_FLAG 2
#define AV1F_OBU_EXTENSION_FLAG (1u << AV1B_OBU_EXTENSION_FLAG)
#define AV1B_OBU_HAS_SIZE_FIELD 1
#define AV1F_OBU_HAS_SIZE_FIELD (1u << AV1B_OBU_HAS_SIZE_FIELD)
#define AV1B_OBU_RESERVED_1BIT  0
#define AV1F_OBU_RESERVED_1BIT  (1u << AV1B_OBU_RESERVED_1BIT)

#define AV1B_AGGR_HDR_FRAG_CONT 7
#define AV1F_AGGR_HDR_FRAG_CONT (1u << AV1B_AGGR_HDR_FRAG_CONT)
#define AV1B_AGGR_HDR_LAST_FRAG 6
#define AV1F_AGGR_HDR_LAST_FRAG (1u << AV1B_AGGR_HDR_LAST_FRAG)
#define AV1S_AGGR_HDR_NUM_OBUS  4
#define AV1M_AGGR_HDR_NUM_OBUS  3
#define AV1B_AGGR_HDR_FIRST_PKT 3
#define AV1F_AGGR_HDR_FIRST_PKT (1u << AV1B_AGGR_HDR_FIRST_PKT)

/// calculate number of required LEB bytes for the given length
static inline unsigned int calc_leb_size(uint32_t length) {
    unsigned int num_lebs = 0;
    do {
        num_lebs++;
        length >>= 7;
    } while (length);
    return num_lebs;
}

/// write out variable number of LEB bytes for the given length
static inline unsigned int write_leb(uint8_t *lebptr, uint32_t length) {
    unsigned int num_lebs = 0;
    do {
        num_lebs++;
        if (length < 0x80) {
            *lebptr = length;
            break;
        }
        *lebptr++ = length | 0x80; // no need to mask out
        length >>= 7;
    } while (1);
    return num_lebs;
}

/// write out fixed number of LEB bytes (may have "unused" bytes)
static inline void write_leb_n(uint8_t *lebptr, uint32_t length, unsigned int num_lebs) {
    for (int i = 0; i < num_lebs; i++) {
        if (i == num_lebs - 1) {
            *lebptr = length & 0x7f;
        } else {
            *lebptr++ = length | 0x80; // no need to mask out
        }
        length >>= 7;
    }
}

/// securely parse LEB bytes and return the resulting encoded length
static inline unsigned int parse_leb(void *logctx, const uint8_t *buf_ptr,
                                     uint32_t buffer_size, uint32_t *obu_size) {
    uint8_t leb128;
    unsigned int num_lebs = 0;
    *obu_size = 0;
    do {
        uint32_t leb7;
        if (!buffer_size) {
            av_log(logctx, AV_LOG_ERROR, "AV1: Out of data in OBU size field AV1 RTP packet\n");
            return 0;
        }
        leb128 = *buf_ptr++;
        leb7 = leb128 & 0x7f;
        buffer_size--;
        /* AV1 spec says that the maximum value returned from leb128 must fit in
         * 32 bits, so if the next byte will shift data out, we have some kind
         * of violation here. It is legal, though, to have the most significant
         * bytes with all zero bits (in the lower 7 bits). */
        if (((num_lebs == 4) && (leb7 >= 0x10)) || ((num_lebs > 4) && leb7)) {
            av_log(logctx, AV_LOG_ERROR, "AV1: OBU size field exceeds 32 bit in AV1 RTP packet\n");
            return 0;
        }
        if ((num_lebs == 7) && (leb128 >= 0x80)) {
            /* leb128 is defined to be up to 8 bytes (why???), 8th byte MUST NOT
             * indicate continuation */
            av_log(logctx, AV_LOG_ERROR, "AV1: OBU size field consists of too many bytes in AV1 RTP packet\n");
            return 0;
        }
        // shifts >= 32 are undefined in C!
        if (num_lebs <= 4) {
            *obu_size |= leb7 << (7 * num_lebs);
        }
        num_lebs++;
    } while (leb128 >= 0x80);
    return num_lebs;
}

#endif /* AVFORMAT_RTP_AV1_H */
