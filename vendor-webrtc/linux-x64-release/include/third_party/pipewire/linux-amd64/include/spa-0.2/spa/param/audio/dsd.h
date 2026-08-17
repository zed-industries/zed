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

#ifndef SPA_AUDIO_DSD_H
#define SPA_AUDIO_DSD_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

#include <spa/param/param.h>
#include <spa/param/audio/raw.h>

/**
 * \addtogroup spa_param
 * \{
 */

/** Extra DSD audio flags */
#define SPA_AUDIO_DSD_FLAG_NONE		(0)		/*< no valid flag */

/* DSD bits are transferred in a buffer grouped in bytes with the bitorder
 * defined by \a bitorder.
 *
 * Channels are placed in separate planes (interleave = 0) or interleaved
 * using the interleave value. A negative interleave value means that the
 * bytes need to be reversed in the group.
 *
 *  Planar (interleave = 0):
 *    plane1: l1 l2 l3 l4 l5 ...
 *    plane2: r1 r2 r3 r4 r5 ...
 *
 *  Interleaved 4:
 *    plane1: l1 l2 l3 l4 r1 r2 r3 r4 l5 l6 l7 l8 r5 r6 r7 r8 l9 ...
 *
 *  Interleaved 2:
 *    plane1: l1 l2 r1 r2 l3 l4 r3 r4  ...
 */
struct spa_audio_info_dsd {
	enum spa_param_bitorder bitorder;		/*< the order of the bits */
	uint32_t flags;					/*< extra flags */
	int32_t interleave;				/*< interleave bytes */
	uint32_t rate;					/*< sample rate (in bytes per second) */
	uint32_t channels;				/*< channels */
	uint32_t position[SPA_AUDIO_MAX_CHANNELS];	/*< channel position from enum spa_audio_channel */
};

#define SPA_AUDIO_INFO_DSD_INIT(...)		(struct spa_audio_info_dsd) { __VA_ARGS__ }

/**
 * \}
 */

#ifdef __cplusplus
}  /* extern "C" */
#endif

#endif /* SPA_AUDIO_DSD_H */
