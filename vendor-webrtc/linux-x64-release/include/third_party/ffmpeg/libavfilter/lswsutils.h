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

/**
 * @file
 * Miscellaneous utilities which make use of the libswscale library
 */

#ifndef AVFILTER_LSWSUTILS_H
#define AVFILTER_LSWSUTILS_H

#include "libswscale/swscale.h"

/**
 * Scale image using libswscale.
 */
int ff_scale_image(uint8_t *dst_data[4], int dst_linesize[4],
                   int dst_w, int dst_h, enum AVPixelFormat dst_pix_fmt,
                   uint8_t *const src_data[4], int src_linesize[4],
                   int src_w, int src_h, enum AVPixelFormat src_pix_fmt,
                   void *log_ctx);

#endif  /* AVFILTER_LSWSUTILS_H */
