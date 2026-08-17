/*
 * Copyright (c) 2007-2013 Intel Corporation. All Rights Reserved.
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
 * \file va_enc_jpeg.h
 * \brief JPEG encoding API
 *
 * This file contains the \ref api_enc_jpeg "JPEG encoding API".
 */

#ifndef VA_ENC_JPEG_H
#define VA_ENC_JPEG_H

#ifdef __cplusplus
extern "C" {
#endif

/**
 * \defgroup api_enc_jpeg JPEG encoding API
 *
 * @{
 */

/**
 * \brief JPEG Encoding Picture Parameter Buffer Structure
 *
 * This structure conveys picture level parameters.
 *
 */
typedef struct  _VAEncPictureParameterBufferJPEG {
    /** \brief holds reconstructed picture. */
    VASurfaceID reconstructed_picture;
    /** \brief picture width. */
    uint16_t picture_width;
    /** \brief picture height. */
    uint16_t picture_height;
    /** \brief holds coded data. */
    VABufferID coded_buf;

    /**
     * \brief pic_flags
     *
     */
    union {
        struct {
            /**
             * \brief profile:
             * 0 - Baseline, 1 - Extended, 2 - Lossless, 3 - Hierarchical
             */
            uint32_t profile     : 2;
            /**
             * \brief progressive:
             * 0 - sequential, 1 - extended, 2 - progressive
             */
            uint32_t progressive : 1;
            /**
             * \brief huffman:
             * 0 - arithmetic, 1 - huffman
             */
            uint32_t huffman     : 1;
            /**
             * \brief interleaved:
             * 0 - non interleaved, 1 - interleaved
             */
            uint32_t interleaved : 1;
            /**
             * \brief differential:
             * 0 - non differential, 1 - differential
             */
            uint32_t differential   : 1;
        } bits;
        uint32_t value;
    } pic_flags;

    /** \brief number of bits per sample. */
    uint8_t    sample_bit_depth;
    /** \brief total number of scans in image. */
    uint8_t    num_scan;
    /** \brief number of image components in frame. */
    uint16_t   num_components;
    /** \brief Component identifier (Ci). */
    uint8_t    component_id[4];
    /** \brief Quantization table selector (Tqi). */
    uint8_t    quantiser_table_selector[4];
    /** \brief number from 1 to 100 that specifies quality of image. */
    uint8_t    quality;

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_LOW];
} VAEncPictureParameterBufferJPEG;


/**
 * \brief Slice parameter for JPEG encoding.
 *
 * This structure conveys slice (scan) level parameters.
 *
 */
typedef struct _VAEncSliceParameterBufferJPEG {
    /** \brief Restart interval definition (Ri). */
    uint16_t    restart_interval;
    /** \brief number of image components in a scan. */
    uint16_t    num_components;
    struct {
        /** \brief Scan component selector (Csj). */
        uint8_t   component_selector;
        /** \brief DC entropy coding table selector (Tdj). */
        uint8_t   dc_table_selector;
        /** \brief AC entropy coding table selector (Taj). */
        uint8_t   ac_table_selector;
    } components[4];

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_LOW];
} VAEncSliceParameterBufferJPEG;

/**
 * \brief Quantization table for JPEG encoding.
 *
 */
typedef struct _VAQMatrixBufferJPEG {
    /** \brief load luma quantization table. */
    int32_t load_lum_quantiser_matrix;
    /** \brief load chroma quantization table. */
    int32_t load_chroma_quantiser_matrix;
    /** \brief luma quantization table. */
    uint8_t lum_quantiser_matrix[64];
    /** \brief chroma quantization table. */
    uint8_t chroma_quantiser_matrix[64];

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_LOW];
} VAQMatrixBufferJPEG;

/**@}*/

#ifdef __cplusplus
}
#endif

#endif /* VA_ENC_JPEG_H */
