/*
 * This file generated automatically from glx.xml by c_client.py.
 * Edit at your peril.
 */

/**
 * @defgroup XCB_Glx_API XCB Glx API
 * @brief Glx XCB Protocol Implementation.
 * @{
 **/

#ifndef __GLX_H
#define __GLX_H

#include "xcb.h"
#include "xproto.h"

#ifdef __cplusplus
extern "C" {
#endif

#define XCB_GLX_MAJOR_VERSION 1
#define XCB_GLX_MINOR_VERSION 4

extern xcb_extension_t xcb_glx_id;

typedef uint32_t xcb_glx_pixmap_t;

/**
 * @brief xcb_glx_pixmap_iterator_t
 **/
typedef struct xcb_glx_pixmap_iterator_t {
    xcb_glx_pixmap_t *data;
    int               rem;
    int               index;
} xcb_glx_pixmap_iterator_t;

typedef uint32_t xcb_glx_context_t;

/**
 * @brief xcb_glx_context_iterator_t
 **/
typedef struct xcb_glx_context_iterator_t {
    xcb_glx_context_t *data;
    int                rem;
    int                index;
} xcb_glx_context_iterator_t;

typedef uint32_t xcb_glx_pbuffer_t;

/**
 * @brief xcb_glx_pbuffer_iterator_t
 **/
typedef struct xcb_glx_pbuffer_iterator_t {
    xcb_glx_pbuffer_t *data;
    int                rem;
    int                index;
} xcb_glx_pbuffer_iterator_t;

typedef uint32_t xcb_glx_window_t;

/**
 * @brief xcb_glx_window_iterator_t
 **/
typedef struct xcb_glx_window_iterator_t {
    xcb_glx_window_t *data;
    int               rem;
    int               index;
} xcb_glx_window_iterator_t;

typedef uint32_t xcb_glx_fbconfig_t;

/**
 * @brief xcb_glx_fbconfig_iterator_t
 **/
typedef struct xcb_glx_fbconfig_iterator_t {
    xcb_glx_fbconfig_t *data;
    int                 rem;
    int                 index;
} xcb_glx_fbconfig_iterator_t;

typedef uint32_t xcb_glx_drawable_t;

/**
 * @brief xcb_glx_drawable_iterator_t
 **/
typedef struct xcb_glx_drawable_iterator_t {
    xcb_glx_drawable_t *data;
    int                 rem;
    int                 index;
} xcb_glx_drawable_iterator_t;

typedef float xcb_glx_float32_t;

/**
 * @brief xcb_glx_float32_iterator_t
 **/
typedef struct xcb_glx_float32_iterator_t {
    xcb_glx_float32_t *data;
    int                rem;
    int                index;
} xcb_glx_float32_iterator_t;

typedef double xcb_glx_float64_t;

/**
 * @brief xcb_glx_float64_iterator_t
 **/
typedef struct xcb_glx_float64_iterator_t {
    xcb_glx_float64_t *data;
    int                rem;
    int                index;
} xcb_glx_float64_iterator_t;

typedef uint32_t xcb_glx_bool32_t;

/**
 * @brief xcb_glx_bool32_iterator_t
 **/
typedef struct xcb_glx_bool32_iterator_t {
    xcb_glx_bool32_t *data;
    int               rem;
    int               index;
} xcb_glx_bool32_iterator_t;

typedef uint32_t xcb_glx_context_tag_t;

/**
 * @brief xcb_glx_context_tag_iterator_t
 **/
typedef struct xcb_glx_context_tag_iterator_t {
    xcb_glx_context_tag_t *data;
    int                    rem;
    int                    index;
} xcb_glx_context_tag_iterator_t;

/** Opcode for xcb_glx_generic. */
#define XCB_GLX_GENERIC -1

/**
 * @brief xcb_glx_generic_error_t
 **/
typedef struct xcb_glx_generic_error_t {
    uint8_t  response_type;
    uint8_t  error_code;
    uint16_t sequence;
    uint32_t bad_value;
    uint16_t minor_opcode;
    uint8_t  major_opcode;
    uint8_t  pad0[21];
} xcb_glx_generic_error_t;

/** Opcode for xcb_glx_bad_context. */
#define XCB_GLX_BAD_CONTEXT 0

typedef xcb_glx_generic_error_t xcb_glx_bad_context_error_t;

/** Opcode for xcb_glx_bad_context_state. */
#define XCB_GLX_BAD_CONTEXT_STATE 1

typedef xcb_glx_generic_error_t xcb_glx_bad_context_state_error_t;

/** Opcode for xcb_glx_bad_drawable. */
#define XCB_GLX_BAD_DRAWABLE 2

typedef xcb_glx_generic_error_t xcb_glx_bad_drawable_error_t;

/** Opcode for xcb_glx_bad_pixmap. */
#define XCB_GLX_BAD_PIXMAP 3

typedef xcb_glx_generic_error_t xcb_glx_bad_pixmap_error_t;

/** Opcode for xcb_glx_bad_context_tag. */
#define XCB_GLX_BAD_CONTEXT_TAG 4

typedef xcb_glx_generic_error_t xcb_glx_bad_context_tag_error_t;

/** Opcode for xcb_glx_bad_current_window. */
#define XCB_GLX_BAD_CURRENT_WINDOW 5

typedef xcb_glx_generic_error_t xcb_glx_bad_current_window_error_t;

/** Opcode for xcb_glx_bad_render_request. */
#define XCB_GLX_BAD_RENDER_REQUEST 6

typedef xcb_glx_generic_error_t xcb_glx_bad_render_request_error_t;

/** Opcode for xcb_glx_bad_large_request. */
#define XCB_GLX_BAD_LARGE_REQUEST 7

typedef xcb_glx_generic_error_t xcb_glx_bad_large_request_error_t;

/** Opcode for xcb_glx_unsupported_private_request. */
#define XCB_GLX_UNSUPPORTED_PRIVATE_REQUEST 8

typedef xcb_glx_generic_error_t xcb_glx_unsupported_private_request_error_t;

/** Opcode for xcb_glx_bad_fb_config. */
#define XCB_GLX_BAD_FB_CONFIG 9

typedef xcb_glx_generic_error_t xcb_glx_bad_fb_config_error_t;

/** Opcode for xcb_glx_bad_pbuffer. */
#define XCB_GLX_BAD_PBUFFER 10

typedef xcb_glx_generic_error_t xcb_glx_bad_pbuffer_error_t;

/** Opcode for xcb_glx_bad_current_drawable. */
#define XCB_GLX_BAD_CURRENT_DRAWABLE 11

typedef xcb_glx_generic_error_t xcb_glx_bad_current_drawable_error_t;

/** Opcode for xcb_glx_bad_window. */
#define XCB_GLX_BAD_WINDOW 12

typedef xcb_glx_generic_error_t xcb_glx_bad_window_error_t;

/** Opcode for xcb_glx_glx_bad_profile_arb. */
#define XCB_GLX_GLX_BAD_PROFILE_ARB 13

typedef xcb_glx_generic_error_t xcb_glx_glx_bad_profile_arb_error_t;

/** Opcode for xcb_glx_pbuffer_clobber. */
#define XCB_GLX_PBUFFER_CLOBBER 0

/**
 * @brief xcb_glx_pbuffer_clobber_event_t
 **/
typedef struct xcb_glx_pbuffer_clobber_event_t {
    uint8_t            response_type;
    uint8_t            pad0;
    uint16_t           sequence;
    uint16_t           event_type;
    uint16_t           draw_type;
    xcb_glx_drawable_t drawable;
    uint32_t           b_mask;
    uint16_t           aux_buffer;
    uint16_t           x;
    uint16_t           y;
    uint16_t           width;
    uint16_t           height;
    uint16_t           count;
    uint8_t            pad1[4];
} xcb_glx_pbuffer_clobber_event_t;

/** Opcode for xcb_glx_buffer_swap_complete. */
#define XCB_GLX_BUFFER_SWAP_COMPLETE 1

/**
 * @brief xcb_glx_buffer_swap_complete_event_t
 **/
typedef struct xcb_glx_buffer_swap_complete_event_t {
    uint8_t            response_type;
    uint8_t            pad0;
    uint16_t           sequence;
    uint16_t           event_type;
    uint8_t            pad1[2];
    xcb_glx_drawable_t drawable;
    uint32_t           ust_hi;
    uint32_t           ust_lo;
    uint32_t           msc_hi;
    uint32_t           msc_lo;
    uint32_t           sbc;
} xcb_glx_buffer_swap_complete_event_t;

typedef enum xcb_glx_pbcet_t {
    XCB_GLX_PBCET_DAMAGED = 32791,
    XCB_GLX_PBCET_SAVED = 32792
} xcb_glx_pbcet_t;

typedef enum xcb_glx_pbcdt_t {
    XCB_GLX_PBCDT_WINDOW = 32793,
    XCB_GLX_PBCDT_PBUFFER = 32794
} xcb_glx_pbcdt_t;

/** Opcode for xcb_glx_render. */
#define XCB_GLX_RENDER 1

/**
 * @brief xcb_glx_render_request_t
 **/
typedef struct xcb_glx_render_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
} xcb_glx_render_request_t;

/** Opcode for xcb_glx_render_large. */
#define XCB_GLX_RENDER_LARGE 2

/**
 * @brief xcb_glx_render_large_request_t
 **/
typedef struct xcb_glx_render_large_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
    uint16_t              request_num;
    uint16_t              request_total;
    uint32_t              data_len;
} xcb_glx_render_large_request_t;

/** Opcode for xcb_glx_create_context. */
#define XCB_GLX_CREATE_CONTEXT 3

/**
 * @brief xcb_glx_create_context_request_t
 **/
typedef struct xcb_glx_create_context_request_t {
    uint8_t           major_opcode;
    uint8_t           minor_opcode;
    uint16_t          length;
    xcb_glx_context_t context;
    xcb_visualid_t    visual;
    uint32_t          screen;
    xcb_glx_context_t share_list;
    uint8_t           is_direct;
    uint8_t           pad0[3];
} xcb_glx_create_context_request_t;

/** Opcode for xcb_glx_destroy_context. */
#define XCB_GLX_DESTROY_CONTEXT 4

/**
 * @brief xcb_glx_destroy_context_request_t
 **/
typedef struct xcb_glx_destroy_context_request_t {
    uint8_t           major_opcode;
    uint8_t           minor_opcode;
    uint16_t          length;
    xcb_glx_context_t context;
} xcb_glx_destroy_context_request_t;

/**
 * @brief xcb_glx_make_current_cookie_t
 **/
typedef struct xcb_glx_make_current_cookie_t {
    unsigned int sequence;
} xcb_glx_make_current_cookie_t;

/** Opcode for xcb_glx_make_current. */
#define XCB_GLX_MAKE_CURRENT 5

/**
 * @brief xcb_glx_make_current_request_t
 **/
typedef struct xcb_glx_make_current_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_drawable_t    drawable;
    xcb_glx_context_t     context;
    xcb_glx_context_tag_t old_context_tag;
} xcb_glx_make_current_request_t;

/**
 * @brief xcb_glx_make_current_reply_t
 **/
typedef struct xcb_glx_make_current_reply_t {
    uint8_t               response_type;
    uint8_t               pad0;
    uint16_t              sequence;
    uint32_t              length;
    xcb_glx_context_tag_t context_tag;
    uint8_t               pad1[20];
} xcb_glx_make_current_reply_t;

/**
 * @brief xcb_glx_is_direct_cookie_t
 **/
typedef struct xcb_glx_is_direct_cookie_t {
    unsigned int sequence;
} xcb_glx_is_direct_cookie_t;

/** Opcode for xcb_glx_is_direct. */
#define XCB_GLX_IS_DIRECT 6

/**
 * @brief xcb_glx_is_direct_request_t
 **/
typedef struct xcb_glx_is_direct_request_t {
    uint8_t           major_opcode;
    uint8_t           minor_opcode;
    uint16_t          length;
    xcb_glx_context_t context;
} xcb_glx_is_direct_request_t;

/**
 * @brief xcb_glx_is_direct_reply_t
 **/
typedef struct xcb_glx_is_direct_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint8_t  is_direct;
    uint8_t  pad1[23];
} xcb_glx_is_direct_reply_t;

/**
 * @brief xcb_glx_query_version_cookie_t
 **/
typedef struct xcb_glx_query_version_cookie_t {
    unsigned int sequence;
} xcb_glx_query_version_cookie_t;

/** Opcode for xcb_glx_query_version. */
#define XCB_GLX_QUERY_VERSION 7

/**
 * @brief xcb_glx_query_version_request_t
 **/
typedef struct xcb_glx_query_version_request_t {
    uint8_t  major_opcode;
    uint8_t  minor_opcode;
    uint16_t length;
    uint32_t major_version;
    uint32_t minor_version;
} xcb_glx_query_version_request_t;

/**
 * @brief xcb_glx_query_version_reply_t
 **/
typedef struct xcb_glx_query_version_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint32_t major_version;
    uint32_t minor_version;
    uint8_t  pad1[16];
} xcb_glx_query_version_reply_t;

/** Opcode for xcb_glx_wait_gl. */
#define XCB_GLX_WAIT_GL 8

/**
 * @brief xcb_glx_wait_gl_request_t
 **/
typedef struct xcb_glx_wait_gl_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
} xcb_glx_wait_gl_request_t;

/** Opcode for xcb_glx_wait_x. */
#define XCB_GLX_WAIT_X 9

/**
 * @brief xcb_glx_wait_x_request_t
 **/
typedef struct xcb_glx_wait_x_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
} xcb_glx_wait_x_request_t;

/** Opcode for xcb_glx_copy_context. */
#define XCB_GLX_COPY_CONTEXT 10

/**
 * @brief xcb_glx_copy_context_request_t
 **/
typedef struct xcb_glx_copy_context_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_t     src;
    xcb_glx_context_t     dest;
    uint32_t              mask;
    xcb_glx_context_tag_t src_context_tag;
} xcb_glx_copy_context_request_t;

typedef enum xcb_glx_gc_t {
    XCB_GLX_GC_GL_CURRENT_BIT = 1,
    XCB_GLX_GC_GL_POINT_BIT = 2,
    XCB_GLX_GC_GL_LINE_BIT = 4,
    XCB_GLX_GC_GL_POLYGON_BIT = 8,
    XCB_GLX_GC_GL_POLYGON_STIPPLE_BIT = 16,
    XCB_GLX_GC_GL_PIXEL_MODE_BIT = 32,
    XCB_GLX_GC_GL_LIGHTING_BIT = 64,
    XCB_GLX_GC_GL_FOG_BIT = 128,
    XCB_GLX_GC_GL_DEPTH_BUFFER_BIT = 256,
    XCB_GLX_GC_GL_ACCUM_BUFFER_BIT = 512,
    XCB_GLX_GC_GL_STENCIL_BUFFER_BIT = 1024,
    XCB_GLX_GC_GL_VIEWPORT_BIT = 2048,
    XCB_GLX_GC_GL_TRANSFORM_BIT = 4096,
    XCB_GLX_GC_GL_ENABLE_BIT = 8192,
    XCB_GLX_GC_GL_COLOR_BUFFER_BIT = 16384,
    XCB_GLX_GC_GL_HINT_BIT = 32768,
    XCB_GLX_GC_GL_EVAL_BIT = 65536,
    XCB_GLX_GC_GL_LIST_BIT = 131072,
    XCB_GLX_GC_GL_TEXTURE_BIT = 262144,
    XCB_GLX_GC_GL_SCISSOR_BIT = 524288,
    XCB_GLX_GC_GL_ALL_ATTRIB_BITS = 16777215
} xcb_glx_gc_t;

/** Opcode for xcb_glx_swap_buffers. */
#define XCB_GLX_SWAP_BUFFERS 11

/**
 * @brief xcb_glx_swap_buffers_request_t
 **/
typedef struct xcb_glx_swap_buffers_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
    xcb_glx_drawable_t    drawable;
} xcb_glx_swap_buffers_request_t;

/** Opcode for xcb_glx_use_x_font. */
#define XCB_GLX_USE_X_FONT 12

/**
 * @brief xcb_glx_use_x_font_request_t
 **/
typedef struct xcb_glx_use_x_font_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
    xcb_font_t            font;
    uint32_t              first;
    uint32_t              count;
    uint32_t              list_base;
} xcb_glx_use_x_font_request_t;

/** Opcode for xcb_glx_create_glx_pixmap. */
#define XCB_GLX_CREATE_GLX_PIXMAP 13

/**
 * @brief xcb_glx_create_glx_pixmap_request_t
 **/
typedef struct xcb_glx_create_glx_pixmap_request_t {
    uint8_t          major_opcode;
    uint8_t          minor_opcode;
    uint16_t         length;
    uint32_t         screen;
    xcb_visualid_t   visual;
    xcb_pixmap_t     pixmap;
    xcb_glx_pixmap_t glx_pixmap;
} xcb_glx_create_glx_pixmap_request_t;

/**
 * @brief xcb_glx_get_visual_configs_cookie_t
 **/
typedef struct xcb_glx_get_visual_configs_cookie_t {
    unsigned int sequence;
} xcb_glx_get_visual_configs_cookie_t;

/** Opcode for xcb_glx_get_visual_configs. */
#define XCB_GLX_GET_VISUAL_CONFIGS 14

/**
 * @brief xcb_glx_get_visual_configs_request_t
 **/
typedef struct xcb_glx_get_visual_configs_request_t {
    uint8_t  major_opcode;
    uint8_t  minor_opcode;
    uint16_t length;
    uint32_t screen;
} xcb_glx_get_visual_configs_request_t;

/**
 * @brief xcb_glx_get_visual_configs_reply_t
 **/
typedef struct xcb_glx_get_visual_configs_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint32_t num_visuals;
    uint32_t num_properties;
    uint8_t  pad1[16];
} xcb_glx_get_visual_configs_reply_t;

/** Opcode for xcb_glx_destroy_glx_pixmap. */
#define XCB_GLX_DESTROY_GLX_PIXMAP 15

/**
 * @brief xcb_glx_destroy_glx_pixmap_request_t
 **/
typedef struct xcb_glx_destroy_glx_pixmap_request_t {
    uint8_t          major_opcode;
    uint8_t          minor_opcode;
    uint16_t         length;
    xcb_glx_pixmap_t glx_pixmap;
} xcb_glx_destroy_glx_pixmap_request_t;

/** Opcode for xcb_glx_vendor_private. */
#define XCB_GLX_VENDOR_PRIVATE 16

/**
 * @brief xcb_glx_vendor_private_request_t
 **/
typedef struct xcb_glx_vendor_private_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    uint32_t              vendor_code;
    xcb_glx_context_tag_t context_tag;
} xcb_glx_vendor_private_request_t;

/**
 * @brief xcb_glx_vendor_private_with_reply_cookie_t
 **/
typedef struct xcb_glx_vendor_private_with_reply_cookie_t {
    unsigned int sequence;
} xcb_glx_vendor_private_with_reply_cookie_t;

/** Opcode for xcb_glx_vendor_private_with_reply. */
#define XCB_GLX_VENDOR_PRIVATE_WITH_REPLY 17

/**
 * @brief xcb_glx_vendor_private_with_reply_request_t
 **/
typedef struct xcb_glx_vendor_private_with_reply_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    uint32_t              vendor_code;
    xcb_glx_context_tag_t context_tag;
} xcb_glx_vendor_private_with_reply_request_t;

/**
 * @brief xcb_glx_vendor_private_with_reply_reply_t
 **/
typedef struct xcb_glx_vendor_private_with_reply_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint32_t retval;
    uint8_t  data1[24];
} xcb_glx_vendor_private_with_reply_reply_t;

/**
 * @brief xcb_glx_query_extensions_string_cookie_t
 **/
typedef struct xcb_glx_query_extensions_string_cookie_t {
    unsigned int sequence;
} xcb_glx_query_extensions_string_cookie_t;

/** Opcode for xcb_glx_query_extensions_string. */
#define XCB_GLX_QUERY_EXTENSIONS_STRING 18

/**
 * @brief xcb_glx_query_extensions_string_request_t
 **/
typedef struct xcb_glx_query_extensions_string_request_t {
    uint8_t  major_opcode;
    uint8_t  minor_opcode;
    uint16_t length;
    uint32_t screen;
} xcb_glx_query_extensions_string_request_t;

/**
 * @brief xcb_glx_query_extensions_string_reply_t
 **/
typedef struct xcb_glx_query_extensions_string_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint8_t  pad1[4];
    uint32_t n;
    uint8_t  pad2[16];
} xcb_glx_query_extensions_string_reply_t;

/**
 * @brief xcb_glx_query_server_string_cookie_t
 **/
typedef struct xcb_glx_query_server_string_cookie_t {
    unsigned int sequence;
} xcb_glx_query_server_string_cookie_t;

/** Opcode for xcb_glx_query_server_string. */
#define XCB_GLX_QUERY_SERVER_STRING 19

/**
 * @brief xcb_glx_query_server_string_request_t
 **/
typedef struct xcb_glx_query_server_string_request_t {
    uint8_t  major_opcode;
    uint8_t  minor_opcode;
    uint16_t length;
    uint32_t screen;
    uint32_t name;
} xcb_glx_query_server_string_request_t;

/**
 * @brief xcb_glx_query_server_string_reply_t
 **/
typedef struct xcb_glx_query_server_string_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint8_t  pad1[4];
    uint32_t str_len;
    uint8_t  pad2[16];
} xcb_glx_query_server_string_reply_t;

/** Opcode for xcb_glx_client_info. */
#define XCB_GLX_CLIENT_INFO 20

/**
 * @brief xcb_glx_client_info_request_t
 **/
typedef struct xcb_glx_client_info_request_t {
    uint8_t  major_opcode;
    uint8_t  minor_opcode;
    uint16_t length;
    uint32_t major_version;
    uint32_t minor_version;
    uint32_t str_len;
} xcb_glx_client_info_request_t;

/**
 * @brief xcb_glx_get_fb_configs_cookie_t
 **/
typedef struct xcb_glx_get_fb_configs_cookie_t {
    unsigned int sequence;
} xcb_glx_get_fb_configs_cookie_t;

/** Opcode for xcb_glx_get_fb_configs. */
#define XCB_GLX_GET_FB_CONFIGS 21

/**
 * @brief xcb_glx_get_fb_configs_request_t
 **/
typedef struct xcb_glx_get_fb_configs_request_t {
    uint8_t  major_opcode;
    uint8_t  minor_opcode;
    uint16_t length;
    uint32_t screen;
} xcb_glx_get_fb_configs_request_t;

/**
 * @brief xcb_glx_get_fb_configs_reply_t
 **/
typedef struct xcb_glx_get_fb_configs_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint32_t num_FB_configs;
    uint32_t num_properties;
    uint8_t  pad1[16];
} xcb_glx_get_fb_configs_reply_t;

/** Opcode for xcb_glx_create_pixmap. */
#define XCB_GLX_CREATE_PIXMAP 22

/**
 * @brief xcb_glx_create_pixmap_request_t
 **/
typedef struct xcb_glx_create_pixmap_request_t {
    uint8_t            major_opcode;
    uint8_t            minor_opcode;
    uint16_t           length;
    uint32_t           screen;
    xcb_glx_fbconfig_t fbconfig;
    xcb_pixmap_t       pixmap;
    xcb_glx_pixmap_t   glx_pixmap;
    uint32_t           num_attribs;
} xcb_glx_create_pixmap_request_t;

/** Opcode for xcb_glx_destroy_pixmap. */
#define XCB_GLX_DESTROY_PIXMAP 23

/**
 * @brief xcb_glx_destroy_pixmap_request_t
 **/
typedef struct xcb_glx_destroy_pixmap_request_t {
    uint8_t          major_opcode;
    uint8_t          minor_opcode;
    uint16_t         length;
    xcb_glx_pixmap_t glx_pixmap;
} xcb_glx_destroy_pixmap_request_t;

/** Opcode for xcb_glx_create_new_context. */
#define XCB_GLX_CREATE_NEW_CONTEXT 24

/**
 * @brief xcb_glx_create_new_context_request_t
 **/
typedef struct xcb_glx_create_new_context_request_t {
    uint8_t            major_opcode;
    uint8_t            minor_opcode;
    uint16_t           length;
    xcb_glx_context_t  context;
    xcb_glx_fbconfig_t fbconfig;
    uint32_t           screen;
    uint32_t           render_type;
    xcb_glx_context_t  share_list;
    uint8_t            is_direct;
    uint8_t            pad0[3];
} xcb_glx_create_new_context_request_t;

/**
 * @brief xcb_glx_query_context_cookie_t
 **/
typedef struct xcb_glx_query_context_cookie_t {
    unsigned int sequence;
} xcb_glx_query_context_cookie_t;

/** Opcode for xcb_glx_query_context. */
#define XCB_GLX_QUERY_CONTEXT 25

/**
 * @brief xcb_glx_query_context_request_t
 **/
typedef struct xcb_glx_query_context_request_t {
    uint8_t           major_opcode;
    uint8_t           minor_opcode;
    uint16_t          length;
    xcb_glx_context_t context;
} xcb_glx_query_context_request_t;

/**
 * @brief xcb_glx_query_context_reply_t
 **/
typedef struct xcb_glx_query_context_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint32_t num_attribs;
    uint8_t  pad1[20];
} xcb_glx_query_context_reply_t;

/**
 * @brief xcb_glx_make_context_current_cookie_t
 **/
typedef struct xcb_glx_make_context_current_cookie_t {
    unsigned int sequence;
} xcb_glx_make_context_current_cookie_t;

/** Opcode for xcb_glx_make_context_current. */
#define XCB_GLX_MAKE_CONTEXT_CURRENT 26

/**
 * @brief xcb_glx_make_context_current_request_t
 **/
typedef struct xcb_glx_make_context_current_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t old_context_tag;
    xcb_glx_drawable_t    drawable;
    xcb_glx_drawable_t    read_drawable;
    xcb_glx_context_t     context;
} xcb_glx_make_context_current_request_t;

/**
 * @brief xcb_glx_make_context_current_reply_t
 **/
typedef struct xcb_glx_make_context_current_reply_t {
    uint8_t               response_type;
    uint8_t               pad0;
    uint16_t              sequence;
    uint32_t              length;
    xcb_glx_context_tag_t context_tag;
    uint8_t               pad1[20];
} xcb_glx_make_context_current_reply_t;

/** Opcode for xcb_glx_create_pbuffer. */
#define XCB_GLX_CREATE_PBUFFER 27

/**
 * @brief xcb_glx_create_pbuffer_request_t
 **/
typedef struct xcb_glx_create_pbuffer_request_t {
    uint8_t            major_opcode;
    uint8_t            minor_opcode;
    uint16_t           length;
    uint32_t           screen;
    xcb_glx_fbconfig_t fbconfig;
    xcb_glx_pbuffer_t  pbuffer;
    uint32_t           num_attribs;
} xcb_glx_create_pbuffer_request_t;

/** Opcode for xcb_glx_destroy_pbuffer. */
#define XCB_GLX_DESTROY_PBUFFER 28

/**
 * @brief xcb_glx_destroy_pbuffer_request_t
 **/
typedef struct xcb_glx_destroy_pbuffer_request_t {
    uint8_t           major_opcode;
    uint8_t           minor_opcode;
    uint16_t          length;
    xcb_glx_pbuffer_t pbuffer;
} xcb_glx_destroy_pbuffer_request_t;

/**
 * @brief xcb_glx_get_drawable_attributes_cookie_t
 **/
typedef struct xcb_glx_get_drawable_attributes_cookie_t {
    unsigned int sequence;
} xcb_glx_get_drawable_attributes_cookie_t;

/** Opcode for xcb_glx_get_drawable_attributes. */
#define XCB_GLX_GET_DRAWABLE_ATTRIBUTES 29

/**
 * @brief xcb_glx_get_drawable_attributes_request_t
 **/
typedef struct xcb_glx_get_drawable_attributes_request_t {
    uint8_t            major_opcode;
    uint8_t            minor_opcode;
    uint16_t           length;
    xcb_glx_drawable_t drawable;
} xcb_glx_get_drawable_attributes_request_t;

/**
 * @brief xcb_glx_get_drawable_attributes_reply_t
 **/
typedef struct xcb_glx_get_drawable_attributes_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint32_t num_attribs;
    uint8_t  pad1[20];
} xcb_glx_get_drawable_attributes_reply_t;

/** Opcode for xcb_glx_change_drawable_attributes. */
#define XCB_GLX_CHANGE_DRAWABLE_ATTRIBUTES 30

/**
 * @brief xcb_glx_change_drawable_attributes_request_t
 **/
typedef struct xcb_glx_change_drawable_attributes_request_t {
    uint8_t            major_opcode;
    uint8_t            minor_opcode;
    uint16_t           length;
    xcb_glx_drawable_t drawable;
    uint32_t           num_attribs;
} xcb_glx_change_drawable_attributes_request_t;

/** Opcode for xcb_glx_create_window. */
#define XCB_GLX_CREATE_WINDOW 31

/**
 * @brief xcb_glx_create_window_request_t
 **/
typedef struct xcb_glx_create_window_request_t {
    uint8_t            major_opcode;
    uint8_t            minor_opcode;
    uint16_t           length;
    uint32_t           screen;
    xcb_glx_fbconfig_t fbconfig;
    xcb_window_t       window;
    xcb_glx_window_t   glx_window;
    uint32_t           num_attribs;
} xcb_glx_create_window_request_t;

/** Opcode for xcb_glx_delete_window. */
#define XCB_GLX_DELETE_WINDOW 32

/**
 * @brief xcb_glx_delete_window_request_t
 **/
typedef struct xcb_glx_delete_window_request_t {
    uint8_t          major_opcode;
    uint8_t          minor_opcode;
    uint16_t         length;
    xcb_glx_window_t glxwindow;
} xcb_glx_delete_window_request_t;

/** Opcode for xcb_glx_set_client_info_arb. */
#define XCB_GLX_SET_CLIENT_INFO_ARB 33

/**
 * @brief xcb_glx_set_client_info_arb_request_t
 **/
typedef struct xcb_glx_set_client_info_arb_request_t {
    uint8_t  major_opcode;
    uint8_t  minor_opcode;
    uint16_t length;
    uint32_t major_version;
    uint32_t minor_version;
    uint32_t num_versions;
    uint32_t gl_str_len;
    uint32_t glx_str_len;
} xcb_glx_set_client_info_arb_request_t;

/** Opcode for xcb_glx_create_context_attribs_arb. */
#define XCB_GLX_CREATE_CONTEXT_ATTRIBS_ARB 34

/**
 * @brief xcb_glx_create_context_attribs_arb_request_t
 **/
typedef struct xcb_glx_create_context_attribs_arb_request_t {
    uint8_t            major_opcode;
    uint8_t            minor_opcode;
    uint16_t           length;
    xcb_glx_context_t  context;
    xcb_glx_fbconfig_t fbconfig;
    uint32_t           screen;
    xcb_glx_context_t  share_list;
    uint8_t            is_direct;
    uint8_t            pad0[3];
    uint32_t           num_attribs;
} xcb_glx_create_context_attribs_arb_request_t;

/** Opcode for xcb_glx_set_client_info_2arb. */
#define XCB_GLX_SET_CLIENT_INFO_2ARB 35

/**
 * @brief xcb_glx_set_client_info_2arb_request_t
 **/
typedef struct xcb_glx_set_client_info_2arb_request_t {
    uint8_t  major_opcode;
    uint8_t  minor_opcode;
    uint16_t length;
    uint32_t major_version;
    uint32_t minor_version;
    uint32_t num_versions;
    uint32_t gl_str_len;
    uint32_t glx_str_len;
} xcb_glx_set_client_info_2arb_request_t;

/** Opcode for xcb_glx_new_list. */
#define XCB_GLX_NEW_LIST 101

/**
 * @brief xcb_glx_new_list_request_t
 **/
typedef struct xcb_glx_new_list_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
    uint32_t              list;
    uint32_t              mode;
} xcb_glx_new_list_request_t;

/** Opcode for xcb_glx_end_list. */
#define XCB_GLX_END_LIST 102

/**
 * @brief xcb_glx_end_list_request_t
 **/
typedef struct xcb_glx_end_list_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
} xcb_glx_end_list_request_t;

/** Opcode for xcb_glx_delete_lists. */
#define XCB_GLX_DELETE_LISTS 103

/**
 * @brief xcb_glx_delete_lists_request_t
 **/
typedef struct xcb_glx_delete_lists_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
    uint32_t              list;
    int32_t               range;
} xcb_glx_delete_lists_request_t;

/**
 * @brief xcb_glx_gen_lists_cookie_t
 **/
typedef struct xcb_glx_gen_lists_cookie_t {
    unsigned int sequence;
} xcb_glx_gen_lists_cookie_t;

/** Opcode for xcb_glx_gen_lists. */
#define XCB_GLX_GEN_LISTS 104

/**
 * @brief xcb_glx_gen_lists_request_t
 **/
typedef struct xcb_glx_gen_lists_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
    int32_t               range;
} xcb_glx_gen_lists_request_t;

/**
 * @brief xcb_glx_gen_lists_reply_t
 **/
typedef struct xcb_glx_gen_lists_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint32_t ret_val;
} xcb_glx_gen_lists_reply_t;

/** Opcode for xcb_glx_feedback_buffer. */
#define XCB_GLX_FEEDBACK_BUFFER 105

/**
 * @brief xcb_glx_feedback_buffer_request_t
 **/
typedef struct xcb_glx_feedback_buffer_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
    int32_t               size;
    int32_t               type;
} xcb_glx_feedback_buffer_request_t;

/** Opcode for xcb_glx_select_buffer. */
#define XCB_GLX_SELECT_BUFFER 106

/**
 * @brief xcb_glx_select_buffer_request_t
 **/
typedef struct xcb_glx_select_buffer_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
    int32_t               size;
} xcb_glx_select_buffer_request_t;

/**
 * @brief xcb_glx_render_mode_cookie_t
 **/
typedef struct xcb_glx_render_mode_cookie_t {
    unsigned int sequence;
} xcb_glx_render_mode_cookie_t;

/** Opcode for xcb_glx_render_mode. */
#define XCB_GLX_RENDER_MODE 107

/**
 * @brief xcb_glx_render_mode_request_t
 **/
typedef struct xcb_glx_render_mode_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
    uint32_t              mode;
} xcb_glx_render_mode_request_t;

/**
 * @brief xcb_glx_render_mode_reply_t
 **/
typedef struct xcb_glx_render_mode_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint32_t ret_val;
    uint32_t n;
    uint32_t new_mode;
    uint8_t  pad1[12];
} xcb_glx_render_mode_reply_t;

typedef enum xcb_glx_rm_t {
    XCB_GLX_RM_GL_RENDER = 7168,
    XCB_GLX_RM_GL_FEEDBACK = 7169,
    XCB_GLX_RM_GL_SELECT = 7170
} xcb_glx_rm_t;

/**
 * @brief xcb_glx_finish_cookie_t
 **/
typedef struct xcb_glx_finish_cookie_t {
    unsigned int sequence;
} xcb_glx_finish_cookie_t;

/** Opcode for xcb_glx_finish. */
#define XCB_GLX_FINISH 108

/**
 * @brief xcb_glx_finish_request_t
 **/
typedef struct xcb_glx_finish_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
} xcb_glx_finish_request_t;

/**
 * @brief xcb_glx_finish_reply_t
 **/
typedef struct xcb_glx_finish_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
} xcb_glx_finish_reply_t;

/** Opcode for xcb_glx_pixel_storef. */
#define XCB_GLX_PIXEL_STOREF 109

/**
 * @brief xcb_glx_pixel_storef_request_t
 **/
typedef struct xcb_glx_pixel_storef_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
    uint32_t              pname;
    xcb_glx_float32_t     datum;
} xcb_glx_pixel_storef_request_t;

/** Opcode for xcb_glx_pixel_storei. */
#define XCB_GLX_PIXEL_STOREI 110

/**
 * @brief xcb_glx_pixel_storei_request_t
 **/
typedef struct xcb_glx_pixel_storei_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
    uint32_t              pname;
    int32_t               datum;
} xcb_glx_pixel_storei_request_t;

/**
 * @brief xcb_glx_read_pixels_cookie_t
 **/
typedef struct xcb_glx_read_pixels_cookie_t {
    unsigned int sequence;
} xcb_glx_read_pixels_cookie_t;

/** Opcode for xcb_glx_read_pixels. */
#define XCB_GLX_READ_PIXELS 111

/**
 * @brief xcb_glx_read_pixels_request_t
 **/
typedef struct xcb_glx_read_pixels_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
    int32_t               x;
    int32_t               y;
    int32_t               width;
    int32_t               height;
    uint32_t              format;
    uint32_t              type;
    uint8_t               swap_bytes;
    uint8_t               lsb_first;
} xcb_glx_read_pixels_request_t;

/**
 * @brief xcb_glx_read_pixels_reply_t
 **/
typedef struct xcb_glx_read_pixels_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint8_t  pad1[24];
} xcb_glx_read_pixels_reply_t;

/**
 * @brief xcb_glx_get_booleanv_cookie_t
 **/
typedef struct xcb_glx_get_booleanv_cookie_t {
    unsigned int sequence;
} xcb_glx_get_booleanv_cookie_t;

/** Opcode for xcb_glx_get_booleanv. */
#define XCB_GLX_GET_BOOLEANV 112

/**
 * @brief xcb_glx_get_booleanv_request_t
 **/
typedef struct xcb_glx_get_booleanv_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
    int32_t               pname;
} xcb_glx_get_booleanv_request_t;

/**
 * @brief xcb_glx_get_booleanv_reply_t
 **/
typedef struct xcb_glx_get_booleanv_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint8_t  pad1[4];
    uint32_t n;
    uint8_t  datum;
    uint8_t  pad2[15];
} xcb_glx_get_booleanv_reply_t;

/**
 * @brief xcb_glx_get_clip_plane_cookie_t
 **/
typedef struct xcb_glx_get_clip_plane_cookie_t {
    unsigned int sequence;
} xcb_glx_get_clip_plane_cookie_t;

/** Opcode for xcb_glx_get_clip_plane. */
#define XCB_GLX_GET_CLIP_PLANE 113

/**
 * @brief xcb_glx_get_clip_plane_request_t
 **/
typedef struct xcb_glx_get_clip_plane_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
    int32_t               plane;
} xcb_glx_get_clip_plane_request_t;

/**
 * @brief xcb_glx_get_clip_plane_reply_t
 **/
typedef struct xcb_glx_get_clip_plane_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint8_t  pad1[24];
} xcb_glx_get_clip_plane_reply_t;

/**
 * @brief xcb_glx_get_doublev_cookie_t
 **/
typedef struct xcb_glx_get_doublev_cookie_t {
    unsigned int sequence;
} xcb_glx_get_doublev_cookie_t;

/** Opcode for xcb_glx_get_doublev. */
#define XCB_GLX_GET_DOUBLEV 114

/**
 * @brief xcb_glx_get_doublev_request_t
 **/
typedef struct xcb_glx_get_doublev_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
    uint32_t              pname;
} xcb_glx_get_doublev_request_t;

/**
 * @brief xcb_glx_get_doublev_reply_t
 **/
typedef struct xcb_glx_get_doublev_reply_t {
    uint8_t           response_type;
    uint8_t           pad0;
    uint16_t          sequence;
    uint32_t          length;
    uint8_t           pad1[4];
    uint32_t          n;
    xcb_glx_float64_t datum;
    uint8_t           pad2[8];
} xcb_glx_get_doublev_reply_t;

/**
 * @brief xcb_glx_get_error_cookie_t
 **/
typedef struct xcb_glx_get_error_cookie_t {
    unsigned int sequence;
} xcb_glx_get_error_cookie_t;

/** Opcode for xcb_glx_get_error. */
#define XCB_GLX_GET_ERROR 115

/**
 * @brief xcb_glx_get_error_request_t
 **/
typedef struct xcb_glx_get_error_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
} xcb_glx_get_error_request_t;

/**
 * @brief xcb_glx_get_error_reply_t
 **/
typedef struct xcb_glx_get_error_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    int32_t  error;
} xcb_glx_get_error_reply_t;

/**
 * @brief xcb_glx_get_floatv_cookie_t
 **/
typedef struct xcb_glx_get_floatv_cookie_t {
    unsigned int sequence;
} xcb_glx_get_floatv_cookie_t;

/** Opcode for xcb_glx_get_floatv. */
#define XCB_GLX_GET_FLOATV 116

/**
 * @brief xcb_glx_get_floatv_request_t
 **/
typedef struct xcb_glx_get_floatv_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
    uint32_t              pname;
} xcb_glx_get_floatv_request_t;

/**
 * @brief xcb_glx_get_floatv_reply_t
 **/
typedef struct xcb_glx_get_floatv_reply_t {
    uint8_t           response_type;
    uint8_t           pad0;
    uint16_t          sequence;
    uint32_t          length;
    uint8_t           pad1[4];
    uint32_t          n;
    xcb_glx_float32_t datum;
    uint8_t           pad2[12];
} xcb_glx_get_floatv_reply_t;

/**
 * @brief xcb_glx_get_integerv_cookie_t
 **/
typedef struct xcb_glx_get_integerv_cookie_t {
    unsigned int sequence;
} xcb_glx_get_integerv_cookie_t;

/** Opcode for xcb_glx_get_integerv. */
#define XCB_GLX_GET_INTEGERV 117

/**
 * @brief xcb_glx_get_integerv_request_t
 **/
typedef struct xcb_glx_get_integerv_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
    uint32_t              pname;
} xcb_glx_get_integerv_request_t;

/**
 * @brief xcb_glx_get_integerv_reply_t
 **/
typedef struct xcb_glx_get_integerv_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint8_t  pad1[4];
    uint32_t n;
    int32_t  datum;
    uint8_t  pad2[12];
} xcb_glx_get_integerv_reply_t;

/**
 * @brief xcb_glx_get_lightfv_cookie_t
 **/
typedef struct xcb_glx_get_lightfv_cookie_t {
    unsigned int sequence;
} xcb_glx_get_lightfv_cookie_t;

/** Opcode for xcb_glx_get_lightfv. */
#define XCB_GLX_GET_LIGHTFV 118

/**
 * @brief xcb_glx_get_lightfv_request_t
 **/
typedef struct xcb_glx_get_lightfv_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
    uint32_t              light;
    uint32_t              pname;
} xcb_glx_get_lightfv_request_t;

/**
 * @brief xcb_glx_get_lightfv_reply_t
 **/
typedef struct xcb_glx_get_lightfv_reply_t {
    uint8_t           response_type;
    uint8_t           pad0;
    uint16_t          sequence;
    uint32_t          length;
    uint8_t           pad1[4];
    uint32_t          n;
    xcb_glx_float32_t datum;
    uint8_t           pad2[12];
} xcb_glx_get_lightfv_reply_t;

/**
 * @brief xcb_glx_get_lightiv_cookie_t
 **/
typedef struct xcb_glx_get_lightiv_cookie_t {
    unsigned int sequence;
} xcb_glx_get_lightiv_cookie_t;

/** Opcode for xcb_glx_get_lightiv. */
#define XCB_GLX_GET_LIGHTIV 119

/**
 * @brief xcb_glx_get_lightiv_request_t
 **/
typedef struct xcb_glx_get_lightiv_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
    uint32_t              light;
    uint32_t              pname;
} xcb_glx_get_lightiv_request_t;

/**
 * @brief xcb_glx_get_lightiv_reply_t
 **/
typedef struct xcb_glx_get_lightiv_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint8_t  pad1[4];
    uint32_t n;
    int32_t  datum;
    uint8_t  pad2[12];
} xcb_glx_get_lightiv_reply_t;

/**
 * @brief xcb_glx_get_mapdv_cookie_t
 **/
typedef struct xcb_glx_get_mapdv_cookie_t {
    unsigned int sequence;
} xcb_glx_get_mapdv_cookie_t;

/** Opcode for xcb_glx_get_mapdv. */
#define XCB_GLX_GET_MAPDV 120

/**
 * @brief xcb_glx_get_mapdv_request_t
 **/
typedef struct xcb_glx_get_mapdv_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
    uint32_t              target;
    uint32_t              query;
} xcb_glx_get_mapdv_request_t;

/**
 * @brief xcb_glx_get_mapdv_reply_t
 **/
typedef struct xcb_glx_get_mapdv_reply_t {
    uint8_t           response_type;
    uint8_t           pad0;
    uint16_t          sequence;
    uint32_t          length;
    uint8_t           pad1[4];
    uint32_t          n;
    xcb_glx_float64_t datum;
    uint8_t           pad2[8];
} xcb_glx_get_mapdv_reply_t;

/**
 * @brief xcb_glx_get_mapfv_cookie_t
 **/
typedef struct xcb_glx_get_mapfv_cookie_t {
    unsigned int sequence;
} xcb_glx_get_mapfv_cookie_t;

/** Opcode for xcb_glx_get_mapfv. */
#define XCB_GLX_GET_MAPFV 121

/**
 * @brief xcb_glx_get_mapfv_request_t
 **/
typedef struct xcb_glx_get_mapfv_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
    uint32_t              target;
    uint32_t              query;
} xcb_glx_get_mapfv_request_t;

/**
 * @brief xcb_glx_get_mapfv_reply_t
 **/
typedef struct xcb_glx_get_mapfv_reply_t {
    uint8_t           response_type;
    uint8_t           pad0;
    uint16_t          sequence;
    uint32_t          length;
    uint8_t           pad1[4];
    uint32_t          n;
    xcb_glx_float32_t datum;
    uint8_t           pad2[12];
} xcb_glx_get_mapfv_reply_t;

/**
 * @brief xcb_glx_get_mapiv_cookie_t
 **/
typedef struct xcb_glx_get_mapiv_cookie_t {
    unsigned int sequence;
} xcb_glx_get_mapiv_cookie_t;

/** Opcode for xcb_glx_get_mapiv. */
#define XCB_GLX_GET_MAPIV 122

/**
 * @brief xcb_glx_get_mapiv_request_t
 **/
typedef struct xcb_glx_get_mapiv_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
    uint32_t              target;
    uint32_t              query;
} xcb_glx_get_mapiv_request_t;

/**
 * @brief xcb_glx_get_mapiv_reply_t
 **/
typedef struct xcb_glx_get_mapiv_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint8_t  pad1[4];
    uint32_t n;
    int32_t  datum;
    uint8_t  pad2[12];
} xcb_glx_get_mapiv_reply_t;

/**
 * @brief xcb_glx_get_materialfv_cookie_t
 **/
typedef struct xcb_glx_get_materialfv_cookie_t {
    unsigned int sequence;
} xcb_glx_get_materialfv_cookie_t;

/** Opcode for xcb_glx_get_materialfv. */
#define XCB_GLX_GET_MATERIALFV 123

/**
 * @brief xcb_glx_get_materialfv_request_t
 **/
typedef struct xcb_glx_get_materialfv_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
    uint32_t              face;
    uint32_t              pname;
} xcb_glx_get_materialfv_request_t;

/**
 * @brief xcb_glx_get_materialfv_reply_t
 **/
typedef struct xcb_glx_get_materialfv_reply_t {
    uint8_t           response_type;
    uint8_t           pad0;
    uint16_t          sequence;
    uint32_t          length;
    uint8_t           pad1[4];
    uint32_t          n;
    xcb_glx_float32_t datum;
    uint8_t           pad2[12];
} xcb_glx_get_materialfv_reply_t;

/**
 * @brief xcb_glx_get_materialiv_cookie_t
 **/
typedef struct xcb_glx_get_materialiv_cookie_t {
    unsigned int sequence;
} xcb_glx_get_materialiv_cookie_t;

/** Opcode for xcb_glx_get_materialiv. */
#define XCB_GLX_GET_MATERIALIV 124

/**
 * @brief xcb_glx_get_materialiv_request_t
 **/
typedef struct xcb_glx_get_materialiv_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
    uint32_t              face;
    uint32_t              pname;
} xcb_glx_get_materialiv_request_t;

/**
 * @brief xcb_glx_get_materialiv_reply_t
 **/
typedef struct xcb_glx_get_materialiv_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint8_t  pad1[4];
    uint32_t n;
    int32_t  datum;
    uint8_t  pad2[12];
} xcb_glx_get_materialiv_reply_t;

/**
 * @brief xcb_glx_get_pixel_mapfv_cookie_t
 **/
typedef struct xcb_glx_get_pixel_mapfv_cookie_t {
    unsigned int sequence;
} xcb_glx_get_pixel_mapfv_cookie_t;

/** Opcode for xcb_glx_get_pixel_mapfv. */
#define XCB_GLX_GET_PIXEL_MAPFV 125

/**
 * @brief xcb_glx_get_pixel_mapfv_request_t
 **/
typedef struct xcb_glx_get_pixel_mapfv_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
    uint32_t              map;
} xcb_glx_get_pixel_mapfv_request_t;

/**
 * @brief xcb_glx_get_pixel_mapfv_reply_t
 **/
typedef struct xcb_glx_get_pixel_mapfv_reply_t {
    uint8_t           response_type;
    uint8_t           pad0;
    uint16_t          sequence;
    uint32_t          length;
    uint8_t           pad1[4];
    uint32_t          n;
    xcb_glx_float32_t datum;
    uint8_t           pad2[12];
} xcb_glx_get_pixel_mapfv_reply_t;

/**
 * @brief xcb_glx_get_pixel_mapuiv_cookie_t
 **/
typedef struct xcb_glx_get_pixel_mapuiv_cookie_t {
    unsigned int sequence;
} xcb_glx_get_pixel_mapuiv_cookie_t;

/** Opcode for xcb_glx_get_pixel_mapuiv. */
#define XCB_GLX_GET_PIXEL_MAPUIV 126

/**
 * @brief xcb_glx_get_pixel_mapuiv_request_t
 **/
typedef struct xcb_glx_get_pixel_mapuiv_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
    uint32_t              map;
} xcb_glx_get_pixel_mapuiv_request_t;

/**
 * @brief xcb_glx_get_pixel_mapuiv_reply_t
 **/
typedef struct xcb_glx_get_pixel_mapuiv_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint8_t  pad1[4];
    uint32_t n;
    uint32_t datum;
    uint8_t  pad2[12];
} xcb_glx_get_pixel_mapuiv_reply_t;

/**
 * @brief xcb_glx_get_pixel_mapusv_cookie_t
 **/
typedef struct xcb_glx_get_pixel_mapusv_cookie_t {
    unsigned int sequence;
} xcb_glx_get_pixel_mapusv_cookie_t;

/** Opcode for xcb_glx_get_pixel_mapusv. */
#define XCB_GLX_GET_PIXEL_MAPUSV 127

/**
 * @brief xcb_glx_get_pixel_mapusv_request_t
 **/
typedef struct xcb_glx_get_pixel_mapusv_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
    uint32_t              map;
} xcb_glx_get_pixel_mapusv_request_t;

/**
 * @brief xcb_glx_get_pixel_mapusv_reply_t
 **/
typedef struct xcb_glx_get_pixel_mapusv_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint8_t  pad1[4];
    uint32_t n;
    uint16_t datum;
    uint8_t  pad2[16];
} xcb_glx_get_pixel_mapusv_reply_t;

/**
 * @brief xcb_glx_get_polygon_stipple_cookie_t
 **/
typedef struct xcb_glx_get_polygon_stipple_cookie_t {
    unsigned int sequence;
} xcb_glx_get_polygon_stipple_cookie_t;

/** Opcode for xcb_glx_get_polygon_stipple. */
#define XCB_GLX_GET_POLYGON_STIPPLE 128

/**
 * @brief xcb_glx_get_polygon_stipple_request_t
 **/
typedef struct xcb_glx_get_polygon_stipple_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
    uint8_t               lsb_first;
} xcb_glx_get_polygon_stipple_request_t;

/**
 * @brief xcb_glx_get_polygon_stipple_reply_t
 **/
typedef struct xcb_glx_get_polygon_stipple_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint8_t  pad1[24];
} xcb_glx_get_polygon_stipple_reply_t;

/**
 * @brief xcb_glx_get_string_cookie_t
 **/
typedef struct xcb_glx_get_string_cookie_t {
    unsigned int sequence;
} xcb_glx_get_string_cookie_t;

/** Opcode for xcb_glx_get_string. */
#define XCB_GLX_GET_STRING 129

/**
 * @brief xcb_glx_get_string_request_t
 **/
typedef struct xcb_glx_get_string_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
    uint32_t              name;
} xcb_glx_get_string_request_t;

/**
 * @brief xcb_glx_get_string_reply_t
 **/
typedef struct xcb_glx_get_string_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint8_t  pad1[4];
    uint32_t n;
    uint8_t  pad2[16];
} xcb_glx_get_string_reply_t;

/**
 * @brief xcb_glx_get_tex_envfv_cookie_t
 **/
typedef struct xcb_glx_get_tex_envfv_cookie_t {
    unsigned int sequence;
} xcb_glx_get_tex_envfv_cookie_t;

/** Opcode for xcb_glx_get_tex_envfv. */
#define XCB_GLX_GET_TEX_ENVFV 130

/**
 * @brief xcb_glx_get_tex_envfv_request_t
 **/
typedef struct xcb_glx_get_tex_envfv_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
    uint32_t              target;
    uint32_t              pname;
} xcb_glx_get_tex_envfv_request_t;

/**
 * @brief xcb_glx_get_tex_envfv_reply_t
 **/
typedef struct xcb_glx_get_tex_envfv_reply_t {
    uint8_t           response_type;
    uint8_t           pad0;
    uint16_t          sequence;
    uint32_t          length;
    uint8_t           pad1[4];
    uint32_t          n;
    xcb_glx_float32_t datum;
    uint8_t           pad2[12];
} xcb_glx_get_tex_envfv_reply_t;

/**
 * @brief xcb_glx_get_tex_enviv_cookie_t
 **/
typedef struct xcb_glx_get_tex_enviv_cookie_t {
    unsigned int sequence;
} xcb_glx_get_tex_enviv_cookie_t;

/** Opcode for xcb_glx_get_tex_enviv. */
#define XCB_GLX_GET_TEX_ENVIV 131

/**
 * @brief xcb_glx_get_tex_enviv_request_t
 **/
typedef struct xcb_glx_get_tex_enviv_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
    uint32_t              target;
    uint32_t              pname;
} xcb_glx_get_tex_enviv_request_t;

/**
 * @brief xcb_glx_get_tex_enviv_reply_t
 **/
typedef struct xcb_glx_get_tex_enviv_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint8_t  pad1[4];
    uint32_t n;
    int32_t  datum;
    uint8_t  pad2[12];
} xcb_glx_get_tex_enviv_reply_t;

/**
 * @brief xcb_glx_get_tex_gendv_cookie_t
 **/
typedef struct xcb_glx_get_tex_gendv_cookie_t {
    unsigned int sequence;
} xcb_glx_get_tex_gendv_cookie_t;

/** Opcode for xcb_glx_get_tex_gendv. */
#define XCB_GLX_GET_TEX_GENDV 132

/**
 * @brief xcb_glx_get_tex_gendv_request_t
 **/
typedef struct xcb_glx_get_tex_gendv_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
    uint32_t              coord;
    uint32_t              pname;
} xcb_glx_get_tex_gendv_request_t;

/**
 * @brief xcb_glx_get_tex_gendv_reply_t
 **/
typedef struct xcb_glx_get_tex_gendv_reply_t {
    uint8_t           response_type;
    uint8_t           pad0;
    uint16_t          sequence;
    uint32_t          length;
    uint8_t           pad1[4];
    uint32_t          n;
    xcb_glx_float64_t datum;
    uint8_t           pad2[8];
} xcb_glx_get_tex_gendv_reply_t;

/**
 * @brief xcb_glx_get_tex_genfv_cookie_t
 **/
typedef struct xcb_glx_get_tex_genfv_cookie_t {
    unsigned int sequence;
} xcb_glx_get_tex_genfv_cookie_t;

/** Opcode for xcb_glx_get_tex_genfv. */
#define XCB_GLX_GET_TEX_GENFV 133

/**
 * @brief xcb_glx_get_tex_genfv_request_t
 **/
typedef struct xcb_glx_get_tex_genfv_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
    uint32_t              coord;
    uint32_t              pname;
} xcb_glx_get_tex_genfv_request_t;

/**
 * @brief xcb_glx_get_tex_genfv_reply_t
 **/
typedef struct xcb_glx_get_tex_genfv_reply_t {
    uint8_t           response_type;
    uint8_t           pad0;
    uint16_t          sequence;
    uint32_t          length;
    uint8_t           pad1[4];
    uint32_t          n;
    xcb_glx_float32_t datum;
    uint8_t           pad2[12];
} xcb_glx_get_tex_genfv_reply_t;

/**
 * @brief xcb_glx_get_tex_geniv_cookie_t
 **/
typedef struct xcb_glx_get_tex_geniv_cookie_t {
    unsigned int sequence;
} xcb_glx_get_tex_geniv_cookie_t;

/** Opcode for xcb_glx_get_tex_geniv. */
#define XCB_GLX_GET_TEX_GENIV 134

/**
 * @brief xcb_glx_get_tex_geniv_request_t
 **/
typedef struct xcb_glx_get_tex_geniv_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
    uint32_t              coord;
    uint32_t              pname;
} xcb_glx_get_tex_geniv_request_t;

/**
 * @brief xcb_glx_get_tex_geniv_reply_t
 **/
typedef struct xcb_glx_get_tex_geniv_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint8_t  pad1[4];
    uint32_t n;
    int32_t  datum;
    uint8_t  pad2[12];
} xcb_glx_get_tex_geniv_reply_t;

/**
 * @brief xcb_glx_get_tex_image_cookie_t
 **/
typedef struct xcb_glx_get_tex_image_cookie_t {
    unsigned int sequence;
} xcb_glx_get_tex_image_cookie_t;

/** Opcode for xcb_glx_get_tex_image. */
#define XCB_GLX_GET_TEX_IMAGE 135

/**
 * @brief xcb_glx_get_tex_image_request_t
 **/
typedef struct xcb_glx_get_tex_image_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
    uint32_t              target;
    int32_t               level;
    uint32_t              format;
    uint32_t              type;
    uint8_t               swap_bytes;
} xcb_glx_get_tex_image_request_t;

/**
 * @brief xcb_glx_get_tex_image_reply_t
 **/
typedef struct xcb_glx_get_tex_image_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint8_t  pad1[8];
    int32_t  width;
    int32_t  height;
    int32_t  depth;
    uint8_t  pad2[4];
} xcb_glx_get_tex_image_reply_t;

/**
 * @brief xcb_glx_get_tex_parameterfv_cookie_t
 **/
typedef struct xcb_glx_get_tex_parameterfv_cookie_t {
    unsigned int sequence;
} xcb_glx_get_tex_parameterfv_cookie_t;

/** Opcode for xcb_glx_get_tex_parameterfv. */
#define XCB_GLX_GET_TEX_PARAMETERFV 136

/**
 * @brief xcb_glx_get_tex_parameterfv_request_t
 **/
typedef struct xcb_glx_get_tex_parameterfv_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
    uint32_t              target;
    uint32_t              pname;
} xcb_glx_get_tex_parameterfv_request_t;

/**
 * @brief xcb_glx_get_tex_parameterfv_reply_t
 **/
typedef struct xcb_glx_get_tex_parameterfv_reply_t {
    uint8_t           response_type;
    uint8_t           pad0;
    uint16_t          sequence;
    uint32_t          length;
    uint8_t           pad1[4];
    uint32_t          n;
    xcb_glx_float32_t datum;
    uint8_t           pad2[12];
} xcb_glx_get_tex_parameterfv_reply_t;

/**
 * @brief xcb_glx_get_tex_parameteriv_cookie_t
 **/
typedef struct xcb_glx_get_tex_parameteriv_cookie_t {
    unsigned int sequence;
} xcb_glx_get_tex_parameteriv_cookie_t;

/** Opcode for xcb_glx_get_tex_parameteriv. */
#define XCB_GLX_GET_TEX_PARAMETERIV 137

/**
 * @brief xcb_glx_get_tex_parameteriv_request_t
 **/
typedef struct xcb_glx_get_tex_parameteriv_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
    uint32_t              target;
    uint32_t              pname;
} xcb_glx_get_tex_parameteriv_request_t;

/**
 * @brief xcb_glx_get_tex_parameteriv_reply_t
 **/
typedef struct xcb_glx_get_tex_parameteriv_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint8_t  pad1[4];
    uint32_t n;
    int32_t  datum;
    uint8_t  pad2[12];
} xcb_glx_get_tex_parameteriv_reply_t;

/**
 * @brief xcb_glx_get_tex_level_parameterfv_cookie_t
 **/
typedef struct xcb_glx_get_tex_level_parameterfv_cookie_t {
    unsigned int sequence;
} xcb_glx_get_tex_level_parameterfv_cookie_t;

/** Opcode for xcb_glx_get_tex_level_parameterfv. */
#define XCB_GLX_GET_TEX_LEVEL_PARAMETERFV 138

/**
 * @brief xcb_glx_get_tex_level_parameterfv_request_t
 **/
typedef struct xcb_glx_get_tex_level_parameterfv_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
    uint32_t              target;
    int32_t               level;
    uint32_t              pname;
} xcb_glx_get_tex_level_parameterfv_request_t;

/**
 * @brief xcb_glx_get_tex_level_parameterfv_reply_t
 **/
typedef struct xcb_glx_get_tex_level_parameterfv_reply_t {
    uint8_t           response_type;
    uint8_t           pad0;
    uint16_t          sequence;
    uint32_t          length;
    uint8_t           pad1[4];
    uint32_t          n;
    xcb_glx_float32_t datum;
    uint8_t           pad2[12];
} xcb_glx_get_tex_level_parameterfv_reply_t;

/**
 * @brief xcb_glx_get_tex_level_parameteriv_cookie_t
 **/
typedef struct xcb_glx_get_tex_level_parameteriv_cookie_t {
    unsigned int sequence;
} xcb_glx_get_tex_level_parameteriv_cookie_t;

/** Opcode for xcb_glx_get_tex_level_parameteriv. */
#define XCB_GLX_GET_TEX_LEVEL_PARAMETERIV 139

/**
 * @brief xcb_glx_get_tex_level_parameteriv_request_t
 **/
typedef struct xcb_glx_get_tex_level_parameteriv_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
    uint32_t              target;
    int32_t               level;
    uint32_t              pname;
} xcb_glx_get_tex_level_parameteriv_request_t;

/**
 * @brief xcb_glx_get_tex_level_parameteriv_reply_t
 **/
typedef struct xcb_glx_get_tex_level_parameteriv_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint8_t  pad1[4];
    uint32_t n;
    int32_t  datum;
    uint8_t  pad2[12];
} xcb_glx_get_tex_level_parameteriv_reply_t;

/**
 * @brief xcb_glx_is_enabled_cookie_t
 **/
typedef struct xcb_glx_is_enabled_cookie_t {
    unsigned int sequence;
} xcb_glx_is_enabled_cookie_t;

/** Opcode for xcb_glx_is_enabled. */
#define XCB_GLX_IS_ENABLED 140

/**
 * @brief xcb_glx_is_enabled_request_t
 **/
typedef struct xcb_glx_is_enabled_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
    uint32_t              capability;
} xcb_glx_is_enabled_request_t;

/**
 * @brief xcb_glx_is_enabled_reply_t
 **/
typedef struct xcb_glx_is_enabled_reply_t {
    uint8_t          response_type;
    uint8_t          pad0;
    uint16_t         sequence;
    uint32_t         length;
    xcb_glx_bool32_t ret_val;
} xcb_glx_is_enabled_reply_t;

/**
 * @brief xcb_glx_is_list_cookie_t
 **/
typedef struct xcb_glx_is_list_cookie_t {
    unsigned int sequence;
} xcb_glx_is_list_cookie_t;

/** Opcode for xcb_glx_is_list. */
#define XCB_GLX_IS_LIST 141

/**
 * @brief xcb_glx_is_list_request_t
 **/
typedef struct xcb_glx_is_list_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
    uint32_t              list;
} xcb_glx_is_list_request_t;

/**
 * @brief xcb_glx_is_list_reply_t
 **/
typedef struct xcb_glx_is_list_reply_t {
    uint8_t          response_type;
    uint8_t          pad0;
    uint16_t         sequence;
    uint32_t         length;
    xcb_glx_bool32_t ret_val;
} xcb_glx_is_list_reply_t;

/** Opcode for xcb_glx_flush. */
#define XCB_GLX_FLUSH 142

/**
 * @brief xcb_glx_flush_request_t
 **/
typedef struct xcb_glx_flush_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
} xcb_glx_flush_request_t;

/**
 * @brief xcb_glx_are_textures_resident_cookie_t
 **/
typedef struct xcb_glx_are_textures_resident_cookie_t {
    unsigned int sequence;
} xcb_glx_are_textures_resident_cookie_t;

/** Opcode for xcb_glx_are_textures_resident. */
#define XCB_GLX_ARE_TEXTURES_RESIDENT 143

/**
 * @brief xcb_glx_are_textures_resident_request_t
 **/
typedef struct xcb_glx_are_textures_resident_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
    int32_t               n;
} xcb_glx_are_textures_resident_request_t;

/**
 * @brief xcb_glx_are_textures_resident_reply_t
 **/
typedef struct xcb_glx_are_textures_resident_reply_t {
    uint8_t          response_type;
    uint8_t          pad0;
    uint16_t         sequence;
    uint32_t         length;
    xcb_glx_bool32_t ret_val;
    uint8_t          pad1[20];
} xcb_glx_are_textures_resident_reply_t;

/** Opcode for xcb_glx_delete_textures. */
#define XCB_GLX_DELETE_TEXTURES 144

/**
 * @brief xcb_glx_delete_textures_request_t
 **/
typedef struct xcb_glx_delete_textures_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
    int32_t               n;
} xcb_glx_delete_textures_request_t;

/**
 * @brief xcb_glx_gen_textures_cookie_t
 **/
typedef struct xcb_glx_gen_textures_cookie_t {
    unsigned int sequence;
} xcb_glx_gen_textures_cookie_t;

/** Opcode for xcb_glx_gen_textures. */
#define XCB_GLX_GEN_TEXTURES 145

/**
 * @brief xcb_glx_gen_textures_request_t
 **/
typedef struct xcb_glx_gen_textures_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
    int32_t               n;
} xcb_glx_gen_textures_request_t;

/**
 * @brief xcb_glx_gen_textures_reply_t
 **/
typedef struct xcb_glx_gen_textures_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint8_t  pad1[24];
} xcb_glx_gen_textures_reply_t;

/**
 * @brief xcb_glx_is_texture_cookie_t
 **/
typedef struct xcb_glx_is_texture_cookie_t {
    unsigned int sequence;
} xcb_glx_is_texture_cookie_t;

/** Opcode for xcb_glx_is_texture. */
#define XCB_GLX_IS_TEXTURE 146

/**
 * @brief xcb_glx_is_texture_request_t
 **/
typedef struct xcb_glx_is_texture_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
    uint32_t              texture;
} xcb_glx_is_texture_request_t;

/**
 * @brief xcb_glx_is_texture_reply_t
 **/
typedef struct xcb_glx_is_texture_reply_t {
    uint8_t          response_type;
    uint8_t          pad0;
    uint16_t         sequence;
    uint32_t         length;
    xcb_glx_bool32_t ret_val;
} xcb_glx_is_texture_reply_t;

/**
 * @brief xcb_glx_get_color_table_cookie_t
 **/
typedef struct xcb_glx_get_color_table_cookie_t {
    unsigned int sequence;
} xcb_glx_get_color_table_cookie_t;

/** Opcode for xcb_glx_get_color_table. */
#define XCB_GLX_GET_COLOR_TABLE 147

/**
 * @brief xcb_glx_get_color_table_request_t
 **/
typedef struct xcb_glx_get_color_table_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
    uint32_t              target;
    uint32_t              format;
    uint32_t              type;
    uint8_t               swap_bytes;
} xcb_glx_get_color_table_request_t;

/**
 * @brief xcb_glx_get_color_table_reply_t
 **/
typedef struct xcb_glx_get_color_table_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint8_t  pad1[8];
    int32_t  width;
    uint8_t  pad2[12];
} xcb_glx_get_color_table_reply_t;

/**
 * @brief xcb_glx_get_color_table_parameterfv_cookie_t
 **/
typedef struct xcb_glx_get_color_table_parameterfv_cookie_t {
    unsigned int sequence;
} xcb_glx_get_color_table_parameterfv_cookie_t;

/** Opcode for xcb_glx_get_color_table_parameterfv. */
#define XCB_GLX_GET_COLOR_TABLE_PARAMETERFV 148

/**
 * @brief xcb_glx_get_color_table_parameterfv_request_t
 **/
typedef struct xcb_glx_get_color_table_parameterfv_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
    uint32_t              target;
    uint32_t              pname;
} xcb_glx_get_color_table_parameterfv_request_t;

/**
 * @brief xcb_glx_get_color_table_parameterfv_reply_t
 **/
typedef struct xcb_glx_get_color_table_parameterfv_reply_t {
    uint8_t           response_type;
    uint8_t           pad0;
    uint16_t          sequence;
    uint32_t          length;
    uint8_t           pad1[4];
    uint32_t          n;
    xcb_glx_float32_t datum;
    uint8_t           pad2[12];
} xcb_glx_get_color_table_parameterfv_reply_t;

/**
 * @brief xcb_glx_get_color_table_parameteriv_cookie_t
 **/
typedef struct xcb_glx_get_color_table_parameteriv_cookie_t {
    unsigned int sequence;
} xcb_glx_get_color_table_parameteriv_cookie_t;

/** Opcode for xcb_glx_get_color_table_parameteriv. */
#define XCB_GLX_GET_COLOR_TABLE_PARAMETERIV 149

/**
 * @brief xcb_glx_get_color_table_parameteriv_request_t
 **/
typedef struct xcb_glx_get_color_table_parameteriv_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
    uint32_t              target;
    uint32_t              pname;
} xcb_glx_get_color_table_parameteriv_request_t;

/**
 * @brief xcb_glx_get_color_table_parameteriv_reply_t
 **/
typedef struct xcb_glx_get_color_table_parameteriv_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint8_t  pad1[4];
    uint32_t n;
    int32_t  datum;
    uint8_t  pad2[12];
} xcb_glx_get_color_table_parameteriv_reply_t;

/**
 * @brief xcb_glx_get_convolution_filter_cookie_t
 **/
typedef struct xcb_glx_get_convolution_filter_cookie_t {
    unsigned int sequence;
} xcb_glx_get_convolution_filter_cookie_t;

/** Opcode for xcb_glx_get_convolution_filter. */
#define XCB_GLX_GET_CONVOLUTION_FILTER 150

/**
 * @brief xcb_glx_get_convolution_filter_request_t
 **/
typedef struct xcb_glx_get_convolution_filter_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
    uint32_t              target;
    uint32_t              format;
    uint32_t              type;
    uint8_t               swap_bytes;
} xcb_glx_get_convolution_filter_request_t;

/**
 * @brief xcb_glx_get_convolution_filter_reply_t
 **/
typedef struct xcb_glx_get_convolution_filter_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint8_t  pad1[8];
    int32_t  width;
    int32_t  height;
    uint8_t  pad2[8];
} xcb_glx_get_convolution_filter_reply_t;

/**
 * @brief xcb_glx_get_convolution_parameterfv_cookie_t
 **/
typedef struct xcb_glx_get_convolution_parameterfv_cookie_t {
    unsigned int sequence;
} xcb_glx_get_convolution_parameterfv_cookie_t;

/** Opcode for xcb_glx_get_convolution_parameterfv. */
#define XCB_GLX_GET_CONVOLUTION_PARAMETERFV 151

/**
 * @brief xcb_glx_get_convolution_parameterfv_request_t
 **/
typedef struct xcb_glx_get_convolution_parameterfv_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
    uint32_t              target;
    uint32_t              pname;
} xcb_glx_get_convolution_parameterfv_request_t;

/**
 * @brief xcb_glx_get_convolution_parameterfv_reply_t
 **/
typedef struct xcb_glx_get_convolution_parameterfv_reply_t {
    uint8_t           response_type;
    uint8_t           pad0;
    uint16_t          sequence;
    uint32_t          length;
    uint8_t           pad1[4];
    uint32_t          n;
    xcb_glx_float32_t datum;
    uint8_t           pad2[12];
} xcb_glx_get_convolution_parameterfv_reply_t;

/**
 * @brief xcb_glx_get_convolution_parameteriv_cookie_t
 **/
typedef struct xcb_glx_get_convolution_parameteriv_cookie_t {
    unsigned int sequence;
} xcb_glx_get_convolution_parameteriv_cookie_t;

/** Opcode for xcb_glx_get_convolution_parameteriv. */
#define XCB_GLX_GET_CONVOLUTION_PARAMETERIV 152

/**
 * @brief xcb_glx_get_convolution_parameteriv_request_t
 **/
typedef struct xcb_glx_get_convolution_parameteriv_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
    uint32_t              target;
    uint32_t              pname;
} xcb_glx_get_convolution_parameteriv_request_t;

/**
 * @brief xcb_glx_get_convolution_parameteriv_reply_t
 **/
typedef struct xcb_glx_get_convolution_parameteriv_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint8_t  pad1[4];
    uint32_t n;
    int32_t  datum;
    uint8_t  pad2[12];
} xcb_glx_get_convolution_parameteriv_reply_t;

/**
 * @brief xcb_glx_get_separable_filter_cookie_t
 **/
typedef struct xcb_glx_get_separable_filter_cookie_t {
    unsigned int sequence;
} xcb_glx_get_separable_filter_cookie_t;

/** Opcode for xcb_glx_get_separable_filter. */
#define XCB_GLX_GET_SEPARABLE_FILTER 153

/**
 * @brief xcb_glx_get_separable_filter_request_t
 **/
typedef struct xcb_glx_get_separable_filter_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
    uint32_t              target;
    uint32_t              format;
    uint32_t              type;
    uint8_t               swap_bytes;
} xcb_glx_get_separable_filter_request_t;

/**
 * @brief xcb_glx_get_separable_filter_reply_t
 **/
typedef struct xcb_glx_get_separable_filter_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint8_t  pad1[8];
    int32_t  row_w;
    int32_t  col_h;
    uint8_t  pad2[8];
} xcb_glx_get_separable_filter_reply_t;

/**
 * @brief xcb_glx_get_histogram_cookie_t
 **/
typedef struct xcb_glx_get_histogram_cookie_t {
    unsigned int sequence;
} xcb_glx_get_histogram_cookie_t;

/** Opcode for xcb_glx_get_histogram. */
#define XCB_GLX_GET_HISTOGRAM 154

/**
 * @brief xcb_glx_get_histogram_request_t
 **/
typedef struct xcb_glx_get_histogram_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
    uint32_t              target;
    uint32_t              format;
    uint32_t              type;
    uint8_t               swap_bytes;
    uint8_t               reset;
} xcb_glx_get_histogram_request_t;

/**
 * @brief xcb_glx_get_histogram_reply_t
 **/
typedef struct xcb_glx_get_histogram_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint8_t  pad1[8];
    int32_t  width;
    uint8_t  pad2[12];
} xcb_glx_get_histogram_reply_t;

/**
 * @brief xcb_glx_get_histogram_parameterfv_cookie_t
 **/
typedef struct xcb_glx_get_histogram_parameterfv_cookie_t {
    unsigned int sequence;
} xcb_glx_get_histogram_parameterfv_cookie_t;

/** Opcode for xcb_glx_get_histogram_parameterfv. */
#define XCB_GLX_GET_HISTOGRAM_PARAMETERFV 155

/**
 * @brief xcb_glx_get_histogram_parameterfv_request_t
 **/
typedef struct xcb_glx_get_histogram_parameterfv_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
    uint32_t              target;
    uint32_t              pname;
} xcb_glx_get_histogram_parameterfv_request_t;

/**
 * @brief xcb_glx_get_histogram_parameterfv_reply_t
 **/
typedef struct xcb_glx_get_histogram_parameterfv_reply_t {
    uint8_t           response_type;
    uint8_t           pad0;
    uint16_t          sequence;
    uint32_t          length;
    uint8_t           pad1[4];
    uint32_t          n;
    xcb_glx_float32_t datum;
    uint8_t           pad2[12];
} xcb_glx_get_histogram_parameterfv_reply_t;

/**
 * @brief xcb_glx_get_histogram_parameteriv_cookie_t
 **/
typedef struct xcb_glx_get_histogram_parameteriv_cookie_t {
    unsigned int sequence;
} xcb_glx_get_histogram_parameteriv_cookie_t;

/** Opcode for xcb_glx_get_histogram_parameteriv. */
#define XCB_GLX_GET_HISTOGRAM_PARAMETERIV 156

/**
 * @brief xcb_glx_get_histogram_parameteriv_request_t
 **/
typedef struct xcb_glx_get_histogram_parameteriv_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
    uint32_t              target;
    uint32_t              pname;
} xcb_glx_get_histogram_parameteriv_request_t;

/**
 * @brief xcb_glx_get_histogram_parameteriv_reply_t
 **/
typedef struct xcb_glx_get_histogram_parameteriv_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint8_t  pad1[4];
    uint32_t n;
    int32_t  datum;
    uint8_t  pad2[12];
} xcb_glx_get_histogram_parameteriv_reply_t;

/**
 * @brief xcb_glx_get_minmax_cookie_t
 **/
typedef struct xcb_glx_get_minmax_cookie_t {
    unsigned int sequence;
} xcb_glx_get_minmax_cookie_t;

/** Opcode for xcb_glx_get_minmax. */
#define XCB_GLX_GET_MINMAX 157

/**
 * @brief xcb_glx_get_minmax_request_t
 **/
typedef struct xcb_glx_get_minmax_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
    uint32_t              target;
    uint32_t              format;
    uint32_t              type;
    uint8_t               swap_bytes;
    uint8_t               reset;
} xcb_glx_get_minmax_request_t;

/**
 * @brief xcb_glx_get_minmax_reply_t
 **/
typedef struct xcb_glx_get_minmax_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint8_t  pad1[24];
} xcb_glx_get_minmax_reply_t;

/**
 * @brief xcb_glx_get_minmax_parameterfv_cookie_t
 **/
typedef struct xcb_glx_get_minmax_parameterfv_cookie_t {
    unsigned int sequence;
} xcb_glx_get_minmax_parameterfv_cookie_t;

/** Opcode for xcb_glx_get_minmax_parameterfv. */
#define XCB_GLX_GET_MINMAX_PARAMETERFV 158

/**
 * @brief xcb_glx_get_minmax_parameterfv_request_t
 **/
typedef struct xcb_glx_get_minmax_parameterfv_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
    uint32_t              target;
    uint32_t              pname;
} xcb_glx_get_minmax_parameterfv_request_t;

/**
 * @brief xcb_glx_get_minmax_parameterfv_reply_t
 **/
typedef struct xcb_glx_get_minmax_parameterfv_reply_t {
    uint8_t           response_type;
    uint8_t           pad0;
    uint16_t          sequence;
    uint32_t          length;
    uint8_t           pad1[4];
    uint32_t          n;
    xcb_glx_float32_t datum;
    uint8_t           pad2[12];
} xcb_glx_get_minmax_parameterfv_reply_t;

/**
 * @brief xcb_glx_get_minmax_parameteriv_cookie_t
 **/
typedef struct xcb_glx_get_minmax_parameteriv_cookie_t {
    unsigned int sequence;
} xcb_glx_get_minmax_parameteriv_cookie_t;

/** Opcode for xcb_glx_get_minmax_parameteriv. */
#define XCB_GLX_GET_MINMAX_PARAMETERIV 159

/**
 * @brief xcb_glx_get_minmax_parameteriv_request_t
 **/
typedef struct xcb_glx_get_minmax_parameteriv_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
    uint32_t              target;
    uint32_t              pname;
} xcb_glx_get_minmax_parameteriv_request_t;

/**
 * @brief xcb_glx_get_minmax_parameteriv_reply_t
 **/
typedef struct xcb_glx_get_minmax_parameteriv_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint8_t  pad1[4];
    uint32_t n;
    int32_t  datum;
    uint8_t  pad2[12];
} xcb_glx_get_minmax_parameteriv_reply_t;

/**
 * @brief xcb_glx_get_compressed_tex_image_arb_cookie_t
 **/
typedef struct xcb_glx_get_compressed_tex_image_arb_cookie_t {
    unsigned int sequence;
} xcb_glx_get_compressed_tex_image_arb_cookie_t;

/** Opcode for xcb_glx_get_compressed_tex_image_arb. */
#define XCB_GLX_GET_COMPRESSED_TEX_IMAGE_ARB 160

/**
 * @brief xcb_glx_get_compressed_tex_image_arb_request_t
 **/
typedef struct xcb_glx_get_compressed_tex_image_arb_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
    uint32_t              target;
    int32_t               level;
} xcb_glx_get_compressed_tex_image_arb_request_t;

/**
 * @brief xcb_glx_get_compressed_tex_image_arb_reply_t
 **/
typedef struct xcb_glx_get_compressed_tex_image_arb_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint8_t  pad1[8];
    int32_t  size;
    uint8_t  pad2[12];
} xcb_glx_get_compressed_tex_image_arb_reply_t;

/** Opcode for xcb_glx_delete_queries_arb. */
#define XCB_GLX_DELETE_QUERIES_ARB 161

/**
 * @brief xcb_glx_delete_queries_arb_request_t
 **/
typedef struct xcb_glx_delete_queries_arb_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
    int32_t               n;
} xcb_glx_delete_queries_arb_request_t;

/**
 * @brief xcb_glx_gen_queries_arb_cookie_t
 **/
typedef struct xcb_glx_gen_queries_arb_cookie_t {
    unsigned int sequence;
} xcb_glx_gen_queries_arb_cookie_t;

/** Opcode for xcb_glx_gen_queries_arb. */
#define XCB_GLX_GEN_QUERIES_ARB 162

/**
 * @brief xcb_glx_gen_queries_arb_request_t
 **/
typedef struct xcb_glx_gen_queries_arb_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
    int32_t               n;
} xcb_glx_gen_queries_arb_request_t;

/**
 * @brief xcb_glx_gen_queries_arb_reply_t
 **/
typedef struct xcb_glx_gen_queries_arb_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint8_t  pad1[24];
} xcb_glx_gen_queries_arb_reply_t;

/**
 * @brief xcb_glx_is_query_arb_cookie_t
 **/
typedef struct xcb_glx_is_query_arb_cookie_t {
    unsigned int sequence;
} xcb_glx_is_query_arb_cookie_t;

/** Opcode for xcb_glx_is_query_arb. */
#define XCB_GLX_IS_QUERY_ARB 163

/**
 * @brief xcb_glx_is_query_arb_request_t
 **/
typedef struct xcb_glx_is_query_arb_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
    uint32_t              id;
} xcb_glx_is_query_arb_request_t;

/**
 * @brief xcb_glx_is_query_arb_reply_t
 **/
typedef struct xcb_glx_is_query_arb_reply_t {
    uint8_t          response_type;
    uint8_t          pad0;
    uint16_t         sequence;
    uint32_t         length;
    xcb_glx_bool32_t ret_val;
} xcb_glx_is_query_arb_reply_t;

/**
 * @brief xcb_glx_get_queryiv_arb_cookie_t
 **/
typedef struct xcb_glx_get_queryiv_arb_cookie_t {
    unsigned int sequence;
} xcb_glx_get_queryiv_arb_cookie_t;

/** Opcode for xcb_glx_get_queryiv_arb. */
#define XCB_GLX_GET_QUERYIV_ARB 164

/**
 * @brief xcb_glx_get_queryiv_arb_request_t
 **/
typedef struct xcb_glx_get_queryiv_arb_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
    uint32_t              target;
    uint32_t              pname;
} xcb_glx_get_queryiv_arb_request_t;

/**
 * @brief xcb_glx_get_queryiv_arb_reply_t
 **/
typedef struct xcb_glx_get_queryiv_arb_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint8_t  pad1[4];
    uint32_t n;
    int32_t  datum;
    uint8_t  pad2[12];
} xcb_glx_get_queryiv_arb_reply_t;

/**
 * @brief xcb_glx_get_query_objectiv_arb_cookie_t
 **/
typedef struct xcb_glx_get_query_objectiv_arb_cookie_t {
    unsigned int sequence;
} xcb_glx_get_query_objectiv_arb_cookie_t;

/** Opcode for xcb_glx_get_query_objectiv_arb. */
#define XCB_GLX_GET_QUERY_OBJECTIV_ARB 165

/**
 * @brief xcb_glx_get_query_objectiv_arb_request_t
 **/
typedef struct xcb_glx_get_query_objectiv_arb_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
    uint32_t              id;
    uint32_t              pname;
} xcb_glx_get_query_objectiv_arb_request_t;

/**
 * @brief xcb_glx_get_query_objectiv_arb_reply_t
 **/
typedef struct xcb_glx_get_query_objectiv_arb_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint8_t  pad1[4];
    uint32_t n;
    int32_t  datum;
    uint8_t  pad2[12];
} xcb_glx_get_query_objectiv_arb_reply_t;

/**
 * @brief xcb_glx_get_query_objectuiv_arb_cookie_t
 **/
typedef struct xcb_glx_get_query_objectuiv_arb_cookie_t {
    unsigned int sequence;
} xcb_glx_get_query_objectuiv_arb_cookie_t;

/** Opcode for xcb_glx_get_query_objectuiv_arb. */
#define XCB_GLX_GET_QUERY_OBJECTUIV_ARB 166

/**
 * @brief xcb_glx_get_query_objectuiv_arb_request_t
 **/
typedef struct xcb_glx_get_query_objectuiv_arb_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_glx_context_tag_t context_tag;
    uint32_t              id;
    uint32_t              pname;
} xcb_glx_get_query_objectuiv_arb_request_t;

/**
 * @brief xcb_glx_get_query_objectuiv_arb_reply_t
 **/
typedef struct xcb_glx_get_query_objectuiv_arb_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint8_t  pad1[4];
    uint32_t n;
    uint32_t datum;
    uint8_t  pad2[12];
} xcb_glx_get_query_objectuiv_arb_reply_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_glx_pixmap_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_glx_pixmap_t)
 */
void
xcb_glx_pixmap_next (xcb_glx_pixmap_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_glx_pixmap_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_glx_pixmap_end (xcb_glx_pixmap_iterator_t i);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_glx_context_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_glx_context_t)
 */
void
xcb_glx_context_next (xcb_glx_context_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_glx_context_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_glx_context_end (xcb_glx_context_iterator_t i);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_glx_pbuffer_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_glx_pbuffer_t)
 */
void
xcb_glx_pbuffer_next (xcb_glx_pbuffer_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_glx_pbuffer_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_glx_pbuffer_end (xcb_glx_pbuffer_iterator_t i);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_glx_window_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_glx_window_t)
 */
void
xcb_glx_window_next (xcb_glx_window_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_glx_window_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_glx_window_end (xcb_glx_window_iterator_t i);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_glx_fbconfig_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_glx_fbconfig_t)
 */
void
xcb_glx_fbconfig_next (xcb_glx_fbconfig_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_glx_fbconfig_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_glx_fbconfig_end (xcb_glx_fbconfig_iterator_t i);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_glx_drawable_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_glx_drawable_t)
 */
void
xcb_glx_drawable_next (xcb_glx_drawable_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_glx_drawable_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_glx_drawable_end (xcb_glx_drawable_iterator_t i);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_glx_float32_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_glx_float32_t)
 */
void
xcb_glx_float32_next (xcb_glx_float32_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_glx_float32_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_glx_float32_end (xcb_glx_float32_iterator_t i);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_glx_float64_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_glx_float64_t)
 */
void
xcb_glx_float64_next (xcb_glx_float64_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_glx_float64_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_glx_float64_end (xcb_glx_float64_iterator_t i);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_glx_bool32_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_glx_bool32_t)
 */
void
xcb_glx_bool32_next (xcb_glx_bool32_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_glx_bool32_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_glx_bool32_end (xcb_glx_bool32_iterator_t i);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_glx_context_tag_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_glx_context_tag_t)
 */
void
xcb_glx_context_tag_next (xcb_glx_context_tag_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_glx_context_tag_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_glx_context_tag_end (xcb_glx_context_tag_iterator_t i);

int
xcb_glx_render_sizeof (const void  *_buffer,
                       uint32_t     data_len);

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
xcb_glx_render_checked (xcb_connection_t      *c,
                        xcb_glx_context_tag_t  context_tag,
                        uint32_t               data_len,
                        const uint8_t         *data);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_glx_render (xcb_connection_t      *c,
                xcb_glx_context_tag_t  context_tag,
                uint32_t               data_len,
                const uint8_t         *data);

uint8_t *
xcb_glx_render_data (const xcb_glx_render_request_t *R);

int
xcb_glx_render_data_length (const xcb_glx_render_request_t *R);

xcb_generic_iterator_t
xcb_glx_render_data_end (const xcb_glx_render_request_t *R);

int
xcb_glx_render_large_sizeof (const void  *_buffer);

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
xcb_glx_render_large_checked (xcb_connection_t      *c,
                              xcb_glx_context_tag_t  context_tag,
                              uint16_t               request_num,
                              uint16_t               request_total,
                              uint32_t               data_len,
                              const uint8_t         *data);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_glx_render_large (xcb_connection_t      *c,
                      xcb_glx_context_tag_t  context_tag,
                      uint16_t               request_num,
                      uint16_t               request_total,
                      uint32_t               data_len,
                      const uint8_t         *data);

uint8_t *
xcb_glx_render_large_data (const xcb_glx_render_large_request_t *R);

int
xcb_glx_render_large_data_length (const xcb_glx_render_large_request_t *R);

xcb_generic_iterator_t
xcb_glx_render_large_data_end (const xcb_glx_render_large_request_t *R);

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
xcb_glx_create_context_checked (xcb_connection_t  *c,
                                xcb_glx_context_t  context,
                                xcb_visualid_t     visual,
                                uint32_t           screen,
                                xcb_glx_context_t  share_list,
                                uint8_t            is_direct);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_glx_create_context (xcb_connection_t  *c,
                        xcb_glx_context_t  context,
                        xcb_visualid_t     visual,
                        uint32_t           screen,
                        xcb_glx_context_t  share_list,
                        uint8_t            is_direct);

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
xcb_glx_destroy_context_checked (xcb_connection_t  *c,
                                 xcb_glx_context_t  context);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_glx_destroy_context (xcb_connection_t  *c,
                         xcb_glx_context_t  context);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_glx_make_current_cookie_t
xcb_glx_make_current (xcb_connection_t      *c,
                      xcb_glx_drawable_t     drawable,
                      xcb_glx_context_t      context,
                      xcb_glx_context_tag_t  old_context_tag);

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
xcb_glx_make_current_cookie_t
xcb_glx_make_current_unchecked (xcb_connection_t      *c,
                                xcb_glx_drawable_t     drawable,
                                xcb_glx_context_t      context,
                                xcb_glx_context_tag_t  old_context_tag);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_make_current_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_glx_make_current_reply_t *
xcb_glx_make_current_reply (xcb_connection_t               *c,
                            xcb_glx_make_current_cookie_t   cookie  /**< */,
                            xcb_generic_error_t           **e);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_glx_is_direct_cookie_t
xcb_glx_is_direct (xcb_connection_t  *c,
                   xcb_glx_context_t  context);

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
xcb_glx_is_direct_cookie_t
xcb_glx_is_direct_unchecked (xcb_connection_t  *c,
                             xcb_glx_context_t  context);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_is_direct_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_glx_is_direct_reply_t *
xcb_glx_is_direct_reply (xcb_connection_t            *c,
                         xcb_glx_is_direct_cookie_t   cookie  /**< */,
                         xcb_generic_error_t        **e);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_glx_query_version_cookie_t
xcb_glx_query_version (xcb_connection_t *c,
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
xcb_glx_query_version_cookie_t
xcb_glx_query_version_unchecked (xcb_connection_t *c,
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
 * xcb_glx_query_version_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_glx_query_version_reply_t *
xcb_glx_query_version_reply (xcb_connection_t                *c,
                             xcb_glx_query_version_cookie_t   cookie  /**< */,
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
xcb_glx_wait_gl_checked (xcb_connection_t      *c,
                         xcb_glx_context_tag_t  context_tag);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_glx_wait_gl (xcb_connection_t      *c,
                 xcb_glx_context_tag_t  context_tag);

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
xcb_glx_wait_x_checked (xcb_connection_t      *c,
                        xcb_glx_context_tag_t  context_tag);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_glx_wait_x (xcb_connection_t      *c,
                xcb_glx_context_tag_t  context_tag);

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
xcb_glx_copy_context_checked (xcb_connection_t      *c,
                              xcb_glx_context_t      src,
                              xcb_glx_context_t      dest,
                              uint32_t               mask,
                              xcb_glx_context_tag_t  src_context_tag);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_glx_copy_context (xcb_connection_t      *c,
                      xcb_glx_context_t      src,
                      xcb_glx_context_t      dest,
                      uint32_t               mask,
                      xcb_glx_context_tag_t  src_context_tag);

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
xcb_glx_swap_buffers_checked (xcb_connection_t      *c,
                              xcb_glx_context_tag_t  context_tag,
                              xcb_glx_drawable_t     drawable);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_glx_swap_buffers (xcb_connection_t      *c,
                      xcb_glx_context_tag_t  context_tag,
                      xcb_glx_drawable_t     drawable);

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
xcb_glx_use_x_font_checked (xcb_connection_t      *c,
                            xcb_glx_context_tag_t  context_tag,
                            xcb_font_t             font,
                            uint32_t               first,
                            uint32_t               count,
                            uint32_t               list_base);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_glx_use_x_font (xcb_connection_t      *c,
                    xcb_glx_context_tag_t  context_tag,
                    xcb_font_t             font,
                    uint32_t               first,
                    uint32_t               count,
                    uint32_t               list_base);

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
xcb_glx_create_glx_pixmap_checked (xcb_connection_t *c,
                                   uint32_t          screen,
                                   xcb_visualid_t    visual,
                                   xcb_pixmap_t      pixmap,
                                   xcb_glx_pixmap_t  glx_pixmap);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_glx_create_glx_pixmap (xcb_connection_t *c,
                           uint32_t          screen,
                           xcb_visualid_t    visual,
                           xcb_pixmap_t      pixmap,
                           xcb_glx_pixmap_t  glx_pixmap);

int
xcb_glx_get_visual_configs_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_glx_get_visual_configs_cookie_t
xcb_glx_get_visual_configs (xcb_connection_t *c,
                            uint32_t          screen);

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
xcb_glx_get_visual_configs_cookie_t
xcb_glx_get_visual_configs_unchecked (xcb_connection_t *c,
                                      uint32_t          screen);

uint32_t *
xcb_glx_get_visual_configs_property_list (const xcb_glx_get_visual_configs_reply_t *R);

int
xcb_glx_get_visual_configs_property_list_length (const xcb_glx_get_visual_configs_reply_t *R);

xcb_generic_iterator_t
xcb_glx_get_visual_configs_property_list_end (const xcb_glx_get_visual_configs_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_visual_configs_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_glx_get_visual_configs_reply_t *
xcb_glx_get_visual_configs_reply (xcb_connection_t                     *c,
                                  xcb_glx_get_visual_configs_cookie_t   cookie  /**< */,
                                  xcb_generic_error_t                 **e);

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
xcb_glx_destroy_glx_pixmap_checked (xcb_connection_t *c,
                                    xcb_glx_pixmap_t  glx_pixmap);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_glx_destroy_glx_pixmap (xcb_connection_t *c,
                            xcb_glx_pixmap_t  glx_pixmap);

int
xcb_glx_vendor_private_sizeof (const void  *_buffer,
                               uint32_t     data_len);

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
xcb_glx_vendor_private_checked (xcb_connection_t      *c,
                                uint32_t               vendor_code,
                                xcb_glx_context_tag_t  context_tag,
                                uint32_t               data_len,
                                const uint8_t         *data);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_glx_vendor_private (xcb_connection_t      *c,
                        uint32_t               vendor_code,
                        xcb_glx_context_tag_t  context_tag,
                        uint32_t               data_len,
                        const uint8_t         *data);

uint8_t *
xcb_glx_vendor_private_data (const xcb_glx_vendor_private_request_t *R);

int
xcb_glx_vendor_private_data_length (const xcb_glx_vendor_private_request_t *R);

xcb_generic_iterator_t
xcb_glx_vendor_private_data_end (const xcb_glx_vendor_private_request_t *R);

int
xcb_glx_vendor_private_with_reply_sizeof (const void  *_buffer,
                                          uint32_t     data_len);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_glx_vendor_private_with_reply_cookie_t
xcb_glx_vendor_private_with_reply (xcb_connection_t      *c,
                                   uint32_t               vendor_code,
                                   xcb_glx_context_tag_t  context_tag,
                                   uint32_t               data_len,
                                   const uint8_t         *data);

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
xcb_glx_vendor_private_with_reply_cookie_t
xcb_glx_vendor_private_with_reply_unchecked (xcb_connection_t      *c,
                                             uint32_t               vendor_code,
                                             xcb_glx_context_tag_t  context_tag,
                                             uint32_t               data_len,
                                             const uint8_t         *data);

uint8_t *
xcb_glx_vendor_private_with_reply_data_2 (const xcb_glx_vendor_private_with_reply_reply_t *R);

int
xcb_glx_vendor_private_with_reply_data_2_length (const xcb_glx_vendor_private_with_reply_reply_t *R);

xcb_generic_iterator_t
xcb_glx_vendor_private_with_reply_data_2_end (const xcb_glx_vendor_private_with_reply_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_vendor_private_with_reply_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_glx_vendor_private_with_reply_reply_t *
xcb_glx_vendor_private_with_reply_reply (xcb_connection_t                            *c,
                                         xcb_glx_vendor_private_with_reply_cookie_t   cookie  /**< */,
                                         xcb_generic_error_t                        **e);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_glx_query_extensions_string_cookie_t
xcb_glx_query_extensions_string (xcb_connection_t *c,
                                 uint32_t          screen);

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
xcb_glx_query_extensions_string_cookie_t
xcb_glx_query_extensions_string_unchecked (xcb_connection_t *c,
                                           uint32_t          screen);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_query_extensions_string_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_glx_query_extensions_string_reply_t *
xcb_glx_query_extensions_string_reply (xcb_connection_t                          *c,
                                       xcb_glx_query_extensions_string_cookie_t   cookie  /**< */,
                                       xcb_generic_error_t                      **e);

int
xcb_glx_query_server_string_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_glx_query_server_string_cookie_t
xcb_glx_query_server_string (xcb_connection_t *c,
                             uint32_t          screen,
                             uint32_t          name);

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
xcb_glx_query_server_string_cookie_t
xcb_glx_query_server_string_unchecked (xcb_connection_t *c,
                                       uint32_t          screen,
                                       uint32_t          name);

char *
xcb_glx_query_server_string_string (const xcb_glx_query_server_string_reply_t *R);

int
xcb_glx_query_server_string_string_length (const xcb_glx_query_server_string_reply_t *R);

xcb_generic_iterator_t
xcb_glx_query_server_string_string_end (const xcb_glx_query_server_string_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_query_server_string_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_glx_query_server_string_reply_t *
xcb_glx_query_server_string_reply (xcb_connection_t                      *c,
                                   xcb_glx_query_server_string_cookie_t   cookie  /**< */,
                                   xcb_generic_error_t                  **e);

int
xcb_glx_client_info_sizeof (const void  *_buffer);

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
xcb_glx_client_info_checked (xcb_connection_t *c,
                             uint32_t          major_version,
                             uint32_t          minor_version,
                             uint32_t          str_len,
                             const char       *string);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_glx_client_info (xcb_connection_t *c,
                     uint32_t          major_version,
                     uint32_t          minor_version,
                     uint32_t          str_len,
                     const char       *string);

char *
xcb_glx_client_info_string (const xcb_glx_client_info_request_t *R);

int
xcb_glx_client_info_string_length (const xcb_glx_client_info_request_t *R);

xcb_generic_iterator_t
xcb_glx_client_info_string_end (const xcb_glx_client_info_request_t *R);

int
xcb_glx_get_fb_configs_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_glx_get_fb_configs_cookie_t
xcb_glx_get_fb_configs (xcb_connection_t *c,
                        uint32_t          screen);

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
xcb_glx_get_fb_configs_cookie_t
xcb_glx_get_fb_configs_unchecked (xcb_connection_t *c,
                                  uint32_t          screen);

uint32_t *
xcb_glx_get_fb_configs_property_list (const xcb_glx_get_fb_configs_reply_t *R);

int
xcb_glx_get_fb_configs_property_list_length (const xcb_glx_get_fb_configs_reply_t *R);

xcb_generic_iterator_t
xcb_glx_get_fb_configs_property_list_end (const xcb_glx_get_fb_configs_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_fb_configs_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_glx_get_fb_configs_reply_t *
xcb_glx_get_fb_configs_reply (xcb_connection_t                 *c,
                              xcb_glx_get_fb_configs_cookie_t   cookie  /**< */,
                              xcb_generic_error_t             **e);

int
xcb_glx_create_pixmap_sizeof (const void  *_buffer);

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
xcb_glx_create_pixmap_checked (xcb_connection_t   *c,
                               uint32_t            screen,
                               xcb_glx_fbconfig_t  fbconfig,
                               xcb_pixmap_t        pixmap,
                               xcb_glx_pixmap_t    glx_pixmap,
                               uint32_t            num_attribs,
                               const uint32_t     *attribs);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_glx_create_pixmap (xcb_connection_t   *c,
                       uint32_t            screen,
                       xcb_glx_fbconfig_t  fbconfig,
                       xcb_pixmap_t        pixmap,
                       xcb_glx_pixmap_t    glx_pixmap,
                       uint32_t            num_attribs,
                       const uint32_t     *attribs);

uint32_t *
xcb_glx_create_pixmap_attribs (const xcb_glx_create_pixmap_request_t *R);

int
xcb_glx_create_pixmap_attribs_length (const xcb_glx_create_pixmap_request_t *R);

xcb_generic_iterator_t
xcb_glx_create_pixmap_attribs_end (const xcb_glx_create_pixmap_request_t *R);

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
xcb_glx_destroy_pixmap_checked (xcb_connection_t *c,
                                xcb_glx_pixmap_t  glx_pixmap);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_glx_destroy_pixmap (xcb_connection_t *c,
                        xcb_glx_pixmap_t  glx_pixmap);

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
xcb_glx_create_new_context_checked (xcb_connection_t   *c,
                                    xcb_glx_context_t   context,
                                    xcb_glx_fbconfig_t  fbconfig,
                                    uint32_t            screen,
                                    uint32_t            render_type,
                                    xcb_glx_context_t   share_list,
                                    uint8_t             is_direct);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_glx_create_new_context (xcb_connection_t   *c,
                            xcb_glx_context_t   context,
                            xcb_glx_fbconfig_t  fbconfig,
                            uint32_t            screen,
                            uint32_t            render_type,
                            xcb_glx_context_t   share_list,
                            uint8_t             is_direct);

int
xcb_glx_query_context_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_glx_query_context_cookie_t
xcb_glx_query_context (xcb_connection_t  *c,
                       xcb_glx_context_t  context);

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
xcb_glx_query_context_cookie_t
xcb_glx_query_context_unchecked (xcb_connection_t  *c,
                                 xcb_glx_context_t  context);

uint32_t *
xcb_glx_query_context_attribs (const xcb_glx_query_context_reply_t *R);

int
xcb_glx_query_context_attribs_length (const xcb_glx_query_context_reply_t *R);

xcb_generic_iterator_t
xcb_glx_query_context_attribs_end (const xcb_glx_query_context_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_query_context_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_glx_query_context_reply_t *
xcb_glx_query_context_reply (xcb_connection_t                *c,
                             xcb_glx_query_context_cookie_t   cookie  /**< */,
                             xcb_generic_error_t            **e);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_glx_make_context_current_cookie_t
xcb_glx_make_context_current (xcb_connection_t      *c,
                              xcb_glx_context_tag_t  old_context_tag,
                              xcb_glx_drawable_t     drawable,
                              xcb_glx_drawable_t     read_drawable,
                              xcb_glx_context_t      context);

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
xcb_glx_make_context_current_cookie_t
xcb_glx_make_context_current_unchecked (xcb_connection_t      *c,
                                        xcb_glx_context_tag_t  old_context_tag,
                                        xcb_glx_drawable_t     drawable,
                                        xcb_glx_drawable_t     read_drawable,
                                        xcb_glx_context_t      context);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_make_context_current_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_glx_make_context_current_reply_t *
xcb_glx_make_context_current_reply (xcb_connection_t                       *c,
                                    xcb_glx_make_context_current_cookie_t   cookie  /**< */,
                                    xcb_generic_error_t                   **e);

int
xcb_glx_create_pbuffer_sizeof (const void  *_buffer);

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
xcb_glx_create_pbuffer_checked (xcb_connection_t   *c,
                                uint32_t            screen,
                                xcb_glx_fbconfig_t  fbconfig,
                                xcb_glx_pbuffer_t   pbuffer,
                                uint32_t            num_attribs,
                                const uint32_t     *attribs);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_glx_create_pbuffer (xcb_connection_t   *c,
                        uint32_t            screen,
                        xcb_glx_fbconfig_t  fbconfig,
                        xcb_glx_pbuffer_t   pbuffer,
                        uint32_t            num_attribs,
                        const uint32_t     *attribs);

uint32_t *
xcb_glx_create_pbuffer_attribs (const xcb_glx_create_pbuffer_request_t *R);

int
xcb_glx_create_pbuffer_attribs_length (const xcb_glx_create_pbuffer_request_t *R);

xcb_generic_iterator_t
xcb_glx_create_pbuffer_attribs_end (const xcb_glx_create_pbuffer_request_t *R);

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
xcb_glx_destroy_pbuffer_checked (xcb_connection_t  *c,
                                 xcb_glx_pbuffer_t  pbuffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_glx_destroy_pbuffer (xcb_connection_t  *c,
                         xcb_glx_pbuffer_t  pbuffer);

int
xcb_glx_get_drawable_attributes_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_glx_get_drawable_attributes_cookie_t
xcb_glx_get_drawable_attributes (xcb_connection_t   *c,
                                 xcb_glx_drawable_t  drawable);

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
xcb_glx_get_drawable_attributes_cookie_t
xcb_glx_get_drawable_attributes_unchecked (xcb_connection_t   *c,
                                           xcb_glx_drawable_t  drawable);

uint32_t *
xcb_glx_get_drawable_attributes_attribs (const xcb_glx_get_drawable_attributes_reply_t *R);

int
xcb_glx_get_drawable_attributes_attribs_length (const xcb_glx_get_drawable_attributes_reply_t *R);

xcb_generic_iterator_t
xcb_glx_get_drawable_attributes_attribs_end (const xcb_glx_get_drawable_attributes_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_drawable_attributes_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_glx_get_drawable_attributes_reply_t *
xcb_glx_get_drawable_attributes_reply (xcb_connection_t                          *c,
                                       xcb_glx_get_drawable_attributes_cookie_t   cookie  /**< */,
                                       xcb_generic_error_t                      **e);

int
xcb_glx_change_drawable_attributes_sizeof (const void  *_buffer);

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
xcb_glx_change_drawable_attributes_checked (xcb_connection_t   *c,
                                            xcb_glx_drawable_t  drawable,
                                            uint32_t            num_attribs,
                                            const uint32_t     *attribs);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_glx_change_drawable_attributes (xcb_connection_t   *c,
                                    xcb_glx_drawable_t  drawable,
                                    uint32_t            num_attribs,
                                    const uint32_t     *attribs);

uint32_t *
xcb_glx_change_drawable_attributes_attribs (const xcb_glx_change_drawable_attributes_request_t *R);

int
xcb_glx_change_drawable_attributes_attribs_length (const xcb_glx_change_drawable_attributes_request_t *R);

xcb_generic_iterator_t
xcb_glx_change_drawable_attributes_attribs_end (const xcb_glx_change_drawable_attributes_request_t *R);

int
xcb_glx_create_window_sizeof (const void  *_buffer);

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
xcb_glx_create_window_checked (xcb_connection_t   *c,
                               uint32_t            screen,
                               xcb_glx_fbconfig_t  fbconfig,
                               xcb_window_t        window,
                               xcb_glx_window_t    glx_window,
                               uint32_t            num_attribs,
                               const uint32_t     *attribs);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_glx_create_window (xcb_connection_t   *c,
                       uint32_t            screen,
                       xcb_glx_fbconfig_t  fbconfig,
                       xcb_window_t        window,
                       xcb_glx_window_t    glx_window,
                       uint32_t            num_attribs,
                       const uint32_t     *attribs);

uint32_t *
xcb_glx_create_window_attribs (const xcb_glx_create_window_request_t *R);

int
xcb_glx_create_window_attribs_length (const xcb_glx_create_window_request_t *R);

xcb_generic_iterator_t
xcb_glx_create_window_attribs_end (const xcb_glx_create_window_request_t *R);

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
xcb_glx_delete_window_checked (xcb_connection_t *c,
                               xcb_glx_window_t  glxwindow);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_glx_delete_window (xcb_connection_t *c,
                       xcb_glx_window_t  glxwindow);

int
xcb_glx_set_client_info_arb_sizeof (const void  *_buffer);

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
xcb_glx_set_client_info_arb_checked (xcb_connection_t *c,
                                     uint32_t          major_version,
                                     uint32_t          minor_version,
                                     uint32_t          num_versions,
                                     uint32_t          gl_str_len,
                                     uint32_t          glx_str_len,
                                     const uint32_t   *gl_versions,
                                     const char       *gl_extension_string,
                                     const char       *glx_extension_string);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_glx_set_client_info_arb (xcb_connection_t *c,
                             uint32_t          major_version,
                             uint32_t          minor_version,
                             uint32_t          num_versions,
                             uint32_t          gl_str_len,
                             uint32_t          glx_str_len,
                             const uint32_t   *gl_versions,
                             const char       *gl_extension_string,
                             const char       *glx_extension_string);

uint32_t *
xcb_glx_set_client_info_arb_gl_versions (const xcb_glx_set_client_info_arb_request_t *R);

int
xcb_glx_set_client_info_arb_gl_versions_length (const xcb_glx_set_client_info_arb_request_t *R);

xcb_generic_iterator_t
xcb_glx_set_client_info_arb_gl_versions_end (const xcb_glx_set_client_info_arb_request_t *R);

char *
xcb_glx_set_client_info_arb_gl_extension_string (const xcb_glx_set_client_info_arb_request_t *R);

int
xcb_glx_set_client_info_arb_gl_extension_string_length (const xcb_glx_set_client_info_arb_request_t *R);

xcb_generic_iterator_t
xcb_glx_set_client_info_arb_gl_extension_string_end (const xcb_glx_set_client_info_arb_request_t *R);

char *
xcb_glx_set_client_info_arb_glx_extension_string (const xcb_glx_set_client_info_arb_request_t *R);

int
xcb_glx_set_client_info_arb_glx_extension_string_length (const xcb_glx_set_client_info_arb_request_t *R);

xcb_generic_iterator_t
xcb_glx_set_client_info_arb_glx_extension_string_end (const xcb_glx_set_client_info_arb_request_t *R);

int
xcb_glx_create_context_attribs_arb_sizeof (const void  *_buffer);

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
xcb_glx_create_context_attribs_arb_checked (xcb_connection_t   *c,
                                            xcb_glx_context_t   context,
                                            xcb_glx_fbconfig_t  fbconfig,
                                            uint32_t            screen,
                                            xcb_glx_context_t   share_list,
                                            uint8_t             is_direct,
                                            uint32_t            num_attribs,
                                            const uint32_t     *attribs);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_glx_create_context_attribs_arb (xcb_connection_t   *c,
                                    xcb_glx_context_t   context,
                                    xcb_glx_fbconfig_t  fbconfig,
                                    uint32_t            screen,
                                    xcb_glx_context_t   share_list,
                                    uint8_t             is_direct,
                                    uint32_t            num_attribs,
                                    const uint32_t     *attribs);

uint32_t *
xcb_glx_create_context_attribs_arb_attribs (const xcb_glx_create_context_attribs_arb_request_t *R);

int
xcb_glx_create_context_attribs_arb_attribs_length (const xcb_glx_create_context_attribs_arb_request_t *R);

xcb_generic_iterator_t
xcb_glx_create_context_attribs_arb_attribs_end (const xcb_glx_create_context_attribs_arb_request_t *R);

int
xcb_glx_set_client_info_2arb_sizeof (const void  *_buffer);

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
xcb_glx_set_client_info_2arb_checked (xcb_connection_t *c,
                                      uint32_t          major_version,
                                      uint32_t          minor_version,
                                      uint32_t          num_versions,
                                      uint32_t          gl_str_len,
                                      uint32_t          glx_str_len,
                                      const uint32_t   *gl_versions,
                                      const char       *gl_extension_string,
                                      const char       *glx_extension_string);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_glx_set_client_info_2arb (xcb_connection_t *c,
                              uint32_t          major_version,
                              uint32_t          minor_version,
                              uint32_t          num_versions,
                              uint32_t          gl_str_len,
                              uint32_t          glx_str_len,
                              const uint32_t   *gl_versions,
                              const char       *gl_extension_string,
                              const char       *glx_extension_string);

uint32_t *
xcb_glx_set_client_info_2arb_gl_versions (const xcb_glx_set_client_info_2arb_request_t *R);

int
xcb_glx_set_client_info_2arb_gl_versions_length (const xcb_glx_set_client_info_2arb_request_t *R);

xcb_generic_iterator_t
xcb_glx_set_client_info_2arb_gl_versions_end (const xcb_glx_set_client_info_2arb_request_t *R);

char *
xcb_glx_set_client_info_2arb_gl_extension_string (const xcb_glx_set_client_info_2arb_request_t *R);

int
xcb_glx_set_client_info_2arb_gl_extension_string_length (const xcb_glx_set_client_info_2arb_request_t *R);

xcb_generic_iterator_t
xcb_glx_set_client_info_2arb_gl_extension_string_end (const xcb_glx_set_client_info_2arb_request_t *R);

char *
xcb_glx_set_client_info_2arb_glx_extension_string (const xcb_glx_set_client_info_2arb_request_t *R);

int
xcb_glx_set_client_info_2arb_glx_extension_string_length (const xcb_glx_set_client_info_2arb_request_t *R);

xcb_generic_iterator_t
xcb_glx_set_client_info_2arb_glx_extension_string_end (const xcb_glx_set_client_info_2arb_request_t *R);

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
xcb_glx_new_list_checked (xcb_connection_t      *c,
                          xcb_glx_context_tag_t  context_tag,
                          uint32_t               list,
                          uint32_t               mode);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_glx_new_list (xcb_connection_t      *c,
                  xcb_glx_context_tag_t  context_tag,
                  uint32_t               list,
                  uint32_t               mode);

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
xcb_glx_end_list_checked (xcb_connection_t      *c,
                          xcb_glx_context_tag_t  context_tag);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_glx_end_list (xcb_connection_t      *c,
                  xcb_glx_context_tag_t  context_tag);

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
xcb_glx_delete_lists_checked (xcb_connection_t      *c,
                              xcb_glx_context_tag_t  context_tag,
                              uint32_t               list,
                              int32_t                range);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_glx_delete_lists (xcb_connection_t      *c,
                      xcb_glx_context_tag_t  context_tag,
                      uint32_t               list,
                      int32_t                range);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_glx_gen_lists_cookie_t
xcb_glx_gen_lists (xcb_connection_t      *c,
                   xcb_glx_context_tag_t  context_tag,
                   int32_t                range);

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
xcb_glx_gen_lists_cookie_t
xcb_glx_gen_lists_unchecked (xcb_connection_t      *c,
                             xcb_glx_context_tag_t  context_tag,
                             int32_t                range);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_gen_lists_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_glx_gen_lists_reply_t *
xcb_glx_gen_lists_reply (xcb_connection_t            *c,
                         xcb_glx_gen_lists_cookie_t   cookie  /**< */,
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
xcb_glx_feedback_buffer_checked (xcb_connection_t      *c,
                                 xcb_glx_context_tag_t  context_tag,
                                 int32_t                size,
                                 int32_t                type);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_glx_feedback_buffer (xcb_connection_t      *c,
                         xcb_glx_context_tag_t  context_tag,
                         int32_t                size,
                         int32_t                type);

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
xcb_glx_select_buffer_checked (xcb_connection_t      *c,
                               xcb_glx_context_tag_t  context_tag,
                               int32_t                size);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_glx_select_buffer (xcb_connection_t      *c,
                       xcb_glx_context_tag_t  context_tag,
                       int32_t                size);

int
xcb_glx_render_mode_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_glx_render_mode_cookie_t
xcb_glx_render_mode (xcb_connection_t      *c,
                     xcb_glx_context_tag_t  context_tag,
                     uint32_t               mode);

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
xcb_glx_render_mode_cookie_t
xcb_glx_render_mode_unchecked (xcb_connection_t      *c,
                               xcb_glx_context_tag_t  context_tag,
                               uint32_t               mode);

uint32_t *
xcb_glx_render_mode_data (const xcb_glx_render_mode_reply_t *R);

int
xcb_glx_render_mode_data_length (const xcb_glx_render_mode_reply_t *R);

xcb_generic_iterator_t
xcb_glx_render_mode_data_end (const xcb_glx_render_mode_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_render_mode_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_glx_render_mode_reply_t *
xcb_glx_render_mode_reply (xcb_connection_t              *c,
                           xcb_glx_render_mode_cookie_t   cookie  /**< */,
                           xcb_generic_error_t          **e);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_glx_finish_cookie_t
xcb_glx_finish (xcb_connection_t      *c,
                xcb_glx_context_tag_t  context_tag);

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
xcb_glx_finish_cookie_t
xcb_glx_finish_unchecked (xcb_connection_t      *c,
                          xcb_glx_context_tag_t  context_tag);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_finish_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_glx_finish_reply_t *
xcb_glx_finish_reply (xcb_connection_t         *c,
                      xcb_glx_finish_cookie_t   cookie  /**< */,
                      xcb_generic_error_t     **e);

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
xcb_glx_pixel_storef_checked (xcb_connection_t      *c,
                              xcb_glx_context_tag_t  context_tag,
                              uint32_t               pname,
                              xcb_glx_float32_t      datum);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_glx_pixel_storef (xcb_connection_t      *c,
                      xcb_glx_context_tag_t  context_tag,
                      uint32_t               pname,
                      xcb_glx_float32_t      datum);

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
xcb_glx_pixel_storei_checked (xcb_connection_t      *c,
                              xcb_glx_context_tag_t  context_tag,
                              uint32_t               pname,
                              int32_t                datum);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_glx_pixel_storei (xcb_connection_t      *c,
                      xcb_glx_context_tag_t  context_tag,
                      uint32_t               pname,
                      int32_t                datum);

int
xcb_glx_read_pixels_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_glx_read_pixels_cookie_t
xcb_glx_read_pixels (xcb_connection_t      *c,
                     xcb_glx_context_tag_t  context_tag,
                     int32_t                x,
                     int32_t                y,
                     int32_t                width,
                     int32_t                height,
                     uint32_t               format,
                     uint32_t               type,
                     uint8_t                swap_bytes,
                     uint8_t                lsb_first);

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
xcb_glx_read_pixels_cookie_t
xcb_glx_read_pixels_unchecked (xcb_connection_t      *c,
                               xcb_glx_context_tag_t  context_tag,
                               int32_t                x,
                               int32_t                y,
                               int32_t                width,
                               int32_t                height,
                               uint32_t               format,
                               uint32_t               type,
                               uint8_t                swap_bytes,
                               uint8_t                lsb_first);

uint8_t *
xcb_glx_read_pixels_data (const xcb_glx_read_pixels_reply_t *R);

int
xcb_glx_read_pixels_data_length (const xcb_glx_read_pixels_reply_t *R);

xcb_generic_iterator_t
xcb_glx_read_pixels_data_end (const xcb_glx_read_pixels_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_read_pixels_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_glx_read_pixels_reply_t *
xcb_glx_read_pixels_reply (xcb_connection_t              *c,
                           xcb_glx_read_pixels_cookie_t   cookie  /**< */,
                           xcb_generic_error_t          **e);

int
xcb_glx_get_booleanv_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_glx_get_booleanv_cookie_t
xcb_glx_get_booleanv (xcb_connection_t      *c,
                      xcb_glx_context_tag_t  context_tag,
                      int32_t                pname);

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
xcb_glx_get_booleanv_cookie_t
xcb_glx_get_booleanv_unchecked (xcb_connection_t      *c,
                                xcb_glx_context_tag_t  context_tag,
                                int32_t                pname);

uint8_t *
xcb_glx_get_booleanv_data (const xcb_glx_get_booleanv_reply_t *R);

int
xcb_glx_get_booleanv_data_length (const xcb_glx_get_booleanv_reply_t *R);

xcb_generic_iterator_t
xcb_glx_get_booleanv_data_end (const xcb_glx_get_booleanv_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_booleanv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_glx_get_booleanv_reply_t *
xcb_glx_get_booleanv_reply (xcb_connection_t               *c,
                            xcb_glx_get_booleanv_cookie_t   cookie  /**< */,
                            xcb_generic_error_t           **e);

int
xcb_glx_get_clip_plane_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_glx_get_clip_plane_cookie_t
xcb_glx_get_clip_plane (xcb_connection_t      *c,
                        xcb_glx_context_tag_t  context_tag,
                        int32_t                plane);

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
xcb_glx_get_clip_plane_cookie_t
xcb_glx_get_clip_plane_unchecked (xcb_connection_t      *c,
                                  xcb_glx_context_tag_t  context_tag,
                                  int32_t                plane);

xcb_glx_float64_t *
xcb_glx_get_clip_plane_data (const xcb_glx_get_clip_plane_reply_t *R);

int
xcb_glx_get_clip_plane_data_length (const xcb_glx_get_clip_plane_reply_t *R);

xcb_generic_iterator_t
xcb_glx_get_clip_plane_data_end (const xcb_glx_get_clip_plane_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_clip_plane_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_glx_get_clip_plane_reply_t *
xcb_glx_get_clip_plane_reply (xcb_connection_t                 *c,
                              xcb_glx_get_clip_plane_cookie_t   cookie  /**< */,
                              xcb_generic_error_t             **e);

int
xcb_glx_get_doublev_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_glx_get_doublev_cookie_t
xcb_glx_get_doublev (xcb_connection_t      *c,
                     xcb_glx_context_tag_t  context_tag,
                     uint32_t               pname);

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
xcb_glx_get_doublev_cookie_t
xcb_glx_get_doublev_unchecked (xcb_connection_t      *c,
                               xcb_glx_context_tag_t  context_tag,
                               uint32_t               pname);

xcb_glx_float64_t *
xcb_glx_get_doublev_data (const xcb_glx_get_doublev_reply_t *R);

int
xcb_glx_get_doublev_data_length (const xcb_glx_get_doublev_reply_t *R);

xcb_generic_iterator_t
xcb_glx_get_doublev_data_end (const xcb_glx_get_doublev_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_doublev_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_glx_get_doublev_reply_t *
xcb_glx_get_doublev_reply (xcb_connection_t              *c,
                           xcb_glx_get_doublev_cookie_t   cookie  /**< */,
                           xcb_generic_error_t          **e);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_glx_get_error_cookie_t
xcb_glx_get_error (xcb_connection_t      *c,
                   xcb_glx_context_tag_t  context_tag);

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
xcb_glx_get_error_cookie_t
xcb_glx_get_error_unchecked (xcb_connection_t      *c,
                             xcb_glx_context_tag_t  context_tag);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_error_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_glx_get_error_reply_t *
xcb_glx_get_error_reply (xcb_connection_t            *c,
                         xcb_glx_get_error_cookie_t   cookie  /**< */,
                         xcb_generic_error_t        **e);

int
xcb_glx_get_floatv_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_glx_get_floatv_cookie_t
xcb_glx_get_floatv (xcb_connection_t      *c,
                    xcb_glx_context_tag_t  context_tag,
                    uint32_t               pname);

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
xcb_glx_get_floatv_cookie_t
xcb_glx_get_floatv_unchecked (xcb_connection_t      *c,
                              xcb_glx_context_tag_t  context_tag,
                              uint32_t               pname);

xcb_glx_float32_t *
xcb_glx_get_floatv_data (const xcb_glx_get_floatv_reply_t *R);

int
xcb_glx_get_floatv_data_length (const xcb_glx_get_floatv_reply_t *R);

xcb_generic_iterator_t
xcb_glx_get_floatv_data_end (const xcb_glx_get_floatv_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_floatv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_glx_get_floatv_reply_t *
xcb_glx_get_floatv_reply (xcb_connection_t             *c,
                          xcb_glx_get_floatv_cookie_t   cookie  /**< */,
                          xcb_generic_error_t         **e);

int
xcb_glx_get_integerv_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_glx_get_integerv_cookie_t
xcb_glx_get_integerv (xcb_connection_t      *c,
                      xcb_glx_context_tag_t  context_tag,
                      uint32_t               pname);

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
xcb_glx_get_integerv_cookie_t
xcb_glx_get_integerv_unchecked (xcb_connection_t      *c,
                                xcb_glx_context_tag_t  context_tag,
                                uint32_t               pname);

int32_t *
xcb_glx_get_integerv_data (const xcb_glx_get_integerv_reply_t *R);

int
xcb_glx_get_integerv_data_length (const xcb_glx_get_integerv_reply_t *R);

xcb_generic_iterator_t
xcb_glx_get_integerv_data_end (const xcb_glx_get_integerv_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_integerv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_glx_get_integerv_reply_t *
xcb_glx_get_integerv_reply (xcb_connection_t               *c,
                            xcb_glx_get_integerv_cookie_t   cookie  /**< */,
                            xcb_generic_error_t           **e);

int
xcb_glx_get_lightfv_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_glx_get_lightfv_cookie_t
xcb_glx_get_lightfv (xcb_connection_t      *c,
                     xcb_glx_context_tag_t  context_tag,
                     uint32_t               light,
                     uint32_t               pname);

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
xcb_glx_get_lightfv_cookie_t
xcb_glx_get_lightfv_unchecked (xcb_connection_t      *c,
                               xcb_glx_context_tag_t  context_tag,
                               uint32_t               light,
                               uint32_t               pname);

xcb_glx_float32_t *
xcb_glx_get_lightfv_data (const xcb_glx_get_lightfv_reply_t *R);

int
xcb_glx_get_lightfv_data_length (const xcb_glx_get_lightfv_reply_t *R);

xcb_generic_iterator_t
xcb_glx_get_lightfv_data_end (const xcb_glx_get_lightfv_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_lightfv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_glx_get_lightfv_reply_t *
xcb_glx_get_lightfv_reply (xcb_connection_t              *c,
                           xcb_glx_get_lightfv_cookie_t   cookie  /**< */,
                           xcb_generic_error_t          **e);

int
xcb_glx_get_lightiv_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_glx_get_lightiv_cookie_t
xcb_glx_get_lightiv (xcb_connection_t      *c,
                     xcb_glx_context_tag_t  context_tag,
                     uint32_t               light,
                     uint32_t               pname);

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
xcb_glx_get_lightiv_cookie_t
xcb_glx_get_lightiv_unchecked (xcb_connection_t      *c,
                               xcb_glx_context_tag_t  context_tag,
                               uint32_t               light,
                               uint32_t               pname);

int32_t *
xcb_glx_get_lightiv_data (const xcb_glx_get_lightiv_reply_t *R);

int
xcb_glx_get_lightiv_data_length (const xcb_glx_get_lightiv_reply_t *R);

xcb_generic_iterator_t
xcb_glx_get_lightiv_data_end (const xcb_glx_get_lightiv_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_lightiv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_glx_get_lightiv_reply_t *
xcb_glx_get_lightiv_reply (xcb_connection_t              *c,
                           xcb_glx_get_lightiv_cookie_t   cookie  /**< */,
                           xcb_generic_error_t          **e);

int
xcb_glx_get_mapdv_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_glx_get_mapdv_cookie_t
xcb_glx_get_mapdv (xcb_connection_t      *c,
                   xcb_glx_context_tag_t  context_tag,
                   uint32_t               target,
                   uint32_t               query);

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
xcb_glx_get_mapdv_cookie_t
xcb_glx_get_mapdv_unchecked (xcb_connection_t      *c,
                             xcb_glx_context_tag_t  context_tag,
                             uint32_t               target,
                             uint32_t               query);

xcb_glx_float64_t *
xcb_glx_get_mapdv_data (const xcb_glx_get_mapdv_reply_t *R);

int
xcb_glx_get_mapdv_data_length (const xcb_glx_get_mapdv_reply_t *R);

xcb_generic_iterator_t
xcb_glx_get_mapdv_data_end (const xcb_glx_get_mapdv_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_mapdv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_glx_get_mapdv_reply_t *
xcb_glx_get_mapdv_reply (xcb_connection_t            *c,
                         xcb_glx_get_mapdv_cookie_t   cookie  /**< */,
                         xcb_generic_error_t        **e);

int
xcb_glx_get_mapfv_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_glx_get_mapfv_cookie_t
xcb_glx_get_mapfv (xcb_connection_t      *c,
                   xcb_glx_context_tag_t  context_tag,
                   uint32_t               target,
                   uint32_t               query);

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
xcb_glx_get_mapfv_cookie_t
xcb_glx_get_mapfv_unchecked (xcb_connection_t      *c,
                             xcb_glx_context_tag_t  context_tag,
                             uint32_t               target,
                             uint32_t               query);

xcb_glx_float32_t *
xcb_glx_get_mapfv_data (const xcb_glx_get_mapfv_reply_t *R);

int
xcb_glx_get_mapfv_data_length (const xcb_glx_get_mapfv_reply_t *R);

xcb_generic_iterator_t
xcb_glx_get_mapfv_data_end (const xcb_glx_get_mapfv_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_mapfv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_glx_get_mapfv_reply_t *
xcb_glx_get_mapfv_reply (xcb_connection_t            *c,
                         xcb_glx_get_mapfv_cookie_t   cookie  /**< */,
                         xcb_generic_error_t        **e);

int
xcb_glx_get_mapiv_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_glx_get_mapiv_cookie_t
xcb_glx_get_mapiv (xcb_connection_t      *c,
                   xcb_glx_context_tag_t  context_tag,
                   uint32_t               target,
                   uint32_t               query);

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
xcb_glx_get_mapiv_cookie_t
xcb_glx_get_mapiv_unchecked (xcb_connection_t      *c,
                             xcb_glx_context_tag_t  context_tag,
                             uint32_t               target,
                             uint32_t               query);

int32_t *
xcb_glx_get_mapiv_data (const xcb_glx_get_mapiv_reply_t *R);

int
xcb_glx_get_mapiv_data_length (const xcb_glx_get_mapiv_reply_t *R);

xcb_generic_iterator_t
xcb_glx_get_mapiv_data_end (const xcb_glx_get_mapiv_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_mapiv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_glx_get_mapiv_reply_t *
xcb_glx_get_mapiv_reply (xcb_connection_t            *c,
                         xcb_glx_get_mapiv_cookie_t   cookie  /**< */,
                         xcb_generic_error_t        **e);

int
xcb_glx_get_materialfv_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_glx_get_materialfv_cookie_t
xcb_glx_get_materialfv (xcb_connection_t      *c,
                        xcb_glx_context_tag_t  context_tag,
                        uint32_t               face,
                        uint32_t               pname);

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
xcb_glx_get_materialfv_cookie_t
xcb_glx_get_materialfv_unchecked (xcb_connection_t      *c,
                                  xcb_glx_context_tag_t  context_tag,
                                  uint32_t               face,
                                  uint32_t               pname);

xcb_glx_float32_t *
xcb_glx_get_materialfv_data (const xcb_glx_get_materialfv_reply_t *R);

int
xcb_glx_get_materialfv_data_length (const xcb_glx_get_materialfv_reply_t *R);

xcb_generic_iterator_t
xcb_glx_get_materialfv_data_end (const xcb_glx_get_materialfv_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_materialfv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_glx_get_materialfv_reply_t *
xcb_glx_get_materialfv_reply (xcb_connection_t                 *c,
                              xcb_glx_get_materialfv_cookie_t   cookie  /**< */,
                              xcb_generic_error_t             **e);

int
xcb_glx_get_materialiv_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_glx_get_materialiv_cookie_t
xcb_glx_get_materialiv (xcb_connection_t      *c,
                        xcb_glx_context_tag_t  context_tag,
                        uint32_t               face,
                        uint32_t               pname);

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
xcb_glx_get_materialiv_cookie_t
xcb_glx_get_materialiv_unchecked (xcb_connection_t      *c,
                                  xcb_glx_context_tag_t  context_tag,
                                  uint32_t               face,
                                  uint32_t               pname);

int32_t *
xcb_glx_get_materialiv_data (const xcb_glx_get_materialiv_reply_t *R);

int
xcb_glx_get_materialiv_data_length (const xcb_glx_get_materialiv_reply_t *R);

xcb_generic_iterator_t
xcb_glx_get_materialiv_data_end (const xcb_glx_get_materialiv_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_materialiv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_glx_get_materialiv_reply_t *
xcb_glx_get_materialiv_reply (xcb_connection_t                 *c,
                              xcb_glx_get_materialiv_cookie_t   cookie  /**< */,
                              xcb_generic_error_t             **e);

int
xcb_glx_get_pixel_mapfv_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_glx_get_pixel_mapfv_cookie_t
xcb_glx_get_pixel_mapfv (xcb_connection_t      *c,
                         xcb_glx_context_tag_t  context_tag,
                         uint32_t               map);

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
xcb_glx_get_pixel_mapfv_cookie_t
xcb_glx_get_pixel_mapfv_unchecked (xcb_connection_t      *c,
                                   xcb_glx_context_tag_t  context_tag,
                                   uint32_t               map);

xcb_glx_float32_t *
xcb_glx_get_pixel_mapfv_data (const xcb_glx_get_pixel_mapfv_reply_t *R);

int
xcb_glx_get_pixel_mapfv_data_length (const xcb_glx_get_pixel_mapfv_reply_t *R);

xcb_generic_iterator_t
xcb_glx_get_pixel_mapfv_data_end (const xcb_glx_get_pixel_mapfv_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_pixel_mapfv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_glx_get_pixel_mapfv_reply_t *
xcb_glx_get_pixel_mapfv_reply (xcb_connection_t                  *c,
                               xcb_glx_get_pixel_mapfv_cookie_t   cookie  /**< */,
                               xcb_generic_error_t              **e);

int
xcb_glx_get_pixel_mapuiv_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_glx_get_pixel_mapuiv_cookie_t
xcb_glx_get_pixel_mapuiv (xcb_connection_t      *c,
                          xcb_glx_context_tag_t  context_tag,
                          uint32_t               map);

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
xcb_glx_get_pixel_mapuiv_cookie_t
xcb_glx_get_pixel_mapuiv_unchecked (xcb_connection_t      *c,
                                    xcb_glx_context_tag_t  context_tag,
                                    uint32_t               map);

uint32_t *
xcb_glx_get_pixel_mapuiv_data (const xcb_glx_get_pixel_mapuiv_reply_t *R);

int
xcb_glx_get_pixel_mapuiv_data_length (const xcb_glx_get_pixel_mapuiv_reply_t *R);

xcb_generic_iterator_t
xcb_glx_get_pixel_mapuiv_data_end (const xcb_glx_get_pixel_mapuiv_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_pixel_mapuiv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_glx_get_pixel_mapuiv_reply_t *
xcb_glx_get_pixel_mapuiv_reply (xcb_connection_t                   *c,
                                xcb_glx_get_pixel_mapuiv_cookie_t   cookie  /**< */,
                                xcb_generic_error_t               **e);

int
xcb_glx_get_pixel_mapusv_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_glx_get_pixel_mapusv_cookie_t
xcb_glx_get_pixel_mapusv (xcb_connection_t      *c,
                          xcb_glx_context_tag_t  context_tag,
                          uint32_t               map);

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
xcb_glx_get_pixel_mapusv_cookie_t
xcb_glx_get_pixel_mapusv_unchecked (xcb_connection_t      *c,
                                    xcb_glx_context_tag_t  context_tag,
                                    uint32_t               map);

uint16_t *
xcb_glx_get_pixel_mapusv_data (const xcb_glx_get_pixel_mapusv_reply_t *R);

int
xcb_glx_get_pixel_mapusv_data_length (const xcb_glx_get_pixel_mapusv_reply_t *R);

xcb_generic_iterator_t
xcb_glx_get_pixel_mapusv_data_end (const xcb_glx_get_pixel_mapusv_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_pixel_mapusv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_glx_get_pixel_mapusv_reply_t *
xcb_glx_get_pixel_mapusv_reply (xcb_connection_t                   *c,
                                xcb_glx_get_pixel_mapusv_cookie_t   cookie  /**< */,
                                xcb_generic_error_t               **e);

int
xcb_glx_get_polygon_stipple_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_glx_get_polygon_stipple_cookie_t
xcb_glx_get_polygon_stipple (xcb_connection_t      *c,
                             xcb_glx_context_tag_t  context_tag,
                             uint8_t                lsb_first);

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
xcb_glx_get_polygon_stipple_cookie_t
xcb_glx_get_polygon_stipple_unchecked (xcb_connection_t      *c,
                                       xcb_glx_context_tag_t  context_tag,
                                       uint8_t                lsb_first);

uint8_t *
xcb_glx_get_polygon_stipple_data (const xcb_glx_get_polygon_stipple_reply_t *R);

int
xcb_glx_get_polygon_stipple_data_length (const xcb_glx_get_polygon_stipple_reply_t *R);

xcb_generic_iterator_t
xcb_glx_get_polygon_stipple_data_end (const xcb_glx_get_polygon_stipple_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_polygon_stipple_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_glx_get_polygon_stipple_reply_t *
xcb_glx_get_polygon_stipple_reply (xcb_connection_t                      *c,
                                   xcb_glx_get_polygon_stipple_cookie_t   cookie  /**< */,
                                   xcb_generic_error_t                  **e);

int
xcb_glx_get_string_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_glx_get_string_cookie_t
xcb_glx_get_string (xcb_connection_t      *c,
                    xcb_glx_context_tag_t  context_tag,
                    uint32_t               name);

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
xcb_glx_get_string_cookie_t
xcb_glx_get_string_unchecked (xcb_connection_t      *c,
                              xcb_glx_context_tag_t  context_tag,
                              uint32_t               name);

char *
xcb_glx_get_string_string (const xcb_glx_get_string_reply_t *R);

int
xcb_glx_get_string_string_length (const xcb_glx_get_string_reply_t *R);

xcb_generic_iterator_t
xcb_glx_get_string_string_end (const xcb_glx_get_string_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_string_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_glx_get_string_reply_t *
xcb_glx_get_string_reply (xcb_connection_t             *c,
                          xcb_glx_get_string_cookie_t   cookie  /**< */,
                          xcb_generic_error_t         **e);

int
xcb_glx_get_tex_envfv_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_glx_get_tex_envfv_cookie_t
xcb_glx_get_tex_envfv (xcb_connection_t      *c,
                       xcb_glx_context_tag_t  context_tag,
                       uint32_t               target,
                       uint32_t               pname);

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
xcb_glx_get_tex_envfv_cookie_t
xcb_glx_get_tex_envfv_unchecked (xcb_connection_t      *c,
                                 xcb_glx_context_tag_t  context_tag,
                                 uint32_t               target,
                                 uint32_t               pname);

xcb_glx_float32_t *
xcb_glx_get_tex_envfv_data (const xcb_glx_get_tex_envfv_reply_t *R);

int
xcb_glx_get_tex_envfv_data_length (const xcb_glx_get_tex_envfv_reply_t *R);

xcb_generic_iterator_t
xcb_glx_get_tex_envfv_data_end (const xcb_glx_get_tex_envfv_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_tex_envfv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_glx_get_tex_envfv_reply_t *
xcb_glx_get_tex_envfv_reply (xcb_connection_t                *c,
                             xcb_glx_get_tex_envfv_cookie_t   cookie  /**< */,
                             xcb_generic_error_t            **e);

int
xcb_glx_get_tex_enviv_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_glx_get_tex_enviv_cookie_t
xcb_glx_get_tex_enviv (xcb_connection_t      *c,
                       xcb_glx_context_tag_t  context_tag,
                       uint32_t               target,
                       uint32_t               pname);

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
xcb_glx_get_tex_enviv_cookie_t
xcb_glx_get_tex_enviv_unchecked (xcb_connection_t      *c,
                                 xcb_glx_context_tag_t  context_tag,
                                 uint32_t               target,
                                 uint32_t               pname);

int32_t *
xcb_glx_get_tex_enviv_data (const xcb_glx_get_tex_enviv_reply_t *R);

int
xcb_glx_get_tex_enviv_data_length (const xcb_glx_get_tex_enviv_reply_t *R);

xcb_generic_iterator_t
xcb_glx_get_tex_enviv_data_end (const xcb_glx_get_tex_enviv_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_tex_enviv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_glx_get_tex_enviv_reply_t *
xcb_glx_get_tex_enviv_reply (xcb_connection_t                *c,
                             xcb_glx_get_tex_enviv_cookie_t   cookie  /**< */,
                             xcb_generic_error_t            **e);

int
xcb_glx_get_tex_gendv_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_glx_get_tex_gendv_cookie_t
xcb_glx_get_tex_gendv (xcb_connection_t      *c,
                       xcb_glx_context_tag_t  context_tag,
                       uint32_t               coord,
                       uint32_t               pname);

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
xcb_glx_get_tex_gendv_cookie_t
xcb_glx_get_tex_gendv_unchecked (xcb_connection_t      *c,
                                 xcb_glx_context_tag_t  context_tag,
                                 uint32_t               coord,
                                 uint32_t               pname);

xcb_glx_float64_t *
xcb_glx_get_tex_gendv_data (const xcb_glx_get_tex_gendv_reply_t *R);

int
xcb_glx_get_tex_gendv_data_length (const xcb_glx_get_tex_gendv_reply_t *R);

xcb_generic_iterator_t
xcb_glx_get_tex_gendv_data_end (const xcb_glx_get_tex_gendv_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_tex_gendv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_glx_get_tex_gendv_reply_t *
xcb_glx_get_tex_gendv_reply (xcb_connection_t                *c,
                             xcb_glx_get_tex_gendv_cookie_t   cookie  /**< */,
                             xcb_generic_error_t            **e);

int
xcb_glx_get_tex_genfv_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_glx_get_tex_genfv_cookie_t
xcb_glx_get_tex_genfv (xcb_connection_t      *c,
                       xcb_glx_context_tag_t  context_tag,
                       uint32_t               coord,
                       uint32_t               pname);

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
xcb_glx_get_tex_genfv_cookie_t
xcb_glx_get_tex_genfv_unchecked (xcb_connection_t      *c,
                                 xcb_glx_context_tag_t  context_tag,
                                 uint32_t               coord,
                                 uint32_t               pname);

xcb_glx_float32_t *
xcb_glx_get_tex_genfv_data (const xcb_glx_get_tex_genfv_reply_t *R);

int
xcb_glx_get_tex_genfv_data_length (const xcb_glx_get_tex_genfv_reply_t *R);

xcb_generic_iterator_t
xcb_glx_get_tex_genfv_data_end (const xcb_glx_get_tex_genfv_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_tex_genfv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_glx_get_tex_genfv_reply_t *
xcb_glx_get_tex_genfv_reply (xcb_connection_t                *c,
                             xcb_glx_get_tex_genfv_cookie_t   cookie  /**< */,
                             xcb_generic_error_t            **e);

int
xcb_glx_get_tex_geniv_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_glx_get_tex_geniv_cookie_t
xcb_glx_get_tex_geniv (xcb_connection_t      *c,
                       xcb_glx_context_tag_t  context_tag,
                       uint32_t               coord,
                       uint32_t               pname);

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
xcb_glx_get_tex_geniv_cookie_t
xcb_glx_get_tex_geniv_unchecked (xcb_connection_t      *c,
                                 xcb_glx_context_tag_t  context_tag,
                                 uint32_t               coord,
                                 uint32_t               pname);

int32_t *
xcb_glx_get_tex_geniv_data (const xcb_glx_get_tex_geniv_reply_t *R);

int
xcb_glx_get_tex_geniv_data_length (const xcb_glx_get_tex_geniv_reply_t *R);

xcb_generic_iterator_t
xcb_glx_get_tex_geniv_data_end (const xcb_glx_get_tex_geniv_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_tex_geniv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_glx_get_tex_geniv_reply_t *
xcb_glx_get_tex_geniv_reply (xcb_connection_t                *c,
                             xcb_glx_get_tex_geniv_cookie_t   cookie  /**< */,
                             xcb_generic_error_t            **e);

int
xcb_glx_get_tex_image_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_glx_get_tex_image_cookie_t
xcb_glx_get_tex_image (xcb_connection_t      *c,
                       xcb_glx_context_tag_t  context_tag,
                       uint32_t               target,
                       int32_t                level,
                       uint32_t               format,
                       uint32_t               type,
                       uint8_t                swap_bytes);

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
xcb_glx_get_tex_image_cookie_t
xcb_glx_get_tex_image_unchecked (xcb_connection_t      *c,
                                 xcb_glx_context_tag_t  context_tag,
                                 uint32_t               target,
                                 int32_t                level,
                                 uint32_t               format,
                                 uint32_t               type,
                                 uint8_t                swap_bytes);

uint8_t *
xcb_glx_get_tex_image_data (const xcb_glx_get_tex_image_reply_t *R);

int
xcb_glx_get_tex_image_data_length (const xcb_glx_get_tex_image_reply_t *R);

xcb_generic_iterator_t
xcb_glx_get_tex_image_data_end (const xcb_glx_get_tex_image_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_tex_image_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_glx_get_tex_image_reply_t *
xcb_glx_get_tex_image_reply (xcb_connection_t                *c,
                             xcb_glx_get_tex_image_cookie_t   cookie  /**< */,
                             xcb_generic_error_t            **e);

int
xcb_glx_get_tex_parameterfv_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_glx_get_tex_parameterfv_cookie_t
xcb_glx_get_tex_parameterfv (xcb_connection_t      *c,
                             xcb_glx_context_tag_t  context_tag,
                             uint32_t               target,
                             uint32_t               pname);

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
xcb_glx_get_tex_parameterfv_cookie_t
xcb_glx_get_tex_parameterfv_unchecked (xcb_connection_t      *c,
                                       xcb_glx_context_tag_t  context_tag,
                                       uint32_t               target,
                                       uint32_t               pname);

xcb_glx_float32_t *
xcb_glx_get_tex_parameterfv_data (const xcb_glx_get_tex_parameterfv_reply_t *R);

int
xcb_glx_get_tex_parameterfv_data_length (const xcb_glx_get_tex_parameterfv_reply_t *R);

xcb_generic_iterator_t
xcb_glx_get_tex_parameterfv_data_end (const xcb_glx_get_tex_parameterfv_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_tex_parameterfv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_glx_get_tex_parameterfv_reply_t *
xcb_glx_get_tex_parameterfv_reply (xcb_connection_t                      *c,
                                   xcb_glx_get_tex_parameterfv_cookie_t   cookie  /**< */,
                                   xcb_generic_error_t                  **e);

int
xcb_glx_get_tex_parameteriv_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_glx_get_tex_parameteriv_cookie_t
xcb_glx_get_tex_parameteriv (xcb_connection_t      *c,
                             xcb_glx_context_tag_t  context_tag,
                             uint32_t               target,
                             uint32_t               pname);

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
xcb_glx_get_tex_parameteriv_cookie_t
xcb_glx_get_tex_parameteriv_unchecked (xcb_connection_t      *c,
                                       xcb_glx_context_tag_t  context_tag,
                                       uint32_t               target,
                                       uint32_t               pname);

int32_t *
xcb_glx_get_tex_parameteriv_data (const xcb_glx_get_tex_parameteriv_reply_t *R);

int
xcb_glx_get_tex_parameteriv_data_length (const xcb_glx_get_tex_parameteriv_reply_t *R);

xcb_generic_iterator_t
xcb_glx_get_tex_parameteriv_data_end (const xcb_glx_get_tex_parameteriv_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_tex_parameteriv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_glx_get_tex_parameteriv_reply_t *
xcb_glx_get_tex_parameteriv_reply (xcb_connection_t                      *c,
                                   xcb_glx_get_tex_parameteriv_cookie_t   cookie  /**< */,
                                   xcb_generic_error_t                  **e);

int
xcb_glx_get_tex_level_parameterfv_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_glx_get_tex_level_parameterfv_cookie_t
xcb_glx_get_tex_level_parameterfv (xcb_connection_t      *c,
                                   xcb_glx_context_tag_t  context_tag,
                                   uint32_t               target,
                                   int32_t                level,
                                   uint32_t               pname);

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
xcb_glx_get_tex_level_parameterfv_cookie_t
xcb_glx_get_tex_level_parameterfv_unchecked (xcb_connection_t      *c,
                                             xcb_glx_context_tag_t  context_tag,
                                             uint32_t               target,
                                             int32_t                level,
                                             uint32_t               pname);

xcb_glx_float32_t *
xcb_glx_get_tex_level_parameterfv_data (const xcb_glx_get_tex_level_parameterfv_reply_t *R);

int
xcb_glx_get_tex_level_parameterfv_data_length (const xcb_glx_get_tex_level_parameterfv_reply_t *R);

xcb_generic_iterator_t
xcb_glx_get_tex_level_parameterfv_data_end (const xcb_glx_get_tex_level_parameterfv_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_tex_level_parameterfv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_glx_get_tex_level_parameterfv_reply_t *
xcb_glx_get_tex_level_parameterfv_reply (xcb_connection_t                            *c,
                                         xcb_glx_get_tex_level_parameterfv_cookie_t   cookie  /**< */,
                                         xcb_generic_error_t                        **e);

int
xcb_glx_get_tex_level_parameteriv_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_glx_get_tex_level_parameteriv_cookie_t
xcb_glx_get_tex_level_parameteriv (xcb_connection_t      *c,
                                   xcb_glx_context_tag_t  context_tag,
                                   uint32_t               target,
                                   int32_t                level,
                                   uint32_t               pname);

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
xcb_glx_get_tex_level_parameteriv_cookie_t
xcb_glx_get_tex_level_parameteriv_unchecked (xcb_connection_t      *c,
                                             xcb_glx_context_tag_t  context_tag,
                                             uint32_t               target,
                                             int32_t                level,
                                             uint32_t               pname);

int32_t *
xcb_glx_get_tex_level_parameteriv_data (const xcb_glx_get_tex_level_parameteriv_reply_t *R);

int
xcb_glx_get_tex_level_parameteriv_data_length (const xcb_glx_get_tex_level_parameteriv_reply_t *R);

xcb_generic_iterator_t
xcb_glx_get_tex_level_parameteriv_data_end (const xcb_glx_get_tex_level_parameteriv_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_tex_level_parameteriv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_glx_get_tex_level_parameteriv_reply_t *
xcb_glx_get_tex_level_parameteriv_reply (xcb_connection_t                            *c,
                                         xcb_glx_get_tex_level_parameteriv_cookie_t   cookie  /**< */,
                                         xcb_generic_error_t                        **e);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_glx_is_enabled_cookie_t
xcb_glx_is_enabled (xcb_connection_t      *c,
                    xcb_glx_context_tag_t  context_tag,
                    uint32_t               capability);

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
xcb_glx_is_enabled_cookie_t
xcb_glx_is_enabled_unchecked (xcb_connection_t      *c,
                              xcb_glx_context_tag_t  context_tag,
                              uint32_t               capability);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_is_enabled_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_glx_is_enabled_reply_t *
xcb_glx_is_enabled_reply (xcb_connection_t             *c,
                          xcb_glx_is_enabled_cookie_t   cookie  /**< */,
                          xcb_generic_error_t         **e);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_glx_is_list_cookie_t
xcb_glx_is_list (xcb_connection_t      *c,
                 xcb_glx_context_tag_t  context_tag,
                 uint32_t               list);

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
xcb_glx_is_list_cookie_t
xcb_glx_is_list_unchecked (xcb_connection_t      *c,
                           xcb_glx_context_tag_t  context_tag,
                           uint32_t               list);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_is_list_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_glx_is_list_reply_t *
xcb_glx_is_list_reply (xcb_connection_t          *c,
                       xcb_glx_is_list_cookie_t   cookie  /**< */,
                       xcb_generic_error_t      **e);

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
xcb_glx_flush_checked (xcb_connection_t      *c,
                       xcb_glx_context_tag_t  context_tag);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_glx_flush (xcb_connection_t      *c,
               xcb_glx_context_tag_t  context_tag);

int
xcb_glx_are_textures_resident_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_glx_are_textures_resident_cookie_t
xcb_glx_are_textures_resident (xcb_connection_t      *c,
                               xcb_glx_context_tag_t  context_tag,
                               int32_t                n,
                               const uint32_t        *textures);

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
xcb_glx_are_textures_resident_cookie_t
xcb_glx_are_textures_resident_unchecked (xcb_connection_t      *c,
                                         xcb_glx_context_tag_t  context_tag,
                                         int32_t                n,
                                         const uint32_t        *textures);

uint8_t *
xcb_glx_are_textures_resident_data (const xcb_glx_are_textures_resident_reply_t *R);

int
xcb_glx_are_textures_resident_data_length (const xcb_glx_are_textures_resident_reply_t *R);

xcb_generic_iterator_t
xcb_glx_are_textures_resident_data_end (const xcb_glx_are_textures_resident_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_are_textures_resident_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_glx_are_textures_resident_reply_t *
xcb_glx_are_textures_resident_reply (xcb_connection_t                        *c,
                                     xcb_glx_are_textures_resident_cookie_t   cookie  /**< */,
                                     xcb_generic_error_t                    **e);

int
xcb_glx_delete_textures_sizeof (const void  *_buffer);

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
xcb_glx_delete_textures_checked (xcb_connection_t      *c,
                                 xcb_glx_context_tag_t  context_tag,
                                 int32_t                n,
                                 const uint32_t        *textures);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_glx_delete_textures (xcb_connection_t      *c,
                         xcb_glx_context_tag_t  context_tag,
                         int32_t                n,
                         const uint32_t        *textures);

uint32_t *
xcb_glx_delete_textures_textures (const xcb_glx_delete_textures_request_t *R);

int
xcb_glx_delete_textures_textures_length (const xcb_glx_delete_textures_request_t *R);

xcb_generic_iterator_t
xcb_glx_delete_textures_textures_end (const xcb_glx_delete_textures_request_t *R);

int
xcb_glx_gen_textures_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_glx_gen_textures_cookie_t
xcb_glx_gen_textures (xcb_connection_t      *c,
                      xcb_glx_context_tag_t  context_tag,
                      int32_t                n);

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
xcb_glx_gen_textures_cookie_t
xcb_glx_gen_textures_unchecked (xcb_connection_t      *c,
                                xcb_glx_context_tag_t  context_tag,
                                int32_t                n);

uint32_t *
xcb_glx_gen_textures_data (const xcb_glx_gen_textures_reply_t *R);

int
xcb_glx_gen_textures_data_length (const xcb_glx_gen_textures_reply_t *R);

xcb_generic_iterator_t
xcb_glx_gen_textures_data_end (const xcb_glx_gen_textures_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_gen_textures_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_glx_gen_textures_reply_t *
xcb_glx_gen_textures_reply (xcb_connection_t               *c,
                            xcb_glx_gen_textures_cookie_t   cookie  /**< */,
                            xcb_generic_error_t           **e);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_glx_is_texture_cookie_t
xcb_glx_is_texture (xcb_connection_t      *c,
                    xcb_glx_context_tag_t  context_tag,
                    uint32_t               texture);

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
xcb_glx_is_texture_cookie_t
xcb_glx_is_texture_unchecked (xcb_connection_t      *c,
                              xcb_glx_context_tag_t  context_tag,
                              uint32_t               texture);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_is_texture_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_glx_is_texture_reply_t *
xcb_glx_is_texture_reply (xcb_connection_t             *c,
                          xcb_glx_is_texture_cookie_t   cookie  /**< */,
                          xcb_generic_error_t         **e);

int
xcb_glx_get_color_table_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_glx_get_color_table_cookie_t
xcb_glx_get_color_table (xcb_connection_t      *c,
                         xcb_glx_context_tag_t  context_tag,
                         uint32_t               target,
                         uint32_t               format,
                         uint32_t               type,
                         uint8_t                swap_bytes);

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
xcb_glx_get_color_table_cookie_t
xcb_glx_get_color_table_unchecked (xcb_connection_t      *c,
                                   xcb_glx_context_tag_t  context_tag,
                                   uint32_t               target,
                                   uint32_t               format,
                                   uint32_t               type,
                                   uint8_t                swap_bytes);

uint8_t *
xcb_glx_get_color_table_data (const xcb_glx_get_color_table_reply_t *R);

int
xcb_glx_get_color_table_data_length (const xcb_glx_get_color_table_reply_t *R);

xcb_generic_iterator_t
xcb_glx_get_color_table_data_end (const xcb_glx_get_color_table_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_color_table_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_glx_get_color_table_reply_t *
xcb_glx_get_color_table_reply (xcb_connection_t                  *c,
                               xcb_glx_get_color_table_cookie_t   cookie  /**< */,
                               xcb_generic_error_t              **e);

int
xcb_glx_get_color_table_parameterfv_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_glx_get_color_table_parameterfv_cookie_t
xcb_glx_get_color_table_parameterfv (xcb_connection_t      *c,
                                     xcb_glx_context_tag_t  context_tag,
                                     uint32_t               target,
                                     uint32_t               pname);

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
xcb_glx_get_color_table_parameterfv_cookie_t
xcb_glx_get_color_table_parameterfv_unchecked (xcb_connection_t      *c,
                                               xcb_glx_context_tag_t  context_tag,
                                               uint32_t               target,
                                               uint32_t               pname);

xcb_glx_float32_t *
xcb_glx_get_color_table_parameterfv_data (const xcb_glx_get_color_table_parameterfv_reply_t *R);

int
xcb_glx_get_color_table_parameterfv_data_length (const xcb_glx_get_color_table_parameterfv_reply_t *R);

xcb_generic_iterator_t
xcb_glx_get_color_table_parameterfv_data_end (const xcb_glx_get_color_table_parameterfv_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_color_table_parameterfv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_glx_get_color_table_parameterfv_reply_t *
xcb_glx_get_color_table_parameterfv_reply (xcb_connection_t                              *c,
                                           xcb_glx_get_color_table_parameterfv_cookie_t   cookie  /**< */,
                                           xcb_generic_error_t                          **e);

int
xcb_glx_get_color_table_parameteriv_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_glx_get_color_table_parameteriv_cookie_t
xcb_glx_get_color_table_parameteriv (xcb_connection_t      *c,
                                     xcb_glx_context_tag_t  context_tag,
                                     uint32_t               target,
                                     uint32_t               pname);

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
xcb_glx_get_color_table_parameteriv_cookie_t
xcb_glx_get_color_table_parameteriv_unchecked (xcb_connection_t      *c,
                                               xcb_glx_context_tag_t  context_tag,
                                               uint32_t               target,
                                               uint32_t               pname);

int32_t *
xcb_glx_get_color_table_parameteriv_data (const xcb_glx_get_color_table_parameteriv_reply_t *R);

int
xcb_glx_get_color_table_parameteriv_data_length (const xcb_glx_get_color_table_parameteriv_reply_t *R);

xcb_generic_iterator_t
xcb_glx_get_color_table_parameteriv_data_end (const xcb_glx_get_color_table_parameteriv_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_color_table_parameteriv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_glx_get_color_table_parameteriv_reply_t *
xcb_glx_get_color_table_parameteriv_reply (xcb_connection_t                              *c,
                                           xcb_glx_get_color_table_parameteriv_cookie_t   cookie  /**< */,
                                           xcb_generic_error_t                          **e);

int
xcb_glx_get_convolution_filter_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_glx_get_convolution_filter_cookie_t
xcb_glx_get_convolution_filter (xcb_connection_t      *c,
                                xcb_glx_context_tag_t  context_tag,
                                uint32_t               target,
                                uint32_t               format,
                                uint32_t               type,
                                uint8_t                swap_bytes);

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
xcb_glx_get_convolution_filter_cookie_t
xcb_glx_get_convolution_filter_unchecked (xcb_connection_t      *c,
                                          xcb_glx_context_tag_t  context_tag,
                                          uint32_t               target,
                                          uint32_t               format,
                                          uint32_t               type,
                                          uint8_t                swap_bytes);

uint8_t *
xcb_glx_get_convolution_filter_data (const xcb_glx_get_convolution_filter_reply_t *R);

int
xcb_glx_get_convolution_filter_data_length (const xcb_glx_get_convolution_filter_reply_t *R);

xcb_generic_iterator_t
xcb_glx_get_convolution_filter_data_end (const xcb_glx_get_convolution_filter_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_convolution_filter_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_glx_get_convolution_filter_reply_t *
xcb_glx_get_convolution_filter_reply (xcb_connection_t                         *c,
                                      xcb_glx_get_convolution_filter_cookie_t   cookie  /**< */,
                                      xcb_generic_error_t                     **e);

int
xcb_glx_get_convolution_parameterfv_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_glx_get_convolution_parameterfv_cookie_t
xcb_glx_get_convolution_parameterfv (xcb_connection_t      *c,
                                     xcb_glx_context_tag_t  context_tag,
                                     uint32_t               target,
                                     uint32_t               pname);

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
xcb_glx_get_convolution_parameterfv_cookie_t
xcb_glx_get_convolution_parameterfv_unchecked (xcb_connection_t      *c,
                                               xcb_glx_context_tag_t  context_tag,
                                               uint32_t               target,
                                               uint32_t               pname);

xcb_glx_float32_t *
xcb_glx_get_convolution_parameterfv_data (const xcb_glx_get_convolution_parameterfv_reply_t *R);

int
xcb_glx_get_convolution_parameterfv_data_length (const xcb_glx_get_convolution_parameterfv_reply_t *R);

xcb_generic_iterator_t
xcb_glx_get_convolution_parameterfv_data_end (const xcb_glx_get_convolution_parameterfv_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_convolution_parameterfv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_glx_get_convolution_parameterfv_reply_t *
xcb_glx_get_convolution_parameterfv_reply (xcb_connection_t                              *c,
                                           xcb_glx_get_convolution_parameterfv_cookie_t   cookie  /**< */,
                                           xcb_generic_error_t                          **e);

int
xcb_glx_get_convolution_parameteriv_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_glx_get_convolution_parameteriv_cookie_t
xcb_glx_get_convolution_parameteriv (xcb_connection_t      *c,
                                     xcb_glx_context_tag_t  context_tag,
                                     uint32_t               target,
                                     uint32_t               pname);

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
xcb_glx_get_convolution_parameteriv_cookie_t
xcb_glx_get_convolution_parameteriv_unchecked (xcb_connection_t      *c,
                                               xcb_glx_context_tag_t  context_tag,
                                               uint32_t               target,
                                               uint32_t               pname);

int32_t *
xcb_glx_get_convolution_parameteriv_data (const xcb_glx_get_convolution_parameteriv_reply_t *R);

int
xcb_glx_get_convolution_parameteriv_data_length (const xcb_glx_get_convolution_parameteriv_reply_t *R);

xcb_generic_iterator_t
xcb_glx_get_convolution_parameteriv_data_end (const xcb_glx_get_convolution_parameteriv_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_convolution_parameteriv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_glx_get_convolution_parameteriv_reply_t *
xcb_glx_get_convolution_parameteriv_reply (xcb_connection_t                              *c,
                                           xcb_glx_get_convolution_parameteriv_cookie_t   cookie  /**< */,
                                           xcb_generic_error_t                          **e);

int
xcb_glx_get_separable_filter_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_glx_get_separable_filter_cookie_t
xcb_glx_get_separable_filter (xcb_connection_t      *c,
                              xcb_glx_context_tag_t  context_tag,
                              uint32_t               target,
                              uint32_t               format,
                              uint32_t               type,
                              uint8_t                swap_bytes);

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
xcb_glx_get_separable_filter_cookie_t
xcb_glx_get_separable_filter_unchecked (xcb_connection_t      *c,
                                        xcb_glx_context_tag_t  context_tag,
                                        uint32_t               target,
                                        uint32_t               format,
                                        uint32_t               type,
                                        uint8_t                swap_bytes);

uint8_t *
xcb_glx_get_separable_filter_rows_and_cols (const xcb_glx_get_separable_filter_reply_t *R);

int
xcb_glx_get_separable_filter_rows_and_cols_length (const xcb_glx_get_separable_filter_reply_t *R);

xcb_generic_iterator_t
xcb_glx_get_separable_filter_rows_and_cols_end (const xcb_glx_get_separable_filter_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_separable_filter_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_glx_get_separable_filter_reply_t *
xcb_glx_get_separable_filter_reply (xcb_connection_t                       *c,
                                    xcb_glx_get_separable_filter_cookie_t   cookie  /**< */,
                                    xcb_generic_error_t                   **e);

int
xcb_glx_get_histogram_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_glx_get_histogram_cookie_t
xcb_glx_get_histogram (xcb_connection_t      *c,
                       xcb_glx_context_tag_t  context_tag,
                       uint32_t               target,
                       uint32_t               format,
                       uint32_t               type,
                       uint8_t                swap_bytes,
                       uint8_t                reset);

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
xcb_glx_get_histogram_cookie_t
xcb_glx_get_histogram_unchecked (xcb_connection_t      *c,
                                 xcb_glx_context_tag_t  context_tag,
                                 uint32_t               target,
                                 uint32_t               format,
                                 uint32_t               type,
                                 uint8_t                swap_bytes,
                                 uint8_t                reset);

uint8_t *
xcb_glx_get_histogram_data (const xcb_glx_get_histogram_reply_t *R);

int
xcb_glx_get_histogram_data_length (const xcb_glx_get_histogram_reply_t *R);

xcb_generic_iterator_t
xcb_glx_get_histogram_data_end (const xcb_glx_get_histogram_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_histogram_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_glx_get_histogram_reply_t *
xcb_glx_get_histogram_reply (xcb_connection_t                *c,
                             xcb_glx_get_histogram_cookie_t   cookie  /**< */,
                             xcb_generic_error_t            **e);

int
xcb_glx_get_histogram_parameterfv_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_glx_get_histogram_parameterfv_cookie_t
xcb_glx_get_histogram_parameterfv (xcb_connection_t      *c,
                                   xcb_glx_context_tag_t  context_tag,
                                   uint32_t               target,
                                   uint32_t               pname);

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
xcb_glx_get_histogram_parameterfv_cookie_t
xcb_glx_get_histogram_parameterfv_unchecked (xcb_connection_t      *c,
                                             xcb_glx_context_tag_t  context_tag,
                                             uint32_t               target,
                                             uint32_t               pname);

xcb_glx_float32_t *
xcb_glx_get_histogram_parameterfv_data (const xcb_glx_get_histogram_parameterfv_reply_t *R);

int
xcb_glx_get_histogram_parameterfv_data_length (const xcb_glx_get_histogram_parameterfv_reply_t *R);

xcb_generic_iterator_t
xcb_glx_get_histogram_parameterfv_data_end (const xcb_glx_get_histogram_parameterfv_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_histogram_parameterfv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_glx_get_histogram_parameterfv_reply_t *
xcb_glx_get_histogram_parameterfv_reply (xcb_connection_t                            *c,
                                         xcb_glx_get_histogram_parameterfv_cookie_t   cookie  /**< */,
                                         xcb_generic_error_t                        **e);

int
xcb_glx_get_histogram_parameteriv_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_glx_get_histogram_parameteriv_cookie_t
xcb_glx_get_histogram_parameteriv (xcb_connection_t      *c,
                                   xcb_glx_context_tag_t  context_tag,
                                   uint32_t               target,
                                   uint32_t               pname);

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
xcb_glx_get_histogram_parameteriv_cookie_t
xcb_glx_get_histogram_parameteriv_unchecked (xcb_connection_t      *c,
                                             xcb_glx_context_tag_t  context_tag,
                                             uint32_t               target,
                                             uint32_t               pname);

int32_t *
xcb_glx_get_histogram_parameteriv_data (const xcb_glx_get_histogram_parameteriv_reply_t *R);

int
xcb_glx_get_histogram_parameteriv_data_length (const xcb_glx_get_histogram_parameteriv_reply_t *R);

xcb_generic_iterator_t
xcb_glx_get_histogram_parameteriv_data_end (const xcb_glx_get_histogram_parameteriv_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_histogram_parameteriv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_glx_get_histogram_parameteriv_reply_t *
xcb_glx_get_histogram_parameteriv_reply (xcb_connection_t                            *c,
                                         xcb_glx_get_histogram_parameteriv_cookie_t   cookie  /**< */,
                                         xcb_generic_error_t                        **e);

int
xcb_glx_get_minmax_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_glx_get_minmax_cookie_t
xcb_glx_get_minmax (xcb_connection_t      *c,
                    xcb_glx_context_tag_t  context_tag,
                    uint32_t               target,
                    uint32_t               format,
                    uint32_t               type,
                    uint8_t                swap_bytes,
                    uint8_t                reset);

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
xcb_glx_get_minmax_cookie_t
xcb_glx_get_minmax_unchecked (xcb_connection_t      *c,
                              xcb_glx_context_tag_t  context_tag,
                              uint32_t               target,
                              uint32_t               format,
                              uint32_t               type,
                              uint8_t                swap_bytes,
                              uint8_t                reset);

uint8_t *
xcb_glx_get_minmax_data (const xcb_glx_get_minmax_reply_t *R);

int
xcb_glx_get_minmax_data_length (const xcb_glx_get_minmax_reply_t *R);

xcb_generic_iterator_t
xcb_glx_get_minmax_data_end (const xcb_glx_get_minmax_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_minmax_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_glx_get_minmax_reply_t *
xcb_glx_get_minmax_reply (xcb_connection_t             *c,
                          xcb_glx_get_minmax_cookie_t   cookie  /**< */,
                          xcb_generic_error_t         **e);

int
xcb_glx_get_minmax_parameterfv_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_glx_get_minmax_parameterfv_cookie_t
xcb_glx_get_minmax_parameterfv (xcb_connection_t      *c,
                                xcb_glx_context_tag_t  context_tag,
                                uint32_t               target,
                                uint32_t               pname);

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
xcb_glx_get_minmax_parameterfv_cookie_t
xcb_glx_get_minmax_parameterfv_unchecked (xcb_connection_t      *c,
                                          xcb_glx_context_tag_t  context_tag,
                                          uint32_t               target,
                                          uint32_t               pname);

xcb_glx_float32_t *
xcb_glx_get_minmax_parameterfv_data (const xcb_glx_get_minmax_parameterfv_reply_t *R);

int
xcb_glx_get_minmax_parameterfv_data_length (const xcb_glx_get_minmax_parameterfv_reply_t *R);

xcb_generic_iterator_t
xcb_glx_get_minmax_parameterfv_data_end (const xcb_glx_get_minmax_parameterfv_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_minmax_parameterfv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_glx_get_minmax_parameterfv_reply_t *
xcb_glx_get_minmax_parameterfv_reply (xcb_connection_t                         *c,
                                      xcb_glx_get_minmax_parameterfv_cookie_t   cookie  /**< */,
                                      xcb_generic_error_t                     **e);

int
xcb_glx_get_minmax_parameteriv_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_glx_get_minmax_parameteriv_cookie_t
xcb_glx_get_minmax_parameteriv (xcb_connection_t      *c,
                                xcb_glx_context_tag_t  context_tag,
                                uint32_t               target,
                                uint32_t               pname);

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
xcb_glx_get_minmax_parameteriv_cookie_t
xcb_glx_get_minmax_parameteriv_unchecked (xcb_connection_t      *c,
                                          xcb_glx_context_tag_t  context_tag,
                                          uint32_t               target,
                                          uint32_t               pname);

int32_t *
xcb_glx_get_minmax_parameteriv_data (const xcb_glx_get_minmax_parameteriv_reply_t *R);

int
xcb_glx_get_minmax_parameteriv_data_length (const xcb_glx_get_minmax_parameteriv_reply_t *R);

xcb_generic_iterator_t
xcb_glx_get_minmax_parameteriv_data_end (const xcb_glx_get_minmax_parameteriv_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_minmax_parameteriv_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_glx_get_minmax_parameteriv_reply_t *
xcb_glx_get_minmax_parameteriv_reply (xcb_connection_t                         *c,
                                      xcb_glx_get_minmax_parameteriv_cookie_t   cookie  /**< */,
                                      xcb_generic_error_t                     **e);

int
xcb_glx_get_compressed_tex_image_arb_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_glx_get_compressed_tex_image_arb_cookie_t
xcb_glx_get_compressed_tex_image_arb (xcb_connection_t      *c,
                                      xcb_glx_context_tag_t  context_tag,
                                      uint32_t               target,
                                      int32_t                level);

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
xcb_glx_get_compressed_tex_image_arb_cookie_t
xcb_glx_get_compressed_tex_image_arb_unchecked (xcb_connection_t      *c,
                                                xcb_glx_context_tag_t  context_tag,
                                                uint32_t               target,
                                                int32_t                level);

uint8_t *
xcb_glx_get_compressed_tex_image_arb_data (const xcb_glx_get_compressed_tex_image_arb_reply_t *R);

int
xcb_glx_get_compressed_tex_image_arb_data_length (const xcb_glx_get_compressed_tex_image_arb_reply_t *R);

xcb_generic_iterator_t
xcb_glx_get_compressed_tex_image_arb_data_end (const xcb_glx_get_compressed_tex_image_arb_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_compressed_tex_image_arb_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_glx_get_compressed_tex_image_arb_reply_t *
xcb_glx_get_compressed_tex_image_arb_reply (xcb_connection_t                               *c,
                                            xcb_glx_get_compressed_tex_image_arb_cookie_t   cookie  /**< */,
                                            xcb_generic_error_t                           **e);

int
xcb_glx_delete_queries_arb_sizeof (const void  *_buffer);

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
xcb_glx_delete_queries_arb_checked (xcb_connection_t      *c,
                                    xcb_glx_context_tag_t  context_tag,
                                    int32_t                n,
                                    const uint32_t        *ids);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_glx_delete_queries_arb (xcb_connection_t      *c,
                            xcb_glx_context_tag_t  context_tag,
                            int32_t                n,
                            const uint32_t        *ids);

uint32_t *
xcb_glx_delete_queries_arb_ids (const xcb_glx_delete_queries_arb_request_t *R);

int
xcb_glx_delete_queries_arb_ids_length (const xcb_glx_delete_queries_arb_request_t *R);

xcb_generic_iterator_t
xcb_glx_delete_queries_arb_ids_end (const xcb_glx_delete_queries_arb_request_t *R);

int
xcb_glx_gen_queries_arb_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_glx_gen_queries_arb_cookie_t
xcb_glx_gen_queries_arb (xcb_connection_t      *c,
                         xcb_glx_context_tag_t  context_tag,
                         int32_t                n);

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
xcb_glx_gen_queries_arb_cookie_t
xcb_glx_gen_queries_arb_unchecked (xcb_connection_t      *c,
                                   xcb_glx_context_tag_t  context_tag,
                                   int32_t                n);

uint32_t *
xcb_glx_gen_queries_arb_data (const xcb_glx_gen_queries_arb_reply_t *R);

int
xcb_glx_gen_queries_arb_data_length (const xcb_glx_gen_queries_arb_reply_t *R);

xcb_generic_iterator_t
xcb_glx_gen_queries_arb_data_end (const xcb_glx_gen_queries_arb_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_gen_queries_arb_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_glx_gen_queries_arb_reply_t *
xcb_glx_gen_queries_arb_reply (xcb_connection_t                  *c,
                               xcb_glx_gen_queries_arb_cookie_t   cookie  /**< */,
                               xcb_generic_error_t              **e);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_glx_is_query_arb_cookie_t
xcb_glx_is_query_arb (xcb_connection_t      *c,
                      xcb_glx_context_tag_t  context_tag,
                      uint32_t               id);

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
xcb_glx_is_query_arb_cookie_t
xcb_glx_is_query_arb_unchecked (xcb_connection_t      *c,
                                xcb_glx_context_tag_t  context_tag,
                                uint32_t               id);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_is_query_arb_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_glx_is_query_arb_reply_t *
xcb_glx_is_query_arb_reply (xcb_connection_t               *c,
                            xcb_glx_is_query_arb_cookie_t   cookie  /**< */,
                            xcb_generic_error_t           **e);

int
xcb_glx_get_queryiv_arb_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_glx_get_queryiv_arb_cookie_t
xcb_glx_get_queryiv_arb (xcb_connection_t      *c,
                         xcb_glx_context_tag_t  context_tag,
                         uint32_t               target,
                         uint32_t               pname);

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
xcb_glx_get_queryiv_arb_cookie_t
xcb_glx_get_queryiv_arb_unchecked (xcb_connection_t      *c,
                                   xcb_glx_context_tag_t  context_tag,
                                   uint32_t               target,
                                   uint32_t               pname);

int32_t *
xcb_glx_get_queryiv_arb_data (const xcb_glx_get_queryiv_arb_reply_t *R);

int
xcb_glx_get_queryiv_arb_data_length (const xcb_glx_get_queryiv_arb_reply_t *R);

xcb_generic_iterator_t
xcb_glx_get_queryiv_arb_data_end (const xcb_glx_get_queryiv_arb_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_queryiv_arb_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_glx_get_queryiv_arb_reply_t *
xcb_glx_get_queryiv_arb_reply (xcb_connection_t                  *c,
                               xcb_glx_get_queryiv_arb_cookie_t   cookie  /**< */,
                               xcb_generic_error_t              **e);

int
xcb_glx_get_query_objectiv_arb_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_glx_get_query_objectiv_arb_cookie_t
xcb_glx_get_query_objectiv_arb (xcb_connection_t      *c,
                                xcb_glx_context_tag_t  context_tag,
                                uint32_t               id,
                                uint32_t               pname);

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
xcb_glx_get_query_objectiv_arb_cookie_t
xcb_glx_get_query_objectiv_arb_unchecked (xcb_connection_t      *c,
                                          xcb_glx_context_tag_t  context_tag,
                                          uint32_t               id,
                                          uint32_t               pname);

int32_t *
xcb_glx_get_query_objectiv_arb_data (const xcb_glx_get_query_objectiv_arb_reply_t *R);

int
xcb_glx_get_query_objectiv_arb_data_length (const xcb_glx_get_query_objectiv_arb_reply_t *R);

xcb_generic_iterator_t
xcb_glx_get_query_objectiv_arb_data_end (const xcb_glx_get_query_objectiv_arb_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_query_objectiv_arb_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_glx_get_query_objectiv_arb_reply_t *
xcb_glx_get_query_objectiv_arb_reply (xcb_connection_t                         *c,
                                      xcb_glx_get_query_objectiv_arb_cookie_t   cookie  /**< */,
                                      xcb_generic_error_t                     **e);

int
xcb_glx_get_query_objectuiv_arb_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_glx_get_query_objectuiv_arb_cookie_t
xcb_glx_get_query_objectuiv_arb (xcb_connection_t      *c,
                                 xcb_glx_context_tag_t  context_tag,
                                 uint32_t               id,
                                 uint32_t               pname);

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
xcb_glx_get_query_objectuiv_arb_cookie_t
xcb_glx_get_query_objectuiv_arb_unchecked (xcb_connection_t      *c,
                                           xcb_glx_context_tag_t  context_tag,
                                           uint32_t               id,
                                           uint32_t               pname);

uint32_t *
xcb_glx_get_query_objectuiv_arb_data (const xcb_glx_get_query_objectuiv_arb_reply_t *R);

int
xcb_glx_get_query_objectuiv_arb_data_length (const xcb_glx_get_query_objectuiv_arb_reply_t *R);

xcb_generic_iterator_t
xcb_glx_get_query_objectuiv_arb_data_end (const xcb_glx_get_query_objectuiv_arb_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_glx_get_query_objectuiv_arb_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_glx_get_query_objectuiv_arb_reply_t *
xcb_glx_get_query_objectuiv_arb_reply (xcb_connection_t                          *c,
                                       xcb_glx_get_query_objectuiv_arb_cookie_t   cookie  /**< */,
                                       xcb_generic_error_t                      **e);


#ifdef __cplusplus
}
#endif

#endif

/**
 * @}
 */
