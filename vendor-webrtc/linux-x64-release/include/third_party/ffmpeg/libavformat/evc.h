/*
 * EVC helper functions for muxers
 * Copyright (c) 2022 Dawid Kozinski <d.kozinski@samsung.com>
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

#ifndef AVFORMAT_EVC_H
#define AVFORMAT_EVC_H

#include <stdint.h>

#include "libavutil/intreadwrite.h"
#include "libavutil/rational.h"
#include "libavcodec/evc.h"
#include "avio.h"

static inline int evc_get_nalu_type(const uint8_t *p, int bits_size)
{
    int unit_type_plus1 = 0;

    if (bits_size >= EVC_NALU_HEADER_SIZE) {
        // forbidden_zero_bit
        if ((p[0] & 0x80) != 0)   // Cannot get bitstream information. Malformed bitstream.
            return -1;

        // nal_unit_type
        unit_type_plus1 = (p[0] >> 1) & 0x3F;
    }

    return unit_type_plus1 - 1;
}

static inline uint32_t evc_read_nal_unit_length(const uint8_t *bits, int bits_size)
{
    if (bits_size >= EVC_NALU_LENGTH_PREFIX_SIZE)
        return AV_RB32(bits);

    return 0;
}

/**
 * Writes EVC sample metadata to the provided AVIOContext.
 *
 * @param pb pointer to the AVIOContext where the evc sample metadata shall be written
 * @param buf input data buffer
 * @param size size in bytes of the input data buffer
 * @param ps_array_completeness @see ISO/IEC 14496-15:2021 Coding of audio-visual objects - Part 15: section 12.3.3.3
 *
 * @return 0 in case of success, a negative error code in case of failure
 */
int ff_isom_write_evcc(AVIOContext *pb, const uint8_t *data,
                       int size, int ps_array_completeness);

#endif // AVFORMAT_EVC_H
