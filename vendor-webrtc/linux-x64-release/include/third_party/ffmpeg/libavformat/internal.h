/*
 * copyright (c) 2001 Fabrice Bellard
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

#ifndef AVFORMAT_INTERNAL_H
#define AVFORMAT_INTERNAL_H

#include <stdint.h>

#include "libavcodec/packet_internal.h"

#include "avformat.h"

#define MAX_URL_SIZE 4096

/** size of probe buffer, for guessing file type from file contents */
#define PROBE_BUF_MIN 2048
#define PROBE_BUF_MAX (1 << 20)

#ifdef DEBUG
#    define hex_dump_debug(class, buf, size) av_hex_dump_log(class, AV_LOG_DEBUG, buf, size)
#else
#    define hex_dump_debug(class, buf, size) do { if (0) av_hex_dump_log(class, AV_LOG_DEBUG, buf, size); } while(0)
#endif

typedef struct AVCodecTag {
    enum AVCodecID id;
    unsigned int tag;
} AVCodecTag;

typedef struct CodecMime{
    char str[32];
    enum AVCodecID id;
} CodecMime;

/*************************************************/
/* fractional numbers for exact pts handling */

/**
 * The exact value of the fractional number is: 'val + num / den'.
 * num is assumed to be 0 <= num < den.
 */
typedef struct FFFrac {
    int64_t val, num, den;
} FFFrac;


typedef struct FFFormatContext {
    /**
     * The public context.
     */
    AVFormatContext pub;

    /**
     * Whether the timestamp shift offset has already been determined.
     * -1: disabled, 0: not yet determined, 1: determined.
     */
    enum {
        AVOID_NEGATIVE_TS_DISABLED = -1,
        AVOID_NEGATIVE_TS_UNKNOWN  = 0,
        AVOID_NEGATIVE_TS_KNOWN    = 1,
    } avoid_negative_ts_status;
#define AVOID_NEGATIVE_TS_ENABLED(status) ((status) >= 0)

    /**
     * This buffer is only needed when packets were already buffered but
     * not decoded, for example to get the codec parameters in MPEG
     * streams.
     */
    PacketList packet_buffer;

    /* av_seek_frame() support */
    int64_t data_offset; /**< offset of the first packet */

    /**
     * The generic code uses this as a temporary packet
     * to parse packets or for muxing, especially flushing.
     * For demuxers, it may also be used for other means
     * for short periods that are guaranteed not to overlap
     * with calls to av_read_frame() (or ff_read_packet())
     * or with each other.
     * It may be used by demuxers as a replacement for
     * stack packets (unless they call one of the aforementioned
     * functions with their own AVFormatContext).
     * Every user has to ensure that this packet is blank
     * after using it.
     */
    AVPacket *parse_pkt;

    /**
     * Used to hold temporary packets for the generic demuxing code.
     * When muxing, it may be used by muxers to hold packets (even
     * permanent ones).
     */
    AVPacket *pkt;

#if FF_API_AVSTREAM_SIDE_DATA
    int inject_global_side_data;
#endif

    int avoid_negative_ts_use_pts;

    /**
     * ID3v2 tag useful for MP3 demuxing
     */
    AVDictionary *id3v2_meta;

    int missing_streams;
} FFFormatContext;

static av_always_inline FFFormatContext *ffformatcontext(AVFormatContext *s)
{
    return (FFFormatContext*)s;
}

typedef struct FFStream {
    /**
     * The public context.
     */
    AVStream pub;

    AVFormatContext *fmtctx;
    /**
     * Set to 1 if the codec allows reordering, so pts can be different
     * from dts.
     */
    int reorder;

    /**
     * bitstream filter to run on stream
     * - encoding: Set by muxer using ff_stream_add_bitstream_filter
     * - decoding: unused
     */
    struct AVBSFContext *bsfc;

    /**
     * Whether or not check_bitstream should still be run on each packet
     */
    int bitstream_checked;

    /**
     * The codec context used by avformat_find_stream_info, the parser, etc.
     */
    struct AVCodecContext *avctx;
    /**
     * 1 if avctx has been initialized with the values from the codec parameters
     */
    int avctx_inited;

    /* the context for extracting extradata in find_stream_info()
     * inited=1/bsf=NULL signals that extracting is not possible (codec not
     * supported) */
    struct {
        struct AVBSFContext *bsf;
        int inited;
    } extract_extradata;

    /**
     * Whether the internal avctx needs to be updated from codecpar (after a late change to codecpar)
     */
    int need_context_update;

    int is_intra_only;

    FFFrac priv_pts;

    /**
     * Stream information used internally by avformat_find_stream_info()
     */
    struct FFStreamInfo *info;

    AVIndexEntry *index_entries; /**< Only used if the format does not
                                    support seeking natively. */
    int nb_index_entries;
    unsigned int index_entries_allocated_size;

    int64_t interleaver_chunk_size;
    int64_t interleaver_chunk_duration;

    /**
     * stream probing state
     * -1   -> probing finished
     *  0   -> no probing requested
     * rest -> perform probing with request_probe being the minimum score to accept.
     */
    int request_probe;
    /**
     * Indicates that everything up to the next keyframe
     * should be discarded.
     */
    int skip_to_keyframe;

    /**
     * Number of samples to skip at the start of the frame decoded from the next packet.
     */
    int skip_samples;

    /**
     * If not 0, the number of samples that should be skipped from the start of
     * the stream (the samples are removed from packets with pts==0, which also
     * assumes negative timestamps do not happen).
     * Intended for use with formats such as mp3 with ad-hoc gapless audio
     * support.
     */
    int64_t start_skip_samples;

    /**
     * If not 0, the first audio sample that should be discarded from the stream.
     * This is broken by design (needs global sample count), but can't be
     * avoided for broken by design formats such as mp3 with ad-hoc gapless
     * audio support.
     */
    int64_t first_discard_sample;

    /**
     * The sample after last sample that is intended to be discarded after
     * first_discard_sample. Works on frame boundaries only. Used to prevent
     * early EOF if the gapless info is broken (considered concatenated mp3s).
     */
    int64_t last_discard_sample;

    /**
     * Number of internally decoded frames, used internally in libavformat, do not access
     * its lifetime differs from info which is why it is not in that structure.
     */
    int nb_decoded_frames;

    /**
     * Timestamp offset added to timestamps before muxing
     */
    int64_t mux_ts_offset;

    /**
     * This is the lowest ts allowed in this track; it may be set by the muxer
     * during init or write_header and influences the automatic timestamp
     * shifting code.
     */
    int64_t lowest_ts_allowed;

    /**
     * Internal data to check for wrapping of the time stamp
     */
    int64_t pts_wrap_reference;

    /**
     * Options for behavior, when a wrap is detected.
     *
     * Defined by AV_PTS_WRAP_ values.
     *
     * If correction is enabled, there are two possibilities:
     * If the first time stamp is near the wrap point, the wrap offset
     * will be subtracted, which will create negative time stamps.
     * Otherwise the offset will be added.
     */
    int pts_wrap_behavior;

    /**
     * Internal data to prevent doing update_initial_durations() twice
     */
    int update_initial_durations_done;

#define MAX_REORDER_DELAY 16

    /**
     * Internal data to generate dts from pts
     */
    int64_t pts_reorder_error[MAX_REORDER_DELAY+1];
    uint8_t pts_reorder_error_count[MAX_REORDER_DELAY+1];

    int64_t pts_buffer[MAX_REORDER_DELAY+1];

    /**
     * Internal data to analyze DTS and detect faulty mpeg streams
     */
    int64_t last_dts_for_order_check;
    uint8_t dts_ordered;
    uint8_t dts_misordered;

#if FF_API_AVSTREAM_SIDE_DATA
    /**
     * Internal data to inject global side data
     */
    int inject_global_side_data;
#endif

    /**
     * display aspect ratio (0 if unknown)
     * - encoding: unused
     * - decoding: Set by libavformat to calculate sample_aspect_ratio internally
     */
    AVRational display_aspect_ratio;

    AVProbeData probe_data;

    /**
     * last packet in packet_buffer for this stream when muxing.
     */
    PacketListEntry *last_in_packet_buffer;

    int64_t last_IP_pts;
    int last_IP_duration;

    /**
     * Number of packets to buffer for codec probing
     */
    int probe_packets;

    /* av_read_frame() support */
    enum AVStreamParseType need_parsing;
    struct AVCodecParserContext *parser;

    /**
     * Number of frames that have been demuxed during avformat_find_stream_info()
     */
    int codec_info_nb_frames;

    /**
     * Stream Identifier
     * This is the MPEG-TS stream identifier +1
     * 0 means unknown
     */
    int stream_identifier;

    // Timestamp generation support:
    /**
     * Timestamp corresponding to the last dts sync point.
     *
     * Initialized when AVCodecParserContext.dts_sync_point >= 0 and
     * a DTS is received from the underlying container. Otherwise set to
     * AV_NOPTS_VALUE by default.
     */
    int64_t first_dts;
    int64_t cur_dts;

    const struct AVCodecDescriptor *codec_desc;

#if FF_API_INTERNAL_TIMING
    AVRational transferred_mux_tb;
#endif
} FFStream;

static av_always_inline FFStream *ffstream(AVStream *st)
{
    return (FFStream*)st;
}

static av_always_inline const FFStream *cffstream(const AVStream *st)
{
    return (const FFStream*)st;
}

#ifdef __GNUC__
#define dynarray_add(tab, nb_ptr, elem)\
do {\
    __typeof__(tab) _tab = (tab);\
    __typeof__(elem) _elem = (elem);\
    (void)sizeof(**_tab == _elem); /* check that types are compatible */\
    av_dynarray_add(_tab, nb_ptr, _elem);\
} while(0)
#else
#define dynarray_add(tab, nb_ptr, elem)\
do {\
    av_dynarray_add((tab), nb_ptr, (elem));\
} while(0)
#endif

/**
 * Automatically create sub-directories
 *
 * @param path will create sub-directories by path
 * @return 0, or < 0 on error
 */
int ff_mkdir_p(const char *path);

/**
 * Write hexadecimal string corresponding to given binary data. The string
 * is zero-terminated.
 *
 * @param buf       the output string is written here;
 *                  needs to be at least 2 * size + 1 bytes long.
 * @param src       the input data to be transformed.
 * @param size      the size (in byte) of src.
 * @param lowercase determines whether to use the range [0-9a-f] or [0-9A-F].
 * @return buf.
 */
char *ff_data_to_hex(char *buf, const uint8_t *src, int size, int lowercase);

/**
 * Parse a string of hexadecimal strings. Any space between the hexadecimal
 * digits is ignored.
 *
 * @param data if non-null, the parsed data is written to this pointer
 * @param p the string to parse
 * @return the number of bytes written (or to be written, if data is null)
 */
int ff_hex_to_data(uint8_t *data, const char *p);

#define NTP_OFFSET 2208988800ULL
#define NTP_OFFSET_US (NTP_OFFSET * 1000000ULL)

/** Get the current time since NTP epoch in microseconds. */
uint64_t ff_ntp_time(void);

/**
 * Get the NTP time stamp formatted as per the RFC-5905.
 *
 * @param ntp_time NTP time in micro seconds (since NTP epoch)
 * @return the formatted NTP time stamp
 */
uint64_t ff_get_formatted_ntp_time(uint64_t ntp_time_us);

/**
 * Parse the NTP time in micro seconds (since NTP epoch).
 *
 * @param ntp_ts NTP time stamp formatted as per the RFC-5905.
 * @return the time in micro seconds (since NTP epoch)
 */
uint64_t ff_parse_ntp_time(uint64_t ntp_ts);

/**
 * Append the media-specific SDP fragment for the media stream c
 * to the buffer buff.
 *
 * Note, the buffer needs to be initialized, since it is appended to
 * existing content.
 *
 * @param buff the buffer to append the SDP fragment to
 * @param size the size of the buff buffer
 * @param st the AVStream of the media to describe
 * @param idx the global stream index
 * @param dest_addr the destination address of the media stream, may be NULL
 * @param dest_type the destination address type, may be NULL
 * @param port the destination port of the media stream, 0 if unknown
 * @param ttl the time to live of the stream, 0 if not multicast
 * @param fmt the AVFormatContext, which might contain options modifying
 *            the generated SDP
 * @return 0 on success, a negative error code on failure
 */
int ff_sdp_write_media(char *buff, int size, const AVStream *st, int idx,
                       const char *dest_addr, const char *dest_type,
                       int port, int ttl, AVFormatContext *fmt);

/**
 * Read a whole line of text from AVIOContext. Stop reading after reaching
 * either a \\n, a \\0 or EOF. The returned string is always \\0-terminated,
 * and may be truncated if the buffer is too small.
 *
 * @param s the read-only AVIOContext
 * @param buf buffer to store the read line
 * @param maxlen size of the buffer
 * @return the length of the string written in the buffer, not including the
 *         final \\0
 */
int ff_get_line(AVIOContext *s, char *buf, int maxlen);

/**
 * Same as ff_get_line but strip the white-space characters in the text tail
 *
 * @param s the read-only AVIOContext
 * @param buf buffer to store the read line
 * @param maxlen size of the buffer
 * @return the length of the string written in the buffer
 */
int ff_get_chomp_line(AVIOContext *s, char *buf, int maxlen);

#define SPACE_CHARS " \t\r\n"

/**
 * Callback function type for ff_parse_key_value.
 *
 * @param key a pointer to the key
 * @param key_len the number of bytes that belong to the key, including the '='
 *                char
 * @param dest return the destination pointer for the value in *dest, may
 *             be null to ignore the value
 * @param dest_len the length of the *dest buffer
 */
typedef void (*ff_parse_key_val_cb)(void *context, const char *key,
                                    int key_len, char **dest, int *dest_len);
/**
 * Parse a string with comma-separated key=value pairs. The value strings
 * may be quoted and may contain escaped characters within quoted strings.
 *
 * @param str the string to parse
 * @param callback_get_buf function that returns where to store the
 *                         unescaped value string.
 * @param context the opaque context pointer to pass to callback_get_buf
 */
void ff_parse_key_value(const char *str, ff_parse_key_val_cb callback_get_buf,
                        void *context);

enum AVCodecID ff_guess_image2_codec(const char *filename);

/**
 * Set the time base and wrapping info for a given stream. This will be used
 * to interpret the stream's timestamps. If the new time base is invalid
 * (numerator or denominator are non-positive), it leaves the stream
 * unchanged.
 *
 * @param st stream
 * @param pts_wrap_bits number of bits effectively used by the pts
 *        (used for wrap control)
 * @param pts_num time base numerator
 * @param pts_den time base denominator
 */
void avpriv_set_pts_info(AVStream *st, int pts_wrap_bits,
                         unsigned int pts_num, unsigned int pts_den);

/**
 * Set the timebase for each stream from the corresponding codec timebase and
 * print it.
 */
int ff_framehash_write_header(AVFormatContext *s);

/**
 * Remove a stream from its AVFormatContext and free it.
 * The stream must be the last stream of the AVFormatContext.
 */
void ff_remove_stream(AVFormatContext *s, AVStream *st);

/**
 * Remove a stream group from its AVFormatContext and free it.
 * The stream group must be the last stream group of the AVFormatContext.
 */
void ff_remove_stream_group(AVFormatContext *s, AVStreamGroup *stg);

unsigned int ff_codec_get_tag(const AVCodecTag *tags, enum AVCodecID id);

enum AVCodecID ff_codec_get_id(const AVCodecTag *tags, unsigned int tag);

/**
 * Select a PCM codec based on the given parameters.
 *
 * @param bps     bits-per-sample
 * @param flt     floating-point
 * @param be      big-endian
 * @param sflags  signed flags. each bit corresponds to one byte of bit depth.
 *                e.g. the 1st bit indicates if 8-bit should be signed or
 *                unsigned, the 2nd bit indicates if 16-bit should be signed or
 *                unsigned, etc... This is useful for formats such as WAVE where
 *                only 8-bit is unsigned and all other bit depths are signed.
 * @return        a PCM codec id or AV_CODEC_ID_NONE
 */
enum AVCodecID ff_get_pcm_codec_id(int bps, int flt, int be, int sflags);

/**
 * Create a new stream and copy to it all parameters from a source stream, with
 * the exception of the index field, which is set when the new stream is
 * created.
 *
 * @param dst_ctx pointer to the context in which the new stream is created
 * @param src pointer to source AVStream
 * @return pointer to the new stream or NULL on error
 */
AVStream *ff_stream_clone(AVFormatContext *dst_ctx, const AVStream *src);

/**
 * Wrap ffurl_move() and log if error happens.
 *
 * @param url_src source path
 * @param url_dst destination path
 * @return        0 or AVERROR on failure
 */
int ff_rename(const char *url_src, const char *url_dst, void *logctx);

/**
 * Allocate extradata with additional AV_INPUT_BUFFER_PADDING_SIZE at end
 * which is always set to 0.
 *
 * Previously allocated extradata in par will be freed.
 *
 * @param size size of extradata
 * @return 0 if OK, AVERROR_xxx on error
 */
int ff_alloc_extradata(AVCodecParameters *par, int size);

/**
 * Copies the whilelists from one context to the other
 */
int ff_copy_whiteblacklists(AVFormatContext *dst, const AVFormatContext *src);

/*
 * A wrapper around AVFormatContext.io_close that should be used
 * instead of calling the pointer directly.
 *
 * @param s AVFormatContext
 * @param *pb the AVIOContext to be closed and freed. Can be NULL.
 * @return >=0 on success, negative AVERROR in case of failure
 */
int ff_format_io_close(AVFormatContext *s, AVIOContext **pb);

/**
 * Utility function to check if the file uses http or https protocol
 *
 * @param s AVFormatContext
 * @param filename URL or file name to open for writing
 */
int ff_is_http_proto(const char *filename);

struct AVBPrint;
/**
 * Finalize buf into extradata and set its size appropriately.
 */
int ff_bprint_to_codecpar_extradata(AVCodecParameters *par, struct AVBPrint *buf);

/**
 * Set AVFormatContext url field to the provided pointer. The pointer must
 * point to a valid string. The existing url field is freed if necessary. Also
 * set the legacy filename field to the same string which was provided in url.
 */
void ff_format_set_url(AVFormatContext *s, char *url);

/**
 * Return a positive value if the given url has one of the given
 * extensions, negative AVERROR on error, 0 otherwise.
 *
 * @param url        url to check against the given extensions
 * @param extensions a comma-separated list of filename extensions
 */
int ff_match_url_ext(const char *url, const char *extensions);

/**
 * Return in 'buf' the path with '%d' replaced by a number.
 *
 * Also handles the '%0nd' format where 'n' is the total number
 * of digits and '%%'.
 *
 * @param buf destination buffer
 * @param buf_size destination buffer size
 * @param path path with substitution template
 * @param number the number to substitute
 * @param flags AV_FRAME_FILENAME_FLAGS_*
 * @return 0 if OK, -1 on format error
 */
int ff_get_frame_filename(char *buf, int buf_size, const char *path,
                          int64_t number, int flags);

#endif /* AVFORMAT_INTERNAL_H */
