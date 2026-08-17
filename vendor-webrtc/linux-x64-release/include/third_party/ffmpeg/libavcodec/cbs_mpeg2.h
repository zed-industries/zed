/*
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

#ifndef AVCODEC_CBS_MPEG2_H
#define AVCODEC_CBS_MPEG2_H

#include <stddef.h>
#include <stdint.h>

#include "libavutil/buffer.h"


enum {
    MPEG2_START_PICTURE         = 0x00,
    MPEG2_START_SLICE_MIN       = 0x01,
    MPEG2_START_SLICE_MAX       = 0xaf,
    MPEG2_START_USER_DATA       = 0xb2,
    MPEG2_START_SEQUENCE_HEADER = 0xb3,
    MPEG2_START_SEQUENCE_ERROR  = 0xb4,
    MPEG2_START_EXTENSION       = 0xb5,
    MPEG2_START_SEQUENCE_END    = 0xb7,
    MPEG2_START_GROUP           = 0xb8,
};

#define MPEG2_START_IS_SLICE(type) \
    ((type) >= MPEG2_START_SLICE_MIN && \
     (type) <= MPEG2_START_SLICE_MAX)

enum {
    MPEG2_EXTENSION_SEQUENCE                  = 0x1,
    MPEG2_EXTENSION_SEQUENCE_DISPLAY          = 0x2,
    MPEG2_EXTENSION_QUANT_MATRIX              = 0x3,
    MPEG2_EXTENSION_COPYRIGHT                 = 0x4,
    MPEG2_EXTENSION_SEQUENCE_SCALABLE         = 0x5,
    MPEG2_EXTENSION_PICTURE_DISPLAY           = 0x7,
    MPEG2_EXTENSION_PICTURE_CODING            = 0x8,
    MPEG2_EXTENSION_PICTURE_SPATIAL_SCALABLE  = 0x9,
    MPEG2_EXTENSION_PICTURE_TEMPORAL_SCALABLE = 0xa,
    MPEG2_EXTENSION_CAMERA_PARAMETERS         = 0xb,
    MPEG2_EXTENSION_ITU_T                     = 0xc,
};


typedef struct MPEG2RawSequenceHeader {
    uint8_t sequence_header_code;

    uint16_t horizontal_size_value;
    uint16_t vertical_size_value;
    uint8_t aspect_ratio_information;
    uint8_t frame_rate_code;
    uint32_t bit_rate_value;
    uint16_t vbv_buffer_size_value;
    uint8_t constrained_parameters_flag;

    uint8_t load_intra_quantiser_matrix;
    uint8_t intra_quantiser_matrix[64];
    uint8_t load_non_intra_quantiser_matrix;
    uint8_t non_intra_quantiser_matrix[64];
} MPEG2RawSequenceHeader;

typedef struct MPEG2RawUserData {
    uint8_t user_data_start_code;

    uint8_t     *user_data;
    AVBufferRef *user_data_ref;
    size_t       user_data_length;
} MPEG2RawUserData;

typedef struct MPEG2RawSequenceExtension {
    uint8_t profile_and_level_indication;
    uint8_t progressive_sequence;
    uint8_t chroma_format;
    uint8_t horizontal_size_extension;
    uint8_t vertical_size_extension;
    uint16_t bit_rate_extension;
    uint8_t vbv_buffer_size_extension;
    uint8_t low_delay;
    uint8_t frame_rate_extension_n;
    uint8_t frame_rate_extension_d;
} MPEG2RawSequenceExtension;

typedef struct MPEG2RawSequenceDisplayExtension {
    uint8_t video_format;

    uint8_t colour_description;
    uint8_t colour_primaries;
    uint8_t transfer_characteristics;
    uint8_t matrix_coefficients;

    uint16_t display_horizontal_size;
    uint16_t display_vertical_size;
} MPEG2RawSequenceDisplayExtension;

typedef struct MPEG2RawGroupOfPicturesHeader {
    uint8_t group_start_code;

    uint32_t time_code;
    uint8_t closed_gop;
    uint8_t broken_link;
} MPEG2RawGroupOfPicturesHeader;

typedef struct MPEG2RawExtraInformation {
    uint8_t     *extra_information;
    AVBufferRef *extra_information_ref;
    size_t       extra_information_length;
} MPEG2RawExtraInformation;

typedef struct MPEG2RawPictureHeader {
    uint8_t picture_start_code;

    uint16_t temporal_reference;
    uint8_t picture_coding_type;
    uint16_t vbv_delay;

    uint8_t full_pel_forward_vector;
    uint8_t forward_f_code;
    uint8_t full_pel_backward_vector;
    uint8_t backward_f_code;

    MPEG2RawExtraInformation extra_information_picture;
} MPEG2RawPictureHeader;

typedef struct MPEG2RawPictureCodingExtension {
    uint8_t f_code[2][2];

    uint8_t intra_dc_precision;
    uint8_t picture_structure;
    uint8_t top_field_first;
    uint8_t frame_pred_frame_dct;
    uint8_t concealment_motion_vectors;
    uint8_t q_scale_type;
    uint8_t intra_vlc_format;
    uint8_t alternate_scan;
    uint8_t repeat_first_field;
    uint8_t chroma_420_type;
    uint8_t progressive_frame;

    uint8_t composite_display_flag;
    uint8_t v_axis;
    uint8_t field_sequence;
    uint8_t sub_carrier;
    uint8_t burst_amplitude;
    uint8_t sub_carrier_phase;
} MPEG2RawPictureCodingExtension;

typedef struct MPEG2RawQuantMatrixExtension {
    uint8_t load_intra_quantiser_matrix;
    uint8_t intra_quantiser_matrix[64];
    uint8_t load_non_intra_quantiser_matrix;
    uint8_t non_intra_quantiser_matrix[64];
    uint8_t load_chroma_intra_quantiser_matrix;
    uint8_t chroma_intra_quantiser_matrix[64];
    uint8_t load_chroma_non_intra_quantiser_matrix;
    uint8_t chroma_non_intra_quantiser_matrix[64];
} MPEG2RawQuantMatrixExtension;

typedef struct MPEG2RawPictureDisplayExtension {
    int16_t frame_centre_horizontal_offset[3];
    int16_t frame_centre_vertical_offset[3];
} MPEG2RawPictureDisplayExtension;

typedef struct MPEG2RawExtensionData {
    uint8_t extension_start_code;
    uint8_t extension_start_code_identifier;

    union {
        MPEG2RawSequenceExtension sequence;
        MPEG2RawSequenceDisplayExtension sequence_display;
        MPEG2RawQuantMatrixExtension quant_matrix;
        MPEG2RawPictureCodingExtension picture_coding;
        MPEG2RawPictureDisplayExtension picture_display;
    } data;
} MPEG2RawExtensionData;

typedef struct MPEG2RawSliceHeader {
    uint8_t slice_vertical_position;

    uint8_t slice_vertical_position_extension;
    uint8_t priority_breakpoint;

    uint8_t quantiser_scale_code;

    uint8_t slice_extension_flag;
    uint8_t intra_slice;
    uint8_t slice_picture_id_enable;
    uint8_t slice_picture_id;

    MPEG2RawExtraInformation extra_information_slice;
} MPEG2RawSliceHeader;

typedef struct MPEG2RawSlice {
    MPEG2RawSliceHeader header;

    uint8_t     *data;
    AVBufferRef *data_ref;
    size_t       data_size;
    int          data_bit_start;
} MPEG2RawSlice;

typedef struct MPEG2RawSequenceEnd {
    uint8_t sequence_end_code;
} MPEG2RawSequenceEnd;


typedef struct CodedBitstreamMPEG2Context {
    // Elements stored in headers which are required for other decoding.
    uint16_t horizontal_size;
    uint16_t vertical_size;
    uint8_t scalable;
    uint8_t scalable_mode;
    uint8_t progressive_sequence;
    uint8_t number_of_frame_centre_offsets;
} CodedBitstreamMPEG2Context;


#endif /* AVCODEC_CBS_MPEG2_H */
