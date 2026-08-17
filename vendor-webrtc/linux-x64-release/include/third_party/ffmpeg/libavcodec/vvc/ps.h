/*
 * VVC parameter set parser
 *
 * Copyright (C) 2023 Nuo Mi
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

#ifndef AVCODEC_VVC_PS_H
#define AVCODEC_VVC_PS_H

#include "libavcodec/cbs_h266.h"
#include "libavcodec/vvc.h"

#define IS_IDR(s)  ((s)->vcl_unit_type == VVC_IDR_W_RADL || (s)->vcl_unit_type == VVC_IDR_N_LP)
#define IS_CRA(s)  ((s)->vcl_unit_type == VVC_CRA_NUT)
#define IS_IRAP(s) (IS_IDR(s) || IS_CRA(s))
#define IS_GDR(s)  ((s)->vcl_unit_type == VVC_GDR_NUT)
#define IS_CVSS(s) (IS_IRAP(s)|| IS_GDR(s))
#define IS_CLVSS(s) (IS_CVSS(s) && s->no_output_before_recovery_flag)
#define IS_RASL(s) ((s)->vcl_unit_type == VVC_RASL_NUT)
#define IS_RADL(s) ((s)->vcl_unit_type == VVC_RADL_NUT)

#define IS_I(rsh) ((rsh)->sh_slice_type == VVC_SLICE_TYPE_I)
#define IS_P(rsh) ((rsh)->sh_slice_type == VVC_SLICE_TYPE_P)
#define IS_B(rsh) ((rsh)->sh_slice_type == VVC_SLICE_TYPE_B)

#define INV_POC INT_MIN
#define GDR_IS_RECOVERED(s)  (s->gdr_recovery_point_poc == INV_POC)
#define GDR_SET_RECOVERED(s) (s->gdr_recovery_point_poc =  INV_POC)

#define LMCS_MAX_BIT_DEPTH  12
#define LMCS_MAX_LUT_SIZE   (1 << LMCS_MAX_BIT_DEPTH)
#define LMCS_MAX_BIN_SIZE   16
#define LADF_MAX_INTERVAL   5

enum {
    CHROMA_FORMAT_MONO,
    CHROMA_FORMAT_420,
    CHROMA_FORMAT_422,
    CHROMA_FORMAT_444,
};

typedef struct VVCSPS {
    const H266RawSPS *r;                                            ///< RefStruct reference

    //derived values
    uint8_t     hshift[VVC_MAX_SAMPLE_ARRAYS];
    uint8_t     vshift[VVC_MAX_SAMPLE_ARRAYS];
    uint32_t    max_pic_order_cnt_lsb;                             ///< MaxPicOrderCntLsb

    uint8_t     pixel_shift;
    enum AVPixelFormat pix_fmt;

    uint8_t     bit_depth;                                          ///< BitDepth
    uint8_t     qp_bd_offset;                                       ///< QpBdOffset
    uint8_t     ctb_log2_size_y;                                    ///< CtbLog2SizeY
    uint16_t    ctb_size_y;                                         ///< CtbSizeY
    uint8_t     min_cb_log2_size_y;                                 ///< MinCbLog2SizeY
    uint8_t     min_cb_size_y;                                      ///< MinCbSizeY
    uint8_t     max_tb_size_y;                                      ///< MaxTbSizeY
    uint8_t     max_ts_size;                                        ///< MaxTsSize
    uint8_t     max_num_merge_cand;                                 ///< MaxNumMergeCand
    uint8_t     max_num_ibc_merge_cand;                             ///< MaxNumIbcMergeCand
    uint8_t     max_num_gpm_merge_cand;                             ///< MaxNumGpmMergeCand
    uint8_t     num_ladf_intervals;                                 ///< sps_num_ladf_intervals_minus2 + 2;
    uint32_t    ladf_interval_lower_bound[LADF_MAX_INTERVAL];       ///< SpsLadfIntervalLowerBound[]
    uint8_t     log2_parallel_merge_level;                          ///< sps_log2_parallel_merge_level_minus2 + 2;
    uint8_t     log2_transform_range;                               ///< Log2TransformRange
    int8_t      chroma_qp_table[3][VVC_MAX_POINTS_IN_QP_TABLE];     ///< ChromaQpTable
} VVCSPS;

typedef struct DBParams {
    int8_t beta_offset[VVC_MAX_SAMPLE_ARRAYS];
    int8_t tc_offset[VVC_MAX_SAMPLE_ARRAYS];
} DBParams;

typedef struct VVCPPS {
    const H266RawPPS *r;                                            ///< RefStruct reference

    //derived value;
    int8_t   chroma_qp_offset[3];                                   ///< pps_cb_qp_offset, pps_cr_qp_offset, pps_joint_cbcr_qp_offset_value
    int8_t   chroma_qp_offset_list[6][3];                           ///< pps_cb_qp_offset_list, pps_cr_qp_offset_list, pps_joint_cbcr_qp_offset_list

    uint16_t width;
    uint16_t height;

    uint16_t slice_start_offset[VVC_MAX_SLICES];
    uint16_t num_ctus_in_slice [VVC_MAX_SLICES];

    uint16_t min_cb_width;
    uint16_t min_cb_height;

    uint16_t ctb_width;
    uint16_t ctb_height;
    uint32_t ctb_count;

    uint16_t min_pu_width;
    uint16_t min_pu_height;
    uint16_t min_tu_width;
    uint16_t min_tu_height;

    uint32_t *ctb_addr_in_slice;            ///< CtbAddrInCurrSlice for entire picture
    uint16_t *col_bd;                       ///< TileColBdVal
    uint16_t *row_bd;                       ///< TileRowBdVal
    uint16_t *ctb_to_col_bd;                ///< CtbToTileColBd
    uint16_t *ctb_to_row_bd;                ///< CtbToTileRowBd

    uint16_t width32;                       ///< width  in 32 pixels
    uint16_t height32;                      ///< height in 32 pixels
    uint16_t width64;                       ///< width  in 64 pixels
    uint16_t height64;                      ///< height in 64 pixels

    uint16_t ref_wraparound_offset;         ///< PpsRefWraparoundOffset

    uint16_t subpic_x[VVC_MAX_SLICES];      ///< SubpicLeftBoundaryPos
    uint16_t subpic_y[VVC_MAX_SLICES];      ///< SubpicTopBoundaryPos
    uint16_t subpic_width[VVC_MAX_SLICES];
    uint16_t subpic_height[VVC_MAX_SLICES];
} VVCPPS;

#define MAX_WEIGHTS 15
typedef struct PredWeightTable {
    uint8_t log2_denom[2];                                      ///< luma_log2_weight_denom, ChromaLog2WeightDenom

    uint8_t nb_weights[2];                                      ///< num_l0_weights, num_l1_weights
    uint8_t weight_flag[2][2][MAX_WEIGHTS];                     ///< luma_weight_l0_flag, chroma_weight_l0_flag,
                                                                ///< luma_weight_l1_flag, chroma_weight_l1_flag,
    int16_t weight[2][VVC_MAX_SAMPLE_ARRAYS][MAX_WEIGHTS];      ///< LumaWeightL0, LumaWeightL1, ChromaWeightL0, ChromaWeightL1
    int16_t offset[2][VVC_MAX_SAMPLE_ARRAYS][MAX_WEIGHTS];      ///< luma_offset_l0, luma_offset_l1, ChromaOffsetL0, ChromaOffsetL1
} PredWeightTable;

typedef struct VVCPH {
    const H266RawPictureHeader *r;
    void *rref;                                     ///< RefStruct reference, backing ph above

    //derived values
    uint32_t max_num_subblock_merge_cand;           ///< MaxNumSubblockMergeCand
    int32_t  poc;                                   ///< PicOrderCntVal

    uint8_t  num_ver_vbs;                           ///< NumVerVirtualBoundaries
    uint16_t vb_pos_x[VVC_MAX_VBS];                 ///< VirtualBoundaryPosX
    uint8_t  num_hor_vbs;                           ///< NumHorVirtualBoundaries
    uint16_t vb_pos_y[VVC_MAX_VBS];                 ///< VirtualBoundaryPosY

    PredWeightTable pwt;
} VVCPH;

#define ALF_NUM_FILTERS_LUMA    25
#define ALF_NUM_FILTERS_CHROMA   8
#define ALF_NUM_FILTERS_CC       4

#define ALF_NUM_COEFF_LUMA      12
#define ALF_NUM_COEFF_CHROMA     6
#define ALF_NUM_COEFF_CC         7

typedef struct VVCALF {
    const H266RawAPS *r;
    int16_t luma_coeff     [ALF_NUM_FILTERS_LUMA][ALF_NUM_COEFF_LUMA];
    uint8_t luma_clip_idx  [ALF_NUM_FILTERS_LUMA][ALF_NUM_COEFF_LUMA];

    uint8_t num_chroma_filters;
    int16_t chroma_coeff   [ALF_NUM_FILTERS_CHROMA][ALF_NUM_COEFF_CHROMA];
    uint8_t chroma_clip_idx[ALF_NUM_FILTERS_CHROMA][ALF_NUM_COEFF_CHROMA];

    uint8_t num_cc_filters[2];        ///< alf_cc_cb_filters_signalled_minus1 + 1, alf_cc_cr_filters_signalled_minus1 + 1
    int16_t cc_coeff[2][ALF_NUM_FILTERS_CC][ALF_NUM_COEFF_CC];
} VVCALF;

enum {
  SL_START_2x2    = 0,
  SL_START_4x4    = 2,
  SL_START_8x8    = 8,
  SL_START_16x16  = 14,
  SL_START_32x32  = 20,
  SL_START_64x64  = 26,
  SL_MAX_ID       = 28,
};

#define SL_MAX_MATRIX_SIZE 8

typedef struct VVCScalingList {
    uint8_t scaling_matrix_rec[SL_MAX_ID][SL_MAX_MATRIX_SIZE * SL_MAX_MATRIX_SIZE];  ///< ScalingMatrixRec
    uint8_t scaling_matrix_dc_rec[SL_MAX_ID - SL_START_16x16];                       ///< ScalingMatrixDcRec[refId − 14]
} VVCScalingList;

typedef struct VVCLMCS {
    uint8_t  min_bin_idx;
    uint8_t  max_bin_idx;

    union {
        uint8_t  u8[LMCS_MAX_LUT_SIZE];
        uint16_t u16[LMCS_MAX_LUT_SIZE]; ///< for high bit-depth
    } fwd_lut, inv_lut;

    uint16_t pivot[LMCS_MAX_BIN_SIZE + 1];
    uint16_t chroma_scale_coeff[LMCS_MAX_BIN_SIZE];
} VVCLMCS;

#define VVC_MAX_ALF_COUNT        8
#define VVC_MAX_LMCS_COUNT       4
#define VVC_MAX_SL_COUNT         8

typedef struct VVCParamSets {
    const VVCSPS            *sps_list[VVC_MAX_SPS_COUNT];       ///< RefStruct reference
    const VVCPPS            *pps_list[VVC_MAX_PPS_COUNT];       ///< RefStruct reference
    const VVCALF            *alf_list[VVC_MAX_ALF_COUNT];       ///< RefStruct reference
    const H266RawAPS        *lmcs_list[VVC_MAX_LMCS_COUNT];     ///< RefStruct reference
    const VVCScalingList    *scaling_list[VVC_MAX_SL_COUNT];    ///< RefStruct reference

    // Bit field of SPS IDs used in the current CVS
    uint16_t                 sps_id_used;
} VVCParamSets;

typedef struct VVCFrameParamSets {
    const VVCSPS           *sps;                                ///< RefStruct reference
    const VVCPPS           *pps;                                ///< RefStruct reference
    VVCPH                   ph;
    const VVCALF           *alf_list[VVC_MAX_ALF_COUNT];        ///< RefStruct reference
    VVCLMCS                 lmcs;
    const VVCScalingList   *sl;                                 ///< RefStruct reference
} VVCFrameParamSets;

typedef struct VVCSH {
    const H266RawSliceHeader *r;                            ///< RefStruct reference

    // derived values
    // ctu address
    uint32_t        num_ctus_in_curr_slice;                 ///< NumCtusInCurrSlice
    const uint32_t* ctb_addr_in_curr_slice;                 ///< CtbAddrInCurrSlice

    // inter
    PredWeightTable pwt;
    int8_t   ref_idx_sym[2];                                ///< RefIdxSymL0, RefIdxSymL1

    // qp_y
    int8_t   slice_qp_y;                                    ///< SliceQpY

    // deblock_offsets
    DBParams deblock;

    // partition constrains
    uint8_t  min_qt_size[2];                                ///< MinQtSizeY, MinQtSizeC
    uint8_t  max_bt_size[2];                                ///< MaxBtSizeY, MaxBtSizeC
    uint8_t  max_tt_size[2];                                ///< MaxTtSizeY, MaxTtSizeC
    uint8_t  max_mtt_depth[2];                              ///< MaxMttDepthY, MaxMttDepthC
    uint8_t  cu_qp_delta_subdiv;                            ///< CuQpDeltaSubdiv
    uint8_t  cu_chroma_qp_offset_subdiv;                    ///< CuChromaQpOffsetSubdiv

    // entries
    uint32_t entry_point_start_ctu[VVC_MAX_ENTRY_POINTS];   ///< entry point start in ctu_addr
} VVCSH;

struct VVCContext;

int ff_vvc_decode_frame_ps(VVCFrameParamSets *fps, struct VVCContext *s);
int ff_vvc_decode_aps(VVCParamSets *ps, const CodedBitstreamUnit *unit);
int ff_vvc_decode_sh(VVCSH *sh, const VVCFrameParamSets *ps, const CodedBitstreamUnit *unit);
void ff_vvc_frame_ps_free(VVCFrameParamSets *fps);
void ff_vvc_ps_uninit(VVCParamSets *ps);

#endif /* AVCODEC_VVC_PS_H */
