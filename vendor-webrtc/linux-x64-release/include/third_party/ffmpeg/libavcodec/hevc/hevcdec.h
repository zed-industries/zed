/*
 * HEVC video decoder
 *
 * Copyright (C) 2012 - 2013 Guillaume Martres
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

#ifndef AVCODEC_HEVC_HEVCDEC_H
#define AVCODEC_HEVC_HEVCDEC_H

#include <stdatomic.h>

#include "libavutil/buffer.h"
#include "libavutil/mem_internal.h"

#include "libavcodec/avcodec.h"
#include "libavcodec/bswapdsp.h"
#include "libavcodec/cabac.h"
#include "libavcodec/dovi_rpu.h"
#include "libavcodec/get_bits.h"
#include "libavcodec/h2645_parse.h"
#include "libavcodec/h274.h"
#include "libavcodec/progressframe.h"
#include "libavcodec/videodsp.h"

#include "dsp.h"
#include "hevc.h"
#include "pred.h"
#include "ps.h"
#include "sei.h"

#define SHIFT_CTB_WPP 2

#define MAX_TB_SIZE 32
#define MAX_QP 51
#define DEFAULT_INTRA_TC_OFFSET 2

#define HEVC_CONTEXTS 199
#define HEVC_STAT_COEFFS 4

#define MRG_MAX_NUM_CANDS     5

#define L0 0
#define L1 1

#define EPEL_EXTRA_BEFORE 1
#define EPEL_EXTRA_AFTER  2
#define EPEL_EXTRA        3
#define QPEL_EXTRA_BEFORE 3
#define QPEL_EXTRA_AFTER  4
#define QPEL_EXTRA        7

#define EDGE_EMU_BUFFER_STRIDE 80

/**
 * Value of the luma sample at position (x, y) in the 2D array tab.
 */
#define SAMPLE(tab, x, y) ((tab)[(y) * s->sps->width + (x)])
#define SAMPLE_CTB(tab, x, y) ((tab)[(y) * min_cb_width + (x)])

#define IS_IDR(s) ((s)->nal_unit_type == HEVC_NAL_IDR_W_RADL || (s)->nal_unit_type == HEVC_NAL_IDR_N_LP)
#define IS_BLA(s) ((s)->nal_unit_type == HEVC_NAL_BLA_W_RADL || (s)->nal_unit_type == HEVC_NAL_BLA_W_LP || \
                   (s)->nal_unit_type == HEVC_NAL_BLA_N_LP)
#define IS_IRAP(s) ((s)->nal_unit_type >= HEVC_NAL_BLA_W_LP && (s)->nal_unit_type <= HEVC_NAL_RSV_IRAP_VCL23)

#define HEVC_RECOVERY_UNSPECIFIED INT_MAX
#define HEVC_RECOVERY_END INT_MIN
#define HEVC_IS_RECOVERING(s) ((s)->recovery_poc != HEVC_RECOVERY_UNSPECIFIED && (s)->recovery_poc != HEVC_RECOVERY_END)

enum RPSType {
    ST_CURR_BEF = 0,
    ST_CURR_AFT,
    ST_FOLL,
    LT_CURR,
    LT_FOLL,
    INTER_LAYER0,
    INTER_LAYER1,
    NB_RPS_TYPE,
};

enum PartMode {
    PART_2Nx2N = 0,
    PART_2NxN  = 1,
    PART_Nx2N  = 2,
    PART_NxN   = 3,
    PART_2NxnU = 4,
    PART_2NxnD = 5,
    PART_nLx2N = 6,
    PART_nRx2N = 7,
};

enum PredMode {
    MODE_INTER = 0,
    MODE_INTRA,
    MODE_SKIP,
};

enum InterPredIdc {
    PRED_L0 = 0,
    PRED_L1,
    PRED_BI,
};

enum PredFlag {
    PF_INTRA = 0,
    PF_L0,
    PF_L1,
    PF_BI,
};

enum IntraPredMode {
    INTRA_PLANAR = 0,
    INTRA_DC,
    INTRA_ANGULAR_2,
    INTRA_ANGULAR_3,
    INTRA_ANGULAR_4,
    INTRA_ANGULAR_5,
    INTRA_ANGULAR_6,
    INTRA_ANGULAR_7,
    INTRA_ANGULAR_8,
    INTRA_ANGULAR_9,
    INTRA_ANGULAR_10,
    INTRA_ANGULAR_11,
    INTRA_ANGULAR_12,
    INTRA_ANGULAR_13,
    INTRA_ANGULAR_14,
    INTRA_ANGULAR_15,
    INTRA_ANGULAR_16,
    INTRA_ANGULAR_17,
    INTRA_ANGULAR_18,
    INTRA_ANGULAR_19,
    INTRA_ANGULAR_20,
    INTRA_ANGULAR_21,
    INTRA_ANGULAR_22,
    INTRA_ANGULAR_23,
    INTRA_ANGULAR_24,
    INTRA_ANGULAR_25,
    INTRA_ANGULAR_26,
    INTRA_ANGULAR_27,
    INTRA_ANGULAR_28,
    INTRA_ANGULAR_29,
    INTRA_ANGULAR_30,
    INTRA_ANGULAR_31,
    INTRA_ANGULAR_32,
    INTRA_ANGULAR_33,
    INTRA_ANGULAR_34,
};

enum SAOType {
    SAO_NOT_APPLIED = 0,
    SAO_BAND,
    SAO_EDGE,
    SAO_APPLIED
};

enum SAOEOClass {
    SAO_EO_HORIZ = 0,
    SAO_EO_VERT,
    SAO_EO_135D,
    SAO_EO_45D,
};

enum ScanType {
    SCAN_DIAG = 0,
    SCAN_HORIZ,
    SCAN_VERT,
};

typedef struct HEVCCABACState {
    uint8_t state[HEVC_CONTEXTS];
    uint8_t stat_coeff[HEVC_STAT_COEFFS];
} HEVCCABACState;

typedef struct LongTermRPS {
    int     poc[32];
    uint8_t poc_msb_present[32];
    uint8_t used[32];
    uint8_t nb_refs;
} LongTermRPS;

typedef struct RefPicList {
    struct HEVCFrame *ref[HEVC_MAX_REFS];
    int list[HEVC_MAX_REFS];
    int isLongTerm[HEVC_MAX_REFS];
    int nb_refs;
} RefPicList;

typedef struct RefPicListTab {
    RefPicList refPicList[2];
} RefPicListTab;

typedef struct SliceHeader {
    unsigned int pps_id;

    /// address (in raster order) of the first block in the current slice segment
    unsigned int   slice_segment_addr;
    /// address (in raster order) of the first block in the current slice
    unsigned int   slice_addr;

    enum HEVCSliceType slice_type;

    int pic_order_cnt_lsb;
    int poc;

    uint8_t first_slice_in_pic_flag;
    uint8_t dependent_slice_segment_flag;
    uint8_t pic_output_flag;
    uint8_t colour_plane_id;
    uint8_t inter_layer_pred;

    /// RPS coded in the slice header itself is stored here
    int short_term_ref_pic_set_sps_flag;
    int short_term_ref_pic_set_size;
    ShortTermRPS slice_rps;
    const ShortTermRPS *short_term_rps;
    int long_term_ref_pic_set_size;
    LongTermRPS long_term_rps;
    unsigned int list_entry_lx[2][32];

    uint8_t rpl_modification_flag[2];
    uint8_t no_output_of_prior_pics_flag;
    uint8_t slice_temporal_mvp_enabled_flag;

    unsigned int nb_refs[2];

    uint8_t slice_sample_adaptive_offset_flag[3];
    uint8_t mvd_l1_zero_flag;

    uint8_t cabac_init_flag;
    uint8_t disable_deblocking_filter_flag; ///< slice_header_disable_deblocking_filter_flag
    uint8_t slice_loop_filter_across_slices_enabled_flag;
    uint8_t collocated_list;

    unsigned int collocated_ref_idx;

    int slice_qp_delta;
    int slice_cb_qp_offset;
    int slice_cr_qp_offset;

    int slice_act_y_qp_offset;
    int slice_act_cb_qp_offset;
    int slice_act_cr_qp_offset;

    uint8_t cu_chroma_qp_offset_enabled_flag;

    int beta_offset;    ///< beta_offset_div2 * 2
    int tc_offset;      ///< tc_offset_div2 * 2

    uint8_t max_num_merge_cand; ///< 5 - 5_minus_max_num_merge_cand
    uint8_t use_integer_mv_flag;

    unsigned *entry_point_offset;
    int * offset;
    int * size;
    int num_entry_point_offsets;

    int8_t slice_qp;

    uint8_t luma_log2_weight_denom;
    int16_t chroma_log2_weight_denom;

    int16_t luma_weight_l0[16];
    int16_t chroma_weight_l0[16][2];
    int16_t chroma_weight_l1[16][2];
    int16_t luma_weight_l1[16];

    int16_t luma_offset_l0[16];
    int16_t chroma_offset_l0[16][2];

    int16_t luma_offset_l1[16];
    int16_t chroma_offset_l1[16][2];

    int slice_ctb_addr_rs;
    unsigned data_offset;
} SliceHeader;

typedef struct CodingUnit {
    int x;
    int y;

    enum PredMode pred_mode;    ///< PredMode
    enum PartMode part_mode;    ///< PartMode

    // Inferred parameters
    uint8_t intra_split_flag;   ///< IntraSplitFlag
    uint8_t max_trafo_depth;    ///< MaxTrafoDepth
    uint8_t cu_transquant_bypass_flag;
} CodingUnit;

typedef struct Mv {
    int16_t x;  ///< horizontal component of motion vector
    int16_t y;  ///< vertical component of motion vector
} Mv;

typedef struct MvField {
    DECLARE_ALIGNED(4, Mv, mv)[2];
    int8_t ref_idx[2];
    int8_t pred_flag;
} MvField;

typedef struct NeighbourAvailable {
    int cand_bottom_left;
    int cand_left;
    int cand_up;
    int cand_up_left;
    int cand_up_right;
    int cand_up_right_sap;
} NeighbourAvailable;

typedef struct PredictionUnit {
    int mpm_idx;
    int rem_intra_luma_pred_mode;
    uint8_t intra_pred_mode[4];
    Mv mvd;
    uint8_t merge_flag;
    uint8_t intra_pred_mode_c[4];
    uint8_t chroma_mode_c[4];
} PredictionUnit;

typedef struct TransformUnit {
    int cu_qp_delta;

    int res_scale_val;

    // Inferred parameters;
    int intra_pred_mode;
    int intra_pred_mode_c;
    int chroma_mode_c;
    uint8_t is_cu_qp_delta_coded;
    uint8_t is_cu_chroma_qp_offset_coded;
    int8_t  cu_qp_offset_cb;
    int8_t  cu_qp_offset_cr;
    uint8_t cross_pf;
} TransformUnit;

typedef struct DBParams {
    int beta_offset;
    int tc_offset;
} DBParams;

#define HEVC_FRAME_FLAG_OUTPUT    (1 << 0)
#define HEVC_FRAME_FLAG_SHORT_REF (1 << 1)
#define HEVC_FRAME_FLAG_LONG_REF  (1 << 2)
#define HEVC_FRAME_FLAG_UNAVAILABLE (1 << 3)
#define HEVC_FRAME_FLAG_CORRUPT (1 << 4)

typedef struct HEVCFrame {
    union {
        struct {
            AVFrame *f;
        };
        ProgressFrame tf;
    };
    AVFrame *frame_grain;
    int needs_fg; /* 1 if grain needs to be applied by the decoder */
    MvField *tab_mvf;              ///< RefStruct reference
    RefPicList *refPicList;
    RefPicListTab **rpl_tab;       ///< RefStruct reference
    int ctb_count;
    int poc;

    const HEVCPPS *pps;            ///< RefStruct reference
    RefPicListTab *rpl;            ///< RefStruct reference
    int nb_rpl_elems;

    void *hwaccel_picture_private; ///< RefStruct reference

    // for secondary-layer frames, this is the DPB index of the base-layer frame
    // from the same AU, if it exists, otherwise -1
    int base_layer_frame;

    /**
     * A combination of HEVC_FRAME_FLAG_*
     */
    uint8_t flags;
} HEVCFrame;

typedef struct HEVCLocalContext {
    uint8_t cabac_state[HEVC_CONTEXTS];

    uint8_t stat_coeff[HEVC_STAT_COEFFS];

    uint8_t first_qp_group;

    void *logctx;
    const struct HEVCContext *parent;

    CABACContext cc;

    /**
     * This is a pointer to the common CABAC state.
     * In case entropy_coding_sync_enabled_flag is set,
     * the CABAC state after decoding the second CTU in a row is
     * stored here and used to initialize the CABAC state before
     * decoding the first CTU in the next row.
     * This is the basis for WPP and in case slice-threading is used,
     * the next row is decoded by another thread making this state
     * shared between threads.
     */
    HEVCCABACState *common_cabac_state;

    int8_t qp_y;
    int8_t curr_qp_y;

    int qPy_pred;

    TransformUnit tu;

    uint8_t ctb_left_flag;
    uint8_t ctb_up_flag;
    uint8_t ctb_up_right_flag;
    uint8_t ctb_up_left_flag;
    int     end_of_tiles_x;
    int     end_of_tiles_y;
    /* +7 is for subpixel interpolation, *2 for high bit depths */
    DECLARE_ALIGNED(32, uint8_t, edge_emu_buffer)[(MAX_PB_SIZE + 7) * EDGE_EMU_BUFFER_STRIDE * 2];
    /* The extended size between the new edge emu buffer is abused by SAO */
    DECLARE_ALIGNED(32, uint8_t, edge_emu_buffer2)[(MAX_PB_SIZE + 7) * EDGE_EMU_BUFFER_STRIDE * 2];
    DECLARE_ALIGNED(32, int16_t, tmp)[MAX_PB_SIZE * MAX_PB_SIZE];

    int ct_depth;
    CodingUnit cu;
    PredictionUnit pu;
    NeighbourAvailable na;

#define BOUNDARY_LEFT_SLICE     (1 << 0)
#define BOUNDARY_LEFT_TILE      (1 << 1)
#define BOUNDARY_UPPER_SLICE    (1 << 2)
#define BOUNDARY_UPPER_TILE     (1 << 3)
    /* properties of the boundary of the current CTB for the purposes
     * of the deblocking filter */
    int boundary_flags;

    // an array of these structs is used for per-thread state - pad its size
    // to avoid false sharing
    char padding[128];
} HEVCLocalContext;

typedef struct HEVCLayerContext {
    HEVCFrame               DPB[32];
    HEVCFrame              *cur_frame;

    const HEVCSPS          *sps; // RefStruct reference

    int                     bs_width;
    int                     bs_height;

    SAOParams              *sao;
    DBParams               *deblock;

    //  CU
    uint8_t                *skip_flag;
    uint8_t                *tab_ct_depth;

    // PU
    uint8_t                *cbf_luma; // cbf_luma of colocated TU
    uint8_t                *tab_ipm;
    uint8_t                *is_pcm;

    // CTB-level flags affecting loop filter operation
    uint8_t                *filter_slice_edges;

    int32_t                *tab_slice_address;

    int8_t                 *qp_y_tab;

    uint8_t                *horizontal_bs;
    uint8_t                *vertical_bs;

    uint8_t                *sao_pixel_buffer_h[3];
    uint8_t                *sao_pixel_buffer_v[3];

    struct AVRefStructPool *tab_mvf_pool;
    struct AVRefStructPool *rpl_tab_pool;
} HEVCLayerContext;

typedef struct HEVCContext {
    const AVClass *c;  // needed by private avoptions
    AVCodecContext *avctx;

    HEVCLocalContext     *local_ctx;
    unsigned           nb_local_ctx;

    // per-layer decoding state, addressed by VPS layer indices
    HEVCLayerContext      layers[HEVC_VPS_MAX_LAYERS];
    // VPS index of the layer currently being decoded
    unsigned              cur_layer;
    // bitmask of layer indices that are active for decoding/output
    unsigned              layers_active_decode;
    unsigned              layers_active_output;

    /** 1 if the independent slice segment header was successfully parsed */
    uint8_t slice_initialized;

    struct AVContainerFifo *output_fifo;

    HEVCParamSets ps;
    HEVCSEI sei;
    struct AVMD5 *md5_ctx;

    /// candidate references for the current frame
    RefPicList rps[NB_RPS_TYPE];

    const HEVCVPS *vps; ///< RefStruct reference
    const HEVCPPS *pps; ///< RefStruct reference
    SliceHeader sh;
    enum HEVCNALUnitType nal_unit_type;
    int temporal_id;  ///< temporal_id_plus1 - 1
    HEVCFrame *cur_frame;
    HEVCFrame *collocated_ref;
    int poc;
    int poc_tid0;
    int slice_idx; ///< number of the slice being currently decoded
    int eos;       ///< current packet contains an EOS/EOB NAL
    int last_eos;  ///< last packet contains an EOS/EOB NAL
    int recovery_poc;

    // NoRaslOutputFlag associated with the last IRAP frame
    int no_rasl_output_flag;

    HEVCPredContext hpc;
    HEVCDSPContext hevcdsp;
    VideoDSPContext vdsp;
    BswapDSPContext bdsp;
    H274FilmGrainDatabase h274db;

    /** used on BE to byteswap the lines for checksumming */
    uint8_t *checksum_buf;
    int      checksum_buf_size;

    /** The target for the common_cabac_state of the local contexts. */
    HEVCCABACState cabac;

    struct ThreadProgress *wpp_progress;
    unsigned            nb_wpp_progress;

    atomic_int wpp_err;

    const uint8_t *data;

    H2645Packet pkt;
    // type of the first VCL NAL of the current frame
    enum HEVCNALUnitType first_nal_type;
    // index in pkt.nals of the NAL unit after which we can call
    // ff_thread_finish_setup()
    unsigned finish_setup_nal_idx;

    int is_nalff;           ///< this flag is != 0 if bitstream is encapsulated
                            ///< as a format defined in 14496-15
    int apply_defdispwin;

    // multi-layer AVOptions
    int         *view_ids;
    unsigned  nb_view_ids;

    unsigned    *view_ids_available;
    unsigned  nb_view_ids_available;

    unsigned    *view_pos_available;
    unsigned  nb_view_pos_available;

    int nal_length_size;    ///< Number of bytes used for nal length (1, 2 or 4)
    int nuh_layer_id;

    int film_grain_warning_shown;

    // dts of the packet currently being decoded
    int64_t pkt_dts;

    AVBufferRef *rpu_buf;       ///< 0 or 1 Dolby Vision RPUs.
    DOVIContext dovi_ctx;       ///< Dolby Vision decoding context
} HEVCContext;

/**
 * Mark all frames in DPB as unused for reference.
 */
void ff_hevc_clear_refs(HEVCLayerContext *l);

/**
 * Drop all frames currently in DPB.
 */
void ff_hevc_flush_dpb(HEVCContext *s);

const RefPicList *ff_hevc_get_ref_list(const HEVCFrame *frame, int x0, int y0);

/**
 * Construct the reference picture sets for the current frame.
 */
int ff_hevc_frame_rps(HEVCContext *s, HEVCLayerContext *l);

/**
 * Construct the reference picture list(s) for the current slice.
 */
int ff_hevc_slice_rpl(HEVCContext *s);

void ff_hevc_save_states(HEVCLocalContext *lc, const HEVCPPS *pps,
                         int ctb_addr_ts);
int ff_hevc_cabac_init(HEVCLocalContext *lc, const HEVCPPS *pps,
                       int ctb_addr_ts, const uint8_t *data, size_t size,
                       int is_wpp);
int ff_hevc_sao_merge_flag_decode(HEVCLocalContext *lc);
int ff_hevc_sao_type_idx_decode(HEVCLocalContext *lc);
int ff_hevc_sao_band_position_decode(HEVCLocalContext *lc);
int ff_hevc_sao_offset_abs_decode(HEVCLocalContext *lc, int bit_depth);
int ff_hevc_sao_offset_sign_decode(HEVCLocalContext *lc);
int ff_hevc_sao_eo_class_decode(HEVCLocalContext *lc);
int ff_hevc_end_of_slice_flag_decode(HEVCLocalContext *lc);
int ff_hevc_cu_transquant_bypass_flag_decode(HEVCLocalContext *lc);
int ff_hevc_skip_flag_decode(HEVCLocalContext *lc, uint8_t *skip_flag,
                             int x0, int y0, int x_cb, int y_cb, int min_cb_width);
int ff_hevc_pred_mode_decode(HEVCLocalContext *lc);
int ff_hevc_split_coding_unit_flag_decode(HEVCLocalContext *lc, uint8_t *tab_ct_depth,
                                          const HEVCSPS *sps,
                                          int ct_depth, int x0, int y0);
int ff_hevc_part_mode_decode(HEVCLocalContext *lc, const HEVCSPS *sps, int log2_cb_size);
int ff_hevc_pcm_flag_decode(HEVCLocalContext *lc);
int ff_hevc_prev_intra_luma_pred_flag_decode(HEVCLocalContext *lc);
int ff_hevc_mpm_idx_decode(HEVCLocalContext *lc);
int ff_hevc_rem_intra_luma_pred_mode_decode(HEVCLocalContext *lc);
int ff_hevc_intra_chroma_pred_mode_decode(HEVCLocalContext *lc);
int ff_hevc_merge_idx_decode(HEVCLocalContext *lc);
int ff_hevc_merge_flag_decode(HEVCLocalContext *lc);
int ff_hevc_inter_pred_idc_decode(HEVCLocalContext *lc, int nPbW, int nPbH);
int ff_hevc_ref_idx_lx_decode(HEVCLocalContext *lc, int num_ref_idx_lx);
int ff_hevc_mvp_lx_flag_decode(HEVCLocalContext *lc);
int ff_hevc_no_residual_syntax_flag_decode(HEVCLocalContext *lc);
int ff_hevc_split_transform_flag_decode(HEVCLocalContext *lc, int log2_trafo_size);
int ff_hevc_cbf_cb_cr_decode(HEVCLocalContext *lc, int trafo_depth);
int ff_hevc_cbf_luma_decode(HEVCLocalContext *lc, int trafo_depth);
int ff_hevc_log2_res_scale_abs(HEVCLocalContext *lc, int idx);
int ff_hevc_res_scale_sign_flag(HEVCLocalContext *lc, int idx);

/**
 * Get the number of candidate references for the current frame.
 */
int ff_hevc_frame_nb_refs(const SliceHeader *sh, const HEVCPPS *pps,
                          unsigned layer_idx);

int ff_hevc_set_new_ref(HEVCContext *s, HEVCLayerContext *l, int poc);

static av_always_inline int ff_hevc_nal_is_nonref(enum HEVCNALUnitType type)
{
    switch (type) {
    case HEVC_NAL_TRAIL_N:
    case HEVC_NAL_TSA_N:
    case HEVC_NAL_STSA_N:
    case HEVC_NAL_RADL_N:
    case HEVC_NAL_RASL_N:
    case HEVC_NAL_VCL_N10:
    case HEVC_NAL_VCL_N12:
    case HEVC_NAL_VCL_N14:
        return 1;
    default: break;
    }
    return 0;
}

/**
 * Find frames in the DPB that are ready for output and either write them to the
 * output FIFO or drop their output flag, depending on the value of discard.
 *
 * @param max_output maximum number of AUs with an output-pending frame in at
 *                   least one layer that can be present in the DPB before output
 *                   is triggered
 * @param max_dpb maximum number of any frames that can be present in the DPB
 *                for any layer before output is triggered
 */
int ff_hevc_output_frames(HEVCContext *s,
                          unsigned layers_active_decode, unsigned layers_active_output,
                          unsigned max_output, unsigned max_dpb, int discard);

void ff_hevc_unref_frame(HEVCFrame *frame, int flags);

void ff_hevc_set_neighbour_available(HEVCLocalContext *lc, int x0, int y0,
                                     int nPbW, int nPbH, int log2_ctb_size);
void ff_hevc_luma_mv_merge_mode(HEVCLocalContext *lc, const HEVCPPS *pps,
                                int x0, int y0,
                                int nPbW, int nPbH, int log2_cb_size,
                                int part_idx, int merge_idx, MvField *mv);
void ff_hevc_luma_mv_mvp_mode(HEVCLocalContext *lc, const HEVCPPS *pps,
                              int x0, int y0,
                              int nPbW, int nPbH, int log2_cb_size,
                              int part_idx, int merge_idx,
                              MvField *mv, int mvp_lx_flag, int LX);
void ff_hevc_hls_filter(HEVCLocalContext *lc, const HEVCLayerContext *l,
                        const HEVCPPS *pps,
                        int x, int y, int ctb_size);
void ff_hevc_hls_filters(HEVCLocalContext *lc, const HEVCLayerContext *l,
                         const HEVCPPS *pps,
                         int x_ctb, int y_ctb, int ctb_size);
void ff_hevc_set_qPy(HEVCLocalContext *lc,
                     const HEVCLayerContext *l, const HEVCPPS *pps,
                     int xBase, int yBase, int log2_cb_size);
void ff_hevc_deblocking_boundary_strengths(HEVCLocalContext *lc, const HEVCLayerContext *l,
                                           const HEVCPPS *pps,
                                           int x0, int y0, int log2_trafo_size);
int ff_hevc_cu_qp_delta_sign_flag(HEVCLocalContext *lc);
int ff_hevc_cu_qp_delta_abs(HEVCLocalContext *lc);
int ff_hevc_cu_chroma_qp_offset_flag(HEVCLocalContext *lc);
int ff_hevc_cu_chroma_qp_offset_idx(HEVCLocalContext *lc, int chroma_qp_offset_list_len_minus1);
void ff_hevc_hls_residual_coding(HEVCLocalContext *lc, const HEVCPPS *pps,
                                 int x0, int y0,
                                 int log2_trafo_size, enum ScanType scan_idx,
                                 int c_idx);

void ff_hevc_hls_mvd_coding(HEVCLocalContext *lc, int x0, int y0, int log2_cb_size);

int ff_hevc_is_alpha_video(const HEVCContext *s);

extern const uint8_t ff_hevc_qpel_extra_before[4];
extern const uint8_t ff_hevc_qpel_extra_after[4];
extern const uint8_t ff_hevc_qpel_extra[4];

#endif /* AVCODEC_HEVC_HEVCDEC_H */
