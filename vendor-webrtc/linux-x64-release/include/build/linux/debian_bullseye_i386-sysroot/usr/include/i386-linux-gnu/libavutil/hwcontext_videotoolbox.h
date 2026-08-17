/*
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

#ifndef AVUTIL_HWCONTEXT_VIDEOTOOLBOX_H
#define AVUTIL_HWCONTEXT_VIDEOTOOLBOX_H

#include <stdint.h>

#include <VideoToolbox/VideoToolbox.h>

#include "pixfmt.h"

/**
 * @file
 * An API-specific header for AV_HWDEVICE_TYPE_VIDEOTOOLBOX.
 *
 * This API currently does not support frame allocation, as the raw VideoToolbox
 * API does allocation, and FFmpeg itself never has the need to allocate frames.
 *
 * If the API user sets a custom pool, AVHWFramesContext.pool must return
 * AVBufferRefs whose data pointer is a CVImageBufferRef or CVPixelBufferRef.
 *
 * Currently AVHWDeviceContext.hwctx and AVHWFramesContext.hwctx are always
 * NULL.
 */

/**
 * Convert a VideoToolbox (actually CoreVideo) format to AVPixelFormat.
 * Returns AV_PIX_FMT_NONE if no known equivalent was found.
 */
enum AVPixelFormat av_map_videotoolbox_format_to_pixfmt(uint32_t cv_fmt);

/**
 * Convert an AVPixelFormat to a VideoToolbox (actually CoreVideo) format.
 * Returns 0 if no known equivalent was found.
 */
uint32_t av_map_videotoolbox_format_from_pixfmt(enum AVPixelFormat pix_fmt);

/**
 * Same as av_map_videotoolbox_format_from_pixfmt function, but can map and
 * return full range pixel formats via a flag.
 */
uint32_t av_map_videotoolbox_format_from_pixfmt2(enum AVPixelFormat pix_fmt, bool full_range);

#endif /* AVUTIL_HWCONTEXT_VIDEOTOOLBOX_H */
