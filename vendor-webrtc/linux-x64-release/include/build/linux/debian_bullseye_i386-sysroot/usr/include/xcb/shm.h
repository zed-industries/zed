/*
 * This file generated automatically from shm.xml by c_client.py.
 * Edit at your peril.
 */

/**
 * @defgroup XCB_Shm_API XCB Shm API
 * @brief Shm XCB Protocol Implementation.
 * @{
 **/

#ifndef __SHM_H
#define __SHM_H

#include "xcb.h"
#include "xproto.h"

#ifdef __cplusplus
extern "C" {
#endif

#define XCB_SHM_MAJOR_VERSION 1
#define XCB_SHM_MINOR_VERSION 2

extern xcb_extension_t xcb_shm_id;

typedef uint32_t xcb_shm_seg_t;

/**
 * @brief xcb_shm_seg_iterator_t
 **/
typedef struct xcb_shm_seg_iterator_t {
    xcb_shm_seg_t *data;
    int            rem;
    int            index;
} xcb_shm_seg_iterator_t;

/** Opcode for xcb_shm_completion. */
#define XCB_SHM_COMPLETION 0

/**
 * @brief xcb_shm_completion_event_t
 **/
typedef struct xcb_shm_completion_event_t {
    uint8_t        response_type;
    uint8_t        pad0;
    uint16_t       sequence;
    xcb_drawable_t drawable;
    uint16_t       minor_event;
    uint8_t        major_event;
    uint8_t        pad1;
    xcb_shm_seg_t  shmseg;
    uint32_t       offset;
} xcb_shm_completion_event_t;

/** Opcode for xcb_shm_bad_seg. */
#define XCB_SHM_BAD_SEG 0

typedef xcb_value_error_t xcb_shm_bad_seg_error_t;

/**
 * @brief xcb_shm_query_version_cookie_t
 **/
typedef struct xcb_shm_query_version_cookie_t {
    unsigned int sequence;
} xcb_shm_query_version_cookie_t;

/** Opcode for xcb_shm_query_version. */
#define XCB_SHM_QUERY_VERSION 0

/**
 * @brief xcb_shm_query_version_request_t
 **/
typedef struct xcb_shm_query_version_request_t {
    uint8_t  major_opcode;
    uint8_t  minor_opcode;
    uint16_t length;
} xcb_shm_query_version_request_t;

/**
 * @brief xcb_shm_query_version_reply_t
 **/
typedef struct xcb_shm_query_version_reply_t {
    uint8_t  response_type;
    uint8_t  shared_pixmaps;
    uint16_t sequence;
    uint32_t length;
    uint16_t major_version;
    uint16_t minor_version;
    uint16_t uid;
    uint16_t gid;
    uint8_t  pixmap_format;
    uint8_t  pad0[15];
} xcb_shm_query_version_reply_t;

/** Opcode for xcb_shm_attach. */
#define XCB_SHM_ATTACH 1

/**
 * @brief xcb_shm_attach_request_t
 **/
typedef struct xcb_shm_attach_request_t {
    uint8_t       major_opcode;
    uint8_t       minor_opcode;
    uint16_t      length;
    xcb_shm_seg_t shmseg;
    uint32_t      shmid;
    uint8_t       read_only;
    uint8_t       pad0[3];
} xcb_shm_attach_request_t;

/** Opcode for xcb_shm_detach. */
#define XCB_SHM_DETACH 2

/**
 * @brief xcb_shm_detach_request_t
 **/
typedef struct xcb_shm_detach_request_t {
    uint8_t       major_opcode;
    uint8_t       minor_opcode;
    uint16_t      length;
    xcb_shm_seg_t shmseg;
} xcb_shm_detach_request_t;

/** Opcode for xcb_shm_put_image. */
#define XCB_SHM_PUT_IMAGE 3

/**
 * @brief xcb_shm_put_image_request_t
 **/
typedef struct xcb_shm_put_image_request_t {
    uint8_t        major_opcode;
    uint8_t        minor_opcode;
    uint16_t       length;
    xcb_drawable_t drawable;
    xcb_gcontext_t gc;
    uint16_t       total_width;
    uint16_t       total_height;
    uint16_t       src_x;
    uint16_t       src_y;
    uint16_t       src_width;
    uint16_t       src_height;
    int16_t        dst_x;
    int16_t        dst_y;
    uint8_t        depth;
    uint8_t        format;
    uint8_t        send_event;
    uint8_t        pad0;
    xcb_shm_seg_t  shmseg;
    uint32_t       offset;
} xcb_shm_put_image_request_t;

/**
 * @brief xcb_shm_get_image_cookie_t
 **/
typedef struct xcb_shm_get_image_cookie_t {
    unsigned int sequence;
} xcb_shm_get_image_cookie_t;

/** Opcode for xcb_shm_get_image. */
#define XCB_SHM_GET_IMAGE 4

/**
 * @brief xcb_shm_get_image_request_t
 **/
typedef struct xcb_shm_get_image_request_t {
    uint8_t        major_opcode;
    uint8_t        minor_opcode;
    uint16_t       length;
    xcb_drawable_t drawable;
    int16_t        x;
    int16_t        y;
    uint16_t       width;
    uint16_t       height;
    uint32_t       plane_mask;
    uint8_t        format;
    uint8_t        pad0[3];
    xcb_shm_seg_t  shmseg;
    uint32_t       offset;
} xcb_shm_get_image_request_t;

/**
 * @brief xcb_shm_get_image_reply_t
 **/
typedef struct xcb_shm_get_image_reply_t {
    uint8_t        response_type;
    uint8_t        depth;
    uint16_t       sequence;
    uint32_t       length;
    xcb_visualid_t visual;
    uint32_t       size;
} xcb_shm_get_image_reply_t;

/** Opcode for xcb_shm_create_pixmap. */
#define XCB_SHM_CREATE_PIXMAP 5

/**
 * @brief xcb_shm_create_pixmap_request_t
 **/
typedef struct xcb_shm_create_pixmap_request_t {
    uint8_t        major_opcode;
    uint8_t        minor_opcode;
    uint16_t       length;
    xcb_pixmap_t   pid;
    xcb_drawable_t drawable;
    uint16_t       width;
    uint16_t       height;
    uint8_t        depth;
    uint8_t        pad0[3];
    xcb_shm_seg_t  shmseg;
    uint32_t       offset;
} xcb_shm_create_pixmap_request_t;

/** Opcode for xcb_shm_attach_fd. */
#define XCB_SHM_ATTACH_FD 6

/**
 * @brief xcb_shm_attach_fd_request_t
 **/
typedef struct xcb_shm_attach_fd_request_t {
    uint8_t       major_opcode;
    uint8_t       minor_opcode;
    uint16_t      length;
    xcb_shm_seg_t shmseg;
    uint8_t       read_only;
    uint8_t       pad0[3];
} xcb_shm_attach_fd_request_t;

/**
 * @brief xcb_shm_create_segment_cookie_t
 **/
typedef struct xcb_shm_create_segment_cookie_t {
    unsigned int sequence;
} xcb_shm_create_segment_cookie_t;

/** Opcode for xcb_shm_create_segment. */
#define XCB_SHM_CREATE_SEGMENT 7

/**
 * @brief xcb_shm_create_segment_request_t
 **/
typedef struct xcb_shm_create_segment_request_t {
    uint8_t       major_opcode;
    uint8_t       minor_opcode;
    uint16_t      length;
    xcb_shm_seg_t shmseg;
    uint32_t      size;
    uint8_t       read_only;
    uint8_t       pad0[3];
} xcb_shm_create_segment_request_t;

/**
 * @brief xcb_shm_create_segment_reply_t
 **/
typedef struct xcb_shm_create_segment_reply_t {
    uint8_t  response_type;
    uint8_t  nfd;
    uint16_t sequence;
    uint32_t length;
    uint8_t  pad0[24];
} xcb_shm_create_segment_reply_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_shm_seg_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_shm_seg_t)
 */
void
xcb_shm_seg_next (xcb_shm_seg_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_shm_seg_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_shm_seg_end (xcb_shm_seg_iterator_t i);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_shm_query_version_cookie_t
xcb_shm_query_version (xcb_connection_t *c);

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
xcb_shm_query_version_cookie_t
xcb_shm_query_version_unchecked (xcb_connection_t *c);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_shm_query_version_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_shm_query_version_reply_t *
xcb_shm_query_version_reply (xcb_connection_t                *c,
                             xcb_shm_query_version_cookie_t   cookie  /**< */,
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
xcb_shm_attach_checked (xcb_connection_t *c,
                        xcb_shm_seg_t     shmseg,
                        uint32_t          shmid,
                        uint8_t           read_only);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_shm_attach (xcb_connection_t *c,
                xcb_shm_seg_t     shmseg,
                uint32_t          shmid,
                uint8_t           read_only);

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
xcb_shm_detach_checked (xcb_connection_t *c,
                        xcb_shm_seg_t     shmseg);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_shm_detach (xcb_connection_t *c,
                xcb_shm_seg_t     shmseg);

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
xcb_shm_put_image_checked (xcb_connection_t *c,
                           xcb_drawable_t    drawable,
                           xcb_gcontext_t    gc,
                           uint16_t          total_width,
                           uint16_t          total_height,
                           uint16_t          src_x,
                           uint16_t          src_y,
                           uint16_t          src_width,
                           uint16_t          src_height,
                           int16_t           dst_x,
                           int16_t           dst_y,
                           uint8_t           depth,
                           uint8_t           format,
                           uint8_t           send_event,
                           xcb_shm_seg_t     shmseg,
                           uint32_t          offset);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_shm_put_image (xcb_connection_t *c,
                   xcb_drawable_t    drawable,
                   xcb_gcontext_t    gc,
                   uint16_t          total_width,
                   uint16_t          total_height,
                   uint16_t          src_x,
                   uint16_t          src_y,
                   uint16_t          src_width,
                   uint16_t          src_height,
                   int16_t           dst_x,
                   int16_t           dst_y,
                   uint8_t           depth,
                   uint8_t           format,
                   uint8_t           send_event,
                   xcb_shm_seg_t     shmseg,
                   uint32_t          offset);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_shm_get_image_cookie_t
xcb_shm_get_image (xcb_connection_t *c,
                   xcb_drawable_t    drawable,
                   int16_t           x,
                   int16_t           y,
                   uint16_t          width,
                   uint16_t          height,
                   uint32_t          plane_mask,
                   uint8_t           format,
                   xcb_shm_seg_t     shmseg,
                   uint32_t          offset);

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
xcb_shm_get_image_cookie_t
xcb_shm_get_image_unchecked (xcb_connection_t *c,
                             xcb_drawable_t    drawable,
                             int16_t           x,
                             int16_t           y,
                             uint16_t          width,
                             uint16_t          height,
                             uint32_t          plane_mask,
                             uint8_t           format,
                             xcb_shm_seg_t     shmseg,
                             uint32_t          offset);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_shm_get_image_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_shm_get_image_reply_t *
xcb_shm_get_image_reply (xcb_connection_t            *c,
                         xcb_shm_get_image_cookie_t   cookie  /**< */,
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
xcb_shm_create_pixmap_checked (xcb_connection_t *c,
                               xcb_pixmap_t      pid,
                               xcb_drawable_t    drawable,
                               uint16_t          width,
                               uint16_t          height,
                               uint8_t           depth,
                               xcb_shm_seg_t     shmseg,
                               uint32_t          offset);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_shm_create_pixmap (xcb_connection_t *c,
                       xcb_pixmap_t      pid,
                       xcb_drawable_t    drawable,
                       uint16_t          width,
                       uint16_t          height,
                       uint8_t           depth,
                       xcb_shm_seg_t     shmseg,
                       uint32_t          offset);

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
xcb_shm_attach_fd_checked (xcb_connection_t *c,
                           xcb_shm_seg_t     shmseg,
                           int32_t           shm_fd,
                           uint8_t           read_only);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_shm_attach_fd (xcb_connection_t *c,
                   xcb_shm_seg_t     shmseg,
                   int32_t           shm_fd,
                   uint8_t           read_only);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_shm_create_segment_cookie_t
xcb_shm_create_segment (xcb_connection_t *c,
                        xcb_shm_seg_t     shmseg,
                        uint32_t          size,
                        uint8_t           read_only);

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
xcb_shm_create_segment_cookie_t
xcb_shm_create_segment_unchecked (xcb_connection_t *c,
                                  xcb_shm_seg_t     shmseg,
                                  uint32_t          size,
                                  uint8_t           read_only);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_shm_create_segment_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_shm_create_segment_reply_t *
xcb_shm_create_segment_reply (xcb_connection_t                 *c,
                              xcb_shm_create_segment_cookie_t   cookie  /**< */,
                              xcb_generic_error_t             **e);

/**
 * Return the reply fds
 * @param c      The connection
 * @param reply  The reply
 *
 * Returns the array of reply fds of the request asked by
 *
 * The returned value must be freed by the caller using free().
 */
int *
xcb_shm_create_segment_reply_fds (xcb_connection_t                *c  /**< */,
                                  xcb_shm_create_segment_reply_t  *reply);


#ifdef __cplusplus
}
#endif

#endif

/**
 * @}
 */
