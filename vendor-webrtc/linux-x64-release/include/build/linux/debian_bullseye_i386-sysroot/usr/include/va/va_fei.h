/*
 * Copyright (c) 2007-2017 Intel Corporation. All Rights Reserved.
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
 * \file va_fei.h
 * \brief The FEI encoding common API
 */

#ifndef VA_FEI_H
#define VA_FEI_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>

/**
 * \defgroup api_fei FEI encoding common API
 *
 * @{
 */

/**
 * \brief FEI specific attribute definitions
 */
/** @name Attribute values for VAConfigAttribFEIFunctionType
 *
 * This is only for VAEntrypointFEI
 * The desired type should be passed to driver when creating the configuration.
 * If VA_FEI_FUNCTION_ENC_PAK is set, VA_FEI_FUNCTION_ENC and VA_FEI_FUNCTION_PAK
 * will be ignored if set also. Combination of VA_FEI_FUNCTION_ENC and VA_FEI_FUNCTION_PAK
 * is not valid. If  VA_FEI_FUNCTION_ENC is set, there will be no bitstream output.
 * If VA_FEI_FUNCTION_PAK is set, two extra input buffers for PAK are needed:
 * VAEncFEIMVBufferType and VAEncFEIMBCodeBufferType.
 * VA_FEI_FUNCTION_ENC_PAK is recommended for best performance.
 *
 **/
/**@{*/
/** \brief ENC only is supported */
#define VA_FEI_FUNCTION_ENC                             0x00000001
/** \brief PAK only is supported */
#define VA_FEI_FUNCTION_PAK                             0x00000002
/** \brief ENC_PAK is supported */
#define VA_FEI_FUNCTION_ENC_PAK                         0x00000004

/**@}*/

/** \brief Attribute value for VAConfigAttribStats */
typedef union _VAConfigAttribValStats {
    struct {
        /** \brief Max number of past reference frames that are supported. */
        uint32_t    max_num_past_references   : 4;
        /** \brief Max number of future reference frames that are supported. */
        uint32_t    max_num_future_references : 4;
        /** \brief Number of supported output buffers for VAStatsStatisticsParameter->outputs */
        uint32_t    num_outputs               : 3;
        /** \brief Interlaced content is supported */
        uint32_t    interlaced                : 1;
        uint32_t    reserved                  : 20;
    } bits;
    uint32_t value;
} VAConfigAttribValStats;

typedef struct _VAPictureStats {
    VASurfaceID picture_id;
    /*
     * see flags below.
     */
    uint32_t flags;
} VAPictureStats;
/* flags in VAPictureStats could be one of the following */
#define VA_PICTURE_STATS_INVALID                   0x00000001
#define VA_PICTURE_STATS_PROGRESSIVE               0x00000000
#define VA_PICTURE_STATS_TOP_FIELD                 0x00000002
#define VA_PICTURE_STATS_BOTTOM_FIELD              0x00000004
/** \brief picutre surface content updated indicator.
 * The picture surface content is updated, it means temporary buffer like downscaled pixel data in driver
 * internal needs be forced freshing
 **/
#define VA_PICTURE_STATS_CONTENT_UPDATED           0x00000010

/** \brief Motion Vector and Statistics frame level controls.
 * common part VAStatsStatisticsParameterBufferType for a MB or CTB
 **/
typedef struct _VAStatsStatisticsParameter {
    /** \brief Source surface ID.  */
    VAPictureStats  input;

    /** \brief Past reference surface ID pointer.  */
    VAPictureStats  *past_references;

    /** \brief Past reference surface number  */
    uint32_t        num_past_references;

    /** \brief Statistics output for past reference surface.
     * Only enabling statistics output for past reference picture when *past_ref_stat_buf is a valid
     * VABufferID, it is needed in case app wants statistics data of both reference and current pictures
     * in very special use cases for better performance.
     * The output layout is defined by VAStatsStatisticsBufferType(for progressive and top field of
     * interlaced case) and VAStatsStatisticsBottomFieldBufferType(only for interlaced case), only
     * pixel_average_16x16/pixel_average_8x8 and variance_16x16/variance_8x8 data are valid.
     **/
    VABufferID      *past_ref_stat_buf;

    /** \brief Future reference surface ID pointer.  */
    VAPictureStats  *future_references;

    /** \brief Future reference surface number  */
    uint32_t        num_future_references;

    /** \brief Statistics output for future reference surface.
     * Only enabling statistics output for future reference picture when *past_ref_stat_buf is a valid
     * VABufferID, it is needed in case app wants statistics data of both reference and current pictures
     * in very special use cases for better performance.
     * The output layout is defined by VAStatsStatisticsBufferType(for progressive and top field of
     * interlaced case) and VAStatsStatisticsBottomFieldBufferType(only for interlaced case), only
     * pixel_average_16x16/pixel_average_8x8 and variance_16x16/variance_8x8 data are valid.
     **/
    VABufferID      *future_ref_stat_buf;

    /** \brief ID of the output buffer.
     * The number of outputs is determined by below DisableMVOutput and DisableStatisticsOutput.
     * The output layout is defined by VAStatsMVBufferType, VAStatsStatisticsBufferType(for progressive and
     * top field of interlaced case) and VAStatsStatisticsBottomFieldBufferType(only for interlaced case).
     **/
    VABufferID      *outputs;

    /** \brief MV predictor. It is valid only when mv_predictor_ctrl is not 0.
     * Each block has a pair of MVs, one for past and one for future reference
     * as defined by VAMotionVector. The block is in raster scan order.
     * Buffer size shall not be less than the number of blocks multiplied by sizeof(VAMotionVector).
     **/
    VABufferID      mv_predictor;

    /** \brief QP input buffer. It is valid only when mb_qp is set to 1.
     * The data in this buffer correspond to the input source.
     * One QP per MB or CTB block in raster scan order, each QP is a signed char (8-bit) value.
     **/
    VABufferID      qp;
} VAStatsStatisticsParameter;

#ifdef __cplusplus
}
#endif

#endif /* VA_FEI_H */
