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
 * \file va_dec_hevc.h
 * \brief The HEVC decoding API
 *
 * This file contains the \ref api_dec_hevc "HEVC decoding API".
 */

#ifndef VA_DEC_HEVC_H
#define VA_DEC_HEVC_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * \defgroup api_dec_hevc HEVC decoding API
 *
 * This HEVC decoding API supports Main and Main Still Picture profiles.
 * And it supports both short slice format and long slice format.
 *
 * @{
 */

/**
 * \brief HEVC Decoding Picture Parameter Buffer Structure
 *
 * This structure conveys picture level parameters and should be sent once
 * per frame.
 *
 */
typedef struct  _VAPictureParameterBufferHEVC {
    /** \brief buffer description of decoded current picture
     * only VA_PICTURE_HEVC_FIELD_PIC and VA_PICTURE_HEVC_BOTTOM_FIELD
     * of "flags" fields are meaningful.
     */
    VAPictureHEVC           CurrPic;
    /** \brief buffer description of reference frames in DPB */
    VAPictureHEVC           ReferenceFrames[15];
    /** \brief picture width, shall be integer multiple of minimum CB size. */
    uint16_t                pic_width_in_luma_samples;
    /** \brief picture height, shall be integer multiple of minimum CB size. */
    uint16_t                pic_height_in_luma_samples;


    union {
        struct {
            /** following flags have same syntax and semantic as those in HEVC spec */
            uint32_t        chroma_format_idc                           : 2;
            uint32_t        separate_colour_plane_flag                  : 1;
            uint32_t        pcm_enabled_flag                            : 1;
            uint32_t        scaling_list_enabled_flag                   : 1;
            uint32_t        transform_skip_enabled_flag                 : 1;
            uint32_t        amp_enabled_flag                            : 1;
            uint32_t        strong_intra_smoothing_enabled_flag         : 1;
            uint32_t        sign_data_hiding_enabled_flag               : 1;
            uint32_t        constrained_intra_pred_flag                 : 1;
            uint32_t        cu_qp_delta_enabled_flag                    : 1;
            uint32_t        weighted_pred_flag                          : 1;
            uint32_t        weighted_bipred_flag                        : 1;
            uint32_t        transquant_bypass_enabled_flag              : 1;
            uint32_t        tiles_enabled_flag                          : 1;
            uint32_t        entropy_coding_sync_enabled_flag            : 1;
            uint32_t        pps_loop_filter_across_slices_enabled_flag  : 1;
            uint32_t        loop_filter_across_tiles_enabled_flag       : 1;
            uint32_t        pcm_loop_filter_disabled_flag               : 1;
            /** set based on sps_max_num_reorder_pics of current temporal layer. */
            uint32_t        NoPicReorderingFlag                         : 1;
            /** picture has no B slices */
            uint32_t        NoBiPredFlag                                : 1;

            uint32_t        ReservedBits                                : 11;
        } bits;
        uint32_t            value;
    } pic_fields;

    /** following parameters have same syntax with those in HEVC spec */
    /** \brief DPB size for current temporal layer */
    uint8_t                 sps_max_dec_pic_buffering_minus1;
    uint8_t                 bit_depth_luma_minus8;
    uint8_t                 bit_depth_chroma_minus8;
    uint8_t                 pcm_sample_bit_depth_luma_minus1;
    uint8_t                 pcm_sample_bit_depth_chroma_minus1;
    uint8_t                 log2_min_luma_coding_block_size_minus3;
    uint8_t                 log2_diff_max_min_luma_coding_block_size;
    uint8_t                 log2_min_transform_block_size_minus2;
    uint8_t                 log2_diff_max_min_transform_block_size;
    uint8_t                 log2_min_pcm_luma_coding_block_size_minus3;
    uint8_t                 log2_diff_max_min_pcm_luma_coding_block_size;
    uint8_t                 max_transform_hierarchy_depth_intra;
    uint8_t                 max_transform_hierarchy_depth_inter;
    int8_t                  init_qp_minus26;
    uint8_t                 diff_cu_qp_delta_depth;
    int8_t                  pps_cb_qp_offset;
    int8_t                  pps_cr_qp_offset;
    uint8_t                 log2_parallel_merge_level_minus2;
    uint8_t                 num_tile_columns_minus1;
    uint8_t                 num_tile_rows_minus1;
    /**
     * when uniform_spacing_flag equals 1, application should populate
     * column_width_minus[], and row_height_minus1[] with approperiate values.
     */
    uint16_t                column_width_minus1[19];
    uint16_t                row_height_minus1[21];

    /**
     *  The Following Parameters are needed for Short Slice Format Only.
     *  Only format decoding can ignore them.
     */

    /**
     * \brief Parameters needed for parsing slice segment headers
     */
    union {
        struct {
            /** following parameters have same syntax with those in HEVC spec */
            uint32_t        lists_modification_present_flag             : 1;
            uint32_t        long_term_ref_pics_present_flag             : 1;
            uint32_t        sps_temporal_mvp_enabled_flag               : 1;
            uint32_t        cabac_init_present_flag                     : 1;
            uint32_t        output_flag_present_flag                    : 1;
            uint32_t        dependent_slice_segments_enabled_flag       : 1;
            uint32_t        pps_slice_chroma_qp_offsets_present_flag    : 1;
            uint32_t        sample_adaptive_offset_enabled_flag         : 1;
            uint32_t        deblocking_filter_override_enabled_flag     : 1;
            uint32_t        pps_disable_deblocking_filter_flag          : 1;
            uint32_t        slice_segment_header_extension_present_flag : 1;

            /** current picture with NUT between 16 and 21 inclusive */
            uint32_t        RapPicFlag                                  : 1;
            /** current picture with NUT between 19 and 20 inclusive */
            uint32_t        IdrPicFlag                                  : 1;
            /** current picture has only intra slices */
            uint32_t        IntraPicFlag                                : 1;

            uint32_t        ReservedBits                                : 18;
        } bits;
        uint32_t            value;
    } slice_parsing_fields;

    /** following parameters have same syntax with those in HEVC spec */
    uint8_t                 log2_max_pic_order_cnt_lsb_minus4;
    uint8_t                 num_short_term_ref_pic_sets;
    uint8_t                 num_long_term_ref_pic_sps;
    uint8_t                 num_ref_idx_l0_default_active_minus1;
    uint8_t                 num_ref_idx_l1_default_active_minus1;
    int8_t                  pps_beta_offset_div2;
    int8_t                  pps_tc_offset_div2;
    uint8_t                 num_extra_slice_header_bits;

    /**
     * \brief number of bits that structure
     * short_term_ref_pic_set( num_short_term_ref_pic_sets ) takes in slice
     * segment header when short_term_ref_pic_set_sps_flag equals 0.
     * if short_term_ref_pic_set_sps_flag equals 1, the value should be 0.
     * the bit count is calculated after emulation prevention bytes are removed
     * from bit streams.
     * This variable is used for accelorater to skip parsing the
     * short_term_ref_pic_set( num_short_term_ref_pic_sets ) structure.
     */
    uint32_t                st_rps_bits;

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_MEDIUM];
} VAPictureParameterBufferHEVC;

/**
 * \brief HEVC Decoding Picture Parameter Buffer Structure for Range Extension
 *
 * This structure conveys picture level HEVC Range Extension parameters
 * and should be sent once per frame. This data structure should be sent
 * together with VAPictureParameterBufferHEVC in a single buffer of
 * \ref VAPictureParameterBufferHEVCExtension since each frame
 * of HEVC range extension contains both picture level parameters and picture
 * level range extension parameters. They should be parsed together. The buffer
 * type is same as \ref VAPictureParameterBufferHEVC.
 *
 */
typedef struct  _VAPictureParameterBufferHEVCRext {
    union {
        struct {
            /** \brief HEVC range extension flags
             *  The following flags bears the same syntax and semantics as
             *  those defined in HEVC bit stream spec.
             */
            uint32_t        transform_skip_rotation_enabled_flag        : 1;
            uint32_t        transform_skip_context_enabled_flag         : 1;
            uint32_t        implicit_rdpcm_enabled_flag                 : 1;
            uint32_t        explicit_rdpcm_enabled_flag                 : 1;
            uint32_t        extended_precision_processing_flag          : 1;
            uint32_t        intra_smoothing_disabled_flag               : 1;
            uint32_t        high_precision_offsets_enabled_flag         : 1;
            uint32_t        persistent_rice_adaptation_enabled_flag     : 1;
            uint32_t        cabac_bypass_alignment_enabled_flag         : 1;
            uint32_t        cross_component_prediction_enabled_flag     : 1;
            uint32_t        chroma_qp_offset_list_enabled_flag          : 1;

            /** \brief Reserved bytes for future use, must be zero */
            uint32_t        reserved                                    : 21;
        } bits;
        uint32_t            value;
    } range_extension_pic_fields;

    /** \brief HEVC range extension flags
     *  The following flags bears the same syntax and semantics as
     *  those defined in HEVC bit stream spec.
     */
    uint8_t                 diff_cu_chroma_qp_offset_depth;
    uint8_t                 chroma_qp_offset_list_len_minus1;
    uint8_t                 log2_sao_offset_scale_luma;
    uint8_t                 log2_sao_offset_scale_chroma;
    uint8_t                 log2_max_transform_skip_block_size_minus2;
    int8_t                  cb_qp_offset_list[6];
    int8_t                  cr_qp_offset_list[6];
} VAPictureParameterBufferHEVCRext;

/**
 *\brief HEVC Decoding Picture Parameter Buffer Structure for
 *Screen Content extension
 *
 *This structure conveys picture level HEVC Scc parameters
 *and should be sent once per frame. This data structure should be sent
 *together with VAPictureParameterBufferHEVC and VAPictureParameterBufferHEVCRext
 *in a single buffer of \ref VAPictureParameterBufferHEVCExtension since each
 *frame of HEVC SCC contains picture level parameters, picture level range
 *extension parameters and picture level Scc parameters. They should be parsed
 *together and the buffer type is same as \ref VAPictureParameterBufferHEVC.
 *
 */
typedef struct  _VAPictureParameterBufferHEVCScc {
    union {
        struct {
            /** \brief HEVC Scc extension flags
             *  The following flags bears the same syntax and semantics as
             *  those defined in HEVC bit stream spec.
             */
            /*  indicates if intra block copy (IBC) is enabled or not. */
            uint32_t        pps_curr_pic_ref_enabled_flag                   : 1;
            /*  indicates if Palette Mode is enabled or not. */
            uint32_t        palette_mode_enabled_flag                       : 1;
            /*  controls the presence and inference of the use_integer_mv_flag syntax
             *  in slice segment header that specifies the resolution of motion
             *  vectors for inter prediction.
             */
            uint32_t        motion_vector_resolution_control_idc            : 2;
            /*  specifies that the intra boundary filtering process is
             *  disabled or not for intra prediction.
             */
            uint32_t        intra_boundary_filtering_disabled_flag          : 1;
            /*  specifies that an adaptive colour transform may be applied
             *  to the residual in the decoding process.
             */
            uint32_t        residual_adaptive_colour_transform_enabled_flag : 1;

            /* specifies that slice_act_y_qp_offset, slice_act_cb_qp_offset,
             * slice_act_cr_qp_offset are present in the slice header
             */
            uint32_t        pps_slice_act_qp_offsets_present_flag           : 1;

            /** \brief Reserved bytes for future use, must be zero */
            uint32_t        reserved                                        : 25;
        } bits;
        uint32_t            value;
    } screen_content_pic_fields;

    /*  specifies the maximum allowed palette size. */
    uint8_t                 palette_max_size;
    /*  Correspond to HEVC syntax elements of the same names.
     *  It specifies the difference between the maximum allowed palette
     *  predictor size and the maximum allowed palette size.
     *  App needs to enforce that the variable PaletteMaxPredictorSize,
     *  which is derived as follows:
     *  PaletteMaxPredictorSize = palette_max_size + delta_palette_max_predictor_size
     *  should have a value range of [0..128].
     */
    uint8_t                 delta_palette_max_predictor_size;
    /** \brief Size of initial palette predictor.
     *  It is derived from pps_num_palette_predictor_initializer or
     *  sps_num_palette_predictor_initializer_minus1.
     *  Details in HEVC SCC spec section 9.3.2.3.
     */
    uint8_t                 predictor_palette_size;
    /** \brief Palette predictor initializer.
     *  It is derived from pps_palette_predictor_initializers[][]
     *  or sps_palette_predictor_initializers[][].
     *  Details in HEVC SCC spec section 9.3.2.3.
     */
    uint16_t                predictor_palette_entries[3][128];
    /*  are used to determine the offsets that are applied to the
     *  quantization parameter values for the luma, Cb and Cr
     *  components, respectively.
     */
    int8_t                  pps_act_y_qp_offset_plus5;
    int8_t                  pps_act_cb_qp_offset_plus5;
    int8_t                  pps_act_cr_qp_offset_plus3;
} VAPictureParameterBufferHEVCScc;

/**
 * \brief HEVC Decoding Picture Parameter Buffer Structure including Extensions
 *
 * This structure conveys picture level HEVC parameters including basic version 1
 * and range extension and screen content extension.
 * The data buffer should be sent once per frame.
 *
 */
typedef struct  _VAPictureParameterBufferHEVCExtension {
    /** \brief basic HEVC picture parameters data structure
     */
    VAPictureParameterBufferHEVC           base;

    /** \brief HEVC range extension picture parameters data structure
     */
    VAPictureParameterBufferHEVCRext       rext;

    /** \brief HEVC screen content picture parameters data structure
     */
    VAPictureParameterBufferHEVCScc        scc;
} VAPictureParameterBufferHEVCExtension;

/**
 * \brief HEVC Slice Parameter Buffer Structure For Long Format
 *
 * VASliceParameterBufferHEVC structure should be accompanied by a
 * slice data buffer, which holds the whole raw slice NAL unit bit streams
 * including start code prefix and emulation prevention bytes not removed.
 *
 * This structure conveys parameters related to slice segment header and should
 * be sent once per slice.
 *
 * For short format, this data structure is not sent by application.
 *
 */
typedef struct  _VASliceParameterBufferHEVC {
    /** @name Codec-independent Slice Parameter Buffer base. */

    /**@{*/

    /** \brief Number of bytes in the slice data buffer for this slice
     * counting from and including NAL unit header.
     */
    uint32_t                slice_data_size;
    /** \brief The offset to the NAL unit header for this slice */
    uint32_t                slice_data_offset;
    /** \brief Slice data buffer flags. See \c VA_SLICE_DATA_FLAG_XXX. */
    uint32_t                slice_data_flag;
    /**
     * \brief Byte offset from NAL unit header to the begining of slice_data().
     *
     * This byte offset is relative to and includes the NAL unit header
     * and represents the number of bytes parsed in the slice_header()
     * after the removal of any emulation prevention bytes in
     * there. However, the slice data buffer passed to the hardware is
     * the original bitstream, thus including any emulation prevention
     * bytes.
     */
    uint32_t                slice_data_byte_offset;
    /** HEVC syntax element. */
    uint32_t                slice_segment_address;
    /** \brief index into ReferenceFrames[]
     * RefPicList[0][] corresponds to RefPicList0[] of HEVC variable.
     * RefPicList[1][] corresponds to RefPicList1[] of HEVC variable.
     * value range [0..14, 0xFF], where 0xFF indicates invalid entry.
     */
    uint8_t                 RefPicList[2][15];
    union {
        uint32_t            value;
        struct {
            /** current slice is last slice of picture. */
            uint32_t        LastSliceOfPic                              : 1;
            /** HEVC syntax element. */
            uint32_t        dependent_slice_segment_flag                : 1;
            /** HEVC syntax element. */
            uint32_t        slice_type                                  : 2;
            /** HEVC syntax element. */
            uint32_t        color_plane_id                              : 2;
            /** HEVC syntax element. */
            uint32_t        slice_sao_luma_flag                         : 1;
            /** HEVC syntax element. */
            uint32_t        slice_sao_chroma_flag                       : 1;
            /** HEVC syntax element. */
            uint32_t        mvd_l1_zero_flag                            : 1;
            /** HEVC syntax element. */
            uint32_t        cabac_init_flag                             : 1;
            /** HEVC syntax element. */
            uint32_t        slice_temporal_mvp_enabled_flag             : 1;
            /** HEVC syntax element. */
            uint32_t        slice_deblocking_filter_disabled_flag       : 1;
            /** HEVC syntax element. */
            uint32_t        collocated_from_l0_flag                     : 1;
            /** HEVC syntax element. */
            uint32_t        slice_loop_filter_across_slices_enabled_flag : 1;
            uint32_t        reserved                                    : 18;
        } fields;
    } LongSliceFlags;

    /** HEVC syntax element. Collocated Reference Picture Index.
     * index to RefPicList[0][] or RefPicList[1][].
     * when slice_temporal_mvp_enabled_flag equals 0, it should take value 0xFF.
     * value range [0..14, 0xFF].
     */
    uint8_t                 collocated_ref_idx;
    /** HEVC syntax element.
     * if num_ref_idx_active_override_flag equals 0, host decoder should
     * set its value to num_ref_idx_l0_default_active_minus1.
     */
    uint8_t                 num_ref_idx_l0_active_minus1;
    /** HEVC syntax element.
     * if num_ref_idx_active_override_flag equals 0, host decoder should
     * set its value to num_ref_idx_l1_default_active_minus1.
     */
    uint8_t                 num_ref_idx_l1_active_minus1;
    /** HEVC syntax element. */
    int8_t                  slice_qp_delta;
    /** HEVC syntax element. */
    int8_t                  slice_cb_qp_offset;
    /** HEVC syntax element. */
    int8_t                  slice_cr_qp_offset;
    /** HEVC syntax element. */
    int8_t                  slice_beta_offset_div2;
    /** HEVC syntax element. */
    int8_t                  slice_tc_offset_div2;
    /** HEVC syntax element. */
    uint8_t                 luma_log2_weight_denom;
    /** HEVC syntax element. */
    int8_t                  delta_chroma_log2_weight_denom;
    /** HEVC syntax element. */
    int8_t                  delta_luma_weight_l0[15];
    /** HEVC syntax element. */
    int8_t                  luma_offset_l0[15];
    /** HEVC syntax element. */
    int8_t                  delta_chroma_weight_l0[15][2];
    /** corresponds to HEVC spec variable of the same name. */
    int8_t                  ChromaOffsetL0[15][2];
    /** HEVC syntax element. */
    int8_t                  delta_luma_weight_l1[15];
    /** HEVC syntax element. */
    int8_t                  luma_offset_l1[15];
    /** HEVC syntax element. */
    int8_t                  delta_chroma_weight_l1[15][2];
    /** corresponds to HEVC spec variable of the same name. */
    int8_t                  ChromaOffsetL1[15][2];
    /** HEVC syntax element. */
    uint8_t                 five_minus_max_num_merge_cand;
    /** HEVC syntax element. */
    uint16_t                 num_entry_point_offsets;
    /** HEVC syntax element. */
    uint16_t                 entry_offset_to_subset_array;
    /** \brief Number of emulation prevention bytes in slice header. */
    uint16_t                slice_data_num_emu_prevn_bytes;
    /**@}*/

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_LOW - 2];
} VASliceParameterBufferHEVC;

/**
 * \brief HEVC Extented Slice Parameter Buffer Structure For Long Format
 *
 * This data structure contains extension profiles (range extension and screen content).
 *
 * VASliceParameterBufferHEVCRext structure should be accompanied by a
 * slice data buffer, which holds the whole raw slice NAL unit bit streams
 * including start code prefix and emulation prevention bytes not removed.
 *
 * This structure conveys parameters related to slice segment header and should
 * be sent once per slice with VASliceParameterBufferHEVC in a single buffer of
 * \ref VASliceParameterBufferHEVCExtension and the buffer type is same as \ref
 * VASliceParameterBufferHEVC.
 *
 * For short format, this data structure is not sent by application.
 *
 */
typedef struct  _VASliceParameterBufferHEVCRext {
    /* below four parameters are used to replace data types of the
     * corresponding parameters of those in \# VASliceParameterBufferHEVC.
     */
    int16_t                 luma_offset_l0[15];
    int16_t                 ChromaOffsetL0[15][2];
    int16_t                 luma_offset_l1[15];
    int16_t                 ChromaOffsetL1[15][2];

    union {
        struct {
            uint32_t        cu_chroma_qp_offset_enabled_flag    : 1;
            uint32_t        use_integer_mv_flag                 : 1;
            /** \brief Reserved bytes for future use, must be zero */
            uint32_t        reserved                            : 30;
        } bits;
        uint32_t            value;
    } slice_ext_flags;

    /** \brief Screen Content Extension parameters.
     *  data range [-12..12]
     */
    int8_t                  slice_act_y_qp_offset;
    int8_t                  slice_act_cb_qp_offset;
    int8_t                  slice_act_cr_qp_offset;
} VASliceParameterBufferHEVCRext;

/**
 * \brief HEVC Decoding Slice Parameter Buffer Structure For Long Format including Extensions
 *
 * This data structure contains both baseline HEVC profiles (main, main10)
 * and extension profiles (range extension and screen content).
 *
 * VASliceParameterBufferHEVCExtension structure should be accompanied by a
 * slice data buffer, which holds the whole raw slice NAL unit bit streams
 * including start code prefix and emulation prevention bytes not removed.
 *
 * This structure conveys parameters related to slice segment header and should
 * be sent once per slice. For HEVC range extension and HEVC Scc decoding,
 * application should parse both basic slice parameters and extented slice
 * parameters into this buffer structure and sent it.
 *
 * For short format, this data structure is not sent by application.
 *
 */
typedef struct  _VASliceParameterBufferHEVCExtension {
    /** \brief baseline HEVC slice parameters data structure */
    VASliceParameterBufferHEVC               base;

    /** \brief extented HEVC slice parameters data structure */
    VASliceParameterBufferHEVCRext           rext;
} VASliceParameterBufferHEVCExtension;

/**
 * \brief HEVC Inverse Quantization Matrix Buffer Structure
 *
 * This structure is sent once per frame,
 * and only when scaling_list_enabled_flag = 1.
 * When sps_scaling_list_data_present_flag = 0, app still
 * needs to send in this structure with default matrix values.
 *
 * Matrix entries are in raster scan order which follows HEVC spec.
 */
typedef struct _VAIQMatrixBufferHEVC {
    /**
     * \brief scaling lists,
     * corresponds to same HEVC spec syntax element
     * ScalingList[ i ][ MatrixID ][ j ].
     *
     * \brief 4x4 scaling,
     * correspongs i = 0, MatrixID is in the range of 0 to 5,
     * inclusive. And j is in the range of 0 to 15, inclusive.
     */
    uint8_t                 ScalingList4x4[6][16];
    /**
     * \brief 8x8 scaling,
     * correspongs i = 1, MatrixID is in the range of 0 to 5,
     * inclusive. And j is in the range of 0 to 63, inclusive.
     */
    uint8_t                 ScalingList8x8[6][64];
    /**
     * \brief 16x16 scaling,
     * correspongs i = 2, MatrixID is in the range of 0 to 5,
     * inclusive. And j is in the range of 0 to 63, inclusive.
     */
    uint8_t                 ScalingList16x16[6][64];
    /**
     * \brief 32x32 scaling,
     * correspongs i = 3, MatrixID is in the range of 0 to 1,
     * inclusive. And j is in the range of 0 to 63, inclusive.
     */
    uint8_t                 ScalingList32x32[2][64];
    /**
     * \brief DC values of the 16x16 scaling lists,
     * corresponds to HEVC spec syntax
     * scaling_list_dc_coef_minus8[ sizeID - 2 ][ matrixID ] + 8
     * with sizeID = 2 and matrixID in the range of 0 to 5, inclusive.
     */
    uint8_t                 ScalingListDC16x16[6];
    /**
     * \brief DC values of the 32x32 scaling lists,
     * corresponds to HEVC spec syntax
     * scaling_list_dc_coef_minus8[ sizeID - 2 ][ matrixID ] + 8
     * with sizeID = 3 and matrixID in the range of 0 to 1, inclusive.
     */
    uint8_t                 ScalingListDC32x32[2];

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_LOW];
} VAIQMatrixBufferHEVC;


/**@}*/

#ifdef __cplusplus
}
#endif

#endif /* VA_DEC_HEVC_H */
