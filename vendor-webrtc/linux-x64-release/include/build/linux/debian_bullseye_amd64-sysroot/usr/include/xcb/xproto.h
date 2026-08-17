/*
 * This file generated automatically from xproto.xml by c_client.py.
 * Edit at your peril.
 */

/**
 * @defgroup XCB__API XCB  API
 * @brief  XCB Protocol Implementation.
 * @{
 **/

#ifndef __XPROTO_H
#define __XPROTO_H

#include "xcb.h"

#ifdef __cplusplus
extern "C" {
#endif

/**
 * @brief xcb_char2b_t
 **/
typedef struct xcb_char2b_t {
    uint8_t byte1;
    uint8_t byte2;
} xcb_char2b_t;

/**
 * @brief xcb_char2b_iterator_t
 **/
typedef struct xcb_char2b_iterator_t {
    xcb_char2b_t *data;
    int           rem;
    int           index;
} xcb_char2b_iterator_t;

typedef uint32_t xcb_window_t;

/**
 * @brief xcb_window_iterator_t
 **/
typedef struct xcb_window_iterator_t {
    xcb_window_t *data;
    int           rem;
    int           index;
} xcb_window_iterator_t;

typedef uint32_t xcb_pixmap_t;

/**
 * @brief xcb_pixmap_iterator_t
 **/
typedef struct xcb_pixmap_iterator_t {
    xcb_pixmap_t *data;
    int           rem;
    int           index;
} xcb_pixmap_iterator_t;

typedef uint32_t xcb_cursor_t;

/**
 * @brief xcb_cursor_iterator_t
 **/
typedef struct xcb_cursor_iterator_t {
    xcb_cursor_t *data;
    int           rem;
    int           index;
} xcb_cursor_iterator_t;

typedef uint32_t xcb_font_t;

/**
 * @brief xcb_font_iterator_t
 **/
typedef struct xcb_font_iterator_t {
    xcb_font_t *data;
    int         rem;
    int         index;
} xcb_font_iterator_t;

typedef uint32_t xcb_gcontext_t;

/**
 * @brief xcb_gcontext_iterator_t
 **/
typedef struct xcb_gcontext_iterator_t {
    xcb_gcontext_t *data;
    int             rem;
    int             index;
} xcb_gcontext_iterator_t;

typedef uint32_t xcb_colormap_t;

/**
 * @brief xcb_colormap_iterator_t
 **/
typedef struct xcb_colormap_iterator_t {
    xcb_colormap_t *data;
    int             rem;
    int             index;
} xcb_colormap_iterator_t;

typedef uint32_t xcb_atom_t;

/**
 * @brief xcb_atom_iterator_t
 **/
typedef struct xcb_atom_iterator_t {
    xcb_atom_t *data;
    int         rem;
    int         index;
} xcb_atom_iterator_t;

typedef uint32_t xcb_drawable_t;

/**
 * @brief xcb_drawable_iterator_t
 **/
typedef struct xcb_drawable_iterator_t {
    xcb_drawable_t *data;
    int             rem;
    int             index;
} xcb_drawable_iterator_t;

typedef uint32_t xcb_fontable_t;

/**
 * @brief xcb_fontable_iterator_t
 **/
typedef struct xcb_fontable_iterator_t {
    xcb_fontable_t *data;
    int             rem;
    int             index;
} xcb_fontable_iterator_t;

typedef uint32_t xcb_bool32_t;

/**
 * @brief xcb_bool32_iterator_t
 **/
typedef struct xcb_bool32_iterator_t {
    xcb_bool32_t *data;
    int           rem;
    int           index;
} xcb_bool32_iterator_t;

typedef uint32_t xcb_visualid_t;

/**
 * @brief xcb_visualid_iterator_t
 **/
typedef struct xcb_visualid_iterator_t {
    xcb_visualid_t *data;
    int             rem;
    int             index;
} xcb_visualid_iterator_t;

typedef uint32_t xcb_timestamp_t;

/**
 * @brief xcb_timestamp_iterator_t
 **/
typedef struct xcb_timestamp_iterator_t {
    xcb_timestamp_t *data;
    int              rem;
    int              index;
} xcb_timestamp_iterator_t;

typedef uint32_t xcb_keysym_t;

/**
 * @brief xcb_keysym_iterator_t
 **/
typedef struct xcb_keysym_iterator_t {
    xcb_keysym_t *data;
    int           rem;
    int           index;
} xcb_keysym_iterator_t;

typedef uint8_t xcb_keycode_t;

/**
 * @brief xcb_keycode_iterator_t
 **/
typedef struct xcb_keycode_iterator_t {
    xcb_keycode_t *data;
    int            rem;
    int            index;
} xcb_keycode_iterator_t;

typedef uint32_t xcb_keycode32_t;

/**
 * @brief xcb_keycode32_iterator_t
 **/
typedef struct xcb_keycode32_iterator_t {
    xcb_keycode32_t *data;
    int              rem;
    int              index;
} xcb_keycode32_iterator_t;

typedef uint8_t xcb_button_t;

/**
 * @brief xcb_button_iterator_t
 **/
typedef struct xcb_button_iterator_t {
    xcb_button_t *data;
    int           rem;
    int           index;
} xcb_button_iterator_t;

/**
 * @brief xcb_point_t
 **/
typedef struct xcb_point_t {
    int16_t x;
    int16_t y;
} xcb_point_t;

/**
 * @brief xcb_point_iterator_t
 **/
typedef struct xcb_point_iterator_t {
    xcb_point_t *data;
    int          rem;
    int          index;
} xcb_point_iterator_t;

/**
 * @brief xcb_rectangle_t
 **/
typedef struct xcb_rectangle_t {
    int16_t  x;
    int16_t  y;
    uint16_t width;
    uint16_t height;
} xcb_rectangle_t;

/**
 * @brief xcb_rectangle_iterator_t
 **/
typedef struct xcb_rectangle_iterator_t {
    xcb_rectangle_t *data;
    int              rem;
    int              index;
} xcb_rectangle_iterator_t;

/**
 * @brief xcb_arc_t
 **/
typedef struct xcb_arc_t {
    int16_t  x;
    int16_t  y;
    uint16_t width;
    uint16_t height;
    int16_t  angle1;
    int16_t  angle2;
} xcb_arc_t;

/**
 * @brief xcb_arc_iterator_t
 **/
typedef struct xcb_arc_iterator_t {
    xcb_arc_t *data;
    int        rem;
    int        index;
} xcb_arc_iterator_t;

/**
 * @brief xcb_format_t
 **/
typedef struct xcb_format_t {
    uint8_t depth;
    uint8_t bits_per_pixel;
    uint8_t scanline_pad;
    uint8_t pad0[5];
} xcb_format_t;

/**
 * @brief xcb_format_iterator_t
 **/
typedef struct xcb_format_iterator_t {
    xcb_format_t *data;
    int           rem;
    int           index;
} xcb_format_iterator_t;

typedef enum xcb_visual_class_t {
    XCB_VISUAL_CLASS_STATIC_GRAY = 0,
    XCB_VISUAL_CLASS_GRAY_SCALE = 1,
    XCB_VISUAL_CLASS_STATIC_COLOR = 2,
    XCB_VISUAL_CLASS_PSEUDO_COLOR = 3,
    XCB_VISUAL_CLASS_TRUE_COLOR = 4,
    XCB_VISUAL_CLASS_DIRECT_COLOR = 5
} xcb_visual_class_t;

/**
 * @brief xcb_visualtype_t
 **/
typedef struct xcb_visualtype_t {
    xcb_visualid_t visual_id;
    uint8_t        _class;
    uint8_t        bits_per_rgb_value;
    uint16_t       colormap_entries;
    uint32_t       red_mask;
    uint32_t       green_mask;
    uint32_t       blue_mask;
    uint8_t        pad0[4];
} xcb_visualtype_t;

/**
 * @brief xcb_visualtype_iterator_t
 **/
typedef struct xcb_visualtype_iterator_t {
    xcb_visualtype_t *data;
    int               rem;
    int               index;
} xcb_visualtype_iterator_t;

/**
 * @brief xcb_depth_t
 **/
typedef struct xcb_depth_t {
    uint8_t  depth;
    uint8_t  pad0;
    uint16_t visuals_len;
    uint8_t  pad1[4];
} xcb_depth_t;

/**
 * @brief xcb_depth_iterator_t
 **/
typedef struct xcb_depth_iterator_t {
    xcb_depth_t *data;
    int          rem;
    int          index;
} xcb_depth_iterator_t;

typedef enum xcb_event_mask_t {
    XCB_EVENT_MASK_NO_EVENT = 0,
    XCB_EVENT_MASK_KEY_PRESS = 1,
    XCB_EVENT_MASK_KEY_RELEASE = 2,
    XCB_EVENT_MASK_BUTTON_PRESS = 4,
    XCB_EVENT_MASK_BUTTON_RELEASE = 8,
    XCB_EVENT_MASK_ENTER_WINDOW = 16,
    XCB_EVENT_MASK_LEAVE_WINDOW = 32,
    XCB_EVENT_MASK_POINTER_MOTION = 64,
    XCB_EVENT_MASK_POINTER_MOTION_HINT = 128,
    XCB_EVENT_MASK_BUTTON_1_MOTION = 256,
    XCB_EVENT_MASK_BUTTON_2_MOTION = 512,
    XCB_EVENT_MASK_BUTTON_3_MOTION = 1024,
    XCB_EVENT_MASK_BUTTON_4_MOTION = 2048,
    XCB_EVENT_MASK_BUTTON_5_MOTION = 4096,
    XCB_EVENT_MASK_BUTTON_MOTION = 8192,
    XCB_EVENT_MASK_KEYMAP_STATE = 16384,
    XCB_EVENT_MASK_EXPOSURE = 32768,
    XCB_EVENT_MASK_VISIBILITY_CHANGE = 65536,
    XCB_EVENT_MASK_STRUCTURE_NOTIFY = 131072,
    XCB_EVENT_MASK_RESIZE_REDIRECT = 262144,
    XCB_EVENT_MASK_SUBSTRUCTURE_NOTIFY = 524288,
    XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT = 1048576,
    XCB_EVENT_MASK_FOCUS_CHANGE = 2097152,
    XCB_EVENT_MASK_PROPERTY_CHANGE = 4194304,
    XCB_EVENT_MASK_COLOR_MAP_CHANGE = 8388608,
    XCB_EVENT_MASK_OWNER_GRAB_BUTTON = 16777216
} xcb_event_mask_t;

typedef enum xcb_backing_store_t {
    XCB_BACKING_STORE_NOT_USEFUL = 0,
    XCB_BACKING_STORE_WHEN_MAPPED = 1,
    XCB_BACKING_STORE_ALWAYS = 2
} xcb_backing_store_t;

/**
 * @brief xcb_screen_t
 **/
typedef struct xcb_screen_t {
    xcb_window_t   root;
    xcb_colormap_t default_colormap;
    uint32_t       white_pixel;
    uint32_t       black_pixel;
    uint32_t       current_input_masks;
    uint16_t       width_in_pixels;
    uint16_t       height_in_pixels;
    uint16_t       width_in_millimeters;
    uint16_t       height_in_millimeters;
    uint16_t       min_installed_maps;
    uint16_t       max_installed_maps;
    xcb_visualid_t root_visual;
    uint8_t        backing_stores;
    uint8_t        save_unders;
    uint8_t        root_depth;
    uint8_t        allowed_depths_len;
} xcb_screen_t;

/**
 * @brief xcb_screen_iterator_t
 **/
typedef struct xcb_screen_iterator_t {
    xcb_screen_t *data;
    int           rem;
    int           index;
} xcb_screen_iterator_t;

/**
 * @brief xcb_setup_request_t
 **/
typedef struct xcb_setup_request_t {
    uint8_t  byte_order;
    uint8_t  pad0;
    uint16_t protocol_major_version;
    uint16_t protocol_minor_version;
    uint16_t authorization_protocol_name_len;
    uint16_t authorization_protocol_data_len;
    uint8_t  pad1[2];
} xcb_setup_request_t;

/**
 * @brief xcb_setup_request_iterator_t
 **/
typedef struct xcb_setup_request_iterator_t {
    xcb_setup_request_t *data;
    int                  rem;
    int                  index;
} xcb_setup_request_iterator_t;

/**
 * @brief xcb_setup_failed_t
 **/
typedef struct xcb_setup_failed_t {
    uint8_t  status;
    uint8_t  reason_len;
    uint16_t protocol_major_version;
    uint16_t protocol_minor_version;
    uint16_t length;
} xcb_setup_failed_t;

/**
 * @brief xcb_setup_failed_iterator_t
 **/
typedef struct xcb_setup_failed_iterator_t {
    xcb_setup_failed_t *data;
    int                 rem;
    int                 index;
} xcb_setup_failed_iterator_t;

/**
 * @brief xcb_setup_authenticate_t
 **/
typedef struct xcb_setup_authenticate_t {
    uint8_t  status;
    uint8_t  pad0[5];
    uint16_t length;
} xcb_setup_authenticate_t;

/**
 * @brief xcb_setup_authenticate_iterator_t
 **/
typedef struct xcb_setup_authenticate_iterator_t {
    xcb_setup_authenticate_t *data;
    int                       rem;
    int                       index;
} xcb_setup_authenticate_iterator_t;

typedef enum xcb_image_order_t {
    XCB_IMAGE_ORDER_LSB_FIRST = 0,
    XCB_IMAGE_ORDER_MSB_FIRST = 1
} xcb_image_order_t;

/**
 * @brief xcb_setup_t
 **/
typedef struct xcb_setup_t {
    uint8_t       status;
    uint8_t       pad0;
    uint16_t      protocol_major_version;
    uint16_t      protocol_minor_version;
    uint16_t      length;
    uint32_t      release_number;
    uint32_t      resource_id_base;
    uint32_t      resource_id_mask;
    uint32_t      motion_buffer_size;
    uint16_t      vendor_len;
    uint16_t      maximum_request_length;
    uint8_t       roots_len;
    uint8_t       pixmap_formats_len;
    uint8_t       image_byte_order;
    uint8_t       bitmap_format_bit_order;
    uint8_t       bitmap_format_scanline_unit;
    uint8_t       bitmap_format_scanline_pad;
    xcb_keycode_t min_keycode;
    xcb_keycode_t max_keycode;
    uint8_t       pad1[4];
} xcb_setup_t;

/**
 * @brief xcb_setup_iterator_t
 **/
typedef struct xcb_setup_iterator_t {
    xcb_setup_t *data;
    int          rem;
    int          index;
} xcb_setup_iterator_t;

typedef enum xcb_mod_mask_t {
    XCB_MOD_MASK_SHIFT = 1,
    XCB_MOD_MASK_LOCK = 2,
    XCB_MOD_MASK_CONTROL = 4,
    XCB_MOD_MASK_1 = 8,
    XCB_MOD_MASK_2 = 16,
    XCB_MOD_MASK_3 = 32,
    XCB_MOD_MASK_4 = 64,
    XCB_MOD_MASK_5 = 128,
    XCB_MOD_MASK_ANY = 32768
} xcb_mod_mask_t;

typedef enum xcb_key_but_mask_t {
    XCB_KEY_BUT_MASK_SHIFT = 1,
    XCB_KEY_BUT_MASK_LOCK = 2,
    XCB_KEY_BUT_MASK_CONTROL = 4,
    XCB_KEY_BUT_MASK_MOD_1 = 8,
    XCB_KEY_BUT_MASK_MOD_2 = 16,
    XCB_KEY_BUT_MASK_MOD_3 = 32,
    XCB_KEY_BUT_MASK_MOD_4 = 64,
    XCB_KEY_BUT_MASK_MOD_5 = 128,
    XCB_KEY_BUT_MASK_BUTTON_1 = 256,
    XCB_KEY_BUT_MASK_BUTTON_2 = 512,
    XCB_KEY_BUT_MASK_BUTTON_3 = 1024,
    XCB_KEY_BUT_MASK_BUTTON_4 = 2048,
    XCB_KEY_BUT_MASK_BUTTON_5 = 4096
} xcb_key_but_mask_t;

typedef enum xcb_window_enum_t {
    XCB_WINDOW_NONE = 0
} xcb_window_enum_t;

/** Opcode for xcb_key_press. */
#define XCB_KEY_PRESS 2

/**
 * @brief xcb_key_press_event_t
 **/
typedef struct xcb_key_press_event_t {
    uint8_t         response_type;
    xcb_keycode_t   detail;
    uint16_t        sequence;
    xcb_timestamp_t time;
    xcb_window_t    root;
    xcb_window_t    event;
    xcb_window_t    child;
    int16_t         root_x;
    int16_t         root_y;
    int16_t         event_x;
    int16_t         event_y;
    uint16_t        state;
    uint8_t         same_screen;
    uint8_t         pad0;
} xcb_key_press_event_t;

/** Opcode for xcb_key_release. */
#define XCB_KEY_RELEASE 3

typedef xcb_key_press_event_t xcb_key_release_event_t;

typedef enum xcb_button_mask_t {
    XCB_BUTTON_MASK_1 = 256,
    XCB_BUTTON_MASK_2 = 512,
    XCB_BUTTON_MASK_3 = 1024,
    XCB_BUTTON_MASK_4 = 2048,
    XCB_BUTTON_MASK_5 = 4096,
    XCB_BUTTON_MASK_ANY = 32768
} xcb_button_mask_t;

/** Opcode for xcb_button_press. */
#define XCB_BUTTON_PRESS 4

/**
 * @brief xcb_button_press_event_t
 **/
typedef struct xcb_button_press_event_t {
    uint8_t         response_type;
    xcb_button_t    detail;
    uint16_t        sequence;
    xcb_timestamp_t time;
    xcb_window_t    root;
    xcb_window_t    event;
    xcb_window_t    child;
    int16_t         root_x;
    int16_t         root_y;
    int16_t         event_x;
    int16_t         event_y;
    uint16_t        state;
    uint8_t         same_screen;
    uint8_t         pad0;
} xcb_button_press_event_t;

/** Opcode for xcb_button_release. */
#define XCB_BUTTON_RELEASE 5

typedef xcb_button_press_event_t xcb_button_release_event_t;

typedef enum xcb_motion_t {
    XCB_MOTION_NORMAL = 0,
    XCB_MOTION_HINT = 1
} xcb_motion_t;

/** Opcode for xcb_motion_notify. */
#define XCB_MOTION_NOTIFY 6

/**
 * @brief xcb_motion_notify_event_t
 **/
typedef struct xcb_motion_notify_event_t {
    uint8_t         response_type;
    uint8_t         detail;
    uint16_t        sequence;
    xcb_timestamp_t time;
    xcb_window_t    root;
    xcb_window_t    event;
    xcb_window_t    child;
    int16_t         root_x;
    int16_t         root_y;
    int16_t         event_x;
    int16_t         event_y;
    uint16_t        state;
    uint8_t         same_screen;
    uint8_t         pad0;
} xcb_motion_notify_event_t;

typedef enum xcb_notify_detail_t {
    XCB_NOTIFY_DETAIL_ANCESTOR = 0,
    XCB_NOTIFY_DETAIL_VIRTUAL = 1,
    XCB_NOTIFY_DETAIL_INFERIOR = 2,
    XCB_NOTIFY_DETAIL_NONLINEAR = 3,
    XCB_NOTIFY_DETAIL_NONLINEAR_VIRTUAL = 4,
    XCB_NOTIFY_DETAIL_POINTER = 5,
    XCB_NOTIFY_DETAIL_POINTER_ROOT = 6,
    XCB_NOTIFY_DETAIL_NONE = 7
} xcb_notify_detail_t;

typedef enum xcb_notify_mode_t {
    XCB_NOTIFY_MODE_NORMAL = 0,
    XCB_NOTIFY_MODE_GRAB = 1,
    XCB_NOTIFY_MODE_UNGRAB = 2,
    XCB_NOTIFY_MODE_WHILE_GRABBED = 3
} xcb_notify_mode_t;

/** Opcode for xcb_enter_notify. */
#define XCB_ENTER_NOTIFY 7

/**
 * @brief xcb_enter_notify_event_t
 **/
typedef struct xcb_enter_notify_event_t {
    uint8_t         response_type;
    uint8_t         detail;
    uint16_t        sequence;
    xcb_timestamp_t time;
    xcb_window_t    root;
    xcb_window_t    event;
    xcb_window_t    child;
    int16_t         root_x;
    int16_t         root_y;
    int16_t         event_x;
    int16_t         event_y;
    uint16_t        state;
    uint8_t         mode;
    uint8_t         same_screen_focus;
} xcb_enter_notify_event_t;

/** Opcode for xcb_leave_notify. */
#define XCB_LEAVE_NOTIFY 8

typedef xcb_enter_notify_event_t xcb_leave_notify_event_t;

/** Opcode for xcb_focus_in. */
#define XCB_FOCUS_IN 9

/**
 * @brief xcb_focus_in_event_t
 **/
typedef struct xcb_focus_in_event_t {
    uint8_t      response_type;
    uint8_t      detail;
    uint16_t     sequence;
    xcb_window_t event;
    uint8_t      mode;
    uint8_t      pad0[3];
} xcb_focus_in_event_t;

/** Opcode for xcb_focus_out. */
#define XCB_FOCUS_OUT 10

typedef xcb_focus_in_event_t xcb_focus_out_event_t;

/** Opcode for xcb_keymap_notify. */
#define XCB_KEYMAP_NOTIFY 11

/**
 * @brief xcb_keymap_notify_event_t
 **/
typedef struct xcb_keymap_notify_event_t {
    uint8_t response_type;
    uint8_t keys[31];
} xcb_keymap_notify_event_t;

/** Opcode for xcb_expose. */
#define XCB_EXPOSE 12

/**
 * @brief xcb_expose_event_t
 **/
typedef struct xcb_expose_event_t {
    uint8_t      response_type;
    uint8_t      pad0;
    uint16_t     sequence;
    xcb_window_t window;
    uint16_t     x;
    uint16_t     y;
    uint16_t     width;
    uint16_t     height;
    uint16_t     count;
    uint8_t      pad1[2];
} xcb_expose_event_t;

/** Opcode for xcb_graphics_exposure. */
#define XCB_GRAPHICS_EXPOSURE 13

/**
 * @brief xcb_graphics_exposure_event_t
 **/
typedef struct xcb_graphics_exposure_event_t {
    uint8_t        response_type;
    uint8_t        pad0;
    uint16_t       sequence;
    xcb_drawable_t drawable;
    uint16_t       x;
    uint16_t       y;
    uint16_t       width;
    uint16_t       height;
    uint16_t       minor_opcode;
    uint16_t       count;
    uint8_t        major_opcode;
    uint8_t        pad1[3];
} xcb_graphics_exposure_event_t;

/** Opcode for xcb_no_exposure. */
#define XCB_NO_EXPOSURE 14

/**
 * @brief xcb_no_exposure_event_t
 **/
typedef struct xcb_no_exposure_event_t {
    uint8_t        response_type;
    uint8_t        pad0;
    uint16_t       sequence;
    xcb_drawable_t drawable;
    uint16_t       minor_opcode;
    uint8_t        major_opcode;
    uint8_t        pad1;
} xcb_no_exposure_event_t;

typedef enum xcb_visibility_t {
    XCB_VISIBILITY_UNOBSCURED = 0,
    XCB_VISIBILITY_PARTIALLY_OBSCURED = 1,
    XCB_VISIBILITY_FULLY_OBSCURED = 2
} xcb_visibility_t;

/** Opcode for xcb_visibility_notify. */
#define XCB_VISIBILITY_NOTIFY 15

/**
 * @brief xcb_visibility_notify_event_t
 **/
typedef struct xcb_visibility_notify_event_t {
    uint8_t      response_type;
    uint8_t      pad0;
    uint16_t     sequence;
    xcb_window_t window;
    uint8_t      state;
    uint8_t      pad1[3];
} xcb_visibility_notify_event_t;

/** Opcode for xcb_create_notify. */
#define XCB_CREATE_NOTIFY 16

/**
 * @brief xcb_create_notify_event_t
 **/
typedef struct xcb_create_notify_event_t {
    uint8_t      response_type;
    uint8_t      pad0;
    uint16_t     sequence;
    xcb_window_t parent;
    xcb_window_t window;
    int16_t      x;
    int16_t      y;
    uint16_t     width;
    uint16_t     height;
    uint16_t     border_width;
    uint8_t      override_redirect;
    uint8_t      pad1;
} xcb_create_notify_event_t;

/** Opcode for xcb_destroy_notify. */
#define XCB_DESTROY_NOTIFY 17

/**
 * @brief xcb_destroy_notify_event_t
 **/
typedef struct xcb_destroy_notify_event_t {
    uint8_t      response_type;
    uint8_t      pad0;
    uint16_t     sequence;
    xcb_window_t event;
    xcb_window_t window;
} xcb_destroy_notify_event_t;

/** Opcode for xcb_unmap_notify. */
#define XCB_UNMAP_NOTIFY 18

/**
 * @brief xcb_unmap_notify_event_t
 **/
typedef struct xcb_unmap_notify_event_t {
    uint8_t      response_type;
    uint8_t      pad0;
    uint16_t     sequence;
    xcb_window_t event;
    xcb_window_t window;
    uint8_t      from_configure;
    uint8_t      pad1[3];
} xcb_unmap_notify_event_t;

/** Opcode for xcb_map_notify. */
#define XCB_MAP_NOTIFY 19

/**
 * @brief xcb_map_notify_event_t
 **/
typedef struct xcb_map_notify_event_t {
    uint8_t      response_type;
    uint8_t      pad0;
    uint16_t     sequence;
    xcb_window_t event;
    xcb_window_t window;
    uint8_t      override_redirect;
    uint8_t      pad1[3];
} xcb_map_notify_event_t;

/** Opcode for xcb_map_request. */
#define XCB_MAP_REQUEST 20

/**
 * @brief xcb_map_request_event_t
 **/
typedef struct xcb_map_request_event_t {
    uint8_t      response_type;
    uint8_t      pad0;
    uint16_t     sequence;
    xcb_window_t parent;
    xcb_window_t window;
} xcb_map_request_event_t;

/** Opcode for xcb_reparent_notify. */
#define XCB_REPARENT_NOTIFY 21

/**
 * @brief xcb_reparent_notify_event_t
 **/
typedef struct xcb_reparent_notify_event_t {
    uint8_t      response_type;
    uint8_t      pad0;
    uint16_t     sequence;
    xcb_window_t event;
    xcb_window_t window;
    xcb_window_t parent;
    int16_t      x;
    int16_t      y;
    uint8_t      override_redirect;
    uint8_t      pad1[3];
} xcb_reparent_notify_event_t;

/** Opcode for xcb_configure_notify. */
#define XCB_CONFIGURE_NOTIFY 22

/**
 * @brief xcb_configure_notify_event_t
 **/
typedef struct xcb_configure_notify_event_t {
    uint8_t      response_type;
    uint8_t      pad0;
    uint16_t     sequence;
    xcb_window_t event;
    xcb_window_t window;
    xcb_window_t above_sibling;
    int16_t      x;
    int16_t      y;
    uint16_t     width;
    uint16_t     height;
    uint16_t     border_width;
    uint8_t      override_redirect;
    uint8_t      pad1;
} xcb_configure_notify_event_t;

/** Opcode for xcb_configure_request. */
#define XCB_CONFIGURE_REQUEST 23

/**
 * @brief xcb_configure_request_event_t
 **/
typedef struct xcb_configure_request_event_t {
    uint8_t      response_type;
    uint8_t      stack_mode;
    uint16_t     sequence;
    xcb_window_t parent;
    xcb_window_t window;
    xcb_window_t sibling;
    int16_t      x;
    int16_t      y;
    uint16_t     width;
    uint16_t     height;
    uint16_t     border_width;
    uint16_t     value_mask;
} xcb_configure_request_event_t;

/** Opcode for xcb_gravity_notify. */
#define XCB_GRAVITY_NOTIFY 24

/**
 * @brief xcb_gravity_notify_event_t
 **/
typedef struct xcb_gravity_notify_event_t {
    uint8_t      response_type;
    uint8_t      pad0;
    uint16_t     sequence;
    xcb_window_t event;
    xcb_window_t window;
    int16_t      x;
    int16_t      y;
} xcb_gravity_notify_event_t;

/** Opcode for xcb_resize_request. */
#define XCB_RESIZE_REQUEST 25

/**
 * @brief xcb_resize_request_event_t
 **/
typedef struct xcb_resize_request_event_t {
    uint8_t      response_type;
    uint8_t      pad0;
    uint16_t     sequence;
    xcb_window_t window;
    uint16_t     width;
    uint16_t     height;
} xcb_resize_request_event_t;

typedef enum xcb_place_t {
    XCB_PLACE_ON_TOP = 0,
/**< The window is now on top of all siblings. */

    XCB_PLACE_ON_BOTTOM = 1
/**< The window is now below all siblings. */

} xcb_place_t;

/** Opcode for xcb_circulate_notify. */
#define XCB_CIRCULATE_NOTIFY 26

/**
 * @brief xcb_circulate_notify_event_t
 **/
typedef struct xcb_circulate_notify_event_t {
    uint8_t      response_type;
    uint8_t      pad0;
    uint16_t     sequence;
    xcb_window_t event;
    xcb_window_t window;
    uint8_t      pad1[4];
    uint8_t      place;
    uint8_t      pad2[3];
} xcb_circulate_notify_event_t;

/** Opcode for xcb_circulate_request. */
#define XCB_CIRCULATE_REQUEST 27

typedef xcb_circulate_notify_event_t xcb_circulate_request_event_t;

typedef enum xcb_property_t {
    XCB_PROPERTY_NEW_VALUE = 0,
    XCB_PROPERTY_DELETE = 1
} xcb_property_t;

/** Opcode for xcb_property_notify. */
#define XCB_PROPERTY_NOTIFY 28

/**
 * @brief xcb_property_notify_event_t
 **/
typedef struct xcb_property_notify_event_t {
    uint8_t         response_type;
    uint8_t         pad0;
    uint16_t        sequence;
    xcb_window_t    window;
    xcb_atom_t      atom;
    xcb_timestamp_t time;
    uint8_t         state;
    uint8_t         pad1[3];
} xcb_property_notify_event_t;

/** Opcode for xcb_selection_clear. */
#define XCB_SELECTION_CLEAR 29

/**
 * @brief xcb_selection_clear_event_t
 **/
typedef struct xcb_selection_clear_event_t {
    uint8_t         response_type;
    uint8_t         pad0;
    uint16_t        sequence;
    xcb_timestamp_t time;
    xcb_window_t    owner;
    xcb_atom_t      selection;
} xcb_selection_clear_event_t;

typedef enum xcb_time_t {
    XCB_TIME_CURRENT_TIME = 0
} xcb_time_t;

typedef enum xcb_atom_enum_t {
    XCB_ATOM_NONE = 0,
    XCB_ATOM_ANY = 0,
    XCB_ATOM_PRIMARY = 1,
    XCB_ATOM_SECONDARY = 2,
    XCB_ATOM_ARC = 3,
    XCB_ATOM_ATOM = 4,
    XCB_ATOM_BITMAP = 5,
    XCB_ATOM_CARDINAL = 6,
    XCB_ATOM_COLORMAP = 7,
    XCB_ATOM_CURSOR = 8,
    XCB_ATOM_CUT_BUFFER0 = 9,
    XCB_ATOM_CUT_BUFFER1 = 10,
    XCB_ATOM_CUT_BUFFER2 = 11,
    XCB_ATOM_CUT_BUFFER3 = 12,
    XCB_ATOM_CUT_BUFFER4 = 13,
    XCB_ATOM_CUT_BUFFER5 = 14,
    XCB_ATOM_CUT_BUFFER6 = 15,
    XCB_ATOM_CUT_BUFFER7 = 16,
    XCB_ATOM_DRAWABLE = 17,
    XCB_ATOM_FONT = 18,
    XCB_ATOM_INTEGER = 19,
    XCB_ATOM_PIXMAP = 20,
    XCB_ATOM_POINT = 21,
    XCB_ATOM_RECTANGLE = 22,
    XCB_ATOM_RESOURCE_MANAGER = 23,
    XCB_ATOM_RGB_COLOR_MAP = 24,
    XCB_ATOM_RGB_BEST_MAP = 25,
    XCB_ATOM_RGB_BLUE_MAP = 26,
    XCB_ATOM_RGB_DEFAULT_MAP = 27,
    XCB_ATOM_RGB_GRAY_MAP = 28,
    XCB_ATOM_RGB_GREEN_MAP = 29,
    XCB_ATOM_RGB_RED_MAP = 30,
    XCB_ATOM_STRING = 31,
    XCB_ATOM_VISUALID = 32,
    XCB_ATOM_WINDOW = 33,
    XCB_ATOM_WM_COMMAND = 34,
    XCB_ATOM_WM_HINTS = 35,
    XCB_ATOM_WM_CLIENT_MACHINE = 36,
    XCB_ATOM_WM_ICON_NAME = 37,
    XCB_ATOM_WM_ICON_SIZE = 38,
    XCB_ATOM_WM_NAME = 39,
    XCB_ATOM_WM_NORMAL_HINTS = 40,
    XCB_ATOM_WM_SIZE_HINTS = 41,
    XCB_ATOM_WM_ZOOM_HINTS = 42,
    XCB_ATOM_MIN_SPACE = 43,
    XCB_ATOM_NORM_SPACE = 44,
    XCB_ATOM_MAX_SPACE = 45,
    XCB_ATOM_END_SPACE = 46,
    XCB_ATOM_SUPERSCRIPT_X = 47,
    XCB_ATOM_SUPERSCRIPT_Y = 48,
    XCB_ATOM_SUBSCRIPT_X = 49,
    XCB_ATOM_SUBSCRIPT_Y = 50,
    XCB_ATOM_UNDERLINE_POSITION = 51,
    XCB_ATOM_UNDERLINE_THICKNESS = 52,
    XCB_ATOM_STRIKEOUT_ASCENT = 53,
    XCB_ATOM_STRIKEOUT_DESCENT = 54,
    XCB_ATOM_ITALIC_ANGLE = 55,
    XCB_ATOM_X_HEIGHT = 56,
    XCB_ATOM_QUAD_WIDTH = 57,
    XCB_ATOM_WEIGHT = 58,
    XCB_ATOM_POINT_SIZE = 59,
    XCB_ATOM_RESOLUTION = 60,
    XCB_ATOM_COPYRIGHT = 61,
    XCB_ATOM_NOTICE = 62,
    XCB_ATOM_FONT_NAME = 63,
    XCB_ATOM_FAMILY_NAME = 64,
    XCB_ATOM_FULL_NAME = 65,
    XCB_ATOM_CAP_HEIGHT = 66,
    XCB_ATOM_WM_CLASS = 67,
    XCB_ATOM_WM_TRANSIENT_FOR = 68
} xcb_atom_enum_t;

/** Opcode for xcb_selection_request. */
#define XCB_SELECTION_REQUEST 30

/**
 * @brief xcb_selection_request_event_t
 **/
typedef struct xcb_selection_request_event_t {
    uint8_t         response_type;
    uint8_t         pad0;
    uint16_t        sequence;
    xcb_timestamp_t time;
    xcb_window_t    owner;
    xcb_window_t    requestor;
    xcb_atom_t      selection;
    xcb_atom_t      target;
    xcb_atom_t      property;
} xcb_selection_request_event_t;

/** Opcode for xcb_selection_notify. */
#define XCB_SELECTION_NOTIFY 31

/**
 * @brief xcb_selection_notify_event_t
 **/
typedef struct xcb_selection_notify_event_t {
    uint8_t         response_type;
    uint8_t         pad0;
    uint16_t        sequence;
    xcb_timestamp_t time;
    xcb_window_t    requestor;
    xcb_atom_t      selection;
    xcb_atom_t      target;
    xcb_atom_t      property;
} xcb_selection_notify_event_t;

typedef enum xcb_colormap_state_t {
    XCB_COLORMAP_STATE_UNINSTALLED = 0,
/**< The colormap was uninstalled. */

    XCB_COLORMAP_STATE_INSTALLED = 1
/**< The colormap was installed. */

} xcb_colormap_state_t;

typedef enum xcb_colormap_enum_t {
    XCB_COLORMAP_NONE = 0
} xcb_colormap_enum_t;

/** Opcode for xcb_colormap_notify. */
#define XCB_COLORMAP_NOTIFY 32

/**
 * @brief xcb_colormap_notify_event_t
 **/
typedef struct xcb_colormap_notify_event_t {
    uint8_t        response_type;
    uint8_t        pad0;
    uint16_t       sequence;
    xcb_window_t   window;
    xcb_colormap_t colormap;
    uint8_t        _new;
    uint8_t        state;
    uint8_t        pad1[2];
} xcb_colormap_notify_event_t;

/**
 * @brief xcb_client_message_data_t
 **/
typedef union xcb_client_message_data_t {
    uint8_t  data8[20];
    uint16_t data16[10];
    uint32_t data32[5];
} xcb_client_message_data_t;

/**
 * @brief xcb_client_message_data_iterator_t
 **/
typedef struct xcb_client_message_data_iterator_t {
    xcb_client_message_data_t *data;
    int                        rem;
    int                        index;
} xcb_client_message_data_iterator_t;

/** Opcode for xcb_client_message. */
#define XCB_CLIENT_MESSAGE 33

/**
 * @brief xcb_client_message_event_t
 **/
typedef struct xcb_client_message_event_t {
    uint8_t                   response_type;
    uint8_t                   format;
    uint16_t                  sequence;
    xcb_window_t              window;
    xcb_atom_t                type;
    xcb_client_message_data_t data;
} xcb_client_message_event_t;

typedef enum xcb_mapping_t {
    XCB_MAPPING_MODIFIER = 0,
    XCB_MAPPING_KEYBOARD = 1,
    XCB_MAPPING_POINTER = 2
} xcb_mapping_t;

/** Opcode for xcb_mapping_notify. */
#define XCB_MAPPING_NOTIFY 34

/**
 * @brief xcb_mapping_notify_event_t
 **/
typedef struct xcb_mapping_notify_event_t {
    uint8_t       response_type;
    uint8_t       pad0;
    uint16_t      sequence;
    uint8_t       request;
    xcb_keycode_t first_keycode;
    uint8_t       count;
    uint8_t       pad1;
} xcb_mapping_notify_event_t;

/** Opcode for xcb_ge_generic. */
#define XCB_GE_GENERIC 35

/**
 * @brief xcb_ge_generic_event_t
 **/
typedef struct xcb_ge_generic_event_t {
    uint8_t  response_type;
    uint8_t  extension;
    uint16_t sequence;
    uint32_t length;
    uint16_t event_type;
    uint8_t  pad0[22];
    uint32_t full_sequence;
} xcb_ge_generic_event_t;

/** Opcode for xcb_request. */
#define XCB_REQUEST 1

/**
 * @brief xcb_request_error_t
 **/
typedef struct xcb_request_error_t {
    uint8_t  response_type;
    uint8_t  error_code;
    uint16_t sequence;
    uint32_t bad_value;
    uint16_t minor_opcode;
    uint8_t  major_opcode;
    uint8_t  pad0;
} xcb_request_error_t;

/** Opcode for xcb_value. */
#define XCB_VALUE 2

/**
 * @brief xcb_value_error_t
 **/
typedef struct xcb_value_error_t {
    uint8_t  response_type;
    uint8_t  error_code;
    uint16_t sequence;
    uint32_t bad_value;
    uint16_t minor_opcode;
    uint8_t  major_opcode;
    uint8_t  pad0;
} xcb_value_error_t;

/** Opcode for xcb_window. */
#define XCB_WINDOW 3

typedef xcb_value_error_t xcb_window_error_t;

/** Opcode for xcb_pixmap. */
#define XCB_PIXMAP 4

typedef xcb_value_error_t xcb_pixmap_error_t;

/** Opcode for xcb_atom. */
#define XCB_ATOM 5

typedef xcb_value_error_t xcb_atom_error_t;

/** Opcode for xcb_cursor. */
#define XCB_CURSOR 6

typedef xcb_value_error_t xcb_cursor_error_t;

/** Opcode for xcb_font. */
#define XCB_FONT 7

typedef xcb_value_error_t xcb_font_error_t;

/** Opcode for xcb_match. */
#define XCB_MATCH 8

typedef xcb_request_error_t xcb_match_error_t;

/** Opcode for xcb_drawable. */
#define XCB_DRAWABLE 9

typedef xcb_value_error_t xcb_drawable_error_t;

/** Opcode for xcb_access. */
#define XCB_ACCESS 10

typedef xcb_request_error_t xcb_access_error_t;

/** Opcode for xcb_alloc. */
#define XCB_ALLOC 11

typedef xcb_request_error_t xcb_alloc_error_t;

/** Opcode for xcb_colormap. */
#define XCB_COLORMAP 12

typedef xcb_value_error_t xcb_colormap_error_t;

/** Opcode for xcb_g_context. */
#define XCB_G_CONTEXT 13

typedef xcb_value_error_t xcb_g_context_error_t;

/** Opcode for xcb_id_choice. */
#define XCB_ID_CHOICE 14

typedef xcb_value_error_t xcb_id_choice_error_t;

/** Opcode for xcb_name. */
#define XCB_NAME 15

typedef xcb_request_error_t xcb_name_error_t;

/** Opcode for xcb_length. */
#define XCB_LENGTH 16

typedef xcb_request_error_t xcb_length_error_t;

/** Opcode for xcb_implementation. */
#define XCB_IMPLEMENTATION 17

typedef xcb_request_error_t xcb_implementation_error_t;

typedef enum xcb_window_class_t {
    XCB_WINDOW_CLASS_COPY_FROM_PARENT = 0,
    XCB_WINDOW_CLASS_INPUT_OUTPUT = 1,
    XCB_WINDOW_CLASS_INPUT_ONLY = 2
} xcb_window_class_t;

typedef enum xcb_cw_t {
    XCB_CW_BACK_PIXMAP = 1,
/**< Overrides the default background-pixmap. The background pixmap and window must
have the same root and same depth. Any size pixmap can be used, although some
sizes may be faster than others.

If `XCB_BACK_PIXMAP_NONE` is specified, the window has no defined background.
The server may fill the contents with the previous screen contents or with
contents of its own choosing.

If `XCB_BACK_PIXMAP_PARENT_RELATIVE` is specified, the parent's background is
used, but the window must have the same depth as the parent (or a Match error
results).   The parent's background is tracked, and the current version is
used each time the window background is required. */

    XCB_CW_BACK_PIXEL = 2,
/**< Overrides `BackPixmap`. A pixmap of undefined size filled with the specified
background pixel is used for the background. Range-checking is not performed,
the background pixel is truncated to the appropriate number of bits. */

    XCB_CW_BORDER_PIXMAP = 4,
/**< Overrides the default border-pixmap. The border pixmap and window must have the
same root and the same depth. Any size pixmap can be used, although some sizes
may be faster than others.

The special value `XCB_COPY_FROM_PARENT` means the parent's border pixmap is
copied (subsequent changes to the parent's border attribute do not affect the
child), but the window must have the same depth as the parent. */

    XCB_CW_BORDER_PIXEL = 8,
/**< Overrides `BorderPixmap`. A pixmap of undefined size filled with the specified
border pixel is used for the border. Range checking is not performed on the
border-pixel value, it is truncated to the appropriate number of bits. */

    XCB_CW_BIT_GRAVITY = 16,
/**< Defines which region of the window should be retained if the window is resized. */

    XCB_CW_WIN_GRAVITY = 32,
/**< Defines how the window should be repositioned if the parent is resized (see
`ConfigureWindow`). */

    XCB_CW_BACKING_STORE = 64,
/**< A backing-store of `WhenMapped` advises the server that maintaining contents of
obscured regions when the window is mapped would be beneficial. A backing-store
of `Always` advises the server that maintaining contents even when the window
is unmapped would be beneficial. In this case, the server may generate an
exposure event when the window is created. A value of `NotUseful` advises the
server that maintaining contents is unnecessary, although a server may still
choose to maintain contents while the window is mapped. Note that if the server
maintains contents, then the server should maintain complete contents not just
the region within the parent boundaries, even if the window is larger than its
parent. While the server maintains contents, exposure events will not normally
be generated, but the server may stop maintaining contents at any time. */

    XCB_CW_BACKING_PLANES = 128,
/**< The backing-planes indicates (with bits set to 1) which bit planes of the
window hold dynamic data that must be preserved in backing-stores and during
save-unders. */

    XCB_CW_BACKING_PIXEL = 256,
/**< The backing-pixel specifies what value to use in planes not covered by
backing-planes. The server is free to save only the specified bit planes in the
backing-store or save-under and regenerate the remaining planes with the
specified pixel value. Any bits beyond the specified depth of the window in
these values are simply ignored. */

    XCB_CW_OVERRIDE_REDIRECT = 512,
/**< The override-redirect specifies whether map and configure requests on this
window should override a SubstructureRedirect on the parent, typically to
inform a window manager not to tamper with the window. */

    XCB_CW_SAVE_UNDER = 1024,
/**< If 1, the server is advised that when this window is mapped, saving the
contents of windows it obscures would be beneficial. */

    XCB_CW_EVENT_MASK = 2048,
/**< The event-mask defines which events the client is interested in for this window
(or for some event types, inferiors of the window). */

    XCB_CW_DONT_PROPAGATE = 4096,
/**< The do-not-propagate-mask defines which events should not be propagated to
ancestor windows when no client has the event type selected in this window. */

    XCB_CW_COLORMAP = 8192,
/**< The colormap specifies the colormap that best reflects the true colors of the window. Servers
capable of supporting multiple hardware colormaps may use this information, and window man-
agers may use it for InstallColormap requests. The colormap must have the same visual type
and root as the window (or a Match error results). If CopyFromParent is specified, the parent's
colormap is copied (subsequent changes to the parent's colormap attribute do not affect the child).
However, the window must have the same visual type as the parent (or a Match error results),
and the parent must not have a colormap of None (or a Match error results). For an explanation
of None, see FreeColormap request. The colormap is copied by sharing the colormap object
between the child and the parent, not by making a complete copy of the colormap contents. */

    XCB_CW_CURSOR = 16384
/**< If a cursor is specified, it will be used whenever the pointer is in the window. If None is speci-
fied, the parent's cursor will be used when the pointer is in the window, and any change in the
parent's cursor will cause an immediate change in the displayed cursor. */

} xcb_cw_t;

typedef enum xcb_back_pixmap_t {
    XCB_BACK_PIXMAP_NONE = 0,
    XCB_BACK_PIXMAP_PARENT_RELATIVE = 1
} xcb_back_pixmap_t;

typedef enum xcb_gravity_t {
    XCB_GRAVITY_BIT_FORGET = 0,
    XCB_GRAVITY_WIN_UNMAP = 0,
    XCB_GRAVITY_NORTH_WEST = 1,
    XCB_GRAVITY_NORTH = 2,
    XCB_GRAVITY_NORTH_EAST = 3,
    XCB_GRAVITY_WEST = 4,
    XCB_GRAVITY_CENTER = 5,
    XCB_GRAVITY_EAST = 6,
    XCB_GRAVITY_SOUTH_WEST = 7,
    XCB_GRAVITY_SOUTH = 8,
    XCB_GRAVITY_SOUTH_EAST = 9,
    XCB_GRAVITY_STATIC = 10
} xcb_gravity_t;

/**
 * @brief xcb_create_window_value_list_t
 **/
typedef struct xcb_create_window_value_list_t {
    xcb_pixmap_t   background_pixmap;
    uint32_t       background_pixel;
    xcb_pixmap_t   border_pixmap;
    uint32_t       border_pixel;
    uint32_t       bit_gravity;
    uint32_t       win_gravity;
    uint32_t       backing_store;
    uint32_t       backing_planes;
    uint32_t       backing_pixel;
    xcb_bool32_t   override_redirect;
    xcb_bool32_t   save_under;
    uint32_t       event_mask;
    uint32_t       do_not_propogate_mask;
    xcb_colormap_t colormap;
    xcb_cursor_t   cursor;
} xcb_create_window_value_list_t;

/** Opcode for xcb_create_window. */
#define XCB_CREATE_WINDOW 1

/**
 * @brief xcb_create_window_request_t
 **/
typedef struct xcb_create_window_request_t {
    uint8_t        major_opcode;
    uint8_t        depth;
    uint16_t       length;
    xcb_window_t   wid;
    xcb_window_t   parent;
    int16_t        x;
    int16_t        y;
    uint16_t       width;
    uint16_t       height;
    uint16_t       border_width;
    uint16_t       _class;
    xcb_visualid_t visual;
    uint32_t       value_mask;
} xcb_create_window_request_t;

/**
 * @brief xcb_change_window_attributes_value_list_t
 **/
typedef struct xcb_change_window_attributes_value_list_t {
    xcb_pixmap_t   background_pixmap;
    uint32_t       background_pixel;
    xcb_pixmap_t   border_pixmap;
    uint32_t       border_pixel;
    uint32_t       bit_gravity;
    uint32_t       win_gravity;
    uint32_t       backing_store;
    uint32_t       backing_planes;
    uint32_t       backing_pixel;
    xcb_bool32_t   override_redirect;
    xcb_bool32_t   save_under;
    uint32_t       event_mask;
    uint32_t       do_not_propogate_mask;
    xcb_colormap_t colormap;
    xcb_cursor_t   cursor;
} xcb_change_window_attributes_value_list_t;

/** Opcode for xcb_change_window_attributes. */
#define XCB_CHANGE_WINDOW_ATTRIBUTES 2

/**
 * @brief xcb_change_window_attributes_request_t
 **/
typedef struct xcb_change_window_attributes_request_t {
    uint8_t      major_opcode;
    uint8_t      pad0;
    uint16_t     length;
    xcb_window_t window;
    uint32_t     value_mask;
} xcb_change_window_attributes_request_t;

typedef enum xcb_map_state_t {
    XCB_MAP_STATE_UNMAPPED = 0,
    XCB_MAP_STATE_UNVIEWABLE = 1,
    XCB_MAP_STATE_VIEWABLE = 2
} xcb_map_state_t;

/**
 * @brief xcb_get_window_attributes_cookie_t
 **/
typedef struct xcb_get_window_attributes_cookie_t {
    unsigned int sequence;
} xcb_get_window_attributes_cookie_t;

/** Opcode for xcb_get_window_attributes. */
#define XCB_GET_WINDOW_ATTRIBUTES 3

/**
 * @brief xcb_get_window_attributes_request_t
 **/
typedef struct xcb_get_window_attributes_request_t {
    uint8_t      major_opcode;
    uint8_t      pad0;
    uint16_t     length;
    xcb_window_t window;
} xcb_get_window_attributes_request_t;

/**
 * @brief xcb_get_window_attributes_reply_t
 **/
typedef struct xcb_get_window_attributes_reply_t {
    uint8_t        response_type;
    uint8_t        backing_store;
    uint16_t       sequence;
    uint32_t       length;
    xcb_visualid_t visual;
    uint16_t       _class;
    uint8_t        bit_gravity;
    uint8_t        win_gravity;
    uint32_t       backing_planes;
    uint32_t       backing_pixel;
    uint8_t        save_under;
    uint8_t        map_is_installed;
    uint8_t        map_state;
    uint8_t        override_redirect;
    xcb_colormap_t colormap;
    uint32_t       all_event_masks;
    uint32_t       your_event_mask;
    uint16_t       do_not_propagate_mask;
    uint8_t        pad0[2];
} xcb_get_window_attributes_reply_t;

/** Opcode for xcb_destroy_window. */
#define XCB_DESTROY_WINDOW 4

/**
 * @brief xcb_destroy_window_request_t
 **/
typedef struct xcb_destroy_window_request_t {
    uint8_t      major_opcode;
    uint8_t      pad0;
    uint16_t     length;
    xcb_window_t window;
} xcb_destroy_window_request_t;

/** Opcode for xcb_destroy_subwindows. */
#define XCB_DESTROY_SUBWINDOWS 5

/**
 * @brief xcb_destroy_subwindows_request_t
 **/
typedef struct xcb_destroy_subwindows_request_t {
    uint8_t      major_opcode;
    uint8_t      pad0;
    uint16_t     length;
    xcb_window_t window;
} xcb_destroy_subwindows_request_t;

typedef enum xcb_set_mode_t {
    XCB_SET_MODE_INSERT = 0,
    XCB_SET_MODE_DELETE = 1
} xcb_set_mode_t;

/** Opcode for xcb_change_save_set. */
#define XCB_CHANGE_SAVE_SET 6

/**
 * @brief xcb_change_save_set_request_t
 **/
typedef struct xcb_change_save_set_request_t {
    uint8_t      major_opcode;
    uint8_t      mode;
    uint16_t     length;
    xcb_window_t window;
} xcb_change_save_set_request_t;

/** Opcode for xcb_reparent_window. */
#define XCB_REPARENT_WINDOW 7

/**
 * @brief xcb_reparent_window_request_t
 **/
typedef struct xcb_reparent_window_request_t {
    uint8_t      major_opcode;
    uint8_t      pad0;
    uint16_t     length;
    xcb_window_t window;
    xcb_window_t parent;
    int16_t      x;
    int16_t      y;
} xcb_reparent_window_request_t;

/** Opcode for xcb_map_window. */
#define XCB_MAP_WINDOW 8

/**
 * @brief xcb_map_window_request_t
 **/
typedef struct xcb_map_window_request_t {
    uint8_t      major_opcode;
    uint8_t      pad0;
    uint16_t     length;
    xcb_window_t window;
} xcb_map_window_request_t;

/** Opcode for xcb_map_subwindows. */
#define XCB_MAP_SUBWINDOWS 9

/**
 * @brief xcb_map_subwindows_request_t
 **/
typedef struct xcb_map_subwindows_request_t {
    uint8_t      major_opcode;
    uint8_t      pad0;
    uint16_t     length;
    xcb_window_t window;
} xcb_map_subwindows_request_t;

/** Opcode for xcb_unmap_window. */
#define XCB_UNMAP_WINDOW 10

/**
 * @brief xcb_unmap_window_request_t
 **/
typedef struct xcb_unmap_window_request_t {
    uint8_t      major_opcode;
    uint8_t      pad0;
    uint16_t     length;
    xcb_window_t window;
} xcb_unmap_window_request_t;

/** Opcode for xcb_unmap_subwindows. */
#define XCB_UNMAP_SUBWINDOWS 11

/**
 * @brief xcb_unmap_subwindows_request_t
 **/
typedef struct xcb_unmap_subwindows_request_t {
    uint8_t      major_opcode;
    uint8_t      pad0;
    uint16_t     length;
    xcb_window_t window;
} xcb_unmap_subwindows_request_t;

typedef enum xcb_config_window_t {
    XCB_CONFIG_WINDOW_X = 1,
    XCB_CONFIG_WINDOW_Y = 2,
    XCB_CONFIG_WINDOW_WIDTH = 4,
    XCB_CONFIG_WINDOW_HEIGHT = 8,
    XCB_CONFIG_WINDOW_BORDER_WIDTH = 16,
    XCB_CONFIG_WINDOW_SIBLING = 32,
    XCB_CONFIG_WINDOW_STACK_MODE = 64
} xcb_config_window_t;

typedef enum xcb_stack_mode_t {
    XCB_STACK_MODE_ABOVE = 0,
    XCB_STACK_MODE_BELOW = 1,
    XCB_STACK_MODE_TOP_IF = 2,
    XCB_STACK_MODE_BOTTOM_IF = 3,
    XCB_STACK_MODE_OPPOSITE = 4
} xcb_stack_mode_t;

/**
 * @brief xcb_configure_window_value_list_t
 **/
typedef struct xcb_configure_window_value_list_t {
    int32_t      x;
    int32_t      y;
    uint32_t     width;
    uint32_t     height;
    uint32_t     border_width;
    xcb_window_t sibling;
    uint32_t     stack_mode;
} xcb_configure_window_value_list_t;

/** Opcode for xcb_configure_window. */
#define XCB_CONFIGURE_WINDOW 12

/**
 * @brief xcb_configure_window_request_t
 **/
typedef struct xcb_configure_window_request_t {
    uint8_t      major_opcode;
    uint8_t      pad0;
    uint16_t     length;
    xcb_window_t window;
    uint16_t     value_mask;
    uint8_t      pad1[2];
} xcb_configure_window_request_t;

typedef enum xcb_circulate_t {
    XCB_CIRCULATE_RAISE_LOWEST = 0,
    XCB_CIRCULATE_LOWER_HIGHEST = 1
} xcb_circulate_t;

/** Opcode for xcb_circulate_window. */
#define XCB_CIRCULATE_WINDOW 13

/**
 * @brief xcb_circulate_window_request_t
 **/
typedef struct xcb_circulate_window_request_t {
    uint8_t      major_opcode;
    uint8_t      direction;
    uint16_t     length;
    xcb_window_t window;
} xcb_circulate_window_request_t;

/**
 * @brief xcb_get_geometry_cookie_t
 **/
typedef struct xcb_get_geometry_cookie_t {
    unsigned int sequence;
} xcb_get_geometry_cookie_t;

/** Opcode for xcb_get_geometry. */
#define XCB_GET_GEOMETRY 14

/**
 * @brief xcb_get_geometry_request_t
 **/
typedef struct xcb_get_geometry_request_t {
    uint8_t        major_opcode;
    uint8_t        pad0;
    uint16_t       length;
    xcb_drawable_t drawable;
} xcb_get_geometry_request_t;

/**
 * @brief xcb_get_geometry_reply_t
 **/
typedef struct xcb_get_geometry_reply_t {
    uint8_t      response_type;
    uint8_t      depth;
    uint16_t     sequence;
    uint32_t     length;
    xcb_window_t root;
    int16_t      x;
    int16_t      y;
    uint16_t     width;
    uint16_t     height;
    uint16_t     border_width;
    uint8_t      pad0[2];
} xcb_get_geometry_reply_t;

/**
 * @brief xcb_query_tree_cookie_t
 **/
typedef struct xcb_query_tree_cookie_t {
    unsigned int sequence;
} xcb_query_tree_cookie_t;

/** Opcode for xcb_query_tree. */
#define XCB_QUERY_TREE 15

/**
 * @brief xcb_query_tree_request_t
 **/
typedef struct xcb_query_tree_request_t {
    uint8_t      major_opcode;
    uint8_t      pad0;
    uint16_t     length;
    xcb_window_t window;
} xcb_query_tree_request_t;

/**
 * @brief xcb_query_tree_reply_t
 **/
typedef struct xcb_query_tree_reply_t {
    uint8_t      response_type;
    uint8_t      pad0;
    uint16_t     sequence;
    uint32_t     length;
    xcb_window_t root;
    xcb_window_t parent;
    uint16_t     children_len;
    uint8_t      pad1[14];
} xcb_query_tree_reply_t;

/**
 * @brief xcb_intern_atom_cookie_t
 **/
typedef struct xcb_intern_atom_cookie_t {
    unsigned int sequence;
} xcb_intern_atom_cookie_t;

/** Opcode for xcb_intern_atom. */
#define XCB_INTERN_ATOM 16

/**
 * @brief xcb_intern_atom_request_t
 **/
typedef struct xcb_intern_atom_request_t {
    uint8_t  major_opcode;
    uint8_t  only_if_exists;
    uint16_t length;
    uint16_t name_len;
    uint8_t  pad0[2];
} xcb_intern_atom_request_t;

/**
 * @brief xcb_intern_atom_reply_t
 **/
typedef struct xcb_intern_atom_reply_t {
    uint8_t    response_type;
    uint8_t    pad0;
    uint16_t   sequence;
    uint32_t   length;
    xcb_atom_t atom;
} xcb_intern_atom_reply_t;

/**
 * @brief xcb_get_atom_name_cookie_t
 **/
typedef struct xcb_get_atom_name_cookie_t {
    unsigned int sequence;
} xcb_get_atom_name_cookie_t;

/** Opcode for xcb_get_atom_name. */
#define XCB_GET_ATOM_NAME 17

/**
 * @brief xcb_get_atom_name_request_t
 **/
typedef struct xcb_get_atom_name_request_t {
    uint8_t    major_opcode;
    uint8_t    pad0;
    uint16_t   length;
    xcb_atom_t atom;
} xcb_get_atom_name_request_t;

/**
 * @brief xcb_get_atom_name_reply_t
 **/
typedef struct xcb_get_atom_name_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint16_t name_len;
    uint8_t  pad1[22];
} xcb_get_atom_name_reply_t;

typedef enum xcb_prop_mode_t {
    XCB_PROP_MODE_REPLACE = 0,
/**< Discard the previous property value and store the new data. */

    XCB_PROP_MODE_PREPEND = 1,
/**< Insert the new data before the beginning of existing data. The `format` must
match existing property value. If the property is undefined, it is treated as
defined with the correct type and format with zero-length data. */

    XCB_PROP_MODE_APPEND = 2
/**< Insert the new data after the beginning of existing data. The `format` must
match existing property value. If the property is undefined, it is treated as
defined with the correct type and format with zero-length data. */

} xcb_prop_mode_t;

/** Opcode for xcb_change_property. */
#define XCB_CHANGE_PROPERTY 18

/**
 * @brief xcb_change_property_request_t
 **/
typedef struct xcb_change_property_request_t {
    uint8_t      major_opcode;
    uint8_t      mode;
    uint16_t     length;
    xcb_window_t window;
    xcb_atom_t   property;
    xcb_atom_t   type;
    uint8_t      format;
    uint8_t      pad0[3];
    uint32_t     data_len;
} xcb_change_property_request_t;

/** Opcode for xcb_delete_property. */
#define XCB_DELETE_PROPERTY 19

/**
 * @brief xcb_delete_property_request_t
 **/
typedef struct xcb_delete_property_request_t {
    uint8_t      major_opcode;
    uint8_t      pad0;
    uint16_t     length;
    xcb_window_t window;
    xcb_atom_t   property;
} xcb_delete_property_request_t;

typedef enum xcb_get_property_type_t {
    XCB_GET_PROPERTY_TYPE_ANY = 0
} xcb_get_property_type_t;

/**
 * @brief xcb_get_property_cookie_t
 **/
typedef struct xcb_get_property_cookie_t {
    unsigned int sequence;
} xcb_get_property_cookie_t;

/** Opcode for xcb_get_property. */
#define XCB_GET_PROPERTY 20

/**
 * @brief xcb_get_property_request_t
 **/
typedef struct xcb_get_property_request_t {
    uint8_t      major_opcode;
    uint8_t      _delete;
    uint16_t     length;
    xcb_window_t window;
    xcb_atom_t   property;
    xcb_atom_t   type;
    uint32_t     long_offset;
    uint32_t     long_length;
} xcb_get_property_request_t;

/**
 * @brief xcb_get_property_reply_t
 **/
typedef struct xcb_get_property_reply_t {
    uint8_t    response_type;
    uint8_t    format;
    uint16_t   sequence;
    uint32_t   length;
    xcb_atom_t type;
    uint32_t   bytes_after;
    uint32_t   value_len;
    uint8_t    pad0[12];
} xcb_get_property_reply_t;

/**
 * @brief xcb_list_properties_cookie_t
 **/
typedef struct xcb_list_properties_cookie_t {
    unsigned int sequence;
} xcb_list_properties_cookie_t;

/** Opcode for xcb_list_properties. */
#define XCB_LIST_PROPERTIES 21

/**
 * @brief xcb_list_properties_request_t
 **/
typedef struct xcb_list_properties_request_t {
    uint8_t      major_opcode;
    uint8_t      pad0;
    uint16_t     length;
    xcb_window_t window;
} xcb_list_properties_request_t;

/**
 * @brief xcb_list_properties_reply_t
 **/
typedef struct xcb_list_properties_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint16_t atoms_len;
    uint8_t  pad1[22];
} xcb_list_properties_reply_t;

/** Opcode for xcb_set_selection_owner. */
#define XCB_SET_SELECTION_OWNER 22

/**
 * @brief xcb_set_selection_owner_request_t
 **/
typedef struct xcb_set_selection_owner_request_t {
    uint8_t         major_opcode;
    uint8_t         pad0;
    uint16_t        length;
    xcb_window_t    owner;
    xcb_atom_t      selection;
    xcb_timestamp_t time;
} xcb_set_selection_owner_request_t;

/**
 * @brief xcb_get_selection_owner_cookie_t
 **/
typedef struct xcb_get_selection_owner_cookie_t {
    unsigned int sequence;
} xcb_get_selection_owner_cookie_t;

/** Opcode for xcb_get_selection_owner. */
#define XCB_GET_SELECTION_OWNER 23

/**
 * @brief xcb_get_selection_owner_request_t
 **/
typedef struct xcb_get_selection_owner_request_t {
    uint8_t    major_opcode;
    uint8_t    pad0;
    uint16_t   length;
    xcb_atom_t selection;
} xcb_get_selection_owner_request_t;

/**
 * @brief xcb_get_selection_owner_reply_t
 **/
typedef struct xcb_get_selection_owner_reply_t {
    uint8_t      response_type;
    uint8_t      pad0;
    uint16_t     sequence;
    uint32_t     length;
    xcb_window_t owner;
} xcb_get_selection_owner_reply_t;

/** Opcode for xcb_convert_selection. */
#define XCB_CONVERT_SELECTION 24

/**
 * @brief xcb_convert_selection_request_t
 **/
typedef struct xcb_convert_selection_request_t {
    uint8_t         major_opcode;
    uint8_t         pad0;
    uint16_t        length;
    xcb_window_t    requestor;
    xcb_atom_t      selection;
    xcb_atom_t      target;
    xcb_atom_t      property;
    xcb_timestamp_t time;
} xcb_convert_selection_request_t;

typedef enum xcb_send_event_dest_t {
    XCB_SEND_EVENT_DEST_POINTER_WINDOW = 0,
    XCB_SEND_EVENT_DEST_ITEM_FOCUS = 1
} xcb_send_event_dest_t;

/** Opcode for xcb_send_event. */
#define XCB_SEND_EVENT 25

/**
 * @brief xcb_send_event_request_t
 **/
typedef struct xcb_send_event_request_t {
    uint8_t      major_opcode;
    uint8_t      propagate;
    uint16_t     length;
    xcb_window_t destination;
    uint32_t     event_mask;
    char         event[32];
} xcb_send_event_request_t;

typedef enum xcb_grab_mode_t {
    XCB_GRAB_MODE_SYNC = 0,
/**< The state of the keyboard appears to freeze: No further keyboard events are
generated by the server until the grabbing client issues a releasing
`AllowEvents` request or until the keyboard grab is released. */

    XCB_GRAB_MODE_ASYNC = 1
/**< Keyboard event processing continues normally. */

} xcb_grab_mode_t;

typedef enum xcb_grab_status_t {
    XCB_GRAB_STATUS_SUCCESS = 0,
    XCB_GRAB_STATUS_ALREADY_GRABBED = 1,
    XCB_GRAB_STATUS_INVALID_TIME = 2,
    XCB_GRAB_STATUS_NOT_VIEWABLE = 3,
    XCB_GRAB_STATUS_FROZEN = 4
} xcb_grab_status_t;

typedef enum xcb_cursor_enum_t {
    XCB_CURSOR_NONE = 0
} xcb_cursor_enum_t;

/**
 * @brief xcb_grab_pointer_cookie_t
 **/
typedef struct xcb_grab_pointer_cookie_t {
    unsigned int sequence;
} xcb_grab_pointer_cookie_t;

/** Opcode for xcb_grab_pointer. */
#define XCB_GRAB_POINTER 26

/**
 * @brief xcb_grab_pointer_request_t
 **/
typedef struct xcb_grab_pointer_request_t {
    uint8_t         major_opcode;
    uint8_t         owner_events;
    uint16_t        length;
    xcb_window_t    grab_window;
    uint16_t        event_mask;
    uint8_t         pointer_mode;
    uint8_t         keyboard_mode;
    xcb_window_t    confine_to;
    xcb_cursor_t    cursor;
    xcb_timestamp_t time;
} xcb_grab_pointer_request_t;

/**
 * @brief xcb_grab_pointer_reply_t
 **/
typedef struct xcb_grab_pointer_reply_t {
    uint8_t  response_type;
    uint8_t  status;
    uint16_t sequence;
    uint32_t length;
} xcb_grab_pointer_reply_t;

/** Opcode for xcb_ungrab_pointer. */
#define XCB_UNGRAB_POINTER 27

/**
 * @brief xcb_ungrab_pointer_request_t
 **/
typedef struct xcb_ungrab_pointer_request_t {
    uint8_t         major_opcode;
    uint8_t         pad0;
    uint16_t        length;
    xcb_timestamp_t time;
} xcb_ungrab_pointer_request_t;

typedef enum xcb_button_index_t {
    XCB_BUTTON_INDEX_ANY = 0,
/**< Any of the following (or none): */

    XCB_BUTTON_INDEX_1 = 1,
/**< The left mouse button. */

    XCB_BUTTON_INDEX_2 = 2,
/**< The right mouse button. */

    XCB_BUTTON_INDEX_3 = 3,
/**< The middle mouse button. */

    XCB_BUTTON_INDEX_4 = 4,
/**< Scroll wheel. TODO: direction? */

    XCB_BUTTON_INDEX_5 = 5
/**< Scroll wheel. TODO: direction? */

} xcb_button_index_t;

/** Opcode for xcb_grab_button. */
#define XCB_GRAB_BUTTON 28

/**
 * @brief xcb_grab_button_request_t
 **/
typedef struct xcb_grab_button_request_t {
    uint8_t      major_opcode;
    uint8_t      owner_events;
    uint16_t     length;
    xcb_window_t grab_window;
    uint16_t     event_mask;
    uint8_t      pointer_mode;
    uint8_t      keyboard_mode;
    xcb_window_t confine_to;
    xcb_cursor_t cursor;
    uint8_t      button;
    uint8_t      pad0;
    uint16_t     modifiers;
} xcb_grab_button_request_t;

/** Opcode for xcb_ungrab_button. */
#define XCB_UNGRAB_BUTTON 29

/**
 * @brief xcb_ungrab_button_request_t
 **/
typedef struct xcb_ungrab_button_request_t {
    uint8_t      major_opcode;
    uint8_t      button;
    uint16_t     length;
    xcb_window_t grab_window;
    uint16_t     modifiers;
    uint8_t      pad0[2];
} xcb_ungrab_button_request_t;

/** Opcode for xcb_change_active_pointer_grab. */
#define XCB_CHANGE_ACTIVE_POINTER_GRAB 30

/**
 * @brief xcb_change_active_pointer_grab_request_t
 **/
typedef struct xcb_change_active_pointer_grab_request_t {
    uint8_t         major_opcode;
    uint8_t         pad0;
    uint16_t        length;
    xcb_cursor_t    cursor;
    xcb_timestamp_t time;
    uint16_t        event_mask;
    uint8_t         pad1[2];
} xcb_change_active_pointer_grab_request_t;

/**
 * @brief xcb_grab_keyboard_cookie_t
 **/
typedef struct xcb_grab_keyboard_cookie_t {
    unsigned int sequence;
} xcb_grab_keyboard_cookie_t;

/** Opcode for xcb_grab_keyboard. */
#define XCB_GRAB_KEYBOARD 31

/**
 * @brief xcb_grab_keyboard_request_t
 **/
typedef struct xcb_grab_keyboard_request_t {
    uint8_t         major_opcode;
    uint8_t         owner_events;
    uint16_t        length;
    xcb_window_t    grab_window;
    xcb_timestamp_t time;
    uint8_t         pointer_mode;
    uint8_t         keyboard_mode;
    uint8_t         pad0[2];
} xcb_grab_keyboard_request_t;

/**
 * @brief xcb_grab_keyboard_reply_t
 **/
typedef struct xcb_grab_keyboard_reply_t {
    uint8_t  response_type;
    uint8_t  status;
    uint16_t sequence;
    uint32_t length;
} xcb_grab_keyboard_reply_t;

/** Opcode for xcb_ungrab_keyboard. */
#define XCB_UNGRAB_KEYBOARD 32

/**
 * @brief xcb_ungrab_keyboard_request_t
 **/
typedef struct xcb_ungrab_keyboard_request_t {
    uint8_t         major_opcode;
    uint8_t         pad0;
    uint16_t        length;
    xcb_timestamp_t time;
} xcb_ungrab_keyboard_request_t;

typedef enum xcb_grab_t {
    XCB_GRAB_ANY = 0
} xcb_grab_t;

/** Opcode for xcb_grab_key. */
#define XCB_GRAB_KEY 33

/**
 * @brief xcb_grab_key_request_t
 **/
typedef struct xcb_grab_key_request_t {
    uint8_t       major_opcode;
    uint8_t       owner_events;
    uint16_t      length;
    xcb_window_t  grab_window;
    uint16_t      modifiers;
    xcb_keycode_t key;
    uint8_t       pointer_mode;
    uint8_t       keyboard_mode;
    uint8_t       pad0[3];
} xcb_grab_key_request_t;

/** Opcode for xcb_ungrab_key. */
#define XCB_UNGRAB_KEY 34

/**
 * @brief xcb_ungrab_key_request_t
 **/
typedef struct xcb_ungrab_key_request_t {
    uint8_t       major_opcode;
    xcb_keycode_t key;
    uint16_t      length;
    xcb_window_t  grab_window;
    uint16_t      modifiers;
    uint8_t       pad0[2];
} xcb_ungrab_key_request_t;

typedef enum xcb_allow_t {
    XCB_ALLOW_ASYNC_POINTER = 0,
/**< For AsyncPointer, if the pointer is frozen by the client, pointer event
processing continues normally. If the pointer is frozen twice by the client on
behalf of two separate grabs, AsyncPointer thaws for both. AsyncPointer has no
effect if the pointer is not frozen by the client, but the pointer need not be
grabbed by the client.

TODO: rewrite this in more understandable terms. */

    XCB_ALLOW_SYNC_POINTER = 1,
/**< For SyncPointer, if the pointer is frozen and actively grabbed by the client,
pointer event processing continues normally until the next ButtonPress or
ButtonRelease event is reported to the client, at which time the pointer again
appears to freeze. However, if the reported event causes the pointer grab to be
released, then the pointer does not freeze. SyncPointer has no effect if the
pointer is not frozen by the client or if the pointer is not grabbed by the
client. */

    XCB_ALLOW_REPLAY_POINTER = 2,
/**< For ReplayPointer, if the pointer is actively grabbed by the client and is
frozen as the result of an event having been sent to the client (either from
the activation of a GrabButton or from a previous AllowEvents with mode
SyncPointer but not from a GrabPointer), then the pointer grab is released and
that event is completely reprocessed, this time ignoring any passive grabs at
or above (towards the root) the grab-window of the grab just released. The
request has no effect if the pointer is not grabbed by the client or if the
pointer is not frozen as the result of an event. */

    XCB_ALLOW_ASYNC_KEYBOARD = 3,
/**< For AsyncKeyboard, if the keyboard is frozen by the client, keyboard event
processing continues normally. If the keyboard is frozen twice by the client on
behalf of two separate grabs, AsyncKeyboard thaws for both. AsyncKeyboard has
no effect if the keyboard is not frozen by the client, but the keyboard need
not be grabbed by the client. */

    XCB_ALLOW_SYNC_KEYBOARD = 4,
/**< For SyncKeyboard, if the keyboard is frozen and actively grabbed by the client,
keyboard event processing continues normally until the next KeyPress or
KeyRelease event is reported to the client, at which time the keyboard again
appears to freeze. However, if the reported event causes the keyboard grab to
be released, then the keyboard does not freeze. SyncKeyboard has no effect if
the keyboard is not frozen by the client or if the keyboard is not grabbed by
the client. */

    XCB_ALLOW_REPLAY_KEYBOARD = 5,
/**< For ReplayKeyboard, if the keyboard is actively grabbed by the client and is
frozen as the result of an event having been sent to the client (either from
the activation of a GrabKey or from a previous AllowEvents with mode
SyncKeyboard but not from a GrabKeyboard), then the keyboard grab is released
and that event is completely reprocessed, this time ignoring any passive grabs
at or above (towards the root) the grab-window of the grab just released. The
request has no effect if the keyboard is not grabbed by the client or if the
keyboard is not frozen as the result of an event. */

    XCB_ALLOW_ASYNC_BOTH = 6,
/**< For AsyncBoth, if the pointer and the keyboard are frozen by the client, event
processing for both devices continues normally. If a device is frozen twice by
the client on behalf of two separate grabs, AsyncBoth thaws for both. AsyncBoth
has no effect unless both pointer and keyboard are frozen by the client. */

    XCB_ALLOW_SYNC_BOTH = 7
/**< For SyncBoth, if both pointer and keyboard are frozen by the client, event
processing (for both devices) continues normally until the next ButtonPress,
ButtonRelease, KeyPress, or KeyRelease event is reported to the client for a
grabbed device (button event for the pointer, key event for the keyboard), at
which time the devices again appear to freeze. However, if the reported event
causes the grab to be released, then the devices do not freeze (but if the
other device is still grabbed, then a subsequent event for it will still cause
both devices to freeze). SyncBoth has no effect unless both pointer and
keyboard are frozen by the client. If the pointer or keyboard is frozen twice
by the client on behalf of two separate grabs, SyncBoth thaws for both (but a
subsequent freeze for SyncBoth will only freeze each device once). */

} xcb_allow_t;

/** Opcode for xcb_allow_events. */
#define XCB_ALLOW_EVENTS 35

/**
 * @brief xcb_allow_events_request_t
 **/
typedef struct xcb_allow_events_request_t {
    uint8_t         major_opcode;
    uint8_t         mode;
    uint16_t        length;
    xcb_timestamp_t time;
} xcb_allow_events_request_t;

/** Opcode for xcb_grab_server. */
#define XCB_GRAB_SERVER 36

/**
 * @brief xcb_grab_server_request_t
 **/
typedef struct xcb_grab_server_request_t {
    uint8_t  major_opcode;
    uint8_t  pad0;
    uint16_t length;
} xcb_grab_server_request_t;

/** Opcode for xcb_ungrab_server. */
#define XCB_UNGRAB_SERVER 37

/**
 * @brief xcb_ungrab_server_request_t
 **/
typedef struct xcb_ungrab_server_request_t {
    uint8_t  major_opcode;
    uint8_t  pad0;
    uint16_t length;
} xcb_ungrab_server_request_t;

/**
 * @brief xcb_query_pointer_cookie_t
 **/
typedef struct xcb_query_pointer_cookie_t {
    unsigned int sequence;
} xcb_query_pointer_cookie_t;

/** Opcode for xcb_query_pointer. */
#define XCB_QUERY_POINTER 38

/**
 * @brief xcb_query_pointer_request_t
 **/
typedef struct xcb_query_pointer_request_t {
    uint8_t      major_opcode;
    uint8_t      pad0;
    uint16_t     length;
    xcb_window_t window;
} xcb_query_pointer_request_t;

/**
 * @brief xcb_query_pointer_reply_t
 **/
typedef struct xcb_query_pointer_reply_t {
    uint8_t      response_type;
    uint8_t      same_screen;
    uint16_t     sequence;
    uint32_t     length;
    xcb_window_t root;
    xcb_window_t child;
    int16_t      root_x;
    int16_t      root_y;
    int16_t      win_x;
    int16_t      win_y;
    uint16_t     mask;
    uint8_t      pad0[2];
} xcb_query_pointer_reply_t;

/**
 * @brief xcb_timecoord_t
 **/
typedef struct xcb_timecoord_t {
    xcb_timestamp_t time;
    int16_t         x;
    int16_t         y;
} xcb_timecoord_t;

/**
 * @brief xcb_timecoord_iterator_t
 **/
typedef struct xcb_timecoord_iterator_t {
    xcb_timecoord_t *data;
    int              rem;
    int              index;
} xcb_timecoord_iterator_t;

/**
 * @brief xcb_get_motion_events_cookie_t
 **/
typedef struct xcb_get_motion_events_cookie_t {
    unsigned int sequence;
} xcb_get_motion_events_cookie_t;

/** Opcode for xcb_get_motion_events. */
#define XCB_GET_MOTION_EVENTS 39

/**
 * @brief xcb_get_motion_events_request_t
 **/
typedef struct xcb_get_motion_events_request_t {
    uint8_t         major_opcode;
    uint8_t         pad0;
    uint16_t        length;
    xcb_window_t    window;
    xcb_timestamp_t start;
    xcb_timestamp_t stop;
} xcb_get_motion_events_request_t;

/**
 * @brief xcb_get_motion_events_reply_t
 **/
typedef struct xcb_get_motion_events_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint32_t events_len;
    uint8_t  pad1[20];
} xcb_get_motion_events_reply_t;

/**
 * @brief xcb_translate_coordinates_cookie_t
 **/
typedef struct xcb_translate_coordinates_cookie_t {
    unsigned int sequence;
} xcb_translate_coordinates_cookie_t;

/** Opcode for xcb_translate_coordinates. */
#define XCB_TRANSLATE_COORDINATES 40

/**
 * @brief xcb_translate_coordinates_request_t
 **/
typedef struct xcb_translate_coordinates_request_t {
    uint8_t      major_opcode;
    uint8_t      pad0;
    uint16_t     length;
    xcb_window_t src_window;
    xcb_window_t dst_window;
    int16_t      src_x;
    int16_t      src_y;
} xcb_translate_coordinates_request_t;

/**
 * @brief xcb_translate_coordinates_reply_t
 **/
typedef struct xcb_translate_coordinates_reply_t {
    uint8_t      response_type;
    uint8_t      same_screen;
    uint16_t     sequence;
    uint32_t     length;
    xcb_window_t child;
    int16_t      dst_x;
    int16_t      dst_y;
} xcb_translate_coordinates_reply_t;

/** Opcode for xcb_warp_pointer. */
#define XCB_WARP_POINTER 41

/**
 * @brief xcb_warp_pointer_request_t
 **/
typedef struct xcb_warp_pointer_request_t {
    uint8_t      major_opcode;
    uint8_t      pad0;
    uint16_t     length;
    xcb_window_t src_window;
    xcb_window_t dst_window;
    int16_t      src_x;
    int16_t      src_y;
    uint16_t     src_width;
    uint16_t     src_height;
    int16_t      dst_x;
    int16_t      dst_y;
} xcb_warp_pointer_request_t;

typedef enum xcb_input_focus_t {
    XCB_INPUT_FOCUS_NONE = 0,
/**< The focus reverts to `XCB_NONE`, so no window will have the input focus. */

    XCB_INPUT_FOCUS_POINTER_ROOT = 1,
/**< The focus reverts to `XCB_POINTER_ROOT` respectively. When the focus reverts,
FocusIn and FocusOut events are generated, but the last-focus-change time is
not changed. */

    XCB_INPUT_FOCUS_PARENT = 2,
/**< The focus reverts to the parent (or closest viewable ancestor) and the new
revert_to value is `XCB_INPUT_FOCUS_NONE`. */

    XCB_INPUT_FOCUS_FOLLOW_KEYBOARD = 3
/**< NOT YET DOCUMENTED. Only relevant for the xinput extension. */

} xcb_input_focus_t;

/** Opcode for xcb_set_input_focus. */
#define XCB_SET_INPUT_FOCUS 42

/**
 * @brief xcb_set_input_focus_request_t
 **/
typedef struct xcb_set_input_focus_request_t {
    uint8_t         major_opcode;
    uint8_t         revert_to;
    uint16_t        length;
    xcb_window_t    focus;
    xcb_timestamp_t time;
} xcb_set_input_focus_request_t;

/**
 * @brief xcb_get_input_focus_cookie_t
 **/
typedef struct xcb_get_input_focus_cookie_t {
    unsigned int sequence;
} xcb_get_input_focus_cookie_t;

/** Opcode for xcb_get_input_focus. */
#define XCB_GET_INPUT_FOCUS 43

/**
 * @brief xcb_get_input_focus_request_t
 **/
typedef struct xcb_get_input_focus_request_t {
    uint8_t  major_opcode;
    uint8_t  pad0;
    uint16_t length;
} xcb_get_input_focus_request_t;

/**
 * @brief xcb_get_input_focus_reply_t
 **/
typedef struct xcb_get_input_focus_reply_t {
    uint8_t      response_type;
    uint8_t      revert_to;
    uint16_t     sequence;
    uint32_t     length;
    xcb_window_t focus;
} xcb_get_input_focus_reply_t;

/**
 * @brief xcb_query_keymap_cookie_t
 **/
typedef struct xcb_query_keymap_cookie_t {
    unsigned int sequence;
} xcb_query_keymap_cookie_t;

/** Opcode for xcb_query_keymap. */
#define XCB_QUERY_KEYMAP 44

/**
 * @brief xcb_query_keymap_request_t
 **/
typedef struct xcb_query_keymap_request_t {
    uint8_t  major_opcode;
    uint8_t  pad0;
    uint16_t length;
} xcb_query_keymap_request_t;

/**
 * @brief xcb_query_keymap_reply_t
 **/
typedef struct xcb_query_keymap_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint8_t  keys[32];
} xcb_query_keymap_reply_t;

/** Opcode for xcb_open_font. */
#define XCB_OPEN_FONT 45

/**
 * @brief xcb_open_font_request_t
 **/
typedef struct xcb_open_font_request_t {
    uint8_t    major_opcode;
    uint8_t    pad0;
    uint16_t   length;
    xcb_font_t fid;
    uint16_t   name_len;
    uint8_t    pad1[2];
} xcb_open_font_request_t;

/** Opcode for xcb_close_font. */
#define XCB_CLOSE_FONT 46

/**
 * @brief xcb_close_font_request_t
 **/
typedef struct xcb_close_font_request_t {
    uint8_t    major_opcode;
    uint8_t    pad0;
    uint16_t   length;
    xcb_font_t font;
} xcb_close_font_request_t;

typedef enum xcb_font_draw_t {
    XCB_FONT_DRAW_LEFT_TO_RIGHT = 0,
    XCB_FONT_DRAW_RIGHT_TO_LEFT = 1
} xcb_font_draw_t;

/**
 * @brief xcb_fontprop_t
 **/
typedef struct xcb_fontprop_t {
    xcb_atom_t name;
    uint32_t   value;
} xcb_fontprop_t;

/**
 * @brief xcb_fontprop_iterator_t
 **/
typedef struct xcb_fontprop_iterator_t {
    xcb_fontprop_t *data;
    int             rem;
    int             index;
} xcb_fontprop_iterator_t;

/**
 * @brief xcb_charinfo_t
 **/
typedef struct xcb_charinfo_t {
    int16_t  left_side_bearing;
    int16_t  right_side_bearing;
    int16_t  character_width;
    int16_t  ascent;
    int16_t  descent;
    uint16_t attributes;
} xcb_charinfo_t;

/**
 * @brief xcb_charinfo_iterator_t
 **/
typedef struct xcb_charinfo_iterator_t {
    xcb_charinfo_t *data;
    int             rem;
    int             index;
} xcb_charinfo_iterator_t;

/**
 * @brief xcb_query_font_cookie_t
 **/
typedef struct xcb_query_font_cookie_t {
    unsigned int sequence;
} xcb_query_font_cookie_t;

/** Opcode for xcb_query_font. */
#define XCB_QUERY_FONT 47

/**
 * @brief xcb_query_font_request_t
 **/
typedef struct xcb_query_font_request_t {
    uint8_t        major_opcode;
    uint8_t        pad0;
    uint16_t       length;
    xcb_fontable_t font;
} xcb_query_font_request_t;

/**
 * @brief xcb_query_font_reply_t
 **/
typedef struct xcb_query_font_reply_t {
    uint8_t        response_type;
    uint8_t        pad0;
    uint16_t       sequence;
    uint32_t       length;
    xcb_charinfo_t min_bounds;
    uint8_t        pad1[4];
    xcb_charinfo_t max_bounds;
    uint8_t        pad2[4];
    uint16_t       min_char_or_byte2;
    uint16_t       max_char_or_byte2;
    uint16_t       default_char;
    uint16_t       properties_len;
    uint8_t        draw_direction;
    uint8_t        min_byte1;
    uint8_t        max_byte1;
    uint8_t        all_chars_exist;
    int16_t        font_ascent;
    int16_t        font_descent;
    uint32_t       char_infos_len;
} xcb_query_font_reply_t;

/**
 * @brief xcb_query_text_extents_cookie_t
 **/
typedef struct xcb_query_text_extents_cookie_t {
    unsigned int sequence;
} xcb_query_text_extents_cookie_t;

/** Opcode for xcb_query_text_extents. */
#define XCB_QUERY_TEXT_EXTENTS 48

/**
 * @brief xcb_query_text_extents_request_t
 **/
typedef struct xcb_query_text_extents_request_t {
    uint8_t        major_opcode;
    uint8_t        odd_length;
    uint16_t       length;
    xcb_fontable_t font;
} xcb_query_text_extents_request_t;

/**
 * @brief xcb_query_text_extents_reply_t
 **/
typedef struct xcb_query_text_extents_reply_t {
    uint8_t  response_type;
    uint8_t  draw_direction;
    uint16_t sequence;
    uint32_t length;
    int16_t  font_ascent;
    int16_t  font_descent;
    int16_t  overall_ascent;
    int16_t  overall_descent;
    int32_t  overall_width;
    int32_t  overall_left;
    int32_t  overall_right;
} xcb_query_text_extents_reply_t;

/**
 * @brief xcb_str_t
 **/
typedef struct xcb_str_t {
    uint8_t name_len;
} xcb_str_t;

/**
 * @brief xcb_str_iterator_t
 **/
typedef struct xcb_str_iterator_t {
    xcb_str_t *data;
    int        rem;
    int        index;
} xcb_str_iterator_t;

/**
 * @brief xcb_list_fonts_cookie_t
 **/
typedef struct xcb_list_fonts_cookie_t {
    unsigned int sequence;
} xcb_list_fonts_cookie_t;

/** Opcode for xcb_list_fonts. */
#define XCB_LIST_FONTS 49

/**
 * @brief xcb_list_fonts_request_t
 **/
typedef struct xcb_list_fonts_request_t {
    uint8_t  major_opcode;
    uint8_t  pad0;
    uint16_t length;
    uint16_t max_names;
    uint16_t pattern_len;
} xcb_list_fonts_request_t;

/**
 * @brief xcb_list_fonts_reply_t
 **/
typedef struct xcb_list_fonts_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint16_t names_len;
    uint8_t  pad1[22];
} xcb_list_fonts_reply_t;

/**
 * @brief xcb_list_fonts_with_info_cookie_t
 **/
typedef struct xcb_list_fonts_with_info_cookie_t {
    unsigned int sequence;
} xcb_list_fonts_with_info_cookie_t;

/** Opcode for xcb_list_fonts_with_info. */
#define XCB_LIST_FONTS_WITH_INFO 50

/**
 * @brief xcb_list_fonts_with_info_request_t
 **/
typedef struct xcb_list_fonts_with_info_request_t {
    uint8_t  major_opcode;
    uint8_t  pad0;
    uint16_t length;
    uint16_t max_names;
    uint16_t pattern_len;
} xcb_list_fonts_with_info_request_t;

/**
 * @brief xcb_list_fonts_with_info_reply_t
 **/
typedef struct xcb_list_fonts_with_info_reply_t {
    uint8_t        response_type;
    uint8_t        name_len;
    uint16_t       sequence;
    uint32_t       length;
    xcb_charinfo_t min_bounds;
    uint8_t        pad0[4];
    xcb_charinfo_t max_bounds;
    uint8_t        pad1[4];
    uint16_t       min_char_or_byte2;
    uint16_t       max_char_or_byte2;
    uint16_t       default_char;
    uint16_t       properties_len;
    uint8_t        draw_direction;
    uint8_t        min_byte1;
    uint8_t        max_byte1;
    uint8_t        all_chars_exist;
    int16_t        font_ascent;
    int16_t        font_descent;
    uint32_t       replies_hint;
} xcb_list_fonts_with_info_reply_t;

/** Opcode for xcb_set_font_path. */
#define XCB_SET_FONT_PATH 51

/**
 * @brief xcb_set_font_path_request_t
 **/
typedef struct xcb_set_font_path_request_t {
    uint8_t  major_opcode;
    uint8_t  pad0;
    uint16_t length;
    uint16_t font_qty;
    uint8_t  pad1[2];
} xcb_set_font_path_request_t;

/**
 * @brief xcb_get_font_path_cookie_t
 **/
typedef struct xcb_get_font_path_cookie_t {
    unsigned int sequence;
} xcb_get_font_path_cookie_t;

/** Opcode for xcb_get_font_path. */
#define XCB_GET_FONT_PATH 52

/**
 * @brief xcb_get_font_path_request_t
 **/
typedef struct xcb_get_font_path_request_t {
    uint8_t  major_opcode;
    uint8_t  pad0;
    uint16_t length;
} xcb_get_font_path_request_t;

/**
 * @brief xcb_get_font_path_reply_t
 **/
typedef struct xcb_get_font_path_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint16_t path_len;
    uint8_t  pad1[22];
} xcb_get_font_path_reply_t;

/** Opcode for xcb_create_pixmap. */
#define XCB_CREATE_PIXMAP 53

/**
 * @brief xcb_create_pixmap_request_t
 **/
typedef struct xcb_create_pixmap_request_t {
    uint8_t        major_opcode;
    uint8_t        depth;
    uint16_t       length;
    xcb_pixmap_t   pid;
    xcb_drawable_t drawable;
    uint16_t       width;
    uint16_t       height;
} xcb_create_pixmap_request_t;

/** Opcode for xcb_free_pixmap. */
#define XCB_FREE_PIXMAP 54

/**
 * @brief xcb_free_pixmap_request_t
 **/
typedef struct xcb_free_pixmap_request_t {
    uint8_t      major_opcode;
    uint8_t      pad0;
    uint16_t     length;
    xcb_pixmap_t pixmap;
} xcb_free_pixmap_request_t;

typedef enum xcb_gc_t {
    XCB_GC_FUNCTION = 1,
/**< TODO: Refer to GX */

    XCB_GC_PLANE_MASK = 2,
/**< In graphics operations, given a source and destination pixel, the result is
computed bitwise on corresponding bits of the pixels; that is, a Boolean
operation is performed in each bit plane. The plane-mask restricts the
operation to a subset of planes, so the result is:

        ((src FUNC dst) AND plane-mask) OR (dst AND (NOT plane-mask)) */

    XCB_GC_FOREGROUND = 4,
/**< Foreground colorpixel. */

    XCB_GC_BACKGROUND = 8,
/**< Background colorpixel. */

    XCB_GC_LINE_WIDTH = 16,
/**< The line-width is measured in pixels and can be greater than or equal to one, a wide line, or the
special value zero, a thin line. */

    XCB_GC_LINE_STYLE = 32,
/**< The line-style defines which sections of a line are drawn:
Solid                The full path of the line is drawn.
DoubleDash           The full path of the line is drawn, but the even dashes are filled differently
                     than the odd dashes (see fill-style), with Butt cap-style used where even and
                     odd dashes meet.
OnOffDash            Only the even dashes are drawn, and cap-style applies to all internal ends of
                     the individual dashes (except NotLast is treated as Butt). */

    XCB_GC_CAP_STYLE = 64,
/**< The cap-style defines how the endpoints of a path are drawn:
NotLast    The result is equivalent to Butt, except that for a line-width of zero the final
           endpoint is not drawn.
Butt       The result is square at the endpoint (perpendicular to the slope of the line)
           with no projection beyond.
Round      The result is a circular arc with its diameter equal to the line-width, centered
           on the endpoint; it is equivalent to Butt for line-width zero.
Projecting The result is square at the end, but the path continues beyond the endpoint for
           a distance equal to half the line-width; it is equivalent to Butt for line-width
           zero. */

    XCB_GC_JOIN_STYLE = 128,
/**< The join-style defines how corners are drawn for wide lines:
Miter               The outer edges of the two lines extend to meet at an angle. However, if the
                    angle is less than 11 degrees, a Bevel join-style is used instead.
Round               The result is a circular arc with a diameter equal to the line-width, centered
                    on the joinpoint.
Bevel               The result is Butt endpoint styles, and then the triangular notch is filled. */

    XCB_GC_FILL_STYLE = 256,
/**< The fill-style defines the contents of the source for line, text, and fill requests. For all text and fill
requests (for example, PolyText8, PolyText16, PolyFillRectangle, FillPoly, and PolyFillArc)
as well as for line requests with line-style Solid, (for example, PolyLine, PolySegment,
PolyRectangle, PolyArc) and for the even dashes for line requests with line-style OnOffDash
or DoubleDash:
Solid                     Foreground
Tiled                     Tile
OpaqueStippled            A tile with the same width and height as stipple but with background
                          everywhere stipple has a zero and with foreground everywhere stipple
                          has a one
Stippled                  Foreground masked by stipple
For the odd dashes for line requests with line-style DoubleDash:
Solid                     Background
Tiled                     Same as for even dashes
OpaqueStippled            Same as for even dashes
Stippled                  Background masked by stipple */

    XCB_GC_FILL_RULE = 512,
/**<  */

    XCB_GC_TILE = 1024,
/**< The tile/stipple represents an infinite two-dimensional plane with the tile/stipple replicated in all
dimensions. When that plane is superimposed on the drawable for use in a graphics operation,
the upper-left corner of some instance of the tile/stipple is at the coordinates within the drawable
specified by the tile/stipple origin. The tile/stipple and clip origins are interpreted relative to the
origin of whatever destination drawable is specified in a graphics request.
The tile pixmap must have the same root and depth as the gcontext (or a Match error results).
The stipple pixmap must have depth one and must have the same root as the gcontext (or a
Match error results). For fill-style Stippled (but not fill-style
OpaqueStippled), the stipple pattern is tiled in a single plane and acts as an
additional clip mask to be ANDed with the clip-mask.
Any size pixmap can be used for tiling or stippling, although some sizes may be faster to use than
others. */

    XCB_GC_STIPPLE = 2048,
/**< The tile/stipple represents an infinite two-dimensional plane with the tile/stipple replicated in all
dimensions. When that plane is superimposed on the drawable for use in a graphics operation,
the upper-left corner of some instance of the tile/stipple is at the coordinates within the drawable
specified by the tile/stipple origin. The tile/stipple and clip origins are interpreted relative to the
origin of whatever destination drawable is specified in a graphics request.
The tile pixmap must have the same root and depth as the gcontext (or a Match error results).
The stipple pixmap must have depth one and must have the same root as the gcontext (or a
Match error results). For fill-style Stippled (but not fill-style
OpaqueStippled), the stipple pattern is tiled in a single plane and acts as an
additional clip mask to be ANDed with the clip-mask.
Any size pixmap can be used for tiling or stippling, although some sizes may be faster to use than
others. */

    XCB_GC_TILE_STIPPLE_ORIGIN_X = 4096,
/**< TODO */

    XCB_GC_TILE_STIPPLE_ORIGIN_Y = 8192,
/**< TODO */

    XCB_GC_FONT = 16384,
/**< Which font to use for the `ImageText8` and `ImageText16` requests. */

    XCB_GC_SUBWINDOW_MODE = 32768,
/**< For ClipByChildren, both source and destination windows are additionally
clipped by all viewable InputOutput children. For IncludeInferiors, neither
source nor destination window is
clipped by inferiors. This will result in including subwindow contents in the source and drawing
through subwindow boundaries of the destination. The use of IncludeInferiors with a source or
destination window of one depth with mapped inferiors of differing depth is not illegal, but the
semantics is undefined by the core protocol. */

    XCB_GC_GRAPHICS_EXPOSURES = 65536,
/**< Whether ExposureEvents should be generated (1) or not (0).

The default is 1. */

    XCB_GC_CLIP_ORIGIN_X = 131072,
/**< TODO */

    XCB_GC_CLIP_ORIGIN_Y = 262144,
/**< TODO */

    XCB_GC_CLIP_MASK = 524288,
/**< The clip-mask restricts writes to the destination drawable. Only pixels where the clip-mask has
bits set to 1 are drawn. Pixels are not drawn outside the area covered by the clip-mask or where
the clip-mask has bits set to 0. The clip-mask affects all graphics requests, but it does not clip
sources. The clip-mask origin is interpreted relative to the origin of whatever destination drawable is specified in a graphics request. If a pixmap is specified as the clip-mask, it must have
depth 1 and have the same root as the gcontext (or a Match error results). If clip-mask is None,
then pixels are always drawn, regardless of the clip origin. The clip-mask can also be set with the
SetClipRectangles request. */

    XCB_GC_DASH_OFFSET = 1048576,
/**< TODO */

    XCB_GC_DASH_LIST = 2097152,
/**< TODO */

    XCB_GC_ARC_MODE = 4194304
/**< TODO */

} xcb_gc_t;

typedef enum xcb_gx_t {
    XCB_GX_CLEAR = 0,
    XCB_GX_AND = 1,
    XCB_GX_AND_REVERSE = 2,
    XCB_GX_COPY = 3,
    XCB_GX_AND_INVERTED = 4,
    XCB_GX_NOOP = 5,
    XCB_GX_XOR = 6,
    XCB_GX_OR = 7,
    XCB_GX_NOR = 8,
    XCB_GX_EQUIV = 9,
    XCB_GX_INVERT = 10,
    XCB_GX_OR_REVERSE = 11,
    XCB_GX_COPY_INVERTED = 12,
    XCB_GX_OR_INVERTED = 13,
    XCB_GX_NAND = 14,
    XCB_GX_SET = 15
} xcb_gx_t;

typedef enum xcb_line_style_t {
    XCB_LINE_STYLE_SOLID = 0,
    XCB_LINE_STYLE_ON_OFF_DASH = 1,
    XCB_LINE_STYLE_DOUBLE_DASH = 2
} xcb_line_style_t;

typedef enum xcb_cap_style_t {
    XCB_CAP_STYLE_NOT_LAST = 0,
    XCB_CAP_STYLE_BUTT = 1,
    XCB_CAP_STYLE_ROUND = 2,
    XCB_CAP_STYLE_PROJECTING = 3
} xcb_cap_style_t;

typedef enum xcb_join_style_t {
    XCB_JOIN_STYLE_MITER = 0,
    XCB_JOIN_STYLE_ROUND = 1,
    XCB_JOIN_STYLE_BEVEL = 2
} xcb_join_style_t;

typedef enum xcb_fill_style_t {
    XCB_FILL_STYLE_SOLID = 0,
    XCB_FILL_STYLE_TILED = 1,
    XCB_FILL_STYLE_STIPPLED = 2,
    XCB_FILL_STYLE_OPAQUE_STIPPLED = 3
} xcb_fill_style_t;

typedef enum xcb_fill_rule_t {
    XCB_FILL_RULE_EVEN_ODD = 0,
    XCB_FILL_RULE_WINDING = 1
} xcb_fill_rule_t;

typedef enum xcb_subwindow_mode_t {
    XCB_SUBWINDOW_MODE_CLIP_BY_CHILDREN = 0,
    XCB_SUBWINDOW_MODE_INCLUDE_INFERIORS = 1
} xcb_subwindow_mode_t;

typedef enum xcb_arc_mode_t {
    XCB_ARC_MODE_CHORD = 0,
    XCB_ARC_MODE_PIE_SLICE = 1
} xcb_arc_mode_t;

/**
 * @brief xcb_create_gc_value_list_t
 **/
typedef struct xcb_create_gc_value_list_t {
    uint32_t     function;
    uint32_t     plane_mask;
    uint32_t     foreground;
    uint32_t     background;
    uint32_t     line_width;
    uint32_t     line_style;
    uint32_t     cap_style;
    uint32_t     join_style;
    uint32_t     fill_style;
    uint32_t     fill_rule;
    xcb_pixmap_t tile;
    xcb_pixmap_t stipple;
    int32_t      tile_stipple_x_origin;
    int32_t      tile_stipple_y_origin;
    xcb_font_t   font;
    uint32_t     subwindow_mode;
    xcb_bool32_t graphics_exposures;
    int32_t      clip_x_origin;
    int32_t      clip_y_origin;
    xcb_pixmap_t clip_mask;
    uint32_t     dash_offset;
    uint32_t     dashes;
    uint32_t     arc_mode;
} xcb_create_gc_value_list_t;

/** Opcode for xcb_create_gc. */
#define XCB_CREATE_GC 55

/**
 * @brief xcb_create_gc_request_t
 **/
typedef struct xcb_create_gc_request_t {
    uint8_t        major_opcode;
    uint8_t        pad0;
    uint16_t       length;
    xcb_gcontext_t cid;
    xcb_drawable_t drawable;
    uint32_t       value_mask;
} xcb_create_gc_request_t;

/**
 * @brief xcb_change_gc_value_list_t
 **/
typedef struct xcb_change_gc_value_list_t {
    uint32_t     function;
    uint32_t     plane_mask;
    uint32_t     foreground;
    uint32_t     background;
    uint32_t     line_width;
    uint32_t     line_style;
    uint32_t     cap_style;
    uint32_t     join_style;
    uint32_t     fill_style;
    uint32_t     fill_rule;
    xcb_pixmap_t tile;
    xcb_pixmap_t stipple;
    int32_t      tile_stipple_x_origin;
    int32_t      tile_stipple_y_origin;
    xcb_font_t   font;
    uint32_t     subwindow_mode;
    xcb_bool32_t graphics_exposures;
    int32_t      clip_x_origin;
    int32_t      clip_y_origin;
    xcb_pixmap_t clip_mask;
    uint32_t     dash_offset;
    uint32_t     dashes;
    uint32_t     arc_mode;
} xcb_change_gc_value_list_t;

/** Opcode for xcb_change_gc. */
#define XCB_CHANGE_GC 56

/**
 * @brief xcb_change_gc_request_t
 **/
typedef struct xcb_change_gc_request_t {
    uint8_t        major_opcode;
    uint8_t        pad0;
    uint16_t       length;
    xcb_gcontext_t gc;
    uint32_t       value_mask;
} xcb_change_gc_request_t;

/** Opcode for xcb_copy_gc. */
#define XCB_COPY_GC 57

/**
 * @brief xcb_copy_gc_request_t
 **/
typedef struct xcb_copy_gc_request_t {
    uint8_t        major_opcode;
    uint8_t        pad0;
    uint16_t       length;
    xcb_gcontext_t src_gc;
    xcb_gcontext_t dst_gc;
    uint32_t       value_mask;
} xcb_copy_gc_request_t;

/** Opcode for xcb_set_dashes. */
#define XCB_SET_DASHES 58

/**
 * @brief xcb_set_dashes_request_t
 **/
typedef struct xcb_set_dashes_request_t {
    uint8_t        major_opcode;
    uint8_t        pad0;
    uint16_t       length;
    xcb_gcontext_t gc;
    uint16_t       dash_offset;
    uint16_t       dashes_len;
} xcb_set_dashes_request_t;

typedef enum xcb_clip_ordering_t {
    XCB_CLIP_ORDERING_UNSORTED = 0,
    XCB_CLIP_ORDERING_Y_SORTED = 1,
    XCB_CLIP_ORDERING_YX_SORTED = 2,
    XCB_CLIP_ORDERING_YX_BANDED = 3
} xcb_clip_ordering_t;

/** Opcode for xcb_set_clip_rectangles. */
#define XCB_SET_CLIP_RECTANGLES 59

/**
 * @brief xcb_set_clip_rectangles_request_t
 **/
typedef struct xcb_set_clip_rectangles_request_t {
    uint8_t        major_opcode;
    uint8_t        ordering;
    uint16_t       length;
    xcb_gcontext_t gc;
    int16_t        clip_x_origin;
    int16_t        clip_y_origin;
} xcb_set_clip_rectangles_request_t;

/** Opcode for xcb_free_gc. */
#define XCB_FREE_GC 60

/**
 * @brief xcb_free_gc_request_t
 **/
typedef struct xcb_free_gc_request_t {
    uint8_t        major_opcode;
    uint8_t        pad0;
    uint16_t       length;
    xcb_gcontext_t gc;
} xcb_free_gc_request_t;

/** Opcode for xcb_clear_area. */
#define XCB_CLEAR_AREA 61

/**
 * @brief xcb_clear_area_request_t
 **/
typedef struct xcb_clear_area_request_t {
    uint8_t      major_opcode;
    uint8_t      exposures;
    uint16_t     length;
    xcb_window_t window;
    int16_t      x;
    int16_t      y;
    uint16_t     width;
    uint16_t     height;
} xcb_clear_area_request_t;

/** Opcode for xcb_copy_area. */
#define XCB_COPY_AREA 62

/**
 * @brief xcb_copy_area_request_t
 **/
typedef struct xcb_copy_area_request_t {
    uint8_t        major_opcode;
    uint8_t        pad0;
    uint16_t       length;
    xcb_drawable_t src_drawable;
    xcb_drawable_t dst_drawable;
    xcb_gcontext_t gc;
    int16_t        src_x;
    int16_t        src_y;
    int16_t        dst_x;
    int16_t        dst_y;
    uint16_t       width;
    uint16_t       height;
} xcb_copy_area_request_t;

/** Opcode for xcb_copy_plane. */
#define XCB_COPY_PLANE 63

/**
 * @brief xcb_copy_plane_request_t
 **/
typedef struct xcb_copy_plane_request_t {
    uint8_t        major_opcode;
    uint8_t        pad0;
    uint16_t       length;
    xcb_drawable_t src_drawable;
    xcb_drawable_t dst_drawable;
    xcb_gcontext_t gc;
    int16_t        src_x;
    int16_t        src_y;
    int16_t        dst_x;
    int16_t        dst_y;
    uint16_t       width;
    uint16_t       height;
    uint32_t       bit_plane;
} xcb_copy_plane_request_t;

typedef enum xcb_coord_mode_t {
    XCB_COORD_MODE_ORIGIN = 0,
/**< Treats all coordinates as relative to the origin. */

    XCB_COORD_MODE_PREVIOUS = 1
/**< Treats all coordinates after the first as relative to the previous coordinate. */

} xcb_coord_mode_t;

/** Opcode for xcb_poly_point. */
#define XCB_POLY_POINT 64

/**
 * @brief xcb_poly_point_request_t
 **/
typedef struct xcb_poly_point_request_t {
    uint8_t        major_opcode;
    uint8_t        coordinate_mode;
    uint16_t       length;
    xcb_drawable_t drawable;
    xcb_gcontext_t gc;
} xcb_poly_point_request_t;

/** Opcode for xcb_poly_line. */
#define XCB_POLY_LINE 65

/**
 * @brief xcb_poly_line_request_t
 **/
typedef struct xcb_poly_line_request_t {
    uint8_t        major_opcode;
    uint8_t        coordinate_mode;
    uint16_t       length;
    xcb_drawable_t drawable;
    xcb_gcontext_t gc;
} xcb_poly_line_request_t;

/**
 * @brief xcb_segment_t
 **/
typedef struct xcb_segment_t {
    int16_t x1;
    int16_t y1;
    int16_t x2;
    int16_t y2;
} xcb_segment_t;

/**
 * @brief xcb_segment_iterator_t
 **/
typedef struct xcb_segment_iterator_t {
    xcb_segment_t *data;
    int            rem;
    int            index;
} xcb_segment_iterator_t;

/** Opcode for xcb_poly_segment. */
#define XCB_POLY_SEGMENT 66

/**
 * @brief xcb_poly_segment_request_t
 **/
typedef struct xcb_poly_segment_request_t {
    uint8_t        major_opcode;
    uint8_t        pad0;
    uint16_t       length;
    xcb_drawable_t drawable;
    xcb_gcontext_t gc;
} xcb_poly_segment_request_t;

/** Opcode for xcb_poly_rectangle. */
#define XCB_POLY_RECTANGLE 67

/**
 * @brief xcb_poly_rectangle_request_t
 **/
typedef struct xcb_poly_rectangle_request_t {
    uint8_t        major_opcode;
    uint8_t        pad0;
    uint16_t       length;
    xcb_drawable_t drawable;
    xcb_gcontext_t gc;
} xcb_poly_rectangle_request_t;

/** Opcode for xcb_poly_arc. */
#define XCB_POLY_ARC 68

/**
 * @brief xcb_poly_arc_request_t
 **/
typedef struct xcb_poly_arc_request_t {
    uint8_t        major_opcode;
    uint8_t        pad0;
    uint16_t       length;
    xcb_drawable_t drawable;
    xcb_gcontext_t gc;
} xcb_poly_arc_request_t;

typedef enum xcb_poly_shape_t {
    XCB_POLY_SHAPE_COMPLEX = 0,
    XCB_POLY_SHAPE_NONCONVEX = 1,
    XCB_POLY_SHAPE_CONVEX = 2
} xcb_poly_shape_t;

/** Opcode for xcb_fill_poly. */
#define XCB_FILL_POLY 69

/**
 * @brief xcb_fill_poly_request_t
 **/
typedef struct xcb_fill_poly_request_t {
    uint8_t        major_opcode;
    uint8_t        pad0;
    uint16_t       length;
    xcb_drawable_t drawable;
    xcb_gcontext_t gc;
    uint8_t        shape;
    uint8_t        coordinate_mode;
    uint8_t        pad1[2];
} xcb_fill_poly_request_t;

/** Opcode for xcb_poly_fill_rectangle. */
#define XCB_POLY_FILL_RECTANGLE 70

/**
 * @brief xcb_poly_fill_rectangle_request_t
 **/
typedef struct xcb_poly_fill_rectangle_request_t {
    uint8_t        major_opcode;
    uint8_t        pad0;
    uint16_t       length;
    xcb_drawable_t drawable;
    xcb_gcontext_t gc;
} xcb_poly_fill_rectangle_request_t;

/** Opcode for xcb_poly_fill_arc. */
#define XCB_POLY_FILL_ARC 71

/**
 * @brief xcb_poly_fill_arc_request_t
 **/
typedef struct xcb_poly_fill_arc_request_t {
    uint8_t        major_opcode;
    uint8_t        pad0;
    uint16_t       length;
    xcb_drawable_t drawable;
    xcb_gcontext_t gc;
} xcb_poly_fill_arc_request_t;

typedef enum xcb_image_format_t {
    XCB_IMAGE_FORMAT_XY_BITMAP = 0,
    XCB_IMAGE_FORMAT_XY_PIXMAP = 1,
    XCB_IMAGE_FORMAT_Z_PIXMAP = 2
} xcb_image_format_t;

/** Opcode for xcb_put_image. */
#define XCB_PUT_IMAGE 72

/**
 * @brief xcb_put_image_request_t
 **/
typedef struct xcb_put_image_request_t {
    uint8_t        major_opcode;
    uint8_t        format;
    uint16_t       length;
    xcb_drawable_t drawable;
    xcb_gcontext_t gc;
    uint16_t       width;
    uint16_t       height;
    int16_t        dst_x;
    int16_t        dst_y;
    uint8_t        left_pad;
    uint8_t        depth;
    uint8_t        pad0[2];
} xcb_put_image_request_t;

/**
 * @brief xcb_get_image_cookie_t
 **/
typedef struct xcb_get_image_cookie_t {
    unsigned int sequence;
} xcb_get_image_cookie_t;

/** Opcode for xcb_get_image. */
#define XCB_GET_IMAGE 73

/**
 * @brief xcb_get_image_request_t
 **/
typedef struct xcb_get_image_request_t {
    uint8_t        major_opcode;
    uint8_t        format;
    uint16_t       length;
    xcb_drawable_t drawable;
    int16_t        x;
    int16_t        y;
    uint16_t       width;
    uint16_t       height;
    uint32_t       plane_mask;
} xcb_get_image_request_t;

/**
 * @brief xcb_get_image_reply_t
 **/
typedef struct xcb_get_image_reply_t {
    uint8_t        response_type;
    uint8_t        depth;
    uint16_t       sequence;
    uint32_t       length;
    xcb_visualid_t visual;
    uint8_t        pad0[20];
} xcb_get_image_reply_t;

/** Opcode for xcb_poly_text_8. */
#define XCB_POLY_TEXT_8 74

/**
 * @brief xcb_poly_text_8_request_t
 **/
typedef struct xcb_poly_text_8_request_t {
    uint8_t        major_opcode;
    uint8_t        pad0;
    uint16_t       length;
    xcb_drawable_t drawable;
    xcb_gcontext_t gc;
    int16_t        x;
    int16_t        y;
} xcb_poly_text_8_request_t;

/** Opcode for xcb_poly_text_16. */
#define XCB_POLY_TEXT_16 75

/**
 * @brief xcb_poly_text_16_request_t
 **/
typedef struct xcb_poly_text_16_request_t {
    uint8_t        major_opcode;
    uint8_t        pad0;
    uint16_t       length;
    xcb_drawable_t drawable;
    xcb_gcontext_t gc;
    int16_t        x;
    int16_t        y;
} xcb_poly_text_16_request_t;

/** Opcode for xcb_image_text_8. */
#define XCB_IMAGE_TEXT_8 76

/**
 * @brief xcb_image_text_8_request_t
 **/
typedef struct xcb_image_text_8_request_t {
    uint8_t        major_opcode;
    uint8_t        string_len;
    uint16_t       length;
    xcb_drawable_t drawable;
    xcb_gcontext_t gc;
    int16_t        x;
    int16_t        y;
} xcb_image_text_8_request_t;

/** Opcode for xcb_image_text_16. */
#define XCB_IMAGE_TEXT_16 77

/**
 * @brief xcb_image_text_16_request_t
 **/
typedef struct xcb_image_text_16_request_t {
    uint8_t        major_opcode;
    uint8_t        string_len;
    uint16_t       length;
    xcb_drawable_t drawable;
    xcb_gcontext_t gc;
    int16_t        x;
    int16_t        y;
} xcb_image_text_16_request_t;

typedef enum xcb_colormap_alloc_t {
    XCB_COLORMAP_ALLOC_NONE = 0,
    XCB_COLORMAP_ALLOC_ALL = 1
} xcb_colormap_alloc_t;

/** Opcode for xcb_create_colormap. */
#define XCB_CREATE_COLORMAP 78

/**
 * @brief xcb_create_colormap_request_t
 **/
typedef struct xcb_create_colormap_request_t {
    uint8_t        major_opcode;
    uint8_t        alloc;
    uint16_t       length;
    xcb_colormap_t mid;
    xcb_window_t   window;
    xcb_visualid_t visual;
} xcb_create_colormap_request_t;

/** Opcode for xcb_free_colormap. */
#define XCB_FREE_COLORMAP 79

/**
 * @brief xcb_free_colormap_request_t
 **/
typedef struct xcb_free_colormap_request_t {
    uint8_t        major_opcode;
    uint8_t        pad0;
    uint16_t       length;
    xcb_colormap_t cmap;
} xcb_free_colormap_request_t;

/** Opcode for xcb_copy_colormap_and_free. */
#define XCB_COPY_COLORMAP_AND_FREE 80

/**
 * @brief xcb_copy_colormap_and_free_request_t
 **/
typedef struct xcb_copy_colormap_and_free_request_t {
    uint8_t        major_opcode;
    uint8_t        pad0;
    uint16_t       length;
    xcb_colormap_t mid;
    xcb_colormap_t src_cmap;
} xcb_copy_colormap_and_free_request_t;

/** Opcode for xcb_install_colormap. */
#define XCB_INSTALL_COLORMAP 81

/**
 * @brief xcb_install_colormap_request_t
 **/
typedef struct xcb_install_colormap_request_t {
    uint8_t        major_opcode;
    uint8_t        pad0;
    uint16_t       length;
    xcb_colormap_t cmap;
} xcb_install_colormap_request_t;

/** Opcode for xcb_uninstall_colormap. */
#define XCB_UNINSTALL_COLORMAP 82

/**
 * @brief xcb_uninstall_colormap_request_t
 **/
typedef struct xcb_uninstall_colormap_request_t {
    uint8_t        major_opcode;
    uint8_t        pad0;
    uint16_t       length;
    xcb_colormap_t cmap;
} xcb_uninstall_colormap_request_t;

/**
 * @brief xcb_list_installed_colormaps_cookie_t
 **/
typedef struct xcb_list_installed_colormaps_cookie_t {
    unsigned int sequence;
} xcb_list_installed_colormaps_cookie_t;

/** Opcode for xcb_list_installed_colormaps. */
#define XCB_LIST_INSTALLED_COLORMAPS 83

/**
 * @brief xcb_list_installed_colormaps_request_t
 **/
typedef struct xcb_list_installed_colormaps_request_t {
    uint8_t      major_opcode;
    uint8_t      pad0;
    uint16_t     length;
    xcb_window_t window;
} xcb_list_installed_colormaps_request_t;

/**
 * @brief xcb_list_installed_colormaps_reply_t
 **/
typedef struct xcb_list_installed_colormaps_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint16_t cmaps_len;
    uint8_t  pad1[22];
} xcb_list_installed_colormaps_reply_t;

/**
 * @brief xcb_alloc_color_cookie_t
 **/
typedef struct xcb_alloc_color_cookie_t {
    unsigned int sequence;
} xcb_alloc_color_cookie_t;

/** Opcode for xcb_alloc_color. */
#define XCB_ALLOC_COLOR 84

/**
 * @brief xcb_alloc_color_request_t
 **/
typedef struct xcb_alloc_color_request_t {
    uint8_t        major_opcode;
    uint8_t        pad0;
    uint16_t       length;
    xcb_colormap_t cmap;
    uint16_t       red;
    uint16_t       green;
    uint16_t       blue;
    uint8_t        pad1[2];
} xcb_alloc_color_request_t;

/**
 * @brief xcb_alloc_color_reply_t
 **/
typedef struct xcb_alloc_color_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint16_t red;
    uint16_t green;
    uint16_t blue;
    uint8_t  pad1[2];
    uint32_t pixel;
} xcb_alloc_color_reply_t;

/**
 * @brief xcb_alloc_named_color_cookie_t
 **/
typedef struct xcb_alloc_named_color_cookie_t {
    unsigned int sequence;
} xcb_alloc_named_color_cookie_t;

/** Opcode for xcb_alloc_named_color. */
#define XCB_ALLOC_NAMED_COLOR 85

/**
 * @brief xcb_alloc_named_color_request_t
 **/
typedef struct xcb_alloc_named_color_request_t {
    uint8_t        major_opcode;
    uint8_t        pad0;
    uint16_t       length;
    xcb_colormap_t cmap;
    uint16_t       name_len;
    uint8_t        pad1[2];
} xcb_alloc_named_color_request_t;

/**
 * @brief xcb_alloc_named_color_reply_t
 **/
typedef struct xcb_alloc_named_color_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint32_t pixel;
    uint16_t exact_red;
    uint16_t exact_green;
    uint16_t exact_blue;
    uint16_t visual_red;
    uint16_t visual_green;
    uint16_t visual_blue;
} xcb_alloc_named_color_reply_t;

/**
 * @brief xcb_alloc_color_cells_cookie_t
 **/
typedef struct xcb_alloc_color_cells_cookie_t {
    unsigned int sequence;
} xcb_alloc_color_cells_cookie_t;

/** Opcode for xcb_alloc_color_cells. */
#define XCB_ALLOC_COLOR_CELLS 86

/**
 * @brief xcb_alloc_color_cells_request_t
 **/
typedef struct xcb_alloc_color_cells_request_t {
    uint8_t        major_opcode;
    uint8_t        contiguous;
    uint16_t       length;
    xcb_colormap_t cmap;
    uint16_t       colors;
    uint16_t       planes;
} xcb_alloc_color_cells_request_t;

/**
 * @brief xcb_alloc_color_cells_reply_t
 **/
typedef struct xcb_alloc_color_cells_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint16_t pixels_len;
    uint16_t masks_len;
    uint8_t  pad1[20];
} xcb_alloc_color_cells_reply_t;

/**
 * @brief xcb_alloc_color_planes_cookie_t
 **/
typedef struct xcb_alloc_color_planes_cookie_t {
    unsigned int sequence;
} xcb_alloc_color_planes_cookie_t;

/** Opcode for xcb_alloc_color_planes. */
#define XCB_ALLOC_COLOR_PLANES 87

/**
 * @brief xcb_alloc_color_planes_request_t
 **/
typedef struct xcb_alloc_color_planes_request_t {
    uint8_t        major_opcode;
    uint8_t        contiguous;
    uint16_t       length;
    xcb_colormap_t cmap;
    uint16_t       colors;
    uint16_t       reds;
    uint16_t       greens;
    uint16_t       blues;
} xcb_alloc_color_planes_request_t;

/**
 * @brief xcb_alloc_color_planes_reply_t
 **/
typedef struct xcb_alloc_color_planes_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint16_t pixels_len;
    uint8_t  pad1[2];
    uint32_t red_mask;
    uint32_t green_mask;
    uint32_t blue_mask;
    uint8_t  pad2[8];
} xcb_alloc_color_planes_reply_t;

/** Opcode for xcb_free_colors. */
#define XCB_FREE_COLORS 88

/**
 * @brief xcb_free_colors_request_t
 **/
typedef struct xcb_free_colors_request_t {
    uint8_t        major_opcode;
    uint8_t        pad0;
    uint16_t       length;
    xcb_colormap_t cmap;
    uint32_t       plane_mask;
} xcb_free_colors_request_t;

typedef enum xcb_color_flag_t {
    XCB_COLOR_FLAG_RED = 1,
    XCB_COLOR_FLAG_GREEN = 2,
    XCB_COLOR_FLAG_BLUE = 4
} xcb_color_flag_t;

/**
 * @brief xcb_coloritem_t
 **/
typedef struct xcb_coloritem_t {
    uint32_t pixel;
    uint16_t red;
    uint16_t green;
    uint16_t blue;
    uint8_t  flags;
    uint8_t  pad0;
} xcb_coloritem_t;

/**
 * @brief xcb_coloritem_iterator_t
 **/
typedef struct xcb_coloritem_iterator_t {
    xcb_coloritem_t *data;
    int              rem;
    int              index;
} xcb_coloritem_iterator_t;

/** Opcode for xcb_store_colors. */
#define XCB_STORE_COLORS 89

/**
 * @brief xcb_store_colors_request_t
 **/
typedef struct xcb_store_colors_request_t {
    uint8_t        major_opcode;
    uint8_t        pad0;
    uint16_t       length;
    xcb_colormap_t cmap;
} xcb_store_colors_request_t;

/** Opcode for xcb_store_named_color. */
#define XCB_STORE_NAMED_COLOR 90

/**
 * @brief xcb_store_named_color_request_t
 **/
typedef struct xcb_store_named_color_request_t {
    uint8_t        major_opcode;
    uint8_t        flags;
    uint16_t       length;
    xcb_colormap_t cmap;
    uint32_t       pixel;
    uint16_t       name_len;
    uint8_t        pad0[2];
} xcb_store_named_color_request_t;

/**
 * @brief xcb_rgb_t
 **/
typedef struct xcb_rgb_t {
    uint16_t red;
    uint16_t green;
    uint16_t blue;
    uint8_t  pad0[2];
} xcb_rgb_t;

/**
 * @brief xcb_rgb_iterator_t
 **/
typedef struct xcb_rgb_iterator_t {
    xcb_rgb_t *data;
    int        rem;
    int        index;
} xcb_rgb_iterator_t;

/**
 * @brief xcb_query_colors_cookie_t
 **/
typedef struct xcb_query_colors_cookie_t {
    unsigned int sequence;
} xcb_query_colors_cookie_t;

/** Opcode for xcb_query_colors. */
#define XCB_QUERY_COLORS 91

/**
 * @brief xcb_query_colors_request_t
 **/
typedef struct xcb_query_colors_request_t {
    uint8_t        major_opcode;
    uint8_t        pad0;
    uint16_t       length;
    xcb_colormap_t cmap;
} xcb_query_colors_request_t;

/**
 * @brief xcb_query_colors_reply_t
 **/
typedef struct xcb_query_colors_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint16_t colors_len;
    uint8_t  pad1[22];
} xcb_query_colors_reply_t;

/**
 * @brief xcb_lookup_color_cookie_t
 **/
typedef struct xcb_lookup_color_cookie_t {
    unsigned int sequence;
} xcb_lookup_color_cookie_t;

/** Opcode for xcb_lookup_color. */
#define XCB_LOOKUP_COLOR 92

/**
 * @brief xcb_lookup_color_request_t
 **/
typedef struct xcb_lookup_color_request_t {
    uint8_t        major_opcode;
    uint8_t        pad0;
    uint16_t       length;
    xcb_colormap_t cmap;
    uint16_t       name_len;
    uint8_t        pad1[2];
} xcb_lookup_color_request_t;

/**
 * @brief xcb_lookup_color_reply_t
 **/
typedef struct xcb_lookup_color_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint16_t exact_red;
    uint16_t exact_green;
    uint16_t exact_blue;
    uint16_t visual_red;
    uint16_t visual_green;
    uint16_t visual_blue;
} xcb_lookup_color_reply_t;

typedef enum xcb_pixmap_enum_t {
    XCB_PIXMAP_NONE = 0
} xcb_pixmap_enum_t;

/** Opcode for xcb_create_cursor. */
#define XCB_CREATE_CURSOR 93

/**
 * @brief xcb_create_cursor_request_t
 **/
typedef struct xcb_create_cursor_request_t {
    uint8_t      major_opcode;
    uint8_t      pad0;
    uint16_t     length;
    xcb_cursor_t cid;
    xcb_pixmap_t source;
    xcb_pixmap_t mask;
    uint16_t     fore_red;
    uint16_t     fore_green;
    uint16_t     fore_blue;
    uint16_t     back_red;
    uint16_t     back_green;
    uint16_t     back_blue;
    uint16_t     x;
    uint16_t     y;
} xcb_create_cursor_request_t;

typedef enum xcb_font_enum_t {
    XCB_FONT_NONE = 0
} xcb_font_enum_t;

/** Opcode for xcb_create_glyph_cursor. */
#define XCB_CREATE_GLYPH_CURSOR 94

/**
 * @brief xcb_create_glyph_cursor_request_t
 **/
typedef struct xcb_create_glyph_cursor_request_t {
    uint8_t      major_opcode;
    uint8_t      pad0;
    uint16_t     length;
    xcb_cursor_t cid;
    xcb_font_t   source_font;
    xcb_font_t   mask_font;
    uint16_t     source_char;
    uint16_t     mask_char;
    uint16_t     fore_red;
    uint16_t     fore_green;
    uint16_t     fore_blue;
    uint16_t     back_red;
    uint16_t     back_green;
    uint16_t     back_blue;
} xcb_create_glyph_cursor_request_t;

/** Opcode for xcb_free_cursor. */
#define XCB_FREE_CURSOR 95

/**
 * @brief xcb_free_cursor_request_t
 **/
typedef struct xcb_free_cursor_request_t {
    uint8_t      major_opcode;
    uint8_t      pad0;
    uint16_t     length;
    xcb_cursor_t cursor;
} xcb_free_cursor_request_t;

/** Opcode for xcb_recolor_cursor. */
#define XCB_RECOLOR_CURSOR 96

/**
 * @brief xcb_recolor_cursor_request_t
 **/
typedef struct xcb_recolor_cursor_request_t {
    uint8_t      major_opcode;
    uint8_t      pad0;
    uint16_t     length;
    xcb_cursor_t cursor;
    uint16_t     fore_red;
    uint16_t     fore_green;
    uint16_t     fore_blue;
    uint16_t     back_red;
    uint16_t     back_green;
    uint16_t     back_blue;
} xcb_recolor_cursor_request_t;

typedef enum xcb_query_shape_of_t {
    XCB_QUERY_SHAPE_OF_LARGEST_CURSOR = 0,
    XCB_QUERY_SHAPE_OF_FASTEST_TILE = 1,
    XCB_QUERY_SHAPE_OF_FASTEST_STIPPLE = 2
} xcb_query_shape_of_t;

/**
 * @brief xcb_query_best_size_cookie_t
 **/
typedef struct xcb_query_best_size_cookie_t {
    unsigned int sequence;
} xcb_query_best_size_cookie_t;

/** Opcode for xcb_query_best_size. */
#define XCB_QUERY_BEST_SIZE 97

/**
 * @brief xcb_query_best_size_request_t
 **/
typedef struct xcb_query_best_size_request_t {
    uint8_t        major_opcode;
    uint8_t        _class;
    uint16_t       length;
    xcb_drawable_t drawable;
    uint16_t       width;
    uint16_t       height;
} xcb_query_best_size_request_t;

/**
 * @brief xcb_query_best_size_reply_t
 **/
typedef struct xcb_query_best_size_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint16_t width;
    uint16_t height;
} xcb_query_best_size_reply_t;

/**
 * @brief xcb_query_extension_cookie_t
 **/
typedef struct xcb_query_extension_cookie_t {
    unsigned int sequence;
} xcb_query_extension_cookie_t;

/** Opcode for xcb_query_extension. */
#define XCB_QUERY_EXTENSION 98

/**
 * @brief xcb_query_extension_request_t
 **/
typedef struct xcb_query_extension_request_t {
    uint8_t  major_opcode;
    uint8_t  pad0;
    uint16_t length;
    uint16_t name_len;
    uint8_t  pad1[2];
} xcb_query_extension_request_t;

/**
 * @brief xcb_query_extension_reply_t
 **/
typedef struct xcb_query_extension_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint8_t  present;
    uint8_t  major_opcode;
    uint8_t  first_event;
    uint8_t  first_error;
} xcb_query_extension_reply_t;

/**
 * @brief xcb_list_extensions_cookie_t
 **/
typedef struct xcb_list_extensions_cookie_t {
    unsigned int sequence;
} xcb_list_extensions_cookie_t;

/** Opcode for xcb_list_extensions. */
#define XCB_LIST_EXTENSIONS 99

/**
 * @brief xcb_list_extensions_request_t
 **/
typedef struct xcb_list_extensions_request_t {
    uint8_t  major_opcode;
    uint8_t  pad0;
    uint16_t length;
} xcb_list_extensions_request_t;

/**
 * @brief xcb_list_extensions_reply_t
 **/
typedef struct xcb_list_extensions_reply_t {
    uint8_t  response_type;
    uint8_t  names_len;
    uint16_t sequence;
    uint32_t length;
    uint8_t  pad0[24];
} xcb_list_extensions_reply_t;

/** Opcode for xcb_change_keyboard_mapping. */
#define XCB_CHANGE_KEYBOARD_MAPPING 100

/**
 * @brief xcb_change_keyboard_mapping_request_t
 **/
typedef struct xcb_change_keyboard_mapping_request_t {
    uint8_t       major_opcode;
    uint8_t       keycode_count;
    uint16_t      length;
    xcb_keycode_t first_keycode;
    uint8_t       keysyms_per_keycode;
    uint8_t       pad0[2];
} xcb_change_keyboard_mapping_request_t;

/**
 * @brief xcb_get_keyboard_mapping_cookie_t
 **/
typedef struct xcb_get_keyboard_mapping_cookie_t {
    unsigned int sequence;
} xcb_get_keyboard_mapping_cookie_t;

/** Opcode for xcb_get_keyboard_mapping. */
#define XCB_GET_KEYBOARD_MAPPING 101

/**
 * @brief xcb_get_keyboard_mapping_request_t
 **/
typedef struct xcb_get_keyboard_mapping_request_t {
    uint8_t       major_opcode;
    uint8_t       pad0;
    uint16_t      length;
    xcb_keycode_t first_keycode;
    uint8_t       count;
} xcb_get_keyboard_mapping_request_t;

/**
 * @brief xcb_get_keyboard_mapping_reply_t
 **/
typedef struct xcb_get_keyboard_mapping_reply_t {
    uint8_t  response_type;
    uint8_t  keysyms_per_keycode;
    uint16_t sequence;
    uint32_t length;
    uint8_t  pad0[24];
} xcb_get_keyboard_mapping_reply_t;

typedef enum xcb_kb_t {
    XCB_KB_KEY_CLICK_PERCENT = 1,
    XCB_KB_BELL_PERCENT = 2,
    XCB_KB_BELL_PITCH = 4,
    XCB_KB_BELL_DURATION = 8,
    XCB_KB_LED = 16,
    XCB_KB_LED_MODE = 32,
    XCB_KB_KEY = 64,
    XCB_KB_AUTO_REPEAT_MODE = 128
} xcb_kb_t;

typedef enum xcb_led_mode_t {
    XCB_LED_MODE_OFF = 0,
    XCB_LED_MODE_ON = 1
} xcb_led_mode_t;

typedef enum xcb_auto_repeat_mode_t {
    XCB_AUTO_REPEAT_MODE_OFF = 0,
    XCB_AUTO_REPEAT_MODE_ON = 1,
    XCB_AUTO_REPEAT_MODE_DEFAULT = 2
} xcb_auto_repeat_mode_t;

/**
 * @brief xcb_change_keyboard_control_value_list_t
 **/
typedef struct xcb_change_keyboard_control_value_list_t {
    int32_t         key_click_percent;
    int32_t         bell_percent;
    int32_t         bell_pitch;
    int32_t         bell_duration;
    uint32_t        led;
    uint32_t        led_mode;
    xcb_keycode32_t key;
    uint32_t        auto_repeat_mode;
} xcb_change_keyboard_control_value_list_t;

/** Opcode for xcb_change_keyboard_control. */
#define XCB_CHANGE_KEYBOARD_CONTROL 102

/**
 * @brief xcb_change_keyboard_control_request_t
 **/
typedef struct xcb_change_keyboard_control_request_t {
    uint8_t  major_opcode;
    uint8_t  pad0;
    uint16_t length;
    uint32_t value_mask;
} xcb_change_keyboard_control_request_t;

/**
 * @brief xcb_get_keyboard_control_cookie_t
 **/
typedef struct xcb_get_keyboard_control_cookie_t {
    unsigned int sequence;
} xcb_get_keyboard_control_cookie_t;

/** Opcode for xcb_get_keyboard_control. */
#define XCB_GET_KEYBOARD_CONTROL 103

/**
 * @brief xcb_get_keyboard_control_request_t
 **/
typedef struct xcb_get_keyboard_control_request_t {
    uint8_t  major_opcode;
    uint8_t  pad0;
    uint16_t length;
} xcb_get_keyboard_control_request_t;

/**
 * @brief xcb_get_keyboard_control_reply_t
 **/
typedef struct xcb_get_keyboard_control_reply_t {
    uint8_t  response_type;
    uint8_t  global_auto_repeat;
    uint16_t sequence;
    uint32_t length;
    uint32_t led_mask;
    uint8_t  key_click_percent;
    uint8_t  bell_percent;
    uint16_t bell_pitch;
    uint16_t bell_duration;
    uint8_t  pad0[2];
    uint8_t  auto_repeats[32];
} xcb_get_keyboard_control_reply_t;

/** Opcode for xcb_bell. */
#define XCB_BELL 104

/**
 * @brief xcb_bell_request_t
 **/
typedef struct xcb_bell_request_t {
    uint8_t  major_opcode;
    int8_t   percent;
    uint16_t length;
} xcb_bell_request_t;

/** Opcode for xcb_change_pointer_control. */
#define XCB_CHANGE_POINTER_CONTROL 105

/**
 * @brief xcb_change_pointer_control_request_t
 **/
typedef struct xcb_change_pointer_control_request_t {
    uint8_t  major_opcode;
    uint8_t  pad0;
    uint16_t length;
    int16_t  acceleration_numerator;
    int16_t  acceleration_denominator;
    int16_t  threshold;
    uint8_t  do_acceleration;
    uint8_t  do_threshold;
} xcb_change_pointer_control_request_t;

/**
 * @brief xcb_get_pointer_control_cookie_t
 **/
typedef struct xcb_get_pointer_control_cookie_t {
    unsigned int sequence;
} xcb_get_pointer_control_cookie_t;

/** Opcode for xcb_get_pointer_control. */
#define XCB_GET_POINTER_CONTROL 106

/**
 * @brief xcb_get_pointer_control_request_t
 **/
typedef struct xcb_get_pointer_control_request_t {
    uint8_t  major_opcode;
    uint8_t  pad0;
    uint16_t length;
} xcb_get_pointer_control_request_t;

/**
 * @brief xcb_get_pointer_control_reply_t
 **/
typedef struct xcb_get_pointer_control_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint16_t acceleration_numerator;
    uint16_t acceleration_denominator;
    uint16_t threshold;
    uint8_t  pad1[18];
} xcb_get_pointer_control_reply_t;

typedef enum xcb_blanking_t {
    XCB_BLANKING_NOT_PREFERRED = 0,
    XCB_BLANKING_PREFERRED = 1,
    XCB_BLANKING_DEFAULT = 2
} xcb_blanking_t;

typedef enum xcb_exposures_t {
    XCB_EXPOSURES_NOT_ALLOWED = 0,
    XCB_EXPOSURES_ALLOWED = 1,
    XCB_EXPOSURES_DEFAULT = 2
} xcb_exposures_t;

/** Opcode for xcb_set_screen_saver. */
#define XCB_SET_SCREEN_SAVER 107

/**
 * @brief xcb_set_screen_saver_request_t
 **/
typedef struct xcb_set_screen_saver_request_t {
    uint8_t  major_opcode;
    uint8_t  pad0;
    uint16_t length;
    int16_t  timeout;
    int16_t  interval;
    uint8_t  prefer_blanking;
    uint8_t  allow_exposures;
} xcb_set_screen_saver_request_t;

/**
 * @brief xcb_get_screen_saver_cookie_t
 **/
typedef struct xcb_get_screen_saver_cookie_t {
    unsigned int sequence;
} xcb_get_screen_saver_cookie_t;

/** Opcode for xcb_get_screen_saver. */
#define XCB_GET_SCREEN_SAVER 108

/**
 * @brief xcb_get_screen_saver_request_t
 **/
typedef struct xcb_get_screen_saver_request_t {
    uint8_t  major_opcode;
    uint8_t  pad0;
    uint16_t length;
} xcb_get_screen_saver_request_t;

/**
 * @brief xcb_get_screen_saver_reply_t
 **/
typedef struct xcb_get_screen_saver_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint16_t timeout;
    uint16_t interval;
    uint8_t  prefer_blanking;
    uint8_t  allow_exposures;
    uint8_t  pad1[18];
} xcb_get_screen_saver_reply_t;

typedef enum xcb_host_mode_t {
    XCB_HOST_MODE_INSERT = 0,
    XCB_HOST_MODE_DELETE = 1
} xcb_host_mode_t;

typedef enum xcb_family_t {
    XCB_FAMILY_INTERNET = 0,
    XCB_FAMILY_DECNET = 1,
    XCB_FAMILY_CHAOS = 2,
    XCB_FAMILY_SERVER_INTERPRETED = 5,
    XCB_FAMILY_INTERNET_6 = 6
} xcb_family_t;

/** Opcode for xcb_change_hosts. */
#define XCB_CHANGE_HOSTS 109

/**
 * @brief xcb_change_hosts_request_t
 **/
typedef struct xcb_change_hosts_request_t {
    uint8_t  major_opcode;
    uint8_t  mode;
    uint16_t length;
    uint8_t  family;
    uint8_t  pad0;
    uint16_t address_len;
} xcb_change_hosts_request_t;

/**
 * @brief xcb_host_t
 **/
typedef struct xcb_host_t {
    uint8_t  family;
    uint8_t  pad0;
    uint16_t address_len;
} xcb_host_t;

/**
 * @brief xcb_host_iterator_t
 **/
typedef struct xcb_host_iterator_t {
    xcb_host_t *data;
    int         rem;
    int         index;
} xcb_host_iterator_t;

/**
 * @brief xcb_list_hosts_cookie_t
 **/
typedef struct xcb_list_hosts_cookie_t {
    unsigned int sequence;
} xcb_list_hosts_cookie_t;

/** Opcode for xcb_list_hosts. */
#define XCB_LIST_HOSTS 110

/**
 * @brief xcb_list_hosts_request_t
 **/
typedef struct xcb_list_hosts_request_t {
    uint8_t  major_opcode;
    uint8_t  pad0;
    uint16_t length;
} xcb_list_hosts_request_t;

/**
 * @brief xcb_list_hosts_reply_t
 **/
typedef struct xcb_list_hosts_reply_t {
    uint8_t  response_type;
    uint8_t  mode;
    uint16_t sequence;
    uint32_t length;
    uint16_t hosts_len;
    uint8_t  pad0[22];
} xcb_list_hosts_reply_t;

typedef enum xcb_access_control_t {
    XCB_ACCESS_CONTROL_DISABLE = 0,
    XCB_ACCESS_CONTROL_ENABLE = 1
} xcb_access_control_t;

/** Opcode for xcb_set_access_control. */
#define XCB_SET_ACCESS_CONTROL 111

/**
 * @brief xcb_set_access_control_request_t
 **/
typedef struct xcb_set_access_control_request_t {
    uint8_t  major_opcode;
    uint8_t  mode;
    uint16_t length;
} xcb_set_access_control_request_t;

typedef enum xcb_close_down_t {
    XCB_CLOSE_DOWN_DESTROY_ALL = 0,
    XCB_CLOSE_DOWN_RETAIN_PERMANENT = 1,
    XCB_CLOSE_DOWN_RETAIN_TEMPORARY = 2
} xcb_close_down_t;

/** Opcode for xcb_set_close_down_mode. */
#define XCB_SET_CLOSE_DOWN_MODE 112

/**
 * @brief xcb_set_close_down_mode_request_t
 **/
typedef struct xcb_set_close_down_mode_request_t {
    uint8_t  major_opcode;
    uint8_t  mode;
    uint16_t length;
} xcb_set_close_down_mode_request_t;

typedef enum xcb_kill_t {
    XCB_KILL_ALL_TEMPORARY = 0
} xcb_kill_t;

/** Opcode for xcb_kill_client. */
#define XCB_KILL_CLIENT 113

/**
 * @brief xcb_kill_client_request_t
 **/
typedef struct xcb_kill_client_request_t {
    uint8_t  major_opcode;
    uint8_t  pad0;
    uint16_t length;
    uint32_t resource;
} xcb_kill_client_request_t;

/** Opcode for xcb_rotate_properties. */
#define XCB_ROTATE_PROPERTIES 114

/**
 * @brief xcb_rotate_properties_request_t
 **/
typedef struct xcb_rotate_properties_request_t {
    uint8_t      major_opcode;
    uint8_t      pad0;
    uint16_t     length;
    xcb_window_t window;
    uint16_t     atoms_len;
    int16_t      delta;
} xcb_rotate_properties_request_t;

typedef enum xcb_screen_saver_t {
    XCB_SCREEN_SAVER_RESET = 0,
    XCB_SCREEN_SAVER_ACTIVE = 1
} xcb_screen_saver_t;

/** Opcode for xcb_force_screen_saver. */
#define XCB_FORCE_SCREEN_SAVER 115

/**
 * @brief xcb_force_screen_saver_request_t
 **/
typedef struct xcb_force_screen_saver_request_t {
    uint8_t  major_opcode;
    uint8_t  mode;
    uint16_t length;
} xcb_force_screen_saver_request_t;

typedef enum xcb_mapping_status_t {
    XCB_MAPPING_STATUS_SUCCESS = 0,
    XCB_MAPPING_STATUS_BUSY = 1,
    XCB_MAPPING_STATUS_FAILURE = 2
} xcb_mapping_status_t;

/**
 * @brief xcb_set_pointer_mapping_cookie_t
 **/
typedef struct xcb_set_pointer_mapping_cookie_t {
    unsigned int sequence;
} xcb_set_pointer_mapping_cookie_t;

/** Opcode for xcb_set_pointer_mapping. */
#define XCB_SET_POINTER_MAPPING 116

/**
 * @brief xcb_set_pointer_mapping_request_t
 **/
typedef struct xcb_set_pointer_mapping_request_t {
    uint8_t  major_opcode;
    uint8_t  map_len;
    uint16_t length;
} xcb_set_pointer_mapping_request_t;

/**
 * @brief xcb_set_pointer_mapping_reply_t
 **/
typedef struct xcb_set_pointer_mapping_reply_t {
    uint8_t  response_type;
    uint8_t  status;
    uint16_t sequence;
    uint32_t length;
} xcb_set_pointer_mapping_reply_t;

/**
 * @brief xcb_get_pointer_mapping_cookie_t
 **/
typedef struct xcb_get_pointer_mapping_cookie_t {
    unsigned int sequence;
} xcb_get_pointer_mapping_cookie_t;

/** Opcode for xcb_get_pointer_mapping. */
#define XCB_GET_POINTER_MAPPING 117

/**
 * @brief xcb_get_pointer_mapping_request_t
 **/
typedef struct xcb_get_pointer_mapping_request_t {
    uint8_t  major_opcode;
    uint8_t  pad0;
    uint16_t length;
} xcb_get_pointer_mapping_request_t;

/**
 * @brief xcb_get_pointer_mapping_reply_t
 **/
typedef struct xcb_get_pointer_mapping_reply_t {
    uint8_t  response_type;
    uint8_t  map_len;
    uint16_t sequence;
    uint32_t length;
    uint8_t  pad0[24];
} xcb_get_pointer_mapping_reply_t;

typedef enum xcb_map_index_t {
    XCB_MAP_INDEX_SHIFT = 0,
    XCB_MAP_INDEX_LOCK = 1,
    XCB_MAP_INDEX_CONTROL = 2,
    XCB_MAP_INDEX_1 = 3,
    XCB_MAP_INDEX_2 = 4,
    XCB_MAP_INDEX_3 = 5,
    XCB_MAP_INDEX_4 = 6,
    XCB_MAP_INDEX_5 = 7
} xcb_map_index_t;

/**
 * @brief xcb_set_modifier_mapping_cookie_t
 **/
typedef struct xcb_set_modifier_mapping_cookie_t {
    unsigned int sequence;
} xcb_set_modifier_mapping_cookie_t;

/** Opcode for xcb_set_modifier_mapping. */
#define XCB_SET_MODIFIER_MAPPING 118

/**
 * @brief xcb_set_modifier_mapping_request_t
 **/
typedef struct xcb_set_modifier_mapping_request_t {
    uint8_t  major_opcode;
    uint8_t  keycodes_per_modifier;
    uint16_t length;
} xcb_set_modifier_mapping_request_t;

/**
 * @brief xcb_set_modifier_mapping_reply_t
 **/
typedef struct xcb_set_modifier_mapping_reply_t {
    uint8_t  response_type;
    uint8_t  status;
    uint16_t sequence;
    uint32_t length;
} xcb_set_modifier_mapping_reply_t;

/**
 * @brief xcb_get_modifier_mapping_cookie_t
 **/
typedef struct xcb_get_modifier_mapping_cookie_t {
    unsigned int sequence;
} xcb_get_modifier_mapping_cookie_t;

/** Opcode for xcb_get_modifier_mapping. */
#define XCB_GET_MODIFIER_MAPPING 119

/**
 * @brief xcb_get_modifier_mapping_request_t
 **/
typedef struct xcb_get_modifier_mapping_request_t {
    uint8_t  major_opcode;
    uint8_t  pad0;
    uint16_t length;
} xcb_get_modifier_mapping_request_t;

/**
 * @brief xcb_get_modifier_mapping_reply_t
 **/
typedef struct xcb_get_modifier_mapping_reply_t {
    uint8_t  response_type;
    uint8_t  keycodes_per_modifier;
    uint16_t sequence;
    uint32_t length;
    uint8_t  pad0[24];
} xcb_get_modifier_mapping_reply_t;

/** Opcode for xcb_no_operation. */
#define XCB_NO_OPERATION 127

/**
 * @brief xcb_no_operation_request_t
 **/
typedef struct xcb_no_operation_request_t {
    uint8_t  major_opcode;
    uint8_t  pad0;
    uint16_t length;
} xcb_no_operation_request_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_char2b_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_char2b_t)
 */
void
xcb_char2b_next (xcb_char2b_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_char2b_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_char2b_end (xcb_char2b_iterator_t i);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_window_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_window_t)
 */
void
xcb_window_next (xcb_window_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_window_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_window_end (xcb_window_iterator_t i);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_pixmap_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_pixmap_t)
 */
void
xcb_pixmap_next (xcb_pixmap_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_pixmap_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_pixmap_end (xcb_pixmap_iterator_t i);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_cursor_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_cursor_t)
 */
void
xcb_cursor_next (xcb_cursor_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_cursor_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_cursor_end (xcb_cursor_iterator_t i);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_font_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_font_t)
 */
void
xcb_font_next (xcb_font_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_font_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_font_end (xcb_font_iterator_t i);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_gcontext_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_gcontext_t)
 */
void
xcb_gcontext_next (xcb_gcontext_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_gcontext_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_gcontext_end (xcb_gcontext_iterator_t i);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_colormap_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_colormap_t)
 */
void
xcb_colormap_next (xcb_colormap_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_colormap_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_colormap_end (xcb_colormap_iterator_t i);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_atom_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_atom_t)
 */
void
xcb_atom_next (xcb_atom_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_atom_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_atom_end (xcb_atom_iterator_t i);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_drawable_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_drawable_t)
 */
void
xcb_drawable_next (xcb_drawable_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_drawable_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_drawable_end (xcb_drawable_iterator_t i);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_fontable_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_fontable_t)
 */
void
xcb_fontable_next (xcb_fontable_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_fontable_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_fontable_end (xcb_fontable_iterator_t i);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_bool32_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_bool32_t)
 */
void
xcb_bool32_next (xcb_bool32_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_bool32_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_bool32_end (xcb_bool32_iterator_t i);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_visualid_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_visualid_t)
 */
void
xcb_visualid_next (xcb_visualid_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_visualid_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_visualid_end (xcb_visualid_iterator_t i);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_timestamp_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_timestamp_t)
 */
void
xcb_timestamp_next (xcb_timestamp_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_timestamp_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_timestamp_end (xcb_timestamp_iterator_t i);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_keysym_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_keysym_t)
 */
void
xcb_keysym_next (xcb_keysym_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_keysym_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_keysym_end (xcb_keysym_iterator_t i);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_keycode_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_keycode_t)
 */
void
xcb_keycode_next (xcb_keycode_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_keycode_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_keycode_end (xcb_keycode_iterator_t i);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_keycode32_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_keycode32_t)
 */
void
xcb_keycode32_next (xcb_keycode32_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_keycode32_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_keycode32_end (xcb_keycode32_iterator_t i);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_button_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_button_t)
 */
void
xcb_button_next (xcb_button_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_button_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_button_end (xcb_button_iterator_t i);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_point_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_point_t)
 */
void
xcb_point_next (xcb_point_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_point_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_point_end (xcb_point_iterator_t i);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_rectangle_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_rectangle_t)
 */
void
xcb_rectangle_next (xcb_rectangle_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_rectangle_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_rectangle_end (xcb_rectangle_iterator_t i);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_arc_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_arc_t)
 */
void
xcb_arc_next (xcb_arc_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_arc_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_arc_end (xcb_arc_iterator_t i);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_format_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_format_t)
 */
void
xcb_format_next (xcb_format_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_format_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_format_end (xcb_format_iterator_t i);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_visualtype_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_visualtype_t)
 */
void
xcb_visualtype_next (xcb_visualtype_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_visualtype_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_visualtype_end (xcb_visualtype_iterator_t i);

int
xcb_depth_sizeof (const void  *_buffer);

xcb_visualtype_t *
xcb_depth_visuals (const xcb_depth_t *R);

int
xcb_depth_visuals_length (const xcb_depth_t *R);

xcb_visualtype_iterator_t
xcb_depth_visuals_iterator (const xcb_depth_t *R);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_depth_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_depth_t)
 */
void
xcb_depth_next (xcb_depth_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_depth_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_depth_end (xcb_depth_iterator_t i);

int
xcb_screen_sizeof (const void  *_buffer);

int
xcb_screen_allowed_depths_length (const xcb_screen_t *R);

xcb_depth_iterator_t
xcb_screen_allowed_depths_iterator (const xcb_screen_t *R);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_screen_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_screen_t)
 */
void
xcb_screen_next (xcb_screen_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_screen_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_screen_end (xcb_screen_iterator_t i);

int
xcb_setup_request_sizeof (const void  *_buffer);

char *
xcb_setup_request_authorization_protocol_name (const xcb_setup_request_t *R);

int
xcb_setup_request_authorization_protocol_name_length (const xcb_setup_request_t *R);

xcb_generic_iterator_t
xcb_setup_request_authorization_protocol_name_end (const xcb_setup_request_t *R);

char *
xcb_setup_request_authorization_protocol_data (const xcb_setup_request_t *R);

int
xcb_setup_request_authorization_protocol_data_length (const xcb_setup_request_t *R);

xcb_generic_iterator_t
xcb_setup_request_authorization_protocol_data_end (const xcb_setup_request_t *R);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_setup_request_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_setup_request_t)
 */
void
xcb_setup_request_next (xcb_setup_request_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_setup_request_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_setup_request_end (xcb_setup_request_iterator_t i);

int
xcb_setup_failed_sizeof (const void  *_buffer);

char *
xcb_setup_failed_reason (const xcb_setup_failed_t *R);

int
xcb_setup_failed_reason_length (const xcb_setup_failed_t *R);

xcb_generic_iterator_t
xcb_setup_failed_reason_end (const xcb_setup_failed_t *R);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_setup_failed_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_setup_failed_t)
 */
void
xcb_setup_failed_next (xcb_setup_failed_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_setup_failed_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_setup_failed_end (xcb_setup_failed_iterator_t i);

int
xcb_setup_authenticate_sizeof (const void  *_buffer);

char *
xcb_setup_authenticate_reason (const xcb_setup_authenticate_t *R);

int
xcb_setup_authenticate_reason_length (const xcb_setup_authenticate_t *R);

xcb_generic_iterator_t
xcb_setup_authenticate_reason_end (const xcb_setup_authenticate_t *R);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_setup_authenticate_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_setup_authenticate_t)
 */
void
xcb_setup_authenticate_next (xcb_setup_authenticate_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_setup_authenticate_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_setup_authenticate_end (xcb_setup_authenticate_iterator_t i);

int
xcb_setup_sizeof (const void  *_buffer);

char *
xcb_setup_vendor (const xcb_setup_t *R);

int
xcb_setup_vendor_length (const xcb_setup_t *R);

xcb_generic_iterator_t
xcb_setup_vendor_end (const xcb_setup_t *R);

xcb_format_t *
xcb_setup_pixmap_formats (const xcb_setup_t *R);

int
xcb_setup_pixmap_formats_length (const xcb_setup_t *R);

xcb_format_iterator_t
xcb_setup_pixmap_formats_iterator (const xcb_setup_t *R);

int
xcb_setup_roots_length (const xcb_setup_t *R);

xcb_screen_iterator_t
xcb_setup_roots_iterator (const xcb_setup_t *R);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_setup_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_setup_t)
 */
void
xcb_setup_next (xcb_setup_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_setup_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_setup_end (xcb_setup_iterator_t i);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_client_message_data_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_client_message_data_t)
 */
void
xcb_client_message_data_next (xcb_client_message_data_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_client_message_data_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_client_message_data_end (xcb_client_message_data_iterator_t i);

int
xcb_create_window_value_list_serialize (void                                 **_buffer,
                                        uint32_t                               value_mask,
                                        const xcb_create_window_value_list_t  *_aux);

int
xcb_create_window_value_list_unpack (const void                      *_buffer,
                                     uint32_t                         value_mask,
                                     xcb_create_window_value_list_t  *_aux);

int
xcb_create_window_value_list_sizeof (const void  *_buffer,
                                     uint32_t     value_mask);

int
xcb_create_window_sizeof (const void  *_buffer);

/**
 * @brief Creates a window
 *
 * @param c The connection
 * @param depth Specifies the new window's depth (TODO: what unit?).
 * \n
 * The special value `XCB_COPY_FROM_PARENT` means the depth is taken from the
 * \a parent window.
 * @param wid The ID with which you will refer to the new window, created by
 * `xcb_generate_id`.
 * @param parent The parent window of the new window.
 * @param x The X coordinate of the new window.
 * @param y The Y coordinate of the new window.
 * @param width The width of the new window.
 * @param height The height of the new window.
 * @param border_width TODO:
 * \n
 * Must be zero if the `class` is `InputOnly` or a `xcb_match_error_t` occurs.
 * @param _class A bitmask of #xcb_window_class_t values.
 * @param _class \n
 * @param visual Specifies the id for the new window's visual.
 * \n
 * The special value `XCB_COPY_FROM_PARENT` means the visual is taken from the
 * \a parent window.
 * @param value_mask A bitmask of #xcb_cw_t values.
 * @return A cookie
 *
 * Creates an unmapped window as child of the specified \a parent window. A
 * CreateNotify event will be generated. The new window is placed on top in the
 * stacking order with respect to siblings.
 * 
 * The coordinate system has the X axis horizontal and the Y axis vertical with
 * the origin [0, 0] at the upper-left corner. Coordinates are integral, in terms
 * of pixels, and coincide with pixel centers. Each window and pixmap has its own
 * coordinate system. For a window, the origin is inside the border at the inside,
 * upper-left corner.
 * 
 * The created window is not yet displayed (mapped), call `xcb_map_window` to
 * display it.
 * 
 * The created window will initially use the same cursor as its parent.
 *
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
xcb_void_cookie_t
xcb_create_window_checked (xcb_connection_t *c,
                           uint8_t           depth,
                           xcb_window_t      wid,
                           xcb_window_t      parent,
                           int16_t           x,
                           int16_t           y,
                           uint16_t          width,
                           uint16_t          height,
                           uint16_t          border_width,
                           uint16_t          _class,
                           xcb_visualid_t    visual,
                           uint32_t          value_mask,
                           const void       *value_list);

/**
 * @brief Creates a window
 *
 * @param c The connection
 * @param depth Specifies the new window's depth (TODO: what unit?).
 * \n
 * The special value `XCB_COPY_FROM_PARENT` means the depth is taken from the
 * \a parent window.
 * @param wid The ID with which you will refer to the new window, created by
 * `xcb_generate_id`.
 * @param parent The parent window of the new window.
 * @param x The X coordinate of the new window.
 * @param y The Y coordinate of the new window.
 * @param width The width of the new window.
 * @param height The height of the new window.
 * @param border_width TODO:
 * \n
 * Must be zero if the `class` is `InputOnly` or a `xcb_match_error_t` occurs.
 * @param _class A bitmask of #xcb_window_class_t values.
 * @param _class \n
 * @param visual Specifies the id for the new window's visual.
 * \n
 * The special value `XCB_COPY_FROM_PARENT` means the visual is taken from the
 * \a parent window.
 * @param value_mask A bitmask of #xcb_cw_t values.
 * @return A cookie
 *
 * Creates an unmapped window as child of the specified \a parent window. A
 * CreateNotify event will be generated. The new window is placed on top in the
 * stacking order with respect to siblings.
 * 
 * The coordinate system has the X axis horizontal and the Y axis vertical with
 * the origin [0, 0] at the upper-left corner. Coordinates are integral, in terms
 * of pixels, and coincide with pixel centers. Each window and pixmap has its own
 * coordinate system. For a window, the origin is inside the border at the inside,
 * upper-left corner.
 * 
 * The created window is not yet displayed (mapped), call `xcb_map_window` to
 * display it.
 * 
 * The created window will initially use the same cursor as its parent.
 *
 */
xcb_void_cookie_t
xcb_create_window (xcb_connection_t *c,
                   uint8_t           depth,
                   xcb_window_t      wid,
                   xcb_window_t      parent,
                   int16_t           x,
                   int16_t           y,
                   uint16_t          width,
                   uint16_t          height,
                   uint16_t          border_width,
                   uint16_t          _class,
                   xcb_visualid_t    visual,
                   uint32_t          value_mask,
                   const void       *value_list);

/**
 * @brief Creates a window
 *
 * @param c The connection
 * @param depth Specifies the new window's depth (TODO: what unit?).
 * \n
 * The special value `XCB_COPY_FROM_PARENT` means the depth is taken from the
 * \a parent window.
 * @param wid The ID with which you will refer to the new window, created by
 * `xcb_generate_id`.
 * @param parent The parent window of the new window.
 * @param x The X coordinate of the new window.
 * @param y The Y coordinate of the new window.
 * @param width The width of the new window.
 * @param height The height of the new window.
 * @param border_width TODO:
 * \n
 * Must be zero if the `class` is `InputOnly` or a `xcb_match_error_t` occurs.
 * @param _class A bitmask of #xcb_window_class_t values.
 * @param _class \n
 * @param visual Specifies the id for the new window's visual.
 * \n
 * The special value `XCB_COPY_FROM_PARENT` means the visual is taken from the
 * \a parent window.
 * @param value_mask A bitmask of #xcb_cw_t values.
 * @return A cookie
 *
 * Creates an unmapped window as child of the specified \a parent window. A
 * CreateNotify event will be generated. The new window is placed on top in the
 * stacking order with respect to siblings.
 * 
 * The coordinate system has the X axis horizontal and the Y axis vertical with
 * the origin [0, 0] at the upper-left corner. Coordinates are integral, in terms
 * of pixels, and coincide with pixel centers. Each window and pixmap has its own
 * coordinate system. For a window, the origin is inside the border at the inside,
 * upper-left corner.
 * 
 * The created window is not yet displayed (mapped), call `xcb_map_window` to
 * display it.
 * 
 * The created window will initially use the same cursor as its parent.
 *
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
xcb_void_cookie_t
xcb_create_window_aux_checked (xcb_connection_t                     *c,
                               uint8_t                               depth,
                               xcb_window_t                          wid,
                               xcb_window_t                          parent,
                               int16_t                               x,
                               int16_t                               y,
                               uint16_t                              width,
                               uint16_t                              height,
                               uint16_t                              border_width,
                               uint16_t                              _class,
                               xcb_visualid_t                        visual,
                               uint32_t                              value_mask,
                               const xcb_create_window_value_list_t *value_list);

/**
 * @brief Creates a window
 *
 * @param c The connection
 * @param depth Specifies the new window's depth (TODO: what unit?).
 * \n
 * The special value `XCB_COPY_FROM_PARENT` means the depth is taken from the
 * \a parent window.
 * @param wid The ID with which you will refer to the new window, created by
 * `xcb_generate_id`.
 * @param parent The parent window of the new window.
 * @param x The X coordinate of the new window.
 * @param y The Y coordinate of the new window.
 * @param width The width of the new window.
 * @param height The height of the new window.
 * @param border_width TODO:
 * \n
 * Must be zero if the `class` is `InputOnly` or a `xcb_match_error_t` occurs.
 * @param _class A bitmask of #xcb_window_class_t values.
 * @param _class \n
 * @param visual Specifies the id for the new window's visual.
 * \n
 * The special value `XCB_COPY_FROM_PARENT` means the visual is taken from the
 * \a parent window.
 * @param value_mask A bitmask of #xcb_cw_t values.
 * @return A cookie
 *
 * Creates an unmapped window as child of the specified \a parent window. A
 * CreateNotify event will be generated. The new window is placed on top in the
 * stacking order with respect to siblings.
 * 
 * The coordinate system has the X axis horizontal and the Y axis vertical with
 * the origin [0, 0] at the upper-left corner. Coordinates are integral, in terms
 * of pixels, and coincide with pixel centers. Each window and pixmap has its own
 * coordinate system. For a window, the origin is inside the border at the inside,
 * upper-left corner.
 * 
 * The created window is not yet displayed (mapped), call `xcb_map_window` to
 * display it.
 * 
 * The created window will initially use the same cursor as its parent.
 *
 */
xcb_void_cookie_t
xcb_create_window_aux (xcb_connection_t                     *c,
                       uint8_t                               depth,
                       xcb_window_t                          wid,
                       xcb_window_t                          parent,
                       int16_t                               x,
                       int16_t                               y,
                       uint16_t                              width,
                       uint16_t                              height,
                       uint16_t                              border_width,
                       uint16_t                              _class,
                       xcb_visualid_t                        visual,
                       uint32_t                              value_mask,
                       const xcb_create_window_value_list_t *value_list);

void *
xcb_create_window_value_list (const xcb_create_window_request_t *R);

int
xcb_change_window_attributes_value_list_serialize (void                                            **_buffer,
                                                   uint32_t                                          value_mask,
                                                   const xcb_change_window_attributes_value_list_t  *_aux);

int
xcb_change_window_attributes_value_list_unpack (const void                                 *_buffer,
                                                uint32_t                                    value_mask,
                                                xcb_change_window_attributes_value_list_t  *_aux);

int
xcb_change_window_attributes_value_list_sizeof (const void  *_buffer,
                                                uint32_t     value_mask);

int
xcb_change_window_attributes_sizeof (const void  *_buffer);

/**
 * @brief change window attributes
 *
 * @param c The connection
 * @param window The window to change.
 * @param value_mask A bitmask of #xcb_cw_t values.
 * @param value_mask \n
 * @param value_list Values for each of the attributes specified in the bitmask \a value_mask. The
 * order has to correspond to the order of possible \a value_mask bits. See the
 * example.
 * @return A cookie
 *
 * Changes the attributes specified by \a value_mask for the specified \a window.
 *
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
xcb_void_cookie_t
xcb_change_window_attributes_checked (xcb_connection_t *c,
                                      xcb_window_t      window,
                                      uint32_t          value_mask,
                                      const void       *value_list);

/**
 * @brief change window attributes
 *
 * @param c The connection
 * @param window The window to change.
 * @param value_mask A bitmask of #xcb_cw_t values.
 * @param value_mask \n
 * @param value_list Values for each of the attributes specified in the bitmask \a value_mask. The
 * order has to correspond to the order of possible \a value_mask bits. See the
 * example.
 * @return A cookie
 *
 * Changes the attributes specified by \a value_mask for the specified \a window.
 *
 */
xcb_void_cookie_t
xcb_change_window_attributes (xcb_connection_t *c,
                              xcb_window_t      window,
                              uint32_t          value_mask,
                              const void       *value_list);

/**
 * @brief change window attributes
 *
 * @param c The connection
 * @param window The window to change.
 * @param value_mask A bitmask of #xcb_cw_t values.
 * @param value_mask \n
 * @param value_list Values for each of the attributes specified in the bitmask \a value_mask. The
 * order has to correspond to the order of possible \a value_mask bits. See the
 * example.
 * @return A cookie
 *
 * Changes the attributes specified by \a value_mask for the specified \a window.
 *
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
xcb_void_cookie_t
xcb_change_window_attributes_aux_checked (xcb_connection_t                                *c,
                                          xcb_window_t                                     window,
                                          uint32_t                                         value_mask,
                                          const xcb_change_window_attributes_value_list_t *value_list);

/**
 * @brief change window attributes
 *
 * @param c The connection
 * @param window The window to change.
 * @param value_mask A bitmask of #xcb_cw_t values.
 * @param value_mask \n
 * @param value_list Values for each of the attributes specified in the bitmask \a value_mask. The
 * order has to correspond to the order of possible \a value_mask bits. See the
 * example.
 * @return A cookie
 *
 * Changes the attributes specified by \a value_mask for the specified \a window.
 *
 */
xcb_void_cookie_t
xcb_change_window_attributes_aux (xcb_connection_t                                *c,
                                  xcb_window_t                                     window,
                                  uint32_t                                         value_mask,
                                  const xcb_change_window_attributes_value_list_t *value_list);

void *
xcb_change_window_attributes_value_list (const xcb_change_window_attributes_request_t *R);

/**
 * @brief Gets window attributes
 *
 * @param c The connection
 * @param window The window to get the attributes from.
 * @return A cookie
 *
 * Gets the current attributes for the specified \a window.
 *
 */
xcb_get_window_attributes_cookie_t
xcb_get_window_attributes (xcb_connection_t *c,
                           xcb_window_t      window);

/**
 * @brief Gets window attributes
 *
 * @param c The connection
 * @param window The window to get the attributes from.
 * @return A cookie
 *
 * Gets the current attributes for the specified \a window.
 *
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
xcb_get_window_attributes_cookie_t
xcb_get_window_attributes_unchecked (xcb_connection_t *c,
                                     xcb_window_t      window);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_get_window_attributes_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_get_window_attributes_reply_t *
xcb_get_window_attributes_reply (xcb_connection_t                    *c,
                                 xcb_get_window_attributes_cookie_t   cookie  /**< */,
                                 xcb_generic_error_t                **e);

/**
 * @brief Destroys a window
 *
 * @param c The connection
 * @param window The window to destroy.
 * @return A cookie
 *
 * Destroys the specified window and all of its subwindows. A DestroyNotify event
 * is generated for each destroyed window (a DestroyNotify event is first generated
 * for any given window's inferiors). If the window was mapped, it will be
 * automatically unmapped before destroying.
 * 
 * Calling DestroyWindow on the root window will do nothing.
 *
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
xcb_void_cookie_t
xcb_destroy_window_checked (xcb_connection_t *c,
                            xcb_window_t      window);

/**
 * @brief Destroys a window
 *
 * @param c The connection
 * @param window The window to destroy.
 * @return A cookie
 *
 * Destroys the specified window and all of its subwindows. A DestroyNotify event
 * is generated for each destroyed window (a DestroyNotify event is first generated
 * for any given window's inferiors). If the window was mapped, it will be
 * automatically unmapped before destroying.
 * 
 * Calling DestroyWindow on the root window will do nothing.
 *
 */
xcb_void_cookie_t
xcb_destroy_window (xcb_connection_t *c,
                    xcb_window_t      window);

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
xcb_destroy_subwindows_checked (xcb_connection_t *c,
                                xcb_window_t      window);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_destroy_subwindows (xcb_connection_t *c,
                        xcb_window_t      window);

/**
 * @brief Changes a client's save set
 *
 * @param c The connection
 * @param mode A bitmask of #xcb_set_mode_t values.
 * @param mode Insert to add the specified window to the save set or Delete to delete it from the save set.
 * @param window The window to add or delete to/from your save set.
 * @return A cookie
 *
 * TODO: explain what the save set is for.
 * 
 * This function either adds or removes the specified window to the client's (your
 * application's) save set.
 *
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
xcb_void_cookie_t
xcb_change_save_set_checked (xcb_connection_t *c,
                             uint8_t           mode,
                             xcb_window_t      window);

/**
 * @brief Changes a client's save set
 *
 * @param c The connection
 * @param mode A bitmask of #xcb_set_mode_t values.
 * @param mode Insert to add the specified window to the save set or Delete to delete it from the save set.
 * @param window The window to add or delete to/from your save set.
 * @return A cookie
 *
 * TODO: explain what the save set is for.
 * 
 * This function either adds or removes the specified window to the client's (your
 * application's) save set.
 *
 */
xcb_void_cookie_t
xcb_change_save_set (xcb_connection_t *c,
                     uint8_t           mode,
                     xcb_window_t      window);

/**
 * @brief Reparents a window
 *
 * @param c The connection
 * @param window The window to reparent.
 * @param parent The new parent of the window.
 * @param x The X position of the window within its new parent.
 * @param y The Y position of the window within its new parent.
 * @return A cookie
 *
 * Makes the specified window a child of the specified parent window. If the
 * window is mapped, it will automatically be unmapped before reparenting and
 * re-mapped after reparenting. The window is placed in the stacking order on top
 * with respect to sibling windows.
 * 
 * After reparenting, a ReparentNotify event is generated.
 *
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
xcb_void_cookie_t
xcb_reparent_window_checked (xcb_connection_t *c,
                             xcb_window_t      window,
                             xcb_window_t      parent,
                             int16_t           x,
                             int16_t           y);

/**
 * @brief Reparents a window
 *
 * @param c The connection
 * @param window The window to reparent.
 * @param parent The new parent of the window.
 * @param x The X position of the window within its new parent.
 * @param y The Y position of the window within its new parent.
 * @return A cookie
 *
 * Makes the specified window a child of the specified parent window. If the
 * window is mapped, it will automatically be unmapped before reparenting and
 * re-mapped after reparenting. The window is placed in the stacking order on top
 * with respect to sibling windows.
 * 
 * After reparenting, a ReparentNotify event is generated.
 *
 */
xcb_void_cookie_t
xcb_reparent_window (xcb_connection_t *c,
                     xcb_window_t      window,
                     xcb_window_t      parent,
                     int16_t           x,
                     int16_t           y);

/**
 * @brief Makes a window visible
 *
 * @param c The connection
 * @param window The window to make visible.
 * @return A cookie
 *
 * Maps the specified window. This means making the window visible (as long as its
 * parent is visible).
 * 
 * This MapWindow request will be translated to a MapRequest request if a window
 * manager is running. The window manager then decides to either map the window or
 * not. Set the override-redirect window attribute to true if you want to bypass
 * this mechanism.
 * 
 * If the window manager decides to map the window (or if no window manager is
 * running), a MapNotify event is generated.
 * 
 * If the window becomes viewable and no earlier contents for it are remembered,
 * the X server tiles the window with its background. If the window's background
 * is undefined, the existing screen contents are not altered, and the X server
 * generates zero or more Expose events.
 * 
 * If the window type is InputOutput, an Expose event will be generated when the
 * window becomes visible. The normal response to an Expose event should be to
 * repaint the window.
 *
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
xcb_void_cookie_t
xcb_map_window_checked (xcb_connection_t *c,
                        xcb_window_t      window);

/**
 * @brief Makes a window visible
 *
 * @param c The connection
 * @param window The window to make visible.
 * @return A cookie
 *
 * Maps the specified window. This means making the window visible (as long as its
 * parent is visible).
 * 
 * This MapWindow request will be translated to a MapRequest request if a window
 * manager is running. The window manager then decides to either map the window or
 * not. Set the override-redirect window attribute to true if you want to bypass
 * this mechanism.
 * 
 * If the window manager decides to map the window (or if no window manager is
 * running), a MapNotify event is generated.
 * 
 * If the window becomes viewable and no earlier contents for it are remembered,
 * the X server tiles the window with its background. If the window's background
 * is undefined, the existing screen contents are not altered, and the X server
 * generates zero or more Expose events.
 * 
 * If the window type is InputOutput, an Expose event will be generated when the
 * window becomes visible. The normal response to an Expose event should be to
 * repaint the window.
 *
 */
xcb_void_cookie_t
xcb_map_window (xcb_connection_t *c,
                xcb_window_t      window);

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
xcb_map_subwindows_checked (xcb_connection_t *c,
                            xcb_window_t      window);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_map_subwindows (xcb_connection_t *c,
                    xcb_window_t      window);

/**
 * @brief Makes a window invisible
 *
 * @param c The connection
 * @param window The window to make invisible.
 * @return A cookie
 *
 * Unmaps the specified window. This means making the window invisible (and all
 * its child windows).
 * 
 * Unmapping a window leads to the `UnmapNotify` event being generated. Also,
 * `Expose` events are generated for formerly obscured windows.
 *
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
xcb_void_cookie_t
xcb_unmap_window_checked (xcb_connection_t *c,
                          xcb_window_t      window);

/**
 * @brief Makes a window invisible
 *
 * @param c The connection
 * @param window The window to make invisible.
 * @return A cookie
 *
 * Unmaps the specified window. This means making the window invisible (and all
 * its child windows).
 * 
 * Unmapping a window leads to the `UnmapNotify` event being generated. Also,
 * `Expose` events are generated for formerly obscured windows.
 *
 */
xcb_void_cookie_t
xcb_unmap_window (xcb_connection_t *c,
                  xcb_window_t      window);

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
xcb_unmap_subwindows_checked (xcb_connection_t *c,
                              xcb_window_t      window);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_unmap_subwindows (xcb_connection_t *c,
                      xcb_window_t      window);

int
xcb_configure_window_value_list_serialize (void                                    **_buffer,
                                           uint16_t                                  value_mask,
                                           const xcb_configure_window_value_list_t  *_aux);

int
xcb_configure_window_value_list_unpack (const void                         *_buffer,
                                        uint16_t                            value_mask,
                                        xcb_configure_window_value_list_t  *_aux);

int
xcb_configure_window_value_list_sizeof (const void  *_buffer,
                                        uint16_t     value_mask);

int
xcb_configure_window_sizeof (const void  *_buffer);

/**
 * @brief Configures window attributes
 *
 * @param c The connection
 * @param window The window to configure.
 * @param value_mask Bitmask of attributes to change.
 * @param value_list New values, corresponding to the attributes in value_mask. The order has to
 * correspond to the order of possible \a value_mask bits. See the example.
 * @return A cookie
 *
 * Configures a window's size, position, border width and stacking order.
 *
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
xcb_void_cookie_t
xcb_configure_window_checked (xcb_connection_t *c,
                              xcb_window_t      window,
                              uint16_t          value_mask,
                              const void       *value_list);

/**
 * @brief Configures window attributes
 *
 * @param c The connection
 * @param window The window to configure.
 * @param value_mask Bitmask of attributes to change.
 * @param value_list New values, corresponding to the attributes in value_mask. The order has to
 * correspond to the order of possible \a value_mask bits. See the example.
 * @return A cookie
 *
 * Configures a window's size, position, border width and stacking order.
 *
 */
xcb_void_cookie_t
xcb_configure_window (xcb_connection_t *c,
                      xcb_window_t      window,
                      uint16_t          value_mask,
                      const void       *value_list);

/**
 * @brief Configures window attributes
 *
 * @param c The connection
 * @param window The window to configure.
 * @param value_mask Bitmask of attributes to change.
 * @param value_list New values, corresponding to the attributes in value_mask. The order has to
 * correspond to the order of possible \a value_mask bits. See the example.
 * @return A cookie
 *
 * Configures a window's size, position, border width and stacking order.
 *
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
xcb_void_cookie_t
xcb_configure_window_aux_checked (xcb_connection_t                        *c,
                                  xcb_window_t                             window,
                                  uint16_t                                 value_mask,
                                  const xcb_configure_window_value_list_t *value_list);

/**
 * @brief Configures window attributes
 *
 * @param c The connection
 * @param window The window to configure.
 * @param value_mask Bitmask of attributes to change.
 * @param value_list New values, corresponding to the attributes in value_mask. The order has to
 * correspond to the order of possible \a value_mask bits. See the example.
 * @return A cookie
 *
 * Configures a window's size, position, border width and stacking order.
 *
 */
xcb_void_cookie_t
xcb_configure_window_aux (xcb_connection_t                        *c,
                          xcb_window_t                             window,
                          uint16_t                                 value_mask,
                          const xcb_configure_window_value_list_t *value_list);

void *
xcb_configure_window_value_list (const xcb_configure_window_request_t *R);

/**
 * @brief Change window stacking order
 *
 * @param c The connection
 * @param direction A bitmask of #xcb_circulate_t values.
 * @param direction \n
 * @param window The window to raise/lower (depending on \a direction).
 * @return A cookie
 *
 * If \a direction is `XCB_CIRCULATE_RAISE_LOWEST`, the lowest mapped child (if
 * any) will be raised to the top of the stack.
 * 
 * If \a direction is `XCB_CIRCULATE_LOWER_HIGHEST`, the highest mapped child will
 * be lowered to the bottom of the stack.
 *
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
xcb_void_cookie_t
xcb_circulate_window_checked (xcb_connection_t *c,
                              uint8_t           direction,
                              xcb_window_t      window);

/**
 * @brief Change window stacking order
 *
 * @param c The connection
 * @param direction A bitmask of #xcb_circulate_t values.
 * @param direction \n
 * @param window The window to raise/lower (depending on \a direction).
 * @return A cookie
 *
 * If \a direction is `XCB_CIRCULATE_RAISE_LOWEST`, the lowest mapped child (if
 * any) will be raised to the top of the stack.
 * 
 * If \a direction is `XCB_CIRCULATE_LOWER_HIGHEST`, the highest mapped child will
 * be lowered to the bottom of the stack.
 *
 */
xcb_void_cookie_t
xcb_circulate_window (xcb_connection_t *c,
                      uint8_t           direction,
                      xcb_window_t      window);

/**
 * @brief Get current window geometry
 *
 * @param c The connection
 * @param drawable The drawable (`Window` or `Pixmap`) of which the geometry will be received.
 * @return A cookie
 *
 * Gets the current geometry of the specified drawable (either `Window` or `Pixmap`).
 *
 */
xcb_get_geometry_cookie_t
xcb_get_geometry (xcb_connection_t *c,
                  xcb_drawable_t    drawable);

/**
 * @brief Get current window geometry
 *
 * @param c The connection
 * @param drawable The drawable (`Window` or `Pixmap`) of which the geometry will be received.
 * @return A cookie
 *
 * Gets the current geometry of the specified drawable (either `Window` or `Pixmap`).
 *
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
xcb_get_geometry_cookie_t
xcb_get_geometry_unchecked (xcb_connection_t *c,
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
 * xcb_get_geometry_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_get_geometry_reply_t *
xcb_get_geometry_reply (xcb_connection_t           *c,
                        xcb_get_geometry_cookie_t   cookie  /**< */,
                        xcb_generic_error_t       **e);

int
xcb_query_tree_sizeof (const void  *_buffer);

/**
 * @brief query the window tree
 *
 * @param c The connection
 * @param window The \a window to query.
 * @return A cookie
 *
 * Gets the root window ID, parent window ID and list of children windows for the
 * specified \a window. The children are listed in bottom-to-top stacking order.
 *
 */
xcb_query_tree_cookie_t
xcb_query_tree (xcb_connection_t *c,
                xcb_window_t      window);

/**
 * @brief query the window tree
 *
 * @param c The connection
 * @param window The \a window to query.
 * @return A cookie
 *
 * Gets the root window ID, parent window ID and list of children windows for the
 * specified \a window. The children are listed in bottom-to-top stacking order.
 *
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
xcb_query_tree_cookie_t
xcb_query_tree_unchecked (xcb_connection_t *c,
                          xcb_window_t      window);

xcb_window_t *
xcb_query_tree_children (const xcb_query_tree_reply_t *R);

int
xcb_query_tree_children_length (const xcb_query_tree_reply_t *R);

xcb_generic_iterator_t
xcb_query_tree_children_end (const xcb_query_tree_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_query_tree_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_query_tree_reply_t *
xcb_query_tree_reply (xcb_connection_t         *c,
                      xcb_query_tree_cookie_t   cookie  /**< */,
                      xcb_generic_error_t     **e);

int
xcb_intern_atom_sizeof (const void  *_buffer);

/**
 * @brief Get atom identifier by name
 *
 * @param c The connection
 * @param only_if_exists Return a valid atom id only if the atom already exists.
 * @param name_len The length of the following \a name.
 * @param name The name of the atom.
 * @return A cookie
 *
 * Retrieves the identifier (xcb_atom_t TODO) for the atom with the specified
 * name. Atoms are used in protocols like EWMH, for example to store window titles
 * (`_NET_WM_NAME` atom) as property of a window.
 * 
 * If \a only_if_exists is 0, the atom will be created if it does not already exist.
 * If \a only_if_exists is 1, `XCB_ATOM_NONE` will be returned if the atom does
 * not yet exist.
 *
 */
xcb_intern_atom_cookie_t
xcb_intern_atom (xcb_connection_t *c,
                 uint8_t           only_if_exists,
                 uint16_t          name_len,
                 const char       *name);

/**
 * @brief Get atom identifier by name
 *
 * @param c The connection
 * @param only_if_exists Return a valid atom id only if the atom already exists.
 * @param name_len The length of the following \a name.
 * @param name The name of the atom.
 * @return A cookie
 *
 * Retrieves the identifier (xcb_atom_t TODO) for the atom with the specified
 * name. Atoms are used in protocols like EWMH, for example to store window titles
 * (`_NET_WM_NAME` atom) as property of a window.
 * 
 * If \a only_if_exists is 0, the atom will be created if it does not already exist.
 * If \a only_if_exists is 1, `XCB_ATOM_NONE` will be returned if the atom does
 * not yet exist.
 *
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
xcb_intern_atom_cookie_t
xcb_intern_atom_unchecked (xcb_connection_t *c,
                           uint8_t           only_if_exists,
                           uint16_t          name_len,
                           const char       *name);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_intern_atom_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_intern_atom_reply_t *
xcb_intern_atom_reply (xcb_connection_t          *c,
                       xcb_intern_atom_cookie_t   cookie  /**< */,
                       xcb_generic_error_t      **e);

int
xcb_get_atom_name_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_get_atom_name_cookie_t
xcb_get_atom_name (xcb_connection_t *c,
                   xcb_atom_t        atom);

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
xcb_get_atom_name_cookie_t
xcb_get_atom_name_unchecked (xcb_connection_t *c,
                             xcb_atom_t        atom);

char *
xcb_get_atom_name_name (const xcb_get_atom_name_reply_t *R);

int
xcb_get_atom_name_name_length (const xcb_get_atom_name_reply_t *R);

xcb_generic_iterator_t
xcb_get_atom_name_name_end (const xcb_get_atom_name_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_get_atom_name_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_get_atom_name_reply_t *
xcb_get_atom_name_reply (xcb_connection_t            *c,
                         xcb_get_atom_name_cookie_t   cookie  /**< */,
                         xcb_generic_error_t        **e);

int
xcb_change_property_sizeof (const void  *_buffer);

/**
 * @brief Changes a window property
 *
 * @param c The connection
 * @param mode A bitmask of #xcb_prop_mode_t values.
 * @param mode \n
 * @param window The window whose property you want to change.
 * @param property The property you want to change (an atom).
 * @param type The type of the property you want to change (an atom).
 * @param format Specifies whether the data should be viewed as a list of 8-bit, 16-bit or
 * 32-bit quantities. Possible values are 8, 16 and 32. This information allows
 * the X server to correctly perform byte-swap operations as necessary.
 * @param data_len Specifies the number of elements (see \a format).
 * @param data The property data.
 * @return A cookie
 *
 * Sets or updates a property on the specified \a window. Properties are for
 * example the window title (`WM_NAME`) or its minimum size (`WM_NORMAL_HINTS`).
 * Protocols such as EWMH also use properties - for example EWMH defines the
 * window title, encoded as UTF-8 string, in the `_NET_WM_NAME` property.
 *
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
xcb_void_cookie_t
xcb_change_property_checked (xcb_connection_t *c,
                             uint8_t           mode,
                             xcb_window_t      window,
                             xcb_atom_t        property,
                             xcb_atom_t        type,
                             uint8_t           format,
                             uint32_t          data_len,
                             const void       *data);

/**
 * @brief Changes a window property
 *
 * @param c The connection
 * @param mode A bitmask of #xcb_prop_mode_t values.
 * @param mode \n
 * @param window The window whose property you want to change.
 * @param property The property you want to change (an atom).
 * @param type The type of the property you want to change (an atom).
 * @param format Specifies whether the data should be viewed as a list of 8-bit, 16-bit or
 * 32-bit quantities. Possible values are 8, 16 and 32. This information allows
 * the X server to correctly perform byte-swap operations as necessary.
 * @param data_len Specifies the number of elements (see \a format).
 * @param data The property data.
 * @return A cookie
 *
 * Sets or updates a property on the specified \a window. Properties are for
 * example the window title (`WM_NAME`) or its minimum size (`WM_NORMAL_HINTS`).
 * Protocols such as EWMH also use properties - for example EWMH defines the
 * window title, encoded as UTF-8 string, in the `_NET_WM_NAME` property.
 *
 */
xcb_void_cookie_t
xcb_change_property (xcb_connection_t *c,
                     uint8_t           mode,
                     xcb_window_t      window,
                     xcb_atom_t        property,
                     xcb_atom_t        type,
                     uint8_t           format,
                     uint32_t          data_len,
                     const void       *data);

void *
xcb_change_property_data (const xcb_change_property_request_t *R);

int
xcb_change_property_data_length (const xcb_change_property_request_t *R);

xcb_generic_iterator_t
xcb_change_property_data_end (const xcb_change_property_request_t *R);

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
xcb_delete_property_checked (xcb_connection_t *c,
                             xcb_window_t      window,
                             xcb_atom_t        property);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_delete_property (xcb_connection_t *c,
                     xcb_window_t      window,
                     xcb_atom_t        property);

int
xcb_get_property_sizeof (const void  *_buffer);

/**
 * @brief Gets a window property
 *
 * @param c The connection
 * @param _delete Whether the property should actually be deleted. For deleting a property, the
 * specified \a type has to match the actual property type.
 * @param window The window whose property you want to get.
 * @param property The property you want to get (an atom).
 * @param type The type of the property you want to get (an atom).
 * @param long_offset Specifies the offset (in 32-bit multiples) in the specified property where the
 * data is to be retrieved.
 * @param long_length Specifies how many 32-bit multiples of data should be retrieved (e.g. if you
 * set \a long_length to 4, you will receive 16 bytes of data).
 * @return A cookie
 *
 * Gets the specified \a property from the specified \a window. Properties are for
 * example the window title (`WM_NAME`) or its minimum size (`WM_NORMAL_HINTS`).
 * Protocols such as EWMH also use properties - for example EWMH defines the
 * window title, encoded as UTF-8 string, in the `_NET_WM_NAME` property.
 * 
 * TODO: talk about \a type
 * 
 * TODO: talk about `delete`
 * 
 * TODO: talk about the offset/length thing. what's a valid use case?
 *
 */
xcb_get_property_cookie_t
xcb_get_property (xcb_connection_t *c,
                  uint8_t           _delete,
                  xcb_window_t      window,
                  xcb_atom_t        property,
                  xcb_atom_t        type,
                  uint32_t          long_offset,
                  uint32_t          long_length);

/**
 * @brief Gets a window property
 *
 * @param c The connection
 * @param _delete Whether the property should actually be deleted. For deleting a property, the
 * specified \a type has to match the actual property type.
 * @param window The window whose property you want to get.
 * @param property The property you want to get (an atom).
 * @param type The type of the property you want to get (an atom).
 * @param long_offset Specifies the offset (in 32-bit multiples) in the specified property where the
 * data is to be retrieved.
 * @param long_length Specifies how many 32-bit multiples of data should be retrieved (e.g. if you
 * set \a long_length to 4, you will receive 16 bytes of data).
 * @return A cookie
 *
 * Gets the specified \a property from the specified \a window. Properties are for
 * example the window title (`WM_NAME`) or its minimum size (`WM_NORMAL_HINTS`).
 * Protocols such as EWMH also use properties - for example EWMH defines the
 * window title, encoded as UTF-8 string, in the `_NET_WM_NAME` property.
 * 
 * TODO: talk about \a type
 * 
 * TODO: talk about `delete`
 * 
 * TODO: talk about the offset/length thing. what's a valid use case?
 *
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
xcb_get_property_cookie_t
xcb_get_property_unchecked (xcb_connection_t *c,
                            uint8_t           _delete,
                            xcb_window_t      window,
                            xcb_atom_t        property,
                            xcb_atom_t        type,
                            uint32_t          long_offset,
                            uint32_t          long_length);

void *
xcb_get_property_value (const xcb_get_property_reply_t *R);

int
xcb_get_property_value_length (const xcb_get_property_reply_t *R);

xcb_generic_iterator_t
xcb_get_property_value_end (const xcb_get_property_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_get_property_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_get_property_reply_t *
xcb_get_property_reply (xcb_connection_t           *c,
                        xcb_get_property_cookie_t   cookie  /**< */,
                        xcb_generic_error_t       **e);

int
xcb_list_properties_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_list_properties_cookie_t
xcb_list_properties (xcb_connection_t *c,
                     xcb_window_t      window);

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
xcb_list_properties_cookie_t
xcb_list_properties_unchecked (xcb_connection_t *c,
                               xcb_window_t      window);

xcb_atom_t *
xcb_list_properties_atoms (const xcb_list_properties_reply_t *R);

int
xcb_list_properties_atoms_length (const xcb_list_properties_reply_t *R);

xcb_generic_iterator_t
xcb_list_properties_atoms_end (const xcb_list_properties_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_list_properties_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_list_properties_reply_t *
xcb_list_properties_reply (xcb_connection_t              *c,
                           xcb_list_properties_cookie_t   cookie  /**< */,
                           xcb_generic_error_t          **e);

/**
 * @brief Sets the owner of a selection
 *
 * @param c The connection
 * @param owner The new owner of the selection.
 * \n
 * The special value `XCB_NONE` means that the selection will have no owner.
 * @param selection The selection.
 * @param time Timestamp to avoid race conditions when running X over the network.
 * \n
 * The selection will not be changed if \a time is earlier than the current
 * last-change time of the \a selection or is later than the current X server time.
 * Otherwise, the last-change time is set to the specified time.
 * \n
 * The special value `XCB_CURRENT_TIME` will be replaced with the current server
 * time.
 * @return A cookie
 *
 * Makes `window` the owner of the selection \a selection and updates the
 * last-change time of the specified selection.
 * 
 * TODO: briefly explain what a selection is.
 *
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
xcb_void_cookie_t
xcb_set_selection_owner_checked (xcb_connection_t *c,
                                 xcb_window_t      owner,
                                 xcb_atom_t        selection,
                                 xcb_timestamp_t   time);

/**
 * @brief Sets the owner of a selection
 *
 * @param c The connection
 * @param owner The new owner of the selection.
 * \n
 * The special value `XCB_NONE` means that the selection will have no owner.
 * @param selection The selection.
 * @param time Timestamp to avoid race conditions when running X over the network.
 * \n
 * The selection will not be changed if \a time is earlier than the current
 * last-change time of the \a selection or is later than the current X server time.
 * Otherwise, the last-change time is set to the specified time.
 * \n
 * The special value `XCB_CURRENT_TIME` will be replaced with the current server
 * time.
 * @return A cookie
 *
 * Makes `window` the owner of the selection \a selection and updates the
 * last-change time of the specified selection.
 * 
 * TODO: briefly explain what a selection is.
 *
 */
xcb_void_cookie_t
xcb_set_selection_owner (xcb_connection_t *c,
                         xcb_window_t      owner,
                         xcb_atom_t        selection,
                         xcb_timestamp_t   time);

/**
 * @brief Gets the owner of a selection
 *
 * @param c The connection
 * @param selection The selection.
 * @return A cookie
 *
 * Gets the owner of the specified selection.
 * 
 * TODO: briefly explain what a selection is.
 *
 */
xcb_get_selection_owner_cookie_t
xcb_get_selection_owner (xcb_connection_t *c,
                         xcb_atom_t        selection);

/**
 * @brief Gets the owner of a selection
 *
 * @param c The connection
 * @param selection The selection.
 * @return A cookie
 *
 * Gets the owner of the specified selection.
 * 
 * TODO: briefly explain what a selection is.
 *
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
xcb_get_selection_owner_cookie_t
xcb_get_selection_owner_unchecked (xcb_connection_t *c,
                                   xcb_atom_t        selection);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_get_selection_owner_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_get_selection_owner_reply_t *
xcb_get_selection_owner_reply (xcb_connection_t                  *c,
                               xcb_get_selection_owner_cookie_t   cookie  /**< */,
                               xcb_generic_error_t              **e);

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
xcb_convert_selection_checked (xcb_connection_t *c,
                               xcb_window_t      requestor,
                               xcb_atom_t        selection,
                               xcb_atom_t        target,
                               xcb_atom_t        property,
                               xcb_timestamp_t   time);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_convert_selection (xcb_connection_t *c,
                       xcb_window_t      requestor,
                       xcb_atom_t        selection,
                       xcb_atom_t        target,
                       xcb_atom_t        property,
                       xcb_timestamp_t   time);

/**
 * @brief send an event
 *
 * @param c The connection
 * @param propagate If \a propagate is true and no clients have selected any event on \a destination,
 * the destination is replaced with the closest ancestor of \a destination for
 * which some client has selected a type in \a event_mask and for which no
 * intervening window has that type in its do-not-propagate-mask. If no such
 * window exists or if the window is an ancestor of the focus window and
 * `InputFocus` was originally specified as the destination, the event is not sent
 * to any clients. Otherwise, the event is reported to every client selecting on
 * the final destination any of the types specified in \a event_mask.
 * @param destination The window to send this event to. Every client which selects any event within
 * \a event_mask on \a destination will get the event.
 * \n
 * The special value `XCB_SEND_EVENT_DEST_POINTER_WINDOW` refers to the window
 * that contains the mouse pointer.
 * \n
 * The special value `XCB_SEND_EVENT_DEST_ITEM_FOCUS` refers to the window which
 * has the keyboard focus.
 * @param event_mask Event_mask for determining which clients should receive the specified event.
 * See \a destination and \a propagate.
 * @param event The event to send to the specified \a destination.
 * @return A cookie
 *
 * Identifies the \a destination window, determines which clients should receive
 * the specified event and ignores any active grabs.
 * 
 * The \a event must be one of the core events or an event defined by an extension,
 * so that the X server can correctly byte-swap the contents as necessary. The
 * contents of \a event are otherwise unaltered and unchecked except for the
 * `send_event` field which is forced to 'true'.
 *
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
xcb_void_cookie_t
xcb_send_event_checked (xcb_connection_t *c,
                        uint8_t           propagate,
                        xcb_window_t      destination,
                        uint32_t          event_mask,
                        const char       *event);

/**
 * @brief send an event
 *
 * @param c The connection
 * @param propagate If \a propagate is true and no clients have selected any event on \a destination,
 * the destination is replaced with the closest ancestor of \a destination for
 * which some client has selected a type in \a event_mask and for which no
 * intervening window has that type in its do-not-propagate-mask. If no such
 * window exists or if the window is an ancestor of the focus window and
 * `InputFocus` was originally specified as the destination, the event is not sent
 * to any clients. Otherwise, the event is reported to every client selecting on
 * the final destination any of the types specified in \a event_mask.
 * @param destination The window to send this event to. Every client which selects any event within
 * \a event_mask on \a destination will get the event.
 * \n
 * The special value `XCB_SEND_EVENT_DEST_POINTER_WINDOW` refers to the window
 * that contains the mouse pointer.
 * \n
 * The special value `XCB_SEND_EVENT_DEST_ITEM_FOCUS` refers to the window which
 * has the keyboard focus.
 * @param event_mask Event_mask for determining which clients should receive the specified event.
 * See \a destination and \a propagate.
 * @param event The event to send to the specified \a destination.
 * @return A cookie
 *
 * Identifies the \a destination window, determines which clients should receive
 * the specified event and ignores any active grabs.
 * 
 * The \a event must be one of the core events or an event defined by an extension,
 * so that the X server can correctly byte-swap the contents as necessary. The
 * contents of \a event are otherwise unaltered and unchecked except for the
 * `send_event` field which is forced to 'true'.
 *
 */
xcb_void_cookie_t
xcb_send_event (xcb_connection_t *c,
                uint8_t           propagate,
                xcb_window_t      destination,
                uint32_t          event_mask,
                const char       *event);

/**
 * @brief Grab the pointer
 *
 * @param c The connection
 * @param owner_events If 1, the \a grab_window will still get the pointer events. If 0, events are not
 * reported to the \a grab_window.
 * @param grab_window Specifies the window on which the pointer should be grabbed.
 * @param event_mask Specifies which pointer events are reported to the client.
 * \n
 * TODO: which values?
 * @param pointer_mode A bitmask of #xcb_grab_mode_t values.
 * @param pointer_mode \n
 * @param keyboard_mode A bitmask of #xcb_grab_mode_t values.
 * @param keyboard_mode \n
 * @param confine_to Specifies the window to confine the pointer in (the user will not be able to
 * move the pointer out of that window).
 * \n
 * The special value `XCB_NONE` means don't confine the pointer.
 * @param cursor Specifies the cursor that should be displayed or `XCB_NONE` to not change the
 * cursor.
 * @param time The time argument allows you to avoid certain circumstances that come up if
 * applications take a long time to respond or if there are long network delays.
 * Consider a situation where you have two applications, both of which normally
 * grab the pointer when clicked on. If both applications specify the timestamp
 * from the event, the second application may wake up faster and successfully grab
 * the pointer before the first application. The first application then will get
 * an indication that the other application grabbed the pointer before its request
 * was processed.
 * \n
 * The special value `XCB_CURRENT_TIME` will be replaced with the current server
 * time.
 * @return A cookie
 *
 * Actively grabs control of the pointer. Further pointer events are reported only to the grabbing client. Overrides any active pointer grab by this client.
 *
 */
xcb_grab_pointer_cookie_t
xcb_grab_pointer (xcb_connection_t *c,
                  uint8_t           owner_events,
                  xcb_window_t      grab_window,
                  uint16_t          event_mask,
                  uint8_t           pointer_mode,
                  uint8_t           keyboard_mode,
                  xcb_window_t      confine_to,
                  xcb_cursor_t      cursor,
                  xcb_timestamp_t   time);

/**
 * @brief Grab the pointer
 *
 * @param c The connection
 * @param owner_events If 1, the \a grab_window will still get the pointer events. If 0, events are not
 * reported to the \a grab_window.
 * @param grab_window Specifies the window on which the pointer should be grabbed.
 * @param event_mask Specifies which pointer events are reported to the client.
 * \n
 * TODO: which values?
 * @param pointer_mode A bitmask of #xcb_grab_mode_t values.
 * @param pointer_mode \n
 * @param keyboard_mode A bitmask of #xcb_grab_mode_t values.
 * @param keyboard_mode \n
 * @param confine_to Specifies the window to confine the pointer in (the user will not be able to
 * move the pointer out of that window).
 * \n
 * The special value `XCB_NONE` means don't confine the pointer.
 * @param cursor Specifies the cursor that should be displayed or `XCB_NONE` to not change the
 * cursor.
 * @param time The time argument allows you to avoid certain circumstances that come up if
 * applications take a long time to respond or if there are long network delays.
 * Consider a situation where you have two applications, both of which normally
 * grab the pointer when clicked on. If both applications specify the timestamp
 * from the event, the second application may wake up faster and successfully grab
 * the pointer before the first application. The first application then will get
 * an indication that the other application grabbed the pointer before its request
 * was processed.
 * \n
 * The special value `XCB_CURRENT_TIME` will be replaced with the current server
 * time.
 * @return A cookie
 *
 * Actively grabs control of the pointer. Further pointer events are reported only to the grabbing client. Overrides any active pointer grab by this client.
 *
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
xcb_grab_pointer_cookie_t
xcb_grab_pointer_unchecked (xcb_connection_t *c,
                            uint8_t           owner_events,
                            xcb_window_t      grab_window,
                            uint16_t          event_mask,
                            uint8_t           pointer_mode,
                            uint8_t           keyboard_mode,
                            xcb_window_t      confine_to,
                            xcb_cursor_t      cursor,
                            xcb_timestamp_t   time);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_grab_pointer_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_grab_pointer_reply_t *
xcb_grab_pointer_reply (xcb_connection_t           *c,
                        xcb_grab_pointer_cookie_t   cookie  /**< */,
                        xcb_generic_error_t       **e);

/**
 * @brief release the pointer
 *
 * @param c The connection
 * @param time Timestamp to avoid race conditions when running X over the network.
 * \n
 * The pointer will not be released if \a time is earlier than the
 * last-pointer-grab time or later than the current X server time.
 * @return A cookie
 *
 * Releases the pointer and any queued events if you actively grabbed the pointer
 * before using `xcb_grab_pointer`, `xcb_grab_button` or within a normal button
 * press.
 * 
 * EnterNotify and LeaveNotify events are generated.
 *
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
xcb_void_cookie_t
xcb_ungrab_pointer_checked (xcb_connection_t *c,
                            xcb_timestamp_t   time);

/**
 * @brief release the pointer
 *
 * @param c The connection
 * @param time Timestamp to avoid race conditions when running X over the network.
 * \n
 * The pointer will not be released if \a time is earlier than the
 * last-pointer-grab time or later than the current X server time.
 * @return A cookie
 *
 * Releases the pointer and any queued events if you actively grabbed the pointer
 * before using `xcb_grab_pointer`, `xcb_grab_button` or within a normal button
 * press.
 * 
 * EnterNotify and LeaveNotify events are generated.
 *
 */
xcb_void_cookie_t
xcb_ungrab_pointer (xcb_connection_t *c,
                    xcb_timestamp_t   time);

/**
 * @brief Grab pointer button(s)
 *
 * @param c The connection
 * @param owner_events If 1, the \a grab_window will still get the pointer events. If 0, events are not
 * reported to the \a grab_window.
 * @param grab_window Specifies the window on which the pointer should be grabbed.
 * @param event_mask Specifies which pointer events are reported to the client.
 * \n
 * TODO: which values?
 * @param pointer_mode A bitmask of #xcb_grab_mode_t values.
 * @param pointer_mode \n
 * @param keyboard_mode A bitmask of #xcb_grab_mode_t values.
 * @param keyboard_mode \n
 * @param confine_to Specifies the window to confine the pointer in (the user will not be able to
 * move the pointer out of that window).
 * \n
 * The special value `XCB_NONE` means don't confine the pointer.
 * @param cursor Specifies the cursor that should be displayed or `XCB_NONE` to not change the
 * cursor.
 * @param button A bitmask of #xcb_button_index_t values.
 * @param button \n
 * @param modifiers The modifiers to grab.
 * \n
 * Using the special value `XCB_MOD_MASK_ANY` means grab the pointer with all
 * possible modifier combinations.
 * @return A cookie
 *
 * This request establishes a passive grab. The pointer is actively grabbed as
 * described in GrabPointer, the last-pointer-grab time is set to the time at
 * which the button was pressed (as transmitted in the ButtonPress event), and the
 * ButtonPress event is reported if all of the following conditions are true:
 * 
 * The pointer is not grabbed and the specified button is logically pressed when
 * the specified modifier keys are logically down, and no other buttons or
 * modifier keys are logically down.
 * 
 * The grab-window contains the pointer.
 * 
 * The confine-to window (if any) is viewable.
 * 
 * A passive grab on the same button/key combination does not exist on any
 * ancestor of grab-window.
 * 
 * The interpretation of the remaining arguments is the same as for GrabPointer.
 * The active grab is terminated automatically when the logical state of the
 * pointer has all buttons released, independent of the logical state of modifier
 * keys. Note that the logical state of a device (as seen by means of the
 * protocol) may lag the physical state if device event processing is frozen. This
 * request overrides all previous passive grabs by the same client on the same
 * button/key combinations on the same window. A modifier of AnyModifier is
 * equivalent to issuing the request for all possible modifier combinations
 * (including the combination of no modifiers). It is not required that all
 * specified modifiers have currently assigned keycodes. A button of AnyButton is
 * equivalent to issuing the request for all possible buttons. Otherwise, it is
 * not required that the button specified currently be assigned to a physical
 * button.
 * 
 * An Access error is generated if some other client has already issued a
 * GrabButton request with the same button/key combination on the same window.
 * When using AnyModifier or AnyButton, the request fails completely (no grabs are
 * established), and an Access error is generated if there is a conflicting grab
 * for any combination. The request has no effect on an active grab.
 *
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
xcb_void_cookie_t
xcb_grab_button_checked (xcb_connection_t *c,
                         uint8_t           owner_events,
                         xcb_window_t      grab_window,
                         uint16_t          event_mask,
                         uint8_t           pointer_mode,
                         uint8_t           keyboard_mode,
                         xcb_window_t      confine_to,
                         xcb_cursor_t      cursor,
                         uint8_t           button,
                         uint16_t          modifiers);

/**
 * @brief Grab pointer button(s)
 *
 * @param c The connection
 * @param owner_events If 1, the \a grab_window will still get the pointer events. If 0, events are not
 * reported to the \a grab_window.
 * @param grab_window Specifies the window on which the pointer should be grabbed.
 * @param event_mask Specifies which pointer events are reported to the client.
 * \n
 * TODO: which values?
 * @param pointer_mode A bitmask of #xcb_grab_mode_t values.
 * @param pointer_mode \n
 * @param keyboard_mode A bitmask of #xcb_grab_mode_t values.
 * @param keyboard_mode \n
 * @param confine_to Specifies the window to confine the pointer in (the user will not be able to
 * move the pointer out of that window).
 * \n
 * The special value `XCB_NONE` means don't confine the pointer.
 * @param cursor Specifies the cursor that should be displayed or `XCB_NONE` to not change the
 * cursor.
 * @param button A bitmask of #xcb_button_index_t values.
 * @param button \n
 * @param modifiers The modifiers to grab.
 * \n
 * Using the special value `XCB_MOD_MASK_ANY` means grab the pointer with all
 * possible modifier combinations.
 * @return A cookie
 *
 * This request establishes a passive grab. The pointer is actively grabbed as
 * described in GrabPointer, the last-pointer-grab time is set to the time at
 * which the button was pressed (as transmitted in the ButtonPress event), and the
 * ButtonPress event is reported if all of the following conditions are true:
 * 
 * The pointer is not grabbed and the specified button is logically pressed when
 * the specified modifier keys are logically down, and no other buttons or
 * modifier keys are logically down.
 * 
 * The grab-window contains the pointer.
 * 
 * The confine-to window (if any) is viewable.
 * 
 * A passive grab on the same button/key combination does not exist on any
 * ancestor of grab-window.
 * 
 * The interpretation of the remaining arguments is the same as for GrabPointer.
 * The active grab is terminated automatically when the logical state of the
 * pointer has all buttons released, independent of the logical state of modifier
 * keys. Note that the logical state of a device (as seen by means of the
 * protocol) may lag the physical state if device event processing is frozen. This
 * request overrides all previous passive grabs by the same client on the same
 * button/key combinations on the same window. A modifier of AnyModifier is
 * equivalent to issuing the request for all possible modifier combinations
 * (including the combination of no modifiers). It is not required that all
 * specified modifiers have currently assigned keycodes. A button of AnyButton is
 * equivalent to issuing the request for all possible buttons. Otherwise, it is
 * not required that the button specified currently be assigned to a physical
 * button.
 * 
 * An Access error is generated if some other client has already issued a
 * GrabButton request with the same button/key combination on the same window.
 * When using AnyModifier or AnyButton, the request fails completely (no grabs are
 * established), and an Access error is generated if there is a conflicting grab
 * for any combination. The request has no effect on an active grab.
 *
 */
xcb_void_cookie_t
xcb_grab_button (xcb_connection_t *c,
                 uint8_t           owner_events,
                 xcb_window_t      grab_window,
                 uint16_t          event_mask,
                 uint8_t           pointer_mode,
                 uint8_t           keyboard_mode,
                 xcb_window_t      confine_to,
                 xcb_cursor_t      cursor,
                 uint8_t           button,
                 uint16_t          modifiers);

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
xcb_ungrab_button_checked (xcb_connection_t *c,
                           uint8_t           button,
                           xcb_window_t      grab_window,
                           uint16_t          modifiers);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_ungrab_button (xcb_connection_t *c,
                   uint8_t           button,
                   xcb_window_t      grab_window,
                   uint16_t          modifiers);

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
xcb_change_active_pointer_grab_checked (xcb_connection_t *c,
                                        xcb_cursor_t      cursor,
                                        xcb_timestamp_t   time,
                                        uint16_t          event_mask);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_change_active_pointer_grab (xcb_connection_t *c,
                                xcb_cursor_t      cursor,
                                xcb_timestamp_t   time,
                                uint16_t          event_mask);

/**
 * @brief Grab the keyboard
 *
 * @param c The connection
 * @param owner_events If 1, the \a grab_window will still get the pointer events. If 0, events are not
 * reported to the \a grab_window.
 * @param grab_window Specifies the window on which the pointer should be grabbed.
 * @param time Timestamp to avoid race conditions when running X over the network.
 * \n
 * The special value `XCB_CURRENT_TIME` will be replaced with the current server
 * time.
 * @param pointer_mode A bitmask of #xcb_grab_mode_t values.
 * @param pointer_mode \n
 * @param keyboard_mode A bitmask of #xcb_grab_mode_t values.
 * @param keyboard_mode \n
 * @return A cookie
 *
 * Actively grabs control of the keyboard and generates FocusIn and FocusOut
 * events. Further key events are reported only to the grabbing client.
 * 
 * Any active keyboard grab by this client is overridden. If the keyboard is
 * actively grabbed by some other client, `AlreadyGrabbed` is returned. If
 * \a grab_window is not viewable, `GrabNotViewable` is returned. If the keyboard
 * is frozen by an active grab of another client, `GrabFrozen` is returned. If the
 * specified \a time is earlier than the last-keyboard-grab time or later than the
 * current X server time, `GrabInvalidTime` is returned. Otherwise, the
 * last-keyboard-grab time is set to the specified time.
 *
 */
xcb_grab_keyboard_cookie_t
xcb_grab_keyboard (xcb_connection_t *c,
                   uint8_t           owner_events,
                   xcb_window_t      grab_window,
                   xcb_timestamp_t   time,
                   uint8_t           pointer_mode,
                   uint8_t           keyboard_mode);

/**
 * @brief Grab the keyboard
 *
 * @param c The connection
 * @param owner_events If 1, the \a grab_window will still get the pointer events. If 0, events are not
 * reported to the \a grab_window.
 * @param grab_window Specifies the window on which the pointer should be grabbed.
 * @param time Timestamp to avoid race conditions when running X over the network.
 * \n
 * The special value `XCB_CURRENT_TIME` will be replaced with the current server
 * time.
 * @param pointer_mode A bitmask of #xcb_grab_mode_t values.
 * @param pointer_mode \n
 * @param keyboard_mode A bitmask of #xcb_grab_mode_t values.
 * @param keyboard_mode \n
 * @return A cookie
 *
 * Actively grabs control of the keyboard and generates FocusIn and FocusOut
 * events. Further key events are reported only to the grabbing client.
 * 
 * Any active keyboard grab by this client is overridden. If the keyboard is
 * actively grabbed by some other client, `AlreadyGrabbed` is returned. If
 * \a grab_window is not viewable, `GrabNotViewable` is returned. If the keyboard
 * is frozen by an active grab of another client, `GrabFrozen` is returned. If the
 * specified \a time is earlier than the last-keyboard-grab time or later than the
 * current X server time, `GrabInvalidTime` is returned. Otherwise, the
 * last-keyboard-grab time is set to the specified time.
 *
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
xcb_grab_keyboard_cookie_t
xcb_grab_keyboard_unchecked (xcb_connection_t *c,
                             uint8_t           owner_events,
                             xcb_window_t      grab_window,
                             xcb_timestamp_t   time,
                             uint8_t           pointer_mode,
                             uint8_t           keyboard_mode);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_grab_keyboard_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_grab_keyboard_reply_t *
xcb_grab_keyboard_reply (xcb_connection_t            *c,
                         xcb_grab_keyboard_cookie_t   cookie  /**< */,
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
xcb_ungrab_keyboard_checked (xcb_connection_t *c,
                             xcb_timestamp_t   time);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_ungrab_keyboard (xcb_connection_t *c,
                     xcb_timestamp_t   time);

/**
 * @brief Grab keyboard key(s)
 *
 * @param c The connection
 * @param owner_events If 1, the \a grab_window will still get the pointer events. If 0, events are not
 * reported to the \a grab_window.
 * @param grab_window Specifies the window on which the pointer should be grabbed.
 * @param modifiers The modifiers to grab.
 * \n
 * Using the special value `XCB_MOD_MASK_ANY` means grab the pointer with all
 * possible modifier combinations.
 * @param key The keycode of the key to grab.
 * \n
 * The special value `XCB_GRAB_ANY` means grab any key.
 * @param pointer_mode A bitmask of #xcb_grab_mode_t values.
 * @param pointer_mode \n
 * @param keyboard_mode A bitmask of #xcb_grab_mode_t values.
 * @param keyboard_mode \n
 * @return A cookie
 *
 * Establishes a passive grab on the keyboard. In the future, the keyboard is
 * actively grabbed (as for `GrabKeyboard`), the last-keyboard-grab time is set to
 * the time at which the key was pressed (as transmitted in the KeyPress event),
 * and the KeyPress event is reported if all of the following conditions are true:
 * 
 * The keyboard is not grabbed and the specified key (which can itself be a
 * modifier key) is logically pressed when the specified modifier keys are
 * logically down, and no other modifier keys are logically down.
 * 
 * Either the grab_window is an ancestor of (or is) the focus window, or the
 * grab_window is a descendant of the focus window and contains the pointer.
 * 
 * A passive grab on the same key combination does not exist on any ancestor of
 * grab_window.
 * 
 * The interpretation of the remaining arguments is as for XGrabKeyboard.  The active grab is terminated
 * automatically when the logical state of the keyboard has the specified key released (independent of the
 * logical state of the modifier keys), at which point a KeyRelease event is reported to the grabbing window.
 * 
 * Note that the logical state of a device (as seen by client applications) may lag the physical state if
 * device event processing is frozen.
 * 
 * A modifiers argument of AnyModifier is equivalent to issuing the request for all possible modifier combinations (including the combination of no modifiers).  It is not required that all modifiers specified
 * have currently assigned KeyCodes.  A keycode argument of AnyKey is equivalent to issuing the request for
 * all possible KeyCodes.  Otherwise, the specified keycode must be in the range specified by min_keycode
 * and max_keycode in the connection setup, or a BadValue error results.
 * 
 * If some other client has issued a XGrabKey with the same key combination on the same window, a BadAccess
 * error results.  When using AnyModifier or AnyKey, the request fails completely, and a BadAccess error
 * results (no grabs are established) if there is a conflicting grab for any combination.
 *
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
xcb_void_cookie_t
xcb_grab_key_checked (xcb_connection_t *c,
                      uint8_t           owner_events,
                      xcb_window_t      grab_window,
                      uint16_t          modifiers,
                      xcb_keycode_t     key,
                      uint8_t           pointer_mode,
                      uint8_t           keyboard_mode);

/**
 * @brief Grab keyboard key(s)
 *
 * @param c The connection
 * @param owner_events If 1, the \a grab_window will still get the pointer events. If 0, events are not
 * reported to the \a grab_window.
 * @param grab_window Specifies the window on which the pointer should be grabbed.
 * @param modifiers The modifiers to grab.
 * \n
 * Using the special value `XCB_MOD_MASK_ANY` means grab the pointer with all
 * possible modifier combinations.
 * @param key The keycode of the key to grab.
 * \n
 * The special value `XCB_GRAB_ANY` means grab any key.
 * @param pointer_mode A bitmask of #xcb_grab_mode_t values.
 * @param pointer_mode \n
 * @param keyboard_mode A bitmask of #xcb_grab_mode_t values.
 * @param keyboard_mode \n
 * @return A cookie
 *
 * Establishes a passive grab on the keyboard. In the future, the keyboard is
 * actively grabbed (as for `GrabKeyboard`), the last-keyboard-grab time is set to
 * the time at which the key was pressed (as transmitted in the KeyPress event),
 * and the KeyPress event is reported if all of the following conditions are true:
 * 
 * The keyboard is not grabbed and the specified key (which can itself be a
 * modifier key) is logically pressed when the specified modifier keys are
 * logically down, and no other modifier keys are logically down.
 * 
 * Either the grab_window is an ancestor of (or is) the focus window, or the
 * grab_window is a descendant of the focus window and contains the pointer.
 * 
 * A passive grab on the same key combination does not exist on any ancestor of
 * grab_window.
 * 
 * The interpretation of the remaining arguments is as for XGrabKeyboard.  The active grab is terminated
 * automatically when the logical state of the keyboard has the specified key released (independent of the
 * logical state of the modifier keys), at which point a KeyRelease event is reported to the grabbing window.
 * 
 * Note that the logical state of a device (as seen by client applications) may lag the physical state if
 * device event processing is frozen.
 * 
 * A modifiers argument of AnyModifier is equivalent to issuing the request for all possible modifier combinations (including the combination of no modifiers).  It is not required that all modifiers specified
 * have currently assigned KeyCodes.  A keycode argument of AnyKey is equivalent to issuing the request for
 * all possible KeyCodes.  Otherwise, the specified keycode must be in the range specified by min_keycode
 * and max_keycode in the connection setup, or a BadValue error results.
 * 
 * If some other client has issued a XGrabKey with the same key combination on the same window, a BadAccess
 * error results.  When using AnyModifier or AnyKey, the request fails completely, and a BadAccess error
 * results (no grabs are established) if there is a conflicting grab for any combination.
 *
 */
xcb_void_cookie_t
xcb_grab_key (xcb_connection_t *c,
              uint8_t           owner_events,
              xcb_window_t      grab_window,
              uint16_t          modifiers,
              xcb_keycode_t     key,
              uint8_t           pointer_mode,
              uint8_t           keyboard_mode);

/**
 * @brief release a key combination
 *
 * @param c The connection
 * @param key The keycode of the specified key combination.
 * \n
 * Using the special value `XCB_GRAB_ANY` means releasing all possible key codes.
 * @param grab_window The window on which the grabbed key combination will be released.
 * @param modifiers The modifiers of the specified key combination.
 * \n
 * Using the special value `XCB_MOD_MASK_ANY` means releasing the key combination
 * with every possible modifier combination.
 * @return A cookie
 *
 * Releases the key combination on \a grab_window if you grabbed it using
 * `xcb_grab_key` before.
 *
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
xcb_void_cookie_t
xcb_ungrab_key_checked (xcb_connection_t *c,
                        xcb_keycode_t     key,
                        xcb_window_t      grab_window,
                        uint16_t          modifiers);

/**
 * @brief release a key combination
 *
 * @param c The connection
 * @param key The keycode of the specified key combination.
 * \n
 * Using the special value `XCB_GRAB_ANY` means releasing all possible key codes.
 * @param grab_window The window on which the grabbed key combination will be released.
 * @param modifiers The modifiers of the specified key combination.
 * \n
 * Using the special value `XCB_MOD_MASK_ANY` means releasing the key combination
 * with every possible modifier combination.
 * @return A cookie
 *
 * Releases the key combination on \a grab_window if you grabbed it using
 * `xcb_grab_key` before.
 *
 */
xcb_void_cookie_t
xcb_ungrab_key (xcb_connection_t *c,
                xcb_keycode_t     key,
                xcb_window_t      grab_window,
                uint16_t          modifiers);

/**
 * @brief release queued events
 *
 * @param c The connection
 * @param mode A bitmask of #xcb_allow_t values.
 * @param mode \n
 * @param time Timestamp to avoid race conditions when running X over the network.
 * \n
 * The special value `XCB_CURRENT_TIME` will be replaced with the current server
 * time.
 * @return A cookie
 *
 * Releases queued events if the client has caused a device (pointer/keyboard) to
 * freeze due to grabbing it actively. This request has no effect if \a time is
 * earlier than the last-grab time of the most recent active grab for this client
 * or if \a time is later than the current X server time.
 *
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
xcb_void_cookie_t
xcb_allow_events_checked (xcb_connection_t *c,
                          uint8_t           mode,
                          xcb_timestamp_t   time);

/**
 * @brief release queued events
 *
 * @param c The connection
 * @param mode A bitmask of #xcb_allow_t values.
 * @param mode \n
 * @param time Timestamp to avoid race conditions when running X over the network.
 * \n
 * The special value `XCB_CURRENT_TIME` will be replaced with the current server
 * time.
 * @return A cookie
 *
 * Releases queued events if the client has caused a device (pointer/keyboard) to
 * freeze due to grabbing it actively. This request has no effect if \a time is
 * earlier than the last-grab time of the most recent active grab for this client
 * or if \a time is later than the current X server time.
 *
 */
xcb_void_cookie_t
xcb_allow_events (xcb_connection_t *c,
                  uint8_t           mode,
                  xcb_timestamp_t   time);

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
xcb_grab_server_checked (xcb_connection_t *c);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_grab_server (xcb_connection_t *c);

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
xcb_ungrab_server_checked (xcb_connection_t *c);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_ungrab_server (xcb_connection_t *c);

/**
 * @brief get pointer coordinates
 *
 * @param c The connection
 * @param window A window to check if the pointer is on the same screen as \a window (see the
 * `same_screen` field in the reply).
 * @return A cookie
 *
 * Gets the root window the pointer is logically on and the pointer coordinates
 * relative to the root window's origin.
 *
 */
xcb_query_pointer_cookie_t
xcb_query_pointer (xcb_connection_t *c,
                   xcb_window_t      window);

/**
 * @brief get pointer coordinates
 *
 * @param c The connection
 * @param window A window to check if the pointer is on the same screen as \a window (see the
 * `same_screen` field in the reply).
 * @return A cookie
 *
 * Gets the root window the pointer is logically on and the pointer coordinates
 * relative to the root window's origin.
 *
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
xcb_query_pointer_cookie_t
xcb_query_pointer_unchecked (xcb_connection_t *c,
                             xcb_window_t      window);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_query_pointer_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_query_pointer_reply_t *
xcb_query_pointer_reply (xcb_connection_t            *c,
                         xcb_query_pointer_cookie_t   cookie  /**< */,
                         xcb_generic_error_t        **e);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_timecoord_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_timecoord_t)
 */
void
xcb_timecoord_next (xcb_timecoord_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_timecoord_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_timecoord_end (xcb_timecoord_iterator_t i);

int
xcb_get_motion_events_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_get_motion_events_cookie_t
xcb_get_motion_events (xcb_connection_t *c,
                       xcb_window_t      window,
                       xcb_timestamp_t   start,
                       xcb_timestamp_t   stop);

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
xcb_get_motion_events_cookie_t
xcb_get_motion_events_unchecked (xcb_connection_t *c,
                                 xcb_window_t      window,
                                 xcb_timestamp_t   start,
                                 xcb_timestamp_t   stop);

xcb_timecoord_t *
xcb_get_motion_events_events (const xcb_get_motion_events_reply_t *R);

int
xcb_get_motion_events_events_length (const xcb_get_motion_events_reply_t *R);

xcb_timecoord_iterator_t
xcb_get_motion_events_events_iterator (const xcb_get_motion_events_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_get_motion_events_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_get_motion_events_reply_t *
xcb_get_motion_events_reply (xcb_connection_t                *c,
                             xcb_get_motion_events_cookie_t   cookie  /**< */,
                             xcb_generic_error_t            **e);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_translate_coordinates_cookie_t
xcb_translate_coordinates (xcb_connection_t *c,
                           xcb_window_t      src_window,
                           xcb_window_t      dst_window,
                           int16_t           src_x,
                           int16_t           src_y);

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
xcb_translate_coordinates_cookie_t
xcb_translate_coordinates_unchecked (xcb_connection_t *c,
                                     xcb_window_t      src_window,
                                     xcb_window_t      dst_window,
                                     int16_t           src_x,
                                     int16_t           src_y);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_translate_coordinates_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_translate_coordinates_reply_t *
xcb_translate_coordinates_reply (xcb_connection_t                    *c,
                                 xcb_translate_coordinates_cookie_t   cookie  /**< */,
                                 xcb_generic_error_t                **e);

/**
 * @brief move mouse pointer
 *
 * @param c The connection
 * @param src_window If \a src_window is not `XCB_NONE` (TODO), the move will only take place if the
 * pointer is inside \a src_window and within the rectangle specified by (\a src_x,
 * \a src_y, \a src_width, \a src_height). The rectangle coordinates are relative to
 * \a src_window.
 * @param dst_window If \a dst_window is not `XCB_NONE` (TODO), the pointer will be moved to the
 * offsets (\a dst_x, \a dst_y) relative to \a dst_window. If \a dst_window is
 * `XCB_NONE` (TODO), the pointer will be moved by the offsets (\a dst_x, \a dst_y)
 * relative to the current position of the pointer.
 * @return A cookie
 *
 * Moves the mouse pointer to the specified position.
 * 
 * If \a src_window is not `XCB_NONE` (TODO), the move will only take place if the
 * pointer is inside \a src_window and within the rectangle specified by (\a src_x,
 * \a src_y, \a src_width, \a src_height). The rectangle coordinates are relative to
 * \a src_window.
 * 
 * If \a dst_window is not `XCB_NONE` (TODO), the pointer will be moved to the
 * offsets (\a dst_x, \a dst_y) relative to \a dst_window. If \a dst_window is
 * `XCB_NONE` (TODO), the pointer will be moved by the offsets (\a dst_x, \a dst_y)
 * relative to the current position of the pointer.
 *
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
xcb_void_cookie_t
xcb_warp_pointer_checked (xcb_connection_t *c,
                          xcb_window_t      src_window,
                          xcb_window_t      dst_window,
                          int16_t           src_x,
                          int16_t           src_y,
                          uint16_t          src_width,
                          uint16_t          src_height,
                          int16_t           dst_x,
                          int16_t           dst_y);

/**
 * @brief move mouse pointer
 *
 * @param c The connection
 * @param src_window If \a src_window is not `XCB_NONE` (TODO), the move will only take place if the
 * pointer is inside \a src_window and within the rectangle specified by (\a src_x,
 * \a src_y, \a src_width, \a src_height). The rectangle coordinates are relative to
 * \a src_window.
 * @param dst_window If \a dst_window is not `XCB_NONE` (TODO), the pointer will be moved to the
 * offsets (\a dst_x, \a dst_y) relative to \a dst_window. If \a dst_window is
 * `XCB_NONE` (TODO), the pointer will be moved by the offsets (\a dst_x, \a dst_y)
 * relative to the current position of the pointer.
 * @return A cookie
 *
 * Moves the mouse pointer to the specified position.
 * 
 * If \a src_window is not `XCB_NONE` (TODO), the move will only take place if the
 * pointer is inside \a src_window and within the rectangle specified by (\a src_x,
 * \a src_y, \a src_width, \a src_height). The rectangle coordinates are relative to
 * \a src_window.
 * 
 * If \a dst_window is not `XCB_NONE` (TODO), the pointer will be moved to the
 * offsets (\a dst_x, \a dst_y) relative to \a dst_window. If \a dst_window is
 * `XCB_NONE` (TODO), the pointer will be moved by the offsets (\a dst_x, \a dst_y)
 * relative to the current position of the pointer.
 *
 */
xcb_void_cookie_t
xcb_warp_pointer (xcb_connection_t *c,
                  xcb_window_t      src_window,
                  xcb_window_t      dst_window,
                  int16_t           src_x,
                  int16_t           src_y,
                  uint16_t          src_width,
                  uint16_t          src_height,
                  int16_t           dst_x,
                  int16_t           dst_y);

/**
 * @brief Sets input focus
 *
 * @param c The connection
 * @param revert_to A bitmask of #xcb_input_focus_t values.
 * @param revert_to Specifies what happens when the \a focus window becomes unviewable (if \a focus
 * is neither `XCB_NONE` nor `XCB_POINTER_ROOT`).
 * @param focus The window to focus. All keyboard events will be reported to this window. The
 * window must be viewable (TODO), or a `xcb_match_error_t` occurs (TODO).
 * \n
 * If \a focus is `XCB_NONE` (TODO), all keyboard events are
 * discarded until a new focus window is set.
 * \n
 * If \a focus is `XCB_POINTER_ROOT` (TODO), focus is on the root window of the
 * screen on which the pointer is on currently.
 * @param time Timestamp to avoid race conditions when running X over the network.
 * \n
 * The special value `XCB_CURRENT_TIME` will be replaced with the current server
 * time.
 * @return A cookie
 *
 * Changes the input focus and the last-focus-change time. If the specified \a time
 * is earlier than the current last-focus-change time, the request is ignored (to
 * avoid race conditions when running X over the network).
 * 
 * A FocusIn and FocusOut event is generated when focus is changed.
 *
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
xcb_void_cookie_t
xcb_set_input_focus_checked (xcb_connection_t *c,
                             uint8_t           revert_to,
                             xcb_window_t      focus,
                             xcb_timestamp_t   time);

/**
 * @brief Sets input focus
 *
 * @param c The connection
 * @param revert_to A bitmask of #xcb_input_focus_t values.
 * @param revert_to Specifies what happens when the \a focus window becomes unviewable (if \a focus
 * is neither `XCB_NONE` nor `XCB_POINTER_ROOT`).
 * @param focus The window to focus. All keyboard events will be reported to this window. The
 * window must be viewable (TODO), or a `xcb_match_error_t` occurs (TODO).
 * \n
 * If \a focus is `XCB_NONE` (TODO), all keyboard events are
 * discarded until a new focus window is set.
 * \n
 * If \a focus is `XCB_POINTER_ROOT` (TODO), focus is on the root window of the
 * screen on which the pointer is on currently.
 * @param time Timestamp to avoid race conditions when running X over the network.
 * \n
 * The special value `XCB_CURRENT_TIME` will be replaced with the current server
 * time.
 * @return A cookie
 *
 * Changes the input focus and the last-focus-change time. If the specified \a time
 * is earlier than the current last-focus-change time, the request is ignored (to
 * avoid race conditions when running X over the network).
 * 
 * A FocusIn and FocusOut event is generated when focus is changed.
 *
 */
xcb_void_cookie_t
xcb_set_input_focus (xcb_connection_t *c,
                     uint8_t           revert_to,
                     xcb_window_t      focus,
                     xcb_timestamp_t   time);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_get_input_focus_cookie_t
xcb_get_input_focus (xcb_connection_t *c);

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
xcb_get_input_focus_cookie_t
xcb_get_input_focus_unchecked (xcb_connection_t *c);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_get_input_focus_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_get_input_focus_reply_t *
xcb_get_input_focus_reply (xcb_connection_t              *c,
                           xcb_get_input_focus_cookie_t   cookie  /**< */,
                           xcb_generic_error_t          **e);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_query_keymap_cookie_t
xcb_query_keymap (xcb_connection_t *c);

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
xcb_query_keymap_cookie_t
xcb_query_keymap_unchecked (xcb_connection_t *c);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_query_keymap_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_query_keymap_reply_t *
xcb_query_keymap_reply (xcb_connection_t           *c,
                        xcb_query_keymap_cookie_t   cookie  /**< */,
                        xcb_generic_error_t       **e);

int
xcb_open_font_sizeof (const void  *_buffer);

/**
 * @brief opens a font
 *
 * @param c The connection
 * @param fid The ID with which you will refer to the font, created by `xcb_generate_id`.
 * @param name_len Length (in bytes) of \a name.
 * @param name A pattern describing an X core font.
 * @return A cookie
 *
 * Opens any X core font matching the given \a name (for example "-misc-fixed-*").
 * 
 * Note that X core fonts are deprecated (but still supported) in favor of
 * client-side rendering using Xft.
 *
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
xcb_void_cookie_t
xcb_open_font_checked (xcb_connection_t *c,
                       xcb_font_t        fid,
                       uint16_t          name_len,
                       const char       *name);

/**
 * @brief opens a font
 *
 * @param c The connection
 * @param fid The ID with which you will refer to the font, created by `xcb_generate_id`.
 * @param name_len Length (in bytes) of \a name.
 * @param name A pattern describing an X core font.
 * @return A cookie
 *
 * Opens any X core font matching the given \a name (for example "-misc-fixed-*").
 * 
 * Note that X core fonts are deprecated (but still supported) in favor of
 * client-side rendering using Xft.
 *
 */
xcb_void_cookie_t
xcb_open_font (xcb_connection_t *c,
               xcb_font_t        fid,
               uint16_t          name_len,
               const char       *name);

char *
xcb_open_font_name (const xcb_open_font_request_t *R);

int
xcb_open_font_name_length (const xcb_open_font_request_t *R);

xcb_generic_iterator_t
xcb_open_font_name_end (const xcb_open_font_request_t *R);

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
xcb_close_font_checked (xcb_connection_t *c,
                        xcb_font_t        font);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_close_font (xcb_connection_t *c,
                xcb_font_t        font);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_fontprop_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_fontprop_t)
 */
void
xcb_fontprop_next (xcb_fontprop_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_fontprop_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_fontprop_end (xcb_fontprop_iterator_t i);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_charinfo_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_charinfo_t)
 */
void
xcb_charinfo_next (xcb_charinfo_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_charinfo_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_charinfo_end (xcb_charinfo_iterator_t i);

int
xcb_query_font_sizeof (const void  *_buffer);

/**
 * @brief query font metrics
 *
 * @param c The connection
 * @param font The fontable (Font or Graphics Context) to query.
 * @return A cookie
 *
 * Queries information associated with the font.
 *
 */
xcb_query_font_cookie_t
xcb_query_font (xcb_connection_t *c,
                xcb_fontable_t    font);

/**
 * @brief query font metrics
 *
 * @param c The connection
 * @param font The fontable (Font or Graphics Context) to query.
 * @return A cookie
 *
 * Queries information associated with the font.
 *
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
xcb_query_font_cookie_t
xcb_query_font_unchecked (xcb_connection_t *c,
                          xcb_fontable_t    font);

xcb_fontprop_t *
xcb_query_font_properties (const xcb_query_font_reply_t *R);

int
xcb_query_font_properties_length (const xcb_query_font_reply_t *R);

xcb_fontprop_iterator_t
xcb_query_font_properties_iterator (const xcb_query_font_reply_t *R);

xcb_charinfo_t *
xcb_query_font_char_infos (const xcb_query_font_reply_t *R);

int
xcb_query_font_char_infos_length (const xcb_query_font_reply_t *R);

xcb_charinfo_iterator_t
xcb_query_font_char_infos_iterator (const xcb_query_font_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_query_font_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_query_font_reply_t *
xcb_query_font_reply (xcb_connection_t         *c,
                      xcb_query_font_cookie_t   cookie  /**< */,
                      xcb_generic_error_t     **e);

int
xcb_query_text_extents_sizeof (const void  *_buffer,
                               uint32_t     string_len);

/**
 * @brief get text extents
 *
 * @param c The connection
 * @param font The \a font to calculate text extents in. You can also pass a graphics context.
 * @param string_len The number of characters in \a string.
 * @param string The text to get text extents for.
 * @return A cookie
 *
 * Query text extents from the X11 server. This request returns the bounding box
 * of the specified 16-bit character string in the specified \a font or the font
 * contained in the specified graphics context.
 * 
 * `font_ascent` is set to the maximum of the ascent metrics of all characters in
 * the string. `font_descent` is set to the maximum of the descent metrics.
 * `overall_width` is set to the sum of the character-width metrics of all
 * characters in the string. For each character in the string, let W be the sum of
 * the character-width metrics of all characters preceding it in the string. Let L
 * be the left-side-bearing metric of the character plus W. Let R be the
 * right-side-bearing metric of the character plus W. The lbearing member is set
 * to the minimum L of all characters in the string. The rbearing member is set to
 * the maximum R.
 * 
 * For fonts defined with linear indexing rather than 2-byte matrix indexing, each
 * `xcb_char2b_t` structure is interpreted as a 16-bit number with byte1 as the
 * most significant byte. If the font has no defined default character, undefined
 * characters in the string are taken to have all zero metrics.
 * 
 * Characters with all zero metrics are ignored. If the font has no defined
 * default_char, the undefined characters in the string are also ignored.
 *
 */
xcb_query_text_extents_cookie_t
xcb_query_text_extents (xcb_connection_t   *c,
                        xcb_fontable_t      font,
                        uint32_t            string_len,
                        const xcb_char2b_t *string);

/**
 * @brief get text extents
 *
 * @param c The connection
 * @param font The \a font to calculate text extents in. You can also pass a graphics context.
 * @param string_len The number of characters in \a string.
 * @param string The text to get text extents for.
 * @return A cookie
 *
 * Query text extents from the X11 server. This request returns the bounding box
 * of the specified 16-bit character string in the specified \a font or the font
 * contained in the specified graphics context.
 * 
 * `font_ascent` is set to the maximum of the ascent metrics of all characters in
 * the string. `font_descent` is set to the maximum of the descent metrics.
 * `overall_width` is set to the sum of the character-width metrics of all
 * characters in the string. For each character in the string, let W be the sum of
 * the character-width metrics of all characters preceding it in the string. Let L
 * be the left-side-bearing metric of the character plus W. Let R be the
 * right-side-bearing metric of the character plus W. The lbearing member is set
 * to the minimum L of all characters in the string. The rbearing member is set to
 * the maximum R.
 * 
 * For fonts defined with linear indexing rather than 2-byte matrix indexing, each
 * `xcb_char2b_t` structure is interpreted as a 16-bit number with byte1 as the
 * most significant byte. If the font has no defined default character, undefined
 * characters in the string are taken to have all zero metrics.
 * 
 * Characters with all zero metrics are ignored. If the font has no defined
 * default_char, the undefined characters in the string are also ignored.
 *
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
xcb_query_text_extents_cookie_t
xcb_query_text_extents_unchecked (xcb_connection_t   *c,
                                  xcb_fontable_t      font,
                                  uint32_t            string_len,
                                  const xcb_char2b_t *string);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_query_text_extents_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_query_text_extents_reply_t *
xcb_query_text_extents_reply (xcb_connection_t                 *c,
                              xcb_query_text_extents_cookie_t   cookie  /**< */,
                              xcb_generic_error_t             **e);

int
xcb_str_sizeof (const void  *_buffer);

char *
xcb_str_name (const xcb_str_t *R);

int
xcb_str_name_length (const xcb_str_t *R);

xcb_generic_iterator_t
xcb_str_name_end (const xcb_str_t *R);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_str_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_str_t)
 */
void
xcb_str_next (xcb_str_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_str_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_str_end (xcb_str_iterator_t i);

int
xcb_list_fonts_sizeof (const void  *_buffer);

/**
 * @brief get matching font names
 *
 * @param c The connection
 * @param max_names The maximum number of fonts to be returned.
 * @param pattern_len The length (in bytes) of \a pattern.
 * @param pattern A font pattern, for example "-misc-fixed-*".
 * \n
 * The asterisk (*) is a wildcard for any number of characters. The question mark
 * (?) is a wildcard for a single character. Use of uppercase or lowercase does
 * not matter.
 * @return A cookie
 *
 * Gets a list of available font names which match the given \a pattern.
 *
 */
xcb_list_fonts_cookie_t
xcb_list_fonts (xcb_connection_t *c,
                uint16_t          max_names,
                uint16_t          pattern_len,
                const char       *pattern);

/**
 * @brief get matching font names
 *
 * @param c The connection
 * @param max_names The maximum number of fonts to be returned.
 * @param pattern_len The length (in bytes) of \a pattern.
 * @param pattern A font pattern, for example "-misc-fixed-*".
 * \n
 * The asterisk (*) is a wildcard for any number of characters. The question mark
 * (?) is a wildcard for a single character. Use of uppercase or lowercase does
 * not matter.
 * @return A cookie
 *
 * Gets a list of available font names which match the given \a pattern.
 *
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
xcb_list_fonts_cookie_t
xcb_list_fonts_unchecked (xcb_connection_t *c,
                          uint16_t          max_names,
                          uint16_t          pattern_len,
                          const char       *pattern);

int
xcb_list_fonts_names_length (const xcb_list_fonts_reply_t *R);

xcb_str_iterator_t
xcb_list_fonts_names_iterator (const xcb_list_fonts_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_list_fonts_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_list_fonts_reply_t *
xcb_list_fonts_reply (xcb_connection_t         *c,
                      xcb_list_fonts_cookie_t   cookie  /**< */,
                      xcb_generic_error_t     **e);

int
xcb_list_fonts_with_info_sizeof (const void  *_buffer);

/**
 * @brief get matching font names and information
 *
 * @param c The connection
 * @param max_names The maximum number of fonts to be returned.
 * @param pattern_len The length (in bytes) of \a pattern.
 * @param pattern A font pattern, for example "-misc-fixed-*".
 * \n
 * The asterisk (*) is a wildcard for any number of characters. The question mark
 * (?) is a wildcard for a single character. Use of uppercase or lowercase does
 * not matter.
 * @return A cookie
 *
 * Gets a list of available font names which match the given \a pattern.
 *
 */
xcb_list_fonts_with_info_cookie_t
xcb_list_fonts_with_info (xcb_connection_t *c,
                          uint16_t          max_names,
                          uint16_t          pattern_len,
                          const char       *pattern);

/**
 * @brief get matching font names and information
 *
 * @param c The connection
 * @param max_names The maximum number of fonts to be returned.
 * @param pattern_len The length (in bytes) of \a pattern.
 * @param pattern A font pattern, for example "-misc-fixed-*".
 * \n
 * The asterisk (*) is a wildcard for any number of characters. The question mark
 * (?) is a wildcard for a single character. Use of uppercase or lowercase does
 * not matter.
 * @return A cookie
 *
 * Gets a list of available font names which match the given \a pattern.
 *
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
xcb_list_fonts_with_info_cookie_t
xcb_list_fonts_with_info_unchecked (xcb_connection_t *c,
                                    uint16_t          max_names,
                                    uint16_t          pattern_len,
                                    const char       *pattern);

xcb_fontprop_t *
xcb_list_fonts_with_info_properties (const xcb_list_fonts_with_info_reply_t *R);

int
xcb_list_fonts_with_info_properties_length (const xcb_list_fonts_with_info_reply_t *R);

xcb_fontprop_iterator_t
xcb_list_fonts_with_info_properties_iterator (const xcb_list_fonts_with_info_reply_t *R);

char *
xcb_list_fonts_with_info_name (const xcb_list_fonts_with_info_reply_t *R);

int
xcb_list_fonts_with_info_name_length (const xcb_list_fonts_with_info_reply_t *R);

xcb_generic_iterator_t
xcb_list_fonts_with_info_name_end (const xcb_list_fonts_with_info_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_list_fonts_with_info_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_list_fonts_with_info_reply_t *
xcb_list_fonts_with_info_reply (xcb_connection_t                   *c,
                                xcb_list_fonts_with_info_cookie_t   cookie  /**< */,
                                xcb_generic_error_t               **e);

int
xcb_set_font_path_sizeof (const void  *_buffer);

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
xcb_set_font_path_checked (xcb_connection_t *c,
                           uint16_t          font_qty,
                           const xcb_str_t  *font);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_set_font_path (xcb_connection_t *c,
                   uint16_t          font_qty,
                   const xcb_str_t  *font);

int
xcb_set_font_path_font_length (const xcb_set_font_path_request_t *R);

xcb_str_iterator_t
xcb_set_font_path_font_iterator (const xcb_set_font_path_request_t *R);

int
xcb_get_font_path_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_get_font_path_cookie_t
xcb_get_font_path (xcb_connection_t *c);

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
xcb_get_font_path_cookie_t
xcb_get_font_path_unchecked (xcb_connection_t *c);

int
xcb_get_font_path_path_length (const xcb_get_font_path_reply_t *R);

xcb_str_iterator_t
xcb_get_font_path_path_iterator (const xcb_get_font_path_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_get_font_path_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_get_font_path_reply_t *
xcb_get_font_path_reply (xcb_connection_t            *c,
                         xcb_get_font_path_cookie_t   cookie  /**< */,
                         xcb_generic_error_t        **e);

/**
 * @brief Creates a pixmap
 *
 * @param c The connection
 * @param depth TODO
 * @param pid The ID with which you will refer to the new pixmap, created by
 * `xcb_generate_id`.
 * @param drawable Drawable to get the screen from.
 * @param width The width of the new pixmap.
 * @param height The height of the new pixmap.
 * @return A cookie
 *
 * Creates a pixmap. The pixmap can only be used on the same screen as \a drawable
 * is on and only with drawables of the same \a depth.
 *
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
xcb_void_cookie_t
xcb_create_pixmap_checked (xcb_connection_t *c,
                           uint8_t           depth,
                           xcb_pixmap_t      pid,
                           xcb_drawable_t    drawable,
                           uint16_t          width,
                           uint16_t          height);

/**
 * @brief Creates a pixmap
 *
 * @param c The connection
 * @param depth TODO
 * @param pid The ID with which you will refer to the new pixmap, created by
 * `xcb_generate_id`.
 * @param drawable Drawable to get the screen from.
 * @param width The width of the new pixmap.
 * @param height The height of the new pixmap.
 * @return A cookie
 *
 * Creates a pixmap. The pixmap can only be used on the same screen as \a drawable
 * is on and only with drawables of the same \a depth.
 *
 */
xcb_void_cookie_t
xcb_create_pixmap (xcb_connection_t *c,
                   uint8_t           depth,
                   xcb_pixmap_t      pid,
                   xcb_drawable_t    drawable,
                   uint16_t          width,
                   uint16_t          height);

/**
 * @brief Destroys a pixmap
 *
 * @param c The connection
 * @param pixmap The pixmap to destroy.
 * @return A cookie
 *
 * Deletes the association between the pixmap ID and the pixmap. The pixmap
 * storage will be freed when there are no more references to it.
 *
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
xcb_void_cookie_t
xcb_free_pixmap_checked (xcb_connection_t *c,
                         xcb_pixmap_t      pixmap);

/**
 * @brief Destroys a pixmap
 *
 * @param c The connection
 * @param pixmap The pixmap to destroy.
 * @return A cookie
 *
 * Deletes the association between the pixmap ID and the pixmap. The pixmap
 * storage will be freed when there are no more references to it.
 *
 */
xcb_void_cookie_t
xcb_free_pixmap (xcb_connection_t *c,
                 xcb_pixmap_t      pixmap);

int
xcb_create_gc_value_list_serialize (void                             **_buffer,
                                    uint32_t                           value_mask,
                                    const xcb_create_gc_value_list_t  *_aux);

int
xcb_create_gc_value_list_unpack (const void                  *_buffer,
                                 uint32_t                     value_mask,
                                 xcb_create_gc_value_list_t  *_aux);

int
xcb_create_gc_value_list_sizeof (const void  *_buffer,
                                 uint32_t     value_mask);

int
xcb_create_gc_sizeof (const void  *_buffer);

/**
 * @brief Creates a graphics context
 *
 * @param c The connection
 * @param cid The ID with which you will refer to the graphics context, created by
 * `xcb_generate_id`.
 * @param drawable Drawable to get the root/depth from.
 * @return A cookie
 *
 * Creates a graphics context. The graphics context can be used with any drawable
 * that has the same root and depth as the specified drawable.
 *
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
xcb_void_cookie_t
xcb_create_gc_checked (xcb_connection_t *c,
                       xcb_gcontext_t    cid,
                       xcb_drawable_t    drawable,
                       uint32_t          value_mask,
                       const void       *value_list);

/**
 * @brief Creates a graphics context
 *
 * @param c The connection
 * @param cid The ID with which you will refer to the graphics context, created by
 * `xcb_generate_id`.
 * @param drawable Drawable to get the root/depth from.
 * @return A cookie
 *
 * Creates a graphics context. The graphics context can be used with any drawable
 * that has the same root and depth as the specified drawable.
 *
 */
xcb_void_cookie_t
xcb_create_gc (xcb_connection_t *c,
               xcb_gcontext_t    cid,
               xcb_drawable_t    drawable,
               uint32_t          value_mask,
               const void       *value_list);

/**
 * @brief Creates a graphics context
 *
 * @param c The connection
 * @param cid The ID with which you will refer to the graphics context, created by
 * `xcb_generate_id`.
 * @param drawable Drawable to get the root/depth from.
 * @return A cookie
 *
 * Creates a graphics context. The graphics context can be used with any drawable
 * that has the same root and depth as the specified drawable.
 *
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
xcb_void_cookie_t
xcb_create_gc_aux_checked (xcb_connection_t                 *c,
                           xcb_gcontext_t                    cid,
                           xcb_drawable_t                    drawable,
                           uint32_t                          value_mask,
                           const xcb_create_gc_value_list_t *value_list);

/**
 * @brief Creates a graphics context
 *
 * @param c The connection
 * @param cid The ID with which you will refer to the graphics context, created by
 * `xcb_generate_id`.
 * @param drawable Drawable to get the root/depth from.
 * @return A cookie
 *
 * Creates a graphics context. The graphics context can be used with any drawable
 * that has the same root and depth as the specified drawable.
 *
 */
xcb_void_cookie_t
xcb_create_gc_aux (xcb_connection_t                 *c,
                   xcb_gcontext_t                    cid,
                   xcb_drawable_t                    drawable,
                   uint32_t                          value_mask,
                   const xcb_create_gc_value_list_t *value_list);

void *
xcb_create_gc_value_list (const xcb_create_gc_request_t *R);

int
xcb_change_gc_value_list_serialize (void                             **_buffer,
                                    uint32_t                           value_mask,
                                    const xcb_change_gc_value_list_t  *_aux);

int
xcb_change_gc_value_list_unpack (const void                  *_buffer,
                                 uint32_t                     value_mask,
                                 xcb_change_gc_value_list_t  *_aux);

int
xcb_change_gc_value_list_sizeof (const void  *_buffer,
                                 uint32_t     value_mask);

int
xcb_change_gc_sizeof (const void  *_buffer);

/**
 * @brief change graphics context components
 *
 * @param c The connection
 * @param gc The graphics context to change.
 * @param value_mask A bitmask of #xcb_gc_t values.
 * @param value_mask \n
 * @param value_list Values for each of the components specified in the bitmask \a value_mask. The
 * order has to correspond to the order of possible \a value_mask bits. See the
 * example.
 * @return A cookie
 *
 * Changes the components specified by \a value_mask for the specified graphics context.
 *
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
xcb_void_cookie_t
xcb_change_gc_checked (xcb_connection_t *c,
                       xcb_gcontext_t    gc,
                       uint32_t          value_mask,
                       const void       *value_list);

/**
 * @brief change graphics context components
 *
 * @param c The connection
 * @param gc The graphics context to change.
 * @param value_mask A bitmask of #xcb_gc_t values.
 * @param value_mask \n
 * @param value_list Values for each of the components specified in the bitmask \a value_mask. The
 * order has to correspond to the order of possible \a value_mask bits. See the
 * example.
 * @return A cookie
 *
 * Changes the components specified by \a value_mask for the specified graphics context.
 *
 */
xcb_void_cookie_t
xcb_change_gc (xcb_connection_t *c,
               xcb_gcontext_t    gc,
               uint32_t          value_mask,
               const void       *value_list);

/**
 * @brief change graphics context components
 *
 * @param c The connection
 * @param gc The graphics context to change.
 * @param value_mask A bitmask of #xcb_gc_t values.
 * @param value_mask \n
 * @param value_list Values for each of the components specified in the bitmask \a value_mask. The
 * order has to correspond to the order of possible \a value_mask bits. See the
 * example.
 * @return A cookie
 *
 * Changes the components specified by \a value_mask for the specified graphics context.
 *
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
xcb_void_cookie_t
xcb_change_gc_aux_checked (xcb_connection_t                 *c,
                           xcb_gcontext_t                    gc,
                           uint32_t                          value_mask,
                           const xcb_change_gc_value_list_t *value_list);

/**
 * @brief change graphics context components
 *
 * @param c The connection
 * @param gc The graphics context to change.
 * @param value_mask A bitmask of #xcb_gc_t values.
 * @param value_mask \n
 * @param value_list Values for each of the components specified in the bitmask \a value_mask. The
 * order has to correspond to the order of possible \a value_mask bits. See the
 * example.
 * @return A cookie
 *
 * Changes the components specified by \a value_mask for the specified graphics context.
 *
 */
xcb_void_cookie_t
xcb_change_gc_aux (xcb_connection_t                 *c,
                   xcb_gcontext_t                    gc,
                   uint32_t                          value_mask,
                   const xcb_change_gc_value_list_t *value_list);

void *
xcb_change_gc_value_list (const xcb_change_gc_request_t *R);

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
xcb_copy_gc_checked (xcb_connection_t *c,
                     xcb_gcontext_t    src_gc,
                     xcb_gcontext_t    dst_gc,
                     uint32_t          value_mask);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_copy_gc (xcb_connection_t *c,
             xcb_gcontext_t    src_gc,
             xcb_gcontext_t    dst_gc,
             uint32_t          value_mask);

int
xcb_set_dashes_sizeof (const void  *_buffer);

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
xcb_set_dashes_checked (xcb_connection_t *c,
                        xcb_gcontext_t    gc,
                        uint16_t          dash_offset,
                        uint16_t          dashes_len,
                        const uint8_t    *dashes);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_set_dashes (xcb_connection_t *c,
                xcb_gcontext_t    gc,
                uint16_t          dash_offset,
                uint16_t          dashes_len,
                const uint8_t    *dashes);

uint8_t *
xcb_set_dashes_dashes (const xcb_set_dashes_request_t *R);

int
xcb_set_dashes_dashes_length (const xcb_set_dashes_request_t *R);

xcb_generic_iterator_t
xcb_set_dashes_dashes_end (const xcb_set_dashes_request_t *R);

int
xcb_set_clip_rectangles_sizeof (const void  *_buffer,
                                uint32_t     rectangles_len);

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
xcb_set_clip_rectangles_checked (xcb_connection_t      *c,
                                 uint8_t                ordering,
                                 xcb_gcontext_t         gc,
                                 int16_t                clip_x_origin,
                                 int16_t                clip_y_origin,
                                 uint32_t               rectangles_len,
                                 const xcb_rectangle_t *rectangles);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_set_clip_rectangles (xcb_connection_t      *c,
                         uint8_t                ordering,
                         xcb_gcontext_t         gc,
                         int16_t                clip_x_origin,
                         int16_t                clip_y_origin,
                         uint32_t               rectangles_len,
                         const xcb_rectangle_t *rectangles);

xcb_rectangle_t *
xcb_set_clip_rectangles_rectangles (const xcb_set_clip_rectangles_request_t *R);

int
xcb_set_clip_rectangles_rectangles_length (const xcb_set_clip_rectangles_request_t *R);

xcb_rectangle_iterator_t
xcb_set_clip_rectangles_rectangles_iterator (const xcb_set_clip_rectangles_request_t *R);

/**
 * @brief Destroys a graphics context
 *
 * @param c The connection
 * @param gc The graphics context to destroy.
 * @return A cookie
 *
 * Destroys the specified \a gc and all associated storage.
 *
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
xcb_void_cookie_t
xcb_free_gc_checked (xcb_connection_t *c,
                     xcb_gcontext_t    gc);

/**
 * @brief Destroys a graphics context
 *
 * @param c The connection
 * @param gc The graphics context to destroy.
 * @return A cookie
 *
 * Destroys the specified \a gc and all associated storage.
 *
 */
xcb_void_cookie_t
xcb_free_gc (xcb_connection_t *c,
             xcb_gcontext_t    gc);

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
xcb_clear_area_checked (xcb_connection_t *c,
                        uint8_t           exposures,
                        xcb_window_t      window,
                        int16_t           x,
                        int16_t           y,
                        uint16_t          width,
                        uint16_t          height);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_clear_area (xcb_connection_t *c,
                uint8_t           exposures,
                xcb_window_t      window,
                int16_t           x,
                int16_t           y,
                uint16_t          width,
                uint16_t          height);

/**
 * @brief copy areas
 *
 * @param c The connection
 * @param src_drawable The source drawable (Window or Pixmap).
 * @param dst_drawable The destination drawable (Window or Pixmap).
 * @param gc The graphics context to use.
 * @param src_x The source X coordinate.
 * @param src_y The source Y coordinate.
 * @param dst_x The destination X coordinate.
 * @param dst_y The destination Y coordinate.
 * @param width The width of the area to copy (in pixels).
 * @param height The height of the area to copy (in pixels).
 * @return A cookie
 *
 * Copies the specified rectangle from \a src_drawable to \a dst_drawable.
 *
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
xcb_void_cookie_t
xcb_copy_area_checked (xcb_connection_t *c,
                       xcb_drawable_t    src_drawable,
                       xcb_drawable_t    dst_drawable,
                       xcb_gcontext_t    gc,
                       int16_t           src_x,
                       int16_t           src_y,
                       int16_t           dst_x,
                       int16_t           dst_y,
                       uint16_t          width,
                       uint16_t          height);

/**
 * @brief copy areas
 *
 * @param c The connection
 * @param src_drawable The source drawable (Window or Pixmap).
 * @param dst_drawable The destination drawable (Window or Pixmap).
 * @param gc The graphics context to use.
 * @param src_x The source X coordinate.
 * @param src_y The source Y coordinate.
 * @param dst_x The destination X coordinate.
 * @param dst_y The destination Y coordinate.
 * @param width The width of the area to copy (in pixels).
 * @param height The height of the area to copy (in pixels).
 * @return A cookie
 *
 * Copies the specified rectangle from \a src_drawable to \a dst_drawable.
 *
 */
xcb_void_cookie_t
xcb_copy_area (xcb_connection_t *c,
               xcb_drawable_t    src_drawable,
               xcb_drawable_t    dst_drawable,
               xcb_gcontext_t    gc,
               int16_t           src_x,
               int16_t           src_y,
               int16_t           dst_x,
               int16_t           dst_y,
               uint16_t          width,
               uint16_t          height);

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
xcb_copy_plane_checked (xcb_connection_t *c,
                        xcb_drawable_t    src_drawable,
                        xcb_drawable_t    dst_drawable,
                        xcb_gcontext_t    gc,
                        int16_t           src_x,
                        int16_t           src_y,
                        int16_t           dst_x,
                        int16_t           dst_y,
                        uint16_t          width,
                        uint16_t          height,
                        uint32_t          bit_plane);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_copy_plane (xcb_connection_t *c,
                xcb_drawable_t    src_drawable,
                xcb_drawable_t    dst_drawable,
                xcb_gcontext_t    gc,
                int16_t           src_x,
                int16_t           src_y,
                int16_t           dst_x,
                int16_t           dst_y,
                uint16_t          width,
                uint16_t          height,
                uint32_t          bit_plane);

int
xcb_poly_point_sizeof (const void  *_buffer,
                       uint32_t     points_len);

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
xcb_poly_point_checked (xcb_connection_t  *c,
                        uint8_t            coordinate_mode,
                        xcb_drawable_t     drawable,
                        xcb_gcontext_t     gc,
                        uint32_t           points_len,
                        const xcb_point_t *points);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_poly_point (xcb_connection_t  *c,
                uint8_t            coordinate_mode,
                xcb_drawable_t     drawable,
                xcb_gcontext_t     gc,
                uint32_t           points_len,
                const xcb_point_t *points);

xcb_point_t *
xcb_poly_point_points (const xcb_poly_point_request_t *R);

int
xcb_poly_point_points_length (const xcb_poly_point_request_t *R);

xcb_point_iterator_t
xcb_poly_point_points_iterator (const xcb_poly_point_request_t *R);

int
xcb_poly_line_sizeof (const void  *_buffer,
                      uint32_t     points_len);

/**
 * @brief draw lines
 *
 * @param c The connection
 * @param coordinate_mode A bitmask of #xcb_coord_mode_t values.
 * @param coordinate_mode \n
 * @param drawable The drawable to draw the line(s) on.
 * @param gc The graphics context to use.
 * @param points_len The number of `xcb_point_t` structures in \a points.
 * @param points An array of points.
 * @return A cookie
 *
 * Draws \a points_len-1 lines between each pair of points (point[i], point[i+1])
 * in the \a points array. The lines are drawn in the order listed in the array.
 * They join correctly at all intermediate points, and if the first and last
 * points coincide, the first and last lines also join correctly. For any given
 * line, a pixel is not drawn more than once. If thin (zero line-width) lines
 * intersect, the intersecting pixels are drawn multiple times. If wide lines
 * intersect, the intersecting pixels are drawn only once, as though the entire
 * request were a single, filled shape.
 *
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
xcb_void_cookie_t
xcb_poly_line_checked (xcb_connection_t  *c,
                       uint8_t            coordinate_mode,
                       xcb_drawable_t     drawable,
                       xcb_gcontext_t     gc,
                       uint32_t           points_len,
                       const xcb_point_t *points);

/**
 * @brief draw lines
 *
 * @param c The connection
 * @param coordinate_mode A bitmask of #xcb_coord_mode_t values.
 * @param coordinate_mode \n
 * @param drawable The drawable to draw the line(s) on.
 * @param gc The graphics context to use.
 * @param points_len The number of `xcb_point_t` structures in \a points.
 * @param points An array of points.
 * @return A cookie
 *
 * Draws \a points_len-1 lines between each pair of points (point[i], point[i+1])
 * in the \a points array. The lines are drawn in the order listed in the array.
 * They join correctly at all intermediate points, and if the first and last
 * points coincide, the first and last lines also join correctly. For any given
 * line, a pixel is not drawn more than once. If thin (zero line-width) lines
 * intersect, the intersecting pixels are drawn multiple times. If wide lines
 * intersect, the intersecting pixels are drawn only once, as though the entire
 * request were a single, filled shape.
 *
 */
xcb_void_cookie_t
xcb_poly_line (xcb_connection_t  *c,
               uint8_t            coordinate_mode,
               xcb_drawable_t     drawable,
               xcb_gcontext_t     gc,
               uint32_t           points_len,
               const xcb_point_t *points);

xcb_point_t *
xcb_poly_line_points (const xcb_poly_line_request_t *R);

int
xcb_poly_line_points_length (const xcb_poly_line_request_t *R);

xcb_point_iterator_t
xcb_poly_line_points_iterator (const xcb_poly_line_request_t *R);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_segment_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_segment_t)
 */
void
xcb_segment_next (xcb_segment_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_segment_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_segment_end (xcb_segment_iterator_t i);

int
xcb_poly_segment_sizeof (const void  *_buffer,
                         uint32_t     segments_len);

/**
 * @brief draw lines
 *
 * @param c The connection
 * @param drawable A drawable (Window or Pixmap) to draw on.
 * @param gc The graphics context to use.
 * \n
 * TODO: document which attributes of a gc are used
 * @param segments_len The number of `xcb_segment_t` structures in \a segments.
 * @param segments An array of `xcb_segment_t` structures.
 * @return A cookie
 *
 * Draws multiple, unconnected lines. For each segment, a line is drawn between
 * (x1, y1) and (x2, y2). The lines are drawn in the order listed in the array of
 * `xcb_segment_t` structures and does not perform joining at coincident
 * endpoints. For any given line, a pixel is not drawn more than once. If lines
 * intersect, the intersecting pixels are drawn multiple times.
 * 
 * TODO: include the xcb_segment_t data structure
 * 
 * TODO: an example
 *
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
xcb_void_cookie_t
xcb_poly_segment_checked (xcb_connection_t    *c,
                          xcb_drawable_t       drawable,
                          xcb_gcontext_t       gc,
                          uint32_t             segments_len,
                          const xcb_segment_t *segments);

/**
 * @brief draw lines
 *
 * @param c The connection
 * @param drawable A drawable (Window or Pixmap) to draw on.
 * @param gc The graphics context to use.
 * \n
 * TODO: document which attributes of a gc are used
 * @param segments_len The number of `xcb_segment_t` structures in \a segments.
 * @param segments An array of `xcb_segment_t` structures.
 * @return A cookie
 *
 * Draws multiple, unconnected lines. For each segment, a line is drawn between
 * (x1, y1) and (x2, y2). The lines are drawn in the order listed in the array of
 * `xcb_segment_t` structures and does not perform joining at coincident
 * endpoints. For any given line, a pixel is not drawn more than once. If lines
 * intersect, the intersecting pixels are drawn multiple times.
 * 
 * TODO: include the xcb_segment_t data structure
 * 
 * TODO: an example
 *
 */
xcb_void_cookie_t
xcb_poly_segment (xcb_connection_t    *c,
                  xcb_drawable_t       drawable,
                  xcb_gcontext_t       gc,
                  uint32_t             segments_len,
                  const xcb_segment_t *segments);

xcb_segment_t *
xcb_poly_segment_segments (const xcb_poly_segment_request_t *R);

int
xcb_poly_segment_segments_length (const xcb_poly_segment_request_t *R);

xcb_segment_iterator_t
xcb_poly_segment_segments_iterator (const xcb_poly_segment_request_t *R);

int
xcb_poly_rectangle_sizeof (const void  *_buffer,
                           uint32_t     rectangles_len);

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
xcb_poly_rectangle_checked (xcb_connection_t      *c,
                            xcb_drawable_t         drawable,
                            xcb_gcontext_t         gc,
                            uint32_t               rectangles_len,
                            const xcb_rectangle_t *rectangles);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_poly_rectangle (xcb_connection_t      *c,
                    xcb_drawable_t         drawable,
                    xcb_gcontext_t         gc,
                    uint32_t               rectangles_len,
                    const xcb_rectangle_t *rectangles);

xcb_rectangle_t *
xcb_poly_rectangle_rectangles (const xcb_poly_rectangle_request_t *R);

int
xcb_poly_rectangle_rectangles_length (const xcb_poly_rectangle_request_t *R);

xcb_rectangle_iterator_t
xcb_poly_rectangle_rectangles_iterator (const xcb_poly_rectangle_request_t *R);

int
xcb_poly_arc_sizeof (const void  *_buffer,
                     uint32_t     arcs_len);

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
xcb_poly_arc_checked (xcb_connection_t *c,
                      xcb_drawable_t    drawable,
                      xcb_gcontext_t    gc,
                      uint32_t          arcs_len,
                      const xcb_arc_t  *arcs);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_poly_arc (xcb_connection_t *c,
              xcb_drawable_t    drawable,
              xcb_gcontext_t    gc,
              uint32_t          arcs_len,
              const xcb_arc_t  *arcs);

xcb_arc_t *
xcb_poly_arc_arcs (const xcb_poly_arc_request_t *R);

int
xcb_poly_arc_arcs_length (const xcb_poly_arc_request_t *R);

xcb_arc_iterator_t
xcb_poly_arc_arcs_iterator (const xcb_poly_arc_request_t *R);

int
xcb_fill_poly_sizeof (const void  *_buffer,
                      uint32_t     points_len);

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
xcb_fill_poly_checked (xcb_connection_t  *c,
                       xcb_drawable_t     drawable,
                       xcb_gcontext_t     gc,
                       uint8_t            shape,
                       uint8_t            coordinate_mode,
                       uint32_t           points_len,
                       const xcb_point_t *points);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_fill_poly (xcb_connection_t  *c,
               xcb_drawable_t     drawable,
               xcb_gcontext_t     gc,
               uint8_t            shape,
               uint8_t            coordinate_mode,
               uint32_t           points_len,
               const xcb_point_t *points);

xcb_point_t *
xcb_fill_poly_points (const xcb_fill_poly_request_t *R);

int
xcb_fill_poly_points_length (const xcb_fill_poly_request_t *R);

xcb_point_iterator_t
xcb_fill_poly_points_iterator (const xcb_fill_poly_request_t *R);

int
xcb_poly_fill_rectangle_sizeof (const void  *_buffer,
                                uint32_t     rectangles_len);

/**
 * @brief Fills rectangles
 *
 * @param c The connection
 * @param drawable The drawable (Window or Pixmap) to draw on.
 * @param gc The graphics context to use.
 * \n
 * The following graphics context components are used: function, plane-mask,
 * fill-style, subwindow-mode, clip-x-origin, clip-y-origin, and clip-mask.
 * \n
 * The following graphics context mode-dependent components are used:
 * foreground, background, tile, stipple, tile-stipple-x-origin, and
 * tile-stipple-y-origin.
 * @param rectangles_len The number of `xcb_rectangle_t` structures in \a rectangles.
 * @param rectangles The rectangles to fill.
 * @return A cookie
 *
 * Fills the specified rectangle(s) in the order listed in the array. For any
 * given rectangle, each pixel is not drawn more than once. If rectangles
 * intersect, the intersecting pixels are drawn multiple times.
 *
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
xcb_void_cookie_t
xcb_poly_fill_rectangle_checked (xcb_connection_t      *c,
                                 xcb_drawable_t         drawable,
                                 xcb_gcontext_t         gc,
                                 uint32_t               rectangles_len,
                                 const xcb_rectangle_t *rectangles);

/**
 * @brief Fills rectangles
 *
 * @param c The connection
 * @param drawable The drawable (Window or Pixmap) to draw on.
 * @param gc The graphics context to use.
 * \n
 * The following graphics context components are used: function, plane-mask,
 * fill-style, subwindow-mode, clip-x-origin, clip-y-origin, and clip-mask.
 * \n
 * The following graphics context mode-dependent components are used:
 * foreground, background, tile, stipple, tile-stipple-x-origin, and
 * tile-stipple-y-origin.
 * @param rectangles_len The number of `xcb_rectangle_t` structures in \a rectangles.
 * @param rectangles The rectangles to fill.
 * @return A cookie
 *
 * Fills the specified rectangle(s) in the order listed in the array. For any
 * given rectangle, each pixel is not drawn more than once. If rectangles
 * intersect, the intersecting pixels are drawn multiple times.
 *
 */
xcb_void_cookie_t
xcb_poly_fill_rectangle (xcb_connection_t      *c,
                         xcb_drawable_t         drawable,
                         xcb_gcontext_t         gc,
                         uint32_t               rectangles_len,
                         const xcb_rectangle_t *rectangles);

xcb_rectangle_t *
xcb_poly_fill_rectangle_rectangles (const xcb_poly_fill_rectangle_request_t *R);

int
xcb_poly_fill_rectangle_rectangles_length (const xcb_poly_fill_rectangle_request_t *R);

xcb_rectangle_iterator_t
xcb_poly_fill_rectangle_rectangles_iterator (const xcb_poly_fill_rectangle_request_t *R);

int
xcb_poly_fill_arc_sizeof (const void  *_buffer,
                          uint32_t     arcs_len);

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
xcb_poly_fill_arc_checked (xcb_connection_t *c,
                           xcb_drawable_t    drawable,
                           xcb_gcontext_t    gc,
                           uint32_t          arcs_len,
                           const xcb_arc_t  *arcs);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_poly_fill_arc (xcb_connection_t *c,
                   xcb_drawable_t    drawable,
                   xcb_gcontext_t    gc,
                   uint32_t          arcs_len,
                   const xcb_arc_t  *arcs);

xcb_arc_t *
xcb_poly_fill_arc_arcs (const xcb_poly_fill_arc_request_t *R);

int
xcb_poly_fill_arc_arcs_length (const xcb_poly_fill_arc_request_t *R);

xcb_arc_iterator_t
xcb_poly_fill_arc_arcs_iterator (const xcb_poly_fill_arc_request_t *R);

int
xcb_put_image_sizeof (const void  *_buffer,
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
xcb_put_image_checked (xcb_connection_t *c,
                       uint8_t           format,
                       xcb_drawable_t    drawable,
                       xcb_gcontext_t    gc,
                       uint16_t          width,
                       uint16_t          height,
                       int16_t           dst_x,
                       int16_t           dst_y,
                       uint8_t           left_pad,
                       uint8_t           depth,
                       uint32_t          data_len,
                       const uint8_t    *data);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_put_image (xcb_connection_t *c,
               uint8_t           format,
               xcb_drawable_t    drawable,
               xcb_gcontext_t    gc,
               uint16_t          width,
               uint16_t          height,
               int16_t           dst_x,
               int16_t           dst_y,
               uint8_t           left_pad,
               uint8_t           depth,
               uint32_t          data_len,
               const uint8_t    *data);

uint8_t *
xcb_put_image_data (const xcb_put_image_request_t *R);

int
xcb_put_image_data_length (const xcb_put_image_request_t *R);

xcb_generic_iterator_t
xcb_put_image_data_end (const xcb_put_image_request_t *R);

int
xcb_get_image_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_get_image_cookie_t
xcb_get_image (xcb_connection_t *c,
               uint8_t           format,
               xcb_drawable_t    drawable,
               int16_t           x,
               int16_t           y,
               uint16_t          width,
               uint16_t          height,
               uint32_t          plane_mask);

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
xcb_get_image_cookie_t
xcb_get_image_unchecked (xcb_connection_t *c,
                         uint8_t           format,
                         xcb_drawable_t    drawable,
                         int16_t           x,
                         int16_t           y,
                         uint16_t          width,
                         uint16_t          height,
                         uint32_t          plane_mask);

uint8_t *
xcb_get_image_data (const xcb_get_image_reply_t *R);

int
xcb_get_image_data_length (const xcb_get_image_reply_t *R);

xcb_generic_iterator_t
xcb_get_image_data_end (const xcb_get_image_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_get_image_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_get_image_reply_t *
xcb_get_image_reply (xcb_connection_t        *c,
                     xcb_get_image_cookie_t   cookie  /**< */,
                     xcb_generic_error_t    **e);

int
xcb_poly_text_8_sizeof (const void  *_buffer,
                        uint32_t     items_len);

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
xcb_poly_text_8_checked (xcb_connection_t *c,
                         xcb_drawable_t    drawable,
                         xcb_gcontext_t    gc,
                         int16_t           x,
                         int16_t           y,
                         uint32_t          items_len,
                         const uint8_t    *items);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_poly_text_8 (xcb_connection_t *c,
                 xcb_drawable_t    drawable,
                 xcb_gcontext_t    gc,
                 int16_t           x,
                 int16_t           y,
                 uint32_t          items_len,
                 const uint8_t    *items);

uint8_t *
xcb_poly_text_8_items (const xcb_poly_text_8_request_t *R);

int
xcb_poly_text_8_items_length (const xcb_poly_text_8_request_t *R);

xcb_generic_iterator_t
xcb_poly_text_8_items_end (const xcb_poly_text_8_request_t *R);

int
xcb_poly_text_16_sizeof (const void  *_buffer,
                         uint32_t     items_len);

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
xcb_poly_text_16_checked (xcb_connection_t *c,
                          xcb_drawable_t    drawable,
                          xcb_gcontext_t    gc,
                          int16_t           x,
                          int16_t           y,
                          uint32_t          items_len,
                          const uint8_t    *items);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_poly_text_16 (xcb_connection_t *c,
                  xcb_drawable_t    drawable,
                  xcb_gcontext_t    gc,
                  int16_t           x,
                  int16_t           y,
                  uint32_t          items_len,
                  const uint8_t    *items);

uint8_t *
xcb_poly_text_16_items (const xcb_poly_text_16_request_t *R);

int
xcb_poly_text_16_items_length (const xcb_poly_text_16_request_t *R);

xcb_generic_iterator_t
xcb_poly_text_16_items_end (const xcb_poly_text_16_request_t *R);

int
xcb_image_text_8_sizeof (const void  *_buffer);

/**
 * @brief Draws text
 *
 * @param c The connection
 * @param string_len The length of the \a string. Note that this parameter limited by 255 due to
 * using 8 bits!
 * @param drawable The drawable (Window or Pixmap) to draw text on.
 * @param gc The graphics context to use.
 * \n
 * The following graphics context components are used: plane-mask, foreground,
 * background, font, subwindow-mode, clip-x-origin, clip-y-origin, and clip-mask.
 * @param x The x coordinate of the first character, relative to the origin of \a drawable.
 * @param y The y coordinate of the first character, relative to the origin of \a drawable.
 * @param string The string to draw. Only the first 255 characters are relevant due to the data
 * type of \a string_len.
 * @return A cookie
 *
 * Fills the destination rectangle with the background pixel from \a gc, then
 * paints the text with the foreground pixel from \a gc. The upper-left corner of
 * the filled rectangle is at [x, y - font-ascent]. The width is overall-width,
 * the height is font-ascent + font-descent. The overall-width, font-ascent and
 * font-descent are as returned by `xcb_query_text_extents` (TODO).
 * 
 * Note that using X core fonts is deprecated (but still supported) in favor of
 * client-side rendering using Xft.
 *
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
xcb_void_cookie_t
xcb_image_text_8_checked (xcb_connection_t *c,
                          uint8_t           string_len,
                          xcb_drawable_t    drawable,
                          xcb_gcontext_t    gc,
                          int16_t           x,
                          int16_t           y,
                          const char       *string);

/**
 * @brief Draws text
 *
 * @param c The connection
 * @param string_len The length of the \a string. Note that this parameter limited by 255 due to
 * using 8 bits!
 * @param drawable The drawable (Window or Pixmap) to draw text on.
 * @param gc The graphics context to use.
 * \n
 * The following graphics context components are used: plane-mask, foreground,
 * background, font, subwindow-mode, clip-x-origin, clip-y-origin, and clip-mask.
 * @param x The x coordinate of the first character, relative to the origin of \a drawable.
 * @param y The y coordinate of the first character, relative to the origin of \a drawable.
 * @param string The string to draw. Only the first 255 characters are relevant due to the data
 * type of \a string_len.
 * @return A cookie
 *
 * Fills the destination rectangle with the background pixel from \a gc, then
 * paints the text with the foreground pixel from \a gc. The upper-left corner of
 * the filled rectangle is at [x, y - font-ascent]. The width is overall-width,
 * the height is font-ascent + font-descent. The overall-width, font-ascent and
 * font-descent are as returned by `xcb_query_text_extents` (TODO).
 * 
 * Note that using X core fonts is deprecated (but still supported) in favor of
 * client-side rendering using Xft.
 *
 */
xcb_void_cookie_t
xcb_image_text_8 (xcb_connection_t *c,
                  uint8_t           string_len,
                  xcb_drawable_t    drawable,
                  xcb_gcontext_t    gc,
                  int16_t           x,
                  int16_t           y,
                  const char       *string);

char *
xcb_image_text_8_string (const xcb_image_text_8_request_t *R);

int
xcb_image_text_8_string_length (const xcb_image_text_8_request_t *R);

xcb_generic_iterator_t
xcb_image_text_8_string_end (const xcb_image_text_8_request_t *R);

int
xcb_image_text_16_sizeof (const void  *_buffer);

/**
 * @brief Draws text
 *
 * @param c The connection
 * @param string_len The length of the \a string in characters. Note that this parameter limited by
 * 255 due to using 8 bits!
 * @param drawable The drawable (Window or Pixmap) to draw text on.
 * @param gc The graphics context to use.
 * \n
 * The following graphics context components are used: plane-mask, foreground,
 * background, font, subwindow-mode, clip-x-origin, clip-y-origin, and clip-mask.
 * @param x The x coordinate of the first character, relative to the origin of \a drawable.
 * @param y The y coordinate of the first character, relative to the origin of \a drawable.
 * @param string The string to draw. Only the first 255 characters are relevant due to the data
 * type of \a string_len. Every character uses 2 bytes (hence the 16 in this
 * request's name).
 * @return A cookie
 *
 * Fills the destination rectangle with the background pixel from \a gc, then
 * paints the text with the foreground pixel from \a gc. The upper-left corner of
 * the filled rectangle is at [x, y - font-ascent]. The width is overall-width,
 * the height is font-ascent + font-descent. The overall-width, font-ascent and
 * font-descent are as returned by `xcb_query_text_extents` (TODO).
 * 
 * Note that using X core fonts is deprecated (but still supported) in favor of
 * client-side rendering using Xft.
 *
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
xcb_void_cookie_t
xcb_image_text_16_checked (xcb_connection_t   *c,
                           uint8_t             string_len,
                           xcb_drawable_t      drawable,
                           xcb_gcontext_t      gc,
                           int16_t             x,
                           int16_t             y,
                           const xcb_char2b_t *string);

/**
 * @brief Draws text
 *
 * @param c The connection
 * @param string_len The length of the \a string in characters. Note that this parameter limited by
 * 255 due to using 8 bits!
 * @param drawable The drawable (Window or Pixmap) to draw text on.
 * @param gc The graphics context to use.
 * \n
 * The following graphics context components are used: plane-mask, foreground,
 * background, font, subwindow-mode, clip-x-origin, clip-y-origin, and clip-mask.
 * @param x The x coordinate of the first character, relative to the origin of \a drawable.
 * @param y The y coordinate of the first character, relative to the origin of \a drawable.
 * @param string The string to draw. Only the first 255 characters are relevant due to the data
 * type of \a string_len. Every character uses 2 bytes (hence the 16 in this
 * request's name).
 * @return A cookie
 *
 * Fills the destination rectangle with the background pixel from \a gc, then
 * paints the text with the foreground pixel from \a gc. The upper-left corner of
 * the filled rectangle is at [x, y - font-ascent]. The width is overall-width,
 * the height is font-ascent + font-descent. The overall-width, font-ascent and
 * font-descent are as returned by `xcb_query_text_extents` (TODO).
 * 
 * Note that using X core fonts is deprecated (but still supported) in favor of
 * client-side rendering using Xft.
 *
 */
xcb_void_cookie_t
xcb_image_text_16 (xcb_connection_t   *c,
                   uint8_t             string_len,
                   xcb_drawable_t      drawable,
                   xcb_gcontext_t      gc,
                   int16_t             x,
                   int16_t             y,
                   const xcb_char2b_t *string);

xcb_char2b_t *
xcb_image_text_16_string (const xcb_image_text_16_request_t *R);

int
xcb_image_text_16_string_length (const xcb_image_text_16_request_t *R);

xcb_char2b_iterator_t
xcb_image_text_16_string_iterator (const xcb_image_text_16_request_t *R);

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
xcb_create_colormap_checked (xcb_connection_t *c,
                             uint8_t           alloc,
                             xcb_colormap_t    mid,
                             xcb_window_t      window,
                             xcb_visualid_t    visual);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_create_colormap (xcb_connection_t *c,
                     uint8_t           alloc,
                     xcb_colormap_t    mid,
                     xcb_window_t      window,
                     xcb_visualid_t    visual);

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
xcb_free_colormap_checked (xcb_connection_t *c,
                           xcb_colormap_t    cmap);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_free_colormap (xcb_connection_t *c,
                   xcb_colormap_t    cmap);

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
xcb_copy_colormap_and_free_checked (xcb_connection_t *c,
                                    xcb_colormap_t    mid,
                                    xcb_colormap_t    src_cmap);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_copy_colormap_and_free (xcb_connection_t *c,
                            xcb_colormap_t    mid,
                            xcb_colormap_t    src_cmap);

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
xcb_install_colormap_checked (xcb_connection_t *c,
                              xcb_colormap_t    cmap);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_install_colormap (xcb_connection_t *c,
                      xcb_colormap_t    cmap);

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
xcb_uninstall_colormap_checked (xcb_connection_t *c,
                                xcb_colormap_t    cmap);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_uninstall_colormap (xcb_connection_t *c,
                        xcb_colormap_t    cmap);

int
xcb_list_installed_colormaps_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_list_installed_colormaps_cookie_t
xcb_list_installed_colormaps (xcb_connection_t *c,
                              xcb_window_t      window);

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
xcb_list_installed_colormaps_cookie_t
xcb_list_installed_colormaps_unchecked (xcb_connection_t *c,
                                        xcb_window_t      window);

xcb_colormap_t *
xcb_list_installed_colormaps_cmaps (const xcb_list_installed_colormaps_reply_t *R);

int
xcb_list_installed_colormaps_cmaps_length (const xcb_list_installed_colormaps_reply_t *R);

xcb_generic_iterator_t
xcb_list_installed_colormaps_cmaps_end (const xcb_list_installed_colormaps_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_list_installed_colormaps_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_list_installed_colormaps_reply_t *
xcb_list_installed_colormaps_reply (xcb_connection_t                       *c,
                                    xcb_list_installed_colormaps_cookie_t   cookie  /**< */,
                                    xcb_generic_error_t                   **e);

/**
 * @brief Allocate a color
 *
 * @param c The connection
 * @param cmap TODO
 * @param red The red value of your color.
 * @param green The green value of your color.
 * @param blue The blue value of your color.
 * @return A cookie
 *
 * Allocates a read-only colormap entry corresponding to the closest RGB value
 * supported by the hardware. If you are using TrueColor, you can take a shortcut
 * and directly calculate the color pixel value to avoid the round trip. But, for
 * example, on 16-bit color setups (VNC), you can easily get the closest supported
 * RGB value to the RGB value you are specifying.
 *
 */
xcb_alloc_color_cookie_t
xcb_alloc_color (xcb_connection_t *c,
                 xcb_colormap_t    cmap,
                 uint16_t          red,
                 uint16_t          green,
                 uint16_t          blue);

/**
 * @brief Allocate a color
 *
 * @param c The connection
 * @param cmap TODO
 * @param red The red value of your color.
 * @param green The green value of your color.
 * @param blue The blue value of your color.
 * @return A cookie
 *
 * Allocates a read-only colormap entry corresponding to the closest RGB value
 * supported by the hardware. If you are using TrueColor, you can take a shortcut
 * and directly calculate the color pixel value to avoid the round trip. But, for
 * example, on 16-bit color setups (VNC), you can easily get the closest supported
 * RGB value to the RGB value you are specifying.
 *
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
xcb_alloc_color_cookie_t
xcb_alloc_color_unchecked (xcb_connection_t *c,
                           xcb_colormap_t    cmap,
                           uint16_t          red,
                           uint16_t          green,
                           uint16_t          blue);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_alloc_color_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_alloc_color_reply_t *
xcb_alloc_color_reply (xcb_connection_t          *c,
                       xcb_alloc_color_cookie_t   cookie  /**< */,
                       xcb_generic_error_t      **e);

int
xcb_alloc_named_color_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_alloc_named_color_cookie_t
xcb_alloc_named_color (xcb_connection_t *c,
                       xcb_colormap_t    cmap,
                       uint16_t          name_len,
                       const char       *name);

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
xcb_alloc_named_color_cookie_t
xcb_alloc_named_color_unchecked (xcb_connection_t *c,
                                 xcb_colormap_t    cmap,
                                 uint16_t          name_len,
                                 const char       *name);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_alloc_named_color_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_alloc_named_color_reply_t *
xcb_alloc_named_color_reply (xcb_connection_t                *c,
                             xcb_alloc_named_color_cookie_t   cookie  /**< */,
                             xcb_generic_error_t            **e);

int
xcb_alloc_color_cells_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_alloc_color_cells_cookie_t
xcb_alloc_color_cells (xcb_connection_t *c,
                       uint8_t           contiguous,
                       xcb_colormap_t    cmap,
                       uint16_t          colors,
                       uint16_t          planes);

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
xcb_alloc_color_cells_cookie_t
xcb_alloc_color_cells_unchecked (xcb_connection_t *c,
                                 uint8_t           contiguous,
                                 xcb_colormap_t    cmap,
                                 uint16_t          colors,
                                 uint16_t          planes);

uint32_t *
xcb_alloc_color_cells_pixels (const xcb_alloc_color_cells_reply_t *R);

int
xcb_alloc_color_cells_pixels_length (const xcb_alloc_color_cells_reply_t *R);

xcb_generic_iterator_t
xcb_alloc_color_cells_pixels_end (const xcb_alloc_color_cells_reply_t *R);

uint32_t *
xcb_alloc_color_cells_masks (const xcb_alloc_color_cells_reply_t *R);

int
xcb_alloc_color_cells_masks_length (const xcb_alloc_color_cells_reply_t *R);

xcb_generic_iterator_t
xcb_alloc_color_cells_masks_end (const xcb_alloc_color_cells_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_alloc_color_cells_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_alloc_color_cells_reply_t *
xcb_alloc_color_cells_reply (xcb_connection_t                *c,
                             xcb_alloc_color_cells_cookie_t   cookie  /**< */,
                             xcb_generic_error_t            **e);

int
xcb_alloc_color_planes_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_alloc_color_planes_cookie_t
xcb_alloc_color_planes (xcb_connection_t *c,
                        uint8_t           contiguous,
                        xcb_colormap_t    cmap,
                        uint16_t          colors,
                        uint16_t          reds,
                        uint16_t          greens,
                        uint16_t          blues);

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
xcb_alloc_color_planes_cookie_t
xcb_alloc_color_planes_unchecked (xcb_connection_t *c,
                                  uint8_t           contiguous,
                                  xcb_colormap_t    cmap,
                                  uint16_t          colors,
                                  uint16_t          reds,
                                  uint16_t          greens,
                                  uint16_t          blues);

uint32_t *
xcb_alloc_color_planes_pixels (const xcb_alloc_color_planes_reply_t *R);

int
xcb_alloc_color_planes_pixels_length (const xcb_alloc_color_planes_reply_t *R);

xcb_generic_iterator_t
xcb_alloc_color_planes_pixels_end (const xcb_alloc_color_planes_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_alloc_color_planes_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_alloc_color_planes_reply_t *
xcb_alloc_color_planes_reply (xcb_connection_t                 *c,
                              xcb_alloc_color_planes_cookie_t   cookie  /**< */,
                              xcb_generic_error_t             **e);

int
xcb_free_colors_sizeof (const void  *_buffer,
                        uint32_t     pixels_len);

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
xcb_free_colors_checked (xcb_connection_t *c,
                         xcb_colormap_t    cmap,
                         uint32_t          plane_mask,
                         uint32_t          pixels_len,
                         const uint32_t   *pixels);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_free_colors (xcb_connection_t *c,
                 xcb_colormap_t    cmap,
                 uint32_t          plane_mask,
                 uint32_t          pixels_len,
                 const uint32_t   *pixels);

uint32_t *
xcb_free_colors_pixels (const xcb_free_colors_request_t *R);

int
xcb_free_colors_pixels_length (const xcb_free_colors_request_t *R);

xcb_generic_iterator_t
xcb_free_colors_pixels_end (const xcb_free_colors_request_t *R);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_coloritem_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_coloritem_t)
 */
void
xcb_coloritem_next (xcb_coloritem_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_coloritem_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_coloritem_end (xcb_coloritem_iterator_t i);

int
xcb_store_colors_sizeof (const void  *_buffer,
                         uint32_t     items_len);

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
xcb_store_colors_checked (xcb_connection_t      *c,
                          xcb_colormap_t         cmap,
                          uint32_t               items_len,
                          const xcb_coloritem_t *items);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_store_colors (xcb_connection_t      *c,
                  xcb_colormap_t         cmap,
                  uint32_t               items_len,
                  const xcb_coloritem_t *items);

xcb_coloritem_t *
xcb_store_colors_items (const xcb_store_colors_request_t *R);

int
xcb_store_colors_items_length (const xcb_store_colors_request_t *R);

xcb_coloritem_iterator_t
xcb_store_colors_items_iterator (const xcb_store_colors_request_t *R);

int
xcb_store_named_color_sizeof (const void  *_buffer);

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
xcb_store_named_color_checked (xcb_connection_t *c,
                               uint8_t           flags,
                               xcb_colormap_t    cmap,
                               uint32_t          pixel,
                               uint16_t          name_len,
                               const char       *name);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_store_named_color (xcb_connection_t *c,
                       uint8_t           flags,
                       xcb_colormap_t    cmap,
                       uint32_t          pixel,
                       uint16_t          name_len,
                       const char       *name);

char *
xcb_store_named_color_name (const xcb_store_named_color_request_t *R);

int
xcb_store_named_color_name_length (const xcb_store_named_color_request_t *R);

xcb_generic_iterator_t
xcb_store_named_color_name_end (const xcb_store_named_color_request_t *R);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_rgb_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_rgb_t)
 */
void
xcb_rgb_next (xcb_rgb_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_rgb_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_rgb_end (xcb_rgb_iterator_t i);

int
xcb_query_colors_sizeof (const void  *_buffer,
                         uint32_t     pixels_len);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_query_colors_cookie_t
xcb_query_colors (xcb_connection_t *c,
                  xcb_colormap_t    cmap,
                  uint32_t          pixels_len,
                  const uint32_t   *pixels);

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
xcb_query_colors_cookie_t
xcb_query_colors_unchecked (xcb_connection_t *c,
                            xcb_colormap_t    cmap,
                            uint32_t          pixels_len,
                            const uint32_t   *pixels);

xcb_rgb_t *
xcb_query_colors_colors (const xcb_query_colors_reply_t *R);

int
xcb_query_colors_colors_length (const xcb_query_colors_reply_t *R);

xcb_rgb_iterator_t
xcb_query_colors_colors_iterator (const xcb_query_colors_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_query_colors_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_query_colors_reply_t *
xcb_query_colors_reply (xcb_connection_t           *c,
                        xcb_query_colors_cookie_t   cookie  /**< */,
                        xcb_generic_error_t       **e);

int
xcb_lookup_color_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_lookup_color_cookie_t
xcb_lookup_color (xcb_connection_t *c,
                  xcb_colormap_t    cmap,
                  uint16_t          name_len,
                  const char       *name);

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
xcb_lookup_color_cookie_t
xcb_lookup_color_unchecked (xcb_connection_t *c,
                            xcb_colormap_t    cmap,
                            uint16_t          name_len,
                            const char       *name);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_lookup_color_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_lookup_color_reply_t *
xcb_lookup_color_reply (xcb_connection_t           *c,
                        xcb_lookup_color_cookie_t   cookie  /**< */,
                        xcb_generic_error_t       **e);

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
xcb_create_cursor_checked (xcb_connection_t *c,
                           xcb_cursor_t      cid,
                           xcb_pixmap_t      source,
                           xcb_pixmap_t      mask,
                           uint16_t          fore_red,
                           uint16_t          fore_green,
                           uint16_t          fore_blue,
                           uint16_t          back_red,
                           uint16_t          back_green,
                           uint16_t          back_blue,
                           uint16_t          x,
                           uint16_t          y);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_create_cursor (xcb_connection_t *c,
                   xcb_cursor_t      cid,
                   xcb_pixmap_t      source,
                   xcb_pixmap_t      mask,
                   uint16_t          fore_red,
                   uint16_t          fore_green,
                   uint16_t          fore_blue,
                   uint16_t          back_red,
                   uint16_t          back_green,
                   uint16_t          back_blue,
                   uint16_t          x,
                   uint16_t          y);

/**
 * @brief create cursor
 *
 * @param c The connection
 * @param cid The ID with which you will refer to the cursor, created by `xcb_generate_id`.
 * @param source_font In which font to look for the cursor glyph.
 * @param mask_font In which font to look for the mask glyph.
 * @param source_char The glyph of \a source_font to use.
 * @param mask_char The glyph of \a mask_font to use as a mask: Pixels which are set to 1 define
 * which source pixels are displayed. All pixels which are set to 0 are not
 * displayed.
 * @param fore_red The red value of the foreground color.
 * @param fore_green The green value of the foreground color.
 * @param fore_blue The blue value of the foreground color.
 * @param back_red The red value of the background color.
 * @param back_green The green value of the background color.
 * @param back_blue The blue value of the background color.
 * @return A cookie
 *
 * Creates a cursor from a font glyph. X provides a set of standard cursor shapes
 * in a special font named cursor. Applications are encouraged to use this
 * interface for their cursors because the font can be customized for the
 * individual display type.
 * 
 * All pixels which are set to 1 in the source will use the foreground color (as
 * specified by \a fore_red, \a fore_green and \a fore_blue). All pixels set to 0
 * will use the background color (as specified by \a back_red, \a back_green and
 * \a back_blue).
 *
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
xcb_void_cookie_t
xcb_create_glyph_cursor_checked (xcb_connection_t *c,
                                 xcb_cursor_t      cid,
                                 xcb_font_t        source_font,
                                 xcb_font_t        mask_font,
                                 uint16_t          source_char,
                                 uint16_t          mask_char,
                                 uint16_t          fore_red,
                                 uint16_t          fore_green,
                                 uint16_t          fore_blue,
                                 uint16_t          back_red,
                                 uint16_t          back_green,
                                 uint16_t          back_blue);

/**
 * @brief create cursor
 *
 * @param c The connection
 * @param cid The ID with which you will refer to the cursor, created by `xcb_generate_id`.
 * @param source_font In which font to look for the cursor glyph.
 * @param mask_font In which font to look for the mask glyph.
 * @param source_char The glyph of \a source_font to use.
 * @param mask_char The glyph of \a mask_font to use as a mask: Pixels which are set to 1 define
 * which source pixels are displayed. All pixels which are set to 0 are not
 * displayed.
 * @param fore_red The red value of the foreground color.
 * @param fore_green The green value of the foreground color.
 * @param fore_blue The blue value of the foreground color.
 * @param back_red The red value of the background color.
 * @param back_green The green value of the background color.
 * @param back_blue The blue value of the background color.
 * @return A cookie
 *
 * Creates a cursor from a font glyph. X provides a set of standard cursor shapes
 * in a special font named cursor. Applications are encouraged to use this
 * interface for their cursors because the font can be customized for the
 * individual display type.
 * 
 * All pixels which are set to 1 in the source will use the foreground color (as
 * specified by \a fore_red, \a fore_green and \a fore_blue). All pixels set to 0
 * will use the background color (as specified by \a back_red, \a back_green and
 * \a back_blue).
 *
 */
xcb_void_cookie_t
xcb_create_glyph_cursor (xcb_connection_t *c,
                         xcb_cursor_t      cid,
                         xcb_font_t        source_font,
                         xcb_font_t        mask_font,
                         uint16_t          source_char,
                         uint16_t          mask_char,
                         uint16_t          fore_red,
                         uint16_t          fore_green,
                         uint16_t          fore_blue,
                         uint16_t          back_red,
                         uint16_t          back_green,
                         uint16_t          back_blue);

/**
 * @brief Deletes a cursor
 *
 * @param c The connection
 * @param cursor The cursor to destroy.
 * @return A cookie
 *
 * Deletes the association between the cursor resource ID and the specified
 * cursor. The cursor is freed when no other resource references it.
 *
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
xcb_void_cookie_t
xcb_free_cursor_checked (xcb_connection_t *c,
                         xcb_cursor_t      cursor);

/**
 * @brief Deletes a cursor
 *
 * @param c The connection
 * @param cursor The cursor to destroy.
 * @return A cookie
 *
 * Deletes the association between the cursor resource ID and the specified
 * cursor. The cursor is freed when no other resource references it.
 *
 */
xcb_void_cookie_t
xcb_free_cursor (xcb_connection_t *c,
                 xcb_cursor_t      cursor);

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
xcb_recolor_cursor_checked (xcb_connection_t *c,
                            xcb_cursor_t      cursor,
                            uint16_t          fore_red,
                            uint16_t          fore_green,
                            uint16_t          fore_blue,
                            uint16_t          back_red,
                            uint16_t          back_green,
                            uint16_t          back_blue);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_recolor_cursor (xcb_connection_t *c,
                    xcb_cursor_t      cursor,
                    uint16_t          fore_red,
                    uint16_t          fore_green,
                    uint16_t          fore_blue,
                    uint16_t          back_red,
                    uint16_t          back_green,
                    uint16_t          back_blue);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_query_best_size_cookie_t
xcb_query_best_size (xcb_connection_t *c,
                     uint8_t           _class,
                     xcb_drawable_t    drawable,
                     uint16_t          width,
                     uint16_t          height);

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
xcb_query_best_size_cookie_t
xcb_query_best_size_unchecked (xcb_connection_t *c,
                               uint8_t           _class,
                               xcb_drawable_t    drawable,
                               uint16_t          width,
                               uint16_t          height);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_query_best_size_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_query_best_size_reply_t *
xcb_query_best_size_reply (xcb_connection_t              *c,
                           xcb_query_best_size_cookie_t   cookie  /**< */,
                           xcb_generic_error_t          **e);

int
xcb_query_extension_sizeof (const void  *_buffer);

/**
 * @brief check if extension is present
 *
 * @param c The connection
 * @param name_len The length of \a name in bytes.
 * @param name The name of the extension to query, for example "RANDR". This is case
 * sensitive!
 * @return A cookie
 *
 * Determines if the specified extension is present on this X11 server.
 * 
 * Every extension has a unique `major_opcode` to identify requests, the minor
 * opcodes and request formats are extension-specific. If the extension provides
 * events and errors, the `first_event` and `first_error` fields in the reply are
 * set accordingly.
 * 
 * There should rarely be a need to use this request directly, XCB provides the
 * `xcb_get_extension_data` function instead.
 *
 */
xcb_query_extension_cookie_t
xcb_query_extension (xcb_connection_t *c,
                     uint16_t          name_len,
                     const char       *name);

/**
 * @brief check if extension is present
 *
 * @param c The connection
 * @param name_len The length of \a name in bytes.
 * @param name The name of the extension to query, for example "RANDR". This is case
 * sensitive!
 * @return A cookie
 *
 * Determines if the specified extension is present on this X11 server.
 * 
 * Every extension has a unique `major_opcode` to identify requests, the minor
 * opcodes and request formats are extension-specific. If the extension provides
 * events and errors, the `first_event` and `first_error` fields in the reply are
 * set accordingly.
 * 
 * There should rarely be a need to use this request directly, XCB provides the
 * `xcb_get_extension_data` function instead.
 *
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
xcb_query_extension_cookie_t
xcb_query_extension_unchecked (xcb_connection_t *c,
                               uint16_t          name_len,
                               const char       *name);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_query_extension_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_query_extension_reply_t *
xcb_query_extension_reply (xcb_connection_t              *c,
                           xcb_query_extension_cookie_t   cookie  /**< */,
                           xcb_generic_error_t          **e);

int
xcb_list_extensions_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_list_extensions_cookie_t
xcb_list_extensions (xcb_connection_t *c);

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
xcb_list_extensions_cookie_t
xcb_list_extensions_unchecked (xcb_connection_t *c);

int
xcb_list_extensions_names_length (const xcb_list_extensions_reply_t *R);

xcb_str_iterator_t
xcb_list_extensions_names_iterator (const xcb_list_extensions_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_list_extensions_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_list_extensions_reply_t *
xcb_list_extensions_reply (xcb_connection_t              *c,
                           xcb_list_extensions_cookie_t   cookie  /**< */,
                           xcb_generic_error_t          **e);

int
xcb_change_keyboard_mapping_sizeof (const void  *_buffer);

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
xcb_change_keyboard_mapping_checked (xcb_connection_t   *c,
                                     uint8_t             keycode_count,
                                     xcb_keycode_t       first_keycode,
                                     uint8_t             keysyms_per_keycode,
                                     const xcb_keysym_t *keysyms);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_change_keyboard_mapping (xcb_connection_t   *c,
                             uint8_t             keycode_count,
                             xcb_keycode_t       first_keycode,
                             uint8_t             keysyms_per_keycode,
                             const xcb_keysym_t *keysyms);

xcb_keysym_t *
xcb_change_keyboard_mapping_keysyms (const xcb_change_keyboard_mapping_request_t *R);

int
xcb_change_keyboard_mapping_keysyms_length (const xcb_change_keyboard_mapping_request_t *R);

xcb_generic_iterator_t
xcb_change_keyboard_mapping_keysyms_end (const xcb_change_keyboard_mapping_request_t *R);

int
xcb_get_keyboard_mapping_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_get_keyboard_mapping_cookie_t
xcb_get_keyboard_mapping (xcb_connection_t *c,
                          xcb_keycode_t     first_keycode,
                          uint8_t           count);

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
xcb_get_keyboard_mapping_cookie_t
xcb_get_keyboard_mapping_unchecked (xcb_connection_t *c,
                                    xcb_keycode_t     first_keycode,
                                    uint8_t           count);

xcb_keysym_t *
xcb_get_keyboard_mapping_keysyms (const xcb_get_keyboard_mapping_reply_t *R);

int
xcb_get_keyboard_mapping_keysyms_length (const xcb_get_keyboard_mapping_reply_t *R);

xcb_generic_iterator_t
xcb_get_keyboard_mapping_keysyms_end (const xcb_get_keyboard_mapping_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_get_keyboard_mapping_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_get_keyboard_mapping_reply_t *
xcb_get_keyboard_mapping_reply (xcb_connection_t                   *c,
                                xcb_get_keyboard_mapping_cookie_t   cookie  /**< */,
                                xcb_generic_error_t               **e);

int
xcb_change_keyboard_control_value_list_serialize (void                                           **_buffer,
                                                  uint32_t                                         value_mask,
                                                  const xcb_change_keyboard_control_value_list_t  *_aux);

int
xcb_change_keyboard_control_value_list_unpack (const void                                *_buffer,
                                               uint32_t                                   value_mask,
                                               xcb_change_keyboard_control_value_list_t  *_aux);

int
xcb_change_keyboard_control_value_list_sizeof (const void  *_buffer,
                                               uint32_t     value_mask);

int
xcb_change_keyboard_control_sizeof (const void  *_buffer);

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
xcb_change_keyboard_control_checked (xcb_connection_t *c,
                                     uint32_t          value_mask,
                                     const void       *value_list);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_change_keyboard_control (xcb_connection_t *c,
                             uint32_t          value_mask,
                             const void       *value_list);

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
xcb_change_keyboard_control_aux_checked (xcb_connection_t                               *c,
                                         uint32_t                                        value_mask,
                                         const xcb_change_keyboard_control_value_list_t *value_list);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_change_keyboard_control_aux (xcb_connection_t                               *c,
                                 uint32_t                                        value_mask,
                                 const xcb_change_keyboard_control_value_list_t *value_list);

void *
xcb_change_keyboard_control_value_list (const xcb_change_keyboard_control_request_t *R);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_get_keyboard_control_cookie_t
xcb_get_keyboard_control (xcb_connection_t *c);

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
xcb_get_keyboard_control_cookie_t
xcb_get_keyboard_control_unchecked (xcb_connection_t *c);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_get_keyboard_control_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_get_keyboard_control_reply_t *
xcb_get_keyboard_control_reply (xcb_connection_t                   *c,
                                xcb_get_keyboard_control_cookie_t   cookie  /**< */,
                                xcb_generic_error_t               **e);

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
xcb_bell_checked (xcb_connection_t *c,
                  int8_t            percent);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_bell (xcb_connection_t *c,
          int8_t            percent);

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
xcb_change_pointer_control_checked (xcb_connection_t *c,
                                    int16_t           acceleration_numerator,
                                    int16_t           acceleration_denominator,
                                    int16_t           threshold,
                                    uint8_t           do_acceleration,
                                    uint8_t           do_threshold);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_change_pointer_control (xcb_connection_t *c,
                            int16_t           acceleration_numerator,
                            int16_t           acceleration_denominator,
                            int16_t           threshold,
                            uint8_t           do_acceleration,
                            uint8_t           do_threshold);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_get_pointer_control_cookie_t
xcb_get_pointer_control (xcb_connection_t *c);

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
xcb_get_pointer_control_cookie_t
xcb_get_pointer_control_unchecked (xcb_connection_t *c);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_get_pointer_control_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_get_pointer_control_reply_t *
xcb_get_pointer_control_reply (xcb_connection_t                  *c,
                               xcb_get_pointer_control_cookie_t   cookie  /**< */,
                               xcb_generic_error_t              **e);

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
xcb_set_screen_saver_checked (xcb_connection_t *c,
                              int16_t           timeout,
                              int16_t           interval,
                              uint8_t           prefer_blanking,
                              uint8_t           allow_exposures);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_set_screen_saver (xcb_connection_t *c,
                      int16_t           timeout,
                      int16_t           interval,
                      uint8_t           prefer_blanking,
                      uint8_t           allow_exposures);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_get_screen_saver_cookie_t
xcb_get_screen_saver (xcb_connection_t *c);

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
xcb_get_screen_saver_cookie_t
xcb_get_screen_saver_unchecked (xcb_connection_t *c);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_get_screen_saver_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_get_screen_saver_reply_t *
xcb_get_screen_saver_reply (xcb_connection_t               *c,
                            xcb_get_screen_saver_cookie_t   cookie  /**< */,
                            xcb_generic_error_t           **e);

int
xcb_change_hosts_sizeof (const void  *_buffer);

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
xcb_change_hosts_checked (xcb_connection_t *c,
                          uint8_t           mode,
                          uint8_t           family,
                          uint16_t          address_len,
                          const uint8_t    *address);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_change_hosts (xcb_connection_t *c,
                  uint8_t           mode,
                  uint8_t           family,
                  uint16_t          address_len,
                  const uint8_t    *address);

uint8_t *
xcb_change_hosts_address (const xcb_change_hosts_request_t *R);

int
xcb_change_hosts_address_length (const xcb_change_hosts_request_t *R);

xcb_generic_iterator_t
xcb_change_hosts_address_end (const xcb_change_hosts_request_t *R);

int
xcb_host_sizeof (const void  *_buffer);

uint8_t *
xcb_host_address (const xcb_host_t *R);

int
xcb_host_address_length (const xcb_host_t *R);

xcb_generic_iterator_t
xcb_host_address_end (const xcb_host_t *R);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_host_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_host_t)
 */
void
xcb_host_next (xcb_host_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_host_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_host_end (xcb_host_iterator_t i);

int
xcb_list_hosts_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_list_hosts_cookie_t
xcb_list_hosts (xcb_connection_t *c);

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
xcb_list_hosts_cookie_t
xcb_list_hosts_unchecked (xcb_connection_t *c);

int
xcb_list_hosts_hosts_length (const xcb_list_hosts_reply_t *R);

xcb_host_iterator_t
xcb_list_hosts_hosts_iterator (const xcb_list_hosts_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_list_hosts_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_list_hosts_reply_t *
xcb_list_hosts_reply (xcb_connection_t         *c,
                      xcb_list_hosts_cookie_t   cookie  /**< */,
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
xcb_set_access_control_checked (xcb_connection_t *c,
                                uint8_t           mode);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_set_access_control (xcb_connection_t *c,
                        uint8_t           mode);

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
xcb_set_close_down_mode_checked (xcb_connection_t *c,
                                 uint8_t           mode);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_set_close_down_mode (xcb_connection_t *c,
                         uint8_t           mode);

/**
 * @brief kills a client
 *
 * @param c The connection
 * @param resource Any resource belonging to the client (for example a Window), used to identify
 * the client connection.
 * \n
 * The special value of `XCB_KILL_ALL_TEMPORARY`, the resources of all clients
 * that have terminated in `RetainTemporary` (TODO) are destroyed.
 * @return A cookie
 *
 * Forces a close down of the client that created the specified \a resource.
 *
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
xcb_void_cookie_t
xcb_kill_client_checked (xcb_connection_t *c,
                         uint32_t          resource);

/**
 * @brief kills a client
 *
 * @param c The connection
 * @param resource Any resource belonging to the client (for example a Window), used to identify
 * the client connection.
 * \n
 * The special value of `XCB_KILL_ALL_TEMPORARY`, the resources of all clients
 * that have terminated in `RetainTemporary` (TODO) are destroyed.
 * @return A cookie
 *
 * Forces a close down of the client that created the specified \a resource.
 *
 */
xcb_void_cookie_t
xcb_kill_client (xcb_connection_t *c,
                 uint32_t          resource);

int
xcb_rotate_properties_sizeof (const void  *_buffer);

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
xcb_rotate_properties_checked (xcb_connection_t *c,
                               xcb_window_t      window,
                               uint16_t          atoms_len,
                               int16_t           delta,
                               const xcb_atom_t *atoms);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_rotate_properties (xcb_connection_t *c,
                       xcb_window_t      window,
                       uint16_t          atoms_len,
                       int16_t           delta,
                       const xcb_atom_t *atoms);

xcb_atom_t *
xcb_rotate_properties_atoms (const xcb_rotate_properties_request_t *R);

int
xcb_rotate_properties_atoms_length (const xcb_rotate_properties_request_t *R);

xcb_generic_iterator_t
xcb_rotate_properties_atoms_end (const xcb_rotate_properties_request_t *R);

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
xcb_force_screen_saver_checked (xcb_connection_t *c,
                                uint8_t           mode);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_force_screen_saver (xcb_connection_t *c,
                        uint8_t           mode);

int
xcb_set_pointer_mapping_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_set_pointer_mapping_cookie_t
xcb_set_pointer_mapping (xcb_connection_t *c,
                         uint8_t           map_len,
                         const uint8_t    *map);

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
xcb_set_pointer_mapping_cookie_t
xcb_set_pointer_mapping_unchecked (xcb_connection_t *c,
                                   uint8_t           map_len,
                                   const uint8_t    *map);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_set_pointer_mapping_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_set_pointer_mapping_reply_t *
xcb_set_pointer_mapping_reply (xcb_connection_t                  *c,
                               xcb_set_pointer_mapping_cookie_t   cookie  /**< */,
                               xcb_generic_error_t              **e);

int
xcb_get_pointer_mapping_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_get_pointer_mapping_cookie_t
xcb_get_pointer_mapping (xcb_connection_t *c);

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
xcb_get_pointer_mapping_cookie_t
xcb_get_pointer_mapping_unchecked (xcb_connection_t *c);

uint8_t *
xcb_get_pointer_mapping_map (const xcb_get_pointer_mapping_reply_t *R);

int
xcb_get_pointer_mapping_map_length (const xcb_get_pointer_mapping_reply_t *R);

xcb_generic_iterator_t
xcb_get_pointer_mapping_map_end (const xcb_get_pointer_mapping_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_get_pointer_mapping_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_get_pointer_mapping_reply_t *
xcb_get_pointer_mapping_reply (xcb_connection_t                  *c,
                               xcb_get_pointer_mapping_cookie_t   cookie  /**< */,
                               xcb_generic_error_t              **e);

int
xcb_set_modifier_mapping_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_set_modifier_mapping_cookie_t
xcb_set_modifier_mapping (xcb_connection_t    *c,
                          uint8_t              keycodes_per_modifier,
                          const xcb_keycode_t *keycodes);

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
xcb_set_modifier_mapping_cookie_t
xcb_set_modifier_mapping_unchecked (xcb_connection_t    *c,
                                    uint8_t              keycodes_per_modifier,
                                    const xcb_keycode_t *keycodes);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_set_modifier_mapping_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_set_modifier_mapping_reply_t *
xcb_set_modifier_mapping_reply (xcb_connection_t                   *c,
                                xcb_set_modifier_mapping_cookie_t   cookie  /**< */,
                                xcb_generic_error_t               **e);

int
xcb_get_modifier_mapping_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_get_modifier_mapping_cookie_t
xcb_get_modifier_mapping (xcb_connection_t *c);

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
xcb_get_modifier_mapping_cookie_t
xcb_get_modifier_mapping_unchecked (xcb_connection_t *c);

xcb_keycode_t *
xcb_get_modifier_mapping_keycodes (const xcb_get_modifier_mapping_reply_t *R);

int
xcb_get_modifier_mapping_keycodes_length (const xcb_get_modifier_mapping_reply_t *R);

xcb_generic_iterator_t
xcb_get_modifier_mapping_keycodes_end (const xcb_get_modifier_mapping_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_get_modifier_mapping_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_get_modifier_mapping_reply_t *
xcb_get_modifier_mapping_reply (xcb_connection_t                   *c,
                                xcb_get_modifier_mapping_cookie_t   cookie  /**< */,
                                xcb_generic_error_t               **e);

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
xcb_no_operation_checked (xcb_connection_t *c);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_no_operation (xcb_connection_t *c);


#ifdef __cplusplus
}
#endif

#endif

/**
 * @}
 */
