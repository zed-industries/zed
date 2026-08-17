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

#ifndef SPA_AUDIO_IEC958_TYPES_H
#define SPA_AUDIO_IEC958_TYPES_H

#ifdef __cplusplus
extern "C" {
#endif

#include <spa/utils/type.h>
#include <spa/param/audio/iec958.h>

#define SPA_TYPE_INFO_AudioIEC958Codec		SPA_TYPE_INFO_ENUM_BASE "AudioIEC958Codec"
#define SPA_TYPE_INFO_AUDIO_IEC958_CODEC_BASE	SPA_TYPE_INFO_AudioIEC958Codec ":"

static const struct spa_type_info spa_type_audio_iec958_codec[] = {
	{ SPA_AUDIO_IEC958_CODEC_UNKNOWN, SPA_TYPE_Int, SPA_TYPE_INFO_AUDIO_IEC958_CODEC_BASE "UNKNOWN", NULL },
	{ SPA_AUDIO_IEC958_CODEC_PCM, SPA_TYPE_Int, SPA_TYPE_INFO_AUDIO_IEC958_CODEC_BASE "PCM", NULL },
	{ SPA_AUDIO_IEC958_CODEC_DTS, SPA_TYPE_Int, SPA_TYPE_INFO_AUDIO_IEC958_CODEC_BASE "DTS", NULL },
	{ SPA_AUDIO_IEC958_CODEC_AC3, SPA_TYPE_Int, SPA_TYPE_INFO_AUDIO_IEC958_CODEC_BASE "AC3", NULL },
	{ SPA_AUDIO_IEC958_CODEC_MPEG, SPA_TYPE_Int, SPA_TYPE_INFO_AUDIO_IEC958_CODEC_BASE "MPEG", NULL },
	{ SPA_AUDIO_IEC958_CODEC_MPEG2_AAC, SPA_TYPE_Int, SPA_TYPE_INFO_AUDIO_IEC958_CODEC_BASE "MPEG2-AAC", NULL },
	{ SPA_AUDIO_IEC958_CODEC_EAC3, SPA_TYPE_Int, SPA_TYPE_INFO_AUDIO_IEC958_CODEC_BASE "EAC3", NULL },
	{ SPA_AUDIO_IEC958_CODEC_TRUEHD, SPA_TYPE_Int, SPA_TYPE_INFO_AUDIO_IEC958_CODEC_BASE "TrueHD", NULL },
	{ SPA_AUDIO_IEC958_CODEC_DTSHD, SPA_TYPE_Int, SPA_TYPE_INFO_AUDIO_IEC958_CODEC_BASE "DTS-HD", NULL },
	{ 0, 0, NULL, NULL },
};

/**
 * \}
 */

#ifdef __cplusplus
}  /* extern "C" */
#endif

#endif /* SPA_AUDIO_RAW_IEC958_TYPES_H */
