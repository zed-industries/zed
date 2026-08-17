/*
 * EVC definitions and enums
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

#ifndef AVCODEC_EVC_H
#define AVCODEC_EVC_H

// The length field that indicates the length in bytes of the following NAL unit is configured to be of 4 bytes
#define EVC_NALU_LENGTH_PREFIX_SIZE     (4)  /* byte */
#define EVC_NALU_HEADER_SIZE            (2)  /* byte */

/**
 * @see ISO_IEC_23094-1_2020, 7.4.2.2 NAL unit header semantic
 *      Table 4 - NAL unit type codes and NAL unit type classes
 */
enum EVCNALUnitType {
    EVC_NOIDR_NUT            = 0,   /* Coded slice of a non-IDR picture */
    EVC_IDR_NUT              = 1,   /* Coded slice of an IDR picture */
    EVC_RSV_VCL_NUT02        = 2,
    EVC_RSV_VCL_NUT03        = 3,
    EVC_RSV_VCL_NUT04        = 4,
    EVC_RSV_VCL_NUT05        = 5,
    EVC_RSV_VCL_NUT06        = 6,
    EVC_RSV_VCL_NUT07        = 7,
    EVC_RSV_VCL_NUT08        = 8,
    EVC_RSV_VCL_NUT09        = 9,
    EVC_RSV_VCL_NUT10        = 10,
    EVC_RSV_VCL_NUT11        = 11,
    EVC_RSV_VCL_NUT12        = 12,
    EVC_RSV_VCL_NUT13        = 13,
    EVC_RSV_VCL_NUT14        = 14,
    EVC_RSV_VCL_NUT15        = 15,
    EVC_RSV_VCL_NUT16        = 16,
    EVC_RSV_VCL_NUT17        = 17,
    EVC_RSV_VCL_NUT18        = 18,
    EVC_RSV_VCL_NUT19        = 19,
    EVC_RSV_VCL_NUT20        = 20,
    EVC_RSV_VCL_NUT21        = 21,
    EVC_RSV_VCL_NUT22        = 22,
    EVC_RSV_VCL_NUT23        = 23,
    EVC_SPS_NUT              = 24,  /* Sequence parameter set */
    EVC_PPS_NUT              = 25,  /* Picture paremeter set */
    EVC_APS_NUT              = 26,  /* Adaptation parameter set */
    EVC_FD_NUT               = 27,  /* Filler data */
    EVC_SEI_NUT              = 28,  /* Supplemental enhancement information */
    EVC_RSV_NONVCL29         = 29,
    EVC_RSV_NONVCL30         = 30,
    EVC_RSV_NONVCL31         = 31,
    EVC_RSV_NONVCL32         = 32,
    EVC_RSV_NONVCL33         = 33,
    EVC_RSV_NONVCL34         = 34,
    EVC_RSV_NONVCL35         = 35,
    EVC_RSV_NONVCL36         = 36,
    EVC_RSV_NONVCL37         = 37,
    EVC_RSV_NONVCL38         = 38,
    EVC_RSV_NONVCL39         = 39,
    EVC_RSV_NONVCL40         = 40,
    EVC_RSV_NONVCL41         = 41,
    EVC_RSV_NONVCL42         = 42,
    EVC_RSV_NONVCL43         = 43,
    EVC_RSV_NONVCL44         = 44,
    EVC_RSV_NONVCL45         = 45,
    EVC_RSV_NONVCL46         = 46,
    EVC_RSV_NONVCL47         = 47,
    EVC_RSV_NONVCL48         = 48,
    EVC_RSV_NONVCL49         = 49,
    EVC_RSV_NONVCL50         = 50,
    EVC_RSV_NONVCL51         = 51,
    EVC_RSV_NONVCL52         = 52,
    EVC_RSV_NONVCL53         = 53,
    EVC_RSV_NONVCL54         = 54,
    EVC_RSV_NONVCL55         = 55,
    EVC_UNSPEC_NUT56         = 56,
    EVC_UNSPEC_NUT57         = 57,
    EVC_UNSPEC_NUT58         = 58,
    EVC_UNSPEC_NUT59         = 59,
    EVC_UNSPEC_NUT60         = 60,
    EVC_UNSPEC_NUT61         = 61,
    EVC_UNSPEC_NUT62         = 62
};

// slice type
// @see ISO_IEC_23094-1_2020 7.4.5 Slice header semantics
//
enum EVCSliceType {
    EVC_SLICE_TYPE_B = 0,
    EVC_SLICE_TYPE_P = 1,
    EVC_SLICE_TYPE_I = 2
};

enum {
    // 7.4.3.1: sps_seq_parameter_set_id is in [0, 15].
    EVC_MAX_SPS_COUNT = 16,

    // 7.4.3.2: pps_pic_parameter_set_id is in [0, 63].
    EVC_MAX_PPS_COUNT = 64,

    // 7.4.3.3: adaptional_parameter_set_id is in [0, 31].
    EVC_MAX_APS_COUNT = 32,

    // 7.4.5: slice header slice_pic_parameter_set_id in [0, 63]
    EVC_MAX_SH_COUNT = 64,

    // E.3.2: cpb_cnt_minus1[i] is in [0, 31].
    EVC_MAX_CPB_CNT = 32,

    // A.4.1: in table A.1 the highest level allows a MaxLumaPs of 35 651 584.
    EVC_MAX_LUMA_PS = 35651584,

    EVC_MAX_NUM_REF_PICS = 21,

    EVC_MAX_NUM_RPLS = 64,

    // A.4.1: pic_width_in_luma_samples and pic_height_in_luma_samples are
    // constrained to be not greater than sqrt(MaxLumaPs * 8).  Hence height/
    // width are bounded above by sqrt(8 * 35651584) = 16888.2 samples.
    EVC_MAX_WIDTH  = 16888,
    EVC_MAX_HEIGHT = 16888,

    // A.4.1: table A.1 allows at most 22 tile rows for any level.
    EVC_MAX_TILE_ROWS    = 22,
    // A.4.1: table A.1 allows at most 20 tile columns for any level.
    EVC_MAX_TILE_COLUMNS = 20,

    // A.4.1: table A.1 allows at most 600 slice segments for any level.
    EVC_MAX_SLICE_SEGMENTS = 600,
};

#endif // AVCODEC_EVC_H
