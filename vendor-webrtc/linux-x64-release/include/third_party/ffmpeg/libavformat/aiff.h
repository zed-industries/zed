/*
 * AIFF/AIFF-C muxer/demuxer common header
 * Copyright (c) 2006  Patrick Guimond
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
 * common header for AIFF muxer and demuxer
 */

#ifndef AVFORMAT_AIFF_H
#define AVFORMAT_AIFF_H

#include "internal.h"

extern const AVCodecTag ff_codec_aiff_tags[];
extern const AVCodecTag *const ff_aiff_codec_tags_list[];

#endif /* AVFORMAT_AIFF_H */
