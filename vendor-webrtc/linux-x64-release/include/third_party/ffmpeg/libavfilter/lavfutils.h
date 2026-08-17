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
 * Miscellaneous utilities which make use of the libavformat library
 */

#ifndef AVFILTER_LAVFUTILS_H
#define AVFILTER_LAVFUTILS_H

#include <stdint.h>
#include "libavutil/pixfmt.h"

/**
 * Load image from filename and put the resulting image in data.
 *
 * @param w pointer to the width of the loaded image
 * @param h pointer to the height of the loaded image
 * @param pix_fmt pointer to the pixel format of the loaded image
 * @param filename the name of the image file to load
 * @param log_ctx log context
 * @return >= 0 in case of success, a negative error code otherwise.
 */
int ff_load_image(uint8_t *data[4], int linesize[4],
                  int *w, int *h, enum AVPixelFormat *pix_fmt,
                  const char *filename, void *log_ctx);

#endif  /* AVFILTER_LAVFUTILS_H */
