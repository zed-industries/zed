/*
 * Copyright (c) 2007-2017 Intel Corporation. All Rights Reserved.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the
 * "Software"), to deal in the Software without restriction, including
 * without limitation the rights to use, copy, modify, merge, publish,
 * distribute, sub license, and/or sell copies of the Software, and to
 * permit persons to whom the Software is furnished to do so, subject to
 * the following conditions:
 *
 * The above copyright notice and this permission notice (including the
 * next paragraph) shall be included in all copies or substantial portions
 * of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
 * OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT.
 * IN NO EVENT SHALL INTEL AND/OR ITS SUPPLIERS BE LIABLE FOR
 * ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
 * TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
 * SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

/**
 * \file va_fei_hevc.h
 * \brief The FEI encoding HEVC special API
 */


#ifndef  __VA_FEI_HEVC_H__
#define  __VA_FEI_HEVC_H__

#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>
#include "va_fei.h"

/**
 * \defgroup api_fei_hevc HEVC FEI encoding API
 *
 * @{
 */

/** \brief FEI frame level control buffer for HEVC */
typedef struct _VAEncMiscParameterFEIFrameControlHEVC {
    /* one of the VAConfigAttribFEIFunctionType values */
    uint32_t      function;
    /** \brief CTB control input buffer. It is valid only when per_ctb_input
     * is set to 1. The data in this buffer correspond to the input source. CTB is in raster scan order,
     * each CTB control data structure is defined by VAEncFEICTBControlHEVC.
     * Buffer size shall not be less than the number of CTBs multiplied by
     * sizeof(VAEncFEICTBControlHEVC).
     */
    VABufferID    ctb_ctrl;
    /** \brief CTB cmd per CTB data output of ENC
     *  it is reserved for CTB level information
     * it should include CU split information and other CTB datas .
     * Each CTB block has one CTB cmd data.
     */
    VABufferID    ctb_cmd;
    /** \brief CU record data output of ENC
     *  it is reserved for CU level information
     * it should include CU detail data. include mode,MV, reference etc.
     * Each CTB block has one CU record data.
     */
    VABufferID    cu_record;
    /** \brief distortion output of ENC or ENC_PAK.
     * Each CTB has one distortion data with VAEncFEIDistortionHevc
     * Buffer size shall not be less than the number of CTBs multiplied by
     * sizeof(VAEncFEIDistortionHevc).
     */
    VABufferID    distortion;
    /** \brief Qp input buffer. It is valid only when per_block_qp is set to 1.
     * The data in this buffer correspond to the input source.
     * One Qp per block block is in raster scan order, each Qp is a signed char (8-bit) value.
     **/
    VABufferID    qp;
    /** \brief MV predictor. It is valid only when mv_predictor_input is set to non-zero.
     * Each CTB block has one or more pair of motion vectors and the corresponding
     * reference indexes as defined by VAEncFEIMVPredictorHEVC. 32x32 block is in raster scan order.
     * Buffer size shall not be less than the number of 16x16 blocks multiplied by
     * sizeof(VAEncFEIMVPredictorHEVC). */
    VABufferID    mv_predictor;

    /** \brief number of MV predictors L0 and L1. the maximum number of motion vector predictor for a 16x16, 32x32 or
     * 64x64 block is four, it must not be greater than maximum supported MV predictor,
     **/
    uint32_t      num_mv_predictors_l0      : 16;
    uint32_t      num_mv_predictors_l1      : 16;

    /** \brief control parameters */
    uint32_t      search_path               : 8;
    uint32_t      len_sp                    : 8;
    uint32_t      reserved0                 : 16;

    /** \brief multi pred l0/1
     * 0000: no internal MV predictor will be used
     * 0001: spatial MV predictors
     * 0100/1000: Reserved
     **/
    uint32_t      multi_pred_l0             : 4;
    uint32_t      multi_pred_l1             : 4;
    /** \brief defines the motion vector precision, like integer/half/quarter pel.
     *  00b: integer pel
     *  01b: half-pel
     *  10b: reserved
     *  11b: quarter-pel
    */
    uint32_t      sub_pel_mode              : 2;
    uint32_t      adaptive_search           : 1;
    /** \brief mv_predictor_input
     * 000: MV predictor disabled
     * 001: MV predictor enabled per 16x16 block
     * 010: MV predictor enabled per 32x32 block
     * 011: MV predictor enabled per 64x64 block
     * 111: MV predictor enabled, block size can vary and is determined by BlockSize in motion vector predictor buffer
     * 100/101/110: Reserved
     **/
    uint32_t      mv_predictor_input        : 3;
    /** \brief enables per CTB or CU qp */
    uint32_t      per_block_qp              : 1;
    /** \brief enables the per CTB input , if 1, need ctb_ctrl to be a real surface ID*/
    uint32_t      per_ctb_input             : 1;
    /** when this flag is set, extra distortion between current CTB and co-located CTB is provided.
     * Extra distortion output has performance impact, set it only when it is needed */
    uint32_t      colocated_ctb_distortion  : 1;
    /** brief specifies whether this CTB should be forced to split to remove Inter big LCU: do not check Inter 32x32
     * PUs. Every 32x32 LCU is split at least once. It can be used to improved performance.
     * 0: ENC determined block type
     * 1: Force to split
     **/
    uint32_t      force_lcu_split           : 1;
    /** \brief enables CU64x64 check */
    uint32_t      enable_cu64_check         : 1;
    /** \brief enables CU64x64 asymmetric motion partition check */
    uint32_t      enable_cu64_amp_check     : 1;
    /** \brief specifies if check the 64x64 merge candidate
     * 0: after skip check,
     * 1: only skip check for 64x64
     Default: 0. This field is used by LCU64 bi-directional.
     **/
    uint32_t      cu64_skip_check_only      : 1;
    uint32_t      reserved1             : 11;
    /** specifies the search region width in pixels.
     *  When bidirectional search is enabled, this applies to both search regions */
    uint32_t      ref_width                 : 8;
    /** specifies the reference region height in pixels. When bidirectional search is enabled,
     * this applies to both search regions. */
    uint32_t      ref_height                : 8;
    /** \brief search window similar for AVC
     *  defines predefined search windows. If it is selected, RefWidth, RefHeight, LenSP and SearchPath are ignored.
     *  0 : not use predefined search window
     *  1 : Tiny – (4 SUs) 24x24 window diamond search
     *  2 : Small – (9 SUs) 28x28 window diamond search
     *  3 : Diamond – (16 SUs) 48x40 window diamond search
     *  4 : Large Diamond – (32 SUs) 48x40 window diamond search
     *  5 : Exhaustive – 48x40 window full search
     *  6 : (64 SUs) 64x64 window full search
     *  Note: option 1, 2, 3 and 4 are valid only when CAP parameter SearchWindow64Support is 0.
     *  And option 6 is valid only when SearchWindow64Support is 1.*/
    uint32_t      search_window             : 8;
    /** \brief number of internal MV predictors for IME searches */
    uint32_t      max_num_ime_search_center : 3;
    /** \brief fast intra prediction enabling bit. It is used as a trade-off between speed and quality.
     *  The flag will be ignored if it's unsupported in the driver*/
    uint32_t      fast_intra_mode           : 1;
    uint32_t      reserved2                 : 4;

    /** \brief specifies number of splits that encoder could be run concurrently
     * 1: level 1, default value
     * 2: level 2
     * 4: level 3
     **/
    uint32_t      num_concurrent_enc_frame_partition : 8;
    uint32_t      reserved3                 : 24;

    /** \brief max frame size control with multi passes QP setting */
    uint32_t      max_frame_size;
    /** \brief number of passes, every pass has different QP */
    uint32_t      num_passes;
    /** \brief delta QP list for every pass */
    uint8_t       *delta_qp;

    uint32_t      reserved4[2];
} VAEncMiscParameterFEIFrameControlHEVC;

/** \brief Application can use this definition as reference to allocate the buffer
 * based on MaxNumPredictor returned from attribute VAConfigAttribFEIMVPredictors query.
 * this buffer allocation is always based on 16x16 block even block size is indicated as 32x32 or 64x64, and buffer
 * layout is always in 32x32 block raster scan order even block size is 16x16 or 64x64. If 32x32 block size is set,
 * only the data in the first 16x16 block (block 0) is used for 32x32 block. If 64x64 block size is set
 * MV layout is still in 32x32 raster scan order, the same as 32x32 and the first 16x16
 * block within each 32x32 block needs to have intended MV data (four 32x32 blocks will have the same MV data in the
 * correspondent first 16x16 block). Data structure for each 16x16 block is defined as below (same as AVC except
 * BlockSize/Reserved bits).
 **/
typedef struct _VAEncFEIMVPredictorHEVC {
    /** \brief Feference index corresponding to the entry of RefPicList0 & RefPicList1 in slice header (final reference
     * list). Note that RefPicList0 & RefPicList1 needs to be the same for all slices.
     * Ref0xIndex – RefPicList0; Ref1xIndex – RefPicList1; x – 0 ~ MaxNumPredictor */
    struct {
        uint8_t   ref_idx_l0    : 4;
        uint8_t   ref_idx_l1    : 4;
    } ref_idx[4]; /* index is predictor number */
    /** \brief Valid only when MVPredictor is set to 011 for HEVC. Only valid in the first 16x16 block.
     * 00: MV predictor disabled for this 32x32 block
     * 01: MV predictor enabled per 16x16 block for this 32x32 block
     * 10: MV predictor enabled per 32x32 block, the rest of 16x16 block data within this 32x32 block are ignored
     * 11: Reserved */
    uint32_t block_size         : 2;
    uint32_t reserved           : 30;

    VAMotionVector mv[4]; /* MaxNumPredictor is 4 */
} VAEncFEIMVPredictorHEVC;    //40 bytes

/** \brief FEI CTB level control data structure */
typedef struct _VAEncFEICTBControlHEVC {
    // DWORD 0
    uint32_t    force_to_intra      : 1;
    uint32_t    force_to_inter      : 1;
    uint32_t    force_to_skip       : 1;
    /** \brief force all coeff to zero */
    uint32_t    force_to_zero_coeff : 1;
    uint32_t    reserved0           : 28;
    // DWORD 1
    uint32_t    reserved1;
    // DWORD 2
    uint32_t    reserved2;
    // DWORD 3
    uint32_t    reserved3;
} VAEncFEICTBControlHEVC;

/** \brief VAEncFEIDistortionHevc defines the data structure for VAEncFEIDistortionBufferType per CTB block.
 * It is output buffer of ENC and ENC_PAK modes, The CTB block is in raster scan order.
 * Buffer size shall not be less than the number of CTB blocks multiple by sizeof(VAEncFEIDistortionHevc).
 **/
typedef struct _VAEncFEIDistortionHevc {
    /** best CTB distortion */
    uint32_t    best_distortion;
    /** only when colocated_ctb_distortion in VAEncMiscParameterFEIFrameControlHEVC is set */
    uint32_t    colocated_ctb_distortion;
} VAEncFEIDistortionHevc;
#ifdef __cplusplus
}
#endif

#endif

