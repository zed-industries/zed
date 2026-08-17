/*
 * Copyright (c) 2012 Clément Bœsch
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

#ifndef AVFORMAT_SUBTITLES_H
#define AVFORMAT_SUBTITLES_H

#include <stdint.h>
#include <stddef.h>
#include "avformat.h"
#include "libavutil/bprint.h"
#include "avio_internal.h"

enum sub_sort {
    SUB_SORT_TS_POS = 0,    ///< sort by timestamps, then position
    SUB_SORT_POS_TS,        ///< sort by position, then timestamps
};

enum ff_utf_type {
    FF_UTF_8,       // or other 8 bit encodings
    FF_UTF16LE,
    FF_UTF16BE,
};

typedef struct {
    int type;
    AVIOContext *pb;
    unsigned char buf[8];
    int buf_pos, buf_len;
    FFIOContext buf_pb;
} FFTextReader;

/**
 * Initialize the FFTextReader from the given AVIOContext. This function will
 * read some bytes from pb, and test for UTF-8 or UTF-16 BOMs. Further accesses
 * to FFTextReader will read more data from pb.
 * If s is not NULL, the user will be warned if a UTF-16 conversion takes place.
 *
 * The purpose of FFTextReader is to transparently convert read data to UTF-8
 * if the stream had a UTF-16 BOM.
 *
 * @param s Pointer to provide av_log context
 * @param r object which will be initialized
 * @param pb stream to read from (referenced as long as FFTextReader is in use)
 */
void ff_text_init_avio(void *s, FFTextReader *r, AVIOContext *pb);

/**
 * Similar to ff_text_init_avio(), but sets it up to read from a bounded buffer.
 *
 * @param r object which will be initialized
 * @param buf buffer to read from (referenced as long as FFTextReader is in use)
 * @param size size of buf
 */
void ff_text_init_buf(FFTextReader *r, const void *buf, size_t size);

/**
 * Return the byte position of the next byte returned by ff_text_r8(). For
 * UTF-16 source streams, this will return the original position, but it will
 * be incorrect if a codepoint was only partially read with ff_text_r8().
 */
int64_t ff_text_pos(FFTextReader *r);

/**
 * Return the next byte. The return value is always 0 - 255. Returns 0 on EOF.
 * If the source stream is UTF-16, this reads from the stream converted to
 * UTF-8. On invalid UTF-16, 0 is returned.
 */
int ff_text_r8(FFTextReader *r);

/**
 * Return non-zero if EOF was reached.
 */
int ff_text_eof(FFTextReader *r);

/**
 * Like ff_text_r8(), but don't remove the byte from the buffer.
 */
int ff_text_peek_r8(FFTextReader *r);

/**
 * Read the given number of bytes (in UTF-8). On error or EOF, \0 bytes are
 * written.
 */
void ff_text_read(FFTextReader *r, char *buf, size_t size);

typedef struct {
    AVPacket **subs;         ///< array of subtitles packets
    int nb_subs;            ///< number of subtitles packets
    int allocated_size;     ///< allocated size for subs
    int current_sub_idx;    ///< current position for the read packet callback
    enum sub_sort sort;     ///< sort method to use when finalizing subtitles
    int keep_duplicates;    ///< set to 1 to keep duplicated subtitle events
} FFDemuxSubtitlesQueue;

/**
 * Insert a new subtitle event.
 *
 * @param event the subtitle line (not zero terminated) or NULL on not yet available event
 * @param len   the length of the event (in strlen() sense, so without '\0')
 * @param merge set to 1 if the current event should be concatenated with the
 *              previous one instead of adding a new entry, 0 otherwise
 */
AVPacket *ff_subtitles_queue_insert(FFDemuxSubtitlesQueue *q,
                                    const uint8_t *event, size_t len, int merge);

/**
 * Same as ff_subtitles_queue_insert but takes an AVBPrint input.
 * Avoids common errors like handling incomplete AVBPrint.
 */
AVPacket *ff_subtitles_queue_insert_bprint(FFDemuxSubtitlesQueue *q,
                                           const AVBPrint *event, int merge);
/**
 * Set missing durations, sort subtitles by PTS (and then byte position), and
 * drop duplicated events.
 */
void ff_subtitles_queue_finalize(void *log_ctx, FFDemuxSubtitlesQueue *q);

/**
 * Generic read_packet() callback for subtitles demuxers using this queue
 * system.
 */
int ff_subtitles_queue_read_packet(FFDemuxSubtitlesQueue *q, AVPacket *pkt);

/**
 * Update current_sub_idx to emulate a seek. Except the first parameter, it
 * matches FFInputFormat->read_seek2 prototypes.
 */
int ff_subtitles_queue_seek(FFDemuxSubtitlesQueue *q, AVFormatContext *s, int stream_index,
                            int64_t min_ts, int64_t ts, int64_t max_ts, int flags);

/**
 * Remove and destroy all the subtitles packets.
 */
void ff_subtitles_queue_clean(FFDemuxSubtitlesQueue *q);

int ff_subtitles_read_packet(AVFormatContext *s, AVPacket *pkt);

int ff_subtitles_read_seek(AVFormatContext *s, int stream_index,
                           int64_t min_ts, int64_t ts, int64_t max_ts, int flags);

int ff_subtitles_read_close(AVFormatContext *s);

/**
 * SMIL helper to load next chunk ("<...>" or untagged content) in buf.
 *
 * @param c cached character, to avoid a backward seek
 * @return size of chunk or error, e.g. AVERROR(ENOMEM) on incomplete buf
 */
int ff_smil_extract_next_text_chunk(FFTextReader *tr, AVBPrint *buf, char *c);

/**
 * SMIL helper to point on the value of an attribute in the given tag.
 *
 * @param s    SMIL tag ("<...>")
 * @param attr the attribute to look for
 */
const char *ff_smil_get_attr_ptr(const char *s, const char *attr);

/**
 * @brief Same as ff_subtitles_read_text_chunk(), but read from an AVIOContext.
 */
av_warn_unused_result
int ff_subtitles_read_chunk(AVIOContext *pb, AVBPrint *buf);

/**
 * @brief Read a subtitles chunk from FFTextReader.
 *
 * A chunk is defined by a multiline "event", ending with a second line break.
 * The trailing line breaks are trimmed. CRLF are supported.
 * Example: "foo\r\nbar\r\n\r\nnext" will print "foo\r\nbar" into buf, and pb
 * will focus on the 'n' of the "next" string.
 *
 * @param tr  I/O context
 * @param buf an initialized buf where the chunk is written
 * @return 0 on success, error value otherwise, e.g. AVERROR(ENOMEM) if buf is incomplete
 *
 * @note buf is cleared before writing into it.
 */
av_warn_unused_result
int ff_subtitles_read_text_chunk(FFTextReader *tr, AVBPrint *buf);

/**
 * Get the number of characters to increment to jump to the next line, or to
 * the end of the string.
 * The function handles the following line breaks schemes:
 * LF, CRLF (MS), or standalone CR (old MacOS).
 */
static av_always_inline int ff_subtitles_next_line(const char *ptr)
{
    int n = strcspn(ptr, "\r\n");
    ptr += n;
    while (*ptr == '\r') {
        ptr++;
        n++;
    }
    if (*ptr == '\n')
        n++;
    return n;
}

/**
 * Read a line of text. Discards line ending characters.
 * The function handles the following line breaks schemes:
 * LF, CRLF (MS), or standalone CR (old MacOS).
 *
 * Returns the number of bytes written to buf. Always writes a terminating 0,
 * similar as with snprintf.
 *
 * @note returns a negative error code if a \0 byte is found
 */
ptrdiff_t ff_subtitles_read_line(FFTextReader *tr, char *buf, size_t size);

#endif /* AVFORMAT_SUBTITLES_H */
