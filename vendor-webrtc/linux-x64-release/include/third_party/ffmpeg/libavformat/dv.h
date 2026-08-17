/*
 * General DV demuxer
 * Copyright (c) 2003 Roman Shaposhnik
 *
 * Many thanks to Dan Dennedy <dan@dennedy.org> for providing wealth
 * of DV technical info.
 *
 * Raw DV format
 * Copyright (c) 2002 Fabrice Bellard
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

#ifndef AVFORMAT_DV_H
#define AVFORMAT_DV_H

#include "avformat.h"

typedef struct DVDemuxContext DVDemuxContext;
DVDemuxContext* avpriv_dv_init_demux(AVFormatContext* s);
int avpriv_dv_get_packet(DVDemuxContext*, AVPacket *);
int avpriv_dv_produce_packet(DVDemuxContext*, AVPacket*, uint8_t*, int, int64_t);
void ff_dv_ts_reset(DVDemuxContext *c, int64_t ts_video);

#endif /* AVFORMAT_DV_H */
