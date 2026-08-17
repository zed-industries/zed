/*
 * H.266 / VVC shared code
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

#ifndef AVCODEC_VVC_H
#define AVCODEC_VVC_H

/**
 * Table 5 – NAL unit type codes and NAL unit type classes
 * in T-REC-H.266-202008
 */
enum VVCNALUnitType {
    VVC_TRAIL_NUT      = 0,
    VVC_STSA_NUT       = 1,
    VVC_RADL_NUT       = 2,
    VVC_RASL_NUT       = 3,
    VVC_RSV_VCL_4      = 4,
    VVC_RSV_VCL_5      = 5,
    VVC_RSV_VCL_6      = 6,
    VVC_IDR_W_RADL     = 7,
    VVC_IDR_N_LP       = 8,
    VVC_CRA_NUT        = 9,
    VVC_GDR_NUT        = 10,
    VVC_RSV_IRAP_11    = 11,
    VVC_OPI_NUT        = 12,
    VVC_DCI_NUT        = 13,
    VVC_VPS_NUT        = 14,
    VVC_SPS_NUT        = 15,
    VVC_PPS_NUT        = 16,
    VVC_PREFIX_APS_NUT = 17,
    VVC_SUFFIX_APS_NUT = 18,
    VVC_PH_NUT         = 19,
    VVC_AUD_NUT        = 20,
    VVC_EOS_NUT        = 21,
    VVC_EOB_NUT        = 22,
    VVC_PREFIX_SEI_NUT = 23,
    VVC_SUFFIX_SEI_NUT = 24,
    VVC_FD_NUT         = 25,
    VVC_RSV_NVCL_26    = 26,
    VVC_RSV_NVCL_27    = 27,
    VVC_UNSPEC_28      = 28,
    VVC_UNSPEC_29      = 29,
    VVC_UNSPEC_30      = 30,
    VVC_UNSPEC_31      = 31,
};

enum VVCSliceType {
    VVC_SLICE_TYPE_B = 0,
    VVC_SLICE_TYPE_P = 1,
    VVC_SLICE_TYPE_I = 2,
};

enum VVCAPSType {
    VVC_ASP_TYPE_ALF     = 0,
    VVC_ASP_TYPE_LMCS    = 1,
    VVC_ASP_TYPE_SCALING = 2,
};

enum {
    //6.2 we can have 3 sample arrays
    VVC_MAX_SAMPLE_ARRAYS = 3,

    //7.4.3.3 vps_max_layers_minus1 is u(6)
    VVC_MAX_LAYERS = 64,

    //7.4.3.3 The value of vps_max_sublayers_minus1 shall be in the range of 0 to 6, inclusive
    VVC_MAX_SUBLAYERS = 7,

    //7.3.2.1 dci_num_ptls_minus1 is u(4)
    VVC_MAX_DCI_PTLS = 16,

    //7.4.3.3 vps_num_ptls_minus1 is u(8)
    VVC_MAX_PTLS = 256,

    //7.4.3.3 vps_num_output_layer_sets_minus2 is u(8)
    VVC_MAX_TOTAL_NUM_OLSS = 257,

    // 7.3.2.3: vps_video_parameter_set_id is u(4).
    VVC_MAX_VPS_COUNT = 16,
    // 7.3.2.4: sps_seq_parameter_set_id is u(4)
    VVC_MAX_SPS_COUNT = 16,
    // 7.3.2.5: pps_pic_parameter_set_id is u(6)
    VVC_MAX_PPS_COUNT = 64,

    // 7.4.4.1: ptl_num_sub_profiles is u(8)
    VVC_MAX_SUB_PROFILES = 256,

    // 7.4.3.18: The variable NumAlfFilters specifying the number of different adaptive loop
    // filters is set equal to 25.
    VVC_NUM_ALF_FILTERS = 25,

    // A.4.2: according to (1577), MaxDpbSize is bounded above by 2 * maxDpbPicBuf(8)
    VVC_MAX_DPB_SIZE = 16,

    //7.4.3.4 sps_num_ref_pic_lists in range [0, 64]
    VVC_MAX_REF_PIC_LISTS = 64,

    //7.4.11 num_ref_entries in range [0, MaxDpbSize + 13]
    VVC_MAX_REF_ENTRIES = VVC_MAX_DPB_SIZE + 13,

    //7.4.3.3 sps_num_points_in_qp_table_minus1[i] in range [0, 36 − sps_qp_table_start_minus26[i]],
    //and sps_qp_table_start_minus26[i] in range [−26 − QpBdOffset, 36].
    //so sps_num_points_in_qp_table_minus1[i] should in range [0, 62 + QpBdOffset]
    //since 16 bits QpBdOffset is 48, sps_num_points_in_qp_table_minus1[i] should range [0, 110]
    VVC_MAX_POINTS_IN_QP_TABLE = 111,

    // 7.4.6.1: hrd_cpb_cnt_minus1 is in [0, 31].
    VVC_MAX_CPB_CNT = 32,

    // A.4.1: the highest level allows a MaxLumaPs of 80,216,064.
    VVC_MAX_LUMA_PS = 80216064,

    // A.4.1: pic_width_in_luma_samples and pic_height_in_luma_samples are
    // constrained to be not greater than sqrt(MaxLumaPs * 8).  Hence height/
    // width are bounded above by sqrt(8 * 80216064) = 25332.4 samples.
    VVC_MAX_WIDTH  = 25332,
    VVC_MAX_HEIGHT = 25332,

    // A.4.1: table A.2 allows at most 990 tiles per AU for any level.
    VVC_MAX_TILES_PER_AU = 990,
    // A.4.1: table A.2 did not define max tile rows.
    // in worest a case, we can have 1x990 tiles picture.
    VVC_MAX_TILE_ROWS    = VVC_MAX_TILES_PER_AU,
    // A.4.1: table A.2 allows at most 30 tile columns for any level.
    VVC_MAX_TILE_COLUMNS = 30,

    // A.4.1 table A.2 allows at most 1000 slices for any level.
    VVC_MAX_SLICES = 1000,

    // 7.4.8: in the worst case (!pps_no_pic_partition_flag and
    // sps_entropy_coding_sync_enabled_flag are both true), entry points can be
    // placed at the beginning of every Ctb row in every tile, giving an
    // upper bound of (num_tile_columns_minus1 + 1) * PicHeightInCtbsY - 1.
    // Only a stream with very high resolution and perverse parameters could
    // get near that, though, so set a lower limit here with the maximum
    // possible value for 8K video (at most 135 32x32 Ctb rows).
    VVC_MAX_ENTRY_POINTS = VVC_MAX_TILE_COLUMNS * 135,

    // {sps, ph}_num_{ver, hor}_virtual_boundaries should in [0, 3]
    VVC_MAX_VBS = 3,
};

#endif /* AVCODEC_VVC_H */
