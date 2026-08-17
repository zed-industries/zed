/*
 * This file generated automatically from xc_misc.xml by c_client.py.
 * Edit at your peril.
 */

/**
 * @defgroup XCB_XCMisc_API XCB XCMisc API
 * @brief XCMisc XCB Protocol Implementation.
 * @{
 **/

#ifndef __XC_MISC_H
#define __XC_MISC_H

#include "xcb.h"

#ifdef __cplusplus
extern "C" {
#endif

#define XCB_XCMISC_MAJOR_VERSION 1
#define XCB_XCMISC_MINOR_VERSION 1

extern xcb_extension_t xcb_xc_misc_id;

/**
 * @brief xcb_xc_misc_get_version_cookie_t
 **/
typedef struct xcb_xc_misc_get_version_cookie_t {
    unsigned int sequence;
} xcb_xc_misc_get_version_cookie_t;

/** Opcode for xcb_xc_misc_get_version. */
#define XCB_XC_MISC_GET_VERSION 0

/**
 * @brief xcb_xc_misc_get_version_request_t
 **/
typedef struct xcb_xc_misc_get_version_request_t {
    uint8_t  major_opcode;
    uint8_t  minor_opcode;
    uint16_t length;
    uint16_t client_major_version;
    uint16_t client_minor_version;
} xcb_xc_misc_get_version_request_t;

/**
 * @brief xcb_xc_misc_get_version_reply_t
 **/
typedef struct xcb_xc_misc_get_version_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint16_t server_major_version;
    uint16_t server_minor_version;
} xcb_xc_misc_get_version_reply_t;

/**
 * @brief xcb_xc_misc_get_xid_range_cookie_t
 **/
typedef struct xcb_xc_misc_get_xid_range_cookie_t {
    unsigned int sequence;
} xcb_xc_misc_get_xid_range_cookie_t;

/** Opcode for xcb_xc_misc_get_xid_range. */
#define XCB_XC_MISC_GET_XID_RANGE 1

/**
 * @brief xcb_xc_misc_get_xid_range_request_t
 **/
typedef struct xcb_xc_misc_get_xid_range_request_t {
    uint8_t  major_opcode;
    uint8_t  minor_opcode;
    uint16_t length;
} xcb_xc_misc_get_xid_range_request_t;

/**
 * @brief xcb_xc_misc_get_xid_range_reply_t
 **/
typedef struct xcb_xc_misc_get_xid_range_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint32_t start_id;
    uint32_t count;
} xcb_xc_misc_get_xid_range_reply_t;

/**
 * @brief xcb_xc_misc_get_xid_list_cookie_t
 **/
typedef struct xcb_xc_misc_get_xid_list_cookie_t {
    unsigned int sequence;
} xcb_xc_misc_get_xid_list_cookie_t;

/** Opcode for xcb_xc_misc_get_xid_list. */
#define XCB_XC_MISC_GET_XID_LIST 2

/**
 * @brief xcb_xc_misc_get_xid_list_request_t
 **/
typedef struct xcb_xc_misc_get_xid_list_request_t {
    uint8_t  major_opcode;
    uint8_t  minor_opcode;
    uint16_t length;
    uint32_t count;
} xcb_xc_misc_get_xid_list_request_t;

/**
 * @brief xcb_xc_misc_get_xid_list_reply_t
 **/
typedef struct xcb_xc_misc_get_xid_list_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint32_t ids_len;
    uint8_t  pad1[20];
} xcb_xc_misc_get_xid_list_reply_t;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_xc_misc_get_version_cookie_t
xcb_xc_misc_get_version (xcb_connection_t *c,
                         uint16_t          client_major_version,
                         uint16_t          client_minor_version);

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
xcb_xc_misc_get_version_cookie_t
xcb_xc_misc_get_version_unchecked (xcb_connection_t *c,
                                   uint16_t          client_major_version,
                                   uint16_t          client_minor_version);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xc_misc_get_version_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_xc_misc_get_version_reply_t *
xcb_xc_misc_get_version_reply (xcb_connection_t                  *c,
                               xcb_xc_misc_get_version_cookie_t   cookie  /**< */,
                               xcb_generic_error_t              **e);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_xc_misc_get_xid_range_cookie_t
xcb_xc_misc_get_xid_range (xcb_connection_t *c);

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
xcb_xc_misc_get_xid_range_cookie_t
xcb_xc_misc_get_xid_range_unchecked (xcb_connection_t *c);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xc_misc_get_xid_range_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_xc_misc_get_xid_range_reply_t *
xcb_xc_misc_get_xid_range_reply (xcb_connection_t                    *c,
                                 xcb_xc_misc_get_xid_range_cookie_t   cookie  /**< */,
                                 xcb_generic_error_t                **e);

int
xcb_xc_misc_get_xid_list_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_xc_misc_get_xid_list_cookie_t
xcb_xc_misc_get_xid_list (xcb_connection_t *c,
                          uint32_t          count);

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
xcb_xc_misc_get_xid_list_cookie_t
xcb_xc_misc_get_xid_list_unchecked (xcb_connection_t *c,
                                    uint32_t          count);

uint32_t *
xcb_xc_misc_get_xid_list_ids (const xcb_xc_misc_get_xid_list_reply_t *R);

int
xcb_xc_misc_get_xid_list_ids_length (const xcb_xc_misc_get_xid_list_reply_t *R);

xcb_generic_iterator_t
xcb_xc_misc_get_xid_list_ids_end (const xcb_xc_misc_get_xid_list_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_xc_misc_get_xid_list_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_xc_misc_get_xid_list_reply_t *
xcb_xc_misc_get_xid_list_reply (xcb_connection_t                   *c,
                                xcb_xc_misc_get_xid_list_cookie_t   cookie  /**< */,
                                xcb_generic_error_t               **e);


#ifdef __cplusplus
}
#endif

#endif

/**
 * @}
 */
