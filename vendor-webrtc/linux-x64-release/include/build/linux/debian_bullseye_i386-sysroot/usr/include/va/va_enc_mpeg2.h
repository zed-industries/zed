/*
 * Copyright (c) 2012 Intel Corporation. All Rights Reserved.
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
 * \file va_enc_mpeg2.h
 * \brief The MPEG-2 encoding API
 *
 * This file contains the \ref api_enc_mpeg2 "MPEG-2 encoding API".
 */

#ifndef _VA_ENC_MPEG2_H_
#define _VA_ENC_MPEG2_H_

#ifdef __cplusplus
extern "C" {
#endif

/**
 * \defgroup api_enc_mpeg2 MPEG-2 encoding API
 *
 * @{
 */

/**
 * \brief MPEG-2 Quantization Matrix Buffer
 *
 */
typedef VAIQMatrixBufferMPEG2 VAQMatrixBufferMPEG2;

/**
 * \brief Packed header types specific to MPEG-2 encoding.
 *
 * Types of packed headers generally used for MPEG-2 encoding.
 */
typedef enum {
    /**
     * \brief Packed Sequence Parameter Set (SPS).
     *
     */
    VAEncPackedHeaderMPEG2_SPS = VAEncPackedHeaderSequence,
    /**
     * \brief Packed Picture Parameter Set (PPS).
     *
     */
    VAEncPackedHeaderMPEG2_PPS = VAEncPackedHeaderPicture,
    /**
     * \brief Packed slice header.
     *
     */
    VAEncPackedHeaderMPEG2_Slice = VAEncPackedHeaderSlice,
} VAEncPackedHeaderTypeMPEG2;

/**
 * \brief Sequence parameter for MPEG-2 encoding
 *
 * This structure holds information for \c sequence_header() and
 * sequence_extension().
 *
 * If packed sequence headers mode is used, i.e. if the encoding
 * pipeline was configured with the #VA_ENC_PACKED_HEADER_SEQUENCE
 * flag, then the driver expects two more buffers to be provided to
 * the same \c vaRenderPicture() as this buffer:
 * - a #VAEncPackedHeaderParameterBuffer with type set to
 *   VAEncPackedHeaderType::VAEncPackedHeaderSequence ;
 * - a #VAEncPackedHeaderDataBuffer which holds the actual packed
 *   header data.
 *
 */
typedef struct _VAEncSequenceParameterBufferMPEG2 {
    /** \brief Period between I frames. */
    uint32_t intra_period;
    /** \brief Period between I/P frames. */
    uint32_t ip_period;
    /** \brief Picture width.
     *
     * A 14bits unsigned inter, the lower 12bits
     * is horizontal_size_value, and the upper
     * 2bits is \c horizontal_size_extension
     *
     */
    uint16_t picture_width;
    /** \brief Picture height.
     *
     * A 14bits unsigned inter, the lower 12bits
     * is vertical_size_value, and the upper 2bits is
     * vertical_size_size_extension
     *
     */
    uint16_t picture_height;
    /**
     * \brief Initial bitrate set for this sequence in CBR or VBR modes.
     *
     * This field represents the initial bitrate value for this
     * sequence if CBR or VBR mode is used, i.e. if the encoder
     * pipeline was created with a #VAConfigAttribRateControl
     * attribute set to either \ref VA_RC_CBR or \ref VA_RC_VBR.
     *
     * bits_per_second may be derived from bit_rate.
     *
     */
    uint32_t bits_per_second;
    /**
     * \brief Frame rate
     *
     * Derived from frame_rate_value, frame_rate_extension_n and
     * frame_rate_extension_d
     *
     */
    float frame_rate;
    /** \brief Same as the element in sequence_header() */
    uint16_t aspect_ratio_information;
    /** \brief Define the size of VBV */
    uint32_t vbv_buffer_size;

    union {
        struct {
            /** \brief Same as the element in Sequence extension() */
            uint32_t profile_and_level_indication   : 8;
            /** \brief Same as the element in Sequence extension() */
            uint32_t progressive_sequence           : 1;
            /** \brief Same as the element in Sequence extension() */
            uint32_t chroma_format                  : 2;
            /** \brief Same as the element in Sequence extension() */
            uint32_t low_delay                      : 1;
            /** \brief Same as the element in Sequence extension() */
            uint32_t frame_rate_extension_n         : 2;
            /** \brief Same as the element in Sequence extension() */
            uint32_t frame_rate_extension_d         : 5;
        } bits;
        uint32_t value;
    } sequence_extension;

    /** \brief Flag to indicate the following GOP header are being updated */
    uint32_t new_gop_header;

    union {
        struct {
            /** \brief Time code */
            uint32_t time_code                      : 25;
            /** \brief Same as the element in GOP header */
            uint32_t closed_gop                     : 1;
            /** \brief SAme as the element in GOP header */
            uint32_t broken_link                    : 1;
        } bits;
        uint32_t value;
    } gop_header;

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_LOW];
} VAEncSequenceParameterBufferMPEG2;

/**
 * \brief Picture parameter for MPEG-2 encoding
 *
 * This structure holds information for picture_header() and
 * picture_coding_extension()
 *
 * If packed picture headers mode is used, i.e. if the encoding
 * pipeline was configured with the #VA_ENC_PACKED_HEADER_PICTURE
 * flag, then the driver expects two more buffers to be provided to
 * the same \c vaRenderPicture() as this buffer:
 * - a #VAEncPackedHeaderParameterBuffer with type set to
 *   VAEncPackedHeaderType::VAEncPackedHeaderPicture ;
 * - a #VAEncPackedHeaderDataBuffer which holds the actual packed
 *   header data.
 *
 */
typedef struct _VAEncPictureParameterBufferMPEG2 {
    /** \brief Forward reference picture */
    VASurfaceID forward_reference_picture;
    /** \brief Backward reference picture */
    VASurfaceID backward_reference_picture;
    /** \brief Reconstructed(decoded) picture */
    VASurfaceID reconstructed_picture;
    /**
     * \brief Output encoded bitstream.
     *
     * \ref coded_buf has type #VAEncCodedBufferType. It should be
     * large enough to hold the compressed NAL slice and possibly SPS
     * and PPS NAL units.
     */
    VABufferID coded_buf;
    /**
     * \brief Flag to indicate the picture is the last one or not.
     *
     * This fields holds 0 if the picture to be encoded is not
     * the last one in the stream. Otherwise, it
     * is \ref MPEG2_LAST_PICTURE_EOSTREAM.
     */
    uint8_t last_picture;
    /** \brief Picture type */
    VAEncPictureType picture_type;
    /** \brief Same as the element in picture_header() */
    uint32_t temporal_reference;
    /** \brief Same as the element in picture_header() */
    uint32_t vbv_delay;
    /** \brief Same as the element in Picture coding extension */
    uint8_t f_code[2][2];
    union {
        struct {
            /** \brief Same as the element in Picture coding extension */
            uint32_t intra_dc_precision             : 2;
            /** \brief Same as the element in Picture coding extension */
            uint32_t picture_structure              : 2;
            /** \brief Same as the element in Picture coding extension */
            uint32_t top_field_first                : 1;
            /** \brief Same as the element in Picture coding extension */
            uint32_t frame_pred_frame_dct           : 1;
            /** \brief Same as the element in Picture coding extension */
            uint32_t concealment_motion_vectors     : 1;
            /** \brief Same as the element in Picture coding extension */
            uint32_t q_scale_type                   : 1;
            /** \brief Same as the element in Picture coding extension */
            uint32_t intra_vlc_format               : 1;
            /** \brief Same as the element in Picture coding extension */
            uint32_t alternate_scan                 : 1;
            /** \brief Same as the element in Picture coding extension */
            uint32_t repeat_first_field             : 1;
            /** \brief Same as the element in Picture coding extension */
            uint32_t progressive_frame              : 1;
            /** \brief Same as the element in Picture coding extension */
            uint32_t composite_display_flag         : 1;
        } bits;
        uint32_t value;
    } picture_coding_extension;

    /* \brief Parameters for composite display
     *
     * Valid only when omposite_display_flag is 1
     */
    union {
        struct {
            /** \brief Same as the element in Picture coding extension */
            uint32_t v_axis                         : 1;
            /** \brief Same as the element in Picture coding extension */
            uint32_t field_sequence                 : 3;
            /** \brief Same as the element in Picture coding extension */
            uint32_t sub_carrier                    : 1;
            /** \brief Same as the element in Picture coding extension */
            uint32_t burst_amplitude                : 7;
            /** \brief Same as the element in Picture coding extension */
            uint32_t sub_carrier_phase              : 8;
        } bits;
        uint32_t value;
    } composite_display;

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_LOW];
} VAEncPictureParameterBufferMPEG2;

/**
 * \brief Slice parameter for MPEG-2 encoding
 *
 */
typedef struct _VAEncSliceParameterBufferMPEG2 {
    /** \brief Starting MB address for this slice. */
    uint32_t macroblock_address;
    /** \brief Number of macroblocks in this slice. */
    uint32_t num_macroblocks;
    /** \brief Same as the element in slice() */
    int32_t quantiser_scale_code;
    /** \brief Flag to indicate intra slice */
    int32_t is_intra_slice;

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_LOW];
} VAEncSliceParameterBufferMPEG2;

typedef struct _VAEncMiscParameterExtensionDataSeqDisplayMPEG2 {
    /** should always be 0x02 to identify it is Sequence Display Extension ISO-13818 */
    uint8_t extension_start_code_identifier;
    /** these field should follow ISO-13818 6.3.6 */
    uint8_t video_format;
    uint8_t colour_description;
    uint8_t colour_primaries;
    uint8_t transfer_characteristics;
    uint8_t matrix_coefficients;
    uint16_t display_horizontal_size;
    uint16_t display_vertical_size;
} VAEncMiscParameterExtensionDataSeqDisplayMPEG2;
/**@}*/

#ifdef __cplusplus
}
#endif

#endif /* _VA_ENC_MPEG2_H_ */
