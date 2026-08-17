/* Simple Plugin API
 *
 * Copyright © 2023 Wim Taymans
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

#ifndef SPA_AUDIO_AAC_H
#define SPA_AUDIO_AAC_H

#ifdef __cplusplus
extern "C" {
#endif

#include <spa/param/audio/raw.h>

enum spa_audio_aac_stream_format {
	SPA_AUDIO_AAC_STREAM_FORMAT_UNKNOWN,
	/* Raw AAC frames */
	SPA_AUDIO_AAC_STREAM_FORMAT_RAW,
	/* ISO/IEC 13818-7 MPEG-2 Audio Data Transport Stream (ADTS) */
	SPA_AUDIO_AAC_STREAM_FORMAT_MP2ADTS,
	/* ISO/IEC 14496-3 MPEG-4 Audio Data Transport Stream (ADTS) */
	SPA_AUDIO_AAC_STREAM_FORMAT_MP4ADTS,
	/* ISO/IEC 14496-3 Low Overhead Audio Stream (LOAS) */
	SPA_AUDIO_AAC_STREAM_FORMAT_MP4LOAS,
	/* ISO/IEC 14496-3 Low Overhead Audio Transport Multiplex (LATM) */
	SPA_AUDIO_AAC_STREAM_FORMAT_MP4LATM,
	/* ISO/IEC 14496-3 Audio Data Interchange Format (ADIF) */
	SPA_AUDIO_AAC_STREAM_FORMAT_ADIF,
	/* ISO/IEC 14496-12 MPEG-4 file format */
	SPA_AUDIO_AAC_STREAM_FORMAT_MP4FF,

	SPA_AUDIO_AAC_STREAM_FORMAT_CUSTOM = 0x10000,
};

struct spa_audio_info_aac {
	uint32_t rate;					/*< sample rate */
	uint32_t channels;				/*< number of channels */
	uint32_t bitrate;				/*< stream bitrate */
	enum spa_audio_aac_stream_format stream_format;	/*< AAC audio stream format */
};

#define SPA_AUDIO_INFO_AAC_INIT(...)		((struct spa_audio_info_aac) { __VA_ARGS__ })

/**
 * \}
 */

#ifdef __cplusplus
}  /* extern "C" */
#endif

#endif /* SPA_AUDIO_AAC_H */
