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

#ifndef AVFORMAT_REPLAYGAIN_H
#define AVFORMAT_REPLAYGAIN_H

#include "libavutil/dict.h"

#include "avformat.h"

/**
 * Parse replaygain tags and export them as per-stream side data.
 */
int ff_replaygain_export(AVStream *st, AVDictionary *metadata);


/**
 * Export already decoded replaygain values as per-stream side data.
 */
int ff_replaygain_export_raw(AVStream *st, int32_t tg, uint32_t tp,
                             int32_t ag, uint32_t ap);

#endif /* AVFORMAT_REPLAYGAIN_H */
