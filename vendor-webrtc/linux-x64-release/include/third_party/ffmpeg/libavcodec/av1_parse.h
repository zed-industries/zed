/*
 * AV1 common parsing code
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

#ifndef AVCODEC_AV1_PARSE_H
#define AVCODEC_AV1_PARSE_H

#include <limits.h>
#include <stdint.h>

#include "libavutil/error.h"
#include "libavutil/intmath.h"
#include "libavutil/macros.h"

#include "av1.h"
#include "get_bits.h"
#include "leb.h"

// OBU header fields + max leb128 length
#define MAX_OBU_HEADER_SIZE (2 + 8)

typedef struct AV1OBU {
    /** Size of payload */
    int size;
    const uint8_t *data;

    /**
     * Size, in bits, of just the data, excluding the trailing_one_bit and
     * any trailing padding.
     */
    int size_bits;

    /** Size of entire OBU, including header */
    int raw_size;
    const uint8_t *raw_data;

    int type;

    int temporal_id;
    int spatial_id;
} AV1OBU;

/** An input packet split into OBUs */
typedef struct AV1Packet {
    AV1OBU *obus;
    int nb_obus;
    int obus_allocated;
    unsigned obus_allocated_size;
} AV1Packet;

/**
 * Extract an OBU from a raw bitstream.
 *
 * @note This function does not copy or store any bitstream data. All
 *       the pointers in the AV1OBU structure will be valid as long
 *       as the input buffer also is.
 */
int ff_av1_extract_obu(AV1OBU *obu, const uint8_t *buf, int length,
                       void *logctx);

/**
 * Split an input packet into OBUs.
 *
 * @note This function does not copy or store any bitstream data. All
 *       the pointers in the AV1Packet structure will be valid as
 *       long as the input buffer also is.
 */
int ff_av1_packet_split(AV1Packet *pkt, const uint8_t *buf, int length,
                        void *logctx);

/**
 * Free all the allocated memory in the packet.
 */
void ff_av1_packet_uninit(AV1Packet *pkt);

static inline int parse_obu_header(const uint8_t *buf, int buf_size,
                                   int64_t *obu_size, int *start_pos, int *type,
                                   int *temporal_id, int *spatial_id)
{
    GetBitContext gb;
    int ret, extension_flag, has_size_flag;
    int64_t size;

    ret = init_get_bits8(&gb, buf, FFMIN(buf_size, MAX_OBU_HEADER_SIZE));
    if (ret < 0)
        return ret;

    if (get_bits1(&gb) != 0) // obu_forbidden_bit
        return AVERROR_INVALIDDATA;

    *type      = get_bits(&gb, 4);
    extension_flag = get_bits1(&gb);
    has_size_flag  = get_bits1(&gb);
    skip_bits1(&gb); // obu_reserved_1bit

    if (extension_flag) {
        *temporal_id = get_bits(&gb, 3);
        *spatial_id  = get_bits(&gb, 2);
        skip_bits(&gb, 3); // extension_header_reserved_3bits
    } else {
        *temporal_id = *spatial_id = 0;
    }

    *obu_size  = has_size_flag ? get_leb128(&gb)
                               : buf_size - 1 - extension_flag;

    if (get_bits_left(&gb) < 0)
        return AVERROR_INVALIDDATA;

    *start_pos = get_bits_count(&gb) / 8;

    size = *obu_size + *start_pos;

    if (size > buf_size)
        return AVERROR_INVALIDDATA;

    return size;
}

static inline int get_obu_bit_length(const uint8_t *buf, int size, int type)
{
    int v;

    /* There are no trailing bits on these */
    if (type == AV1_OBU_TILE_GROUP ||
        type == AV1_OBU_TILE_LIST ||
        type == AV1_OBU_FRAME) {
        if (size > INT_MAX / 8)
            return AVERROR(ERANGE);
        else
            return size * 8;
    }

    while (size > 0 && buf[size - 1] == 0)
        size--;

    if (!size)
        return 0;

    v = buf[size - 1];

    if (size > INT_MAX / 8)
        return AVERROR(ERANGE);
    size *= 8;

    /* Remove the trailing_one_bit and following trailing zeros */
    if (v)
        size -= ff_ctz(v) + 1;

    return size;
}

AVRational ff_av1_framerate(int64_t ticks_per_frame, int64_t units_per_tick,
                            int64_t time_scale);

#endif /* AVCODEC_AV1_PARSE_H */
