// Copyright 2016 The Chromium Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef COMPONENTS_GRPC_SUPPORT_INCLUDE_BIDIRECTIONAL_STREAM_C_H_
#define COMPONENTS_GRPC_SUPPORT_INCLUDE_BIDIRECTIONAL_STREAM_C_H_

#if defined(WIN32)
#define GRPC_SUPPORT_EXPORT
#else
#define GRPC_SUPPORT_EXPORT __attribute__((visibility("default")))
#endif

#ifdef __cplusplus
extern "C" {
#endif

#include <stddef.h>

/* Engine API. */

/* Opaque object representing a Bidirectional stream creating engine. Created
 * and configured outside of this API to facilitate sharing with other
 * components */
typedef struct stream_engine {
  void* obj;
  void* annotation;
} stream_engine;

/* Bidirectional Stream API */

/* Opaque object representing Bidirectional Stream. */
typedef struct bidirectional_stream {
  void* obj;
  void* annotation;
} bidirectional_stream;

/* A single request or response header element. */
typedef struct bidirectional_stream_header {
  const char* key;
  const char* value;
} bidirectional_stream_header;

/* Array of request or response headers or trailers. */
typedef struct bidirectional_stream_header_array {
  size_t count;
  size_t capacity;
  bidirectional_stream_header* headers;
} bidirectional_stream_header_array;

/* Set of callbacks used to receive callbacks from bidirectional stream. */
typedef struct bidirectional_stream_callback {
  /* Invoked when the stream is ready for reading and writing.
   * Consumer may call bidirectional_stream_read() to start reading data.
   * Consumer may call bidirectional_stream_write() to start writing
   * data.
   */
  void (*on_stream_ready)(bidirectional_stream* stream);

  /* Invoked when initial response headers are received.
   * Consumer must call bidirectional_stream_read() to start reading.
   * Consumer may call bidirectional_stream_write() to start writing or
   * close the stream. Contents of |headers| is valid for duration of the call.
   */
  void (*on_response_headers_received)(
      bidirectional_stream* stream,
      const bidirectional_stream_header_array* headers,
      const char* negotiated_protocol);

  /* Invoked when data is read into the buffer passed to
   * bidirectional_stream_read(). Only part of the buffer may be
   * populated. To continue reading, call bidirectional_stream_read().
   * It may be invoked after on_response_trailers_received()}, if there was
   * pending read data before trailers were received.
   *
   * If |bytes_read| is 0, it means the remote side has signaled that it will
   * send no more data; future calls to bidirectional_stream_read()
   * will result in the on_data_read() callback or on_succeded() callback if
   * bidirectional_stream_write() was invoked with end_of_stream set to
   * true.
   */
  void (*on_read_completed)(bidirectional_stream* stream,
                            char* data,
                            int bytes_read);

  /**
   * Invoked when all data passed to bidirectional_stream_write() is
   * sent. To continue writing, call bidirectional_stream_write().
   */
  void (*on_write_completed)(bidirectional_stream* stream, const char* data);

  /* Invoked when trailers are received before closing the stream. Only invoked
   * when server sends trailers, which it may not. May be invoked while there is
   * read data remaining in local buffer. Contents of |trailers| is valid for
   * duration of the call.
   */
  void (*on_response_trailers_received)(
      bidirectional_stream* stream,
      const bidirectional_stream_header_array* trailers);

  /**
   * Invoked when there is no data to be read or written and the stream is
   * closed successfully remotely and locally. Once invoked, no further callback
   * methods will be invoked.
   */
  void (*on_succeded)(bidirectional_stream* stream);

  /**
   * Invoked if the stream failed for any reason after
   * bidirectional_stream_start(). HTTP/2 error codes are
   * mapped to chrome net error codes. Once invoked, no further callback methods
   * will be invoked.
   */
  void (*on_failed)(bidirectional_stream* stream, int net_error);

  /**
   * Invoked if the stream was canceled via
   * bidirectional_stream_cancel(). Once invoked, no further callback
   * methods will be invoked.
   */
  void (*on_canceled)(bidirectional_stream* stream);
} bidirectional_stream_callback;

/* Creates a new stream object that uses |engine| and |callback|. All stream
 * tasks are performed asynchronously on the |engine| network thread. |callback|
 * methods are invoked synchronously on the |engine| network thread, but must
 * not run tasks on the current thread to prevent blocking networking operations
 * and causing exceptions during shutdown. The |annotation| is stored in
 * bidirectional stream for arbitrary use by application.
 *
 * Returned |bidirectional_stream*| is owned by the caller, and must be
 * destroyed using |bidirectional_stream_destroy|.
 *
 * Both |calback| and |engine| must remain valid until stream is destroyed.
 */
GRPC_SUPPORT_EXPORT
bidirectional_stream* bidirectional_stream_create(
    stream_engine* engine,
    void* annotation,
    bidirectional_stream_callback* callback);

/* TBD: The following methods return int. Should it be a custom type? */

/* Destroys stream object. Destroy could be called from any thread, including
 * network thread, but is posted, so |stream| is valid until calling task is
 * complete.
 */
GRPC_SUPPORT_EXPORT
int bidirectional_stream_destroy(bidirectional_stream* stream);

/**
 * Disables or enables auto flush. By default, data is flushed after
 * every bidirectional_stream_write(). If the auto flush is disabled,
 * the client should explicitly call bidirectional_stream_flush to flush
 * the data.
 */
GRPC_SUPPORT_EXPORT void bidirectional_stream_disable_auto_flush(
    bidirectional_stream* stream,
    bool disable_auto_flush);

/**
 * Delays sending request headers until bidirectional_stream_flush()
 * is called. This flag is currently only respected when QUIC is negotiated.
 * When true, QUIC will send request header frame along with data frame(s)
 * as a single packet when possible.
 */
GRPC_SUPPORT_EXPORT
void bidirectional_stream_delay_request_headers_until_flush(
    bidirectional_stream* stream,
    bool delay_headers_until_flush);

/* Starts the stream by sending request to |url| using |method| and |headers|.
 * If |end_of_stream| is true, then no data is expected to be written. The
 * |method| is HTTP verb, with PUT having a special meaning to mark idempotent
 * request, which could use QUIC 0-RTT.
 */
GRPC_SUPPORT_EXPORT
int bidirectional_stream_start(bidirectional_stream* stream,
                               const char* url,
                               int priority,
                               const char* method,
                               const bidirectional_stream_header_array* headers,
                               bool end_of_stream);

/* Reads response data into |buffer| of |capacity| length. Must only be called
 * at most once in response to each invocation of the
 * on_stream_ready()/on_response_headers_received() and on_read_completed()
 * methods of the bidirectional_stream_callback.
 * Each call will result in an invocation of the callback's
 * on_read_completed() method if data is read, or its on_failed() method if
 * there's an error. The callback's on_succeeded() method is also invoked if
 * there is no more data to read and |end_of_stream| was previously sent.
 */
GRPC_SUPPORT_EXPORT
int bidirectional_stream_read(bidirectional_stream* stream,
                              char* buffer,
                              int capacity);

/* Writes request data from |buffer| of |buffer_length| length. If auto flush is
 * disabled, data will be sent only after bidirectional_stream_flush() is
 * called.
 * Each call will result in an invocation the callback's on_write_completed()
 * method if data is sent, or its on_failed() method if there's an error.
 * The callback's on_succeeded() method is also invoked if |end_of_stream| is
 * set and all response data has been read.
 */
GRPC_SUPPORT_EXPORT
int bidirectional_stream_write(bidirectional_stream* stream,
                               const char* buffer,
                               int buffer_length,
                               bool end_of_stream);

/**
 * Flushes pending writes. This method should not be called before invocation of
 * on_stream_ready() method of the bidirectional_stream_callback.
 * For each previously called bidirectional_stream_write()
 * a corresponding on_write_completed() callback will be invoked when the buffer
 * is sent.
 */
GRPC_SUPPORT_EXPORT
void bidirectional_stream_flush(bidirectional_stream* stream);

/* Cancels the stream. Can be called at any time after
 * bidirectional_stream_start(). The on_canceled() method of
 * bidirectional_stream_callback will be invoked when cancelation
 * is complete and no further callback methods will be invoked. If the
 * stream has completed or has not started, calling
 * bidirectional_stream_cancel() has no effect and on_canceled() will not
 * be invoked. At most one callback method may be invoked after
 * bidirectional_stream_cancel() has completed.
 */
GRPC_SUPPORT_EXPORT
void bidirectional_stream_cancel(bidirectional_stream* stream);

/* Returns true if the |stream| was successfully started and is now done
 * (succeeded, canceled, or failed).
 * Returns false if the |stream| stream is not yet started or is in progress.
 */
GRPC_SUPPORT_EXPORT
bool bidirectional_stream_is_done(bidirectional_stream* stream);

#ifdef __cplusplus
}
#endif

#endif  // COMPONENTS_GRPC_SUPPORT_INCLUDE_BIDIRECTIONAL_STREAM_H_
