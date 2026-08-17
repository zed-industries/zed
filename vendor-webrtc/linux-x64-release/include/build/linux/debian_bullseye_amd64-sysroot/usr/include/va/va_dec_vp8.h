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
 * \file va_dec_vp.h
 * \brief VP8 decoding API
 *
 * This file contains the \ref api_dec_vp8 "VP8 decoding API".
 */

#ifndef VA_DEC_VP8_H
#define VA_DEC_VP8_H

#ifdef __cplusplus
extern "C" {
#endif

/**
 * \defgroup api_dec_vp8 VP8 decoding API
 *
 * @{
 */

/**
 * \brief VPX Bool Coder Context structure
 *
 * This common structure is defined for potential sharing by other VP formats
 *
 */
typedef struct _VABoolCoderContextVPX {
    /* partition 0 "range" */
    uint8_t range;
    /* partition 0 "value" */
    uint8_t value;
    /*
     * 'partition 0 number of shifts before an output byte is available'
     * it is the number of remaining bits in 'value' for decoding, range [0, 7].
     */

    uint8_t count;
} VABoolCoderContextVPX;

/**
 * \brief VP8 Decoding Picture Parameter Buffer Structure
 *
 * This structure conveys frame level parameters and should be sent once
 * per frame.
 *
 */
typedef struct  _VAPictureParameterBufferVP8 {
    /* frame width in pixels */
    uint32_t frame_width;
    /* frame height in pixels */
    uint32_t frame_height;

    /* specifies the "last" reference frame */
    VASurfaceID last_ref_frame;
    /* specifies the "golden" reference frame */
    VASurfaceID golden_ref_frame;
    /* specifies the "alternate" referrence frame */
    VASurfaceID alt_ref_frame;
    /* specifies the out-of-loop deblocked frame, not used currently */
    VASurfaceID out_of_loop_frame;

    union {
        struct {
            /* same as key_frame in bitstream syntax, 0 means a key frame */
            uint32_t key_frame          : 1;
            /* same as version in bitstream syntax */
            uint32_t version            : 3;
            /* same as segmentation_enabled in bitstream syntax */
            uint32_t segmentation_enabled       : 1;
            /* same as update_mb_segmentation_map in bitstream syntax */
            uint32_t update_mb_segmentation_map : 1;
            /* same as update_segment_feature_data in bitstream syntax */
            uint32_t update_segment_feature_data    : 1;
            /* same as filter_type in bitstream syntax */
            uint32_t filter_type            : 1;
            /* same as sharpness_level in bitstream syntax */
            uint32_t sharpness_level        : 3;
            /* same as loop_filter_adj_enable in bitstream syntax */
            uint32_t loop_filter_adj_enable     : 1;
            /* same as mode_ref_lf_delta_update in bitstream syntax */
            uint32_t mode_ref_lf_delta_update   : 1;
            /* same as sign_bias_golden in bitstream syntax */
            uint32_t sign_bias_golden       : 1;
            /* same as sign_bias_alternate in bitstream syntax */
            uint32_t sign_bias_alternate        : 1;
            /* same as mb_no_coeff_skip in bitstream syntax */
            uint32_t mb_no_coeff_skip       : 1;
            /* flag to indicate that loop filter should be disabled */
            uint32_t loop_filter_disable        : 1;
        } bits;
        uint32_t value;
    } pic_fields;

    /*
     * probabilities of the segment_id decoding tree and same as
     * mb_segment_tree_probs in the spec.
     */
    uint8_t mb_segment_tree_probs[3];

    /* Post-adjustment loop filter levels for the 4 segments */
    uint8_t loop_filter_level[4];
    /* loop filter deltas for reference frame based MB level adjustment */
    int8_t loop_filter_deltas_ref_frame[4];
    /* loop filter deltas for coding mode based MB level adjustment */
    int8_t loop_filter_deltas_mode[4];

    /* same as prob_skip_false in bitstream syntax */
    uint8_t prob_skip_false;
    /* same as prob_intra in bitstream syntax */
    uint8_t prob_intra;
    /* same as prob_last in bitstream syntax */
    uint8_t prob_last;
    /* same as prob_gf in bitstream syntax */
    uint8_t prob_gf;

    /*
     * list of 4 probabilities of the luma intra prediction mode decoding
     * tree and same as y_mode_probs in frame header
     */
    uint8_t y_mode_probs[4];
    /*
     * list of 3 probabilities of the chroma intra prediction mode decoding
     * tree and same as uv_mode_probs in frame header
     */
    uint8_t uv_mode_probs[3];
    /*
     * updated mv decoding probabilities and same as mv_probs in
     * frame header
     */
    uint8_t mv_probs[2][19];

    VABoolCoderContextVPX bool_coder_ctx;

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_LOW];
} VAPictureParameterBufferVP8;

/**
 * \brief VP8 Slice Parameter Buffer Structure
 *
 * This structure conveys parameters related to data partitions and should be
 * sent once per frame. Slice data buffer of VASliceDataBufferType is used
 * to send the partition data.
 *
 */
typedef struct  _VASliceParameterBufferVP8 {
    /*
     * number of bytes in the slice data buffer for the partitions
     */
    uint32_t slice_data_size;
    /*
     * offset to the first byte of partition data (control partition)
     */
    uint32_t slice_data_offset;
    /*
     * see VA_SLICE_DATA_FLAG_XXX definitions
     */
    uint32_t slice_data_flag;
    /*
     * offset to the first bit of MB from the first byte of partition data(slice_data_offset)
     */
    uint32_t macroblock_offset;

    /*
     * Partitions
     * (1<<log2_nbr_of_dct_partitions)+1, count both control partition (frame header) and toke partition
     */
    uint8_t num_of_partitions;
    /*
     * partition_size[0] is remaining bytes of control partition after parsed by application.
     * exclude current byte for the remaining bits in bool_coder_ctx.
     * exclude the uncompress data chunk since first_part_size 'excluding the uncompressed data chunk'
     */
    uint32_t partition_size[9];

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_LOW];
} VASliceParameterBufferVP8;

/**
 * \brief VP8 Coefficient Probability Data Buffer Structure
 *
 * Contains the contents of the token probability table, which may be
 * incrementally modified in the frame header. There are four dimensions to
 * the token probability array. The outermost dimension is indexed by the
 * type of plane being decoded; the next dimension is selected by the
 * position of the coefficient being decoded; the third dimension, * roughly
 * speaking, measures the "local complexity" or extent to which nearby
 * coefficients are non-zero; the fourth, and final, dimension of the token
 * probability array is indexed by the position in the token tree structure,
 * as are all tree probability arrays. This structure is sent once per frame.
 *
 */
typedef struct _VAProbabilityDataBufferVP8 {
    uint8_t dct_coeff_probs[4][8][3][11];

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_LOW];
} VAProbabilityDataBufferVP8;

/**
 * \brief VP8 Inverse Quantization Matrix Buffer Structure
 *
 * Contains quantization indices for yac(0),ydc(1),y2dc(2),y2ac(3),uvdc(4),
 * uvac(5) for each segment (0-3). When segmentation is disabled, only
 * quantization_index[0][] will be used. This structure is sent once per frame.
 */
typedef struct _VAIQMatrixBufferVP8 {
    /*
     * array first dimensional is segment and 2nd dimensional is Q index
     * all Q indexs should be clipped to be range [0, 127]
     */
    uint16_t quantization_index[4][6];

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_LOW];
} VAIQMatrixBufferVP8;

/**@}*/

#ifdef __cplusplus
}
#endif

#endif /* VA_DEC_VP8_H */
