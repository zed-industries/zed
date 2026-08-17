/*
 * PCM common functions
 * Copyright (C) 2007  Aurelien Jacobs <aurel@gnuage.org>
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

#ifndef AVFORMAT_PCM_H
#define AVFORMAT_PCM_H

#include "avformat.h"

int ff_pcm_default_packet_size(AVCodecParameters *par);
int ff_pcm_read_packet(AVFormatContext *s, AVPacket *pkt);
int ff_pcm_read_seek(AVFormatContext *s,
                     int stream_index, int64_t timestamp, int flags);

#endif /* AVFORMAT_PCM_H */
