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
 * \file va_dec_jpeg.h
 * \brief The JPEG decoding API
 *
 * This file contains the \ref api_dec_jpeg "JPEG decoding API".
 */

#ifndef VA_DEC_JPEG_H
#define VA_DEC_JPEG_H

#ifdef __cplusplus
extern "C" {
#endif

#include <va/va.h>

/**
 * \defgroup api_dec_jpeg JPEG decoding API
 *
 * This JPEG decoding API supports Baseline profile only.
 *
 * @{
 */

/**
 * \brief Picture parameter for JPEG decoding.
 *
 * This structure holds information from the frame header, along with
 * definitions from additional segments.
 */
typedef struct _VAPictureParameterBufferJPEGBaseline {
    /** \brief Picture width in pixels. */
    uint16_t      picture_width;
    /** \brief Picture height in pixels. */
    uint16_t      picture_height;

    struct {
        /** \brief Component identifier (Ci). */
        uint8_t   component_id;
        /** \brief Horizontal sampling factor (Hi). */
        uint8_t   h_sampling_factor;
        /** \brief Vertical sampling factor (Vi). */
        uint8_t   v_sampling_factor;
        /* \brief Quantization table selector (Tqi). */
        uint8_t   quantiser_table_selector;
    }                   components[255];
    /** \brief Number of components in frame (Nf). */
    uint8_t       num_components;

    /** \brief Input color space 0: YUV, 1: RGB, 2: BGR, others: reserved */
    uint8_t       color_space;
    /** \brief Set to VA_ROTATION_* for a single rotation angle reported by VAConfigAttribDecJPEG. */
    uint32_t      rotation;
    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_MEDIUM - 1];
} VAPictureParameterBufferJPEGBaseline;

/**
 * \brief Quantization table for JPEG decoding.
 *
 * This structure holds the complete quantization tables. This is an
 * aggregation of all quantization table (DQT) segments maintained by
 * the application. i.e. up to 4 quantization tables are stored in
 * there for baseline profile.
 *
 * The #load_quantization_table array can be used as a hint to notify
 * the VA driver implementation about which table(s) actually changed
 * since the last submission of this buffer.
 *
 * The #quantiser_table values are specified in zig-zag scan order.
 */
typedef struct _VAIQMatrixBufferJPEGBaseline {
    /** \brief Specifies which #quantiser_table is valid. */
    uint8_t       load_quantiser_table[4];
    /** \brief Quanziation tables indexed by table identifier (Tqi). */
    uint8_t       quantiser_table[4][64];

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_LOW];
} VAIQMatrixBufferJPEGBaseline;

/**
 * \brief Slice parameter for JPEG decoding.
 *
 * This structure holds information from the scan header, along with
 * definitions from additional segments. The associated slice data
 * buffer holds all entropy coded segments (ECS) in the scan.
 */
typedef struct _VASliceParameterBufferJPEGBaseline {
    /** @name Codec-independent Slice Parameter Buffer base. */
    /**@{*/
    /** \brief Number of bytes in the slice data buffer for this slice. */
    uint32_t        slice_data_size;
    /** \brief The offset to the first byte of the first MCU. */
    uint32_t        slice_data_offset;
    /** \brief Slice data buffer flags. See \c VA_SLICE_DATA_FLAG_xxx. */
    uint32_t        slice_data_flag;
    /**@}*/

    /** \brief Scan horizontal position. */
    uint32_t        slice_horizontal_position;
    /** \brief Scan vertical position. */
    uint32_t        slice_vertical_position;

    struct {
        /** \brief Scan component selector (Csj). */
        uint8_t   component_selector;
        /** \brief DC entropy coding table selector (Tdj). */
        uint8_t   dc_table_selector;
        /** \brief AC entropy coding table selector (Taj). */
        uint8_t   ac_table_selector;
    }                   components[4];
    /** \brief Number of components in scan (Ns). */
    uint8_t       num_components;

    /** \brief Restart interval definition (Ri). */
    uint16_t      restart_interval;
    /** \brief Number of MCUs in a scan. */
    uint32_t        num_mcus;

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_LOW];
} VASliceParameterBufferJPEGBaseline;

/**@}*/

#ifdef __cplusplus
}
#endif

#endif /* VA_DEC_JPEG_H */
