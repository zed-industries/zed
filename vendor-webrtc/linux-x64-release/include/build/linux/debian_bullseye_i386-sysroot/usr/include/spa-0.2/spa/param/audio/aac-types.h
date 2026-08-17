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

#ifndef SPA_AUDIO_AAC_TYPES_H
#define SPA_AUDIO_AAC_TYPES_H

#ifdef __cplusplus
extern "C" {
#endif

#include <spa/utils/type.h>
#include <spa/param/audio/aac.h>

#define SPA_TYPE_INFO_AudioAACStreamFormat		SPA_TYPE_INFO_ENUM_BASE "AudioAACStreamFormat"
#define SPA_TYPE_INFO_AUDIO_AAC_STREAM_FORMAT_BASE	SPA_TYPE_INFO_AudioAACStreamFormat ":"

static const struct spa_type_info spa_type_audio_aac_stream_format[] = {
	{ SPA_AUDIO_AAC_STREAM_FORMAT_UNKNOWN, SPA_TYPE_Int, SPA_TYPE_INFO_AUDIO_AAC_STREAM_FORMAT_BASE "UNKNOWN", NULL },
	{ SPA_AUDIO_AAC_STREAM_FORMAT_RAW, SPA_TYPE_Int, SPA_TYPE_INFO_AUDIO_AAC_STREAM_FORMAT_BASE "RAW", NULL },
	{ SPA_AUDIO_AAC_STREAM_FORMAT_MP2ADTS, SPA_TYPE_Int, SPA_TYPE_INFO_AUDIO_AAC_STREAM_FORMAT_BASE "MP2ADTS", NULL },
	{ SPA_AUDIO_AAC_STREAM_FORMAT_MP4ADTS, SPA_TYPE_Int, SPA_TYPE_INFO_AUDIO_AAC_STREAM_FORMAT_BASE "MP4ADTS", NULL },
	{ SPA_AUDIO_AAC_STREAM_FORMAT_MP4LOAS, SPA_TYPE_Int, SPA_TYPE_INFO_AUDIO_AAC_STREAM_FORMAT_BASE "MP4LOAS", NULL },
	{ SPA_AUDIO_AAC_STREAM_FORMAT_MP4LATM, SPA_TYPE_Int, SPA_TYPE_INFO_AUDIO_AAC_STREAM_FORMAT_BASE "MP4LATM", NULL },
	{ SPA_AUDIO_AAC_STREAM_FORMAT_ADIF, SPA_TYPE_Int, SPA_TYPE_INFO_AUDIO_AAC_STREAM_FORMAT_BASE "ADIF", NULL },
	{ SPA_AUDIO_AAC_STREAM_FORMAT_MP4FF, SPA_TYPE_Int, SPA_TYPE_INFO_AUDIO_AAC_STREAM_FORMAT_BASE "MP4FF", NULL },
	{ 0, 0, NULL, NULL },
};

/**
 * \}
 */

#ifdef __cplusplus
}  /* extern "C" */
#endif

#endif /* SPA_AUDIO_AAC_TYPES_H */
