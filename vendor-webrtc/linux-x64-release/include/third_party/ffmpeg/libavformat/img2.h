/*
 * Image format
 * Copyright (c) 2014 Michael Niedermayer
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

#ifndef AVFORMAT_IMG2_H
#define AVFORMAT_IMG2_H

#include <stdint.h>
#include "avformat.h"
#include "libavutil/opt.h"

#if HAVE_GLOB
#include <glob.h>
#endif

enum PatternType {
    PT_GLOB_SEQUENCE,
    PT_GLOB,
    PT_SEQUENCE,
    PT_NONE,
    PT_DEFAULT
};

typedef struct VideoDemuxData {
    const AVClass *class;  /**< Class for private options. */
    int img_first;
    int img_last;
    int img_number;
    int64_t pts;
    int img_count;
    int is_pipe;
    int split_planes;       /**< use independent file for each Y, U, V plane */
    char path[1024];
    char *pixel_format;     /**< Set by a private option. */
    int width, height;      /**< Set by a private option. */
    AVRational framerate;   /**< Set by a private option. */
    int loop;
    int pattern_type; /**< PatternType */
    int use_glob;
#if HAVE_GLOB
    glob_t globstate;
#endif
    int start_number;
    int start_number_range;
    int frame_size;
    int ts_from_file;
    int export_path_metadata; /**< enabled when set to 1. */
} VideoDemuxData;

typedef struct IdStrMap {
    enum AVCodecID id;
    char str[12];
} IdStrMap;

extern const IdStrMap ff_img_tags[];

extern const AVOption ff_img_options[];

int ff_img_read_header(AVFormatContext *s1);

int ff_img_read_packet(AVFormatContext *s1, AVPacket *pkt);
#endif
