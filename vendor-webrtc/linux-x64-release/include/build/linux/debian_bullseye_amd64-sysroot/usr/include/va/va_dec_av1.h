/*
 * Copyright (c) 2019 Intel Corporation. All Rights Reserved.
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
 * \file va_dec_av1.h
 * \brief The AV1 decoding API
 *
 * This file contains the \ref api_dec_av1 "AV1 decoding API".
 */

#ifndef VA_DEC_AV1_H
#define VA_DEC_AV1_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * \defgroup api_dec_av1 AV1 decoding API
 *
 * This AV1 decoding API supports 8-bit/10bit 420 format only.
 *
 * @{
 */

/** Attribute value for VAConfigAttribDecAV1Features.
 *
 * This attribute decribes the supported features of a AV1
 * decoder configuration.
 *
 */
typedef union VAConfigAttribValDecAV1Features {
    struct {
        /** large scale tile
         *
         * This conveys whether AV1 large scale tile is supported by HW.
         * 0 - unsupported, 1 - supported.
         */
        uint32_t lst_support     : 2;
        /* Reserved for future use. */
        uint32_t reserved        : 30;
    } bits;
    uint32_t value;
} VAConfigAttribValDecAV1Features;

/**
 * \brief AV1 Decoding Picture Parameter Buffer Structure
 *
 * This structure conveys picture level parameters.
 * App should send a surface with this data structure down to VAAPI once
 * per frame.
 *
 */

/** \brief Segmentation Information
  */
typedef struct _VASegmentationStructAV1 {
    union {
        struct {
            /** Indicates whether segmentation map related syntax elements
            *  are present or not for current frame. If equal to 0,
            *  the segmentation map related syntax elements are
            *  not present for the current frame and the control flags of
            *  segmentation map related tables feature_data[][], and
            *  feature_mask[] are not valid and shall be ignored by accelerator.
            */
            uint32_t         enabled                                     : 1;
            /** Value 1 indicates that the segmentation map are updated
             *  during the decoding of this frame.
             *  Value 0 means that the segmentation map from the previous
             *  frame is used.
             */
            uint32_t         update_map                                  : 1;
            /** Value 1 indicates that the updates to the segmentation map
             *  are coded relative to the existing segmentation map.
             *  Value 0 indicates that the new segmentation map is coded
             *  without reference to the existing segmentation map.
             */
            uint32_t         temporal_update                             : 1;
            /** Value 1 indicates that new parameters are about to be
             *  specified for each segment.
             *  Value 0 indicates that the segmentation parameters
             *  should keep their existing values.
             */
            uint32_t         update_data                                 : 1;

            /** \brief Reserved bytes for future use, must be zero */
            uint32_t         reserved                                    : 28;
        } bits;
        uint32_t             value;
    } segment_info_fields;

    /** \brief Segmentation parameters for current frame.
     *  feature_data[segment_id][feature_id]
     *  where segment_id has value range [0..7] indicating the segment id.
     *  and feature_id is defined as
        typedef enum {
            SEG_LVL_ALT_Q,       // Use alternate Quantizer ....
            SEG_LVL_ALT_LF_Y_V,  // Use alternate loop filter value on y plane vertical
            SEG_LVL_ALT_LF_Y_H,  // Use alternate loop filter value on y plane horizontal
            SEG_LVL_ALT_LF_U,    // Use alternate loop filter value on u plane
            SEG_LVL_ALT_LF_V,    // Use alternate loop filter value on v plane
            SEG_LVL_REF_FRAME,   // Optional Segment reference frame
            SEG_LVL_SKIP,        // Optional Segment (0,0) + skip mode
            SEG_LVL_GLOBALMV,
            SEG_LVL_MAX
        } SEG_LVL_FEATURES;
     *  feature_data[][] is equivalent to variable FeatureData[][] in spec,
     *  which is after clip3() operation.
     *  Clip3(x, y, z) = (z < x)? x : ((z > y)? y : z);
     *  The limit is defined in Segmentation_Feature_Max[ SEG_LVL_MAX ] = {
     *  255, MAX_LOOP_FILTER, MAX_LOOP_FILTER, MAX_LOOP_FILTER, MAX_LOOP_FILTER, 7, 0, 0 }
     */
    int16_t                 feature_data[8][8];

    /** \brief indicates if a feature is enabled or not.
     *  Each bit field itself is the feature_id. Index is segment_id.
     *  feature_mask[segment_id] & (1 << feature_id) equal to 1 specify that the feature of
     *  feature_id for segment of segment_id is enabled, otherwise disabled.
     */
    uint8_t                 feature_mask[8];

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_LOW];

} VASegmentationStructAV1;

/** \brief Film Grain Information
  */
typedef struct _VAFilmGrainStructAV1 {
    union {
        struct {
            /** \brief Specify whether or not film grain is applied on current frame.
             *  If set to 0, all the rest parameters should be set to zero
             *  and ignored.
             */
            uint32_t        apply_grain                                 : 1;
            uint32_t        chroma_scaling_from_luma                    : 1;
            uint32_t        grain_scaling_minus_8                       : 2;
            uint32_t        ar_coeff_lag                                : 2;
            uint32_t        ar_coeff_shift_minus_6                      : 2;
            uint32_t        grain_scale_shift                           : 2;
            uint32_t        overlap_flag                                : 1;
            uint32_t        clip_to_restricted_range                    : 1;
            /** \brief Reserved bytes for future use, must be zero */
            uint32_t        reserved                                    : 20;
        } bits;
        uint32_t            value;
    } film_grain_info_fields;

    uint16_t                grain_seed;
    /*  value range [0..14] */
    uint8_t                 num_y_points;
    uint8_t                 point_y_value[14];
    uint8_t                 point_y_scaling[14];
    /*  value range [0..10] */
    uint8_t                 num_cb_points;
    uint8_t                 point_cb_value[10];
    uint8_t                 point_cb_scaling[10];
    /*  value range [0..10] */
    uint8_t                 num_cr_points;
    uint8_t                 point_cr_value[10];
    uint8_t                 point_cr_scaling[10];
    /*  value range [-128..127] */
    int8_t                  ar_coeffs_y[24];
    int8_t                  ar_coeffs_cb[25];
    int8_t                  ar_coeffs_cr[25];
    uint8_t                 cb_mult;
    uint8_t                 cb_luma_mult;
    uint16_t                cb_offset;
    uint8_t                 cr_mult;
    uint8_t                 cr_luma_mult;
    uint16_t                cr_offset;

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_LOW];

} VAFilmGrainStructAV1;

typedef enum {
    /** identity transformation, 0-parameter */
    VAAV1TransformationIdentity           = 0,
    /** translational motion, 2-parameter */
    VAAV1TransformationTranslation        = 1,
    /** simplified affine with rotation + zoom only, 4-parameter */
    VAAV1TransformationRotzoom            = 2,
    /** affine, 6-parameter */
    VAAV1TransformationAffine             = 3,
    /** transformation count */
    VAAV1TransformationCount
} VAAV1TransformationType;

typedef struct _VAWarpedMotionParamsAV1 {

    /** \brief Specify the type of warped motion */
    VAAV1TransformationType  wmtype;

    /** \brief Specify warp motion parameters
     *  wm.wmmat[] corresponds to gm_params[][] in spec.
     *  Details in AV1 spec section 5.9.24 or refer to libaom code
     *  https://aomedia.googlesource.com/aom/+/refs/heads/master/av1/decoder/decodeframe.c
     */
    int32_t                 wmmat[8];

    /* valid or invalid on affine set */
    uint8_t  invalid;

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_LOW];

} VAWarpedMotionParamsAV1;

/**
 * \brief AV1 Decoding Picture Parameter Buffer Structure
 *
 * This structure conveys picture level parameters and should be sent once
 * per frame.
 *
 */
typedef struct  _VADecPictureParameterBufferAV1 {
    /**@{*/

    /** \brief sequence level information
     */

    /** \brief AV1 bit stream profile
     */
    uint8_t                 profile;

    uint8_t                 order_hint_bits_minus_1;

    /** \brief bit depth index
     *  value range [0..2]
     *  0 - bit depth 8;
     *  1 - bit depth 10;
     *  2 - bit depth 12;
     */
    uint8_t                 bit_depth_idx;

    /** \brief corresponds to AV1 spec variable of the same name. */
    uint8_t                 matrix_coefficients;

    union {
        struct {
            uint32_t        still_picture                               : 1;
            uint32_t        use_128x128_superblock                      : 1;
            uint32_t        enable_filter_intra                         : 1;
            uint32_t        enable_intra_edge_filter                    : 1;

            /** read_compound_tools */
            uint32_t        enable_interintra_compound                  : 1;
            uint32_t        enable_masked_compound                      : 1;

            uint32_t        enable_dual_filter                          : 1;
            uint32_t        enable_order_hint                           : 1;
            uint32_t        enable_jnt_comp                             : 1;
            uint32_t        enable_cdef                                 : 1;
            uint32_t        mono_chrome                                 : 1;
            uint32_t        color_range                                 : 1;
            uint32_t        subsampling_x                               : 1;
            uint32_t        subsampling_y                               : 1;
            va_deprecated uint32_t        chroma_sample_position        : 1;
            uint32_t        film_grain_params_present                   : 1;
            /** \brief Reserved bytes for future use, must be zero */
            uint32_t        reserved                                    : 16;
        } fields;
        uint32_t value;
    } seq_info_fields;

    /** \brief Picture level information
     */

    /** \brief buffer description of decoded current picture
     */
    VASurfaceID             current_frame;

    /** \brief display buffer of current picture
     *  Used for film grain applied decoded picture.
     *  Valid only when apply_grain equals 1.
     */
    VASurfaceID             current_display_picture;

    /** \brief number of anchor frames for large scale tile
     *  This parameter gives the number of entries of anchor_frames_list[].
     *  Value range [0..128].
     */
    uint8_t                anchor_frames_num;

    /** \brief anchor frame list for large scale tile
     *  For large scale tile applications, the anchor frames could come from
     *  previously decoded frames in current sequence (aka. internal), or
     *  from external sources.
     *  For external anchor frames, application should call API
     *  vaCreateBuffer() to generate frame buffers and populate them with
     *  pixel frames. And this process may happen multiple times.
     *  The array anchor_frames_list[] is used to register all the available
     *  anchor frames from both external and internal, up to the current
     *  frame instance. If a previously registerred anchor frame is no longer
     *  needed, it should be removed from the list. But it does not prevent
     *  applications from relacing the frame buffer with new anchor frames.
     *  Please note that the internal anchor frames may not still be present
     *  in the current DPB buffer. But if it is in the anchor_frames_list[],
     *  it should not be replaced with other frames or removed from memory
     *  until it is not shown in the list.
     *  This number of entries of the list is given by parameter anchor_frames_num.
     */
    VASurfaceID             *anchor_frames_list;

    /** \brief Picture resolution minus 1
     *  Picture original resolution. If SuperRes is enabled,
     *  this is the upscaled resolution.
     *  value range [0..65535]
     */
    uint16_t                frame_width_minus1;
    uint16_t                frame_height_minus1;

    /** \brief Output frame buffer size in unit of tiles
     *  Valid only when large_scale_tile equals 1.
     *  value range [0..65535]
     */
    uint16_t                output_frame_width_in_tiles_minus_1;
    uint16_t                output_frame_height_in_tiles_minus_1;

    /** \brief Surface indices of reference frames in DPB.
     *
     *  Contains a list of uncompressed frame buffer surface indices as references.
     *  Application needs to make sure all the entries point to valid frames
     *  except for intra frames by checking ref_frame_id[]. If missing frame
     *  is identified, application may choose to perform error recovery by
     *  pointing problematic index to an alternative frame buffer.
     *  Driver is not responsible to validate reference frames' id.
     */
    VASurfaceID             ref_frame_map[8];

    /** \brief Reference frame indices.
     *
     *  Contains a list of indices into ref_frame_map[8].
     *  It specifies the reference frame correspondence.
     *  The indices of the array are defined as [LAST_FRAME – LAST_FRAME,
     *  LAST2_FRAME – LAST_FRAME, …, ALTREF_FRAME – LAST_FRAME], where each
     *  symbol is defined as:
     *  enum{INTRA_FRAME = 0, LAST_FRAME, LAST2_FRAME, LAST3_FRAME, GOLDEN_FRAME,
     *  BWDREF_FRAME, ALTREF2_FRAME, ALTREF_FRAME};
     */
    uint8_t                 ref_frame_idx[7];

    /** \brief primary reference frame index
     *  Index into ref_frame_idx[], specifying which reference frame contains
     *  propagated info that should be loaded at the start of the frame.
     *  When value equals PRIMARY_REF_NONE (7), it indicates there is
     *  no primary reference frame.
     *  value range [0..7]
     */
    uint8_t                 primary_ref_frame;

    uint8_t                 order_hint;

    VASegmentationStructAV1 seg_info;
    VAFilmGrainStructAV1    film_grain_info;

    /** \brief tile structure
     *  When uniform_tile_spacing_flag == 1, width_in_sbs_minus_1[] and
     *  height_in_sbs_minus_1[] should be ignored, which will be generated
     *  by driver based on tile_cols and tile_rows.
     */
    uint8_t                 tile_cols;
    uint8_t                 tile_rows;

    /* The width/height of a tile minus 1 in units of superblocks. Though the
     * maximum number of tiles is 64, since ones of the last tile are computed
     * from ones of the other tiles and frame_width/height, they are not
     * necessarily specified.
     */
    uint16_t                width_in_sbs_minus_1[63];
    uint16_t                height_in_sbs_minus_1[63];

    /** \brief number of tiles minus 1 in large scale tile list
     *  Same as AV1 semantic element.
     *  Valid only when large_scale_tiles == 1.
     */
    uint16_t                tile_count_minus_1;

    /* specify the tile index for context updating */
    uint16_t                context_update_tile_id;

    union {
        struct {
            /** \brief flags for current picture
             *  same syntax and semantic as those in AV1 code
             */

            /** \brief Frame Type
             *  0:     KEY_FRAME;
             *  1:     INTER_FRAME;
             *  2:     INTRA_ONLY_FRAME;
             *  3:     SWITCH_FRAME
             *  For SWITCH_FRAME, application shall set error_resilient_mode = 1,
             *  refresh_frame_flags, etc. appropriately. And driver will convert it
             *  to INTER_FRAME.
             */
            uint32_t        frame_type                                  : 2;
            uint32_t        show_frame                                  : 1;
            uint32_t        showable_frame                              : 1;
            uint32_t        error_resilient_mode                        : 1;
            uint32_t        disable_cdf_update                          : 1;
            uint32_t        allow_screen_content_tools                  : 1;
            uint32_t        force_integer_mv                            : 1;
            uint32_t        allow_intrabc                               : 1;
            uint32_t        use_superres                                : 1;
            uint32_t        allow_high_precision_mv                     : 1;
            uint32_t        is_motion_mode_switchable                   : 1;
            uint32_t        use_ref_frame_mvs                           : 1;
            /* disable_frame_end_update_cdf is coded as refresh_frame_context. */
            uint32_t        disable_frame_end_update_cdf                : 1;
            uint32_t        uniform_tile_spacing_flag                   : 1;
            uint32_t        allow_warped_motion                         : 1;
            /** \brief indicate if current frame in large scale tile mode */
            uint32_t        large_scale_tile                            : 1;

            /** \brief Reserved bytes for future use, must be zero */
            uint32_t        reserved                                    : 15;
        } bits;
        uint32_t            value;
    } pic_info_fields;

    /** \brief Supper resolution scale denominator.
     *  When use_superres=1, superres_scale_denominator must be in the range [9..16].
     *  When use_superres=0, superres_scale_denominator must be 8.
     */
    uint8_t                 superres_scale_denominator;

    /** \brief Interpolation filter.
     *  value range [0..4]
     */
    uint8_t                 interp_filter;

    /** \brief luma loop filter levels.
     *  value range [0..63].
     */
    uint8_t                 filter_level[2];

    /** \brief chroma loop filter levels.
     *  value range [0..63].
     */
    uint8_t                 filter_level_u;
    uint8_t                 filter_level_v;

    union {
        struct {
            /** \brief flags for reference pictures
             *  same syntax and semantic as those in AV1 code
             */
            uint8_t         sharpness_level                             : 3;
            uint8_t         mode_ref_delta_enabled                      : 1;
            uint8_t         mode_ref_delta_update                       : 1;

            /** \brief Reserved bytes for future use, must be zero */
            uint8_t         reserved                                    : 3;
        } bits;
        uint8_t             value;
    } loop_filter_info_fields;

    /** \brief The adjustment needed for the filter level based on
     *  the chosen reference frame.
     *  value range [-64..63].
     */
    int8_t                  ref_deltas[8];

    /** \brief The adjustment needed for the filter level based on
     *  the chosen mode.
     *  value range [-64..63].
     */
    int8_t                  mode_deltas[2];

    /** \brief quantization
     */
    /** \brief Y AC index
     *  value range [0..255]
     */
    uint8_t                base_qindex;
    /** \brief Y DC delta from Y AC
     *  value range [-64..63]
     */
    int8_t                  y_dc_delta_q;
    /** \brief U DC delta from Y AC
     *  value range [-64..63]
     */
    int8_t                  u_dc_delta_q;
    /** \brief U AC delta from Y AC
     *  value range [-64..63]
     */
    int8_t                  u_ac_delta_q;
    /** \brief V DC delta from Y AC
     *  value range [-64..63]
     */
    int8_t                  v_dc_delta_q;
    /** \brief V AC delta from Y AC
     *  value range [-64..63]
     */
    int8_t                  v_ac_delta_q;

    /** \brief quantization_matrix
     */
    union {
        struct {
            uint16_t        using_qmatrix                               : 1;
            /** \brief qm level
             *  value range [0..15]
             *  Invalid if using_qmatrix equals 0.
             */
            uint16_t        qm_y                                        : 4;
            uint16_t        qm_u                                        : 4;
            uint16_t        qm_v                                        : 4;

            /** \brief Reserved bytes for future use, must be zero */
            uint16_t        reserved                                    : 3;
        } bits;
        uint16_t            value;
    } qmatrix_fields;

    union {
        struct {
            /** \brief delta_q parameters
             */
            uint32_t        delta_q_present_flag                        : 1;
            uint32_t        log2_delta_q_res                            : 2;

            /** \brief delta_lf parameters
             */
            uint32_t        delta_lf_present_flag                       : 1;
            uint32_t        log2_delta_lf_res                           : 2;

            /** \brief CONFIG_LOOPFILTER_LEVEL
             */
            uint32_t        delta_lf_multi                              : 1;

            /** \brief read_tx_mode
             *  value range [0..2]
             */
            uint32_t        tx_mode                                     : 2;

            /* AV1 frame reference mode semantic */
            uint32_t        reference_select                            : 1;

            uint32_t        reduced_tx_set_used                         : 1;

            uint32_t        skip_mode_present                           : 1;

            /** \brief Reserved bytes for future use, must be zero */
            uint32_t        reserved                                    : 20;
        } bits;
        uint32_t            value;
    } mode_control_fields;

    /** \brief CDEF parameters
     */
    /*  value range [0..3]  */
    uint8_t                 cdef_damping_minus_3;
    /*  value range [0..3]  */
    uint8_t                 cdef_bits;

    /** Encode cdef strength:
     *
     * The cdef_y_strengths[] and cdef_uv_strengths[] are expected to be packed
     * with both primary and secondary strength. The secondary strength is
     * given in the lower two bits and the primary strength is given in the next
     * four bits.
     *
     * cdef_y_strengths[] & cdef_uv_strengths[] should be derived as:
     * (cdef_y_strengths[]) = (cdef_y_pri_strength[] << 2) | (cdef_y_sec_strength[] & 0x03)
     * (cdef_uv_strengths[]) = (cdef_uv_pri_strength[] << 2) | (cdef_uv_sec_strength[] & 0x03)
     * In which, cdef_y_pri_strength[]/cdef_y_sec_strength[]/cdef_uv_pri_strength[]/cdef_uv_sec_strength[]
     * are variables defined in AV1 Spec 5.9.19. The cdef_y_strengths[] & cdef_uv_strengths[]
     * are corresponding to LIBAOM variables cm->cdef_strengths[] & cm->cdef_uv_strengths[] respectively.
     */
    /*  value range [0..63]  */
    uint8_t                 cdef_y_strengths[8];
    /*  value range [0..63]  */
    uint8_t                 cdef_uv_strengths[8];

    /** \brief loop restoration parameters
     */
    union {
        struct {
            uint16_t        yframe_restoration_type                     : 2;
            uint16_t        cbframe_restoration_type                    : 2;
            uint16_t        crframe_restoration_type                    : 2;
            uint16_t        lr_unit_shift                               : 2;
            uint16_t        lr_uv_shift                                 : 1;

            /** \brief Reserved bytes for future use, must be zero */
            uint16_t        reserved                                    : 7;
        } bits;
        uint16_t            value;
    } loop_restoration_fields;

    /** \brief global motion
     */
    VAWarpedMotionParamsAV1 wm[7];

    /**@}*/

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_MEDIUM];
} VADecPictureParameterBufferAV1;


/**
 * \brief AV1 Slice Parameter Buffer Structure
 *
 * This structure conveys parameters related to bit stream data and should be
 * sent once per tile.
 *
 * It uses the name VASliceParameterBufferAV1 to be consistent with other codec,
 * but actually means VATileParameterBufferAV1.
 *
 * Slice data buffer of VASliceDataBufferType is used
 * to send the bitstream.
 *
 * Please note that host decoder is responsible to parse out the
 * per tile information. And the bit stream in sent to driver in per
 * tile granularity.
 */
typedef struct _VASliceParameterBufferAV1 {
    /**@{*/
    /** \brief The byte count of current tile in the bitstream buffer,
     *  starting from first byte of the buffer.
     *  It uses the name slice_data_size to be consistent with other codec,
     *  but actually means tile_data_size.
     */
    uint32_t                slice_data_size;
    /**
     * offset to the first byte of the data buffer.
     */
    uint32_t                slice_data_offset;
    /**
     * see VA_SLICE_DATA_FLAG_XXX definitions
     */
    uint32_t                slice_data_flag;

    uint16_t                tile_row;
    uint16_t                tile_column;

    va_deprecated uint16_t  tg_start;
    va_deprecated uint16_t  tg_end;
    /** \brief anchor frame index for large scale tile.
     *  index into an array AnchorFrames of the frames that the tile uses
     *  for prediction.
     *  valid only when large_scale_tile equals 1.
     */
    uint8_t                 anchor_frame_idx;

    /** \brief tile index in the tile list.
     *  Valid only when large_scale_tile is enabled.
     *  Driver uses this field to decide the tile output location.
     */
    uint16_t                tile_idx_in_tile_list;

    /**@}*/

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_LOW];
} VASliceParameterBufferAV1;


/**@}*/

#ifdef __cplusplus
}
#endif

#endif /* VA_DEC_AV1_H */
