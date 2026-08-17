/*
 * This file generated automatically from dri2.xml by c_client.py.
 * Edit at your peril.
 */

/**
 * @defgroup XCB_DRI2_API XCB DRI2 API
 * @brief DRI2 XCB Protocol Implementation.
 * @{
 **/

#ifndef __DRI2_H
#define __DRI2_H

#include "xcb.h"
#include "xproto.h"

#ifdef __cplusplus
extern "C" {
#endif

#define XCB_DRI2_MAJOR_VERSION 1
#define XCB_DRI2_MINOR_VERSION 4

extern xcb_extension_t xcb_dri2_id;

typedef enum xcb_dri2_attachment_t {
    XCB_DRI2_ATTACHMENT_BUFFER_FRONT_LEFT = 0,
    XCB_DRI2_ATTACHMENT_BUFFER_BACK_LEFT = 1,
    XCB_DRI2_ATTACHMENT_BUFFER_FRONT_RIGHT = 2,
    XCB_DRI2_ATTACHMENT_BUFFER_BACK_RIGHT = 3,
    XCB_DRI2_ATTACHMENT_BUFFER_DEPTH = 4,
    XCB_DRI2_ATTACHMENT_BUFFER_STENCIL = 5,
    XCB_DRI2_ATTACHMENT_BUFFER_ACCUM = 6,
    XCB_DRI2_ATTACHMENT_BUFFER_FAKE_FRONT_LEFT = 7,
    XCB_DRI2_ATTACHMENT_BUFFER_FAKE_FRONT_RIGHT = 8,
    XCB_DRI2_ATTACHMENT_BUFFER_DEPTH_STENCIL = 9,
    XCB_DRI2_ATTACHMENT_BUFFER_HIZ = 10
} xcb_dri2_attachment_t;

typedef enum xcb_dri2_driver_type_t {
    XCB_DRI2_DRIVER_TYPE_DRI = 0,
    XCB_DRI2_DRIVER_TYPE_VDPAU = 1
} xcb_dri2_driver_type_t;

typedef enum xcb_dri2_event_type_t {
    XCB_DRI2_EVENT_TYPE_EXCHANGE_COMPLETE = 1,
    XCB_DRI2_EVENT_TYPE_BLIT_COMPLETE = 2,
    XCB_DRI2_EVENT_TYPE_FLIP_COMPLETE = 3
} xcb_dri2_event_type_t;

/**
 * @brief xcb_dri2_dri2_buffer_t
 **/
typedef struct xcb_dri2_dri2_buffer_t {
    uint32_t attachment;
    uint32_t name;
    uint32_t pitch;
    uint32_t cpp;
    uint32_t flags;
} xcb_dri2_dri2_buffer_t;

/**
 * @brief xcb_dri2_dri2_buffer_iterator_t
 **/
typedef struct xcb_dri2_dri2_buffer_iterator_t {
    xcb_dri2_dri2_buffer_t *data;
    int                     rem;
    int                     index;
} xcb_dri2_dri2_buffer_iterator_t;

/**
 * @brief xcb_dri2_attach_format_t
 **/
typedef struct xcb_dri2_attach_format_t {
    uint32_t attachment;
    uint32_t format;
} xcb_dri2_attach_format_t;

/**
 * @brief xcb_dri2_attach_format_iterator_t
 **/
typedef struct xcb_dri2_attach_format_iterator_t {
    xcb_dri2_attach_format_t *data;
    int                       rem;
    int                       index;
} xcb_dri2_attach_format_iterator_t;

/**
 * @brief xcb_dri2_query_version_cookie_t
 **/
typedef struct xcb_dri2_query_version_cookie_t {
    unsigned int sequence;
} xcb_dri2_query_version_cookie_t;

/** Opcode for xcb_dri2_query_version. */
#define XCB_DRI2_QUERY_VERSION 0

/**
 * @brief xcb_dri2_query_version_request_t
 **/
typedef struct xcb_dri2_query_version_request_t {
    uint8_t  major_opcode;
    uint8_t  minor_opcode;
    uint16_t length;
    uint32_t major_version;
    uint32_t minor_version;
} xcb_dri2_query_version_request_t;

/**
 * @brief xcb_dri2_query_version_reply_t
 **/
typedef struct xcb_dri2_query_version_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint32_t major_version;
    uint32_t minor_version;
} xcb_dri2_query_version_reply_t;

/**
 * @brief xcb_dri2_connect_cookie_t
 **/
typedef struct xcb_dri2_connect_cookie_t {
    unsigned int sequence;
} xcb_dri2_connect_cookie_t;

/** Opcode for xcb_dri2_connect. */
#define XCB_DRI2_CONNECT 1

/**
 * @brief xcb_dri2_connect_request_t
 **/
typedef struct xcb_dri2_connect_request_t {
    uint8_t      major_opcode;
    uint8_t      minor_opcode;
    uint16_t     length;
    xcb_window_t window;
    uint32_t     driver_type;
} xcb_dri2_connect_request_t;

/**
 * @brief xcb_dri2_connect_reply_t
 **/
typedef struct xcb_dri2_connect_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint32_t driver_name_length;
    uint32_t device_name_length;
    uint8_t  pad1[16];
} xcb_dri2_connect_reply_t;

/**
 * @brief xcb_dri2_authenticate_cookie_t
 **/
typedef struct xcb_dri2_authenticate_cookie_t {
    unsigned int sequence;
} xcb_dri2_authenticate_cookie_t;

/** Opcode for xcb_dri2_authenticate. */
#define XCB_DRI2_AUTHENTICATE 2

/**
 * @brief xcb_dri2_authenticate_request_t
 **/
typedef struct xcb_dri2_authenticate_request_t {
    uint8_t      major_opcode;
    uint8_t      minor_opcode;
    uint16_t     length;
    xcb_window_t window;
    uint32_t     magic;
} xcb_dri2_authenticate_request_t;

/**
 * @brief xcb_dri2_authenticate_reply_t
 **/
typedef struct xcb_dri2_authenticate_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint32_t authenticated;
} xcb_dri2_authenticate_reply_t;

/** Opcode for xcb_dri2_create_drawable. */
#define XCB_DRI2_CREATE_DRAWABLE 3

/**
 * @brief xcb_dri2_create_drawable_request_t
 **/
typedef struct xcb_dri2_create_drawable_request_t {
    uint8_t        major_opcode;
    uint8_t        minor_opcode;
    uint16_t       length;
    xcb_drawable_t drawable;
} xcb_dri2_create_drawable_request_t;

/** Opcode for xcb_dri2_destroy_drawable. */
#define XCB_DRI2_DESTROY_DRAWABLE 4

/**
 * @brief xcb_dri2_destroy_drawable_request_t
 **/
typedef struct xcb_dri2_destroy_drawable_request_t {
    uint8_t        major_opcode;
    uint8_t        minor_opcode;
    uint16_t       length;
    xcb_drawable_t drawable;
} xcb_dri2_destroy_drawable_request_t;

/**
 * @brief xcb_dri2_get_buffers_cookie_t
 **/
typedef struct xcb_dri2_get_buffers_cookie_t {
    unsigned int sequence;
} xcb_dri2_get_buffers_cookie_t;

/** Opcode for xcb_dri2_get_buffers. */
#define XCB_DRI2_GET_BUFFERS 5

/**
 * @brief xcb_dri2_get_buffers_request_t
 **/
typedef struct xcb_dri2_get_buffers_request_t {
    uint8_t        major_opcode;
    uint8_t        minor_opcode;
    uint16_t       length;
    xcb_drawable_t drawable;
    uint32_t       count;
} xcb_dri2_get_buffers_request_t;

/**
 * @brief xcb_dri2_get_buffers_reply_t
 **/
typedef struct xcb_dri2_get_buffers_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint32_t width;
    uint32_t height;
    uint32_t count;
    uint8_t  pad1[12];
} xcb_dri2_get_buffers_reply_t;

/**
 * @brief xcb_dri2_copy_region_cookie_t
 **/
typedef struct xcb_dri2_copy_region_cookie_t {
    unsigned int sequence;
} xcb_dri2_copy_region_cookie_t;

/** Opcode for xcb_dri2_copy_region. */
#define XCB_DRI2_COPY_REGION 6

/**
 * @brief xcb_dri2_copy_region_request_t
 **/
typedef struct xcb_dri2_copy_region_request_t {
    uint8_t        major_opcode;
    uint8_t        minor_opcode;
    uint16_t       length;
    xcb_drawable_t drawable;
    uint32_t       region;
    uint32_t       dest;
    uint32_t       src;
} xcb_dri2_copy_region_request_t;

/**
 * @brief xcb_dri2_copy_region_reply_t
 **/
typedef struct xcb_dri2_copy_region_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
} xcb_dri2_copy_region_reply_t;

/**
 * @brief xcb_dri2_get_buffers_with_format_cookie_t
 **/
typedef struct xcb_dri2_get_buffers_with_format_cookie_t {
    unsigned int sequence;
} xcb_dri2_get_buffers_with_format_cookie_t;

/** Opcode for xcb_dri2_get_buffers_with_format. */
#define XCB_DRI2_GET_BUFFERS_WITH_FORMAT 7

/**
 * @brief xcb_dri2_get_buffers_with_format_request_t
 **/
typedef struct xcb_dri2_get_buffers_with_format_request_t {
    uint8_t        major_opcode;
    uint8_t        minor_opcode;
    uint16_t       length;
    xcb_drawable_t drawable;
    uint32_t       count;
} xcb_dri2_get_buffers_with_format_request_t;

/**
 * @brief xcb_dri2_get_buffers_with_format_reply_t
 **/
typedef struct xcb_dri2_get_buffers_with_format_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint32_t width;
    uint32_t height;
    uint32_t count;
    uint8_t  pad1[12];
} xcb_dri2_get_buffers_with_format_reply_t;

/**
 * @brief xcb_dri2_swap_buffers_cookie_t
 **/
typedef struct xcb_dri2_swap_buffers_cookie_t {
    unsigned int sequence;
} xcb_dri2_swap_buffers_cookie_t;

/** Opcode for xcb_dri2_swap_buffers. */
#define XCB_DRI2_SWAP_BUFFERS 8

/**
 * @brief xcb_dri2_swap_buffers_request_t
 **/
typedef struct xcb_dri2_swap_buffers_request_t {
    uint8_t        major_opcode;
    uint8_t        minor_opcode;
    uint16_t       length;
    xcb_drawable_t drawable;
    uint32_t       target_msc_hi;
    uint32_t       target_msc_lo;
    uint32_t       divisor_hi;
    uint32_t       divisor_lo;
    uint32_t       remainder_hi;
    uint32_t       remainder_lo;
} xcb_dri2_swap_buffers_request_t;

/**
 * @brief xcb_dri2_swap_buffers_reply_t
 **/
typedef struct xcb_dri2_swap_buffers_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint32_t swap_hi;
    uint32_t swap_lo;
} xcb_dri2_swap_buffers_reply_t;

/**
 * @brief xcb_dri2_get_msc_cookie_t
 **/
typedef struct xcb_dri2_get_msc_cookie_t {
    unsigned int sequence;
} xcb_dri2_get_msc_cookie_t;

/** Opcode for xcb_dri2_get_msc. */
#define XCB_DRI2_GET_MSC 9

/**
 * @brief xcb_dri2_get_msc_request_t
 **/
typedef struct xcb_dri2_get_msc_request_t {
    uint8_t        major_opcode;
    uint8_t        minor_opcode;
    uint16_t       length;
    xcb_drawable_t drawable;
} xcb_dri2_get_msc_request_t;

/**
 * @brief xcb_dri2_get_msc_reply_t
 **/
typedef struct xcb_dri2_get_msc_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint32_t ust_hi;
    uint32_t ust_lo;
    uint32_t msc_hi;
    uint32_t msc_lo;
    uint32_t sbc_hi;
    uint32_t sbc_lo;
} xcb_dri2_get_msc_reply_t;

/**
 * @brief xcb_dri2_wait_msc_cookie_t
 **/
typedef struct xcb_dri2_wait_msc_cookie_t {
    unsigned int sequence;
} xcb_dri2_wait_msc_cookie_t;

/** Opcode for xcb_dri2_wait_msc. */
#define XCB_DRI2_WAIT_MSC 10

/**
 * @brief xcb_dri2_wait_msc_request_t
 **/
typedef struct xcb_dri2_wait_msc_request_t {
    uint8_t        major_opcode;
    uint8_t        minor_opcode;
    uint16_t       length;
    xcb_drawable_t drawable;
    uint32_t       target_msc_hi;
    uint32_t       target_msc_lo;
    uint32_t       divisor_hi;
    uint32_t       divisor_lo;
    uint32_t       remainder_hi;
    uint32_t       remainder_lo;
} xcb_dri2_wait_msc_request_t;

/**
 * @brief xcb_dri2_wait_msc_reply_t
 **/
typedef struct xcb_dri2_wait_msc_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint32_t ust_hi;
    uint32_t ust_lo;
    uint32_t msc_hi;
    uint32_t msc_lo;
    uint32_t sbc_hi;
    uint32_t sbc_lo;
} xcb_dri2_wait_msc_reply_t;

/**
 * @brief xcb_dri2_wait_sbc_cookie_t
 **/
typedef struct xcb_dri2_wait_sbc_cookie_t {
    unsigned int sequence;
} xcb_dri2_wait_sbc_cookie_t;

/** Opcode for xcb_dri2_wait_sbc. */
#define XCB_DRI2_WAIT_SBC 11

/**
 * @brief xcb_dri2_wait_sbc_request_t
 **/
typedef struct xcb_dri2_wait_sbc_request_t {
    uint8_t        major_opcode;
    uint8_t        minor_opcode;
    uint16_t       length;
    xcb_drawable_t drawable;
    uint32_t       target_sbc_hi;
    uint32_t       target_sbc_lo;
} xcb_dri2_wait_sbc_request_t;

/**
 * @brief xcb_dri2_wait_sbc_reply_t
 **/
typedef struct xcb_dri2_wait_sbc_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint32_t ust_hi;
    uint32_t ust_lo;
    uint32_t msc_hi;
    uint32_t msc_lo;
    uint32_t sbc_hi;
    uint32_t sbc_lo;
} xcb_dri2_wait_sbc_reply_t;

/** Opcode for xcb_dri2_swap_interval. */
#define XCB_DRI2_SWAP_INTERVAL 12

/**
 * @brief xcb_dri2_swap_interval_request_t
 **/
typedef struct xcb_dri2_swap_interval_request_t {
    uint8_t        major_opcode;
    uint8_t        minor_opcode;
    uint16_t       length;
    xcb_drawable_t drawable;
    uint32_t       interval;
} xcb_dri2_swap_interval_request_t;

/**
 * @brief xcb_dri2_get_param_cookie_t
 **/
typedef struct xcb_dri2_get_param_cookie_t {
    unsigned int sequence;
} xcb_dri2_get_param_cookie_t;

/** Opcode for xcb_dri2_get_param. */
#define XCB_DRI2_GET_PARAM 13

/**
 * @brief xcb_dri2_get_param_request_t
 **/
typedef struct xcb_dri2_get_param_request_t {
    uint8_t        major_opcode;
    uint8_t        minor_opcode;
    uint16_t       length;
    xcb_drawable_t drawable;
    uint32_t       param;
} xcb_dri2_get_param_request_t;

/**
 * @brief xcb_dri2_get_param_reply_t
 **/
typedef struct xcb_dri2_get_param_reply_t {
    uint8_t  response_type;
    uint8_t  is_param_recognized;
    uint16_t sequence;
    uint32_t length;
    uint32_t value_hi;
    uint32_t value_lo;
} xcb_dri2_get_param_reply_t;

/** Opcode for xcb_dri2_buffer_swap_complete. */
#define XCB_DRI2_BUFFER_SWAP_COMPLETE 0

/**
 * @brief xcb_dri2_buffer_swap_complete_event_t
 **/
typedef struct xcb_dri2_buffer_swap_complete_event_t {
    uint8_t        response_type;
    uint8_t        pad0;
    uint16_t       sequence;
    uint16_t       event_type;
    uint8_t        pad1[2];
    xcb_drawable_t drawable;
    uint32_t       ust_hi;
    uint32_t       ust_lo;
    uint32_t       msc_hi;
    uint32_t       msc_lo;
    uint32_t       sbc;
} xcb_dri2_buffer_swap_complete_event_t;

/** Opcode for xcb_dri2_invalidate_buffers. */
#define XCB_DRI2_INVALIDATE_BUFFERS 1

/**
 * @brief xcb_dri2_invalidate_buffers_event_t
 **/
typedef struct xcb_dri2_invalidate_buffers_event_t {
    uint8_t        response_type;
    uint8_t        pad0;
    uint16_t       sequence;
    xcb_drawable_t drawable;
} xcb_dri2_invalidate_buffers_event_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_dri2_dri2_buffer_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_dri2_dri2_buffer_t)
 */
void
xcb_dri2_dri2_buffer_next (xcb_dri2_dri2_buffer_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_dri2_dri2_buffer_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_dri2_dri2_buffer_end (xcb_dri2_dri2_buffer_iterator_t i);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_dri2_attach_format_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_dri2_attach_format_t)
 */
void
xcb_dri2_attach_format_next (xcb_dri2_attach_format_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_dri2_attach_format_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_dri2_attach_format_end (xcb_dri2_attach_format_iterator_t i);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_dri2_query_version_cookie_t
xcb_dri2_query_version (xcb_connection_t *c,
                        uint32_t          major_version,
                        uint32_t          minor_version);

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
xcb_dri2_query_version_cookie_t
xcb_dri2_query_version_unchecked (xcb_connection_t *c,
                                  uint32_t          major_version,
                                  uint32_t          minor_version);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_dri2_query_version_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_dri2_query_version_reply_t *
xcb_dri2_query_version_reply (xcb_connection_t                 *c,
                              xcb_dri2_query_version_cookie_t   cookie  /**< */,
                              xcb_generic_error_t             **e);

int
xcb_dri2_connect_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_dri2_connect_cookie_t
xcb_dri2_connect (xcb_connection_t *c,
                  xcb_window_t      window,
                  uint32_t          driver_type);

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
xcb_dri2_connect_cookie_t
xcb_dri2_connect_unchecked (xcb_connection_t *c,
                            xcb_window_t      window,
                            uint32_t          driver_type);

char *
xcb_dri2_connect_driver_name (const xcb_dri2_connect_reply_t *R);

int
xcb_dri2_connect_driver_name_length (const xcb_dri2_connect_reply_t *R);

xcb_generic_iterator_t
xcb_dri2_connect_driver_name_end (const xcb_dri2_connect_reply_t *R);

void *
xcb_dri2_connect_alignment_pad (const xcb_dri2_connect_reply_t *R);

int
xcb_dri2_connect_alignment_pad_length (const xcb_dri2_connect_reply_t *R);

xcb_generic_iterator_t
xcb_dri2_connect_alignment_pad_end (const xcb_dri2_connect_reply_t *R);

char *
xcb_dri2_connect_device_name (const xcb_dri2_connect_reply_t *R);

int
xcb_dri2_connect_device_name_length (const xcb_dri2_connect_reply_t *R);

xcb_generic_iterator_t
xcb_dri2_connect_device_name_end (const xcb_dri2_connect_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_dri2_connect_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_dri2_connect_reply_t *
xcb_dri2_connect_reply (xcb_connection_t           *c,
                        xcb_dri2_connect_cookie_t   cookie  /**< */,
                        xcb_generic_error_t       **e);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_dri2_authenticate_cookie_t
xcb_dri2_authenticate (xcb_connection_t *c,
                       xcb_window_t      window,
                       uint32_t          magic);

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
xcb_dri2_authenticate_cookie_t
xcb_dri2_authenticate_unchecked (xcb_connection_t *c,
                                 xcb_window_t      window,
                                 uint32_t          magic);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_dri2_authenticate_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_dri2_authenticate_reply_t *
xcb_dri2_authenticate_reply (xcb_connection_t                *c,
                             xcb_dri2_authenticate_cookie_t   cookie  /**< */,
                             xcb_generic_error_t            **e);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
xcb_void_cookie_t
xcb_dri2_create_drawable_checked (xcb_connection_t *c,
                                  xcb_drawable_t    drawable);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_dri2_create_drawable (xcb_connection_t *c,
                          xcb_drawable_t    drawable);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
xcb_void_cookie_t
xcb_dri2_destroy_drawable_checked (xcb_connection_t *c,
                                   xcb_drawable_t    drawable);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_dri2_destroy_drawable (xcb_connection_t *c,
                           xcb_drawable_t    drawable);

int
xcb_dri2_get_buffers_sizeof (const void  *_buffer,
                             uint32_t     attachments_len);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_dri2_get_buffers_cookie_t
xcb_dri2_get_buffers (xcb_connection_t *c,
                      xcb_drawable_t    drawable,
                      uint32_t          count,
                      uint32_t          attachments_len,
                      const uint32_t   *attachments);

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
xcb_dri2_get_buffers_cookie_t
xcb_dri2_get_buffers_unchecked (xcb_connection_t *c,
                                xcb_drawable_t    drawable,
                                uint32_t          count,
                                uint32_t          attachments_len,
                                const uint32_t   *attachments);

xcb_dri2_dri2_buffer_t *
xcb_dri2_get_buffers_buffers (const xcb_dri2_get_buffers_reply_t *R);

int
xcb_dri2_get_buffers_buffers_length (const xcb_dri2_get_buffers_reply_t *R);

xcb_dri2_dri2_buffer_iterator_t
xcb_dri2_get_buffers_buffers_iterator (const xcb_dri2_get_buffers_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_dri2_get_buffers_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_dri2_get_buffers_reply_t *
xcb_dri2_get_buffers_reply (xcb_connection_t               *c,
                            xcb_dri2_get_buffers_cookie_t   cookie  /**< */,
                            xcb_generic_error_t           **e);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_dri2_copy_region_cookie_t
xcb_dri2_copy_region (xcb_connection_t *c,
                      xcb_drawable_t    drawable,
                      uint32_t          region,
                      uint32_t          dest,
                      uint32_t          src);

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
xcb_dri2_copy_region_cookie_t
xcb_dri2_copy_region_unchecked (xcb_connection_t *c,
                                xcb_drawable_t    drawable,
                                uint32_t          region,
                                uint32_t          dest,
                                uint32_t          src);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_dri2_copy_region_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_dri2_copy_region_reply_t *
xcb_dri2_copy_region_reply (xcb_connection_t               *c,
                            xcb_dri2_copy_region_cookie_t   cookie  /**< */,
                            xcb_generic_error_t           **e);

int
xcb_dri2_get_buffers_with_format_sizeof (const void  *_buffer,
                                         uint32_t     attachments_len);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_dri2_get_buffers_with_format_cookie_t
xcb_dri2_get_buffers_with_format (xcb_connection_t               *c,
                                  xcb_drawable_t                  drawable,
                                  uint32_t                        count,
                                  uint32_t                        attachments_len,
                                  const xcb_dri2_attach_format_t *attachments);

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
xcb_dri2_get_buffers_with_format_cookie_t
xcb_dri2_get_buffers_with_format_unchecked (xcb_connection_t               *c,
                                            xcb_drawable_t                  drawable,
                                            uint32_t                        count,
                                            uint32_t                        attachments_len,
                                            const xcb_dri2_attach_format_t *attachments);

xcb_dri2_dri2_buffer_t *
xcb_dri2_get_buffers_with_format_buffers (const xcb_dri2_get_buffers_with_format_reply_t *R);

int
xcb_dri2_get_buffers_with_format_buffers_length (const xcb_dri2_get_buffers_with_format_reply_t *R);

xcb_dri2_dri2_buffer_iterator_t
xcb_dri2_get_buffers_with_format_buffers_iterator (const xcb_dri2_get_buffers_with_format_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_dri2_get_buffers_with_format_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_dri2_get_buffers_with_format_reply_t *
xcb_dri2_get_buffers_with_format_reply (xcb_connection_t                           *c,
                                        xcb_dri2_get_buffers_with_format_cookie_t   cookie  /**< */,
                                        xcb_generic_error_t                       **e);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_dri2_swap_buffers_cookie_t
xcb_dri2_swap_buffers (xcb_connection_t *c,
                       xcb_drawable_t    drawable,
                       uint32_t          target_msc_hi,
                       uint32_t          target_msc_lo,
                       uint32_t          divisor_hi,
                       uint32_t          divisor_lo,
                       uint32_t          remainder_hi,
                       uint32_t          remainder_lo);

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
xcb_dri2_swap_buffers_cookie_t
xcb_dri2_swap_buffers_unchecked (xcb_connection_t *c,
                                 xcb_drawable_t    drawable,
                                 uint32_t          target_msc_hi,
                                 uint32_t          target_msc_lo,
                                 uint32_t          divisor_hi,
                                 uint32_t          divisor_lo,
                                 uint32_t          remainder_hi,
                                 uint32_t          remainder_lo);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_dri2_swap_buffers_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_dri2_swap_buffers_reply_t *
xcb_dri2_swap_buffers_reply (xcb_connection_t                *c,
                             xcb_dri2_swap_buffers_cookie_t   cookie  /**< */,
                             xcb_generic_error_t            **e);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_dri2_get_msc_cookie_t
xcb_dri2_get_msc (xcb_connection_t *c,
                  xcb_drawable_t    drawable);

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
xcb_dri2_get_msc_cookie_t
xcb_dri2_get_msc_unchecked (xcb_connection_t *c,
                            xcb_drawable_t    drawable);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_dri2_get_msc_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_dri2_get_msc_reply_t *
xcb_dri2_get_msc_reply (xcb_connection_t           *c,
                        xcb_dri2_get_msc_cookie_t   cookie  /**< */,
                        xcb_generic_error_t       **e);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_dri2_wait_msc_cookie_t
xcb_dri2_wait_msc (xcb_connection_t *c,
                   xcb_drawable_t    drawable,
                   uint32_t          target_msc_hi,
                   uint32_t          target_msc_lo,
                   uint32_t          divisor_hi,
                   uint32_t          divisor_lo,
                   uint32_t          remainder_hi,
                   uint32_t          remainder_lo);

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
xcb_dri2_wait_msc_cookie_t
xcb_dri2_wait_msc_unchecked (xcb_connection_t *c,
                             xcb_drawable_t    drawable,
                             uint32_t          target_msc_hi,
                             uint32_t          target_msc_lo,
                             uint32_t          divisor_hi,
                             uint32_t          divisor_lo,
                             uint32_t          remainder_hi,
                             uint32_t          remainder_lo);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_dri2_wait_msc_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_dri2_wait_msc_reply_t *
xcb_dri2_wait_msc_reply (xcb_connection_t            *c,
                         xcb_dri2_wait_msc_cookie_t   cookie  /**< */,
                         xcb_generic_error_t        **e);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_dri2_wait_sbc_cookie_t
xcb_dri2_wait_sbc (xcb_connection_t *c,
                   xcb_drawable_t    drawable,
                   uint32_t          target_sbc_hi,
                   uint32_t          target_sbc_lo);

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
xcb_dri2_wait_sbc_cookie_t
xcb_dri2_wait_sbc_unchecked (xcb_connection_t *c,
                             xcb_drawable_t    drawable,
                             uint32_t          target_sbc_hi,
                             uint32_t          target_sbc_lo);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_dri2_wait_sbc_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_dri2_wait_sbc_reply_t *
xcb_dri2_wait_sbc_reply (xcb_connection_t            *c,
                         xcb_dri2_wait_sbc_cookie_t   cookie  /**< */,
                         xcb_generic_error_t        **e);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
xcb_void_cookie_t
xcb_dri2_swap_interval_checked (xcb_connection_t *c,
                                xcb_drawable_t    drawable,
                                uint32_t          interval);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_dri2_swap_interval (xcb_connection_t *c,
                        xcb_drawable_t    drawable,
                        uint32_t          interval);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_dri2_get_param_cookie_t
xcb_dri2_get_param (xcb_connection_t *c,
                    xcb_drawable_t    drawable,
                    uint32_t          param);

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
xcb_dri2_get_param_cookie_t
xcb_dri2_get_param_unchecked (xcb_connection_t *c,
                              xcb_drawable_t    drawable,
                              uint32_t          param);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_dri2_get_param_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_dri2_get_param_reply_t *
xcb_dri2_get_param_reply (xcb_connection_t             *c,
                          xcb_dri2_get_param_cookie_t   cookie  /**< */,
                          xcb_generic_error_t         **e);


#ifdef __cplusplus
}
#endif

#endif

/**
 * @}
 */
