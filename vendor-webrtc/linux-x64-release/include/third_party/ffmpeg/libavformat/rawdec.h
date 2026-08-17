/*
 * RAW demuxers
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

#ifndef AVFORMAT_RAWDEC_H
#define AVFORMAT_RAWDEC_H

#include "avformat.h"
#include "demux.h"
#include "libavutil/log.h"

typedef struct FFRawVideoDemuxerContext {
    const AVClass *class;     /**< Class for private options. */
    int raw_packet_size;
    char *video_size;         /**< String describing video size, set by a private option. */
    char *pixel_format;       /**< Set by a private option. */
    AVRational framerate;     /**< AVRational describing framerate, set by a private option. */
} FFRawVideoDemuxerContext;

typedef struct FFRawDemuxerContext {
    const AVClass *class;     /**< Class for private options. */
    int raw_packet_size;
} FFRawDemuxerContext;

extern const AVClass ff_rawvideo_demuxer_class;
extern const AVClass ff_raw_demuxer_class;

int ff_raw_read_partial_packet(AVFormatContext *s, AVPacket *pkt);

int ff_raw_audio_read_header(AVFormatContext *s);

int ff_raw_video_read_header(AVFormatContext *s);

int ff_raw_subtitle_read_header(AVFormatContext *s);

#define FF_DEF_RAWVIDEO_DEMUXER2(shortname, longname, probe, ext, id, flag)\
const FFInputFormat ff_ ## shortname ## _demuxer = {\
    .p.name         = #shortname,\
    .p.long_name    = NULL_IF_CONFIG_SMALL(longname),\
    .p.extensions   = ext,\
    .p.flags        = flag | AVFMT_NOTIMESTAMPS,\
    .p.priv_class   = &ff_rawvideo_demuxer_class,\
    .read_probe     = probe,\
    .read_header    = ff_raw_video_read_header,\
    .read_packet    = ff_raw_read_partial_packet,\
    .raw_codec_id   = id,\
    .priv_data_size = sizeof(FFRawVideoDemuxerContext),\
};

#define FF_DEF_RAWVIDEO_DEMUXER(shortname, longname, probe, ext, id)\
FF_DEF_RAWVIDEO_DEMUXER2(shortname, longname, probe, ext, id, AVFMT_GENERIC_INDEX)

#define FF_DEF_RAWSUB_DEMUXER(shortname, longname, probe, ext, id, flag)\
const FFInputFormat ff_ ## shortname ## _demuxer = {\
    .p.name         = #shortname,\
    .p.long_name    = NULL_IF_CONFIG_SMALL(longname),\
    .p.extensions   = ext,\
    .p.flags        = flag,\
    .p.priv_class   = &ff_raw_demuxer_class,\
    .read_probe     = probe,\
    .read_header    = ff_raw_subtitle_read_header,\
    .read_packet    = ff_raw_read_partial_packet,\
    .raw_codec_id   = id,\
    .priv_data_size = sizeof(FFRawDemuxerContext),\
};

#endif /* AVFORMAT_RAWDEC_H */
