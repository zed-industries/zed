/*
 * Apple HTTP Live Streaming segmenter
 * Copyright (c) 2012, Luca Barbato
 * Copyright (c) 2017 Akamai Technologies, Inc.
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

#ifndef AVFORMAT_HLSPLAYLIST_H
#define AVFORMAT_HLSPLAYLIST_H

#include <stdint.h>

#include "avformat.h"
#include "avio.h"

typedef enum {
    PLAYLIST_TYPE_NONE,
    PLAYLIST_TYPE_EVENT,
    PLAYLIST_TYPE_VOD,
    PLAYLIST_TYPE_NB,
} PlaylistType;

void ff_hls_write_playlist_version(AVIOContext *out, int version);
void ff_hls_write_audio_rendition(AVIOContext *out, const char *agroup,
                                  const char *filename, const char *language,
                                  int name_id, int is_default, int nb_channels);
void ff_hls_write_subtitle_rendition(AVIOContext *out, const char *sgroup,
                                     const char *filename, const char *language,
                                     const char *sname, int name_id, int is_default);
void ff_hls_write_stream_info(AVStream *st, AVIOContext *out, int bandwidth,
                              int avg_bandwidth,
                              const char *filename, const char *agroup,
                              const char *codecs, const char *ccgroup,
                              const char *sgroup);
void ff_hls_write_playlist_header(AVIOContext *out, int version, int allowcache,
                                  int target_duration, int64_t sequence,
                                  uint32_t playlist_type, int iframe_mode);
void ff_hls_write_init_file(AVIOContext *out, const char *filename,
                            int byterange_mode, int64_t size, int64_t pos);
int ff_hls_write_file_entry(AVIOContext *out, int insert_discont,
                            int byterange_mode, double duration,
                            int round_duration, int64_t size,
                            int64_t pos /* Used only if HLS_SINGLE_FILE flag is set */,
                            const char *baseurl /* Ignored if NULL */,
                            const char *filename, double *prog_date_time,
                            int64_t video_keyframe_size, int64_t video_keyframe_pos,
                            int iframe_mode);
void ff_hls_write_end_list (AVIOContext *out);

#endif /* AVFORMAT_HLSPLAYLIST_H_ */
