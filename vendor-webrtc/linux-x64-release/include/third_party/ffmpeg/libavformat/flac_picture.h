/*
 * Raw FLAC picture parser
 * Copyright (c) 2001 Fabrice Bellard
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

#ifndef AVFORMAT_FLAC_PICTURE_H
#define AVFORMAT_FLAC_PICTURE_H

#include "avformat.h"

#define RETURN_ERROR(code) do { ret = (code); goto fail; } while (0)

/**
 * Parse a FLAC METADATA_BLOCK_PICTURE
 *
 * @param s   AVFormatContext for logging and the attached picture stream.
 * @param buf `*buf` points to the actual data which must be padded by
 *            AV_INPUT_BUFFER_PADDING_SIZE bytes not counted in buf_size.
 *            This function may take ownership of `*buf` and reset it.
 * @param buf_size size of `*buf` (excluding padding)
 * @param truncate_workaround If set, additional data may be read from s->pb if
 *                            truncation has been detected.
 */
int ff_flac_parse_picture(AVFormatContext *s, uint8_t **buf, int buf_size,
                          int truncate_workaround);

#endif /* AVFORMAT_FLAC_PICTURE_H */
