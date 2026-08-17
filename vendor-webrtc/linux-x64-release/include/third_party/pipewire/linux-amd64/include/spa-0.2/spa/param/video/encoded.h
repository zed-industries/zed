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

#ifndef SPA_VIDEO_ENCODED_H
#define SPA_VIDEO_ENCODED_H

#ifdef __cplusplus
extern "C" {
#endif

/**
 * \addtogroup spa_param
 * \{
 */

#include <spa/param/format.h>

enum spa_h264_stream_format {
	SPA_H264_STREAM_FORMAT_UNKNOWN = 0,
	SPA_H264_STREAM_FORMAT_AVC,
	SPA_H264_STREAM_FORMAT_AVC3,
	SPA_H264_STREAM_FORMAT_BYTESTREAM
};

enum spa_h264_alignment {
	SPA_H264_ALIGNMENT_UNKNOWN = 0,
	SPA_H264_ALIGNMENT_AU,
	SPA_H264_ALIGNMENT_NAL
};

struct spa_video_info_h264 {
	struct spa_rectangle size;
	struct spa_fraction framerate;
	struct spa_fraction max_framerate;
	enum spa_h264_stream_format stream_format;
	enum spa_h264_alignment alignment;
};

struct spa_video_info_mjpg {
	struct spa_rectangle size;
	struct spa_fraction framerate;
	struct spa_fraction max_framerate;
};

/**
 * \}
 */

#ifdef __cplusplus
} /* extern "C" */
#endif

#endif /* SPA_VIDEO_ENCODED_H */
