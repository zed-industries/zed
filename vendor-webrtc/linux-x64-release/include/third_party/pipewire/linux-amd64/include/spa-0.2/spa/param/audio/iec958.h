/* Simple Plugin API
 *
 * Copyright © 2021 Wim Taymans
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

#ifndef SPA_AUDIO_IEC958_H
#define SPA_AUDIO_IEC958_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * \addtogroup spa_param
 * \{
 */
enum spa_audio_iec958_codec {
	SPA_AUDIO_IEC958_CODEC_UNKNOWN,

	SPA_AUDIO_IEC958_CODEC_PCM,
	SPA_AUDIO_IEC958_CODEC_DTS,
	SPA_AUDIO_IEC958_CODEC_AC3,
	SPA_AUDIO_IEC958_CODEC_MPEG,		/**< MPEG-1 or MPEG-2 (Part 3, not AAC) */
	SPA_AUDIO_IEC958_CODEC_MPEG2_AAC,	/**< MPEG-2 AAC */

	SPA_AUDIO_IEC958_CODEC_EAC3,

	SPA_AUDIO_IEC958_CODEC_TRUEHD,		/**< Dolby TrueHD */
	SPA_AUDIO_IEC958_CODEC_DTSHD,		/**< DTS-HD Master Audio */
};

struct spa_audio_info_iec958 {
	enum spa_audio_iec958_codec codec;	/*< format, one of the DSP formats in enum spa_audio_format_dsp */
	uint32_t flags;				/*< extra flags */
	uint32_t rate;				/*< sample rate */
};

#define SPA_AUDIO_INFO_IEC958_INIT(...)		(struct spa_audio_info_iec958) { __VA_ARGS__ }

/**
 * \}
 */

#ifdef __cplusplus
}  /* extern "C" */
#endif

#endif /* SPA_AUDIO_IEC958_H */
