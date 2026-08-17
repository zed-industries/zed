/* Simple Plugin API
 *
 * Copyright © 2018 Wim Taymans
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice (including the next
 * paragraph) shall be included in all copies or substantial portions of the
 * Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
 * DEALINGS IN THE SOFTWARE.
 */

#ifndef SPA_PARAM_VIDEO_FORMAT_UTILS_H
#define SPA_PARAM_VIDEO_FORMAT_UTILS_H

#ifdef __cplusplus
extern "C" {
#endif

/**
 * \addtogroup spa_param
 * \{
 */
#include <spa/pod/parser.h>
#include <spa/pod/builder.h>
#include <spa/param/video/format.h>
#include <spa/param/format-utils.h>

static inline int
spa_format_video_raw_parse(const struct spa_pod *format,
			   struct spa_video_info_raw *info)
{
	return spa_pod_parse_object(format,
		SPA_TYPE_OBJECT_Format, NULL,
		SPA_FORMAT_VIDEO_format,		SPA_POD_Id(&info->format),
		SPA_FORMAT_VIDEO_modifier,		SPA_POD_OPT_Long(&info->modifier),
		SPA_FORMAT_VIDEO_size,			SPA_POD_Rectangle(&info->size),
		SPA_FORMAT_VIDEO_framerate,		SPA_POD_Fraction(&info->framerate),
		SPA_FORMAT_VIDEO_maxFramerate,		SPA_POD_OPT_Fraction(&info->max_framerate),
		SPA_FORMAT_VIDEO_views,			SPA_POD_OPT_Int(&info->views),
		SPA_FORMAT_VIDEO_interlaceMode,		SPA_POD_OPT_Id(&info->interlace_mode),
		SPA_FORMAT_VIDEO_pixelAspectRatio,	SPA_POD_OPT_Fraction(&info->pixel_aspect_ratio),
		SPA_FORMAT_VIDEO_multiviewMode,		SPA_POD_OPT_Id(&info->multiview_mode),
		SPA_FORMAT_VIDEO_multiviewFlags,	SPA_POD_OPT_Id(&info->multiview_flags),
		SPA_FORMAT_VIDEO_chromaSite,		SPA_POD_OPT_Id(&info->chroma_site),
		SPA_FORMAT_VIDEO_colorRange,		SPA_POD_OPT_Id(&info->color_range),
		SPA_FORMAT_VIDEO_colorMatrix,		SPA_POD_OPT_Id(&info->color_matrix),
		SPA_FORMAT_VIDEO_transferFunction,	SPA_POD_OPT_Id(&info->transfer_function),
		SPA_FORMAT_VIDEO_colorPrimaries,	SPA_POD_OPT_Id(&info->color_primaries));
}

static inline int
spa_format_video_dsp_parse(const struct spa_pod *format,
			   struct spa_video_info_dsp *info)
{
	return spa_pod_parse_object(format,
		SPA_TYPE_OBJECT_Format, NULL,
		SPA_FORMAT_VIDEO_format,		SPA_POD_Id(&info->format),
		SPA_FORMAT_VIDEO_modifier,		SPA_POD_OPT_Long(&info->modifier));
}

static inline struct spa_pod *
spa_format_video_raw_build(struct spa_pod_builder *builder, uint32_t id,
			   struct spa_video_info_raw *info)
{
	struct spa_pod_frame f;
	spa_pod_builder_push_object(builder, &f, SPA_TYPE_OBJECT_Format, id);
	spa_pod_builder_add(builder,
			SPA_FORMAT_mediaType,		SPA_POD_Id(SPA_MEDIA_TYPE_video),
			SPA_FORMAT_mediaSubtype,	SPA_POD_Id(SPA_MEDIA_SUBTYPE_raw),
			0);
	if (info->format != SPA_VIDEO_FORMAT_UNKNOWN)
		spa_pod_builder_add(builder,
			SPA_FORMAT_VIDEO_format,	SPA_POD_Id(info->format), 0);
	if (info->size.width != 0 && info->size.height != 0)
		spa_pod_builder_add(builder,
			SPA_FORMAT_VIDEO_size,		SPA_POD_Rectangle(&info->size), 0);
	if (info->framerate.denom != 0)
		spa_pod_builder_add(builder,
			SPA_FORMAT_VIDEO_framerate,	SPA_POD_Fraction(&info->framerate), 0);
	if (info->modifier != 0)
		spa_pod_builder_add(builder,
			SPA_FORMAT_VIDEO_modifier,	SPA_POD_Long(info->modifier), 0);
	if (info->max_framerate.denom != 0)
		spa_pod_builder_add(builder,
			SPA_FORMAT_VIDEO_maxFramerate,	SPA_POD_Fraction(info->max_framerate), 0);
	if (info->views != 0)
		spa_pod_builder_add(builder,
			SPA_FORMAT_VIDEO_views,		SPA_POD_Int(info->views), 0);
	if (info->interlace_mode != 0)
		spa_pod_builder_add(builder,
			SPA_FORMAT_VIDEO_interlaceMode,	SPA_POD_Id(info->interlace_mode), 0);
	if (info->pixel_aspect_ratio.denom != 0)
		spa_pod_builder_add(builder,
			SPA_FORMAT_VIDEO_pixelAspectRatio,SPA_POD_Fraction(info->pixel_aspect_ratio), 0);
	if (info->multiview_mode != 0)
		spa_pod_builder_add(builder,
			SPA_FORMAT_VIDEO_multiviewMode,	SPA_POD_Id(info->multiview_mode), 0);
	if (info->multiview_flags != 0)
		spa_pod_builder_add(builder,
			SPA_FORMAT_VIDEO_multiviewFlags,SPA_POD_Id(info->multiview_flags), 0);
	if (info->chroma_site != 0)
		spa_pod_builder_add(builder,
			SPA_FORMAT_VIDEO_chromaSite,	SPA_POD_Id(info->chroma_site), 0);
	if (info->color_range != 0)
		spa_pod_builder_add(builder,
			SPA_FORMAT_VIDEO_colorRange,	SPA_POD_Id(info->color_range), 0);
	if (info->color_matrix != 0)
		spa_pod_builder_add(builder,
			SPA_FORMAT_VIDEO_colorMatrix,	SPA_POD_Id(info->color_matrix), 0);
	if (info->transfer_function != 0)
		spa_pod_builder_add(builder,
			SPA_FORMAT_VIDEO_transferFunction,SPA_POD_Id(info->transfer_function), 0);
	if (info->color_primaries != 0)
		spa_pod_builder_add(builder,
			SPA_FORMAT_VIDEO_colorPrimaries,SPA_POD_Id(info->color_primaries), 0);
	return (struct spa_pod*)spa_pod_builder_pop(builder, &f);
}

static inline struct spa_pod *
spa_format_video_dsp_build(struct spa_pod_builder *builder, uint32_t id,
			   struct spa_video_info_dsp *info)
{
	struct spa_pod_frame f;
	spa_pod_builder_push_object(builder, &f, SPA_TYPE_OBJECT_Format, id);
	spa_pod_builder_add(builder,
			SPA_FORMAT_mediaType,		SPA_POD_Id(SPA_MEDIA_TYPE_video),
			SPA_FORMAT_mediaSubtype,	SPA_POD_Id(SPA_MEDIA_SUBTYPE_dsp),
			0);
	if (info->format != SPA_VIDEO_FORMAT_UNKNOWN)
		spa_pod_builder_add(builder,
			SPA_FORMAT_VIDEO_format,	SPA_POD_Id(info->format), 0);
	if (info->modifier)
		spa_pod_builder_add(builder,
			SPA_FORMAT_VIDEO_modifier,	SPA_POD_Long(info->modifier), 0);
	return (struct spa_pod*)spa_pod_builder_pop(builder, &f);
}

static inline int
spa_format_video_h264_parse(const struct spa_pod *format,
			    struct spa_video_info_h264 *info)
{
	return spa_pod_parse_object(format,
			SPA_TYPE_OBJECT_Format, NULL,
			SPA_FORMAT_VIDEO_size,			SPA_POD_OPT_Rectangle(&info->size),
			SPA_FORMAT_VIDEO_framerate,		SPA_POD_OPT_Fraction(&info->framerate),
			SPA_FORMAT_VIDEO_maxFramerate,		SPA_POD_OPT_Fraction(&info->max_framerate),
			SPA_FORMAT_VIDEO_H264_streamFormat,	SPA_POD_OPT_Id(&info->stream_format),
			SPA_FORMAT_VIDEO_H264_alignment,	SPA_POD_OPT_Id(&info->alignment));
}

static inline struct spa_pod *
spa_format_video_h264_build(struct spa_pod_builder *builder, uint32_t id,
			   struct spa_video_info_h264 *info)
{
	struct spa_pod_frame f;
	spa_pod_builder_push_object(builder, &f, SPA_TYPE_OBJECT_Format, id);
	spa_pod_builder_add(builder,
			SPA_FORMAT_mediaType,		SPA_POD_Id(SPA_MEDIA_TYPE_video),
			SPA_FORMAT_mediaSubtype,	SPA_POD_Id(SPA_MEDIA_SUBTYPE_h264),
			0);
	if (info->size.width != 0 && info->size.height != 0)
		spa_pod_builder_add(builder,
			SPA_FORMAT_VIDEO_size,		SPA_POD_Rectangle(&info->size), 0);
	if (info->framerate.denom != 0)
		spa_pod_builder_add(builder,
			SPA_FORMAT_VIDEO_framerate,	SPA_POD_Fraction(&info->framerate), 0);
	if (info->max_framerate.denom != 0)
		spa_pod_builder_add(builder,
			SPA_FORMAT_VIDEO_maxFramerate,	SPA_POD_Fraction(info->max_framerate), 0);
	if (info->stream_format != 0)
		spa_pod_builder_add(builder,
			SPA_FORMAT_VIDEO_H264_streamFormat, SPA_POD_Id(info->stream_format), 0);
	if (info->alignment != 0)
		spa_pod_builder_add(builder,
			SPA_FORMAT_VIDEO_H264_alignment, SPA_POD_Id(info->alignment), 0);
	return (struct spa_pod*)spa_pod_builder_pop(builder, &f);
}

static inline int
spa_format_video_mjpg_parse(const struct spa_pod *format,
			    struct spa_video_info_mjpg *info)
{
	return spa_pod_parse_object(format,
			SPA_TYPE_OBJECT_Format, NULL,
			SPA_FORMAT_VIDEO_size,		SPA_POD_OPT_Rectangle(&info->size),
			SPA_FORMAT_VIDEO_framerate,	SPA_POD_OPT_Fraction(&info->framerate),
			SPA_FORMAT_VIDEO_maxFramerate,	SPA_POD_OPT_Fraction(&info->max_framerate));
}

static inline struct spa_pod *
spa_format_video_mjpg_build(struct spa_pod_builder *builder, uint32_t id,
			   struct spa_video_info_mjpg *info)
{
	struct spa_pod_frame f;
	spa_pod_builder_push_object(builder, &f, SPA_TYPE_OBJECT_Format, id);
	spa_pod_builder_add(builder,
			SPA_FORMAT_mediaType,		SPA_POD_Id(SPA_MEDIA_TYPE_video),
			SPA_FORMAT_mediaSubtype,	SPA_POD_Id(SPA_MEDIA_SUBTYPE_mjpg),
			0);
	if (info->size.width != 0 && info->size.height != 0)
		spa_pod_builder_add(builder,
			SPA_FORMAT_VIDEO_size,		SPA_POD_Rectangle(&info->size), 0);
	if (info->framerate.denom != 0)
		spa_pod_builder_add(builder,
			SPA_FORMAT_VIDEO_framerate,	SPA_POD_Fraction(&info->framerate), 0);
	if (info->max_framerate.denom != 0)
		spa_pod_builder_add(builder,
			SPA_FORMAT_VIDEO_maxFramerate,	SPA_POD_Fraction(info->max_framerate), 0);
	return (struct spa_pod*)spa_pod_builder_pop(builder, &f);
}

/**
 * \}
 */

#ifdef __cplusplus
} /* extern "C" */
#endif

#endif /* SPA_PARAM_VIDEO_FORMAT_UTILS_H */
