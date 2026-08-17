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

#ifndef AVFORMAT_AVIO_INTERNAL_H
#define AVFORMAT_AVIO_INTERNAL_H

#include "avio.h"

#include "libavutil/log.h"

extern const AVClass ff_avio_class;

typedef struct FFIOContext {
    AVIOContext pub;
    /**
     * A callback that is used instead of short_seek_threshold.
     */
    int (*short_seek_get)(void *opaque);

    /**
     * Threshold to favor readahead over seek.
     */
    int short_seek_threshold;

    enum AVIODataMarkerType current_type;
    int64_t last_time;

    /**
     * max filesize, used to limit allocations
     */
    int64_t maxsize;

    /**
     * Bytes read statistic
     */
    int64_t bytes_read;

    /**
     * Bytes written statistic
     */
    int64_t bytes_written;

    /**
     * seek statistic
     */
    int seek_count;

    /**
     * writeout statistic
     */
    int writeout_count;

    /**
     * Original buffer size
     * used after probing to ensure seekback and to reset the buffer size
     */
    int orig_buffer_size;

    /**
     * Written output size
     * is updated each time a successful writeout ends up further position-wise
     */
    int64_t written_output_size;
} FFIOContext;

static av_always_inline FFIOContext *ffiocontext(AVIOContext *ctx)
{
    return (FFIOContext*)ctx;
}

void ffio_init_context(FFIOContext *s,
                  unsigned char *buffer,
                  int buffer_size,
                  int write_flag,
                  void *opaque,
                  int (*read_packet)(void *opaque, uint8_t *buf, int buf_size),
                  int (*write_packet)(void *opaque, const uint8_t *buf, int buf_size),
                  int64_t (*seek)(void *opaque, int64_t offset, int whence));

/**
 * Wrap a buffer in an AVIOContext for reading.
 */
void ffio_init_read_context(FFIOContext *s, const uint8_t *buffer, int buffer_size);

/**
 * Wrap a buffer in an AVIOContext for writing.
 */
void ffio_init_write_context(FFIOContext *s, uint8_t *buffer, int buffer_size);

/**
 * Read size bytes from AVIOContext, returning a pointer.
 * Note that the data pointed at by the returned pointer is only
 * valid until the next call that references the same IO context.
 * @param s IO context
 * @param buf pointer to buffer into which to assemble the requested
 *    data if it is not available in contiguous addresses in the
 *    underlying buffer
 * @param size number of bytes requested
 * @param data address at which to store pointer: this will be a
 *    a direct pointer into the underlying buffer if the requested
 *    number of bytes are available at contiguous addresses, otherwise
 *    will be a copy of buf
 * @return number of bytes read or AVERROR
 */
int ffio_read_indirect(AVIOContext *s, unsigned char *buf, int size, const unsigned char **data);

void ffio_fill(AVIOContext *s, int b, int64_t count);

static av_always_inline void ffio_wfourcc(AVIOContext *pb, const uint8_t *s)
{
    avio_wl32(pb, MKTAG(s[0], s[1], s[2], s[3]));
}

/**
 * Rewind the AVIOContext using the specified buffer containing the first buf_size bytes of the file.
 * Used after probing to avoid seeking.
 * Joins buf and s->buffer, taking any overlap into consideration.
 * @note s->buffer must overlap with buf or they can't be joined and the function fails
 *
 * @param s The read-only AVIOContext to rewind
 * @param buf The probe buffer containing the first buf_size bytes of the file
 * @param buf_size The size of buf
 * @return >= 0 in case of success, a negative value corresponding to an
 * AVERROR code in case of failure
 */
int ffio_rewind_with_probe_data(AVIOContext *s, unsigned char **buf, int buf_size);

uint64_t ffio_read_varlen(AVIOContext *bc);

/**
 * Read a unsigned integer coded as a variable number of up to eight
 * little-endian bytes, where the MSB in a byte signals another byte
 * must be read.
 * All coded bytes are read, but values > UINT_MAX are truncated.
 */
unsigned int ffio_read_leb(AVIOContext *s);

void ffio_write_leb(AVIOContext *s, unsigned val);

/**
 * Write a sequence of text lines, converting line endings.
 * All input line endings (LF, CRLF, CR) are converted to the configured line ending.
 * @param s The AVIOContext to write to
 * @param buf The buffer to write
 * @param size The size of the buffer, or <0 to use the full length of a null-terminated string
 * @param ending The line ending sequence to convert to, or NULL for \n
 */
void ffio_write_lines(AVIOContext *s, const unsigned char *buf, int size,
                      const unsigned char *ending);

/**
 * Read size bytes from AVIOContext into buf.
 * Check that exactly size bytes have been read.
 * @return number of bytes read or AVERROR
 */
int ffio_read_size(AVIOContext *s, unsigned char *buf, int size);

/**
 * Reallocate a given buffer for AVIOContext.
 *
 * @param s the AVIOContext to realloc.
 * @param buf_size required new buffer size.
 * @return 0 on success, a negative AVERROR on failure.
 */
int ffio_realloc_buf(AVIOContext *s, int buf_size);

/**
 * Ensures that the requested seekback buffer size will be available
 *
 * Will ensure that when reading sequentially up to buf_size, seeking
 * within the current pos and pos+buf_size is possible.
 * Once the stream position moves outside this window or another
 * ffio_ensure_seekback call requests a buffer outside this window this
 * guarantee is lost.
 */
int ffio_ensure_seekback(AVIOContext *s, int64_t buf_size);

int ffio_limit(AVIOContext *s, int size);

void ffio_init_checksum(AVIOContext *s,
                        unsigned long (*update_checksum)(unsigned long c, const uint8_t *p, unsigned int len),
                        unsigned long checksum);
unsigned long ffio_get_checksum(AVIOContext *s);
unsigned long ff_crc04C11DB7_update(unsigned long checksum, const uint8_t *buf,
                                    unsigned int len);
unsigned long ff_crcEDB88320_update(unsigned long checksum, const uint8_t *buf,
                                    unsigned int len);
unsigned long ff_crcA001_update(unsigned long checksum, const uint8_t *buf,
                                unsigned int len);

/**
 * Open a write only packetized memory stream with a maximum packet
 * size of 'max_packet_size'.  The stream is stored in a memory buffer
 * with a big-endian 4 byte header giving the packet size in bytes.
 *
 * @param s new IO context
 * @param max_packet_size maximum packet size (must be > 0)
 * @return zero if no error.
 */
int ffio_open_dyn_packet_buf(AVIOContext **s, int max_packet_size);

/**
 * Return the URLContext associated with the AVIOContext
 *
 * @param s IO context
 * @return pointer to URLContext or NULL.
 */
struct URLContext *ffio_geturlcontext(AVIOContext *s);

/**
 * Create and initialize a AVIOContext for accessing the
 * resource referenced by the URLContext h.
 * @note When the URLContext h has been opened in read+write mode, the
 * AVIOContext can be used only for writing.
 *
 * @param s Used to return the pointer to the created AVIOContext.
 * In case of failure the pointed to value is set to NULL.
 * @return >= 0 in case of success, a negative value corresponding to an
 * AVERROR code in case of failure
 */
int ffio_fdopen(AVIOContext **s, struct URLContext *h);


/**
 * Read url related dictionary options from the AVIOContext and write to the given dictionary
 */
int ffio_copy_url_options(AVIOContext* pb, AVDictionary** avio_opts);

/**
 * Open a write-only fake memory stream. The written data is not stored
 * anywhere - this is only used for measuring the amount of data
 * written.
 *
 * @param s new IO context
 * @return zero if no error.
 */
int ffio_open_null_buf(AVIOContext **s);

int ffio_open_whitelist(AVIOContext **s, const char *url, int flags,
                         const AVIOInterruptCB *int_cb, AVDictionary **options,
                         const char *whitelist, const char *blacklist);

/**
 * Close a null buffer.
 *
 * @param s an IO context opened by ffio_open_null_buf
 * @return the number of bytes written to the null buffer
 */
int ffio_close_null_buf(AVIOContext *s);

/**
 * Reset a dynamic buffer.
 *
 * Resets everything, but keeps the allocated buffer for later use.
 */
void ffio_reset_dyn_buf(AVIOContext *s);

/**
 * Free a dynamic buffer.
 *
 * @param s a pointer to an IO context opened by avio_open_dyn_buf()
 */
void ffio_free_dyn_buf(AVIOContext **s);

struct AVBPrint;
/**
 * Read a whole line of text from AVIOContext to an AVBPrint buffer overwriting
 * its contents. Stop reading after reaching a \\r, a \\n, a \\r\\n, a \\0 or
 * EOF. The line ending characters are NOT included in the buffer, but they
 * are skipped on the input.
 *
 * @param s the read-only AVIOContext
 * @param bp the AVBPrint buffer
 * @return the length of the read line not including the line endings,
 *         negative on error, or if the buffer becomes truncated.
 */
int64_t ff_read_line_to_bprint_overwrite(AVIOContext *s, struct AVBPrint *bp);

/**
 * Read a whole null-terminated string of text from AVIOContext to an AVBPrint
 * buffer overwriting its contents. Stop reading after reaching the maximum
 * length, a \\0 or EOF.
 *
 * @param s the read-only AVIOContext
 * @param bp the AVBPrint buffer
 * @param max_len the maximum length to be read from the AVIOContext.
 *                Negative (< 0) values signal that there is no known maximum
 *                length applicable. A maximum length of zero means that the
 *                AVIOContext is not touched, and the function returns
 *                with a read length of zero. In all cases the AVBprint
 *                is cleared.
 * @return the length of the read string not including the terminating null,
 *         negative on error, or if the buffer becomes truncated.
 */
int64_t ff_read_string_to_bprint_overwrite(AVIOContext *s, struct AVBPrint *bp,
                                           int64_t max_len);

#endif /* AVFORMAT_AVIO_INTERNAL_H */
