/*
 * VVC CTU(Coding Tree Unit) parser
 *
 * Copyright (C) 2022 Nuo Mi
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

#ifndef AVCODEC_VVC_CTU_H
#define AVCODEC_VVC_CTU_H

#include <stdbool.h>

#include "libavcodec/cabac.h"
#include "libavutil/mem_internal.h"

#include "dec.h"

#define MAX_CTU_SIZE            128

#define MAX_CU_SIZE             MAX_CTU_SIZE
#define MIN_CU_SIZE             4
#define MIN_CU_LOG2             2
#define MAX_CU_DEPTH            7

#define MAX_PARTS_IN_CTU        ((MAX_CTU_SIZE >> MIN_CU_LOG2) * (MAX_CTU_SIZE >> MIN_CU_LOG2))

#define MIN_PU_SIZE             4

#define MAX_TB_SIZE             64
#define MIN_TU_SIZE             4
#define MAX_TUS_IN_CU           64

#define MAX_QP                  63

#define MAX_PB_SIZE             128
#define MAX_SCALING_RATIO       8
#define EDGE_EMU_BUFFER_STRIDE  ((MAX_PB_SIZE + 32) * MAX_SCALING_RATIO)

#define CHROMA_EXTRA_BEFORE     1
#define CHROMA_EXTRA_AFTER      2
#define CHROMA_EXTRA            3
#define LUMA_EXTRA_BEFORE       3
#define LUMA_EXTRA_AFTER        4
#define LUMA_EXTRA              7
#define BILINEAR_EXTRA_BEFORE   0
#define BILINEAR_EXTRA_AFTER    1
#define BILINEAR_EXTRA          1

#define SCALED_INT(pos) ((pos) >> 10)

#define MAX_CONTROL_POINTS      3

#define AFFINE_MIN_BLOCK_SIZE   4

#define MRG_MAX_NUM_CANDS       6
#define MAX_NUM_HMVP_CANDS      5

#define SAO_PADDING_SIZE        1

#define ALF_PADDING_SIZE        8
#define ALF_BLOCK_SIZE          4

#define ALF_BORDER_LUMA         3
#define ALF_BORDER_CHROMA       2

#define ALF_VB_POS_ABOVE_LUMA   4
#define ALF_VB_POS_ABOVE_CHROMA 2

#define ALF_GRADIENT_STEP       2
#define ALF_GRADIENT_BORDER     2
#define ALF_GRADIENT_SIZE       ((MAX_CU_SIZE + ALF_GRADIENT_BORDER * 2) / ALF_GRADIENT_STEP)
#define ALF_NUM_DIR             4


/**
 * Value of the luma sample at position (x, y) in the 2D array tab.
 */
#define SAMPLE(tab, x, y) ((tab)[(y) * s->pps->width + (x)])
#define SAMPLE_CTB(tab, x, y) ((tab)[(y) * min_cb_width + (x)])
#define CTB(tab, x, y) ((tab)[(y) * fc->ps.pps->ctb_width + (x)])

enum SAOType {
    SAO_NOT_APPLIED = 0,
    SAO_BAND,
    SAO_EDGE,
};

enum SAOEOClass {
    SAO_EO_HORIZ = 0,
    SAO_EO_VERT,
    SAO_EO_135D,
    SAO_EO_45D,
};

typedef struct NeighbourAvailable {
    int cand_left;
    int cand_up;
    int cand_up_left;
    int cand_up_right;
    int cand_up_right_sap;
} NeighbourAvailable;

enum IspType{
    ISP_NO_SPLIT,
    ISP_HOR_SPLIT,
    ISP_VER_SPLIT,
};

typedef enum VVCSplitMode {
    SPLIT_NONE,
    SPLIT_TT_HOR,
    SPLIT_BT_HOR,
    SPLIT_TT_VER,
    SPLIT_BT_VER,
    SPLIT_QT,
} VVCSplitMode;

typedef enum MtsIdx {
    MTS_DCT2_DCT2,
    MTS_DST7_DST7,
    MTS_DST7_DCT8,
    MTS_DCT8_DST7,
    MTS_DCT8_DCT8,
} MtsIdx;

typedef struct TransformBlock {
    uint8_t has_coeffs;
    uint8_t c_idx;
    uint8_t ts;             ///<  transform_skip_flag
    int x0;
    int y0;

    int tb_width;
    int tb_height;
    int log2_tb_width;
    int log2_tb_height;

    int max_scan_x;
    int max_scan_y;
    int min_scan_x;
    int min_scan_y;

    int qp;
    int rect_non_ts_flag;
    int bd_shift;
    int bd_offset;

    int *coeffs;
} TransformBlock;

typedef enum VVCTreeType {
    SINGLE_TREE,
    DUAL_TREE_LUMA,
    DUAL_TREE_CHROMA,
} VVCTreeType;

typedef struct TransformUnit {
    int x0;
    int y0;
    int width;
    int height;
    bool avail[CHROMA + 1];                             // contains luma/chroma block

    uint8_t joint_cbcr_residual_flag;                   ///< tu_joint_cbcr_residual_flag

    uint8_t coded_flag[VVC_MAX_SAMPLE_ARRAYS];          ///< tu_y_coded_flag, tu_cb_coded_flag, tu_cr_coded_flag
    uint8_t nb_tbs;
    TransformBlock tbs[VVC_MAX_SAMPLE_ARRAYS];

    struct TransformUnit *next;                         ///< RefStruct reference
} TransformUnit;

typedef enum PredMode {
    MODE_INTER,
    MODE_INTRA,
    MODE_SKIP,
    MODE_PLT,
    MODE_IBC,
} PredMode;

typedef struct Mv {
    int x;  ///< horizontal component of motion vector
    int y;  ///< vertical component of motion vector
} Mv;

typedef struct MvField {
    DECLARE_ALIGNED(8, Mv, mv)[2];  ///< mvL0, vvL1
    int8_t  ref_idx[2];             ///< refIdxL0, refIdxL1
    uint8_t hpel_if_idx;            ///< hpelIfIdx
    uint8_t bcw_idx;                ///< bcwIdx
    uint8_t pred_flag;
    uint8_t ciip_flag;              ///< ciip_flag
} MvField;

typedef struct DMVRInfo {
    DECLARE_ALIGNED(8, Mv, mv)[2];  ///< mvL0, vvL1
    uint8_t dmvr_enabled;
} DMVRInfo;

typedef enum MotionModelIdc {
    MOTION_TRANSLATION,
    MOTION_4_PARAMS_AFFINE,
    MOTION_6_PARAMS_AFFINE,
} MotionModelIdc;

typedef enum PredFlag {
    PF_INTRA = 0x0,
    PF_L0    = 0x1,
    PF_L1    = 0x2,
    PF_BI    = 0x3,
    PF_IBC   = PF_L0 | 0x4,
} PredFlag;

typedef enum IntraPredMode {
    INTRA_INVALID   = -1,
    INTRA_PLANAR    = 0,
    INTRA_DC,
    INTRA_HORZ      = 18,
    INTRA_DIAG      = 34,
    INTRA_VERT      = 50,
    INTRA_VDIAG     = 66,
    INTRA_LT_CCLM   = 81,
    INTRA_L_CCLM,
    INTRA_T_CCLM
} IntraPredMode;

typedef struct MotionInfo {
    MotionModelIdc motion_model_idc; ///< MotionModelIdc
    int8_t   ref_idx[2];             ///< refIdxL0, refIdxL1
    uint8_t  hpel_if_idx;            ///< hpelIfIdx
    uint8_t  bcw_idx;                ///< bcwIdx
    PredFlag pred_flag;

    Mv mv[2][MAX_CONTROL_POINTS];

    int num_sb_x, num_sb_y;
} MotionInfo;

typedef struct PredictionUnit {
    uint8_t general_merge_flag;
    uint8_t mmvd_merge_flag;
    //InterPredIdc inter_pred_idc;
    uint8_t inter_affine_flag;

    //subblock predict
    uint8_t merge_subblock_flag;

    uint8_t merge_gpm_flag;
    uint8_t gpm_partition_idx;
    MvField gpm_mv[2];

    int sym_mvd_flag;

    MotionInfo mi;

    // for regular prediction only
    uint8_t dmvr_flag;
    uint8_t bdof_flag;

    int16_t diff_mv_x[2][AFFINE_MIN_BLOCK_SIZE * AFFINE_MIN_BLOCK_SIZE];   ///< diffMvLX
    int16_t diff_mv_y[2][AFFINE_MIN_BLOCK_SIZE * AFFINE_MIN_BLOCK_SIZE];   ///< diffMvLX
    int cb_prof_flag[2];
} PredictionUnit;

typedef struct CodingUnit {
    VVCTreeType tree_type;
    int x0;
    int y0;
    int cb_width;
    int cb_height;
    int ch_type;
    int cqt_depth;

    uint8_t coded_flag;

    uint8_t sbt_flag;
    uint8_t sbt_horizontal_flag;
    uint8_t sbt_pos_flag;

    int lfnst_idx;
    MtsIdx mts_idx;

    uint8_t act_enabled_flag;

    uint8_t intra_luma_ref_idx;                     ///< IntraLumaRefLineIdx[][]
    uint8_t intra_mip_flag;                         ///< intra_mip_flag
    uint8_t skip_flag;                              ///< cu_skip_flag;

    //inter
    uint8_t ciip_flag;

    // Inferred parameters
    enum IspType isp_split_type;                    ///< IntraSubPartitionsSplitType

    enum PredMode pred_mode;                        ///< PredMode

    int num_intra_subpartitions;

    IntraPredMode intra_pred_mode_y;                ///< IntraPredModeY
    IntraPredMode intra_pred_mode_c;                ///< IntraPredModeC
    int mip_chroma_direct_flag;                     ///< MipChromaDirectFlag

    int bdpcm_flag[VVC_MAX_SAMPLE_ARRAYS];          ///< BdpcmFlag

    int apply_lfnst_flag[VVC_MAX_SAMPLE_ARRAYS];    ///< ApplyLfnstFlag[]

    struct {
        TransformUnit *head;                        ///< RefStruct reference
        TransformUnit *tail;                        ///< RefStruct reference
    } tus;

    int8_t qp[4];                                   ///< QpY, Qp′Cb, Qp′Cr, Qp′CbCr

    PredictionUnit pu;

    struct CodingUnit *next;                        ///< RefStruct reference
} CodingUnit;

typedef struct CTU {
    int max_y[2][VVC_MAX_REF_ENTRIES];
    int max_y_idx[2];
    int has_dmvr;
} CTU;

typedef struct ReconstructedArea {
    int x;
    int y;
    int w;
    int h;
} ReconstructedArea;

typedef struct VVCCabacState {
    uint16_t state[2];
    uint8_t  shift[2];
} VVCCabacState;

// VVC_CONTEXTS matched with SYNTAX_ELEMENT_LAST, it's checked by cabac_init_state.
#define VVC_CONTEXTS 378
typedef struct EntryPoint {
    int8_t qp_y;                                    ///< QpY

    int stat_coeff[VVC_MAX_SAMPLE_ARRAYS];          ///< StatCoeff

    VVCCabacState cabac_state[VVC_CONTEXTS];
    CABACContext cc;

    int ctu_start;
    int ctu_end;

    uint8_t is_first_qg;                            // first quantization group

    MvField hmvp[MAX_NUM_HMVP_CANDS];               ///< HmvpCandList
    int     num_hmvp;                               ///< NumHmvpCand
    MvField hmvp_ibc[MAX_NUM_HMVP_CANDS];           ///< HmvpIbcCandList
    int     num_hmvp_ibc;                           ///< NumHmvpIbcCand
} EntryPoint;

typedef struct VVCLocalContext {
    uint8_t ctb_left_flag;
    uint8_t ctb_up_flag;
    uint8_t ctb_up_right_flag;
    uint8_t ctb_up_left_flag;
    int     end_of_tiles_x;
    int     end_of_tiles_y;

    /* *2 for high bit depths */
    DECLARE_ALIGNED(32, uint8_t, edge_emu_buffer)[EDGE_EMU_BUFFER_STRIDE * EDGE_EMU_BUFFER_STRIDE * 2];
    DECLARE_ALIGNED(32, int16_t, tmp)[MAX_PB_SIZE * MAX_PB_SIZE];
    DECLARE_ALIGNED(32, int16_t, tmp1)[MAX_PB_SIZE * MAX_PB_SIZE];
    DECLARE_ALIGNED(32, int16_t, tmp2)[MAX_PB_SIZE * MAX_PB_SIZE];
    DECLARE_ALIGNED(32, uint8_t, ciip_tmp)[MAX_PB_SIZE * MAX_PB_SIZE * 2];
    DECLARE_ALIGNED(32, uint8_t, sao_buffer)[(MAX_CTU_SIZE + 2 * SAO_PADDING_SIZE) * EDGE_EMU_BUFFER_STRIDE * 2];
    DECLARE_ALIGNED(32, uint8_t, alf_buffer_luma)[(MAX_CTU_SIZE + 2 * ALF_PADDING_SIZE) * EDGE_EMU_BUFFER_STRIDE * 2];
    DECLARE_ALIGNED(32, uint8_t, alf_buffer_chroma)[(MAX_CTU_SIZE + 2 * ALF_PADDING_SIZE) * EDGE_EMU_BUFFER_STRIDE * 2];
    DECLARE_ALIGNED(32, int32_t, alf_gradient_tmp)[ALF_GRADIENT_SIZE * ALF_GRADIENT_SIZE * ALF_NUM_DIR];

    struct {
        int sbt_num_fourths_tb0;                ///< SbtNumFourthsTb0

        uint8_t is_cu_qp_delta_coded;           ///< IsCuQpDeltaCoded
        int cu_qg_top_left_x;                   ///< CuQgTopLeftX
        int cu_qg_top_left_y;                   ///< CuQgTopLeftY
        int is_cu_chroma_qp_offset_coded;       ///< IsCuChromaQpOffsetCoded
        int chroma_qp_offset[3];                ///< CuQpOffsetCb, CuQpOffsetCr, CuQpOffsetCbCr

        int infer_tu_cbf_luma;                  ///< InferTuCbfLuma
        int prev_tu_cbf_y;                      ///< prevTuCbfY;

        int lfnst_dc_only;                      ///< LfnstDcOnly
        int lfnst_zero_out_sig_coeff_flag;      ///< LfnstZeroOutSigCoeffFlag

        int mts_dc_only;                        ///< MtsDcOnly
        int mts_zero_out_sig_coeff_flag;        ///< MtsZeroOutSigCoeffFlag;
    } parse;

    struct {
        // lmcs cache, for recon only
        int chroma_scale;
        int x_vpdu;
        int y_vpdu;
    } lmcs;

    CodingUnit *cu;
    ReconstructedArea ras[2][MAX_PARTS_IN_CTU];
    int num_ras[2];

    NeighbourAvailable na;

#define BOUNDARY_LEFT_SLICE     (1 << 0)
#define BOUNDARY_LEFT_TILE      (1 << 1)
#define BOUNDARY_LEFT_SUBPIC    (1 << 2)
#define BOUNDARY_UPPER_SLICE    (1 << 3)
#define BOUNDARY_UPPER_TILE     (1 << 4)
#define BOUNDARY_UPPER_SUBPIC   (1 << 5)
    /* properties of the boundary of the current CTB for the purposes
     * of the deblocking filter */
    int boundary_flags;

    SliceContext *sc;
    VVCFrameContext *fc;
    EntryPoint *ep;
    int *coeffs;
} VVCLocalContext;

typedef struct VVCAllowedSplit {
    int qt;
    int btv;
    int bth;
    int ttv;
    int tth;
} VVCAllowedSplit;

typedef struct SAOParams {
    int offset_abs[3][4];               ///< sao_offset_abs
    int offset_sign[3][4];              ///< sao_offset_sign

    uint8_t band_position[3];           ///< sao_band_position

    int eo_class[3];                    ///< sao_eo_class

    int16_t offset_val[3][5];           ///< SaoOffsetVal

    uint8_t type_idx[3];                ///< sao_type_idx
} SAOParams;

typedef struct ALFParams {
    uint8_t ctb_flag[3];                ///< alf_ctb_flag[]
    uint8_t ctb_filt_set_idx_y;         ///< AlfCtbFiltSetIdxY
    uint8_t alf_ctb_filter_alt_idx[2];  ///< alf_ctb_filter_alt_idx[]
    uint8_t ctb_cc_idc[2];              ///< alf_ctb_cc_cb_idc, alf_ctb_cc_cr_idc
} ALFParams;

typedef struct VVCRect {
    int l;                  // left
    int t;                  // top
    int r;                  // right
    int b;                  // bottom
} VVCRect;

/**
 * parse a CTU
 * @param lc local context for CTU
 * @param ctb_idx CTB(CTU) address in the current slice
 * @param rs raster order for the CTU.
 * @param rx raster order x for the CTU.
 * @param ry raster order y for the CTU.
 * @return AVERROR
 */
int ff_vvc_coding_tree_unit(VVCLocalContext *lc, int ctu_idx, int rs, int rx, int ry);

//utils
void ff_vvc_set_neighbour_available(VVCLocalContext *lc, int x0, int y0, int w, int h);
void ff_vvc_decode_neighbour(VVCLocalContext *lc, int x_ctb, int y_ctb, int rx, int ry, int rs);
void ff_vvc_ctu_free_cus(CodingUnit **cus);
int ff_vvc_get_qPy(const VVCFrameContext *fc, int xc, int yc);
void ff_vvc_ep_init_stat_coeff(EntryPoint *ep, int bit_depth, int persistent_rice_adaptation_enabled_flag);

#endif // AVCODEC_VVC_CTU_H
