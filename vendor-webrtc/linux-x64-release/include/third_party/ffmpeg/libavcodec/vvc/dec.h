/*
 * VVC video decoder
 *
 * Copyright (C) 2021 Nuo Mi
 * Copyright (C) 2022 Xu Mu
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

#ifndef AVCODEC_VVC_DEC_H
#define AVCODEC_VVC_DEC_H

#include "libavcodec/videodsp.h"
#include "libavcodec/vvc.h"

#include "ps.h"
#include "dsp.h"

#define LUMA                    0
#define CHROMA                  1
#define CB                      1
#define CR                      2
#define JCBCR                   3

#define MIN_TU_LOG2             2                       ///< MinTbLog2SizeY
#define MIN_PU_LOG2             2

#define L0                      0
#define L1                      1

typedef struct VVCRefPic {
    struct VVCFrame *ref;
    int poc;
    int is_lt;                  // is long term reference

    // for RPR
    int is_scaled;              ///< RprConstraintsActiveFlag
    int scale[2];               ///< RefPicScale[]
} VVCRefPic;

typedef struct RefPicList {
    VVCRefPic refs[VVC_MAX_REF_ENTRIES];
    int nb_refs;
} RefPicList;

typedef struct RefPicListTab {
    RefPicList refPicList[2];
} RefPicListTab;

typedef struct VVCWindow {
    int16_t left_offset;
    int16_t right_offset;
    int16_t top_offset;
    int16_t bottom_offset;
} VVCWindow;

typedef struct VVCFrame {
    struct AVFrame *frame;

    const VVCSPS *sps;                          ///< RefStruct reference
    const VVCPPS *pps;                          ///< RefStruct reference
    struct MvField *tab_dmvr_mvf;               ///< RefStruct reference
    RefPicListTab **rpl_tab;                    ///< RefStruct reference
    RefPicListTab  *rpl;                        ///< RefStruct reference
    int nb_rpl_elems;

    int ctb_count;

    int poc;

    //for RPR
    VVCWindow scaling_win;                      ///< pps_scaling_win_left_offset * SubWithC,  pps_scaling_win_right_offset  * SubWithC,
                                                ///< pps_scaling_win_top_offset  * SubHeigtC, pps_scaling_win_bottom_offset * SubHiehgtC
    int ref_width;                              ///< CurrPicScalWinWidthL
    int ref_height;                             ///< CurrPicScalWinHeightL

    struct VVCFrame *collocated_ref;

    struct FrameProgress *progress;             ///< RefStruct reference

    /**
     * A sequence counter, so that old frames are output first
     * after a POC reset
     */
    uint16_t sequence;
    /**
     * A combination of VVC_FRAME_FLAG_*
     */
    uint8_t flags;

    void *hwaccel_picture_private; ///< hardware accelerator private data
} VVCFrame;

typedef struct SliceContext {
    int slice_idx;
    VVCSH sh;
    struct EntryPoint *eps;
    int nb_eps;
    RefPicList *rpl;
    void *ref;                      ///< RefStruct reference, backing slice data
} SliceContext;

typedef struct VVCFrameContext {
    void *log_ctx;

    // +1 for the current frame
    VVCFrame DPB[VVC_MAX_DPB_SIZE + 1];

    struct AVFrame *frame;
    struct AVFrame *output_frame;

    VVCFrameParamSets ps;

    SliceContext  **slices;
    int nb_slices;
    int nb_slices_allocated;

    VVCFrame *ref;

    VVCDSPContext vvcdsp;
    VideoDSPContext vdsp;

    struct VVCFrameThread *ft;

    uint64_t decode_order;

    struct AVRefStructPool *tab_dmvr_mvf_pool;
    struct AVRefStructPool *rpl_tab_pool;

    struct AVRefStructPool *cu_pool;
    struct AVRefStructPool *tu_pool;

    struct {
        int16_t *slice_idx;

        DBParams  *deblock;
        struct SAOParams *sao;
        struct ALFParams *alf;

        int     *cb_pos_x[2];                           ///< CbPosX[][][]
        int     *cb_pos_y[2];                           ///< CbPosY[][][]
        uint8_t *cb_width[2];                           ///< CbWidth[][][]
        uint8_t *cb_height[2];                          ///< CbHeight[][][]
        uint8_t *cqt_depth[2];                          ///< CqtDepth[][][]
        int8_t  *qp[VVC_MAX_SAMPLE_ARRAYS];

        uint8_t *skip;                                  ///< CuSkipFlag[][]
        uint8_t *ispmf;                                 ///< intra_sub_partitions_mode_flag
        uint8_t *msm[2];                                ///< MttSplitMode[][][] in 32 pixels
        uint8_t *imf;                                   ///< IntraMipFlag[][], intra_mip_transposed_flag[][], intra_mip_mode[][]
        uint8_t *ipm;                                   ///< IntraPredModeY[][]
        uint8_t *cpm[2];                                ///< CuPredMode[][][]
        uint8_t *msf;                                   ///< MergeSubblockFlag[][]
        uint8_t *iaf;                                   ///< InterAffineFlag[][]
        uint8_t *mmi;                                   ///< MotionModelIdc[][]
        struct Mv      *cp_mv[2];                       ///< CpMvLX[][][][MAX_CONTROL_POINTS];
        struct MvField *mvf;                            ///< MvDmvrL0, MvDmvrL1

        uint8_t *tu_coded_flag[VVC_MAX_SAMPLE_ARRAYS];  ///< tu_y_coded_flag[][],  tu_cb_coded_flag[][],  tu_cr_coded_flag[][]
        uint8_t *tu_joint_cbcr_residual_flag;           ///< tu_joint_cbcr_residual_flag[][]
        uint8_t *tb_width[2];
        uint8_t *tb_height[2];
        uint8_t *pcmf[2];

        uint8_t *bs[2][VVC_MAX_SAMPLE_ARRAYS];          ///< horizontal, vertical boundary filtering strength
        uint8_t *max_len_p[2];                          ///< horizontal, vertical maxFilterLengthPs for luma
        uint8_t *max_len_q[2];                          ///< horizontal, vertical maxFilterLengthQs for luma

        uint8_t *sao_pixel_buffer_h[VVC_MAX_SAMPLE_ARRAYS];
        uint8_t *sao_pixel_buffer_v[VVC_MAX_SAMPLE_ARRAYS];
        uint8_t *alf_pixel_buffer_h[VVC_MAX_SAMPLE_ARRAYS][2];
        uint8_t *alf_pixel_buffer_v[VVC_MAX_SAMPLE_ARRAYS][2];

        int                 *coeffs;
        struct CTU          *ctus;
        struct CodingUnit  **cus;

        uint8_t *ibc_vir_buf[VVC_MAX_SAMPLE_ARRAYS];    ///< IbcVirBuf[]

        //used in arrays_init only
        struct {
            int ctu_count;
            int ctu_size;
            int pic_size_in_min_cb;
            int pic_size_in_min_pu;
            int pic_size_in_min_tu;
            int ctu_width;
            int ctu_height;
            int width;
            int height;
            int chroma_format_idc;
            int pixel_shift;
            int ibc_buffer_width;       ///< IbcBufWidth
        } sz;
    } tab;
} VVCFrameContext;

typedef struct VVCContext {
    struct AVCodecContext *avctx;

    CodedBitstreamContext *cbc;
    CodedBitstreamFragment current_frame;

    VVCParamSets ps;

    int temporal_id;        ///< temporal_id_plus1 - 1
    int poc_tid0;

    int eos;                ///< current packet contains an EOS/EOB NAL
    int last_eos;           ///< last packet contains an EOS/EOB NAL

    enum VVCNALUnitType vcl_unit_type;
    int no_output_before_recovery_flag; ///< NoOutputBeforeRecoveryFlag
    int gdr_recovery_point_poc;         ///< recoveryPointPocVal

    /**
     * Sequence counters for decoded and output frames, so that old
     * frames are output first after a POC reset
     */
    uint16_t seq_decode;
    uint16_t seq_output;

    struct FFExecutor *executor;

    VVCFrameContext *fcs;
    int nb_fcs;

    uint64_t nb_frames;     ///< processed frames
    int nb_delayed;         ///< delayed frames
}  VVCContext ;

#endif /* AVCODEC_VVC_DEC_H */
