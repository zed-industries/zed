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

#ifndef SPA_AUDIO_WMA_TYPES_H
#define SPA_AUDIO_WMA_TYPES_H

#ifdef __cplusplus
extern "C" {
#endif

#include <spa/utils/type.h>
#include <spa/param/audio/wma.h>

#define SPA_TYPE_INFO_AudioWMAProfile		SPA_TYPE_INFO_ENUM_BASE "AudioWMAProfile"
#define SPA_TYPE_INFO_AUDIO_WMA_PROFILE_BASE	SPA_TYPE_INFO_AudioWMAProfile ":"

static const struct spa_type_info spa_type_audio_wma_profile[] = {
	{ SPA_AUDIO_WMA_PROFILE_UNKNOWN, SPA_TYPE_Int, SPA_TYPE_INFO_AUDIO_WMA_PROFILE_BASE "UNKNOWN", NULL },
	{ SPA_AUDIO_WMA_PROFILE_WMA7, SPA_TYPE_Int, SPA_TYPE_INFO_AUDIO_WMA_PROFILE_BASE "WMA7", NULL },
	{ SPA_AUDIO_WMA_PROFILE_WMA8, SPA_TYPE_Int, SPA_TYPE_INFO_AUDIO_WMA_PROFILE_BASE "WMA8", NULL },
	{ SPA_AUDIO_WMA_PROFILE_WMA9, SPA_TYPE_Int, SPA_TYPE_INFO_AUDIO_WMA_PROFILE_BASE "WMA9", NULL },
	{ SPA_AUDIO_WMA_PROFILE_WMA10, SPA_TYPE_Int, SPA_TYPE_INFO_AUDIO_WMA_PROFILE_BASE "WMA10", NULL },
	{ SPA_AUDIO_WMA_PROFILE_WMA9_PRO, SPA_TYPE_Int, SPA_TYPE_INFO_AUDIO_WMA_PROFILE_BASE "WMA9-Pro", NULL },
	{ SPA_AUDIO_WMA_PROFILE_WMA9_LOSSLESS, SPA_TYPE_Int, SPA_TYPE_INFO_AUDIO_WMA_PROFILE_BASE "WMA9-Lossless", NULL },
	{ SPA_AUDIO_WMA_PROFILE_WMA10_LOSSLESS, SPA_TYPE_Int, SPA_TYPE_INFO_AUDIO_WMA_PROFILE_BASE "WMA10-Lossless", NULL },
	{ 0, 0, NULL, NULL },
};
/**
 * \}
 */

#ifdef __cplusplus
}  /* extern "C" */
#endif

#endif /* SPA_AUDIO_WMA_TYPES_H */
