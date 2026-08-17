/*
 * This file generated automatically from present.xml by c_client.py.
 * Edit at your peril.
 */

/**
 * @defgroup XCB_Present_API XCB Present API
 * @brief Present XCB Protocol Implementation.
 * @{
 **/

#ifndef __PRESENT_H
#define __PRESENT_H

#include "xcb.h"
#include "xproto.h"
#include "randr.h"
#include "xfixes.h"
#include "sync.h"

#ifdef __cplusplus
extern "C" {
#endif

#define XCB_PRESENT_MAJOR_VERSION 1
#define XCB_PRESENT_MINOR_VERSION 2

extern xcb_extension_t xcb_present_id;

typedef enum xcb_present_event_enum_t {
    XCB_PRESENT_EVENT_CONFIGURE_NOTIFY = 0,
    XCB_PRESENT_EVENT_COMPLETE_NOTIFY = 1,
    XCB_PRESENT_EVENT_IDLE_NOTIFY = 2,
    XCB_PRESENT_EVENT_REDIRECT_NOTIFY = 3
} xcb_present_event_enum_t;

typedef enum xcb_present_event_mask_t {
    XCB_PRESENT_EVENT_MASK_NO_EVENT = 0,
    XCB_PRESENT_EVENT_MASK_CONFIGURE_NOTIFY = 1,
    XCB_PRESENT_EVENT_MASK_COMPLETE_NOTIFY = 2,
    XCB_PRESENT_EVENT_MASK_IDLE_NOTIFY = 4,
    XCB_PRESENT_EVENT_MASK_REDIRECT_NOTIFY = 8
} xcb_present_event_mask_t;

typedef enum xcb_present_option_t {
    XCB_PRESENT_OPTION_NONE = 0,
    XCB_PRESENT_OPTION_ASYNC = 1,
    XCB_PRESENT_OPTION_COPY = 2,
    XCB_PRESENT_OPTION_UST = 4,
    XCB_PRESENT_OPTION_SUBOPTIMAL = 8
} xcb_present_option_t;

typedef enum xcb_present_capability_t {
    XCB_PRESENT_CAPABILITY_NONE = 0,
    XCB_PRESENT_CAPABILITY_ASYNC = 1,
    XCB_PRESENT_CAPABILITY_FENCE = 2,
    XCB_PRESENT_CAPABILITY_UST = 4
} xcb_present_capability_t;

typedef enum xcb_present_complete_kind_t {
    XCB_PRESENT_COMPLETE_KIND_PIXMAP = 0,
    XCB_PRESENT_COMPLETE_KIND_NOTIFY_MSC = 1
} xcb_present_complete_kind_t;

typedef enum xcb_present_complete_mode_t {
    XCB_PRESENT_COMPLETE_MODE_COPY = 0,
    XCB_PRESENT_COMPLETE_MODE_FLIP = 1,
    XCB_PRESENT_COMPLETE_MODE_SKIP = 2,
    XCB_PRESENT_COMPLETE_MODE_SUBOPTIMAL_COPY = 3
} xcb_present_complete_mode_t;

/**
 * @brief xcb_present_notify_t
 **/
typedef struct xcb_present_notify_t {
    xcb_window_t window;
    uint32_t     serial;
} xcb_present_notify_t;

/**
 * @brief xcb_present_notify_iterator_t
 **/
typedef struct xcb_present_notify_iterator_t {
    xcb_present_notify_t *data;
    int                   rem;
    int                   index;
} xcb_present_notify_iterator_t;

/**
 * @brief xcb_present_query_version_cookie_t
 **/
typedef struct xcb_present_query_version_cookie_t {
    unsigned int sequence;
} xcb_present_query_version_cookie_t;

/** Opcode for xcb_present_query_version. */
#define XCB_PRESENT_QUERY_VERSION 0

/**
 * @brief xcb_present_query_version_request_t
 **/
typedef struct xcb_present_query_version_request_t {
    uint8_t  major_opcode;
    uint8_t  minor_opcode;
    uint16_t length;
    uint32_t major_version;
    uint32_t minor_version;
} xcb_present_query_version_request_t;

/**
 * @brief xcb_present_query_version_reply_t
 **/
typedef struct xcb_present_query_version_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint32_t major_version;
    uint32_t minor_version;
} xcb_present_query_version_reply_t;

/** Opcode for xcb_present_pixmap. */
#define XCB_PRESENT_PIXMAP 1

/**
 * @brief xcb_present_pixmap_request_t
 **/
typedef struct xcb_present_pixmap_request_t {
    uint8_t             major_opcode;
    uint8_t             minor_opcode;
    uint16_t            length;
    xcb_window_t        window;
    xcb_pixmap_t        pixmap;
    uint32_t            serial;
    xcb_xfixes_region_t valid;
    xcb_xfixes_region_t update;
    int16_t             x_off;
    int16_t             y_off;
    xcb_randr_crtc_t    target_crtc;
    xcb_sync_fence_t    wait_fence;
    xcb_sync_fence_t    idle_fence;
    uint32_t            options;
    uint8_t             pad0[4];
    uint64_t            target_msc;
    uint64_t            divisor;
    uint64_t            remainder;
} xcb_present_pixmap_request_t;

/** Opcode for xcb_present_notify_msc. */
#define XCB_PRESENT_NOTIFY_MSC 2

/**
 * @brief xcb_present_notify_msc_request_t
 **/
typedef struct xcb_present_notify_msc_request_t {
    uint8_t      major_opcode;
    uint8_t      minor_opcode;
    uint16_t     length;
    xcb_window_t window;
    uint32_t     serial;
    uint8_t      pad0[4];
    uint64_t     target_msc;
    uint64_t     divisor;
    uint64_t     remainder;
} xcb_present_notify_msc_request_t;

typedef uint32_t xcb_present_event_t;

/**
 * @brief xcb_present_event_iterator_t
 **/
typedef struct xcb_present_event_iterator_t {
    xcb_present_event_t *data;
    int                  rem;
    int                  index;
} xcb_present_event_iterator_t;

/** Opcode for xcb_present_select_input. */
#define XCB_PRESENT_SELECT_INPUT 3

/**
 * @brief xcb_present_select_input_request_t
 **/
typedef struct xcb_present_select_input_request_t {
    uint8_t             major_opcode;
    uint8_t             minor_opcode;
    uint16_t            length;
    xcb_present_event_t eid;
    xcb_window_t        window;
    uint32_t            event_mask;
} xcb_present_select_input_request_t;

/**
 * @brief xcb_present_query_capabilities_cookie_t
 **/
typedef struct xcb_present_query_capabilities_cookie_t {
    unsigned int sequence;
} xcb_present_query_capabilities_cookie_t;

/** Opcode for xcb_present_query_capabilities. */
#define XCB_PRESENT_QUERY_CAPABILITIES 4

/**
 * @brief xcb_present_query_capabilities_request_t
 **/
typedef struct xcb_present_query_capabilities_request_t {
    uint8_t  major_opcode;
    uint8_t  minor_opcode;
    uint16_t length;
    uint32_t target;
} xcb_present_query_capabilities_request_t;

/**
 * @brief xcb_present_query_capabilities_reply_t
 **/
typedef struct xcb_present_query_capabilities_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint32_t capabilities;
} xcb_present_query_capabilities_reply_t;

/** Opcode for xcb_present_generic. */
#define XCB_PRESENT_GENERIC 0

/**
 * @brief xcb_present_generic_event_t
 **/
typedef struct xcb_present_generic_event_t {
    uint8_t             response_type;
    uint8_t             extension;
    uint16_t            sequence;
    uint32_t            length;
    uint16_t            evtype;
    uint8_t             pad0[2];
    xcb_present_event_t event;
} xcb_present_generic_event_t;

/** Opcode for xcb_present_configure_notify. */
#define XCB_PRESENT_CONFIGURE_NOTIFY 0

/**
 * @brief xcb_present_configure_notify_event_t
 **/
typedef struct xcb_present_configure_notify_event_t {
    uint8_t             response_type;
    uint8_t             extension;
    uint16_t            sequence;
    uint32_t            length;
    uint16_t            event_type;
    uint8_t             pad0[2];
    xcb_present_event_t event;
    xcb_window_t        window;
    int16_t             x;
    int16_t             y;
    uint16_t            width;
    uint16_t            height;
    int16_t             off_x;
    int16_t             off_y;
    uint32_t            full_sequence;
    uint16_t            pixmap_width;
    uint16_t            pixmap_height;
    uint32_t            pixmap_flags;
} xcb_present_configure_notify_event_t;

/** Opcode for xcb_present_complete_notify. */
#define XCB_PRESENT_COMPLETE_NOTIFY 1

/**
 * @brief xcb_present_complete_notify_event_t
 **/
typedef struct xcb_present_complete_notify_event_t {
    uint8_t             response_type;
    uint8_t             extension;
    uint16_t            sequence;
    uint32_t            length;
    uint16_t            event_type;
    uint8_t             kind;
    uint8_t             mode;
    xcb_present_event_t event;
    xcb_window_t        window;
    uint32_t            serial;
    uint64_t            ust;
    uint32_t            full_sequence;
    uint64_t            msc;
} XCB_PACKED xcb_present_complete_notify_event_t;

/** Opcode for xcb_present_idle_notify. */
#define XCB_PRESENT_IDLE_NOTIFY 2

/**
 * @brief xcb_present_idle_notify_event_t
 **/
typedef struct xcb_present_idle_notify_event_t {
    uint8_t             response_type;
    uint8_t             extension;
    uint16_t            sequence;
    uint32_t            length;
    uint16_t            event_type;
    uint8_t             pad0[2];
    xcb_present_event_t event;
    xcb_window_t        window;
    uint32_t            serial;
    xcb_pixmap_t        pixmap;
    xcb_sync_fence_t    idle_fence;
    uint32_t            full_sequence;
} xcb_present_idle_notify_event_t;

/** Opcode for xcb_present_redirect_notify. */
#define XCB_PRESENT_REDIRECT_NOTIFY 3

/**
 * @brief xcb_present_redirect_notify_event_t
 **/
typedef struct xcb_present_redirect_notify_event_t {
    uint8_t             response_type;
    uint8_t             extension;
    uint16_t            sequence;
    uint32_t            length;
    uint16_t            event_type;
    uint8_t             update_window;
    uint8_t             pad0;
    xcb_present_event_t event;
    xcb_window_t        event_window;
    xcb_window_t        window;
    xcb_pixmap_t        pixmap;
    uint32_t            serial;
    uint32_t            full_sequence;
    xcb_xfixes_region_t valid_region;
    xcb_xfixes_region_t update_region;
    xcb_rectangle_t     valid_rect;
    xcb_rectangle_t     update_rect;
    int16_t             x_off;
    int16_t             y_off;
    xcb_randr_crtc_t    target_crtc;
    xcb_sync_fence_t    wait_fence;
    xcb_sync_fence_t    idle_fence;
    uint32_t            options;
    uint8_t             pad1[4];
    uint64_t            target_msc;
    uint64_t            divisor;
    uint64_t            remainder;
} XCB_PACKED xcb_present_redirect_notify_event_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_present_notify_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_present_notify_t)
 */
void
xcb_present_notify_next (xcb_present_notify_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_present_notify_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_present_notify_end (xcb_present_notify_iterator_t i);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_present_query_version_cookie_t
xcb_present_query_version (xcb_connection_t *c,
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
xcb_present_query_version_cookie_t
xcb_present_query_version_unchecked (xcb_connection_t *c,
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
 * xcb_present_query_version_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_present_query_version_reply_t *
xcb_present_query_version_reply (xcb_connection_t                    *c,
                                 xcb_present_query_version_cookie_t   cookie  /**< */,
                                 xcb_generic_error_t                **e);

int
xcb_present_pixmap_sizeof (const void  *_buffer,
                           uint32_t     notifies_len);

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
xcb_present_pixmap_checked (xcb_connection_t           *c,
                            xcb_window_t                window,
                            xcb_pixmap_t                pixmap,
                            uint32_t                    serial,
                            xcb_xfixes_region_t         valid,
                            xcb_xfixes_region_t         update,
                            int16_t                     x_off,
                            int16_t                     y_off,
                            xcb_randr_crtc_t            target_crtc,
                            xcb_sync_fence_t            wait_fence,
                            xcb_sync_fence_t            idle_fence,
                            uint32_t                    options,
                            uint64_t                    target_msc,
                            uint64_t                    divisor,
                            uint64_t                    remainder,
                            uint32_t                    notifies_len,
                            const xcb_present_notify_t *notifies);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_present_pixmap (xcb_connection_t           *c,
                    xcb_window_t                window,
                    xcb_pixmap_t                pixmap,
                    uint32_t                    serial,
                    xcb_xfixes_region_t         valid,
                    xcb_xfixes_region_t         update,
                    int16_t                     x_off,
                    int16_t                     y_off,
                    xcb_randr_crtc_t            target_crtc,
                    xcb_sync_fence_t            wait_fence,
                    xcb_sync_fence_t            idle_fence,
                    uint32_t                    options,
                    uint64_t                    target_msc,
                    uint64_t                    divisor,
                    uint64_t                    remainder,
                    uint32_t                    notifies_len,
                    const xcb_present_notify_t *notifies);

xcb_present_notify_t *
xcb_present_pixmap_notifies (const xcb_present_pixmap_request_t *R);

int
xcb_present_pixmap_notifies_length (const xcb_present_pixmap_request_t *R);

xcb_present_notify_iterator_t
xcb_present_pixmap_notifies_iterator (const xcb_present_pixmap_request_t *R);

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
xcb_present_notify_msc_checked (xcb_connection_t *c,
                                xcb_window_t      window,
                                uint32_t          serial,
                                uint64_t          target_msc,
                                uint64_t          divisor,
                                uint64_t          remainder);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_present_notify_msc (xcb_connection_t *c,
                        xcb_window_t      window,
                        uint32_t          serial,
                        uint64_t          target_msc,
                        uint64_t          divisor,
                        uint64_t          remainder);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_present_event_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_present_event_t)
 */
void
xcb_present_event_next (xcb_present_event_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_present_event_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_present_event_end (xcb_present_event_iterator_t i);

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
xcb_present_select_input_checked (xcb_connection_t    *c,
                                  xcb_present_event_t  eid,
                                  xcb_window_t         window,
                                  uint32_t             event_mask);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_present_select_input (xcb_connection_t    *c,
                          xcb_present_event_t  eid,
                          xcb_window_t         window,
                          uint32_t             event_mask);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_present_query_capabilities_cookie_t
xcb_present_query_capabilities (xcb_connection_t *c,
                                uint32_t          target);

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
xcb_present_query_capabilities_cookie_t
xcb_present_query_capabilities_unchecked (xcb_connection_t *c,
                                          uint32_t          target);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_present_query_capabilities_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_present_query_capabilities_reply_t *
xcb_present_query_capabilities_reply (xcb_connection_t                         *c,
                                      xcb_present_query_capabilities_cookie_t   cookie  /**< */,
                                      xcb_generic_error_t                     **e);

int
xcb_present_redirect_notify_sizeof (const void  *_buffer,
                                    uint32_t     notifies_len);

xcb_present_notify_t *
xcb_present_redirect_notify_notifies (const xcb_present_redirect_notify_event_t *R);

int
xcb_present_redirect_notify_notifies_length (const xcb_present_redirect_notify_event_t *R);

xcb_present_notify_iterator_t
xcb_present_redirect_notify_notifies_iterator (const xcb_present_redirect_notify_event_t *R);


#ifdef __cplusplus
}
#endif

#endif

/**
 * @}
 */
