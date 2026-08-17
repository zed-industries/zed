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

#ifndef SPA_AUDIO_WMA_H
#define SPA_AUDIO_WMA_H

#ifdef __cplusplus
extern "C" {
#endif

#include <spa/param/audio/raw.h>

enum spa_audio_wma_profile {
	SPA_AUDIO_WMA_PROFILE_UNKNOWN,

	SPA_AUDIO_WMA_PROFILE_WMA7,
	SPA_AUDIO_WMA_PROFILE_WMA8,
	SPA_AUDIO_WMA_PROFILE_WMA9,
	SPA_AUDIO_WMA_PROFILE_WMA10,
	SPA_AUDIO_WMA_PROFILE_WMA9_PRO,
	SPA_AUDIO_WMA_PROFILE_WMA9_LOSSLESS,
	SPA_AUDIO_WMA_PROFILE_WMA10_LOSSLESS,

	SPA_AUDIO_WMA_PROFILE_CUSTOM = 0x10000,
};

struct spa_audio_info_wma {
	uint32_t rate;				/*< sample rate */
	uint32_t channels;			/*< number of channels */
	uint32_t bitrate;			/*< stream bitrate */
	uint32_t block_align;			/*< block alignment */
	enum spa_audio_wma_profile profile;	/*< WMA profile */

};

#define SPA_AUDIO_INFO_WMA_INIT(...)		((struct spa_audio_info_wma) { __VA_ARGS__ })

/**
 * \}
 */

#ifdef __cplusplus
}  /* extern "C" */
#endif

#endif /* SPA_AUDIO_WMA_H */
