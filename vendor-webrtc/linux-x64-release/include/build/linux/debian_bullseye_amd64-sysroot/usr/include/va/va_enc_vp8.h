/*
 * Copyright (c) 2007-2012 Intel Corporation. All Rights Reserved.
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
 * \file va_enc_vp8.h
 * \brief VP8 encoding API
 *
 * This file contains the \ref api_enc_vp8 "VP8 encoding API".
 */

#ifndef VA_ENC_VP8_H
#define VA_ENC_VP8_H

#ifdef __cplusplus
extern "C" {
#endif

/**
 * \defgroup api_enc_vp8 VP8 encoding API
 *
 * @{
 */

/**
 * \brief VP8 Encoding Sequence Parameter Buffer Structure
 *
 * This structure conveys sequence level parameters.
 *
 */
typedef struct  _VAEncSequenceParameterBufferVP8 {
    /* frame width in pixels */
    uint32_t frame_width;
    /* frame height in pixels */
    uint32_t frame_height;
    /* horizontal scale */
    uint32_t frame_width_scale;
    /* vertical scale */
    uint32_t frame_height_scale;

    /* whether to enable error resilience features */
    uint32_t error_resilient;
    /* auto keyframe placement, non-zero means enable auto keyframe placement */
    uint32_t kf_auto;
    /* keyframe minimum interval */
    uint32_t kf_min_dist;
    /* keyframe maximum interval */
    uint32_t kf_max_dist;


    /* RC related fields. RC modes are set with VAConfigAttribRateControl */
    /* For VP8, CBR implies HRD conformance and VBR implies no HRD conformance */

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
    uint32_t bits_per_second;
    /* Period between I frames. */
    uint32_t intra_period;

    /* reference and reconstructed frame buffers
     * Used for driver auto reference management when configured through
     * VAConfigAttribEncAutoReference.
     */
    VASurfaceID reference_frames[4];

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_LOW];
} VAEncSequenceParameterBufferVP8;


/**
 * \brief VP8 Encoding Picture Parameter Buffer Structure
 *
 * This structure conveys picture level parameters.
 *
 */
typedef struct  _VAEncPictureParameterBufferVP8 {
    /* surface to store reconstructed frame  */
    VASurfaceID reconstructed_frame;

    /*
     * surfaces to store reference frames in non auto reference mode
     * VA_INVALID_SURFACE can be used to denote an invalid reference frame.
     */
    VASurfaceID ref_last_frame;
    VASurfaceID ref_gf_frame;
    VASurfaceID ref_arf_frame;

    /* buffer to store coded data */
    VABufferID coded_buf;

    union {
        struct {
            /* force this frame to be a keyframe */
            uint32_t force_kf                       : 1;
            /* don't reference the last frame */
            uint32_t no_ref_last                    : 1;
            /* don't reference the golden frame */
            uint32_t no_ref_gf                      : 1;
            /* don't reference the alternate reference frame */
            uint32_t no_ref_arf                     : 1;
            /* The temporal id the frame belongs to. */
            uint32_t temporal_id                    : 8;
            /**
            *  following two flags indicate the reference order
            *  LastRef is specified by 01b;
            *  GoldRef is specified by 10b;
            *  AltRef  is specified by 11b;
            *  first_ref specifies the reference frame which is searched first.
            *  second_ref specifies the reference frame which is searched second
            *  if there is.
            */
            uint32_t first_ref                      : 2;
            uint32_t second_ref                     : 2;
            /** \brief Reserved for future use, must be zero */
            uint32_t reserved                       : 16;
        } bits;
        uint32_t value;
    } ref_flags;

    union {
        struct {
            /* version */
            uint32_t frame_type                     : 1;
            uint32_t version                        : 3;
            /* show_frame */
            uint32_t show_frame                     : 1;
            /* color_space */
            uint32_t color_space                    : 1;
            /*  0: bicubic, 1: bilinear, other: none */
            uint32_t recon_filter_type              : 2;
            /*  0: no loop fitler, 1: simple loop filter */
            uint32_t loop_filter_type               : 2;
            /* 0: disabled, 1: normal, 2: simple */
            uint32_t auto_partitions                : 1;
            /* same as log2_nbr_of_dct_partitions in frame header syntax */
            uint32_t num_token_partitions           : 2;

            /**
             * The following fields correspond to the same VP8 syntax elements
             * in the frame header.
             */
            /**
                 * 0: clamping of reconstruction pixels is disabled,
                 * 1: clamping enabled.
                 */
            uint32_t clamping_type                  : 1;
            /* indicate segmentation is enabled for the current frame. */
            uint32_t segmentation_enabled           : 1;
            /**
             * Determines if the MB segmentation map is updated in the current
             * frame.
             */
            uint32_t update_mb_segmentation_map     : 1;
            /**
             * Indicates if the segment feature data is updated in the current
             * frame.
             */
            uint32_t update_segment_feature_data    : 1;
            /**
             * indicates if the MB level loop filter adjustment is enabled for
             * the current frame (0 off, 1 on).
             */
            uint32_t loop_filter_adj_enable         : 1;
            /**
             * Determines whether updated token probabilities are used only for
             * this frame or until further update.
             * It may be used by application to enable error resilient mode.
             * In this mode probability updates are allowed only at Key Frames.
             */
            uint32_t refresh_entropy_probs          : 1;
            /**
             * Determines if the current decoded frame refreshes the golden frame.
             */
            uint32_t refresh_golden_frame           : 1;
            /**
             * Determines if the current decoded frame refreshes the alternate
             * reference frame.
             */
            uint32_t refresh_alternate_frame        : 1;
            /**
             * Determines if the current decoded frame refreshes the last frame
             * reference buffer.
             */
            uint32_t refresh_last                   : 1;
            /**
             * Determines if the golden reference is replaced by another reference.
             */
            uint32_t copy_buffer_to_golden          : 2;
            /**
             * Determines if the alternate reference is replaced by another reference.
             */
            uint32_t copy_buffer_to_alternate       : 2;
            /**
             * Controls the sign of motion vectors when the golden frame is referenced.
             */
            uint32_t sign_bias_golden               : 1;
            /**
             * Controls the sign of motion vectors when the alternate frame is
             * referenced.
             */
            uint32_t sign_bias_alternate            : 1;
            /**
             * Enables or disables the skipping of macroblocks containing no
             * non-zero coefficients.
             */
            uint32_t mb_no_coeff_skip               : 1;
            /**
             * Enforces unconditional per-MB loop filter delta update setting frame
             * header flags mode_ref_lf_delta_update, all mb_mode_delta_update_flag[4],
             * and all ref_frame_delta_update_flag[4] to 1.
            * Since loop filter deltas are not automatically refreshed to default
             * values at key frames, dropped frame with delta update may prevent
             * correct decoding from the next key frame.
            * Encoder application is advised to set this flag to 1 at key frames.
            */
            uint32_t forced_lf_adjustment           : 1;
            uint32_t reserved                       : 2;
        } bits;
        uint32_t value;
    } pic_flags;

    /**
     * Contains a list of 4 loop filter level values (updated value if applicable)
     * controlling the deblocking filter strength. Each entry represents a segment.
     * When segmentation is disabled, use entry 0.
     * When loop_filter_level is 0, loop filter shall be disabled.
     */
    int8_t loop_filter_level[4];

    /**
     * Contains a list of 4 delta values for reference frame based MB-level
     * loop filter adjustment.
     * If no update, then set to 0.
     */
    int8_t ref_lf_delta[4];

    /**
     * Contains a list of 4 delta values for coding mode based MB-level loop
     * filter adjustment.
     * If no update, then set to 0.
     */
    int8_t mode_lf_delta[4];

    /**
     * Controls the deblocking filter sensitivity.
     * Corresponds to the same VP8 syntax element in frame header.
     */
    uint8_t sharpness_level;

    /**
     * Application supplied maximum clamp value for Qindex used in quantization.
     * Qindex will not be allowed to exceed this value.
     * It has a valid range [0..127] inclusive.
     */
    uint8_t clamp_qindex_high;

    /**
     * Application supplied minimum clamp value for Qindex used in quantization.
     * Qindex will not be allowed to be lower than this value.
     * It has a valid range [0..127] inclusive.
     * Condition clamp_qindex_low <= clamp_qindex_high must be guaranteed,
     * otherwise they are ignored.
     */
    uint8_t clamp_qindex_low;

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_LOW];
} VAEncPictureParameterBufferVP8;


/**
 * \brief VP8 MB Segmentation ID Buffer
 *
 * application provides buffer containing the initial segmentation id for each
 * MB, in raster scan order. Rate control may reassign it.
 * For an 640x480 video, the buffer has 1200 entries.
 * the value of each entry should be in the range [0..3], inclusive.
 * If segmentation is not enabled, application does not need to provide it.
 */
typedef struct _VAEncMBMapBufferVP8 {
    /**
     * number of MBs in the frame.
     * It is also the number of entries of mb_segment_id[];
     */
    uint32_t num_mbs;
    /**
     * per MB Segmentation ID Buffer
     */
    uint8_t *mb_segment_id;

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_LOW];
} VAEncMBMapBufferVP8;


/**
 * \brief VP8 Quantization Matrix Buffer Structure
 *
 * Contains quantization index for yac(0-3) for each segment and quantization
 * index deltas, ydc(0), y2dc(1), y2ac(2), uvdc(3), uvac(4) that are applied
 * to all segments.  When segmentation is disabled, only quantization_index[0]
 * will be used. This structure is sent once per frame.
 */
typedef struct _VAQMatrixBufferVP8 {
    uint16_t quantization_index[4];
    int16_t quantization_index_delta[5];

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_LOW];
} VAQMatrixBufferVP8;



/**@}*/

#ifdef __cplusplus
}
#endif

#endif /* VA_ENC_VP8_H */
