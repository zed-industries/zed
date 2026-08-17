/*
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
 * EVC decoder/parser shared code
 */

#ifndef AVCODEC_EVC_PARSE_H
#define AVCODEC_EVC_PARSE_H

#include <stdint.h>

#include "libavutil/intreadwrite.h"
#include "libavutil/log.h"
#include "evc.h"
#include "evc_ps.h"

// The sturcture reflects Slice Header RBSP(raw byte sequence payload) layout
// @see ISO_IEC_23094-1 section 7.3.2.6
//
// The following descriptors specify the parsing process of each element
// u(n)  - unsigned integer using n bits
// ue(v) - unsigned integer 0-th order Exp_Golomb-coded syntax element with the left bit first
// u(n)  - unsigned integer using n bits.
//         When n is "v" in the syntax table, the number of bits varies in a manner dependent on the value of other syntax elements.
typedef struct EVCParserSliceHeader {
    uint8_t slice_pic_parameter_set_id;                                      // ue(v)
    uint8_t single_tile_in_slice_flag;                                       // u(1)
    uint8_t first_tile_id;                                                   // u(v)
    uint8_t arbitrary_slice_flag;                                            // u(1)
    uint8_t last_tile_id;                                                    // u(v)
    uint32_t num_remaining_tiles_in_slice_minus1;                            // ue(v)
    uint16_t delta_tile_id_minus1[EVC_MAX_TILE_ROWS * EVC_MAX_TILE_COLUMNS]; // ue(v)

    uint8_t slice_type;                                                      // ue(v)
    uint8_t no_output_of_prior_pics_flag;                                    // u(1)
    uint8_t mmvd_group_enable_flag;                                          // u(1)
    uint8_t slice_alf_enabled_flag;                                          // u(1)

    uint8_t slice_alf_luma_aps_id;                                           // u(5)
    uint8_t slice_alf_map_flag;                                              // u(1)
    uint8_t slice_alf_chroma_idc;                                            // u(2)
    uint8_t slice_alf_chroma_aps_id;                                         // u(5)
    uint8_t slice_alf_chroma_map_flag;                                       // u(1)
    uint8_t slice_alf_chroma2_aps_id;                                        // u(5)
    uint8_t slice_alf_chroma2_map_flag;                                      // u(1)
    uint16_t slice_pic_order_cnt_lsb;                                        // u(v)

    // @note
    // Currently the structure does not reflect the entire Slice Header RBSP layout.
    // It contains only the fields that are necessary to read from the NAL unit all the values
    // necessary for the correct initialization of the AVCodecContext structure.

    // @note
    // If necessary, add the missing fields to the structure to reflect
    // the contents of the entire NAL unit of the SPS type

} EVCParserSliceHeader;

// picture order count of the current picture
typedef struct EVCParserPoc {
    int PicOrderCntVal;     // current picture order count value
    int prevPicOrderCntVal; // the picture order count of the previous Tid0 picture
    int DocOffset;          // the decoding order count of the previous picture
} EVCParserPoc;

static inline uint32_t evc_read_nal_unit_length(const uint8_t *bits, int bits_size, void *logctx)
{
    uint32_t nalu_len = 0;

    if (bits_size < EVC_NALU_LENGTH_PREFIX_SIZE) {
        av_log(logctx, AV_LOG_ERROR, "Can't read NAL unit length\n");
        return 0;
    }

    nalu_len = AV_RB32(bits);

    return nalu_len;
}

int ff_evc_parse_slice_header(GetBitContext *gb, EVCParserSliceHeader *sh,
                              const EVCParamSets *ps, enum EVCNALUnitType nalu_type);

// POC (picture order count of the current picture) derivation
// @see ISO/IEC 23094-1:2020(E) 8.3.1 Decoding process for picture order count
int ff_evc_derive_poc(const EVCParamSets *ps, const EVCParserSliceHeader *sh,
                      EVCParserPoc *poc, enum EVCNALUnitType nalu_type, int tid);

#endif /* AVCODEC_EVC_PARSE_H */
