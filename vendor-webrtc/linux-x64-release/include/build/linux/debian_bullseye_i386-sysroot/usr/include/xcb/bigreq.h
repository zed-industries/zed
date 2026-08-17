/*
 * This file generated automatically from bigreq.xml by c_client.py.
 * Edit at your peril.
 */

/**
 * @defgroup XCB_BigRequests_API XCB BigRequests API
 * @brief BigRequests XCB Protocol Implementation.
 * @{
 **/

#ifndef __BIGREQ_H
#define __BIGREQ_H

#include "xcb.h"

#ifdef __cplusplus
extern "C" {
#endif

#define XCB_BIGREQUESTS_MAJOR_VERSION 0
#define XCB_BIGREQUESTS_MINOR_VERSION 0

extern xcb_extension_t xcb_big_requests_id;

/**
 * @brief xcb_big_requests_enable_cookie_t
 **/
typedef struct xcb_big_requests_enable_cookie_t {
    unsigned int sequence;
} xcb_big_requests_enable_cookie_t;

/** Opcode for xcb_big_requests_enable. */
#define XCB_BIG_REQUESTS_ENABLE 0

/**
 * @brief xcb_big_requests_enable_request_t
 **/
typedef struct xcb_big_requests_enable_request_t {
    uint8_t  major_opcode;
    uint8_t  minor_opcode;
    uint16_t length;
} xcb_big_requests_enable_request_t;

/**
 * @brief xcb_big_requests_enable_reply_t
 **/
typedef struct xcb_big_requests_enable_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint32_t maximum_request_length;
} xcb_big_requests_enable_reply_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_big_requests_enable_cookie_t
xcb_big_requests_enable (xcb_connection_t *c);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
xcb_big_requests_enable_cookie_t
xcb_big_requests_enable_unchecked (xcb_connection_t *c);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_big_requests_enable_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_big_requests_enable_reply_t *
xcb_big_requests_enable_reply (xcb_connection_t                  *c,
                               xcb_big_requests_enable_cookie_t   cookie  /**< */,
                               xcb_generic_error_t              **e);


#ifdef __cplusplus
}
#endif

#endif

/**
 * @}
 */
