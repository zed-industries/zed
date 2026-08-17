/*
 * Copyright (c) 2007-2015 Intel Corporation. All Rights Reserved.
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
 * \file va_enc_vp9.h
 * \brief VP9 encoding API
 *
 * This file contains the \ref api_enc_vp9 "VP9 encoding API".
 *
 */

#ifndef VA_ENC_VP9_H
#define VA_ENC_VP9_H

#ifdef __cplusplus
extern "C" {
#endif

/**
 * \defgroup api_enc_vp9 VP9 encoding API
 *
 * @{
 */

/**
 * \brief VP9 Encoding Status Data Buffer Structure
 *
 * This structure is used to convey status data from encoder to application.
 * Driver allocates VACodedBufferVP9Status as a private data buffer.
 * Driver encapsulates the status buffer with a VACodedBufferSegment,
 * and sets VACodedBufferSegment.status to be VA_CODED_BUF_STATUS_CODEC_SPECIFIC.
 * And driver associates status data segment to the bit stream buffer segment
 * by setting VACodedBufferSegment.next of coded_buf (bit stream) to the private
 * buffer segment of status data.
 * Application accesses it by calling VAMapBuffer() with VAEncCodedBufferType.
 */
typedef struct  _VACodedBufferVP9Status {
    /** Final quantization index used (yac), determined by BRC.
     *  Application is providing quantization index deltas
     *  ydc(0), y2dc(1), y2ac(2), uvdc(3), uvac(4) that are applied to all segments
     *  and segmentation qi deltas, they will not be changed by BRC.
     */
    uint16_t    base_qp_index;

    /** Final loopfilter levels for the frame, if segmentation is disabled only
     *  index 0 is used.
     *  If loop_filter_level is 0, it indicates loop filter is disabled.
     */
    uint8_t     loop_filter_level;

    /**
     * Long term reference frame indication from BRC.  BRC recommends the
     * current frame that is being queried is a good candidate for a long
     * term reference.
     */
    uint8_t     long_term_indication;

    /* suggested next frame width */
    uint16_t    next_frame_width;

    /* suggested next frame height */
    uint16_t    next_frame_height;

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_LOW];
} VACodedBufferVP9Status;

/**
 * \brief VP9 Encoding Sequence Parameter Buffer Structure
 *
 * This structure conveys sequence level parameters.
 *
 */
typedef struct  _VAEncSequenceParameterBufferVP9 {
    /** \brief Frame size note:
     *  Picture resolution may change frame by frame.
     *  Application needs to allocate surfaces and frame buffers based on
     *  max frame resolution in case resolution changes for later frames.
     *  The source and recon surfaces allocated should be 64x64(SB) aligned
     *  on both horizontal and vertical directions.
     *  But buffers on the surfaces need to be aligned to CU boundaries.
     */
    /* maximum frame width in pixels for the whole sequence */
    uint32_t    max_frame_width;

    /* maximum frame height in pixels for the whole sequence */
    uint32_t    max_frame_height;

    /* auto keyframe placement, non-zero means enable auto keyframe placement */
    uint32_t    kf_auto;

    /* keyframe minimum interval */
    uint32_t    kf_min_dist;

    /* keyframe maximum interval */
    uint32_t    kf_max_dist;


    /* RC related fields. RC modes are set with VAConfigAttribRateControl */
    /* For VP9, CBR implies HRD conformance and VBR implies no HRD conformance */

    /**
     * Initial bitrate set for this sequence in CBR or VBR modes.
     *
     * This field represents the initial bitrate value for this
     * sequence if CBR or VBR mode is used, i.e. if the encoder
     * pipeline was created with a #VAConfigAttribRateControl
     * attribute set to either \ref VA_RC_CBR or \ref VA_RC_VBR.
     *
     * The bitrate can be modified later on through
     * #VAEncMiscParameterRateControl buffers.
     */
    uint32_t    bits_per_second;

    /* Period between key frames */
    uint32_t    intra_period;

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_LOW];
} VAEncSequenceParameterBufferVP9;


/**
 * \brief VP9 Encoding Picture Parameter Buffer Structure
 *
 * This structure conveys picture level parameters.
 *
 */
typedef struct  _VAEncPictureParameterBufferVP9 {
    /** VP9 encoder may support dynamic scaling function.
     *  If enabled (enable_dynamic_scaling is set), application may request
     *  GPU encodes picture with a different resolution from the raw source.
     *  GPU should handle the scaling process of source and
     *  all reference frames.
     */
    /* raw source frame width in pixels */
    uint32_t    frame_width_src;
    /* raw source frame height in pixels */
    uint32_t    frame_height_src;

    /* to be encoded frame width in pixels */
    uint32_t    frame_width_dst;
    /* to be encoded frame height in pixels */
    uint32_t    frame_height_dst;

    /* surface to store reconstructed frame, not used for enc only case */
    VASurfaceID reconstructed_frame;

    /** \brief reference frame buffers
     *  Each entry of the array specifies the surface index of the picture
     *  that is referred by current picture or will be referred by any future
     *  picture. The valid entries take value from 0 to 127, inclusive.
     *  Non-valid entries, those do not point to pictures which are referred
     *  by current picture or future pictures, should take value 0xFF.
     *  Other values are not allowed.
     *
     *  Application should update this array based on the refreshing
     *  information expected.
     */
    VASurfaceID reference_frames[8];

    /* buffer to store coded data */
    VABufferID  coded_buf;

    union {
        struct {
            /* force this frame to be a keyframe */
            uint32_t    force_kf                       : 1;

            /** \brief Indiates which frames to be used as reference.
             *  (Ref_frame_ctrl & 0x01) ? 1: last frame as reference frame, 0: not.
             *  (Ref_frame_ctrl & 0x02) ? 1: golden frame as reference frame, 0: not.
             *  (Ref_frame_ctrl & 0x04) ? 1: alt frame as reference frame, 0: not.
             *  L0 is for forward prediction.
             *  L1 is for backward prediction.
             */
            uint32_t    ref_frame_ctrl_l0              : 3;
            uint32_t    ref_frame_ctrl_l1              : 3;

            /** \brief Last Reference Frame index
             *  Specifies the index to RefFrameList[] which points to the LAST
             *  reference frame. It corresponds to active_ref_idx[0] in VP9 code.
             */
            uint32_t    ref_last_idx                   : 3;

            /** \brief Specifies the Sign Bias of the LAST reference frame.
             *  It corresponds to ref_frame_sign_bias[LAST_FRAME] in VP9 code.
             */
            uint32_t    ref_last_sign_bias             : 1;

            /** \brief GOLDEN Reference Frame index
             *  Specifies the index to RefFrameList[] which points to the Golden
             *  reference frame. It corresponds to active_ref_idx[1] in VP9 code.
             */
            uint32_t    ref_gf_idx                     : 3;

            /** \brief Specifies the Sign Bias of the GOLDEN reference frame.
             *  It corresponds to ref_frame_sign_bias[GOLDEN_FRAME] in VP9 code.
             */
            uint32_t    ref_gf_sign_bias               : 1;

            /** \brief Alternate Reference Frame index
             *  Specifies the index to RefFrameList[] which points to the Alternate
             *  reference frame. It corresponds to active_ref_idx[2] in VP9 code.
             */
            uint32_t    ref_arf_idx                    : 3;

            /** \brief Specifies the Sign Bias of the ALTERNATE reference frame.
             *  It corresponds to ref_frame_sign_bias[ALTREF_FRAME] in VP9 code.
             */
            uint32_t    ref_arf_sign_bias              : 1;

            /* The temporal id the frame belongs to */
            uint32_t    temporal_id                    : 8;

            uint32_t    reserved                       : 5;
        } bits;
        uint32_t value;
    } ref_flags;

    union {
        struct {
            /**
             * Indicates if the current frame is a key frame or not.
             * Corresponds to the same VP9 syntax element in frame tag.
             */
            uint32_t    frame_type                     : 1;

            /** \brief show_frame
             *  0: current frame is not for display
               *  1: current frame is for display
             */
            uint32_t    show_frame                     : 1;

            /**
             * The following fields correspond to the same VP9 syntax elements
             * in the frame header.
             */
            uint32_t    error_resilient_mode           : 1;

            /** \brief Indicate intra-only for inter pictures.
             *  Must be 0 for key frames.
             *  0: inter frame use both intra and inter blocks
             *  1: inter frame use only intra blocks.
             */
            uint32_t    intra_only                     : 1;

            /** \brief Indicate high precision mode for Motion Vector prediction
             *  0: normal mode
             *  1: high precision mode
             */
            uint32_t    allow_high_precision_mv        : 1;

            /** \brief Motion Compensation Filter type
             *  0: eight-tap  (only this mode is supported now.)
             *  1: eight-tap-smooth
             *  2: eight-tap-sharp
             *  3: bilinear
             *  4: switchable
             */
            uint32_t    mcomp_filter_type              : 3;
            uint32_t    frame_parallel_decoding_mode   : 1;
            uint32_t    reset_frame_context            : 2;
            uint32_t    refresh_frame_context          : 1;
            uint32_t    frame_context_idx              : 2;
            uint32_t    segmentation_enabled           : 1;

            /* corresponds to variable temporal_update in VP9 code.
             * Indicates whether Segment ID is from bitstream or from previous
             * frame.
             * 0: Segment ID from bitstream
             * 1: Segment ID from previous frame
             */
            uint32_t    segmentation_temporal_update   : 1;

            /* corresponds to variable update_mb_segmentation_map in VP9 code.
             * Indicates how hardware determines segmentation ID
             * 0: intra block - segment id is 0;
             *    inter block - segment id from previous frame
             * 1: intra block - segment id from bitstream (app or GPU decides)
             *    inter block - depends on segmentation_temporal_update
             */
            uint32_t    segmentation_update_map        : 1;

            /** \brief Specifies if the picture is coded in lossless mode.
             *
             *  lossless_mode = base_qindex == 0 && y_dc_delta_q == 0  \
             *                  && uv_dc_delta_q == 0 && uv_ac_delta_q == 0;
             *  Where base_qindex, y_dc_delta_q, uv_dc_delta_q and uv_ac_delta_q
             *  are all variables in VP9 code.
             *
             *  When enabled, tx_mode needs to be set to 4x4 only and all
             *  tu_size in CU record set to 4x4 for entire frame.
             *  Software also has to program such that final_qindex=0 and
             *  final_filter_level=0 following the Quant Scale and
             *  Filter Level Table in Segmentation State section.
             *  Hardware forces Hadamard Tx when this bit is set.
             *  When lossless_mode is on, BRC has to be turned off.
             *  0: normal mode
             *  1: lossless mode
             */
            uint32_t    lossless_mode                  : 1;

            /** \brief MV prediction mode. Corresponds to VP9 variable with same name.
             *  comp_prediction_mode = 0:       single prediction ony,
             *  comp_prediction_mode = 1:       compound prediction,
             *  comp_prediction_mode = 2:       hybrid prediction
             *
             *  Not mandatory. App may suggest the setting based on power or
             *  performance. Kernal may use it as a guildline and decide the proper
             *  setting on its own.
             */
            uint32_t    comp_prediction_mode           : 2;

            /** \brief Indicate how segmentation is specified
             *  0   application specifies segmentation partitioning and
             *      relevant parameters.
             *  1   GPU may decide on segmentation. If application already
             *      provides segmentation information, GPU may choose to
             *      honor it and further split into more levels if possible.
             */
            uint32_t    auto_segmentation              : 1;

            /** \brief Indicate super frame syntax should be inserted
             *  0   current frame is not encapsulated in super frame structure
             *  1   current fame is to be encapsulated in super frame structure.
             *      super frame index syntax will be inserted by encoder at
             *      the end of current frame.
             */
            uint32_t    super_frame_flag               : 1;

            uint32_t    reserved                       : 10;
        } bits;
        uint32_t    value;
    } pic_flags;

    /** \brief indicate which frames in DPB should be refreshed.
     *  same syntax and semantic as in VP9 code.
     */
    uint8_t     refresh_frame_flags;

    /** \brief Base Q index in the VP9 term.
     *  Added with per segment delta Q index to get Q index of Luma AC.
     */
    uint8_t     luma_ac_qindex;

    /**
     *  Q index delta from base Q index in the VP9 term for Luma DC.
     */
    int8_t      luma_dc_qindex_delta;

    /**
     *  Q index delta from base Q index in the VP9 term for Chroma AC.
     */
    int8_t      chroma_ac_qindex_delta;

    /**
     *  Q index delta from base Q index in the VP9 term for Chroma DC.
     */
    int8_t      chroma_dc_qindex_delta;

    /** \brief filter level
     *  Corresponds to the same VP9 syntax element in frame header.
     */
    uint8_t     filter_level;

    /**
     * Controls the deblocking filter sensitivity.
     * Corresponds to the same VP9 syntax element in frame header.
     */
    uint8_t     sharpness_level;

    /** \brief Loop filter level reference delta values.
     *  Contains a list of 4 delta values for reference frame based block-level
     *  loop filter adjustment.
     *  If no update, set to 0.
     *  value range [-63..63]
     */
    int8_t      ref_lf_delta[4];

    /** \brief Loop filter level mode delta values.
     *  Contains a list of 4 delta values for coding mode based MB-level loop
     *  filter adjustment.
     *  If no update, set to 0.
     *  value range [-63..63]
     */
    int8_t      mode_lf_delta[2];

    /**
     *  Offset from starting position of output bitstream in bits where
     *  ref_lf_delta[] should be inserted. This offset should cover any metadata
     *  ahead of uncompressed header in inserted bit stream buffer (the offset
     *  should be same as that for final output bitstream buffer).
     *
     *  In BRC mode, always insert ref_lf_delta[] (This implies uncompressed
     *  header should have mode_ref_delta_enabled=1 and mode_ref_delta_update=1).
     */
    uint16_t    bit_offset_ref_lf_delta;

    /**
     *  Offset from starting position of output bitstream in bits where
     *  mode_lf_delta[] should be inserted.
     *
     *  In BRC mode, always insert mode_lf_delta[] (This implies uncompressed
     *  header should have mode_ref_delta_enabled=1 and mode_ref_delta_update=1).
     */
    uint16_t    bit_offset_mode_lf_delta;

    /**
     *  Offset from starting position of output bitstream in bits where (loop)
     *  filter_level should be inserted.
     */
    uint16_t    bit_offset_lf_level;

    /**
     *  Offset from starting position of output bitstream in bits where
     *  Base Qindex should be inserted.
     */
    uint16_t    bit_offset_qindex;

    /**
     *  Offset from starting position of output bitstream in bits where
     *  First Partition Size should be inserted.
     */
    uint16_t    bit_offset_first_partition_size;

    /**
     *  Offset from starting position of output bitstream in bits where
     *  segmentation_enabled is located in bitstream. When auto_segmentation
     *  is enabled, GPU uses this offset to locate and update the
     *  segmentation related information.
     */
    uint16_t    bit_offset_segmentation;

    /** \brief length in bit of segmentation portion from the location
     *  in bit stream where segmentation_enabled syntax is coded.
     *  When auto_segmentation is enabled, GPU uses this bit size to locate
     *  and update the information after segmentation.
     */
    uint16_t    bit_size_segmentation;


    /** \brief log2 of number of tile rows
     *  Corresponds to the same VP9 syntax element in frame header.
     *  value range [0..2]
     */
    uint8_t     log2_tile_rows;

    /** \brief log2 of number of tile columns
     *  Corresponds to the same VP9 syntax element in frame header.
     *  value range [0..6]
     */
    uint8_t     log2_tile_columns;

    /** \brief indicate frame-skip happens
     *  Application may choose to drop/skip one or mulitple encoded frames or
     *  to-be-encoded frame due to various reasons such as insufficient
     *  bandwidth.
     *  Application uses the following three flags to inform GPU about frame-skip.
     *
     *  value range of skip_frame_flag: [0..2]
     *      0 - encode as normal, no skip;
     *      1 - one or more frames were skipped by application prior to the
     *          current frame. Encode the current frame as normal. The driver
     *          will pass the number_skip_frames and skip_frames_size
     *          to bit rate control for adjustment.
     *      2 - the current frame is to be skipped. Do not encode it but encrypt
     *          the packed header contents. This is for the secure encoding case
     *          where application generates a frame of all skipped blocks.
     *          The packed header will contain the skipped frame.
     */
    uint8_t     skip_frame_flag;

    /** \brief The number of frames skipped prior to the current frame.
     *  It includes only the skipped frames that were not counted before,
     *  and does not include the frame with skip_frame_flag == 2.
     *  Valid when skip_frame_flag = 1.
     */
    uint8_t     number_skip_frames;

    /** \brief When skip_frame_flag = 1, the size of the skipped frames in bits.
     *  It includes only the skipped frames that were not counted before,
     *  and does not include the frame size with skip_frame_flag = 2.
     *  When skip_frame_flag = 2, it is the size of the current skipped frame
     *  that is to be encrypted.
     */
    uint32_t    skip_frames_size;

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t    va_reserved[VA_PADDING_MEDIUM];
} VAEncPictureParameterBufferVP9;


/**
 * \brief Per segment parameters
 */
typedef struct _VAEncSegParamVP9 {
    union {
        struct {
            /** \brief Indicates if per segment reference frame indicator is enabled.
             *  Corresponding to variable feature_enabled when
             *  j == SEG_LVL_REF_FRAME in function setup_segmentation() VP9 code.
             */
            uint8_t     segment_reference_enabled       : 1;

            /** \brief Specifies per segment reference indication.
             *  0: reserved
             *  1: Last ref
             *  2: golden
             *  3: altref
             *  Value can be derived from variable data when
             *  j == SEG_LVL_REF_FRAME in function setup_segmentation() VP9 code.
             *  value range: [0..3]
             */
            uint8_t     segment_reference               : 2;

            /** \brief Indicates if per segment skip mode is enabled.
             *  Corresponding to variable feature_enabled when
             *  j == SEG_LVL_SKIP in function setup_segmentation() VP9 code.
             */
            uint8_t     segment_reference_skipped       : 1;

            uint8_t     reserved                        : 4;

        } bits;
        uint8_t value;
    } seg_flags;

    /** \brief Specifies per segment Loop Filter Delta.
     *  Must be 0 when segmentation_enabled == 0.
     *  value range: [-63..63]
     */
    int8_t      segment_lf_level_delta;

    /** \brief Specifies per segment QIndex Delta.
     *  Must be 0 when segmentation_enabled == 0.
     *  value range: [-255..255]
     */
    int16_t     segment_qindex_delta;

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_LOW];
} VAEncSegParamVP9;

/**
 *  Structure to convey all segment related information.
 *  If segmentation is disabled, this data structure is still required.
 *  In this case, only seg_data[0] contains valid data.
 *  This buffer is sent once per frame.
 *
 *  The buffer is created with VABufferType VAQMatrixBufferType.
 *
 */
typedef struct _VAEncMiscParameterTypeVP9PerSegmantParam {
    /**
     *  Parameters for 8 segments.
     */
    VAEncSegParamVP9    seg_data[8];

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_LOW];
} VAEncMiscParameterTypeVP9PerSegmantParam;


/**
 * \brief VP9 Block Segmentation ID Buffer
 *
 * The application provides a buffer of VAEncMacroblockMapBufferType containing
 * the initial segmentation id for each 8x8 block, one byte each, in raster scan order.
 * Rate control may reassign it.  For example, a 640x480 video, the buffer has 4800 entries.
 * The value of each entry should be in the range [0..7], inclusive.
 * If segmentation is not enabled, the application does not need to provide it.
 */


/**@}*/

#ifdef __cplusplus
}
#endif

#endif /* VA_ENC_VP9_H */
