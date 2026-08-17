/*
 * Copyright (c) 2007-2011 Intel Corporation. All Rights Reserved.
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
 * \file va_vpp.h
 * \brief The video processing API
 *
 * This file contains the \ref api_vpp "Video processing API".
 */

#ifndef VA_VPP_H
#define VA_VPP_H

#ifdef __cplusplus
extern "C" {
#endif

/**
 * \defgroup api_vpp Video processing API
 *
 * @{
 *
 * The video processing API uses the same paradigm as for decoding:
 * - Query for supported filters;
 * - Set up a video processing pipeline;
 * - Send video processing parameters through VA buffers.
 *
 * \section api_vpp_caps Query for supported filters
 *
 * Checking whether video processing is supported can be performed
 * with vaQueryConfigEntrypoints() and the profile argument set to
 * #VAProfileNone. If video processing is supported, then the list of
 * returned entry-points will include #VAEntrypointVideoProc.
 *
 * \code
 * VAEntrypoint *entrypoints;
 * int i, num_entrypoints, supportsVideoProcessing = 0;
 *
 * num_entrypoints = vaMaxNumEntrypoints();
 * entrypoints = malloc(num_entrypoints * sizeof(entrypoints[0]);
 * vaQueryConfigEntrypoints(va_dpy, VAProfileNone,
 *     entrypoints, &num_entrypoints);
 *
 * for (i = 0; !supportsVideoProcessing && i < num_entrypoints; i++) {
 *     if (entrypoints[i] == VAEntrypointVideoProc)
 *         supportsVideoProcessing = 1;
 * }
 * \endcode
 *
 * Then, the vaQueryVideoProcFilters() function is used to query the
 * list of video processing filters.
 *
 * \code
 * VAProcFilterType filters[VAProcFilterCount];
 * unsigned int num_filters = VAProcFilterCount;
 *
 * // num_filters shall be initialized to the length of the array
 * vaQueryVideoProcFilters(va_dpy, vpp_ctx, &filters, &num_filters);
 * \endcode
 *
 * Finally, individual filter capabilities can be checked with
 * vaQueryVideoProcFilterCaps().
 *
 * \code
 * VAProcFilterCap denoise_caps;
 * unsigned int num_denoise_caps = 1;
 * vaQueryVideoProcFilterCaps(va_dpy, vpp_ctx,
 *     VAProcFilterNoiseReduction,
 *     &denoise_caps, &num_denoise_caps
 * );
 *
 * VAProcFilterCapDeinterlacing deinterlacing_caps[VAProcDeinterlacingCount];
 * unsigned int num_deinterlacing_caps = VAProcDeinterlacingCount;
 * vaQueryVideoProcFilterCaps(va_dpy, vpp_ctx,
 *     VAProcFilterDeinterlacing,
 *     &deinterlacing_caps, &num_deinterlacing_caps
 * );
 * \endcode
 *
 * \section api_vpp_setup Set up a video processing pipeline
 *
 * A video processing pipeline buffer is created for each source
 * surface we want to process. However, buffers holding filter
 * parameters can be created once and for all. Rationale is to avoid
 * multiple creation/destruction chains of filter buffers and also
 * because filter parameters generally won't change frame after
 * frame. e.g. this makes it possible to implement a checkerboard of
 * videos where the same filters are applied to each video source.
 *
 * The general control flow is demonstrated by the following pseudo-code:
 * \code
 * // Create filters
 * VABufferID denoise_filter, deint_filter;
 * VABufferID filter_bufs[VAProcFilterCount];
 * unsigned int num_filter_bufs;
 *
 * for (i = 0; i < num_filters; i++) {
 *     switch (filters[i]) {
 *     case VAProcFilterNoiseReduction: {       // Noise reduction filter
 *         VAProcFilterParameterBuffer denoise;
 *         denoise.type  = VAProcFilterNoiseReduction;
 *         denoise.value = 0.5;
 *         vaCreateBuffer(va_dpy, vpp_ctx,
 *             VAProcFilterParameterBufferType, sizeof(denoise), 1,
 *             &denoise, &denoise_filter
 *         );
 *         filter_bufs[num_filter_bufs++] = denoise_filter;
 *         break;
 *     }
 *
 *     case VAProcFilterDeinterlacing:          // Motion-adaptive deinterlacing
 *         for (j = 0; j < num_deinterlacing_caps; j++) {
 *             VAProcFilterCapDeinterlacing * const cap = &deinterlacing_caps[j];
 *             if (cap->type != VAProcDeinterlacingMotionAdaptive)
 *                 continue;
 *
 *             VAProcFilterParameterBufferDeinterlacing deint;
 *             deint.type                   = VAProcFilterDeinterlacing;
 *             deint.algorithm              = VAProcDeinterlacingMotionAdaptive;
 *             vaCreateBuffer(va_dpy, vpp_ctx,
 *                 VAProcFilterParameterBufferType, sizeof(deint), 1,
 *                 &deint, &deint_filter
 *             );
 *             filter_bufs[num_filter_bufs++] = deint_filter;
 *         }
 *     }
 * }
 * \endcode
 *
 * Once the video processing pipeline is set up, the caller shall check the
 * implied capabilities and requirements with vaQueryVideoProcPipelineCaps().
 * This function can be used to validate the number of reference frames are
 * needed by the specified deinterlacing algorithm, the supported color
 * primaries, etc.
 * \code
 * // Create filters
 * VAProcPipelineCaps pipeline_caps;
 * VASurfaceID *forward_references;
 * unsigned int num_forward_references;
 * VASurfaceID *backward_references;
 * unsigned int num_backward_references;
 * VAProcColorStandardType in_color_standards[VAProcColorStandardCount];
 * VAProcColorStandardType out_color_standards[VAProcColorStandardCount];
 *
 * pipeline_caps.input_color_standards      = NULL;
 * pipeline_caps.num_input_color_standards  = ARRAY_ELEMS(in_color_standards);
 * pipeline_caps.output_color_standards     = NULL;
 * pipeline_caps.num_output_color_standards = ARRAY_ELEMS(out_color_standards);
 * vaQueryVideoProcPipelineCaps(va_dpy, vpp_ctx,
 *     filter_bufs, num_filter_bufs,
 *     &pipeline_caps
 * );
 *
 * num_forward_references  = pipeline_caps.num_forward_references;
 * forward_references      =
 *     malloc(num__forward_references * sizeof(VASurfaceID));
 * num_backward_references = pipeline_caps.num_backward_references;
 * backward_references     =
 *     malloc(num_backward_references * sizeof(VASurfaceID));
 * \endcode
 *
 * \section api_vpp_submit Send video processing parameters through VA buffers
 *
 * Video processing pipeline parameters are submitted for each source
 * surface to process. Video filter parameters can also change, per-surface.
 * e.g. the list of reference frames used for deinterlacing.
 *
 * \code
 * foreach (iteration) {
 *     vaBeginPicture(va_dpy, vpp_ctx, vpp_surface);
 *     foreach (surface) {
 *         VARectangle output_region;
 *         VABufferID pipeline_buf;
 *         VAProcPipelineParameterBuffer *pipeline_param;
 *
 *         vaCreateBuffer(va_dpy, vpp_ctx,
 *             VAProcPipelineParameterBuffer, sizeof(*pipeline_param), 1,
 *             NULL, &pipeline_buf
 *         );
 *
 *         // Setup output region for this surface
 *         // e.g. upper left corner for the first surface
 *         output_region.x     = BORDER;
 *         output_region.y     = BORDER;
 *         output_region.width =
 *             (vpp_surface_width - (Nx_surfaces + 1) * BORDER) / Nx_surfaces;
 *         output_region.height =
 *             (vpp_surface_height - (Ny_surfaces + 1) * BORDER) / Ny_surfaces;
 *
 *         vaMapBuffer(va_dpy, pipeline_buf, &pipeline_param);
 *         pipeline_param->surface              = surface;
 *         pipeline_param->surface_region       = NULL;
 *         pipeline_param->output_region        = &output_region;
 *         pipeline_param->output_background_color = 0;
 *         if (first surface to render)
 *             pipeline_param->output_background_color = 0xff000000; // black
 *         pipeline_param->filter_flags         = VA_FILTER_SCALING_HQ;
 *         pipeline_param->filters              = filter_bufs;
 *         pipeline_param->num_filters          = num_filter_bufs;
 *         vaUnmapBuffer(va_dpy, pipeline_buf);
 *
 *         // Update reference frames for deinterlacing, if necessary
 *         pipeline_param->forward_references      = forward_references;
 *         pipeline_param->num_forward_references  = num_forward_references_used;
 *         pipeline_param->backward_references     = backward_references;
 *         pipeline_param->num_backward_references = num_bacward_references_used;
 *
 *         // Apply filters
 *         vaRenderPicture(va_dpy, vpp_ctx, &pipeline_buf, 1);
 *     }
 *     vaEndPicture(va_dpy, vpp_ctx);
 * }
 * \endcode
 */

/** \brief Video filter types. */
typedef enum _VAProcFilterType {
    VAProcFilterNone = 0,
    /** \brief Noise reduction filter. */
    VAProcFilterNoiseReduction,
    /** \brief Deinterlacing filter. */
    VAProcFilterDeinterlacing,
    /** \brief Sharpening filter. */
    VAProcFilterSharpening,
    /** \brief Color balance parameters. */
    VAProcFilterColorBalance,
    /** \brief Skin Tone Enhancement. */
    VAProcFilterSkinToneEnhancement,
    /** \brief Total Color Correction. */
    VAProcFilterTotalColorCorrection,
    /** \brief Human Vision System(HVS) Noise reduction filter. */
    VAProcFilterHVSNoiseReduction,
    /** \brief High Dynamic Range Tone Mapping. */
    VAProcFilterHighDynamicRangeToneMapping,
    /** \brief Three-Dimensional Look Up Table (3DLUT). */
    VAProcFilter3DLUT,
    /** \brief Number of video filters. */
    VAProcFilterCount
} VAProcFilterType;

/** \brief Deinterlacing types. */
typedef enum _VAProcDeinterlacingType {
    VAProcDeinterlacingNone = 0,
    /** \brief Bob deinterlacing algorithm. */
    VAProcDeinterlacingBob,
    /** \brief Weave deinterlacing algorithm. */
    VAProcDeinterlacingWeave,
    /** \brief Motion adaptive deinterlacing algorithm. */
    VAProcDeinterlacingMotionAdaptive,
    /** \brief Motion compensated deinterlacing algorithm. */
    VAProcDeinterlacingMotionCompensated,
    /** \brief Number of deinterlacing algorithms. */
    VAProcDeinterlacingCount
} VAProcDeinterlacingType;

/** \brief Color balance types. */
typedef enum _VAProcColorBalanceType {
    VAProcColorBalanceNone = 0,
    /** \brief Hue. */
    VAProcColorBalanceHue,
    /** \brief Saturation. */
    VAProcColorBalanceSaturation,
    /** \brief Brightness. */
    VAProcColorBalanceBrightness,
    /** \brief Contrast. */
    VAProcColorBalanceContrast,
    /** \brief Automatically adjusted saturation. */
    VAProcColorBalanceAutoSaturation,
    /** \brief Automatically adjusted brightness. */
    VAProcColorBalanceAutoBrightness,
    /** \brief Automatically adjusted contrast. */
    VAProcColorBalanceAutoContrast,
    /** \brief Number of color balance attributes. */
    VAProcColorBalanceCount
} VAProcColorBalanceType;

/** \brief Color standard types.
 *
 * These define a set of color properties corresponding to particular
 * video standards.
 *
 * Where matrix_coefficients is specified, it applies only to YUV data -
 * RGB data always use the identity matrix (matrix_coefficients = 0).
 */
typedef enum _VAProcColorStandardType {
    VAProcColorStandardNone = 0,
    /** \brief ITU-R BT.601.
     *
     * It is unspecified whether this will use 525-line or 625-line values;
     * specify the colour primaries and matrix coefficients explicitly if
     * it is known which one is required.
     *
     * Equivalent to:
     *   colour_primaries = 5 or 6
     *   transfer_characteristics = 6
     *   matrix_coefficients = 5 or 6
     */
    VAProcColorStandardBT601,
    /** \brief ITU-R BT.709.
     *
     * Equivalent to:
     *   colour_primaries = 1
     *   transfer_characteristics = 1
     *   matrix_coefficients = 1
     */
    VAProcColorStandardBT709,
    /** \brief ITU-R BT.470-2 System M.
     *
     * Equivalent to:
     *   colour_primaries = 4
     *   transfer_characteristics = 4
     *   matrix_coefficients = 4
     */
    VAProcColorStandardBT470M,
    /** \brief ITU-R BT.470-2 System B, G.
     *
     * Equivalent to:
     *   colour_primaries = 5
     *   transfer_characteristics = 5
     *   matrix_coefficients = 5
     */
    VAProcColorStandardBT470BG,
    /** \brief SMPTE-170M.
     *
     * Equivalent to:
     *   colour_primaries = 6
     *   transfer_characteristics = 6
     *   matrix_coefficients = 6
     */
    VAProcColorStandardSMPTE170M,
    /** \brief SMPTE-240M.
     *
     * Equivalent to:
     *   colour_primaries = 7
     *   transfer_characteristics = 7
     *   matrix_coefficients = 7
     */
    VAProcColorStandardSMPTE240M,
    /** \brief Generic film.
     *
     * Equivalent to:
     *   colour_primaries = 8
     *   transfer_characteristics = 1
     *   matrix_coefficients = 1
     */
    VAProcColorStandardGenericFilm,
    /** \brief sRGB.
     *
     * Equivalent to:
     *   colour_primaries = 1
     *   transfer_characteristics = 13
     *   matrix_coefficients = 0
     */
    VAProcColorStandardSRGB,
    /** \brief stRGB.
     *
     * ???
     */
    VAProcColorStandardSTRGB,
    /** \brief xvYCC601.
     *
     * Equivalent to:
     *   colour_primaries = 1
     *   transfer_characteristics = 11
     *   matrix_coefficients = 5
     */
    VAProcColorStandardXVYCC601,
    /** \brief xvYCC709.
     *
     * Equivalent to:
     *   colour_primaries = 1
     *   transfer_characteristics = 11
     *   matrix_coefficients = 1
     */
    VAProcColorStandardXVYCC709,
    /** \brief ITU-R BT.2020.
     *
     * Equivalent to:
     *   colour_primaries = 9
     *   transfer_characteristics = 14
     *   matrix_coefficients = 9
     */
    VAProcColorStandardBT2020,
    /** \brief Explicitly specified color properties.
     *
     * Use corresponding color properties section.
     * For example, HDR10 content:
     *   colour_primaries = 9 (BT2020)
     *   transfer_characteristics = 16 (SMPTE ST2084)
     *   matrix_coefficients = 9
     */
    VAProcColorStandardExplicit,
    /** \brief Number of color standards. */
    VAProcColorStandardCount
} VAProcColorStandardType;

/** \brief Total color correction types. */
typedef enum _VAProcTotalColorCorrectionType {
    VAProcTotalColorCorrectionNone = 0,
    /** \brief Red Saturation. */
    VAProcTotalColorCorrectionRed,
    /** \brief Green Saturation. */
    VAProcTotalColorCorrectionGreen,
    /** \brief Blue Saturation. */
    VAProcTotalColorCorrectionBlue,
    /** \brief Cyan Saturation. */
    VAProcTotalColorCorrectionCyan,
    /** \brief Magenta Saturation. */
    VAProcTotalColorCorrectionMagenta,
    /** \brief Yellow Saturation. */
    VAProcTotalColorCorrectionYellow,
    /** \brief Number of color correction attributes. */
    VAProcTotalColorCorrectionCount
} VAProcTotalColorCorrectionType;

/** \brief High Dynamic Range Metadata types. */
typedef enum _VAProcHighDynamicRangeMetadataType {
    VAProcHighDynamicRangeMetadataNone = 0,
    /** \brief Metadata type for HDR10. */
    VAProcHighDynamicRangeMetadataHDR10,
    /** \brief Number of Metadata type. */
    VAProcHighDynamicRangeMetadataTypeCount
} VAProcHighDynamicRangeMetadataType;

/** \brief Video Processing Mode. */
typedef enum _VAProcMode {
    /**
     * \brief Default Mode.
     * In this mode, pipeline is decided in driver to the appropriate mode.
     * e.g. a mode that's a balance between power and performance.
     */
    VAProcDefaultMode = 0,
    /**
     * \brief Power Saving Mode.
     * In this mode, pipeline is optimized for power saving.
     */
    VAProcPowerSavingMode,
    /**
     * \brief Performance Mode.
     * In this mode, pipeline is optimized for performance.
     */
    VAProcPerformanceMode
} VAProcMode;

/** @name Video blending flags */
/**@{*/
/** \brief Global alpha blending. */
#define VA_BLEND_GLOBAL_ALPHA           0x0001
/** \brief Premultiplied alpha blending (RGBA surfaces only). */
#define VA_BLEND_PREMULTIPLIED_ALPHA    0x0002
/** \brief Luma color key (YUV surfaces only). */
#define VA_BLEND_LUMA_KEY               0x0010
/**@}*/

/** \brief Video blending state definition. */
typedef struct _VABlendState {
    /** \brief Video blending flags. */
    unsigned int        flags;
    /**
     * \brief Global alpha value.
     *
     * Valid if \flags has VA_BLEND_GLOBAL_ALPHA.
     * Valid range is 0.0 to 1.0 inclusive.
     */
    float               global_alpha;
    /**
     * \brief Minimum luma value.
     *
     * Valid if \flags has VA_BLEND_LUMA_KEY.
     * Valid range is 0.0 to 1.0 inclusive.
     * \ref min_luma shall be set to a sensible value lower than \ref max_luma.
     */
    float               min_luma;
    /**
     * \brief Maximum luma value.
     *
     * Valid if \flags has VA_BLEND_LUMA_KEY.
     * Valid range is 0.0 to 1.0 inclusive.
     * \ref max_luma shall be set to a sensible value larger than \ref min_luma.
     */
    float               max_luma;
} VABlendState;

/** @name Video pipeline flags */
/**@{*/
/** \brief Specifies whether to apply subpictures when processing a surface. */
#define VA_PROC_PIPELINE_SUBPICTURES    0x00000001
/**
 * \brief Specifies whether to apply power or performance
 * optimizations to a pipeline.
 *
 * When processing several surfaces, it may be necessary to prioritize
 * more certain pipelines than others. This flag is only a hint to the
 * video processor so that it can omit certain filters to save power
 * for example. Typically, this flag could be used with video surfaces
 * decoded from a secondary bitstream.
 */
#define VA_PROC_PIPELINE_FAST           0x00000002
/**@}*/

/** @name Video filter flags */
/**@{*/
/** \brief Specifies whether the filter shall be present in the pipeline. */
#define VA_PROC_FILTER_MANDATORY        0x00000001
/**@}*/

/** @name Pipeline end flags */
/**@{*/
/** \brief Specifies the pipeline is the last. */
#define VA_PIPELINE_FLAG_END        0x00000004
/**@}*/

/** @name Chroma Siting flag */
/**@{*/
/** vertical chroma sitting take bit 0-1, horizontal chroma sitting take bit 2-3
 * vertical chromma siting | horizontal chroma sitting to be chroma sitting */
#define VA_CHROMA_SITING_UNKNOWN              0x00
/** \brief Chroma samples are co-sited vertically on the top with the luma samples. */
#define VA_CHROMA_SITING_VERTICAL_TOP         0x01
/** \brief Chroma samples are not co-sited vertically with the luma samples. */
#define VA_CHROMA_SITING_VERTICAL_CENTER      0x02
/** \brief Chroma samples are co-sited vertically on the bottom with the luma samples. */
#define VA_CHROMA_SITING_VERTICAL_BOTTOM      0x03
/** \brief Chroma samples are co-sited horizontally on the left with the luma samples. */
#define VA_CHROMA_SITING_HORIZONTAL_LEFT      0x04
/** \brief Chroma samples are not co-sited horizontally with the luma samples. */
#define VA_CHROMA_SITING_HORIZONTAL_CENTER    0x08
/**@}*/

/**
 * This is to indicate that the color-space conversion uses full range or reduced range.
 * VA_SOURCE_RANGE_FULL(Full range): Y/Cb/Cr is in [0, 255]. It is mainly used
 *      for JPEG/JFIF formats. The combination with the BT601 flag means that
 *      JPEG/JFIF color-space conversion matrix is used.
 * VA_SOURCE_RANGE_REDUCED(Reduced range): Y is in [16, 235] and Cb/Cr is in [16, 240].
 *      It is mainly used for the YUV->RGB color-space conversion in SDTV/HDTV/UHDTV.
 */
#define VA_SOURCE_RANGE_UNKNOWN         0
#define VA_SOURCE_RANGE_REDUCED         1
#define VA_SOURCE_RANGE_FULL            2

/** @name Tone Mapping flags multiple HDR mode*/
/**@{*/
/** \brief Tone Mapping from HDR content to HDR display. */
#define VA_TONE_MAPPING_HDR_TO_HDR      0x0001
/** \brief Tone Mapping from HDR content to SDR display. */
#define VA_TONE_MAPPING_HDR_TO_SDR      0x0002
/** \brief Tone Mapping from HDR content to EDR display. */
#define VA_TONE_MAPPING_HDR_TO_EDR      0x0004
/** \brief Tone Mapping from SDR content to HDR display. */
#define VA_TONE_MAPPING_SDR_TO_HDR      0x0008
/**@}*/

/** \brief Video processing pipeline capabilities. */
typedef struct _VAProcPipelineCaps {
    /** \brief Pipeline flags. See VAProcPipelineParameterBuffer::pipeline_flags. */
    uint32_t        pipeline_flags;
    /** \brief Extra filter flags. See VAProcPipelineParameterBuffer::filter_flags. */
    uint32_t        filter_flags;
    /** \brief Number of forward reference frames that are needed. */
    uint32_t        num_forward_references;
    /** \brief Number of backward reference frames that are needed. */
    uint32_t        num_backward_references;
    /** \brief List of color standards supported on input. */
    VAProcColorStandardType *input_color_standards;
    /** \brief Number of elements in \ref input_color_standards array. */
    uint32_t        num_input_color_standards;
    /** \brief List of color standards supported on output. */
    VAProcColorStandardType *output_color_standards;
    /** \brief Number of elements in \ref output_color_standards array. */
    uint32_t        num_output_color_standards;

    /**
     * \brief Rotation flags.
     *
     * For each rotation angle supported by the underlying hardware,
     * the corresponding bit is set in \ref rotation_flags. See
     * "Rotation angles" for a description of rotation angles.
     *
     * A value of 0 means the underlying hardware does not support any
     * rotation. Otherwise, a check for a specific rotation angle can be
     * performed as follows:
     *
     * \code
     * VAProcPipelineCaps pipeline_caps;
     * ...
     * vaQueryVideoProcPipelineCaps(va_dpy, vpp_ctx,
     *     filter_bufs, num_filter_bufs,
     *     &pipeline_caps
     * );
     * ...
     * if (pipeline_caps.rotation_flags & (1 << VA_ROTATION_xxx)) {
     *     // Clockwise rotation by xxx degrees is supported
     *     ...
     * }
     * \endcode
     */
    uint32_t        rotation_flags;
    /** \brief Blend flags. See "Video blending flags". */
    uint32_t        blend_flags;
    /**
     * \brief Mirroring flags.
     *
     * For each mirroring direction supported by the underlying hardware,
     * the corresponding bit is set in \ref mirror_flags. See
     * "Mirroring directions" for a description of mirroring directions.
     *
     */
    uint32_t        mirror_flags;
    /** \brief Number of additional output surfaces supported by the pipeline  */
    uint32_t        num_additional_outputs;

    /** \brief Number of elements in \ref input_pixel_format array. */
    uint32_t        num_input_pixel_formats;
    /** \brief List of input pixel formats in fourcc. */
    uint32_t        *input_pixel_format;
    /** \brief Number of elements in \ref output_pixel_format array. */
    uint32_t        num_output_pixel_formats;
    /** \brief List of output pixel formats in fourcc. */
    uint32_t        *output_pixel_format;

    /** \brief Max supported input width in pixels. */
    uint32_t        max_input_width;
    /** \brief Max supported input height in pixels. */
    uint32_t        max_input_height;
    /** \brief Min supported input width in pixels. */
    uint32_t        min_input_width;
    /** \brief Min supported input height in pixels. */
    uint32_t        min_input_height;

    /** \brief Max supported output width in pixels. */
    uint32_t        max_output_width;
    /** \brief Max supported output height in pixels. */
    uint32_t        max_output_height;
    /** \brief Min supported output width in pixels. */
    uint32_t        min_output_width;
    /** \brief Min supported output height in pixels. */
    uint32_t        min_output_height;
    /** \brief Reserved bytes for future use, must be zero */
#if defined(__AMD64__) || defined(__x86_64__) || defined(__amd64__) || defined(__LP64__)
    uint32_t        va_reserved[VA_PADDING_HIGH - 2];
#else
    uint32_t        va_reserved[VA_PADDING_HIGH];
#endif
} VAProcPipelineCaps;

/** \brief Specification of values supported by the filter. */
typedef struct _VAProcFilterValueRange {
    /** \brief Minimum value supported, inclusive. */
    float               min_value;
    /** \brief Maximum value supported, inclusive. */
    float               max_value;
    /** \brief Default value. */
    float               default_value;
    /** \brief Step value that alters the filter behaviour in a sensible way. */
    float               step;

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t            va_reserved[VA_PADDING_LOW];
} VAProcFilterValueRange;

typedef struct _VAProcColorProperties {
    /** Chroma sample location.\c VA_CHROMA_SITING_VERTICAL_XXX | VA_CHROMA_SITING_HORIZONTAL_XXX */
    uint8_t chroma_sample_location;
    /** Color range. \c VA_SOURCE_RANGE_XXX*/
    uint8_t color_range;
    /** Colour primaries.
     *
     * See ISO/IEC 23001-8 or ITU H.273, section 8.1 and table 2.
     * Only used if the color standard in use is \c VAColorStandardExplicit.
     * Below list the typical colour primaries for the reference.
     * ---------------------------------------------------------------------------------
     * | Value | Primaries                  | Informative Remark                       |
     * --------------------------------------------------------------------------------
     * | 1     |primary  x        y         |Rec.ITU-R BT.709-5                        |
     * |       |green    0.300    0.600     |IEC 61966-2-1(sRGB or sYCC)               |
     * |       |blue     0.150    0.060     |                                          |
     * |       |red      0.640    0.330     |                                          |
     * |       |whiteD65 0.3127   0.3290    |                                          |
     * ---------------------------------------------------------------------------------
     * | 6     |primary  x        y         |Rec.ITU-R BT.601-6 525                    |
     * |       |green    0.310    0.595     |                                          |
     * |       |blue     0.155    0.070     |                                          |
     * |       |red      0.630    0.340     |                                          |
     * |       |whiteD65 0.3127   0.3290    |                                          |
     * ---------------------------------------------------------------------------------
     * | 9     |primary  x        y         |Rec.ITU-R BT.2020                         |
     * |       |green    0.170    0.797     |                                          |
     * |       |blue     0.131    0.046     |                                          |
     * |       |red      0.708    0.292     |                                          |
     * |       |whiteD65 0.3127   0.3290    |                                          |
     * ---------------------------------------------------------------------------------
     */
    uint8_t colour_primaries;
    /** Transfer characteristics.
     *
     * See ISO/IEC 23001-8 or ITU H.273, section 8.2 and table 3.
     * Only used if the color standard in use is \c VAColorStandardExplicit.
     * Below list the typical transfer characteristics for the reference.
     * -----------------------------------------------------------
     * | Value | Informative Remark                              |
     * -----------------------------------------------------------
     * | 1     |Rec.ITU-R BT.709-5                               |
     * |       |colour gamut system                              |
     * -----------------------------------------------------------
     * | 4     |Assumed display gamma 2.2                        |
     * -----------------------------------------------------------
     * | 6     |Rec.ITU-R BT.601-6 525 or 625                    |
     * -----------------------------------------------------------
     * | 8     |Linear transfer characteristics                  |
     * -----------------------------------------------------------
     * | 13    |IEC 61966-2-1(sRGB or sYCC)                      |
     * -----------------------------------------------------------
     * | 14,15 |Rec.ITU-R BT.2020                                |
     * -----------------------------------------------------------
     * | 16    |SMPTE ST 2084 for 10,12,14 and 16bit system      |
     * -----------------------------------------------------------
     */
    uint8_t transfer_characteristics;
    /** Matrix coefficients.
     *
     * See ISO/IEC 23001-8 or ITU H.273, section 8.3 and table 4.
     * Only used if the color standard in use is \c VAColorStandardExplicit.
     */
    uint8_t matrix_coefficients;
    /** Reserved bytes for future use, must be zero. */
    uint8_t reserved[3];
} VAProcColorProperties;

/** \brief Describes High Dynamic Range Meta Data for HDR10.
 *
 *  Specifies the colour volume(the colour primaries, white point and luminance range) of
 *  a display considered to be the mastering display for the associated video content -e.g.,
 *  the colour volume of a display that was used for viewing while authoring the video content.
 *  See ITU-T H.265 D.3.27 Mastering display colour volume SEI message semantics.
 *
 *  Specifies upper bounds for the nominal light level of the content. See ITU-T H.265 D.3.35
 *  Content light level information SEI message semantics.
 *
 *  This structure can be used to indicate the HDR10 metadata for 1) the content which was authored;
 *  2) the display on which the content will be presented. If it is for display, max_content_light_level
 *  and max_pic_average_light_level are ignored.
 */
typedef struct _VAHdrMetaDataHDR10 {
    /**
     * \brief X chromaticity coordinate of the mastering display.
     *
     * Index value c equal to 0 should correspond to the green primary.
     * Index value c equal to 1 should correspond to the blue primary.
     * Index value c equal to 2 should correspond to the red primary.
     * The value for display_primaries_x shall be in the range of 0 to 50000 inclusive.
     */
    uint16_t    display_primaries_x[3];
    /**
     * \brief Y chromaticity coordinate of the mastering display.
     *
     * Index value c equal to 0 should correspond to the green primary.
     * Index value c equal to 1 should correspond to the blue primary.
     * Index value c equal to 2 should correspond to the red primary.
     * The value for display_primaries_y shall be in the range of 0 to 50000 inclusive.
     */
    uint16_t    display_primaries_y[3];
    /**
     * \brief X chromaticity coordinate of the white point of the mastering display.
     *
     * The value for white_point_x shall be in the range of 0 to 50000 inclusive.
     */
    uint16_t    white_point_x;
    /**
     * \brief Y chromaticity coordinate of the white point of the mastering display.
     *
     * The value for white_point_y shall be in the range of 0 to 50000 inclusive.
     */
    uint16_t    white_point_y;
    /**
     * \brief The maximum display luminance of the mastering display.
     *
     * The value is in units of 0.0001 candelas per square metre.
     */
    uint32_t    max_display_mastering_luminance;
    /**
     * \brief The minumum display luminance of the mastering display.
     *
     * The value is in units of 0.0001 candelas per square metre.
     */
    uint32_t    min_display_mastering_luminance;
    /**
     * \brief The maximum content light level (MaxCLL).
     *
     * The value is in units of 1 candelas per square metre.
     */
    uint16_t    max_content_light_level;
    /**
     * \brief The maximum picture average light level (MaxFALL).
     *
     * The value is in units of 1 candelas per square metre.
     */
    uint16_t    max_pic_average_light_level;
    /** Resevered */
    uint16_t    reserved[VA_PADDING_HIGH];
} VAHdrMetaDataHDR10;

/** \brief Capabilities specification for the High Dynamic Range filter. */
typedef struct _VAProcFilterCapHighDynamicRange {
    /** \brief high dynamic range type. */
    VAProcHighDynamicRangeMetadataType     metadata_type;
    /**
     * \brief flag for high dynamic range tone mapping
     *
     * The flag is the combination of VA_TONE_MAPPING_XXX_TO_XXX.
     * It could be VA_TONE_MAPPING_HDR_TO_HDR | VA_TONE_MAPPING_HDR_TO_SDR.
     * SDR content to SDR display is always supported by default since it is legacy path.
     */
    uint16_t                               caps_flag;
    /** \brief Reserved bytes for future use, must be zero */
    uint16_t                               va_reserved[VA_PADDING_HIGH];
} VAProcFilterCapHighDynamicRange;

/** \brief High Dynamic Range Meta Data. */
typedef struct _VAHdrMetaData {
    /** \brief high dynamic range metadata type, HDR10 etc. */
    VAProcHighDynamicRangeMetadataType       metadata_type;
    /**
     *  \brief Pointer to high dynamic range metadata.
     *
     *  The pointer could point to VAHdrMetaDataHDR10 or other HDR meta data.
     */
    void*                                    metadata;
    /**
     *  \brief Size of high dynamic range metadata.
     */
    uint32_t                                 metadata_size;
    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                                 reserved[VA_PADDING_LOW];
} VAHdrMetaData;

/**
 * \brief Video processing pipeline configuration.
 *
 * This buffer defines a video processing pipeline. The actual filters to
 * be applied are provided in the \c filters field, they can be re-used
 * in other processing pipelines.
 *
 * The target surface is specified by the \c render_target argument of
 * \c vaBeginPicture(). The general usage model is described as follows:
 * - \c vaBeginPicture(): specify the target surface that receives the
 *   processed output;
 * - \c vaRenderPicture(): specify a surface to be processed and composed
 *   into the \c render_target. Use as many \c vaRenderPicture() calls as
 *   necessary surfaces to compose ;
 * - \c vaEndPicture(): tell the driver to start processing the surfaces
 *   with the requested filters.
 *
 * If a filter (e.g. noise reduction) needs to be applied with different
 * values for multiple surfaces, the application needs to create as many
 * filter parameter buffers as necessary. i.e. the filter parameters shall
 * not change between two calls to \c vaRenderPicture().
 *
 * For composition usage models, the first surface to process will generally
 * use an opaque background color, i.e. \c output_background_color set with
 * the most significant byte set to \c 0xff. For instance, \c 0xff000000 for
 * a black background. Then, subsequent surfaces would use a transparent
 * background color.
 */
typedef struct _VAProcPipelineParameterBuffer {
    /**
     * \brief Source surface ID.
     *
     * ID of the source surface to process. If subpictures are associated
     * with the video surfaces then they shall be rendered to the target
     * surface, if the #VA_PROC_PIPELINE_SUBPICTURES pipeline flag is set.
     */
    VASurfaceID         surface;
    /**
     * \brief Region within the source surface to be processed.
     *
     * Pointer to a #VARectangle defining the region within the source
     * surface to be processed. If NULL, \c surface_region implies the
     * whole surface.
     */
    const VARectangle  *surface_region;
    /**
     * \brief Requested input color standard.
     *
     * Color properties are implicitly converted throughout the processing
     * pipeline. The video processor chooses the best moment to apply
     * this conversion. The set of supported color standards for input shall
     * be queried with vaQueryVideoProcPipelineCaps().
     *
     * If this is set to VAProcColorStandardExplicit, the color properties
     * are specified explicitly in surface_color_properties instead.
     */
    VAProcColorStandardType surface_color_standard;
    /**
     * \brief Region within the output surface.
     *
     * Pointer to a #VARectangle defining the region within the output
     * surface that receives the processed pixels. If NULL, \c output_region
     * implies the whole surface.
     *
     * Note that any pixels residing outside the specified region will
     * be filled in with the \ref output_background_color.
     */
    const VARectangle  *output_region;
    /**
     * \brief Background color.
     *
     * Background color used to fill in pixels that reside outside of the
     * specified \ref output_region. The color is specified in ARGB format:
     * [31:24] alpha, [23:16] red, [15:8] green, [7:0] blue.
     *
     * Unless the alpha value is zero or the \ref output_region represents
     * the whole target surface size, implementations shall not render the
     * source surface to the target surface directly. Rather, in order to
     * maintain the exact semantics of \ref output_background_color, the
     * driver shall use a temporary surface and fill it in with the
     * appropriate background color. Next, the driver will blend this
     * temporary surface into the target surface.
     */
    uint32_t        output_background_color;
    /**
     * \brief Requested output color standard.
     *
     * If this is set to VAProcColorStandardExplicit, the color properties
     * are specified explicitly in output_color_properties instead.
     */
    VAProcColorStandardType output_color_standard;
    /**
     * \brief Pipeline filters. See video pipeline flags.
     *
     * Flags to control the pipeline, like whether to apply subpictures
     * or not, notify the driver that it can opt for power optimizations,
     * should this be needed.
     */
    uint32_t        pipeline_flags;
    /**
     * \brief Extra filter flags. See vaPutSurface() flags.
     *
     * Filter flags are used as a fast path, wherever possible, to use
     * vaPutSurface() flags instead of explicit filter parameter buffers.
     *
     * Allowed filter flags API-wise. Use vaQueryVideoProcPipelineCaps()
     * to check for implementation details:
     * - Bob-deinterlacing: \c VA_FRAME_PICTURE, \c VA_TOP_FIELD,
     *   \c VA_BOTTOM_FIELD. Note that any deinterlacing filter
     *   (#VAProcFilterDeinterlacing) will override those flags.
     * - Color space conversion: \c VA_SRC_BT601, \c VA_SRC_BT709,
     *   \c VA_SRC_SMPTE_240.
     * - Scaling: \c VA_FILTER_SCALING_DEFAULT, \c VA_FILTER_SCALING_FAST,
     *   \c VA_FILTER_SCALING_HQ, \c VA_FILTER_SCALING_NL_ANAMORPHIC.
     * - Interpolation Method: \c VA_FILTER_INTERPOLATION_DEFAULT,
     *   \c VA_FILTER_INTERPOLATION_NEAREST_NEIGHBOR,
     *   \c VA_FILTER_INTERPOLATION_BILINEAR, \c VA_FILTER_INTERPOLATION_ADVANCED.
     */
    uint32_t        filter_flags;
    /**
     * \brief Array of filters to apply to the surface.
     *
     * The list of filters shall be ordered in the same way the driver expects
     * them. i.e. as was returned from vaQueryVideoProcFilters().
     * Otherwise, a #VA_STATUS_ERROR_INVALID_FILTER_CHAIN is returned
     * from vaRenderPicture() with this buffer.
     *
     * #VA_STATUS_ERROR_UNSUPPORTED_FILTER is returned if the list
     * contains an unsupported filter.
     *
     */
    VABufferID         *filters;
    /** \brief Actual number of filters. */
    uint32_t           num_filters;
    /** \brief Array of forward reference frames (past frames). */
    VASurfaceID        *forward_references;
    /** \brief Number of forward reference frames that were supplied. */
    uint32_t           num_forward_references;
    /** \brief Array of backward reference frames (future frames). */
    VASurfaceID        *backward_references;
    /** \brief Number of backward reference frames that were supplied. */
    uint32_t           num_backward_references;
    /**
     * \brief Rotation state. See rotation angles.
     *
     * The rotation angle is clockwise. There is no specific rotation
     * center for this operation. Rather, The source \ref surface is
     * first rotated by the specified angle and then scaled to fit the
     * \ref output_region.
     *
     * This means that the top-left hand corner (0,0) of the output
     * (rotated) surface is expressed as follows:
     * - \ref VA_ROTATION_NONE: (0,0) is the top left corner of the
     *   source surface -- no rotation is performed ;
     * - \ref VA_ROTATION_90: (0,0) is the bottom-left corner of the
     *   source surface ;
     * - \ref VA_ROTATION_180: (0,0) is the bottom-right corner of the
     *   source surface -- the surface is flipped around the X axis ;
     * - \ref VA_ROTATION_270: (0,0) is the top-right corner of the
     *   source surface.
     *
     * Check VAProcPipelineCaps::rotation_flags first prior to
     * defining a specific rotation angle. Otherwise, the hardware can
     * perfectly ignore this variable if it does not support any
     * rotation.
     */
    uint32_t        rotation_state;
    /**
     * \brief blending state. See "Video blending state definition".
     *
     * If \ref blend_state is NULL, then default operation mode depends
     * on the source \ref surface format:
     * - RGB: per-pixel alpha blending ;
     * - YUV: no blending, i.e override the underlying pixels.
     *
     * Otherwise, \ref blend_state is a pointer to a #VABlendState
     * structure that shall be live until vaEndPicture().
     *
     * Implementation note: the driver is responsible for checking the
     * blend state flags against the actual source \ref surface format.
     * e.g. premultiplied alpha blending is only applicable to RGB
     * surfaces, and luma keying is only applicable to YUV surfaces.
     * If a mismatch occurs, then #VA_STATUS_ERROR_INVALID_BLEND_STATE
     * is returned.
     */
    const VABlendState *blend_state;
    /**
     * \bried mirroring state. See "Mirroring directions".
     *
     * Mirroring of an image can be performed either along the
     * horizontal or vertical axis. It is assumed that the rotation
     * operation is always performed before the mirroring operation.
     */
    uint32_t      mirror_state;
    /** \brief Array of additional output surfaces. */
    VASurfaceID        *additional_outputs;
    /** \brief Number of additional output surfaces. */
    uint32_t        num_additional_outputs;
    /**
     * \brief Flag to indicate the input surface flag
     *
     * bit0~3: Surface sample type
     * - 0000: Progressive --> VA_FRAME_PICTURE
     * - 0001: Single Top Field --> VA_TOP_FIELD
     * - 0010: Single Bottom Field --> VA_BOTTOM_FIELD
     * - 0100: Interleaved Top Field First --> VA_TOP_FIELD_FIRST
     * - 1000: Interleaved Bottom Field First --> VA_BOTTOM_FIELD_FIRST
     *
     * For interlaced scaling, examples as follow:
     * - 1. Interleaved to Interleaved (Suppose input is top field first)
     *   -- set input_surface_flag as VA_TOP_FIELD_FIRST
     *   -- set output_surface_flag as VA_TOP_FIELD_FIRST
     * - 2. Interleaved to Field (Suppose input is top field first)
     *   An interleaved frame need to be passed twice.
     *   First cycle to get the first field:
     *   -- set input_surface_flag as VA_TOP_FIELD_FIRST
     *   -- set output_surface_flag as VA_TOP_FIELD
     *   Second cycle to get the second field:
     *   -- set input_surface_flag as VA_TOP_FIELD_FIRST
     *   -- set output_surface_flag as VA_BOTTOM_FIELD
     * - 3. Field to Interleaved (Suppose first field is top field)
     *   -- create two surfaces, one for top field, the other for bottom field
     *   -- set surface with the first field surface id
     *   -- set backward_reference with the second field surface id
     *   -- set input_surface_flag as VA_TOP_FIELD
     *   -- set output_surface_flag as VA_TOP_FIELD_FIRST
     * - 4. Field to Field:
     *   -- set flag according to each frame.
     *
     * bit31: Surface encryption
     * - 0: non-protected
     * - 1: protected
     *
     * bit4~30 for future
     */
    uint32_t        input_surface_flag;
    /**
     * \brief Flag to indicate the output surface flag
     *
     * bit0~3: Surface sample type
     * - 0000: Progressive --> VA_FRAME_PICTURE
     * - 0001: Top Field --> VA_TOP_FIELD
     * - 0010: Bottom Field --> VA_BOTTOM_FIELD
     * - 0100: Top Field First --> VA_TOP_FIELD_FIRST
     * - 1000: Bottom Field First --> VA_BOTTOM_FIELD_FIRST
     *
     * bit31: Surface encryption
     * - 0: non-protected
     * - 1: protected
     *
     * bit4~30 for future
     */
    uint32_t        output_surface_flag;
    /**
     * \brief Input Color Properties. See "VAProcColorProperties".
     */
    VAProcColorProperties  input_color_properties;
    /**
     * \brief Output Color Properties. See "VAProcColorProperties".
     */
    VAProcColorProperties  output_color_properties;
    /**
     * \brief Processing mode. See "VAProcMode".
     */
    VAProcMode             processing_mode;
    /**
     * \brief Output High Dynamic Metadata.
     *
     * If output_metadata is NULL, then output default to SDR.
     */
    VAHdrMetaData          *output_hdr_metadata;

    /** \brief Reserved bytes for future use, must be zero */
#if defined(__AMD64__) || defined(__x86_64__) || defined(__amd64__)|| defined(__LP64__)
    uint32_t                va_reserved[VA_PADDING_LARGE - 16];
#else
    uint32_t                va_reserved[VA_PADDING_LARGE - 13];
#endif
} VAProcPipelineParameterBuffer;

/**
 * \brief Filter parameter buffer base.
 *
 * This is a helper structure used by driver implementations only.
 * Users are not supposed to allocate filter parameter buffers of this
 * type.
 */
typedef struct _VAProcFilterParameterBufferBase {
    /** \brief Filter type. */
    VAProcFilterType    type;
} VAProcFilterParameterBufferBase;

/**
 * \brief Default filter parametrization.
 *
 * Unless there is a filter-specific parameter buffer,
 * #VAProcFilterParameterBuffer is the default type to use.
 */
typedef struct _VAProcFilterParameterBuffer {
    /** \brief Filter type. */
    VAProcFilterType    type;
    /** \brief Value. */
    float               value;

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_LOW];
} VAProcFilterParameterBuffer;

/** @name De-interlacing flags */
/**@{*/
/**
 * \brief Bottom field first in the input frame.
 * if this is not set then assumes top field first.
 */
#define VA_DEINTERLACING_BOTTOM_FIELD_FIRST 0x0001
/**
 * \brief Bottom field used in deinterlacing.
 * if this is not set then assumes top field is used.
 */
#define VA_DEINTERLACING_BOTTOM_FIELD       0x0002
/**
 * \brief A single field is stored in the input frame.
 * if this is not set then assumes the frame contains two interleaved fields.
 */
#define VA_DEINTERLACING_ONE_FIELD      0x0004
/**
 * \brief Film Mode Detection is enabled. If enabled, driver performs inverse
 * of various pulldowns, such as 3:2 pulldown.
 * if this is not set then assumes FMD is disabled.
 */
#define VA_DEINTERLACING_FMD_ENABLE     0x0008

//Scene change parameter for ADI on Linux, if enabled, driver use spatial DI(Bob), instead of ADI. if not, use old behavior for ADI
//Input stream is TFF(set flags = 0), SRC0,1,2,3 are interlaced frame (top +bottom fields), DSTs are progressive frames
//30i->30p
//SRC0 -> BOBDI,  no reference, set flag = 0, output DST0
//SRC1 -> ADI, reference frame=SRC0, set flags = 0, call VP, output DST1
//SRC2 -> ADI, reference frame=SRC1, set flags = 0x0010(decimal 16), call VP, output DST2(T4)
//SRC3 -> ADI, reference frame=SRC2, set flags = 0, call VP, output DST3
//30i->60p
//SRC0 -> BOBDI, no reference, set flag = 0, output DST0
//SRC0 -> BOBDI, no reference, set flag =0x0002, output DST1

//SRC1 -> ADI, reference frame =SRC0, set flags = 0, call VP, output DST2
//SRC1 -> ADI, reference frame =SRC0, set flags = 0x0012(decimal18), call VP, output DST3(B3)

//SRC2 -> ADI, reference frame =SRC1, set flags =  0x0010(decimal 16), call VP, output DST4(T4)
//SRC2 -> ADI, reference frame =SRC1, set flags =  0x0002, call VP, output DST5

//SRC3 -> ADI, reference frame =SRC2, set flags =  0, call VP, output DST6
//SRC3 -> ADI, reference frame =SRC1, set flags = 0x0002, call VP, output DST7

#define VA_DEINTERLACING_SCD_ENABLE     0x0010

/**@}*/

/** \brief Deinterlacing filter parametrization. */
typedef struct _VAProcFilterParameterBufferDeinterlacing {
    /** \brief Filter type. Shall be set to #VAProcFilterDeinterlacing. */
    VAProcFilterType            type;
    /** \brief Deinterlacing algorithm. */
    VAProcDeinterlacingType     algorithm;
    /** \brief Deinterlacing flags. */
    uint32_t            flags;

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_LOW];
} VAProcFilterParameterBufferDeinterlacing;

/**
 * \brief Color balance filter parametrization.
 *
 * This buffer defines color balance attributes. A VA buffer can hold
 * several color balance attributes by creating a VA buffer of desired
 * number of elements. This can be achieved by the following pseudo-code:
 *
 * \code
 * enum { kHue, kSaturation, kBrightness, kContrast };
 *
 * // Initial color balance parameters
 * static const VAProcFilterParameterBufferColorBalance colorBalanceParams[4] =
 * {
 *     [kHue] =
 *         { VAProcFilterColorBalance, VAProcColorBalanceHue, 0.5 },
 *     [kSaturation] =
 *         { VAProcFilterColorBalance, VAProcColorBalanceSaturation, 0.5 },
 *     [kBrightness] =
 *         { VAProcFilterColorBalance, VAProcColorBalanceBrightness, 0.5 },
 *     [kSaturation] =
 *         { VAProcFilterColorBalance, VAProcColorBalanceSaturation, 0.5 }
 * };
 *
 * // Create buffer
 * VABufferID colorBalanceBuffer;
 * vaCreateBuffer(va_dpy, vpp_ctx,
 *     VAProcFilterParameterBufferType, sizeof(*pColorBalanceParam), 4,
 *     colorBalanceParams,
 *     &colorBalanceBuffer
 * );
 *
 * VAProcFilterParameterBufferColorBalance *pColorBalanceParam;
 * vaMapBuffer(va_dpy, colorBalanceBuffer, &pColorBalanceParam);
 * {
 *     // Change brightness only
 *     pColorBalanceBuffer[kBrightness].value = 0.75;
 * }
 * vaUnmapBuffer(va_dpy, colorBalanceBuffer);
 * \endcode
 */
typedef struct _VAProcFilterParameterBufferColorBalance {
    /** \brief Filter type. Shall be set to #VAProcFilterColorBalance. */
    VAProcFilterType            type;
    /** \brief Color balance attribute. */
    VAProcColorBalanceType      attrib;
    /**
     * \brief Color balance value.
     *
     * Special case for automatically adjusted attributes. e.g.
     * #VAProcColorBalanceAutoSaturation,
     * #VAProcColorBalanceAutoBrightness,
     * #VAProcColorBalanceAutoContrast.
     * - If \ref value is \c 1.0 +/- \c FLT_EPSILON, the attribute is
     *   automatically adjusted and overrides any other attribute of
     *   the same type that would have been set explicitly;
     * - If \ref value is \c 0.0 +/- \c FLT_EPSILON, the attribute is
     *   disabled and other attribute of the same type is used instead.
     */
    float                       value;

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_LOW];
} VAProcFilterParameterBufferColorBalance;

/** \brief Total color correction filter parametrization. */
typedef struct _VAProcFilterParameterBufferTotalColorCorrection {
    /** \brief Filter type. Shall be set to #VAProcFilterTotalColorCorrection. */
    VAProcFilterType                  type;
    /** \brief Color to correct. */
    VAProcTotalColorCorrectionType    attrib;
    /** \brief Color correction value. */
    float                             value;
} VAProcFilterParameterBufferTotalColorCorrection;

/** @name Video Processing Human Vision System (HVS) Denoise Mode.*/
/**@{*/
/**
 *  \brief Default Mode.
 *  This mode is decided in driver to the appropriate mode.
 */
#define VA_PROC_HVS_DENOISE_DEFAULT               0x0000
/**
 *  \brief Auto BDRate Mode.
 *  Indicates auto BD rate improvement in pre-processing (such as before video encoding), ignore Strength.
 */
#define VA_PROC_HVS_DENOISE_AUTO_BDRATE           0x0001
/**
 *  \brief Auto Subjective Mode.
 *  Indicates auto subjective quality improvement in pre-processing (such as before video encoding), ignore Strength.
 */
#define VA_PROC_HVS_DENOISE_AUTO_SUBJECTIVE       0x0002
/**
 *  \brief Manual Mode.
 *  Indicates manual mode, allow to adjust the denoise strength manually (need to set Strength explicitly).
 */
#define VA_PROC_HVS_DENOISE_MANUAL                0x0003
/**@}*/

/** \brief Human Vision System(HVS) Noise reduction filter parametrization. */
typedef struct _VAProcFilterParameterBufferHVSNoiseReduction {
    /** \brief Filter type. Shall be set to #VAProcFilterHVSNoiseReduction. */
    VAProcFilterType    type;
    /** \brief QP for encoding, used for HVS Denoise */
    uint16_t            qp;
    /**
     *  \brief QP to Noise Reduction Strength Mode, used for Human Vision System Based Noise Reduction.
     *  Controls Noise Reduction strength of conservative and aggressive mode.
     *  It is an integer from [0-16].
     *  Value 0 means completely turn off Noise Reduction;
     *  Value 16 means the most aggressive mode of Noise Reduction;
     *  Value 10 is the default value.
     */
    uint16_t            strength;
    /**
     *  \brief HVS Denoise Mode which controls denoise method.
     *  It is a value of VA_PROC_HVS_DENOISE_xxx.
     *  Please see the definition of VA_PROC_HVS_DENOISE_xxx.
     */
    uint16_t            mode;
    /** \brief Reserved bytes for future use, must be zero */
    uint16_t            va_reserved[VA_PADDING_HIGH - 1];
} VAProcFilterParameterBufferHVSNoiseReduction;

/** \brief High Dynamic Range(HDR) Tone Mapping filter parametrization. */
typedef struct _VAProcFilterParameterBufferHDRToneMapping {
    /** \brief Filter type. Shall be set to #VAProcFilterHighDynamicRangeToneMapping.*/
    VAProcFilterType    type;
    /**
     *  \brief High Dynamic Range metadata, could be HDR10 etc.
     *
     *  This metadata is mainly for the input surface. Given that dynamic metadata is changing
     *  on frame-by-frame or scene-by-scene basis for HDR10 plus, differentiate the metadata
     *  for the input and output.
     */
    VAHdrMetaData       data;
    /** \brief Reserved bytes for future use, must be zero */
    uint32_t            va_reserved[VA_PADDING_HIGH];
} VAProcFilterParameterBufferHDRToneMapping;

/** @name 3DLUT Channel Layout and Mapping */
/**@{*/
/** \brief 3DLUT Channel Layout is unknown. */
#define VA_3DLUT_CHANNEL_UNKNOWN              0x00000000
/** \brief 3DLUT Channel Layout is R, G, B, the default layout. Map RGB to RGB. */
#define VA_3DLUT_CHANNEL_RGB_RGB              0x00000001
/** \brief 3DLUT Channel Layout is Y, U, V. Map YUV to RGB. */
#define VA_3DLUT_CHANNEL_YUV_RGB              0x00000002
/** \brief 3DLUT Channel Layout is V, U, Y. Map VUY to RGB. */
#define VA_3DLUT_CHANNEL_VUY_RGB              0x00000004
/**@}*/

/**
  *  \brief 3DLUT filter parametrization.
  *
  *  3DLUT (Three Dimensional Look Up Table) is often used when converting an image or a video frame
  *  from one color representation to another, for example, when converting log and gamma encodings,
  *  changing the color space, applying a color correction, changing the dynamic range, gamut mapping etc.
  *
  *  This buffer defines 3DLUT attributes and memory layout. The typical 3DLUT has fixed number(lut_size)
  *  per dimension and memory layout is 3 dimensional array as 3dlut[stride_0][stride_1][stride_2] (lut_size
  *  shall be smaller than stride_0/1/2).
  *
  *  API user should query hardware capability by using the VAProcFilterCap3DLUT to get the 3DLUT attributes
  *  which hardware supports, and use these attributes. For example, if the user queries hardware, the API user
  *  could get caps with 3dlut[33][33][64] (lut_size = 33, lut_stride[0/1/2] = 33/33/64). API user shall not
  *  use the attributes which hardware can not support.
  *
  *  3DLUT is usually used to transform input RGB/YUV values in one color space to output RGB values in another
  *  color space. Based on 1) the format and color space of VPP input and output and 2) 3DLUT memory layout and
  *  channel mapping, driver will enable some color space conversion implicitly if needed. For example, the input of
  *  VPP is P010 format in BT2020 color space, the output of VPP is NV12 in BT709 color space and the 3DLUT channel
  *  mapping is VA_3DLUT_CHANNEL_RGB_RGB, driver could build the data pipeline as P010(BT2020)->RGB(BT2020)
  *  ->3DULT(BT709)->NV12(BT709). Please note, the limitation of 3DLUT filter color space is that the color space of
  *  3DLUT filter input data needs to be same as the input data of VPP; the color space of 3DLUT filter output data
  *  needs to be same as the output data of VPP; format does not have such limitation.
  */
typedef struct _VAProcFilterParameterBuffer3DLUT {
    /** \brief Filter type. Shall be set to #VAProcFilter3DLUT.*/
    VAProcFilterType    type;

    /** \brief lut_surface contains 3DLUT data in the 3DLUT memory layout, must be linear */
    VASurfaceID         lut_surface;
    /**
      * \brief lut_size is the number of valid points on every dimension of the three dimensional look up table.
      * The size of LUT (lut_size) shall be same among every dimension of the three dimensional look up table.
      * The size of LUT (lut_size) shall be smaller than lut_stride[0/1/2].
      */
    uint16_t            lut_size;
    /**
      *  \brief lut_stride are the number of points on every dimension of the three dimensional look up table.
      *  Three dimension can has 3 different stride, lut3d[lut_stride[0]][lut_stride[1]][lut_stride[2]].
      *  But the valid point shall start from 0, the range of valid point is [0, lut_size-1] for every dimension.
      */
    uint16_t            lut_stride[3];
    /** \brief bit_depth is the number of bits for every channel R, G or B (or Y, U, V) */
    uint16_t            bit_depth;
    /** \brief num_channel is the number of channels */
    uint16_t            num_channel;

    /** \brief channel_mapping defines the mapping of input and output channels, could be one of VA_3DLUT_CHANNEL_XXX*/
    uint32_t            channel_mapping;

    /** \brief reserved bytes for future use, must be zero */
    uint32_t            va_reserved[VA_PADDING_HIGH];
} VAProcFilterParameterBuffer3DLUT;

/** \brief Capabilities specification for the 3DLUT filter. */
typedef struct _VAProcFilterCap3DLUT {
    /** \brief lut_size is the number of valid points on every dimension of the three dimensional look up table. */
    uint16_t            lut_size;
    /**  \brief lut_stride are the number of points on every dimension of the three dimensional look up table. lut3d[lut_stride[0]][lut_stride[1]][lut_stride[2]]*/
    uint16_t            lut_stride[3];
    /** \brief bit_depth is the number of bits for every channel R, G or B (or Y, U, V) */
    uint16_t            bit_depth;
    /** \brief num_channel is the number of channels */
    uint16_t            num_channel;
    /** \brief channel_mapping defines the mapping of channels, could be some combination of VA_3DLUT_CHANNEL_XXX*/
    uint32_t            channel_mapping;

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t            va_reserved[VA_PADDING_HIGH];
} VAProcFilterCap3DLUT;

/**
 * \brief Default filter cap specification (single range value).
 *
 * Unless there is a filter-specific cap structure, #VAProcFilterCap is the
 * default type to use for output caps from vaQueryVideoProcFilterCaps().
 */
typedef struct _VAProcFilterCap {
    /** \brief Range of supported values for the filter. */
    VAProcFilterValueRange      range;

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_LOW];
} VAProcFilterCap;

/** \brief Capabilities specification for the deinterlacing filter. */
typedef struct _VAProcFilterCapDeinterlacing {
    /** \brief Deinterlacing algorithm. */
    VAProcDeinterlacingType     type;

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_LOW];
} VAProcFilterCapDeinterlacing;

/** \brief Capabilities specification for the color balance filter. */
typedef struct _VAProcFilterCapColorBalance {
    /** \brief Color balance operation. */
    VAProcColorBalanceType      type;
    /** \brief Range of supported values for the specified operation. */
    VAProcFilterValueRange      range;

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_LOW];
} VAProcFilterCapColorBalance;

/** \brief Capabilities specification for the Total Color Correction filter. */
typedef struct _VAProcFilterCapTotalColorCorrection {
    /** \brief Color to correct. */
    VAProcTotalColorCorrectionType    type;
    /** \brief Range of supported values for the specified color. */
    VAProcFilterValueRange            range;
} VAProcFilterCapTotalColorCorrection;

/**
 * \brief Queries video processing filters.
 *
 * This function returns the list of video processing filters supported
 * by the driver. The \c filters array is allocated by the user and
 * \c num_filters shall be initialized to the number of allocated
 * elements in that array. Upon successful return, the actual number
 * of filters will be overwritten into \c num_filters. Otherwise,
 * \c VA_STATUS_ERROR_MAX_NUM_EXCEEDED is returned and \c num_filters
 * is adjusted to the number of elements that would be returned if enough
 * space was available.
 *
 * The list of video processing filters supported by the driver shall
 * be ordered in the way they can be iteratively applied. This is needed
 * for both correctness, i.e. some filters would not mean anything if
 * applied at the beginning of the pipeline; but also for performance
 * since some filters can be applied in a single pass (e.g. noise
 * reduction + deinterlacing).
 *
 * @param[in] dpy               the VA display
 * @param[in] context           the video processing context
 * @param[out] filters          the output array of #VAProcFilterType elements
 * @param[in,out] num_filters the number of elements allocated on input,
 *      the number of elements actually filled in on output
 */
VAStatus
vaQueryVideoProcFilters(
    VADisplay           dpy,
    VAContextID         context,
    VAProcFilterType   *filters,
    unsigned int       *num_filters
);

/**
 * \brief Queries video filter capabilities.
 *
 * This function returns the list of capabilities supported by the driver
 * for a specific video filter. The \c filter_caps array is allocated by
 * the user and \c num_filter_caps shall be initialized to the number
 * of allocated elements in that array. Upon successful return, the
 * actual number of filters will be overwritten into \c num_filter_caps.
 * Otherwise, \c VA_STATUS_ERROR_MAX_NUM_EXCEEDED is returned and
 * \c num_filter_caps is adjusted to the number of elements that would be
 * returned if enough space was available.
 *
 * @param[in] dpy               the VA display
 * @param[in] context           the video processing context
 * @param[in] type              the video filter type
 * @param[out] filter_caps      the output array of #VAProcFilterCap elements
 * @param[in,out] num_filter_caps the number of elements allocated on input,
 *      the number of elements actually filled in output
 */
VAStatus
vaQueryVideoProcFilterCaps(
    VADisplay           dpy,
    VAContextID         context,
    VAProcFilterType    type,
    void               *filter_caps,
    unsigned int       *num_filter_caps
);

/**
 * \brief Queries video processing pipeline capabilities.
 *
 * This function returns the video processing pipeline capabilities. The
 * \c filters array defines the video processing pipeline and is an array
 * of buffers holding filter parameters.
 *
 * Note: the #VAProcPipelineCaps structure contains user-provided arrays.
 * If non-NULL, the corresponding \c num_* fields shall be filled in on
 * input with the number of elements allocated. Upon successful return,
 * the actual number of elements will be overwritten into the \c num_*
 * fields. Otherwise, \c VA_STATUS_ERROR_MAX_NUM_EXCEEDED is returned
 * and \c num_* fields are adjusted to the number of elements that would
 * be returned if enough space was available.
 *
 * @param[in] dpy               the VA display
 * @param[in] context           the video processing context
 * @param[in] filters           the array of VA buffers defining the video
 *      processing pipeline
 * @param[in] num_filters       the number of elements in filters
 * @param[in,out] pipeline_caps the video processing pipeline capabilities
 */
VAStatus
vaQueryVideoProcPipelineCaps(
    VADisplay           dpy,
    VAContextID         context,
    VABufferID         *filters,
    unsigned int        num_filters,
    VAProcPipelineCaps *pipeline_caps
);

/**@}*/

#ifdef __cplusplus
}
#endif

#endif /* VA_VPP_H */
