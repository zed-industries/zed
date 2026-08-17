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

#ifndef SPA_PARAM_AUDIO_FORMAT_UTILS_H
#define SPA_PARAM_AUDIO_FORMAT_UTILS_H

#ifdef __cplusplus
extern "C" {
#endif

/**
 * \addtogroup spa_param
 * \{
 */

#include <spa/pod/parser.h>
#include <spa/pod/builder.h>
#include <spa/param/audio/format.h>
#include <spa/param/format-utils.h>

static inline int
spa_format_audio_raw_parse(const struct spa_pod *format, struct spa_audio_info_raw *info)
{
	struct spa_pod *position = NULL;
	int res;
	info->flags = 0;
	res = spa_pod_parse_object(format,
			SPA_TYPE_OBJECT_Format, NULL,
			SPA_FORMAT_AUDIO_format,	SPA_POD_Id(&info->format),
			SPA_FORMAT_AUDIO_rate,		SPA_POD_Int(&info->rate),
			SPA_FORMAT_AUDIO_channels,	SPA_POD_Int(&info->channels),
			SPA_FORMAT_AUDIO_position,	SPA_POD_OPT_Pod(&position));
	if (position == NULL ||
	    !spa_pod_copy_array(position, SPA_TYPE_Id, info->position, SPA_AUDIO_MAX_CHANNELS))
		SPA_FLAG_SET(info->flags, SPA_AUDIO_FLAG_UNPOSITIONED);

	return res;
}

static inline int
spa_format_audio_dsp_parse(const struct spa_pod *format, struct spa_audio_info_dsp *info)
{
	int res;
	res = spa_pod_parse_object(format,
			SPA_TYPE_OBJECT_Format, NULL,
			SPA_FORMAT_AUDIO_format,	SPA_POD_Id(&info->format));
	return res;
}

static inline int
spa_format_audio_iec958_parse(const struct spa_pod *format, struct spa_audio_info_iec958 *info)
{
	int res;
	res = spa_pod_parse_object(format,
			SPA_TYPE_OBJECT_Format, NULL,
			SPA_FORMAT_AUDIO_iec958Codec,	SPA_POD_Id(&info->codec),
			SPA_FORMAT_AUDIO_rate,		SPA_POD_Int(&info->rate));
	return res;
}

static inline int
spa_format_audio_dsd_parse(const struct spa_pod *format, struct spa_audio_info_dsd *info)
{
	struct spa_pod *position = NULL;
	int res;
	info->flags = 0;
	res = spa_pod_parse_object(format,
			SPA_TYPE_OBJECT_Format, NULL,
			SPA_FORMAT_AUDIO_bitorder,	SPA_POD_Id(&info->bitorder),
			SPA_FORMAT_AUDIO_interleave,	SPA_POD_Int(&info->interleave),
			SPA_FORMAT_AUDIO_rate,		SPA_POD_Int(&info->rate),
			SPA_FORMAT_AUDIO_channels,	SPA_POD_Int(&info->channels),
			SPA_FORMAT_AUDIO_position,	SPA_POD_OPT_Pod(&position));
	if (position == NULL ||
	    !spa_pod_copy_array(position, SPA_TYPE_Id, info->position, SPA_AUDIO_MAX_CHANNELS))
		SPA_FLAG_SET(info->flags, SPA_AUDIO_FLAG_UNPOSITIONED);

	return res;
}

static inline struct spa_pod *
spa_format_audio_raw_build(struct spa_pod_builder *builder, uint32_t id, struct spa_audio_info_raw *info)
{
	struct spa_pod_frame f;
	spa_pod_builder_push_object(builder, &f, SPA_TYPE_OBJECT_Format, id);
	spa_pod_builder_add(builder,
			SPA_FORMAT_mediaType,		SPA_POD_Id(SPA_MEDIA_TYPE_audio),
			SPA_FORMAT_mediaSubtype,	SPA_POD_Id(SPA_MEDIA_SUBTYPE_raw),
			0);
	if (info->format != SPA_AUDIO_FORMAT_UNKNOWN)
		spa_pod_builder_add(builder,
			SPA_FORMAT_AUDIO_format,	SPA_POD_Id(info->format), 0);
	if (info->rate != 0)
		spa_pod_builder_add(builder,
			SPA_FORMAT_AUDIO_rate,		SPA_POD_Int(info->rate), 0);
	if (info->channels != 0) {
		spa_pod_builder_add(builder,
			SPA_FORMAT_AUDIO_channels,	SPA_POD_Int(info->channels), 0);
		if (!SPA_FLAG_IS_SET(info->flags, SPA_AUDIO_FLAG_UNPOSITIONED)) {
			spa_pod_builder_add(builder, SPA_FORMAT_AUDIO_position,
				SPA_POD_Array(sizeof(uint32_t), SPA_TYPE_Id,
					info->channels, info->position), 0);
		}
	}
	return (struct spa_pod*)spa_pod_builder_pop(builder, &f);
}

static inline struct spa_pod *
spa_format_audio_dsp_build(struct spa_pod_builder *builder, uint32_t id, struct spa_audio_info_dsp *info)
{
	struct spa_pod_frame f;
	spa_pod_builder_push_object(builder, &f, SPA_TYPE_OBJECT_Format, id);
	spa_pod_builder_add(builder,
			SPA_FORMAT_mediaType,		SPA_POD_Id(SPA_MEDIA_TYPE_audio),
			SPA_FORMAT_mediaSubtype,	SPA_POD_Id(SPA_MEDIA_SUBTYPE_dsp),
			0);
	if (info->format != SPA_AUDIO_FORMAT_UNKNOWN)
		spa_pod_builder_add(builder,
			SPA_FORMAT_AUDIO_format,	SPA_POD_Id(info->format), 0);
	return (struct spa_pod*)spa_pod_builder_pop(builder, &f);
}


static inline struct spa_pod *
spa_format_audio_iec958_build(struct spa_pod_builder *builder, uint32_t id, struct spa_audio_info_iec958 *info)
{
	struct spa_pod_frame f;
	spa_pod_builder_push_object(builder, &f, SPA_TYPE_OBJECT_Format, id);
	spa_pod_builder_add(builder,
			SPA_FORMAT_mediaType,		SPA_POD_Id(SPA_MEDIA_TYPE_audio),
			SPA_FORMAT_mediaSubtype,	SPA_POD_Id(SPA_MEDIA_SUBTYPE_iec958),
			0);
	if (info->codec != SPA_AUDIO_IEC958_CODEC_UNKNOWN)
		spa_pod_builder_add(builder,
			SPA_FORMAT_AUDIO_iec958Codec,	SPA_POD_Id(info->codec), 0);
	if (info->rate != 0)
		spa_pod_builder_add(builder,
			SPA_FORMAT_AUDIO_rate,		SPA_POD_Int(info->rate), 0);
	return (struct spa_pod*)spa_pod_builder_pop(builder, &f);
}

static inline struct spa_pod *
spa_format_audio_dsd_build(struct spa_pod_builder *builder, uint32_t id, struct spa_audio_info_dsd *info)
{
	struct spa_pod_frame f;
	spa_pod_builder_push_object(builder, &f, SPA_TYPE_OBJECT_Format, id);
	spa_pod_builder_add(builder,
			SPA_FORMAT_mediaType,		SPA_POD_Id(SPA_MEDIA_TYPE_audio),
			SPA_FORMAT_mediaSubtype,	SPA_POD_Id(SPA_MEDIA_SUBTYPE_dsd),
			0);
	if (info->bitorder != SPA_PARAM_BITORDER_unknown)
		spa_pod_builder_add(builder,
			SPA_FORMAT_AUDIO_bitorder,	SPA_POD_Id(info->bitorder), 0);
	if (info->interleave != 0)
		spa_pod_builder_add(builder,
			SPA_FORMAT_AUDIO_interleave,	SPA_POD_Int(info->interleave), 0);
	if (info->rate != 0)
		spa_pod_builder_add(builder,
			SPA_FORMAT_AUDIO_rate,		SPA_POD_Int(info->rate), 0);
	if (info->channels != 0) {
		spa_pod_builder_add(builder,
			SPA_FORMAT_AUDIO_channels,	SPA_POD_Int(info->channels), 0);
		if (!SPA_FLAG_IS_SET(info->flags, SPA_AUDIO_FLAG_UNPOSITIONED)) {
			spa_pod_builder_add(builder, SPA_FORMAT_AUDIO_position,
				SPA_POD_Array(sizeof(uint32_t), SPA_TYPE_Id,
					info->channels, info->position), 0);
		}
	}
	return (struct spa_pod*)spa_pod_builder_pop(builder, &f);
}

/**
 * \}
 */

#ifdef __cplusplus
}  /* extern "C" */
#endif

#endif /* SPA_PARAM_AUDIO_FORMAT_UTILS_H */
