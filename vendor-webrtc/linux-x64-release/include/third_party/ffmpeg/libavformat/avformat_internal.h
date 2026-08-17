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

/*
 * APIs internal to the generic avformat layer.
 *
 * MUST NOT be included by individual muxers or demuxers.
 */

#ifndef AVFORMAT_AVFORMAT_INTERNAL_H
#define AVFORMAT_AVFORMAT_INTERNAL_H

#include <stdint.h>

#include "avformat.h"
#include "internal.h"

typedef struct FormatContextInternal {
    FFFormatContext fc;

    union {
        // muxing only
        struct {
            /**
             * Whether or not avformat_init_output has already been called
             */
            int initialized;

            /**
             * Whether or not avformat_init_output fully initialized streams
             */
            int streams_initialized;


            /**
             * Number of streams relevant for interleaving.
             * Muxing only.
             */
            int nb_interleaved_streams;

            /**
             * The interleavement function in use. Always set.
             */
            int (*interleave_packet)(struct AVFormatContext *s, AVPacket *pkt,
                                     int flush, int has_packet);

#if FF_API_COMPUTE_PKT_FIELDS2
            int missing_ts_warning;
#endif
        };

        // demuxing only
        struct {
            /**
             * Raw packets from the demuxer, prior to parsing and decoding.
             * This buffer is used for buffering packets until the codec can
             * be identified, as parsing cannot be done without knowing the
             * codec.
             */
            PacketList raw_packet_buffer;

            /**
             * Sum of the size of packets in raw_packet_buffer, in bytes.
             */
            int raw_packet_buffer_size;

            /**
             * Packets split by the parser get queued here.
             */
            PacketList parse_queue;

            /**
             * Contexts and child contexts do not contain a metadata option
             */
            int metafree;

            /**
             * Set if chapter ids are strictly monotonic.
             */
            int chapter_ids_monotonic;
        };
    };

#if FF_API_LAVF_SHORTEST
    /**
     * Timestamp of the end of the shortest stream.
     */
    int64_t shortest_end;
#endif
} FormatContextInternal;

static av_always_inline FormatContextInternal *ff_fc_internal(AVFormatContext *s)
{
    return (FormatContextInternal*)s;
}

#define RELATIVE_TS_BASE (INT64_MAX - (1LL << 48))

static av_always_inline int is_relative(int64_t ts)
{
    return ts > (RELATIVE_TS_BASE - (1LL << 48));
}

/**
 * Wrap a given time stamp, if there is an indication for an overflow
 *
 * @param st stream
 * @param timestamp the time stamp to wrap
 * @return resulting time stamp
 */
int64_t ff_wrap_timestamp(const AVStream *st, int64_t timestamp);

typedef struct FFStreamGroup {
    /**
     * The public context.
     */
    AVStreamGroup pub;

    AVFormatContext *fmtctx;
} FFStreamGroup;

static av_always_inline FFStreamGroup *ffstreamgroup(AVStreamGroup *stg)
{
    return (FFStreamGroup*)stg;
}

static av_always_inline const FFStreamGroup *cffstreamgroup(const AVStreamGroup *stg)
{
    return (const FFStreamGroup*)stg;
}

void ff_flush_packet_queue(AVFormatContext *s);

const struct AVCodec *ff_find_decoder(AVFormatContext *s, const AVStream *st,
                                      enum AVCodecID codec_id);

/**
 * Frees a stream without modifying the corresponding AVFormatContext.
 * Must only be called if the latter doesn't matter or if the stream
 * is not yet attached to an AVFormatContext.
 */
void ff_free_stream(AVStream **st);

/**
 * Frees a stream group without modifying the corresponding AVFormatContext.
 * Must only be called if the latter doesn't matter or if the stream
 * is not yet attached to an AVFormatContext.
 */
void ff_free_stream_group(AVStreamGroup **pstg);

int ff_is_intra_only(enum AVCodecID id);

struct FFOutputFormat;
struct FFInputFormat;
void avpriv_register_devices(const struct FFOutputFormat * const o[],
                             const struct FFInputFormat * const i[]);

#endif // AVFORMAT_AVFORMAT_INTERNAL_H
