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

/**
 * @file
 * unbuffered private I/O API
 */

#ifndef AVFORMAT_URL_H
#define AVFORMAT_URL_H

#include "avio.h"

#include "libavutil/dict.h"
#include "libavutil/log.h"

#define URL_PROTOCOL_FLAG_NESTED_SCHEME 1 /*< The protocol name can be the first part of a nested protocol scheme */
#define URL_PROTOCOL_FLAG_NETWORK       2 /*< The protocol uses network */

typedef struct URLContext {
    const AVClass *av_class;    /**< information for av_log(). Set by url_open(). */
    const struct URLProtocol *prot;
    void *priv_data;
    char *filename;             /**< specified URL */
    int flags;
    int max_packet_size;        /**< if non zero, the stream is packetized with this max packet size */
    int is_streamed;            /**< true if streamed (no seek possible), default = false */
    int is_connected;
    AVIOInterruptCB interrupt_callback;
    int64_t rw_timeout;         /**< maximum time to wait for (network) read/write operation completion, in mcs */
    const char *protocol_whitelist;
    const char *protocol_blacklist;
    int min_packet_size;        /**< if non zero, the stream is packetized with this min packet size */
} URLContext;

typedef struct URLProtocol {
    const char *name;
    int     (*url_open)( URLContext *h, const char *url, int flags);
    /**
     * This callback is to be used by protocols which open further nested
     * protocols. options are then to be passed to ffurl_open_whitelist()
     * or ffurl_connect() for those nested protocols.
     */
    int     (*url_open2)(URLContext *h, const char *url, int flags, AVDictionary **options);
    int     (*url_accept)(URLContext *s, URLContext **c);
    int     (*url_handshake)(URLContext *c);

    /**
     * Read data from the protocol.
     * If data is immediately available (even less than size), EOF is
     * reached or an error occurs (including EINTR), return immediately.
     * Otherwise:
     * In non-blocking mode, return AVERROR(EAGAIN) immediately.
     * In blocking mode, wait for data/EOF/error with a short timeout (0.1s),
     * and return AVERROR(EAGAIN) on timeout.
     * Checking interrupt_callback, looping on EINTR and EAGAIN and until
     * enough data has been read is left to the calling function; see
     * retry_transfer_wrapper in avio.c.
     */
    int     (*url_read)( URLContext *h, unsigned char *buf, int size);
    int     (*url_write)(URLContext *h, const unsigned char *buf, int size);
    int64_t (*url_seek)( URLContext *h, int64_t pos, int whence);
    int     (*url_close)(URLContext *h);
    int (*url_read_pause)(void *urlcontext, int pause);
    int64_t (*url_read_seek)(void *urlcontext, int stream_index,
                             int64_t timestamp, int flags);
    int (*url_get_file_handle)(URLContext *h);
    int (*url_get_multi_file_handle)(URLContext *h, int **handles,
                                     int *numhandles);
    int (*url_get_short_seek)(URLContext *h);
    int (*url_shutdown)(URLContext *h, int flags);
    const AVClass *priv_data_class;
    int priv_data_size;
    int flags;
    int (*url_check)(URLContext *h, int mask);
    int (*url_open_dir)(URLContext *h);
    int (*url_read_dir)(URLContext *h, AVIODirEntry **next);
    int (*url_close_dir)(URLContext *h);
    int (*url_delete)(URLContext *h);
    int (*url_move)(URLContext *h_src, URLContext *h_dst);
    const char *default_whitelist;
} URLProtocol;

/**
 * Create a URLContext for accessing to the resource indicated by
 * url, but do not initiate the connection yet.
 *
 * @param puc pointer to the location where, in case of success, the
 * function puts the pointer to the created URLContext
 * @param flags flags which control how the resource indicated by url
 * is to be opened
 * @param int_cb interrupt callback to use for the URLContext, may be
 * NULL
 * @return >= 0 in case of success, a negative value corresponding to an
 * AVERROR code in case of failure
 */
int ffurl_alloc(URLContext **puc, const char *filename, int flags,
                const AVIOInterruptCB *int_cb);

/**
 * Connect an URLContext that has been allocated by ffurl_alloc
 *
 * @param options  A dictionary filled with options for nested protocols,
 * i.e. it will be passed to url_open2() for protocols implementing it.
 * This parameter will be destroyed and replaced with a dict containing options
 * that were not found. May be NULL.
 */
int ffurl_connect(URLContext *uc, AVDictionary **options);

/**
 * Create an URLContext for accessing to the resource indicated by
 * url, and open it.
 *
 * @param puc pointer to the location where, in case of success, the
 * function puts the pointer to the created URLContext
 * @param flags flags which control how the resource indicated by url
 * is to be opened
 * @param int_cb interrupt callback to use for the URLContext, may be
 * NULL
 * @param options  A dictionary filled with protocol-private options. On return
 * this parameter will be destroyed and replaced with a dict containing options
 * that were not found. May be NULL.
 * @param parent An enclosing URLContext, whose generic options should
 *               be applied to this URLContext as well.
 * @return >= 0 in case of success, a negative value corresponding to an
 * AVERROR code in case of failure
 */
int ffurl_open_whitelist(URLContext **puc, const char *filename, int flags,
               const AVIOInterruptCB *int_cb, AVDictionary **options,
               const char *whitelist, const char* blacklist,
               URLContext *parent);

/**
 * Accept an URLContext c on an URLContext s
 *
 * @param  s server context
 * @param  c client context, must be unallocated.
 * @return >= 0 on success, ff_neterrno() on failure.
 */
int ffurl_accept(URLContext *s, URLContext **c);

/**
 * Perform one step of the protocol handshake to accept a new client.
 * See avio_handshake() for details.
 * Implementations should try to return decreasing values.
 * If the protocol uses an underlying protocol, the underlying handshake is
 * usually the first step, and the return value can be:
 * (largest value for this protocol) + (return value from other protocol)
 *
 * @param  c the client context
 * @return >= 0 on success or a negative value corresponding
 *         to an AVERROR code on failure
 */
int ffurl_handshake(URLContext *c);

int ffurl_read2(void *urlcontext, uint8_t *buf, int size);
/**
 * Read up to size bytes from the resource accessed by h, and store
 * the read bytes in buf.
 *
 * @return The number of bytes actually read, or a negative value
 * corresponding to an AVERROR code in case of error. A value of zero
 * indicates that it is not possible to read more from the accessed
 * resource (except if the value of the size argument is also zero).
 */
static inline int ffurl_read(URLContext *h, uint8_t *buf, int size)
{
    return ffurl_read2(h, buf, size);
}

/**
 * Read as many bytes as possible (up to size), calling the
 * read function multiple times if necessary.
 * This makes special short-read handling in applications
 * unnecessary, if the return value is < size then it is
 * certain there was either an error or the end of file was reached.
 */
int ffurl_read_complete(URLContext *h, unsigned char *buf, int size);

int ffurl_write2(void *urlcontext, const uint8_t *buf, int size);
/**
 * Write size bytes from buf to the resource accessed by h.
 *
 * @return the number of bytes actually written, or a negative value
 * corresponding to an AVERROR code in case of failure
 */
static inline int ffurl_write(URLContext *h, const uint8_t *buf, int size)
{
    return ffurl_write2(h, buf, size);
}

int64_t ffurl_seek2(void *urlcontext, int64_t pos, int whence);
/**
 * Change the position that will be used by the next read/write
 * operation on the resource accessed by h.
 *
 * @param pos specifies the new position to set
 * @param whence specifies how pos should be interpreted, it must be
 * one of SEEK_SET (seek from the beginning), SEEK_CUR (seek from the
 * current position), SEEK_END (seek from the end), or AVSEEK_SIZE
 * (return the filesize of the requested resource, pos is ignored).
 * @return a negative value corresponding to an AVERROR code in case
 * of failure, or the resulting file position, measured in bytes from
 * the beginning of the file. You can use this feature together with
 * SEEK_CUR to read the current file position.
 */
static inline int64_t ffurl_seek(URLContext *h, int64_t pos, int whence)
{
    return ffurl_seek2(h, pos, whence);
}

/**
 * Close the resource accessed by the URLContext h, and free the
 * memory used by it. Also set the URLContext pointer to NULL.
 *
 * @return a negative value if an error condition occurred, 0
 * otherwise
 */
int ffurl_closep(URLContext **h);
int ffurl_close(URLContext *h);

/**
 * Return the filesize of the resource accessed by h, AVERROR(ENOSYS)
 * if the operation is not supported by h, or another negative value
 * corresponding to an AVERROR error code in case of failure.
 */
int64_t ffurl_size(URLContext *h);

/**
 * Return the file descriptor associated with this URL. For RTP, this
 * will return only the RTP file descriptor, not the RTCP file descriptor.
 *
 * @return the file descriptor associated with this URL, or <0 on error.
 */
int ffurl_get_file_handle(URLContext *h);

/**
 * Return the file descriptors associated with this URL.
 *
 * @return 0 on success or <0 on error.
 */
int ffurl_get_multi_file_handle(URLContext *h, int **handles, int *numhandles);

/**
 * Return the current short seek threshold value for this URL.
 *
 * @return threshold (>0) on success or <=0 on error.
 */
int ffurl_get_short_seek(void *urlcontext);

/**
 * Signal the URLContext that we are done reading or writing the stream.
 *
 * @param h pointer to the resource
 * @param flags flags which control how the resource indicated by url
 * is to be shutdown
 *
 * @return a negative value if an error condition occurred, 0
 * otherwise
 */
int ffurl_shutdown(URLContext *h, int flags);

/**
 * Check if the user has requested to interrupt a blocking function
 * associated with cb.
 */
int ff_check_interrupt(AVIOInterruptCB *cb);

/* udp.c */
int ff_udp_set_remote_url(URLContext *h, const char *uri);
int ff_udp_get_local_port(URLContext *h);

/**
 * Assemble a URL string from components. This is the reverse operation
 * of av_url_split.
 *
 * Note, this requires networking to be initialized, so the caller must
 * ensure ff_network_init has been called.
 *
 * @see av_url_split
 *
 * @param str the buffer to fill with the url
 * @param size the size of the str buffer
 * @param proto the protocol identifier, if null, the separator
 *              after the identifier is left out, too
 * @param authorization an optional authorization string, may be null.
 *                      An empty string is treated the same as a null string.
 * @param hostname the host name string
 * @param port the port number, left out from the string if negative
 * @param fmt a generic format string for everything to add after the
 *            host/port, may be null
 * @return the number of characters written to the destination buffer
 */
int ff_url_join(char *str, int size, const char *proto,
                const char *authorization, const char *hostname,
                int port, const char *fmt, ...) av_printf_format(7, 8);

/**
 * Convert a relative url into an absolute url, given a base url.
 *
 * @param buf the buffer where output absolute url is written
 * @param size the size of buf
 * @param base the base url, may be equal to buf.
 * @param rel the new url, which is interpreted relative to base
 * @param handle_dos_paths handle DOS paths for file or unspecified protocol
 */
int ff_make_absolute_url2(char *buf, int size, const char *base,
                         const char *rel, int handle_dos_paths);

/**
 * Convert a relative url into an absolute url, given a base url.
 *
 * Same as ff_make_absolute_url2 with handle_dos_paths being equal to
 * HAVE_DOS_PATHS config variable.
 */
int ff_make_absolute_url(char *buf, int size, const char *base,
                         const char *rel);

/**
 * Allocate directory entry with default values.
 *
 * @return entry or NULL on error
 */
AVIODirEntry *ff_alloc_dir_entry(void);

const AVClass *ff_urlcontext_child_class_iterate(void **iter);

/**
 * Construct a list of protocols matching a given whitelist and/or blacklist.
 *
 * @param whitelist a comma-separated list of allowed protocol names or NULL. If
 *                  this is a non-empty string, only protocols in this list will
 *                  be included.
 * @param blacklist a comma-separated list of forbidden protocol names or NULL.
 *                  If this is a non-empty string, all protocols in this list
 *                  will be excluded.
 *
 * @return a NULL-terminated array of matching protocols. The array must be
 * freed by the caller.
 */
const URLProtocol **ffurl_get_protocols(const char *whitelist,
                                        const char *blacklist);

typedef struct URLComponents {
    const char *url;        /**< whole URL, for reference */
    const char *scheme;     /**< possibly including lavf-specific options */
    const char *authority;  /**< "//" if it is a real URL */
    const char *userinfo;   /**< including final '@' if present */
    const char *host;
    const char *port;       /**< including initial ':' if present */
    const char *path;
    const char *query;      /**< including initial '?' if present */
    const char *fragment;   /**< including initial '#' if present */
    const char *end;
} URLComponents;

#define url_component_end_scheme      authority
#define url_component_end_authority   userinfo
#define url_component_end_userinfo    host
#define url_component_end_host        port
#define url_component_end_port        path
#define url_component_end_path        query
#define url_component_end_query       fragment
#define url_component_end_fragment    end
#define url_component_end_authority_full path

#define URL_COMPONENT_HAVE(uc, component) \
    ((uc).url_component_end_##component > (uc).component)

/**
 * Parse an URL to find the components.
 *
 * Each component runs until the start of the next component,
 * possibly including a mandatory delimiter.
 *
 * @param uc   structure to fill with pointers to the components.
 * @param url  URL to parse.
 * @param end  end of the URL, or NULL to parse to the end of string.
 *
 * @return  >= 0 for success or an AVERROR code, especially if the URL is
 *          malformed.
 */
int ff_url_decompose(URLComponents *uc, const char *url, const char *end);

/**
 * Move or rename a resource.
 *
 * @note url_src and url_dst should share the same protocol and authority.
 *
 * @param url_src url to resource to be moved
 * @param url_dst new url to resource if the operation succeeded
 * @return >=0 on success or negative on error.
 */
int ffurl_move(const char *url_src, const char *url_dst);

/**
 * Delete a resource.
 *
 * @param url resource to be deleted.
 * @return >=0 on success or negative on error.
 */
int ffurl_delete(const char *url);

#endif /* AVFORMAT_URL_H */
