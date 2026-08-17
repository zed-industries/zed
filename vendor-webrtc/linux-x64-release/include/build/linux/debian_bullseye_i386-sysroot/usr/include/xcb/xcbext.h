/*
 * Copyright (C) 2001-2004 Bart Massey and Jamey Sharp.
 * All Rights Reserved.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 * 
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 * 
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
 * ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 * 
 * Except as contained in this notice, the names of the authors or their
 * institutions shall not be used in advertising or otherwise to promote the
 * sale, use or other dealings in this Software without prior written
 * authorization from the authors.
 */

#ifndef __XCBEXT_H
#define __XCBEXT_H

#include "xcb.h"

#ifdef __cplusplus
extern "C" {
#endif

/* xcb_ext.c */

struct xcb_extension_t {
    const char *name;
    int global_id;
};


/* xcb_out.c */

typedef struct {
    size_t count;
    xcb_extension_t *ext;
    uint8_t opcode;
    uint8_t isvoid;
} xcb_protocol_request_t;

enum xcb_send_request_flags_t {
    XCB_REQUEST_CHECKED = 1 << 0,
    XCB_REQUEST_RAW = 1 << 1,
    XCB_REQUEST_DISCARD_REPLY = 1 << 2,
    XCB_REQUEST_REPLY_FDS = 1 << 3
};

/**
 * @brief Send a request to the server.
 * @param c The connection to the X server.
 * @param flags A combination of flags from the xcb_send_request_flags_t enumeration.
 * @param vector Data to send; must have two iovecs before start for internal use.
 * @param request Information about the request to be sent.
 * @return The request's sequence number on success, 0 otherwise.
 *
 * This function sends a new request to the X server. The data of the request is
 * given as an array of @c iovecs in the @p vector argument. The length of that
 * array and the necessary management information are given in the @p request
 * argument.
 *
 * When this function returns, the request might or might not be sent already.
 * Use xcb_flush() to make sure that it really was sent.
 *
 * Please note that this function is not the preferred way for sending requests.
 * It's better to use the generated wrapper functions.
 *
 * Please note that xcb might use index -1 and -2 of the @p vector array internally,
 * so they must be valid!
 */
unsigned int xcb_send_request(xcb_connection_t *c, int flags, struct iovec *vector, const xcb_protocol_request_t *request);

/**
 * @brief Send a request to the server.
 * @param c The connection to the X server.
 * @param flags A combination of flags from the xcb_send_request_flags_t enumeration.
 * @param vector Data to send; must have two iovecs before start for internal use.
 * @param request Information about the request to be sent.
 * @param num_fds Number of additional file descriptors to send to the server
 * @param fds Additional file descriptors that should be send to the server.
 * @return The request's sequence number on success, 0 otherwise.
 *
 * This function sends a new request to the X server. The data of the request is
 * given as an array of @c iovecs in the @p vector argument. The length of that
 * array and the necessary management information are given in the @p request
 * argument.
 *
 * If @p num_fds is non-zero, @p fds points to an array of file descriptors that
 * will be sent to the X server along with this request. After this function
 * returns, all file descriptors sent are owned by xcb and will be closed
 * eventually.
 *
 * When this function returns, the request might or might not be sent already.
 * Use xcb_flush() to make sure that it really was sent.
 *
 * Please note that this function is not the preferred way for sending requests.
 *
 * Please note that xcb might use index -1 and -2 of the @p vector array internally,
 * so they must be valid!
 */
unsigned int xcb_send_request_with_fds(xcb_connection_t *c, int flags, struct iovec *vector,
                const xcb_protocol_request_t *request, unsigned int num_fds, int *fds);

/**
 * @brief Send a request to the server, with 64-bit sequence number returned.
 * @param c The connection to the X server.
 * @param flags A combination of flags from the xcb_send_request_flags_t enumeration.
 * @param vector Data to send; must have two iovecs before start for internal use.
 * @param request Information about the request to be sent.
 * @return The request's sequence number on success, 0 otherwise.
 *
 * This function sends a new request to the X server. The data of the request is
 * given as an array of @c iovecs in the @p vector argument. The length of that
 * array and the necessary management information are given in the @p request
 * argument.
 *
 * When this function returns, the request might or might not be sent already.
 * Use xcb_flush() to make sure that it really was sent.
 *
 * Please note that this function is not the preferred way for sending requests.
 * It's better to use the generated wrapper functions.
 *
 * Please note that xcb might use index -1 and -2 of the @p vector array internally,
 * so they must be valid!
 */
uint64_t xcb_send_request64(xcb_connection_t *c, int flags, struct iovec *vector, const xcb_protocol_request_t *request);

/**
 * @brief Send a request to the server, with 64-bit sequence number returned.
 * @param c The connection to the X server.
 * @param flags A combination of flags from the xcb_send_request_flags_t enumeration.
 * @param vector Data to send; must have two iovecs before start for internal use.
 * @param request Information about the request to be sent.
 * @param num_fds Number of additional file descriptors to send to the server
 * @param fds Additional file descriptors that should be send to the server.
 * @return The request's sequence number on success, 0 otherwise.
 *
 * This function sends a new request to the X server. The data of the request is
 * given as an array of @c iovecs in the @p vector argument. The length of that
 * array and the necessary management information are given in the @p request
 * argument.
 *
 * If @p num_fds is non-zero, @p fds points to an array of file descriptors that
 * will be sent to the X server along with this request. After this function
 * returns, all file descriptors sent are owned by xcb and will be closed
 * eventually.
 *
 * When this function returns, the request might or might not be sent already.
 * Use xcb_flush() to make sure that it really was sent.
 *
 * Please note that this function is not the preferred way for sending requests.
 * It's better to use the generated wrapper functions.
 *
 * Please note that xcb might use index -1 and -2 of the @p vector array internally,
 * so they must be valid!
 */
uint64_t xcb_send_request_with_fds64(xcb_connection_t *c, int flags, struct iovec *vector,
                const xcb_protocol_request_t *request, unsigned int num_fds, int *fds);

/**
 * @brief Send a file descriptor to the server in the next call to xcb_send_request.
 * @param c The connection to the X server.
 * @param fd The file descriptor to send.
 *
 * After this function returns, the file descriptor given is owned by xcb and
 * will be closed eventually.
 *
 * @deprecated This function cannot be used in a thread-safe way. Two threads
 * that run xcb_send_fd(); xcb_send_request(); could mix up their file
 * descriptors. Instead, xcb_send_request_with_fds() should be used.
 */
void xcb_send_fd(xcb_connection_t *c, int fd);

/**
 * @brief Take over the write side of the socket
 * @param c The connection to the X server.
 * @param return_socket Callback function that will be called when xcb wants
 *                        to use the socket again.
 * @param closure Argument to the callback function.
 * @param flags A combination of flags from the xcb_send_request_flags_t enumeration.
 * @param sent Location to the sequence number of the last sequence request.
 *              Must not be NULL.
 * @return 1 on success, else 0.
 *
 * xcb_take_socket allows external code to ask XCB for permission to
 * take over the write side of the socket and send raw data with
 * xcb_writev. xcb_take_socket provides the sequence number of the last
 * request XCB sent. The caller of xcb_take_socket must supply a
 * callback which XCB can call when it wants the write side of the
 * socket back to make a request. This callback synchronizes with the
 * external socket owner and flushes any output queues if appropriate.
 * If you are sending requests which won't cause a reply, please note the
 * comment for xcb_writev which explains some sequence number wrap issues.
 *
 * All replies that are generated while the socket is owned externally have
 * @p flags applied to them. For example, use XCB_REQUEST_CHECK if you don't
 * want errors to go to xcb's normal error handling, but instead having to be
 * picked up via xcb_wait_for_reply(), xcb_poll_for_reply() or
 * xcb_request_check().
 */
int xcb_take_socket(xcb_connection_t *c, void (*return_socket)(void *closure), void *closure, int flags, uint64_t *sent);

/**
 * @brief Send raw data to the X server.
 * @param c The connection to the X server.
 * @param vector Array of data to be sent.
 * @param count Number of entries in @p vector.
 * @param requests Number of requests that are being sent.
 * @return 1 on success, else 0.
 *
 * You must own the write-side of the socket (you've called
 * xcb_take_socket, and haven't returned from return_socket yet) to call
 * xcb_writev. Also, the iovec must have at least 1 byte of data in it.
 * You have to make sure that xcb can detect sequence number wraps correctly.
 * This means that the first request you send after xcb_take_socket must cause a
 * reply (e.g. just insert a GetInputFocus request). After every (1 << 16) - 1
 * requests without a reply, you have to insert a request which will cause a
 * reply. You can again use GetInputFocus for this. You do not have to wait for
 * any of the GetInputFocus replies, but can instead handle them via
 * xcb_discard_reply().
 */
int xcb_writev(xcb_connection_t *c, struct iovec *vector, int count, uint64_t requests);


/* xcb_in.c */

/**
 * @brief Wait for the reply of a given request.
 * @param c The connection to the X server.
 * @param request Sequence number of the request as returned by xcb_send_request().
 * @param e Location to store errors in, or NULL. Ignored for unchecked requests.
 *
 * Returns the reply to the given request or returns null in the event of
 * errors. Blocks until the reply or error for the request arrives, or an I/O
 * error occurs.
 */
void *xcb_wait_for_reply(xcb_connection_t *c, unsigned int request, xcb_generic_error_t **e);

/**
 * @brief Wait for the reply of a given request, with 64-bit sequence number
 * @param c The connection to the X server.
 * @param request 64-bit sequence number of the request as returned by xcb_send_request64().
 * @param e Location to store errors in, or NULL. Ignored for unchecked requests.
 *
 * Returns the reply to the given request or returns null in the event of
 * errors. Blocks until the reply or error for the request arrives, or an I/O
 * error occurs.
 *
 * Unlike its xcb_wait_for_reply() counterpart, the given sequence number is not
 * automatically "widened" to 64-bit.
 */
void *xcb_wait_for_reply64(xcb_connection_t *c, uint64_t request, xcb_generic_error_t **e);

/**
 * @brief Poll for the reply of a given request.
 * @param c The connection to the X server.
 * @param request Sequence number of the request as returned by xcb_send_request().
 * @param reply Location to store the reply in, must not be NULL.
 * @param error Location to store errors in, or NULL. Ignored for unchecked requests.
 * @return 1 when the reply to the request was returned, else 0.
 *
 * Checks if the reply to the given request already received. Does not block.
 */
int xcb_poll_for_reply(xcb_connection_t *c, unsigned int request, void **reply, xcb_generic_error_t **error);

/**
 * @brief Poll for the reply of a given request, with 64-bit sequence number.
 * @param c The connection to the X server.
 * @param request 64-bit sequence number of the request as returned by xcb_send_request().
 * @param reply Location to store the reply in, must not be NULL.
 * @param error Location to store errors in, or NULL. Ignored for unchecked requests.
 * @return 1 when the reply to the request was returned, else 0.
 *
 * Checks if the reply to the given request already received. Does not block.
 *
 * Unlike its xcb_poll_for_reply() counterpart, the given sequence number is not
 * automatically "widened" to 64-bit.
 */
int xcb_poll_for_reply64(xcb_connection_t *c, uint64_t request, void **reply, xcb_generic_error_t **error);

/**
 * @brief Don't use this, only needed by the generated code.
 * @param c The connection to the X server.
 * @param reply A reply that was received from the server
 * @param replylen The size of the reply.
 * @return Pointer to the location where received file descriptors are stored.
 */
int *xcb_get_reply_fds(xcb_connection_t *c, void *reply, size_t replylen);


/* xcb_util.c */

/**
 * @param mask The mask to check
 * @return The number of set bits in the mask
 */
int xcb_popcount(uint32_t mask);

/**
 * @param list The base of an array
 * @param len The length of the array
 * @return The sum of all entries in the array.
 */
int xcb_sumof(uint8_t *list, int len);

#ifdef __cplusplus
}
#endif

#endif
