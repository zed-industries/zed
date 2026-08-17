/*
 * Copyright (c) 2007 Bobby Bingham
 *
 * This file is part of FFmpeg.
 *
 * FFmpeg is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * FFmpeg is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with FFmpeg; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA
 */

#ifndef AVFILTER_VIDEO_H
#define AVFILTER_VIDEO_H

#include "avfilter.h"
#include "filters.h"

/**
 * An AVFilterPad array whose only entry has name "default"
 * and is of type AVMEDIA_TYPE_VIDEO.
 */
extern const AVFilterPad ff_video_default_filterpad[1];

AVFrame *ff_default_get_video_buffer(AVFilterLink *link, int w, int h);
AVFrame *ff_default_get_video_buffer2(AVFilterLink *link, int w, int h, int align);
AVFrame *ff_null_get_video_buffer(AVFilterLink *link, int w, int h);

/**
 * Request a picture buffer with a specific set of permissions.
 *
 * @param link  the output link to the filter from which the buffer will
 *              be requested
 * @param w     the minimum width of the buffer to allocate
 * @param h     the minimum height of the buffer to allocate
 * @return      on success, an AVFrame owned by the caller, NULL on error
 */
AVFrame *ff_get_video_buffer(AVFilterLink *link, int w, int h);

/**
 * Returns true if a pixel format is "regular YUV", which includes all pixel
 * formats that are affected by YUV colorspace negotiation.
 */
int ff_fmt_is_regular_yuv(enum AVPixelFormat fmt);

/**
 * Returns true if a YUV pixel format is forced full range (i.e. YUVJ).
 */
int ff_fmt_is_forced_full_range(enum AVPixelFormat fmt);

#endif /* AVFILTER_VIDEO_H */
