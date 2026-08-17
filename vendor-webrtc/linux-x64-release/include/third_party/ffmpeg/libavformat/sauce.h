/*
 * SAUCE header parser
 * Copyright (c) 2010 Peter Ross <pross@xvid.org>
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

/**
 * @file
 * SAUCE header parser
 */

#ifndef AVFORMAT_SAUCE_H
#define AVFORMAT_SAUCE_H

#include "avformat.h"

/**
 * @param avctx AVFormatContext
 * @param[out] fsize return length of file, less SAUCE header
 * @param[out] got_width set to non-zero if SAUCE header reported height
 * @param get_height Tell SAUCE header to parse height
 */
int ff_sauce_read(AVFormatContext *avctx, uint64_t *fsize, int *got_width, int get_height);

#endif /* AVFORMAT_SAUCE_H */
