/*
 * Copyright (c) 2014 Intel Corporation. All Rights Reserved.
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
 * \file va_dec_vp9.h
 * \brief The VP9 decoding API
 *
 * This file contains the \ref api_dec_vp9 "VP9 decoding API".
 */

#ifndef VA_DEC_VP9_H
#define VA_DEC_VP9_H

#ifdef __cplusplus
extern "C" {
#endif

/**
 * \defgroup api_dec_vp9 VP9 decoding API
 *
 * This VP9 decoding API supports 8-bit 420 format only.
 *
 * @{
 */




/**
 * \brief VP9 Decoding Picture Parameter Buffer Structure
 *
 * This structure conveys picture level parameters.
 * App should send a surface with this data structure down to VAAPI once
 * per frame.
 *
 */
typedef struct  _VADecPictureParameterBufferVP9 {
    /** \brief picture width
     *  Picture original resolution. The value may not be multiple of 8.
     */
    uint16_t                frame_width;
    /** \brief picture height
     *  Picture original resolution. The value may not be multiple of 8.
     */
    uint16_t                frame_height;

    /** \brief Surface indices of reference frames in DPB.
     *
     *  Each entry of the list specifies the surface index of the picture
     *  that is referred by current picture or will be referred by any future
     *  picture.
     *  Application who calls this API should update this list based on the
     *  refreshing information from VP9 bitstream.
     */
    VASurfaceID             reference_frames[8];

    union {
        struct {
            /** \brief flags for current picture
             *  same syntax and semantic as those in VP9 code
             */
            uint32_t        subsampling_x                               : 1;
            uint32_t        subsampling_y                               : 1;
            uint32_t        frame_type                                  : 1;
            uint32_t        show_frame                                  : 1;
            uint32_t        error_resilient_mode                        : 1;
            uint32_t        intra_only                                  : 1;
            uint32_t        allow_high_precision_mv                     : 1;
            uint32_t        mcomp_filter_type                           : 3;
            uint32_t        frame_parallel_decoding_mode                : 1;
            uint32_t        reset_frame_context                         : 2;
            uint32_t        refresh_frame_context                       : 1;
            uint32_t        frame_context_idx                           : 2;
            uint32_t        segmentation_enabled                        : 1;

            /** \brief corresponds to variable temporal_update in VP9 code.
             */
            uint32_t        segmentation_temporal_update                : 1;
            /** \brief corresponds to variable update_mb_segmentation_map
             *  in VP9 code.
             */
            uint32_t        segmentation_update_map                     : 1;

            /** \brief Index of reference_frames[] and points to the
             *  LAST reference frame.
             *  It corresponds to active_ref_idx[0] in VP9 code.
             */
            uint32_t        last_ref_frame                              : 3;
            /** \brief Sign Bias of the LAST reference frame.
             *  It corresponds to ref_frame_sign_bias[LAST_FRAME] in VP9 code.
             */
            uint32_t        last_ref_frame_sign_bias                    : 1;
            /** \brief Index of reference_frames[] and points to the
             *  GOLDERN reference frame.
             *  It corresponds to active_ref_idx[1] in VP9 code.
             */
            uint32_t        golden_ref_frame                            : 3;
            /** \brief Sign Bias of the GOLDERN reference frame.
             *  Corresponds to ref_frame_sign_bias[GOLDERN_FRAME] in VP9 code.
             */
            uint32_t        golden_ref_frame_sign_bias                  : 1;
            /** \brief Index of reference_frames[] and points to the
             *  ALTERNATE reference frame.
             *  Corresponds to active_ref_idx[2] in VP9 code.
             */
            uint32_t        alt_ref_frame                               : 3;
            /** \brief Sign Bias of the ALTERNATE reference frame.
             *  Corresponds to ref_frame_sign_bias[ALTREF_FRAME] in VP9 code.
             */
            uint32_t        alt_ref_frame_sign_bias                     : 1;
            /** \brief Lossless Mode
             *  LosslessFlag = base_qindex == 0 &&
             *                 y_dc_delta_q == 0 &&
             *                 uv_dc_delta_q == 0 &&
             *                 uv_ac_delta_q == 0;
             *  Where base_qindex, y_dc_delta_q, uv_dc_delta_q and uv_ac_delta_q
             *  are all variables in VP9 code.
             */
            uint32_t        lossless_flag                               : 1;
        } bits;
        uint32_t            value;
    } pic_fields;

    /* following parameters have same syntax with those in VP9 code */
    uint8_t                 filter_level;
    uint8_t                 sharpness_level;

    /** \brief number of tile rows specified by (1 << log2_tile_rows).
     *  It corresponds the variable with same name in VP9 code.
     */
    uint8_t                 log2_tile_rows;
    /** \brief number of tile columns specified by (1 << log2_tile_columns).
     *  It corresponds the variable with same name in VP9 code.
     */
    uint8_t                 log2_tile_columns;
    /** \brief Number of bytes taken up by the uncompressed frame header,
     *  which corresponds to byte length of function
     *  read_uncompressed_header() in VP9 code.
     *  Specifically, it is the byte count from bit stream buffer start to
     *  the last byte of uncompressed frame header.
     *  If there are other meta data in the buffer before uncompressed header,
     *  its size should be also included here.
     */
    uint8_t                 frame_header_length_in_bytes;

    /** \brief The byte count of compressed header the bitstream buffer,
     *  which corresponds to syntax first_partition_size in code.
     */
    uint16_t                first_partition_size;

    /** These values are segment probabilities with same names in VP9
     *  function setup_segmentation(). They should be parsed directly from
     *  bitstream by application.
     */
    uint8_t                 mb_segment_tree_probs[7];
    uint8_t                 segment_pred_probs[3];

    /** \brief VP9 Profile definition
     *  value range [0..3].
      */
    uint8_t                 profile;

    /** \brief VP9 bit depth per sample
     *  same for both luma and chroma samples.
     */
    uint8_t                 bit_depth;

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_MEDIUM];

} VADecPictureParameterBufferVP9;



/**
 * \brief VP9 Segmentation Parameter Data Structure
 *
 * This structure conveys per segment parameters.
 * 8 of this data structure will be included in VASegmentationParameterBufferVP9
 * and sent to API in a single buffer.
 *
 */
typedef struct  _VASegmentParameterVP9 {
    union {
        struct {
            /** \brief Indicates if per segment reference frame indicator
             *  is enabled.
             *  Corresponding to variable feature_enabled when
             *  j == SEG_LVL_REF_FRAME in function setup_segmentation() VP9 code.
             */
            uint16_t        segment_reference_enabled                   : 1;
            /** \brief Specifies per segment reference indication.
             *  0: reserved
             *  1: Last ref
             *  2: golden
             *  3: altref
             *  Value can be derived from variable data when
             *  j == SEG_LVL_REF_FRAME in function setup_segmentation() VP9 code.
             */
            uint16_t        segment_reference                           : 2;
            /** \brief Indicates if per segment skip feature is enabled.
             *  Corresponding to variable feature_enabled when
             *  j == SEG_LVL_SKIP in function setup_segmentation() VP9 code.
             */
            uint16_t        segment_reference_skipped                   : 1;
        } fields;
        uint16_t            value;
    } segment_flags;

    /** \brief Specifies the filter level information per segment.
     *  The value corresponds to variable lfi->lvl[seg][ref][mode] in VP9 code,
     *  where m is [ref], and n is [mode] in FilterLevel[m][n].
     */
    uint8_t                 filter_level[4][2];
    /** \brief Specifies per segment Luma AC quantization scale.
     *  Corresponding to y_dequant[qindex][1] in vp9_mb_init_quantizer()
     *  function of VP9 code.
     */
    int16_t                 luma_ac_quant_scale;
    /** \brief Specifies per segment Luma DC quantization scale.
     *  Corresponding to y_dequant[qindex][0] in vp9_mb_init_quantizer()
     *  function of VP9 code.
     */
    int16_t                 luma_dc_quant_scale;
    /** \brief Specifies per segment Chroma AC quantization scale.
     *  Corresponding to uv_dequant[qindex][1] in vp9_mb_init_quantizer()
     *  function of VP9 code.
     */
    int16_t                 chroma_ac_quant_scale;
    /** \brief Specifies per segment Chroma DC quantization scale.
     *  Corresponding to uv_dequant[qindex][0] in vp9_mb_init_quantizer()
     *  function of VP9 code.
     */
    int16_t                 chroma_dc_quant_scale;

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_LOW];

} VASegmentParameterVP9;



/**
 * \brief VP9 Slice Parameter Buffer Structure
 *
 * This structure conveys parameters related to segmentation data and should be
 * sent once per frame.
 *
 * When segmentation is disabled, only SegParam[0] has valid values,
 * all other entries should be populated with 0.
 * Otherwise, all eight entries should be valid.
 *
 * Slice data buffer of VASliceDataBufferType is used
 * to send the bitstream which should include whole or part of partition 0
 * (at least compressed header) to the end of frame.
 *
 */
typedef struct _VASliceParameterBufferVP9 {
    /** \brief The byte count of current frame in the bitstream buffer,
     *  starting from first byte of the buffer.
     *  It uses the name slice_data_size to be consitent with other codec,
     *  but actually means frame_data_size.
     */
    uint32_t slice_data_size;
    /**
     * offset to the first byte of partition data (control partition)
     */
    uint32_t slice_data_offset;
    /**
     * see VA_SLICE_DATA_FLAG_XXX definitions
     */
    uint32_t slice_data_flag;

    /**
     * \brief per segment information
     */
    VASegmentParameterVP9   seg_param[8];

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_LOW];

} VASliceParameterBufferVP9;


/**@}*/

#ifdef __cplusplus
}
#endif

#endif /* VA_DEC_VP9_H */
