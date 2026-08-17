/*
 * This file generated automatically from randr.xml by c_client.py.
 * Edit at your peril.
 */

/**
 * @defgroup XCB_RandR_API XCB RandR API
 * @brief RandR XCB Protocol Implementation.
 * @{
 **/

#ifndef __RANDR_H
#define __RANDR_H

#include "xcb.h"
#include "xproto.h"
#include "render.h"

#ifdef __cplusplus
extern "C" {
#endif

#define XCB_RANDR_MAJOR_VERSION 1
#define XCB_RANDR_MINOR_VERSION 6

extern xcb_extension_t xcb_randr_id;

typedef uint32_t xcb_randr_mode_t;

/**
 * @brief xcb_randr_mode_iterator_t
 **/
typedef struct xcb_randr_mode_iterator_t {
    xcb_randr_mode_t *data;
    int               rem;
    int               index;
} xcb_randr_mode_iterator_t;

typedef uint32_t xcb_randr_crtc_t;

/**
 * @brief xcb_randr_crtc_iterator_t
 **/
typedef struct xcb_randr_crtc_iterator_t {
    xcb_randr_crtc_t *data;
    int               rem;
    int               index;
} xcb_randr_crtc_iterator_t;

typedef uint32_t xcb_randr_output_t;

/**
 * @brief xcb_randr_output_iterator_t
 **/
typedef struct xcb_randr_output_iterator_t {
    xcb_randr_output_t *data;
    int                 rem;
    int                 index;
} xcb_randr_output_iterator_t;

typedef uint32_t xcb_randr_provider_t;

/**
 * @brief xcb_randr_provider_iterator_t
 **/
typedef struct xcb_randr_provider_iterator_t {
    xcb_randr_provider_t *data;
    int                   rem;
    int                   index;
} xcb_randr_provider_iterator_t;

typedef uint32_t xcb_randr_lease_t;

/**
 * @brief xcb_randr_lease_iterator_t
 **/
typedef struct xcb_randr_lease_iterator_t {
    xcb_randr_lease_t *data;
    int                rem;
    int                index;
} xcb_randr_lease_iterator_t;

/** Opcode for xcb_randr_bad_output. */
#define XCB_RANDR_BAD_OUTPUT 0

/**
 * @brief xcb_randr_bad_output_error_t
 **/
typedef struct xcb_randr_bad_output_error_t {
    uint8_t  response_type;
    uint8_t  error_code;
    uint16_t sequence;
} xcb_randr_bad_output_error_t;

/** Opcode for xcb_randr_bad_crtc. */
#define XCB_RANDR_BAD_CRTC 1

/**
 * @brief xcb_randr_bad_crtc_error_t
 **/
typedef struct xcb_randr_bad_crtc_error_t {
    uint8_t  response_type;
    uint8_t  error_code;
    uint16_t sequence;
} xcb_randr_bad_crtc_error_t;

/** Opcode for xcb_randr_bad_mode. */
#define XCB_RANDR_BAD_MODE 2

/**
 * @brief xcb_randr_bad_mode_error_t
 **/
typedef struct xcb_randr_bad_mode_error_t {
    uint8_t  response_type;
    uint8_t  error_code;
    uint16_t sequence;
} xcb_randr_bad_mode_error_t;

/** Opcode for xcb_randr_bad_provider. */
#define XCB_RANDR_BAD_PROVIDER 3

/**
 * @brief xcb_randr_bad_provider_error_t
 **/
typedef struct xcb_randr_bad_provider_error_t {
    uint8_t  response_type;
    uint8_t  error_code;
    uint16_t sequence;
} xcb_randr_bad_provider_error_t;

typedef enum xcb_randr_rotation_t {
    XCB_RANDR_ROTATION_ROTATE_0 = 1,
    XCB_RANDR_ROTATION_ROTATE_90 = 2,
    XCB_RANDR_ROTATION_ROTATE_180 = 4,
    XCB_RANDR_ROTATION_ROTATE_270 = 8,
    XCB_RANDR_ROTATION_REFLECT_X = 16,
    XCB_RANDR_ROTATION_REFLECT_Y = 32
} xcb_randr_rotation_t;

/**
 * @brief xcb_randr_screen_size_t
 **/
typedef struct xcb_randr_screen_size_t {
    uint16_t width;
    uint16_t height;
    uint16_t mwidth;
    uint16_t mheight;
} xcb_randr_screen_size_t;

/**
 * @brief xcb_randr_screen_size_iterator_t
 **/
typedef struct xcb_randr_screen_size_iterator_t {
    xcb_randr_screen_size_t *data;
    int                      rem;
    int                      index;
} xcb_randr_screen_size_iterator_t;

/**
 * @brief xcb_randr_refresh_rates_t
 **/
typedef struct xcb_randr_refresh_rates_t {
    uint16_t nRates;
} xcb_randr_refresh_rates_t;

/**
 * @brief xcb_randr_refresh_rates_iterator_t
 **/
typedef struct xcb_randr_refresh_rates_iterator_t {
    xcb_randr_refresh_rates_t *data;
    int                        rem;
    int                        index;
} xcb_randr_refresh_rates_iterator_t;

/**
 * @brief xcb_randr_query_version_cookie_t
 **/
typedef struct xcb_randr_query_version_cookie_t {
    unsigned int sequence;
} xcb_randr_query_version_cookie_t;

/** Opcode for xcb_randr_query_version. */
#define XCB_RANDR_QUERY_VERSION 0

/**
 * @brief xcb_randr_query_version_request_t
 **/
typedef struct xcb_randr_query_version_request_t {
    uint8_t  major_opcode;
    uint8_t  minor_opcode;
    uint16_t length;
    uint32_t major_version;
    uint32_t minor_version;
} xcb_randr_query_version_request_t;

/**
 * @brief xcb_randr_query_version_reply_t
 **/
typedef struct xcb_randr_query_version_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint32_t major_version;
    uint32_t minor_version;
    uint8_t  pad1[16];
} xcb_randr_query_version_reply_t;

typedef enum xcb_randr_set_config_t {
    XCB_RANDR_SET_CONFIG_SUCCESS = 0,
    XCB_RANDR_SET_CONFIG_INVALID_CONFIG_TIME = 1,
    XCB_RANDR_SET_CONFIG_INVALID_TIME = 2,
    XCB_RANDR_SET_CONFIG_FAILED = 3
} xcb_randr_set_config_t;

/**
 * @brief xcb_randr_set_screen_config_cookie_t
 **/
typedef struct xcb_randr_set_screen_config_cookie_t {
    unsigned int sequence;
} xcb_randr_set_screen_config_cookie_t;

/** Opcode for xcb_randr_set_screen_config. */
#define XCB_RANDR_SET_SCREEN_CONFIG 2

/**
 * @brief xcb_randr_set_screen_config_request_t
 **/
typedef struct xcb_randr_set_screen_config_request_t {
    uint8_t         major_opcode;
    uint8_t         minor_opcode;
    uint16_t        length;
    xcb_window_t    window;
    xcb_timestamp_t timestamp;
    xcb_timestamp_t config_timestamp;
    uint16_t        sizeID;
    uint16_t        rotation;
    uint16_t        rate;
    uint8_t         pad0[2];
} xcb_randr_set_screen_config_request_t;

/**
 * @brief xcb_randr_set_screen_config_reply_t
 **/
typedef struct xcb_randr_set_screen_config_reply_t {
    uint8_t         response_type;
    uint8_t         status;
    uint16_t        sequence;
    uint32_t        length;
    xcb_timestamp_t new_timestamp;
    xcb_timestamp_t config_timestamp;
    xcb_window_t    root;
    uint16_t        subpixel_order;
    uint8_t         pad0[10];
} xcb_randr_set_screen_config_reply_t;

typedef enum xcb_randr_notify_mask_t {
    XCB_RANDR_NOTIFY_MASK_SCREEN_CHANGE = 1,
    XCB_RANDR_NOTIFY_MASK_CRTC_CHANGE = 2,
    XCB_RANDR_NOTIFY_MASK_OUTPUT_CHANGE = 4,
    XCB_RANDR_NOTIFY_MASK_OUTPUT_PROPERTY = 8,
    XCB_RANDR_NOTIFY_MASK_PROVIDER_CHANGE = 16,
    XCB_RANDR_NOTIFY_MASK_PROVIDER_PROPERTY = 32,
    XCB_RANDR_NOTIFY_MASK_RESOURCE_CHANGE = 64,
    XCB_RANDR_NOTIFY_MASK_LEASE = 128
} xcb_randr_notify_mask_t;

/** Opcode for xcb_randr_select_input. */
#define XCB_RANDR_SELECT_INPUT 4

/**
 * @brief xcb_randr_select_input_request_t
 **/
typedef struct xcb_randr_select_input_request_t {
    uint8_t      major_opcode;
    uint8_t      minor_opcode;
    uint16_t     length;
    xcb_window_t window;
    uint16_t     enable;
    uint8_t      pad0[2];
} xcb_randr_select_input_request_t;

/**
 * @brief xcb_randr_get_screen_info_cookie_t
 **/
typedef struct xcb_randr_get_screen_info_cookie_t {
    unsigned int sequence;
} xcb_randr_get_screen_info_cookie_t;

/** Opcode for xcb_randr_get_screen_info. */
#define XCB_RANDR_GET_SCREEN_INFO 5

/**
 * @brief xcb_randr_get_screen_info_request_t
 **/
typedef struct xcb_randr_get_screen_info_request_t {
    uint8_t      major_opcode;
    uint8_t      minor_opcode;
    uint16_t     length;
    xcb_window_t window;
} xcb_randr_get_screen_info_request_t;

/**
 * @brief xcb_randr_get_screen_info_reply_t
 **/
typedef struct xcb_randr_get_screen_info_reply_t {
    uint8_t         response_type;
    uint8_t         rotations;
    uint16_t        sequence;
    uint32_t        length;
    xcb_window_t    root;
    xcb_timestamp_t timestamp;
    xcb_timestamp_t config_timestamp;
    uint16_t        nSizes;
    uint16_t        sizeID;
    uint16_t        rotation;
    uint16_t        rate;
    uint16_t        nInfo;
    uint8_t         pad0[2];
} xcb_randr_get_screen_info_reply_t;

/**
 * @brief xcb_randr_get_screen_size_range_cookie_t
 **/
typedef struct xcb_randr_get_screen_size_range_cookie_t {
    unsigned int sequence;
} xcb_randr_get_screen_size_range_cookie_t;

/** Opcode for xcb_randr_get_screen_size_range. */
#define XCB_RANDR_GET_SCREEN_SIZE_RANGE 6

/**
 * @brief xcb_randr_get_screen_size_range_request_t
 **/
typedef struct xcb_randr_get_screen_size_range_request_t {
    uint8_t      major_opcode;
    uint8_t      minor_opcode;
    uint16_t     length;
    xcb_window_t window;
} xcb_randr_get_screen_size_range_request_t;

/**
 * @brief xcb_randr_get_screen_size_range_reply_t
 **/
typedef struct xcb_randr_get_screen_size_range_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint16_t min_width;
    uint16_t min_height;
    uint16_t max_width;
    uint16_t max_height;
    uint8_t  pad1[16];
} xcb_randr_get_screen_size_range_reply_t;

/** Opcode for xcb_randr_set_screen_size. */
#define XCB_RANDR_SET_SCREEN_SIZE 7

/**
 * @brief xcb_randr_set_screen_size_request_t
 **/
typedef struct xcb_randr_set_screen_size_request_t {
    uint8_t      major_opcode;
    uint8_t      minor_opcode;
    uint16_t     length;
    xcb_window_t window;
    uint16_t     width;
    uint16_t     height;
    uint32_t     mm_width;
    uint32_t     mm_height;
} xcb_randr_set_screen_size_request_t;

typedef enum xcb_randr_mode_flag_t {
    XCB_RANDR_MODE_FLAG_HSYNC_POSITIVE = 1,
    XCB_RANDR_MODE_FLAG_HSYNC_NEGATIVE = 2,
    XCB_RANDR_MODE_FLAG_VSYNC_POSITIVE = 4,
    XCB_RANDR_MODE_FLAG_VSYNC_NEGATIVE = 8,
    XCB_RANDR_MODE_FLAG_INTERLACE = 16,
    XCB_RANDR_MODE_FLAG_DOUBLE_SCAN = 32,
    XCB_RANDR_MODE_FLAG_CSYNC = 64,
    XCB_RANDR_MODE_FLAG_CSYNC_POSITIVE = 128,
    XCB_RANDR_MODE_FLAG_CSYNC_NEGATIVE = 256,
    XCB_RANDR_MODE_FLAG_HSKEW_PRESENT = 512,
    XCB_RANDR_MODE_FLAG_BCAST = 1024,
    XCB_RANDR_MODE_FLAG_PIXEL_MULTIPLEX = 2048,
    XCB_RANDR_MODE_FLAG_DOUBLE_CLOCK = 4096,
    XCB_RANDR_MODE_FLAG_HALVE_CLOCK = 8192
} xcb_randr_mode_flag_t;

/**
 * @brief xcb_randr_mode_info_t
 **/
typedef struct xcb_randr_mode_info_t {
    uint32_t id;
    uint16_t width;
    uint16_t height;
    uint32_t dot_clock;
    uint16_t hsync_start;
    uint16_t hsync_end;
    uint16_t htotal;
    uint16_t hskew;
    uint16_t vsync_start;
    uint16_t vsync_end;
    uint16_t vtotal;
    uint16_t name_len;
    uint32_t mode_flags;
} xcb_randr_mode_info_t;

/**
 * @brief xcb_randr_mode_info_iterator_t
 **/
typedef struct xcb_randr_mode_info_iterator_t {
    xcb_randr_mode_info_t *data;
    int                    rem;
    int                    index;
} xcb_randr_mode_info_iterator_t;

/**
 * @brief xcb_randr_get_screen_resources_cookie_t
 **/
typedef struct xcb_randr_get_screen_resources_cookie_t {
    unsigned int sequence;
} xcb_randr_get_screen_resources_cookie_t;

/** Opcode for xcb_randr_get_screen_resources. */
#define XCB_RANDR_GET_SCREEN_RESOURCES 8

/**
 * @brief xcb_randr_get_screen_resources_request_t
 **/
typedef struct xcb_randr_get_screen_resources_request_t {
    uint8_t      major_opcode;
    uint8_t      minor_opcode;
    uint16_t     length;
    xcb_window_t window;
} xcb_randr_get_screen_resources_request_t;

/**
 * @brief xcb_randr_get_screen_resources_reply_t
 **/
typedef struct xcb_randr_get_screen_resources_reply_t {
    uint8_t         response_type;
    uint8_t         pad0;
    uint16_t        sequence;
    uint32_t        length;
    xcb_timestamp_t timestamp;
    xcb_timestamp_t config_timestamp;
    uint16_t        num_crtcs;
    uint16_t        num_outputs;
    uint16_t        num_modes;
    uint16_t        names_len;
    uint8_t         pad1[8];
} xcb_randr_get_screen_resources_reply_t;

typedef enum xcb_randr_connection_t {
    XCB_RANDR_CONNECTION_CONNECTED = 0,
    XCB_RANDR_CONNECTION_DISCONNECTED = 1,
    XCB_RANDR_CONNECTION_UNKNOWN = 2
} xcb_randr_connection_t;

/**
 * @brief xcb_randr_get_output_info_cookie_t
 **/
typedef struct xcb_randr_get_output_info_cookie_t {
    unsigned int sequence;
} xcb_randr_get_output_info_cookie_t;

/** Opcode for xcb_randr_get_output_info. */
#define XCB_RANDR_GET_OUTPUT_INFO 9

/**
 * @brief xcb_randr_get_output_info_request_t
 **/
typedef struct xcb_randr_get_output_info_request_t {
    uint8_t            major_opcode;
    uint8_t            minor_opcode;
    uint16_t           length;
    xcb_randr_output_t output;
    xcb_timestamp_t    config_timestamp;
} xcb_randr_get_output_info_request_t;

/**
 * @brief xcb_randr_get_output_info_reply_t
 **/
typedef struct xcb_randr_get_output_info_reply_t {
    uint8_t          response_type;
    uint8_t          status;
    uint16_t         sequence;
    uint32_t         length;
    xcb_timestamp_t  timestamp;
    xcb_randr_crtc_t crtc;
    uint32_t         mm_width;
    uint32_t         mm_height;
    uint8_t          connection;
    uint8_t          subpixel_order;
    uint16_t         num_crtcs;
    uint16_t         num_modes;
    uint16_t         num_preferred;
    uint16_t         num_clones;
    uint16_t         name_len;
} xcb_randr_get_output_info_reply_t;

/**
 * @brief xcb_randr_list_output_properties_cookie_t
 **/
typedef struct xcb_randr_list_output_properties_cookie_t {
    unsigned int sequence;
} xcb_randr_list_output_properties_cookie_t;

/** Opcode for xcb_randr_list_output_properties. */
#define XCB_RANDR_LIST_OUTPUT_PROPERTIES 10

/**
 * @brief xcb_randr_list_output_properties_request_t
 **/
typedef struct xcb_randr_list_output_properties_request_t {
    uint8_t            major_opcode;
    uint8_t            minor_opcode;
    uint16_t           length;
    xcb_randr_output_t output;
} xcb_randr_list_output_properties_request_t;

/**
 * @brief xcb_randr_list_output_properties_reply_t
 **/
typedef struct xcb_randr_list_output_properties_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint16_t num_atoms;
    uint8_t  pad1[22];
} xcb_randr_list_output_properties_reply_t;

/**
 * @brief xcb_randr_query_output_property_cookie_t
 **/
typedef struct xcb_randr_query_output_property_cookie_t {
    unsigned int sequence;
} xcb_randr_query_output_property_cookie_t;

/** Opcode for xcb_randr_query_output_property. */
#define XCB_RANDR_QUERY_OUTPUT_PROPERTY 11

/**
 * @brief xcb_randr_query_output_property_request_t
 **/
typedef struct xcb_randr_query_output_property_request_t {
    uint8_t            major_opcode;
    uint8_t            minor_opcode;
    uint16_t           length;
    xcb_randr_output_t output;
    xcb_atom_t         property;
} xcb_randr_query_output_property_request_t;

/**
 * @brief xcb_randr_query_output_property_reply_t
 **/
typedef struct xcb_randr_query_output_property_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint8_t  pending;
    uint8_t  range;
    uint8_t  immutable;
    uint8_t  pad1[21];
} xcb_randr_query_output_property_reply_t;

/** Opcode for xcb_randr_configure_output_property. */
#define XCB_RANDR_CONFIGURE_OUTPUT_PROPERTY 12

/**
 * @brief xcb_randr_configure_output_property_request_t
 **/
typedef struct xcb_randr_configure_output_property_request_t {
    uint8_t            major_opcode;
    uint8_t            minor_opcode;
    uint16_t           length;
    xcb_randr_output_t output;
    xcb_atom_t         property;
    uint8_t            pending;
    uint8_t            range;
    uint8_t            pad0[2];
} xcb_randr_configure_output_property_request_t;

/** Opcode for xcb_randr_change_output_property. */
#define XCB_RANDR_CHANGE_OUTPUT_PROPERTY 13

/**
 * @brief xcb_randr_change_output_property_request_t
 **/
typedef struct xcb_randr_change_output_property_request_t {
    uint8_t            major_opcode;
    uint8_t            minor_opcode;
    uint16_t           length;
    xcb_randr_output_t output;
    xcb_atom_t         property;
    xcb_atom_t         type;
    uint8_t            format;
    uint8_t            mode;
    uint8_t            pad0[2];
    uint32_t           num_units;
} xcb_randr_change_output_property_request_t;

/** Opcode for xcb_randr_delete_output_property. */
#define XCB_RANDR_DELETE_OUTPUT_PROPERTY 14

/**
 * @brief xcb_randr_delete_output_property_request_t
 **/
typedef struct xcb_randr_delete_output_property_request_t {
    uint8_t            major_opcode;
    uint8_t            minor_opcode;
    uint16_t           length;
    xcb_randr_output_t output;
    xcb_atom_t         property;
} xcb_randr_delete_output_property_request_t;

/**
 * @brief xcb_randr_get_output_property_cookie_t
 **/
typedef struct xcb_randr_get_output_property_cookie_t {
    unsigned int sequence;
} xcb_randr_get_output_property_cookie_t;

/** Opcode for xcb_randr_get_output_property. */
#define XCB_RANDR_GET_OUTPUT_PROPERTY 15

/**
 * @brief xcb_randr_get_output_property_request_t
 **/
typedef struct xcb_randr_get_output_property_request_t {
    uint8_t            major_opcode;
    uint8_t            minor_opcode;
    uint16_t           length;
    xcb_randr_output_t output;
    xcb_atom_t         property;
    xcb_atom_t         type;
    uint32_t           long_offset;
    uint32_t           long_length;
    uint8_t            _delete;
    uint8_t            pending;
    uint8_t            pad0[2];
} xcb_randr_get_output_property_request_t;

/**
 * @brief xcb_randr_get_output_property_reply_t
 **/
typedef struct xcb_randr_get_output_property_reply_t {
    uint8_t    response_type;
    uint8_t    format;
    uint16_t   sequence;
    uint32_t   length;
    xcb_atom_t type;
    uint32_t   bytes_after;
    uint32_t   num_items;
    uint8_t    pad0[12];
} xcb_randr_get_output_property_reply_t;

/**
 * @brief xcb_randr_create_mode_cookie_t
 **/
typedef struct xcb_randr_create_mode_cookie_t {
    unsigned int sequence;
} xcb_randr_create_mode_cookie_t;

/** Opcode for xcb_randr_create_mode. */
#define XCB_RANDR_CREATE_MODE 16

/**
 * @brief xcb_randr_create_mode_request_t
 **/
typedef struct xcb_randr_create_mode_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_window_t          window;
    xcb_randr_mode_info_t mode_info;
} xcb_randr_create_mode_request_t;

/**
 * @brief xcb_randr_create_mode_reply_t
 **/
typedef struct xcb_randr_create_mode_reply_t {
    uint8_t          response_type;
    uint8_t          pad0;
    uint16_t         sequence;
    uint32_t         length;
    xcb_randr_mode_t mode;
    uint8_t          pad1[20];
} xcb_randr_create_mode_reply_t;

/** Opcode for xcb_randr_destroy_mode. */
#define XCB_RANDR_DESTROY_MODE 17

/**
 * @brief xcb_randr_destroy_mode_request_t
 **/
typedef struct xcb_randr_destroy_mode_request_t {
    uint8_t          major_opcode;
    uint8_t          minor_opcode;
    uint16_t         length;
    xcb_randr_mode_t mode;
} xcb_randr_destroy_mode_request_t;

/** Opcode for xcb_randr_add_output_mode. */
#define XCB_RANDR_ADD_OUTPUT_MODE 18

/**
 * @brief xcb_randr_add_output_mode_request_t
 **/
typedef struct xcb_randr_add_output_mode_request_t {
    uint8_t            major_opcode;
    uint8_t            minor_opcode;
    uint16_t           length;
    xcb_randr_output_t output;
    xcb_randr_mode_t   mode;
} xcb_randr_add_output_mode_request_t;

/** Opcode for xcb_randr_delete_output_mode. */
#define XCB_RANDR_DELETE_OUTPUT_MODE 19

/**
 * @brief xcb_randr_delete_output_mode_request_t
 **/
typedef struct xcb_randr_delete_output_mode_request_t {
    uint8_t            major_opcode;
    uint8_t            minor_opcode;
    uint16_t           length;
    xcb_randr_output_t output;
    xcb_randr_mode_t   mode;
} xcb_randr_delete_output_mode_request_t;

/**
 * @brief xcb_randr_get_crtc_info_cookie_t
 **/
typedef struct xcb_randr_get_crtc_info_cookie_t {
    unsigned int sequence;
} xcb_randr_get_crtc_info_cookie_t;

/** Opcode for xcb_randr_get_crtc_info. */
#define XCB_RANDR_GET_CRTC_INFO 20

/**
 * @brief xcb_randr_get_crtc_info_request_t
 **/
typedef struct xcb_randr_get_crtc_info_request_t {
    uint8_t          major_opcode;
    uint8_t          minor_opcode;
    uint16_t         length;
    xcb_randr_crtc_t crtc;
    xcb_timestamp_t  config_timestamp;
} xcb_randr_get_crtc_info_request_t;

/**
 * @brief xcb_randr_get_crtc_info_reply_t
 **/
typedef struct xcb_randr_get_crtc_info_reply_t {
    uint8_t          response_type;
    uint8_t          status;
    uint16_t         sequence;
    uint32_t         length;
    xcb_timestamp_t  timestamp;
    int16_t          x;
    int16_t          y;
    uint16_t         width;
    uint16_t         height;
    xcb_randr_mode_t mode;
    uint16_t         rotation;
    uint16_t         rotations;
    uint16_t         num_outputs;
    uint16_t         num_possible_outputs;
} xcb_randr_get_crtc_info_reply_t;

/**
 * @brief xcb_randr_set_crtc_config_cookie_t
 **/
typedef struct xcb_randr_set_crtc_config_cookie_t {
    unsigned int sequence;
} xcb_randr_set_crtc_config_cookie_t;

/** Opcode for xcb_randr_set_crtc_config. */
#define XCB_RANDR_SET_CRTC_CONFIG 21

/**
 * @brief xcb_randr_set_crtc_config_request_t
 **/
typedef struct xcb_randr_set_crtc_config_request_t {
    uint8_t          major_opcode;
    uint8_t          minor_opcode;
    uint16_t         length;
    xcb_randr_crtc_t crtc;
    xcb_timestamp_t  timestamp;
    xcb_timestamp_t  config_timestamp;
    int16_t          x;
    int16_t          y;
    xcb_randr_mode_t mode;
    uint16_t         rotation;
    uint8_t          pad0[2];
} xcb_randr_set_crtc_config_request_t;

/**
 * @brief xcb_randr_set_crtc_config_reply_t
 **/
typedef struct xcb_randr_set_crtc_config_reply_t {
    uint8_t         response_type;
    uint8_t         status;
    uint16_t        sequence;
    uint32_t        length;
    xcb_timestamp_t timestamp;
    uint8_t         pad0[20];
} xcb_randr_set_crtc_config_reply_t;

/**
 * @brief xcb_randr_get_crtc_gamma_size_cookie_t
 **/
typedef struct xcb_randr_get_crtc_gamma_size_cookie_t {
    unsigned int sequence;
} xcb_randr_get_crtc_gamma_size_cookie_t;

/** Opcode for xcb_randr_get_crtc_gamma_size. */
#define XCB_RANDR_GET_CRTC_GAMMA_SIZE 22

/**
 * @brief xcb_randr_get_crtc_gamma_size_request_t
 **/
typedef struct xcb_randr_get_crtc_gamma_size_request_t {
    uint8_t          major_opcode;
    uint8_t          minor_opcode;
    uint16_t         length;
    xcb_randr_crtc_t crtc;
} xcb_randr_get_crtc_gamma_size_request_t;

/**
 * @brief xcb_randr_get_crtc_gamma_size_reply_t
 **/
typedef struct xcb_randr_get_crtc_gamma_size_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint16_t size;
    uint8_t  pad1[22];
} xcb_randr_get_crtc_gamma_size_reply_t;

/**
 * @brief xcb_randr_get_crtc_gamma_cookie_t
 **/
typedef struct xcb_randr_get_crtc_gamma_cookie_t {
    unsigned int sequence;
} xcb_randr_get_crtc_gamma_cookie_t;

/** Opcode for xcb_randr_get_crtc_gamma. */
#define XCB_RANDR_GET_CRTC_GAMMA 23

/**
 * @brief xcb_randr_get_crtc_gamma_request_t
 **/
typedef struct xcb_randr_get_crtc_gamma_request_t {
    uint8_t          major_opcode;
    uint8_t          minor_opcode;
    uint16_t         length;
    xcb_randr_crtc_t crtc;
} xcb_randr_get_crtc_gamma_request_t;

/**
 * @brief xcb_randr_get_crtc_gamma_reply_t
 **/
typedef struct xcb_randr_get_crtc_gamma_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint16_t size;
    uint8_t  pad1[22];
} xcb_randr_get_crtc_gamma_reply_t;

/** Opcode for xcb_randr_set_crtc_gamma. */
#define XCB_RANDR_SET_CRTC_GAMMA 24

/**
 * @brief xcb_randr_set_crtc_gamma_request_t
 **/
typedef struct xcb_randr_set_crtc_gamma_request_t {
    uint8_t          major_opcode;
    uint8_t          minor_opcode;
    uint16_t         length;
    xcb_randr_crtc_t crtc;
    uint16_t         size;
    uint8_t          pad0[2];
} xcb_randr_set_crtc_gamma_request_t;

/**
 * @brief xcb_randr_get_screen_resources_current_cookie_t
 **/
typedef struct xcb_randr_get_screen_resources_current_cookie_t {
    unsigned int sequence;
} xcb_randr_get_screen_resources_current_cookie_t;

/** Opcode for xcb_randr_get_screen_resources_current. */
#define XCB_RANDR_GET_SCREEN_RESOURCES_CURRENT 25

/**
 * @brief xcb_randr_get_screen_resources_current_request_t
 **/
typedef struct xcb_randr_get_screen_resources_current_request_t {
    uint8_t      major_opcode;
    uint8_t      minor_opcode;
    uint16_t     length;
    xcb_window_t window;
} xcb_randr_get_screen_resources_current_request_t;

/**
 * @brief xcb_randr_get_screen_resources_current_reply_t
 **/
typedef struct xcb_randr_get_screen_resources_current_reply_t {
    uint8_t         response_type;
    uint8_t         pad0;
    uint16_t        sequence;
    uint32_t        length;
    xcb_timestamp_t timestamp;
    xcb_timestamp_t config_timestamp;
    uint16_t        num_crtcs;
    uint16_t        num_outputs;
    uint16_t        num_modes;
    uint16_t        names_len;
    uint8_t         pad1[8];
} xcb_randr_get_screen_resources_current_reply_t;

typedef enum xcb_randr_transform_t {
    XCB_RANDR_TRANSFORM_UNIT = 1,
    XCB_RANDR_TRANSFORM_SCALE_UP = 2,
    XCB_RANDR_TRANSFORM_SCALE_DOWN = 4,
    XCB_RANDR_TRANSFORM_PROJECTIVE = 8
} xcb_randr_transform_t;

/** Opcode for xcb_randr_set_crtc_transform. */
#define XCB_RANDR_SET_CRTC_TRANSFORM 26

/**
 * @brief xcb_randr_set_crtc_transform_request_t
 **/
typedef struct xcb_randr_set_crtc_transform_request_t {
    uint8_t                major_opcode;
    uint8_t                minor_opcode;
    uint16_t               length;
    xcb_randr_crtc_t       crtc;
    xcb_render_transform_t transform;
    uint16_t               filter_len;
    uint8_t                pad0[2];
} xcb_randr_set_crtc_transform_request_t;

/**
 * @brief xcb_randr_get_crtc_transform_cookie_t
 **/
typedef struct xcb_randr_get_crtc_transform_cookie_t {
    unsigned int sequence;
} xcb_randr_get_crtc_transform_cookie_t;

/** Opcode for xcb_randr_get_crtc_transform. */
#define XCB_RANDR_GET_CRTC_TRANSFORM 27

/**
 * @brief xcb_randr_get_crtc_transform_request_t
 **/
typedef struct xcb_randr_get_crtc_transform_request_t {
    uint8_t          major_opcode;
    uint8_t          minor_opcode;
    uint16_t         length;
    xcb_randr_crtc_t crtc;
} xcb_randr_get_crtc_transform_request_t;

/**
 * @brief xcb_randr_get_crtc_transform_reply_t
 **/
typedef struct xcb_randr_get_crtc_transform_reply_t {
    uint8_t                response_type;
    uint8_t                pad0;
    uint16_t               sequence;
    uint32_t               length;
    xcb_render_transform_t pending_transform;
    uint8_t                has_transforms;
    uint8_t                pad1[3];
    xcb_render_transform_t current_transform;
    uint8_t                pad2[4];
    uint16_t               pending_len;
    uint16_t               pending_nparams;
    uint16_t               current_len;
    uint16_t               current_nparams;
} xcb_randr_get_crtc_transform_reply_t;

/**
 * @brief xcb_randr_get_panning_cookie_t
 **/
typedef struct xcb_randr_get_panning_cookie_t {
    unsigned int sequence;
} xcb_randr_get_panning_cookie_t;

/** Opcode for xcb_randr_get_panning. */
#define XCB_RANDR_GET_PANNING 28

/**
 * @brief xcb_randr_get_panning_request_t
 **/
typedef struct xcb_randr_get_panning_request_t {
    uint8_t          major_opcode;
    uint8_t          minor_opcode;
    uint16_t         length;
    xcb_randr_crtc_t crtc;
} xcb_randr_get_panning_request_t;

/**
 * @brief xcb_randr_get_panning_reply_t
 **/
typedef struct xcb_randr_get_panning_reply_t {
    uint8_t         response_type;
    uint8_t         status;
    uint16_t        sequence;
    uint32_t        length;
    xcb_timestamp_t timestamp;
    uint16_t        left;
    uint16_t        top;
    uint16_t        width;
    uint16_t        height;
    uint16_t        track_left;
    uint16_t        track_top;
    uint16_t        track_width;
    uint16_t        track_height;
    int16_t         border_left;
    int16_t         border_top;
    int16_t         border_right;
    int16_t         border_bottom;
} xcb_randr_get_panning_reply_t;

/**
 * @brief xcb_randr_set_panning_cookie_t
 **/
typedef struct xcb_randr_set_panning_cookie_t {
    unsigned int sequence;
} xcb_randr_set_panning_cookie_t;

/** Opcode for xcb_randr_set_panning. */
#define XCB_RANDR_SET_PANNING 29

/**
 * @brief xcb_randr_set_panning_request_t
 **/
typedef struct xcb_randr_set_panning_request_t {
    uint8_t          major_opcode;
    uint8_t          minor_opcode;
    uint16_t         length;
    xcb_randr_crtc_t crtc;
    xcb_timestamp_t  timestamp;
    uint16_t         left;
    uint16_t         top;
    uint16_t         width;
    uint16_t         height;
    uint16_t         track_left;
    uint16_t         track_top;
    uint16_t         track_width;
    uint16_t         track_height;
    int16_t          border_left;
    int16_t          border_top;
    int16_t          border_right;
    int16_t          border_bottom;
} xcb_randr_set_panning_request_t;

/**
 * @brief xcb_randr_set_panning_reply_t
 **/
typedef struct xcb_randr_set_panning_reply_t {
    uint8_t         response_type;
    uint8_t         status;
    uint16_t        sequence;
    uint32_t        length;
    xcb_timestamp_t timestamp;
} xcb_randr_set_panning_reply_t;

/** Opcode for xcb_randr_set_output_primary. */
#define XCB_RANDR_SET_OUTPUT_PRIMARY 30

/**
 * @brief xcb_randr_set_output_primary_request_t
 **/
typedef struct xcb_randr_set_output_primary_request_t {
    uint8_t            major_opcode;
    uint8_t            minor_opcode;
    uint16_t           length;
    xcb_window_t       window;
    xcb_randr_output_t output;
} xcb_randr_set_output_primary_request_t;

/**
 * @brief xcb_randr_get_output_primary_cookie_t
 **/
typedef struct xcb_randr_get_output_primary_cookie_t {
    unsigned int sequence;
} xcb_randr_get_output_primary_cookie_t;

/** Opcode for xcb_randr_get_output_primary. */
#define XCB_RANDR_GET_OUTPUT_PRIMARY 31

/**
 * @brief xcb_randr_get_output_primary_request_t
 **/
typedef struct xcb_randr_get_output_primary_request_t {
    uint8_t      major_opcode;
    uint8_t      minor_opcode;
    uint16_t     length;
    xcb_window_t window;
} xcb_randr_get_output_primary_request_t;

/**
 * @brief xcb_randr_get_output_primary_reply_t
 **/
typedef struct xcb_randr_get_output_primary_reply_t {
    uint8_t            response_type;
    uint8_t            pad0;
    uint16_t           sequence;
    uint32_t           length;
    xcb_randr_output_t output;
} xcb_randr_get_output_primary_reply_t;

/**
 * @brief xcb_randr_get_providers_cookie_t
 **/
typedef struct xcb_randr_get_providers_cookie_t {
    unsigned int sequence;
} xcb_randr_get_providers_cookie_t;

/** Opcode for xcb_randr_get_providers. */
#define XCB_RANDR_GET_PROVIDERS 32

/**
 * @brief xcb_randr_get_providers_request_t
 **/
typedef struct xcb_randr_get_providers_request_t {
    uint8_t      major_opcode;
    uint8_t      minor_opcode;
    uint16_t     length;
    xcb_window_t window;
} xcb_randr_get_providers_request_t;

/**
 * @brief xcb_randr_get_providers_reply_t
 **/
typedef struct xcb_randr_get_providers_reply_t {
    uint8_t         response_type;
    uint8_t         pad0;
    uint16_t        sequence;
    uint32_t        length;
    xcb_timestamp_t timestamp;
    uint16_t        num_providers;
    uint8_t         pad1[18];
} xcb_randr_get_providers_reply_t;

typedef enum xcb_randr_provider_capability_t {
    XCB_RANDR_PROVIDER_CAPABILITY_SOURCE_OUTPUT = 1,
    XCB_RANDR_PROVIDER_CAPABILITY_SINK_OUTPUT = 2,
    XCB_RANDR_PROVIDER_CAPABILITY_SOURCE_OFFLOAD = 4,
    XCB_RANDR_PROVIDER_CAPABILITY_SINK_OFFLOAD = 8
} xcb_randr_provider_capability_t;

/**
 * @brief xcb_randr_get_provider_info_cookie_t
 **/
typedef struct xcb_randr_get_provider_info_cookie_t {
    unsigned int sequence;
} xcb_randr_get_provider_info_cookie_t;

/** Opcode for xcb_randr_get_provider_info. */
#define XCB_RANDR_GET_PROVIDER_INFO 33

/**
 * @brief xcb_randr_get_provider_info_request_t
 **/
typedef struct xcb_randr_get_provider_info_request_t {
    uint8_t              major_opcode;
    uint8_t              minor_opcode;
    uint16_t             length;
    xcb_randr_provider_t provider;
    xcb_timestamp_t      config_timestamp;
} xcb_randr_get_provider_info_request_t;

/**
 * @brief xcb_randr_get_provider_info_reply_t
 **/
typedef struct xcb_randr_get_provider_info_reply_t {
    uint8_t         response_type;
    uint8_t         status;
    uint16_t        sequence;
    uint32_t        length;
    xcb_timestamp_t timestamp;
    uint32_t        capabilities;
    uint16_t        num_crtcs;
    uint16_t        num_outputs;
    uint16_t        num_associated_providers;
    uint16_t        name_len;
    uint8_t         pad0[8];
} xcb_randr_get_provider_info_reply_t;

/** Opcode for xcb_randr_set_provider_offload_sink. */
#define XCB_RANDR_SET_PROVIDER_OFFLOAD_SINK 34

/**
 * @brief xcb_randr_set_provider_offload_sink_request_t
 **/
typedef struct xcb_randr_set_provider_offload_sink_request_t {
    uint8_t              major_opcode;
    uint8_t              minor_opcode;
    uint16_t             length;
    xcb_randr_provider_t provider;
    xcb_randr_provider_t sink_provider;
    xcb_timestamp_t      config_timestamp;
} xcb_randr_set_provider_offload_sink_request_t;

/** Opcode for xcb_randr_set_provider_output_source. */
#define XCB_RANDR_SET_PROVIDER_OUTPUT_SOURCE 35

/**
 * @brief xcb_randr_set_provider_output_source_request_t
 **/
typedef struct xcb_randr_set_provider_output_source_request_t {
    uint8_t              major_opcode;
    uint8_t              minor_opcode;
    uint16_t             length;
    xcb_randr_provider_t provider;
    xcb_randr_provider_t source_provider;
    xcb_timestamp_t      config_timestamp;
} xcb_randr_set_provider_output_source_request_t;

/**
 * @brief xcb_randr_list_provider_properties_cookie_t
 **/
typedef struct xcb_randr_list_provider_properties_cookie_t {
    unsigned int sequence;
} xcb_randr_list_provider_properties_cookie_t;

/** Opcode for xcb_randr_list_provider_properties. */
#define XCB_RANDR_LIST_PROVIDER_PROPERTIES 36

/**
 * @brief xcb_randr_list_provider_properties_request_t
 **/
typedef struct xcb_randr_list_provider_properties_request_t {
    uint8_t              major_opcode;
    uint8_t              minor_opcode;
    uint16_t             length;
    xcb_randr_provider_t provider;
} xcb_randr_list_provider_properties_request_t;

/**
 * @brief xcb_randr_list_provider_properties_reply_t
 **/
typedef struct xcb_randr_list_provider_properties_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint16_t num_atoms;
    uint8_t  pad1[22];
} xcb_randr_list_provider_properties_reply_t;

/**
 * @brief xcb_randr_query_provider_property_cookie_t
 **/
typedef struct xcb_randr_query_provider_property_cookie_t {
    unsigned int sequence;
} xcb_randr_query_provider_property_cookie_t;

/** Opcode for xcb_randr_query_provider_property. */
#define XCB_RANDR_QUERY_PROVIDER_PROPERTY 37

/**
 * @brief xcb_randr_query_provider_property_request_t
 **/
typedef struct xcb_randr_query_provider_property_request_t {
    uint8_t              major_opcode;
    uint8_t              minor_opcode;
    uint16_t             length;
    xcb_randr_provider_t provider;
    xcb_atom_t           property;
} xcb_randr_query_provider_property_request_t;

/**
 * @brief xcb_randr_query_provider_property_reply_t
 **/
typedef struct xcb_randr_query_provider_property_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint8_t  pending;
    uint8_t  range;
    uint8_t  immutable;
    uint8_t  pad1[21];
} xcb_randr_query_provider_property_reply_t;

/** Opcode for xcb_randr_configure_provider_property. */
#define XCB_RANDR_CONFIGURE_PROVIDER_PROPERTY 38

/**
 * @brief xcb_randr_configure_provider_property_request_t
 **/
typedef struct xcb_randr_configure_provider_property_request_t {
    uint8_t              major_opcode;
    uint8_t              minor_opcode;
    uint16_t             length;
    xcb_randr_provider_t provider;
    xcb_atom_t           property;
    uint8_t              pending;
    uint8_t              range;
    uint8_t              pad0[2];
} xcb_randr_configure_provider_property_request_t;

/** Opcode for xcb_randr_change_provider_property. */
#define XCB_RANDR_CHANGE_PROVIDER_PROPERTY 39

/**
 * @brief xcb_randr_change_provider_property_request_t
 **/
typedef struct xcb_randr_change_provider_property_request_t {
    uint8_t              major_opcode;
    uint8_t              minor_opcode;
    uint16_t             length;
    xcb_randr_provider_t provider;
    xcb_atom_t           property;
    xcb_atom_t           type;
    uint8_t              format;
    uint8_t              mode;
    uint8_t              pad0[2];
    uint32_t             num_items;
} xcb_randr_change_provider_property_request_t;

/** Opcode for xcb_randr_delete_provider_property. */
#define XCB_RANDR_DELETE_PROVIDER_PROPERTY 40

/**
 * @brief xcb_randr_delete_provider_property_request_t
 **/
typedef struct xcb_randr_delete_provider_property_request_t {
    uint8_t              major_opcode;
    uint8_t              minor_opcode;
    uint16_t             length;
    xcb_randr_provider_t provider;
    xcb_atom_t           property;
} xcb_randr_delete_provider_property_request_t;

/**
 * @brief xcb_randr_get_provider_property_cookie_t
 **/
typedef struct xcb_randr_get_provider_property_cookie_t {
    unsigned int sequence;
} xcb_randr_get_provider_property_cookie_t;

/** Opcode for xcb_randr_get_provider_property. */
#define XCB_RANDR_GET_PROVIDER_PROPERTY 41

/**
 * @brief xcb_randr_get_provider_property_request_t
 **/
typedef struct xcb_randr_get_provider_property_request_t {
    uint8_t              major_opcode;
    uint8_t              minor_opcode;
    uint16_t             length;
    xcb_randr_provider_t provider;
    xcb_atom_t           property;
    xcb_atom_t           type;
    uint32_t             long_offset;
    uint32_t             long_length;
    uint8_t              _delete;
    uint8_t              pending;
    uint8_t              pad0[2];
} xcb_randr_get_provider_property_request_t;

/**
 * @brief xcb_randr_get_provider_property_reply_t
 **/
typedef struct xcb_randr_get_provider_property_reply_t {
    uint8_t    response_type;
    uint8_t    format;
    uint16_t   sequence;
    uint32_t   length;
    xcb_atom_t type;
    uint32_t   bytes_after;
    uint32_t   num_items;
    uint8_t    pad0[12];
} xcb_randr_get_provider_property_reply_t;

/** Opcode for xcb_randr_screen_change_notify. */
#define XCB_RANDR_SCREEN_CHANGE_NOTIFY 0

/**
 * @brief xcb_randr_screen_change_notify_event_t
 **/
typedef struct xcb_randr_screen_change_notify_event_t {
    uint8_t         response_type;
    uint8_t         rotation;
    uint16_t        sequence;
    xcb_timestamp_t timestamp;
    xcb_timestamp_t config_timestamp;
    xcb_window_t    root;
    xcb_window_t    request_window;
    uint16_t        sizeID;
    uint16_t        subpixel_order;
    uint16_t        width;
    uint16_t        height;
    uint16_t        mwidth;
    uint16_t        mheight;
} xcb_randr_screen_change_notify_event_t;

typedef enum xcb_randr_notify_t {
    XCB_RANDR_NOTIFY_CRTC_CHANGE = 0,
    XCB_RANDR_NOTIFY_OUTPUT_CHANGE = 1,
    XCB_RANDR_NOTIFY_OUTPUT_PROPERTY = 2,
    XCB_RANDR_NOTIFY_PROVIDER_CHANGE = 3,
    XCB_RANDR_NOTIFY_PROVIDER_PROPERTY = 4,
    XCB_RANDR_NOTIFY_RESOURCE_CHANGE = 5,
    XCB_RANDR_NOTIFY_LEASE = 6
} xcb_randr_notify_t;

/**
 * @brief xcb_randr_crtc_change_t
 **/
typedef struct xcb_randr_crtc_change_t {
    xcb_timestamp_t  timestamp;
    xcb_window_t     window;
    xcb_randr_crtc_t crtc;
    xcb_randr_mode_t mode;
    uint16_t         rotation;
    uint8_t          pad0[2];
    int16_t          x;
    int16_t          y;
    uint16_t         width;
    uint16_t         height;
} xcb_randr_crtc_change_t;

/**
 * @brief xcb_randr_crtc_change_iterator_t
 **/
typedef struct xcb_randr_crtc_change_iterator_t {
    xcb_randr_crtc_change_t *data;
    int                      rem;
    int                      index;
} xcb_randr_crtc_change_iterator_t;

/**
 * @brief xcb_randr_output_change_t
 **/
typedef struct xcb_randr_output_change_t {
    xcb_timestamp_t    timestamp;
    xcb_timestamp_t    config_timestamp;
    xcb_window_t       window;
    xcb_randr_output_t output;
    xcb_randr_crtc_t   crtc;
    xcb_randr_mode_t   mode;
    uint16_t           rotation;
    uint8_t            connection;
    uint8_t            subpixel_order;
} xcb_randr_output_change_t;

/**
 * @brief xcb_randr_output_change_iterator_t
 **/
typedef struct xcb_randr_output_change_iterator_t {
    xcb_randr_output_change_t *data;
    int                        rem;
    int                        index;
} xcb_randr_output_change_iterator_t;

/**
 * @brief xcb_randr_output_property_t
 **/
typedef struct xcb_randr_output_property_t {
    xcb_window_t       window;
    xcb_randr_output_t output;
    xcb_atom_t         atom;
    xcb_timestamp_t    timestamp;
    uint8_t            status;
    uint8_t            pad0[11];
} xcb_randr_output_property_t;

/**
 * @brief xcb_randr_output_property_iterator_t
 **/
typedef struct xcb_randr_output_property_iterator_t {
    xcb_randr_output_property_t *data;
    int                          rem;
    int                          index;
} xcb_randr_output_property_iterator_t;

/**
 * @brief xcb_randr_provider_change_t
 **/
typedef struct xcb_randr_provider_change_t {
    xcb_timestamp_t      timestamp;
    xcb_window_t         window;
    xcb_randr_provider_t provider;
    uint8_t              pad0[16];
} xcb_randr_provider_change_t;

/**
 * @brief xcb_randr_provider_change_iterator_t
 **/
typedef struct xcb_randr_provider_change_iterator_t {
    xcb_randr_provider_change_t *data;
    int                          rem;
    int                          index;
} xcb_randr_provider_change_iterator_t;

/**
 * @brief xcb_randr_provider_property_t
 **/
typedef struct xcb_randr_provider_property_t {
    xcb_window_t         window;
    xcb_randr_provider_t provider;
    xcb_atom_t           atom;
    xcb_timestamp_t      timestamp;
    uint8_t              state;
    uint8_t              pad0[11];
} xcb_randr_provider_property_t;

/**
 * @brief xcb_randr_provider_property_iterator_t
 **/
typedef struct xcb_randr_provider_property_iterator_t {
    xcb_randr_provider_property_t *data;
    int                            rem;
    int                            index;
} xcb_randr_provider_property_iterator_t;

/**
 * @brief xcb_randr_resource_change_t
 **/
typedef struct xcb_randr_resource_change_t {
    xcb_timestamp_t timestamp;
    xcb_window_t    window;
    uint8_t         pad0[20];
} xcb_randr_resource_change_t;

/**
 * @brief xcb_randr_resource_change_iterator_t
 **/
typedef struct xcb_randr_resource_change_iterator_t {
    xcb_randr_resource_change_t *data;
    int                          rem;
    int                          index;
} xcb_randr_resource_change_iterator_t;

/**
 * @brief xcb_randr_monitor_info_t
 **/
typedef struct xcb_randr_monitor_info_t {
    xcb_atom_t name;
    uint8_t    primary;
    uint8_t    automatic;
    uint16_t   nOutput;
    int16_t    x;
    int16_t    y;
    uint16_t   width;
    uint16_t   height;
    uint32_t   width_in_millimeters;
    uint32_t   height_in_millimeters;
} xcb_randr_monitor_info_t;

/**
 * @brief xcb_randr_monitor_info_iterator_t
 **/
typedef struct xcb_randr_monitor_info_iterator_t {
    xcb_randr_monitor_info_t *data;
    int                       rem;
    int                       index;
} xcb_randr_monitor_info_iterator_t;

/**
 * @brief xcb_randr_get_monitors_cookie_t
 **/
typedef struct xcb_randr_get_monitors_cookie_t {
    unsigned int sequence;
} xcb_randr_get_monitors_cookie_t;

/** Opcode for xcb_randr_get_monitors. */
#define XCB_RANDR_GET_MONITORS 42

/**
 * @brief xcb_randr_get_monitors_request_t
 **/
typedef struct xcb_randr_get_monitors_request_t {
    uint8_t      major_opcode;
    uint8_t      minor_opcode;
    uint16_t     length;
    xcb_window_t window;
    uint8_t      get_active;
} xcb_randr_get_monitors_request_t;

/**
 * @brief xcb_randr_get_monitors_reply_t
 **/
typedef struct xcb_randr_get_monitors_reply_t {
    uint8_t         response_type;
    uint8_t         pad0;
    uint16_t        sequence;
    uint32_t        length;
    xcb_timestamp_t timestamp;
    uint32_t        nMonitors;
    uint32_t        nOutputs;
    uint8_t         pad1[12];
} xcb_randr_get_monitors_reply_t;

/** Opcode for xcb_randr_set_monitor. */
#define XCB_RANDR_SET_MONITOR 43

/**
 * @brief xcb_randr_set_monitor_request_t
 **/
typedef struct xcb_randr_set_monitor_request_t {
    uint8_t      major_opcode;
    uint8_t      minor_opcode;
    uint16_t     length;
    xcb_window_t window;
} xcb_randr_set_monitor_request_t;

/** Opcode for xcb_randr_delete_monitor. */
#define XCB_RANDR_DELETE_MONITOR 44

/**
 * @brief xcb_randr_delete_monitor_request_t
 **/
typedef struct xcb_randr_delete_monitor_request_t {
    uint8_t      major_opcode;
    uint8_t      minor_opcode;
    uint16_t     length;
    xcb_window_t window;
    xcb_atom_t   name;
} xcb_randr_delete_monitor_request_t;

/**
 * @brief xcb_randr_create_lease_cookie_t
 **/
typedef struct xcb_randr_create_lease_cookie_t {
    unsigned int sequence;
} xcb_randr_create_lease_cookie_t;

/** Opcode for xcb_randr_create_lease. */
#define XCB_RANDR_CREATE_LEASE 45

/**
 * @brief xcb_randr_create_lease_request_t
 **/
typedef struct xcb_randr_create_lease_request_t {
    uint8_t           major_opcode;
    uint8_t           minor_opcode;
    uint16_t          length;
    xcb_window_t      window;
    xcb_randr_lease_t lid;
    uint16_t          num_crtcs;
    uint16_t          num_outputs;
} xcb_randr_create_lease_request_t;

/**
 * @brief xcb_randr_create_lease_reply_t
 **/
typedef struct xcb_randr_create_lease_reply_t {
    uint8_t  response_type;
    uint8_t  nfd;
    uint16_t sequence;
    uint32_t length;
    uint8_t  pad0[24];
} xcb_randr_create_lease_reply_t;

/** Opcode for xcb_randr_free_lease. */
#define XCB_RANDR_FREE_LEASE 46

/**
 * @brief xcb_randr_free_lease_request_t
 **/
typedef struct xcb_randr_free_lease_request_t {
    uint8_t           major_opcode;
    uint8_t           minor_opcode;
    uint16_t          length;
    xcb_randr_lease_t lid;
    uint8_t           terminate;
} xcb_randr_free_lease_request_t;

/**
 * @brief xcb_randr_lease_notify_t
 **/
typedef struct xcb_randr_lease_notify_t {
    xcb_timestamp_t   timestamp;
    xcb_window_t      window;
    xcb_randr_lease_t lease;
    uint8_t           created;
    uint8_t           pad0[15];
} xcb_randr_lease_notify_t;

/**
 * @brief xcb_randr_lease_notify_iterator_t
 **/
typedef struct xcb_randr_lease_notify_iterator_t {
    xcb_randr_lease_notify_t *data;
    int                       rem;
    int                       index;
} xcb_randr_lease_notify_iterator_t;

/**
 * @brief xcb_randr_notify_data_t
 **/
typedef union xcb_randr_notify_data_t {
    xcb_randr_crtc_change_t       cc;
    xcb_randr_output_change_t     oc;
    xcb_randr_output_property_t   op;
    xcb_randr_provider_change_t   pc;
    xcb_randr_provider_property_t pp;
    xcb_randr_resource_change_t   rc;
    xcb_randr_lease_notify_t      lc;
} xcb_randr_notify_data_t;

/**
 * @brief xcb_randr_notify_data_iterator_t
 **/
typedef struct xcb_randr_notify_data_iterator_t {
    xcb_randr_notify_data_t *data;
    int                      rem;
    int                      index;
} xcb_randr_notify_data_iterator_t;

/** Opcode for xcb_randr_notify. */
#define XCB_RANDR_NOTIFY 1

/**
 * @brief xcb_randr_notify_event_t
 **/
typedef struct xcb_randr_notify_event_t {
    uint8_t                 response_type;
    uint8_t                 subCode;
    uint16_t                sequence;
    xcb_randr_notify_data_t u;
} xcb_randr_notify_event_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_randr_mode_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_randr_mode_t)
 */
void
xcb_randr_mode_next (xcb_randr_mode_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_randr_mode_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_randr_mode_end (xcb_randr_mode_iterator_t i);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_randr_crtc_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_randr_crtc_t)
 */
void
xcb_randr_crtc_next (xcb_randr_crtc_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_randr_crtc_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_randr_crtc_end (xcb_randr_crtc_iterator_t i);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_randr_output_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_randr_output_t)
 */
void
xcb_randr_output_next (xcb_randr_output_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_randr_output_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_randr_output_end (xcb_randr_output_iterator_t i);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_randr_provider_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_randr_provider_t)
 */
void
xcb_randr_provider_next (xcb_randr_provider_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_randr_provider_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_randr_provider_end (xcb_randr_provider_iterator_t i);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_randr_lease_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_randr_lease_t)
 */
void
xcb_randr_lease_next (xcb_randr_lease_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_randr_lease_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_randr_lease_end (xcb_randr_lease_iterator_t i);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_randr_screen_size_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_randr_screen_size_t)
 */
void
xcb_randr_screen_size_next (xcb_randr_screen_size_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_randr_screen_size_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_randr_screen_size_end (xcb_randr_screen_size_iterator_t i);

int
xcb_randr_refresh_rates_sizeof (const void  *_buffer);

uint16_t *
xcb_randr_refresh_rates_rates (const xcb_randr_refresh_rates_t *R);

int
xcb_randr_refresh_rates_rates_length (const xcb_randr_refresh_rates_t *R);

xcb_generic_iterator_t
xcb_randr_refresh_rates_rates_end (const xcb_randr_refresh_rates_t *R);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_randr_refresh_rates_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_randr_refresh_rates_t)
 */
void
xcb_randr_refresh_rates_next (xcb_randr_refresh_rates_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_randr_refresh_rates_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_randr_refresh_rates_end (xcb_randr_refresh_rates_iterator_t i);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_randr_query_version_cookie_t
xcb_randr_query_version (xcb_connection_t *c,
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
xcb_randr_query_version_cookie_t
xcb_randr_query_version_unchecked (xcb_connection_t *c,
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
 * xcb_randr_query_version_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_randr_query_version_reply_t *
xcb_randr_query_version_reply (xcb_connection_t                  *c,
                               xcb_randr_query_version_cookie_t   cookie  /**< */,
                               xcb_generic_error_t              **e);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_randr_set_screen_config_cookie_t
xcb_randr_set_screen_config (xcb_connection_t *c,
                             xcb_window_t      window,
                             xcb_timestamp_t   timestamp,
                             xcb_timestamp_t   config_timestamp,
                             uint16_t          sizeID,
                             uint16_t          rotation,
                             uint16_t          rate);

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
xcb_randr_set_screen_config_cookie_t
xcb_randr_set_screen_config_unchecked (xcb_connection_t *c,
                                       xcb_window_t      window,
                                       xcb_timestamp_t   timestamp,
                                       xcb_timestamp_t   config_timestamp,
                                       uint16_t          sizeID,
                                       uint16_t          rotation,
                                       uint16_t          rate);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_randr_set_screen_config_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_randr_set_screen_config_reply_t *
xcb_randr_set_screen_config_reply (xcb_connection_t                      *c,
                                   xcb_randr_set_screen_config_cookie_t   cookie  /**< */,
                                   xcb_generic_error_t                  **e);

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
xcb_randr_select_input_checked (xcb_connection_t *c,
                                xcb_window_t      window,
                                uint16_t          enable);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_randr_select_input (xcb_connection_t *c,
                        xcb_window_t      window,
                        uint16_t          enable);

int
xcb_randr_get_screen_info_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_randr_get_screen_info_cookie_t
xcb_randr_get_screen_info (xcb_connection_t *c,
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
xcb_randr_get_screen_info_cookie_t
xcb_randr_get_screen_info_unchecked (xcb_connection_t *c,
                                     xcb_window_t      window);

xcb_randr_screen_size_t *
xcb_randr_get_screen_info_sizes (const xcb_randr_get_screen_info_reply_t *R);

int
xcb_randr_get_screen_info_sizes_length (const xcb_randr_get_screen_info_reply_t *R);

xcb_randr_screen_size_iterator_t
xcb_randr_get_screen_info_sizes_iterator (const xcb_randr_get_screen_info_reply_t *R);

int
xcb_randr_get_screen_info_rates_length (const xcb_randr_get_screen_info_reply_t *R);

xcb_randr_refresh_rates_iterator_t
xcb_randr_get_screen_info_rates_iterator (const xcb_randr_get_screen_info_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_randr_get_screen_info_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_randr_get_screen_info_reply_t *
xcb_randr_get_screen_info_reply (xcb_connection_t                    *c,
                                 xcb_randr_get_screen_info_cookie_t   cookie  /**< */,
                                 xcb_generic_error_t                **e);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_randr_get_screen_size_range_cookie_t
xcb_randr_get_screen_size_range (xcb_connection_t *c,
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
xcb_randr_get_screen_size_range_cookie_t
xcb_randr_get_screen_size_range_unchecked (xcb_connection_t *c,
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
 * xcb_randr_get_screen_size_range_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_randr_get_screen_size_range_reply_t *
xcb_randr_get_screen_size_range_reply (xcb_connection_t                          *c,
                                       xcb_randr_get_screen_size_range_cookie_t   cookie  /**< */,
                                       xcb_generic_error_t                      **e);

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
xcb_randr_set_screen_size_checked (xcb_connection_t *c,
                                   xcb_window_t      window,
                                   uint16_t          width,
                                   uint16_t          height,
                                   uint32_t          mm_width,
                                   uint32_t          mm_height);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_randr_set_screen_size (xcb_connection_t *c,
                           xcb_window_t      window,
                           uint16_t          width,
                           uint16_t          height,
                           uint32_t          mm_width,
                           uint32_t          mm_height);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_randr_mode_info_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_randr_mode_info_t)
 */
void
xcb_randr_mode_info_next (xcb_randr_mode_info_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_randr_mode_info_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_randr_mode_info_end (xcb_randr_mode_info_iterator_t i);

int
xcb_randr_get_screen_resources_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_randr_get_screen_resources_cookie_t
xcb_randr_get_screen_resources (xcb_connection_t *c,
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
xcb_randr_get_screen_resources_cookie_t
xcb_randr_get_screen_resources_unchecked (xcb_connection_t *c,
                                          xcb_window_t      window);

xcb_randr_crtc_t *
xcb_randr_get_screen_resources_crtcs (const xcb_randr_get_screen_resources_reply_t *R);

int
xcb_randr_get_screen_resources_crtcs_length (const xcb_randr_get_screen_resources_reply_t *R);

xcb_generic_iterator_t
xcb_randr_get_screen_resources_crtcs_end (const xcb_randr_get_screen_resources_reply_t *R);

xcb_randr_output_t *
xcb_randr_get_screen_resources_outputs (const xcb_randr_get_screen_resources_reply_t *R);

int
xcb_randr_get_screen_resources_outputs_length (const xcb_randr_get_screen_resources_reply_t *R);

xcb_generic_iterator_t
xcb_randr_get_screen_resources_outputs_end (const xcb_randr_get_screen_resources_reply_t *R);

xcb_randr_mode_info_t *
xcb_randr_get_screen_resources_modes (const xcb_randr_get_screen_resources_reply_t *R);

int
xcb_randr_get_screen_resources_modes_length (const xcb_randr_get_screen_resources_reply_t *R);

xcb_randr_mode_info_iterator_t
xcb_randr_get_screen_resources_modes_iterator (const xcb_randr_get_screen_resources_reply_t *R);

uint8_t *
xcb_randr_get_screen_resources_names (const xcb_randr_get_screen_resources_reply_t *R);

int
xcb_randr_get_screen_resources_names_length (const xcb_randr_get_screen_resources_reply_t *R);

xcb_generic_iterator_t
xcb_randr_get_screen_resources_names_end (const xcb_randr_get_screen_resources_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_randr_get_screen_resources_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_randr_get_screen_resources_reply_t *
xcb_randr_get_screen_resources_reply (xcb_connection_t                         *c,
                                      xcb_randr_get_screen_resources_cookie_t   cookie  /**< */,
                                      xcb_generic_error_t                     **e);

int
xcb_randr_get_output_info_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_randr_get_output_info_cookie_t
xcb_randr_get_output_info (xcb_connection_t   *c,
                           xcb_randr_output_t  output,
                           xcb_timestamp_t     config_timestamp);

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
xcb_randr_get_output_info_cookie_t
xcb_randr_get_output_info_unchecked (xcb_connection_t   *c,
                                     xcb_randr_output_t  output,
                                     xcb_timestamp_t     config_timestamp);

xcb_randr_crtc_t *
xcb_randr_get_output_info_crtcs (const xcb_randr_get_output_info_reply_t *R);

int
xcb_randr_get_output_info_crtcs_length (const xcb_randr_get_output_info_reply_t *R);

xcb_generic_iterator_t
xcb_randr_get_output_info_crtcs_end (const xcb_randr_get_output_info_reply_t *R);

xcb_randr_mode_t *
xcb_randr_get_output_info_modes (const xcb_randr_get_output_info_reply_t *R);

int
xcb_randr_get_output_info_modes_length (const xcb_randr_get_output_info_reply_t *R);

xcb_generic_iterator_t
xcb_randr_get_output_info_modes_end (const xcb_randr_get_output_info_reply_t *R);

xcb_randr_output_t *
xcb_randr_get_output_info_clones (const xcb_randr_get_output_info_reply_t *R);

int
xcb_randr_get_output_info_clones_length (const xcb_randr_get_output_info_reply_t *R);

xcb_generic_iterator_t
xcb_randr_get_output_info_clones_end (const xcb_randr_get_output_info_reply_t *R);

uint8_t *
xcb_randr_get_output_info_name (const xcb_randr_get_output_info_reply_t *R);

int
xcb_randr_get_output_info_name_length (const xcb_randr_get_output_info_reply_t *R);

xcb_generic_iterator_t
xcb_randr_get_output_info_name_end (const xcb_randr_get_output_info_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_randr_get_output_info_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_randr_get_output_info_reply_t *
xcb_randr_get_output_info_reply (xcb_connection_t                    *c,
                                 xcb_randr_get_output_info_cookie_t   cookie  /**< */,
                                 xcb_generic_error_t                **e);

int
xcb_randr_list_output_properties_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_randr_list_output_properties_cookie_t
xcb_randr_list_output_properties (xcb_connection_t   *c,
                                  xcb_randr_output_t  output);

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
xcb_randr_list_output_properties_cookie_t
xcb_randr_list_output_properties_unchecked (xcb_connection_t   *c,
                                            xcb_randr_output_t  output);

xcb_atom_t *
xcb_randr_list_output_properties_atoms (const xcb_randr_list_output_properties_reply_t *R);

int
xcb_randr_list_output_properties_atoms_length (const xcb_randr_list_output_properties_reply_t *R);

xcb_generic_iterator_t
xcb_randr_list_output_properties_atoms_end (const xcb_randr_list_output_properties_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_randr_list_output_properties_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_randr_list_output_properties_reply_t *
xcb_randr_list_output_properties_reply (xcb_connection_t                           *c,
                                        xcb_randr_list_output_properties_cookie_t   cookie  /**< */,
                                        xcb_generic_error_t                       **e);

int
xcb_randr_query_output_property_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_randr_query_output_property_cookie_t
xcb_randr_query_output_property (xcb_connection_t   *c,
                                 xcb_randr_output_t  output,
                                 xcb_atom_t          property);

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
xcb_randr_query_output_property_cookie_t
xcb_randr_query_output_property_unchecked (xcb_connection_t   *c,
                                           xcb_randr_output_t  output,
                                           xcb_atom_t          property);

int32_t *
xcb_randr_query_output_property_valid_values (const xcb_randr_query_output_property_reply_t *R);

int
xcb_randr_query_output_property_valid_values_length (const xcb_randr_query_output_property_reply_t *R);

xcb_generic_iterator_t
xcb_randr_query_output_property_valid_values_end (const xcb_randr_query_output_property_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_randr_query_output_property_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_randr_query_output_property_reply_t *
xcb_randr_query_output_property_reply (xcb_connection_t                          *c,
                                       xcb_randr_query_output_property_cookie_t   cookie  /**< */,
                                       xcb_generic_error_t                      **e);

int
xcb_randr_configure_output_property_sizeof (const void  *_buffer,
                                            uint32_t     values_len);

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
xcb_randr_configure_output_property_checked (xcb_connection_t   *c,
                                             xcb_randr_output_t  output,
                                             xcb_atom_t          property,
                                             uint8_t             pending,
                                             uint8_t             range,
                                             uint32_t            values_len,
                                             const int32_t      *values);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_randr_configure_output_property (xcb_connection_t   *c,
                                     xcb_randr_output_t  output,
                                     xcb_atom_t          property,
                                     uint8_t             pending,
                                     uint8_t             range,
                                     uint32_t            values_len,
                                     const int32_t      *values);

int32_t *
xcb_randr_configure_output_property_values (const xcb_randr_configure_output_property_request_t *R);

int
xcb_randr_configure_output_property_values_length (const xcb_randr_configure_output_property_request_t *R);

xcb_generic_iterator_t
xcb_randr_configure_output_property_values_end (const xcb_randr_configure_output_property_request_t *R);

int
xcb_randr_change_output_property_sizeof (const void  *_buffer);

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
xcb_randr_change_output_property_checked (xcb_connection_t   *c,
                                          xcb_randr_output_t  output,
                                          xcb_atom_t          property,
                                          xcb_atom_t          type,
                                          uint8_t             format,
                                          uint8_t             mode,
                                          uint32_t            num_units,
                                          const void         *data);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_randr_change_output_property (xcb_connection_t   *c,
                                  xcb_randr_output_t  output,
                                  xcb_atom_t          property,
                                  xcb_atom_t          type,
                                  uint8_t             format,
                                  uint8_t             mode,
                                  uint32_t            num_units,
                                  const void         *data);

void *
xcb_randr_change_output_property_data (const xcb_randr_change_output_property_request_t *R);

int
xcb_randr_change_output_property_data_length (const xcb_randr_change_output_property_request_t *R);

xcb_generic_iterator_t
xcb_randr_change_output_property_data_end (const xcb_randr_change_output_property_request_t *R);

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
xcb_randr_delete_output_property_checked (xcb_connection_t   *c,
                                          xcb_randr_output_t  output,
                                          xcb_atom_t          property);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_randr_delete_output_property (xcb_connection_t   *c,
                                  xcb_randr_output_t  output,
                                  xcb_atom_t          property);

int
xcb_randr_get_output_property_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_randr_get_output_property_cookie_t
xcb_randr_get_output_property (xcb_connection_t   *c,
                               xcb_randr_output_t  output,
                               xcb_atom_t          property,
                               xcb_atom_t          type,
                               uint32_t            long_offset,
                               uint32_t            long_length,
                               uint8_t             _delete,
                               uint8_t             pending);

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
xcb_randr_get_output_property_cookie_t
xcb_randr_get_output_property_unchecked (xcb_connection_t   *c,
                                         xcb_randr_output_t  output,
                                         xcb_atom_t          property,
                                         xcb_atom_t          type,
                                         uint32_t            long_offset,
                                         uint32_t            long_length,
                                         uint8_t             _delete,
                                         uint8_t             pending);

uint8_t *
xcb_randr_get_output_property_data (const xcb_randr_get_output_property_reply_t *R);

int
xcb_randr_get_output_property_data_length (const xcb_randr_get_output_property_reply_t *R);

xcb_generic_iterator_t
xcb_randr_get_output_property_data_end (const xcb_randr_get_output_property_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_randr_get_output_property_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_randr_get_output_property_reply_t *
xcb_randr_get_output_property_reply (xcb_connection_t                        *c,
                                     xcb_randr_get_output_property_cookie_t   cookie  /**< */,
                                     xcb_generic_error_t                    **e);

int
xcb_randr_create_mode_sizeof (const void  *_buffer,
                              uint32_t     name_len);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_randr_create_mode_cookie_t
xcb_randr_create_mode (xcb_connection_t      *c,
                       xcb_window_t           window,
                       xcb_randr_mode_info_t  mode_info,
                       uint32_t               name_len,
                       const char            *name);

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
xcb_randr_create_mode_cookie_t
xcb_randr_create_mode_unchecked (xcb_connection_t      *c,
                                 xcb_window_t           window,
                                 xcb_randr_mode_info_t  mode_info,
                                 uint32_t               name_len,
                                 const char            *name);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_randr_create_mode_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_randr_create_mode_reply_t *
xcb_randr_create_mode_reply (xcb_connection_t                *c,
                             xcb_randr_create_mode_cookie_t   cookie  /**< */,
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
xcb_randr_destroy_mode_checked (xcb_connection_t *c,
                                xcb_randr_mode_t  mode);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_randr_destroy_mode (xcb_connection_t *c,
                        xcb_randr_mode_t  mode);

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
xcb_randr_add_output_mode_checked (xcb_connection_t   *c,
                                   xcb_randr_output_t  output,
                                   xcb_randr_mode_t    mode);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_randr_add_output_mode (xcb_connection_t   *c,
                           xcb_randr_output_t  output,
                           xcb_randr_mode_t    mode);

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
xcb_randr_delete_output_mode_checked (xcb_connection_t   *c,
                                      xcb_randr_output_t  output,
                                      xcb_randr_mode_t    mode);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_randr_delete_output_mode (xcb_connection_t   *c,
                              xcb_randr_output_t  output,
                              xcb_randr_mode_t    mode);

int
xcb_randr_get_crtc_info_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_randr_get_crtc_info_cookie_t
xcb_randr_get_crtc_info (xcb_connection_t *c,
                         xcb_randr_crtc_t  crtc,
                         xcb_timestamp_t   config_timestamp);

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
xcb_randr_get_crtc_info_cookie_t
xcb_randr_get_crtc_info_unchecked (xcb_connection_t *c,
                                   xcb_randr_crtc_t  crtc,
                                   xcb_timestamp_t   config_timestamp);

xcb_randr_output_t *
xcb_randr_get_crtc_info_outputs (const xcb_randr_get_crtc_info_reply_t *R);

int
xcb_randr_get_crtc_info_outputs_length (const xcb_randr_get_crtc_info_reply_t *R);

xcb_generic_iterator_t
xcb_randr_get_crtc_info_outputs_end (const xcb_randr_get_crtc_info_reply_t *R);

xcb_randr_output_t *
xcb_randr_get_crtc_info_possible (const xcb_randr_get_crtc_info_reply_t *R);

int
xcb_randr_get_crtc_info_possible_length (const xcb_randr_get_crtc_info_reply_t *R);

xcb_generic_iterator_t
xcb_randr_get_crtc_info_possible_end (const xcb_randr_get_crtc_info_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_randr_get_crtc_info_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_randr_get_crtc_info_reply_t *
xcb_randr_get_crtc_info_reply (xcb_connection_t                  *c,
                               xcb_randr_get_crtc_info_cookie_t   cookie  /**< */,
                               xcb_generic_error_t              **e);

int
xcb_randr_set_crtc_config_sizeof (const void  *_buffer,
                                  uint32_t     outputs_len);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_randr_set_crtc_config_cookie_t
xcb_randr_set_crtc_config (xcb_connection_t         *c,
                           xcb_randr_crtc_t          crtc,
                           xcb_timestamp_t           timestamp,
                           xcb_timestamp_t           config_timestamp,
                           int16_t                   x,
                           int16_t                   y,
                           xcb_randr_mode_t          mode,
                           uint16_t                  rotation,
                           uint32_t                  outputs_len,
                           const xcb_randr_output_t *outputs);

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
xcb_randr_set_crtc_config_cookie_t
xcb_randr_set_crtc_config_unchecked (xcb_connection_t         *c,
                                     xcb_randr_crtc_t          crtc,
                                     xcb_timestamp_t           timestamp,
                                     xcb_timestamp_t           config_timestamp,
                                     int16_t                   x,
                                     int16_t                   y,
                                     xcb_randr_mode_t          mode,
                                     uint16_t                  rotation,
                                     uint32_t                  outputs_len,
                                     const xcb_randr_output_t *outputs);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_randr_set_crtc_config_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_randr_set_crtc_config_reply_t *
xcb_randr_set_crtc_config_reply (xcb_connection_t                    *c,
                                 xcb_randr_set_crtc_config_cookie_t   cookie  /**< */,
                                 xcb_generic_error_t                **e);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_randr_get_crtc_gamma_size_cookie_t
xcb_randr_get_crtc_gamma_size (xcb_connection_t *c,
                               xcb_randr_crtc_t  crtc);

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
xcb_randr_get_crtc_gamma_size_cookie_t
xcb_randr_get_crtc_gamma_size_unchecked (xcb_connection_t *c,
                                         xcb_randr_crtc_t  crtc);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_randr_get_crtc_gamma_size_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_randr_get_crtc_gamma_size_reply_t *
xcb_randr_get_crtc_gamma_size_reply (xcb_connection_t                        *c,
                                     xcb_randr_get_crtc_gamma_size_cookie_t   cookie  /**< */,
                                     xcb_generic_error_t                    **e);

int
xcb_randr_get_crtc_gamma_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_randr_get_crtc_gamma_cookie_t
xcb_randr_get_crtc_gamma (xcb_connection_t *c,
                          xcb_randr_crtc_t  crtc);

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
xcb_randr_get_crtc_gamma_cookie_t
xcb_randr_get_crtc_gamma_unchecked (xcb_connection_t *c,
                                    xcb_randr_crtc_t  crtc);

uint16_t *
xcb_randr_get_crtc_gamma_red (const xcb_randr_get_crtc_gamma_reply_t *R);

int
xcb_randr_get_crtc_gamma_red_length (const xcb_randr_get_crtc_gamma_reply_t *R);

xcb_generic_iterator_t
xcb_randr_get_crtc_gamma_red_end (const xcb_randr_get_crtc_gamma_reply_t *R);

uint16_t *
xcb_randr_get_crtc_gamma_green (const xcb_randr_get_crtc_gamma_reply_t *R);

int
xcb_randr_get_crtc_gamma_green_length (const xcb_randr_get_crtc_gamma_reply_t *R);

xcb_generic_iterator_t
xcb_randr_get_crtc_gamma_green_end (const xcb_randr_get_crtc_gamma_reply_t *R);

uint16_t *
xcb_randr_get_crtc_gamma_blue (const xcb_randr_get_crtc_gamma_reply_t *R);

int
xcb_randr_get_crtc_gamma_blue_length (const xcb_randr_get_crtc_gamma_reply_t *R);

xcb_generic_iterator_t
xcb_randr_get_crtc_gamma_blue_end (const xcb_randr_get_crtc_gamma_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_randr_get_crtc_gamma_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_randr_get_crtc_gamma_reply_t *
xcb_randr_get_crtc_gamma_reply (xcb_connection_t                   *c,
                                xcb_randr_get_crtc_gamma_cookie_t   cookie  /**< */,
                                xcb_generic_error_t               **e);

int
xcb_randr_set_crtc_gamma_sizeof (const void  *_buffer);

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
xcb_randr_set_crtc_gamma_checked (xcb_connection_t *c,
                                  xcb_randr_crtc_t  crtc,
                                  uint16_t          size,
                                  const uint16_t   *red,
                                  const uint16_t   *green,
                                  const uint16_t   *blue);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_randr_set_crtc_gamma (xcb_connection_t *c,
                          xcb_randr_crtc_t  crtc,
                          uint16_t          size,
                          const uint16_t   *red,
                          const uint16_t   *green,
                          const uint16_t   *blue);

uint16_t *
xcb_randr_set_crtc_gamma_red (const xcb_randr_set_crtc_gamma_request_t *R);

int
xcb_randr_set_crtc_gamma_red_length (const xcb_randr_set_crtc_gamma_request_t *R);

xcb_generic_iterator_t
xcb_randr_set_crtc_gamma_red_end (const xcb_randr_set_crtc_gamma_request_t *R);

uint16_t *
xcb_randr_set_crtc_gamma_green (const xcb_randr_set_crtc_gamma_request_t *R);

int
xcb_randr_set_crtc_gamma_green_length (const xcb_randr_set_crtc_gamma_request_t *R);

xcb_generic_iterator_t
xcb_randr_set_crtc_gamma_green_end (const xcb_randr_set_crtc_gamma_request_t *R);

uint16_t *
xcb_randr_set_crtc_gamma_blue (const xcb_randr_set_crtc_gamma_request_t *R);

int
xcb_randr_set_crtc_gamma_blue_length (const xcb_randr_set_crtc_gamma_request_t *R);

xcb_generic_iterator_t
xcb_randr_set_crtc_gamma_blue_end (const xcb_randr_set_crtc_gamma_request_t *R);

int
xcb_randr_get_screen_resources_current_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_randr_get_screen_resources_current_cookie_t
xcb_randr_get_screen_resources_current (xcb_connection_t *c,
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
xcb_randr_get_screen_resources_current_cookie_t
xcb_randr_get_screen_resources_current_unchecked (xcb_connection_t *c,
                                                  xcb_window_t      window);

xcb_randr_crtc_t *
xcb_randr_get_screen_resources_current_crtcs (const xcb_randr_get_screen_resources_current_reply_t *R);

int
xcb_randr_get_screen_resources_current_crtcs_length (const xcb_randr_get_screen_resources_current_reply_t *R);

xcb_generic_iterator_t
xcb_randr_get_screen_resources_current_crtcs_end (const xcb_randr_get_screen_resources_current_reply_t *R);

xcb_randr_output_t *
xcb_randr_get_screen_resources_current_outputs (const xcb_randr_get_screen_resources_current_reply_t *R);

int
xcb_randr_get_screen_resources_current_outputs_length (const xcb_randr_get_screen_resources_current_reply_t *R);

xcb_generic_iterator_t
xcb_randr_get_screen_resources_current_outputs_end (const xcb_randr_get_screen_resources_current_reply_t *R);

xcb_randr_mode_info_t *
xcb_randr_get_screen_resources_current_modes (const xcb_randr_get_screen_resources_current_reply_t *R);

int
xcb_randr_get_screen_resources_current_modes_length (const xcb_randr_get_screen_resources_current_reply_t *R);

xcb_randr_mode_info_iterator_t
xcb_randr_get_screen_resources_current_modes_iterator (const xcb_randr_get_screen_resources_current_reply_t *R);

uint8_t *
xcb_randr_get_screen_resources_current_names (const xcb_randr_get_screen_resources_current_reply_t *R);

int
xcb_randr_get_screen_resources_current_names_length (const xcb_randr_get_screen_resources_current_reply_t *R);

xcb_generic_iterator_t
xcb_randr_get_screen_resources_current_names_end (const xcb_randr_get_screen_resources_current_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_randr_get_screen_resources_current_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_randr_get_screen_resources_current_reply_t *
xcb_randr_get_screen_resources_current_reply (xcb_connection_t                                 *c,
                                              xcb_randr_get_screen_resources_current_cookie_t   cookie  /**< */,
                                              xcb_generic_error_t                             **e);

int
xcb_randr_set_crtc_transform_sizeof (const void  *_buffer,
                                     uint32_t     filter_params_len);

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
xcb_randr_set_crtc_transform_checked (xcb_connection_t         *c,
                                      xcb_randr_crtc_t          crtc,
                                      xcb_render_transform_t    transform,
                                      uint16_t                  filter_len,
                                      const char               *filter_name,
                                      uint32_t                  filter_params_len,
                                      const xcb_render_fixed_t *filter_params);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_randr_set_crtc_transform (xcb_connection_t         *c,
                              xcb_randr_crtc_t          crtc,
                              xcb_render_transform_t    transform,
                              uint16_t                  filter_len,
                              const char               *filter_name,
                              uint32_t                  filter_params_len,
                              const xcb_render_fixed_t *filter_params);

char *
xcb_randr_set_crtc_transform_filter_name (const xcb_randr_set_crtc_transform_request_t *R);

int
xcb_randr_set_crtc_transform_filter_name_length (const xcb_randr_set_crtc_transform_request_t *R);

xcb_generic_iterator_t
xcb_randr_set_crtc_transform_filter_name_end (const xcb_randr_set_crtc_transform_request_t *R);

xcb_render_fixed_t *
xcb_randr_set_crtc_transform_filter_params (const xcb_randr_set_crtc_transform_request_t *R);

int
xcb_randr_set_crtc_transform_filter_params_length (const xcb_randr_set_crtc_transform_request_t *R);

xcb_generic_iterator_t
xcb_randr_set_crtc_transform_filter_params_end (const xcb_randr_set_crtc_transform_request_t *R);

int
xcb_randr_get_crtc_transform_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_randr_get_crtc_transform_cookie_t
xcb_randr_get_crtc_transform (xcb_connection_t *c,
                              xcb_randr_crtc_t  crtc);

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
xcb_randr_get_crtc_transform_cookie_t
xcb_randr_get_crtc_transform_unchecked (xcb_connection_t *c,
                                        xcb_randr_crtc_t  crtc);

char *
xcb_randr_get_crtc_transform_pending_filter_name (const xcb_randr_get_crtc_transform_reply_t *R);

int
xcb_randr_get_crtc_transform_pending_filter_name_length (const xcb_randr_get_crtc_transform_reply_t *R);

xcb_generic_iterator_t
xcb_randr_get_crtc_transform_pending_filter_name_end (const xcb_randr_get_crtc_transform_reply_t *R);

xcb_render_fixed_t *
xcb_randr_get_crtc_transform_pending_params (const xcb_randr_get_crtc_transform_reply_t *R);

int
xcb_randr_get_crtc_transform_pending_params_length (const xcb_randr_get_crtc_transform_reply_t *R);

xcb_generic_iterator_t
xcb_randr_get_crtc_transform_pending_params_end (const xcb_randr_get_crtc_transform_reply_t *R);

char *
xcb_randr_get_crtc_transform_current_filter_name (const xcb_randr_get_crtc_transform_reply_t *R);

int
xcb_randr_get_crtc_transform_current_filter_name_length (const xcb_randr_get_crtc_transform_reply_t *R);

xcb_generic_iterator_t
xcb_randr_get_crtc_transform_current_filter_name_end (const xcb_randr_get_crtc_transform_reply_t *R);

xcb_render_fixed_t *
xcb_randr_get_crtc_transform_current_params (const xcb_randr_get_crtc_transform_reply_t *R);

int
xcb_randr_get_crtc_transform_current_params_length (const xcb_randr_get_crtc_transform_reply_t *R);

xcb_generic_iterator_t
xcb_randr_get_crtc_transform_current_params_end (const xcb_randr_get_crtc_transform_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_randr_get_crtc_transform_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_randr_get_crtc_transform_reply_t *
xcb_randr_get_crtc_transform_reply (xcb_connection_t                       *c,
                                    xcb_randr_get_crtc_transform_cookie_t   cookie  /**< */,
                                    xcb_generic_error_t                   **e);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_randr_get_panning_cookie_t
xcb_randr_get_panning (xcb_connection_t *c,
                       xcb_randr_crtc_t  crtc);

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
xcb_randr_get_panning_cookie_t
xcb_randr_get_panning_unchecked (xcb_connection_t *c,
                                 xcb_randr_crtc_t  crtc);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_randr_get_panning_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_randr_get_panning_reply_t *
xcb_randr_get_panning_reply (xcb_connection_t                *c,
                             xcb_randr_get_panning_cookie_t   cookie  /**< */,
                             xcb_generic_error_t            **e);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_randr_set_panning_cookie_t
xcb_randr_set_panning (xcb_connection_t *c,
                       xcb_randr_crtc_t  crtc,
                       xcb_timestamp_t   timestamp,
                       uint16_t          left,
                       uint16_t          top,
                       uint16_t          width,
                       uint16_t          height,
                       uint16_t          track_left,
                       uint16_t          track_top,
                       uint16_t          track_width,
                       uint16_t          track_height,
                       int16_t           border_left,
                       int16_t           border_top,
                       int16_t           border_right,
                       int16_t           border_bottom);

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
xcb_randr_set_panning_cookie_t
xcb_randr_set_panning_unchecked (xcb_connection_t *c,
                                 xcb_randr_crtc_t  crtc,
                                 xcb_timestamp_t   timestamp,
                                 uint16_t          left,
                                 uint16_t          top,
                                 uint16_t          width,
                                 uint16_t          height,
                                 uint16_t          track_left,
                                 uint16_t          track_top,
                                 uint16_t          track_width,
                                 uint16_t          track_height,
                                 int16_t           border_left,
                                 int16_t           border_top,
                                 int16_t           border_right,
                                 int16_t           border_bottom);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_randr_set_panning_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_randr_set_panning_reply_t *
xcb_randr_set_panning_reply (xcb_connection_t                *c,
                             xcb_randr_set_panning_cookie_t   cookie  /**< */,
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
xcb_randr_set_output_primary_checked (xcb_connection_t   *c,
                                      xcb_window_t        window,
                                      xcb_randr_output_t  output);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_randr_set_output_primary (xcb_connection_t   *c,
                              xcb_window_t        window,
                              xcb_randr_output_t  output);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_randr_get_output_primary_cookie_t
xcb_randr_get_output_primary (xcb_connection_t *c,
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
xcb_randr_get_output_primary_cookie_t
xcb_randr_get_output_primary_unchecked (xcb_connection_t *c,
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
 * xcb_randr_get_output_primary_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_randr_get_output_primary_reply_t *
xcb_randr_get_output_primary_reply (xcb_connection_t                       *c,
                                    xcb_randr_get_output_primary_cookie_t   cookie  /**< */,
                                    xcb_generic_error_t                   **e);

int
xcb_randr_get_providers_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_randr_get_providers_cookie_t
xcb_randr_get_providers (xcb_connection_t *c,
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
xcb_randr_get_providers_cookie_t
xcb_randr_get_providers_unchecked (xcb_connection_t *c,
                                   xcb_window_t      window);

xcb_randr_provider_t *
xcb_randr_get_providers_providers (const xcb_randr_get_providers_reply_t *R);

int
xcb_randr_get_providers_providers_length (const xcb_randr_get_providers_reply_t *R);

xcb_generic_iterator_t
xcb_randr_get_providers_providers_end (const xcb_randr_get_providers_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_randr_get_providers_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_randr_get_providers_reply_t *
xcb_randr_get_providers_reply (xcb_connection_t                  *c,
                               xcb_randr_get_providers_cookie_t   cookie  /**< */,
                               xcb_generic_error_t              **e);

int
xcb_randr_get_provider_info_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_randr_get_provider_info_cookie_t
xcb_randr_get_provider_info (xcb_connection_t     *c,
                             xcb_randr_provider_t  provider,
                             xcb_timestamp_t       config_timestamp);

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
xcb_randr_get_provider_info_cookie_t
xcb_randr_get_provider_info_unchecked (xcb_connection_t     *c,
                                       xcb_randr_provider_t  provider,
                                       xcb_timestamp_t       config_timestamp);

xcb_randr_crtc_t *
xcb_randr_get_provider_info_crtcs (const xcb_randr_get_provider_info_reply_t *R);

int
xcb_randr_get_provider_info_crtcs_length (const xcb_randr_get_provider_info_reply_t *R);

xcb_generic_iterator_t
xcb_randr_get_provider_info_crtcs_end (const xcb_randr_get_provider_info_reply_t *R);

xcb_randr_output_t *
xcb_randr_get_provider_info_outputs (const xcb_randr_get_provider_info_reply_t *R);

int
xcb_randr_get_provider_info_outputs_length (const xcb_randr_get_provider_info_reply_t *R);

xcb_generic_iterator_t
xcb_randr_get_provider_info_outputs_end (const xcb_randr_get_provider_info_reply_t *R);

xcb_randr_provider_t *
xcb_randr_get_provider_info_associated_providers (const xcb_randr_get_provider_info_reply_t *R);

int
xcb_randr_get_provider_info_associated_providers_length (const xcb_randr_get_provider_info_reply_t *R);

xcb_generic_iterator_t
xcb_randr_get_provider_info_associated_providers_end (const xcb_randr_get_provider_info_reply_t *R);

uint32_t *
xcb_randr_get_provider_info_associated_capability (const xcb_randr_get_provider_info_reply_t *R);

int
xcb_randr_get_provider_info_associated_capability_length (const xcb_randr_get_provider_info_reply_t *R);

xcb_generic_iterator_t
xcb_randr_get_provider_info_associated_capability_end (const xcb_randr_get_provider_info_reply_t *R);

char *
xcb_randr_get_provider_info_name (const xcb_randr_get_provider_info_reply_t *R);

int
xcb_randr_get_provider_info_name_length (const xcb_randr_get_provider_info_reply_t *R);

xcb_generic_iterator_t
xcb_randr_get_provider_info_name_end (const xcb_randr_get_provider_info_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_randr_get_provider_info_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_randr_get_provider_info_reply_t *
xcb_randr_get_provider_info_reply (xcb_connection_t                      *c,
                                   xcb_randr_get_provider_info_cookie_t   cookie  /**< */,
                                   xcb_generic_error_t                  **e);

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
xcb_randr_set_provider_offload_sink_checked (xcb_connection_t     *c,
                                             xcb_randr_provider_t  provider,
                                             xcb_randr_provider_t  sink_provider,
                                             xcb_timestamp_t       config_timestamp);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_randr_set_provider_offload_sink (xcb_connection_t     *c,
                                     xcb_randr_provider_t  provider,
                                     xcb_randr_provider_t  sink_provider,
                                     xcb_timestamp_t       config_timestamp);

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
xcb_randr_set_provider_output_source_checked (xcb_connection_t     *c,
                                              xcb_randr_provider_t  provider,
                                              xcb_randr_provider_t  source_provider,
                                              xcb_timestamp_t       config_timestamp);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_randr_set_provider_output_source (xcb_connection_t     *c,
                                      xcb_randr_provider_t  provider,
                                      xcb_randr_provider_t  source_provider,
                                      xcb_timestamp_t       config_timestamp);

int
xcb_randr_list_provider_properties_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_randr_list_provider_properties_cookie_t
xcb_randr_list_provider_properties (xcb_connection_t     *c,
                                    xcb_randr_provider_t  provider);

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
xcb_randr_list_provider_properties_cookie_t
xcb_randr_list_provider_properties_unchecked (xcb_connection_t     *c,
                                              xcb_randr_provider_t  provider);

xcb_atom_t *
xcb_randr_list_provider_properties_atoms (const xcb_randr_list_provider_properties_reply_t *R);

int
xcb_randr_list_provider_properties_atoms_length (const xcb_randr_list_provider_properties_reply_t *R);

xcb_generic_iterator_t
xcb_randr_list_provider_properties_atoms_end (const xcb_randr_list_provider_properties_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_randr_list_provider_properties_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_randr_list_provider_properties_reply_t *
xcb_randr_list_provider_properties_reply (xcb_connection_t                             *c,
                                          xcb_randr_list_provider_properties_cookie_t   cookie  /**< */,
                                          xcb_generic_error_t                         **e);

int
xcb_randr_query_provider_property_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_randr_query_provider_property_cookie_t
xcb_randr_query_provider_property (xcb_connection_t     *c,
                                   xcb_randr_provider_t  provider,
                                   xcb_atom_t            property);

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
xcb_randr_query_provider_property_cookie_t
xcb_randr_query_provider_property_unchecked (xcb_connection_t     *c,
                                             xcb_randr_provider_t  provider,
                                             xcb_atom_t            property);

int32_t *
xcb_randr_query_provider_property_valid_values (const xcb_randr_query_provider_property_reply_t *R);

int
xcb_randr_query_provider_property_valid_values_length (const xcb_randr_query_provider_property_reply_t *R);

xcb_generic_iterator_t
xcb_randr_query_provider_property_valid_values_end (const xcb_randr_query_provider_property_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_randr_query_provider_property_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_randr_query_provider_property_reply_t *
xcb_randr_query_provider_property_reply (xcb_connection_t                            *c,
                                         xcb_randr_query_provider_property_cookie_t   cookie  /**< */,
                                         xcb_generic_error_t                        **e);

int
xcb_randr_configure_provider_property_sizeof (const void  *_buffer,
                                              uint32_t     values_len);

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
xcb_randr_configure_provider_property_checked (xcb_connection_t     *c,
                                               xcb_randr_provider_t  provider,
                                               xcb_atom_t            property,
                                               uint8_t               pending,
                                               uint8_t               range,
                                               uint32_t              values_len,
                                               const int32_t        *values);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_randr_configure_provider_property (xcb_connection_t     *c,
                                       xcb_randr_provider_t  provider,
                                       xcb_atom_t            property,
                                       uint8_t               pending,
                                       uint8_t               range,
                                       uint32_t              values_len,
                                       const int32_t        *values);

int32_t *
xcb_randr_configure_provider_property_values (const xcb_randr_configure_provider_property_request_t *R);

int
xcb_randr_configure_provider_property_values_length (const xcb_randr_configure_provider_property_request_t *R);

xcb_generic_iterator_t
xcb_randr_configure_provider_property_values_end (const xcb_randr_configure_provider_property_request_t *R);

int
xcb_randr_change_provider_property_sizeof (const void  *_buffer);

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
xcb_randr_change_provider_property_checked (xcb_connection_t     *c,
                                            xcb_randr_provider_t  provider,
                                            xcb_atom_t            property,
                                            xcb_atom_t            type,
                                            uint8_t               format,
                                            uint8_t               mode,
                                            uint32_t              num_items,
                                            const void           *data);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_randr_change_provider_property (xcb_connection_t     *c,
                                    xcb_randr_provider_t  provider,
                                    xcb_atom_t            property,
                                    xcb_atom_t            type,
                                    uint8_t               format,
                                    uint8_t               mode,
                                    uint32_t              num_items,
                                    const void           *data);

void *
xcb_randr_change_provider_property_data (const xcb_randr_change_provider_property_request_t *R);

int
xcb_randr_change_provider_property_data_length (const xcb_randr_change_provider_property_request_t *R);

xcb_generic_iterator_t
xcb_randr_change_provider_property_data_end (const xcb_randr_change_provider_property_request_t *R);

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
xcb_randr_delete_provider_property_checked (xcb_connection_t     *c,
                                            xcb_randr_provider_t  provider,
                                            xcb_atom_t            property);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_randr_delete_provider_property (xcb_connection_t     *c,
                                    xcb_randr_provider_t  provider,
                                    xcb_atom_t            property);

int
xcb_randr_get_provider_property_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_randr_get_provider_property_cookie_t
xcb_randr_get_provider_property (xcb_connection_t     *c,
                                 xcb_randr_provider_t  provider,
                                 xcb_atom_t            property,
                                 xcb_atom_t            type,
                                 uint32_t              long_offset,
                                 uint32_t              long_length,
                                 uint8_t               _delete,
                                 uint8_t               pending);

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
xcb_randr_get_provider_property_cookie_t
xcb_randr_get_provider_property_unchecked (xcb_connection_t     *c,
                                           xcb_randr_provider_t  provider,
                                           xcb_atom_t            property,
                                           xcb_atom_t            type,
                                           uint32_t              long_offset,
                                           uint32_t              long_length,
                                           uint8_t               _delete,
                                           uint8_t               pending);

void *
xcb_randr_get_provider_property_data (const xcb_randr_get_provider_property_reply_t *R);

int
xcb_randr_get_provider_property_data_length (const xcb_randr_get_provider_property_reply_t *R);

xcb_generic_iterator_t
xcb_randr_get_provider_property_data_end (const xcb_randr_get_provider_property_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_randr_get_provider_property_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_randr_get_provider_property_reply_t *
xcb_randr_get_provider_property_reply (xcb_connection_t                          *c,
                                       xcb_randr_get_provider_property_cookie_t   cookie  /**< */,
                                       xcb_generic_error_t                      **e);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_randr_crtc_change_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_randr_crtc_change_t)
 */
void
xcb_randr_crtc_change_next (xcb_randr_crtc_change_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_randr_crtc_change_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_randr_crtc_change_end (xcb_randr_crtc_change_iterator_t i);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_randr_output_change_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_randr_output_change_t)
 */
void
xcb_randr_output_change_next (xcb_randr_output_change_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_randr_output_change_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_randr_output_change_end (xcb_randr_output_change_iterator_t i);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_randr_output_property_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_randr_output_property_t)
 */
void
xcb_randr_output_property_next (xcb_randr_output_property_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_randr_output_property_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_randr_output_property_end (xcb_randr_output_property_iterator_t i);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_randr_provider_change_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_randr_provider_change_t)
 */
void
xcb_randr_provider_change_next (xcb_randr_provider_change_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_randr_provider_change_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_randr_provider_change_end (xcb_randr_provider_change_iterator_t i);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_randr_provider_property_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_randr_provider_property_t)
 */
void
xcb_randr_provider_property_next (xcb_randr_provider_property_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_randr_provider_property_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_randr_provider_property_end (xcb_randr_provider_property_iterator_t i);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_randr_resource_change_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_randr_resource_change_t)
 */
void
xcb_randr_resource_change_next (xcb_randr_resource_change_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_randr_resource_change_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_randr_resource_change_end (xcb_randr_resource_change_iterator_t i);

int
xcb_randr_monitor_info_sizeof (const void  *_buffer);

xcb_randr_output_t *
xcb_randr_monitor_info_outputs (const xcb_randr_monitor_info_t *R);

int
xcb_randr_monitor_info_outputs_length (const xcb_randr_monitor_info_t *R);

xcb_generic_iterator_t
xcb_randr_monitor_info_outputs_end (const xcb_randr_monitor_info_t *R);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_randr_monitor_info_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_randr_monitor_info_t)
 */
void
xcb_randr_monitor_info_next (xcb_randr_monitor_info_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_randr_monitor_info_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_randr_monitor_info_end (xcb_randr_monitor_info_iterator_t i);

int
xcb_randr_get_monitors_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_randr_get_monitors_cookie_t
xcb_randr_get_monitors (xcb_connection_t *c,
                        xcb_window_t      window,
                        uint8_t           get_active);

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
xcb_randr_get_monitors_cookie_t
xcb_randr_get_monitors_unchecked (xcb_connection_t *c,
                                  xcb_window_t      window,
                                  uint8_t           get_active);

int
xcb_randr_get_monitors_monitors_length (const xcb_randr_get_monitors_reply_t *R);

xcb_randr_monitor_info_iterator_t
xcb_randr_get_monitors_monitors_iterator (const xcb_randr_get_monitors_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_randr_get_monitors_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_randr_get_monitors_reply_t *
xcb_randr_get_monitors_reply (xcb_connection_t                 *c,
                              xcb_randr_get_monitors_cookie_t   cookie  /**< */,
                              xcb_generic_error_t             **e);

int
xcb_randr_set_monitor_sizeof (const void  *_buffer);

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
xcb_randr_set_monitor_checked (xcb_connection_t         *c,
                               xcb_window_t              window,
                               xcb_randr_monitor_info_t *monitorinfo);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_randr_set_monitor (xcb_connection_t         *c,
                       xcb_window_t              window,
                       xcb_randr_monitor_info_t *monitorinfo);

xcb_randr_monitor_info_t *
xcb_randr_set_monitor_monitorinfo (const xcb_randr_set_monitor_request_t *R);

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
xcb_randr_delete_monitor_checked (xcb_connection_t *c,
                                  xcb_window_t      window,
                                  xcb_atom_t        name);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_randr_delete_monitor (xcb_connection_t *c,
                          xcb_window_t      window,
                          xcb_atom_t        name);

int
xcb_randr_create_lease_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_randr_create_lease_cookie_t
xcb_randr_create_lease (xcb_connection_t         *c,
                        xcb_window_t              window,
                        xcb_randr_lease_t         lid,
                        uint16_t                  num_crtcs,
                        uint16_t                  num_outputs,
                        const xcb_randr_crtc_t   *crtcs,
                        const xcb_randr_output_t *outputs);

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
xcb_randr_create_lease_cookie_t
xcb_randr_create_lease_unchecked (xcb_connection_t         *c,
                                  xcb_window_t              window,
                                  xcb_randr_lease_t         lid,
                                  uint16_t                  num_crtcs,
                                  uint16_t                  num_outputs,
                                  const xcb_randr_crtc_t   *crtcs,
                                  const xcb_randr_output_t *outputs);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_randr_create_lease_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_randr_create_lease_reply_t *
xcb_randr_create_lease_reply (xcb_connection_t                 *c,
                              xcb_randr_create_lease_cookie_t   cookie  /**< */,
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
xcb_randr_create_lease_reply_fds (xcb_connection_t                *c  /**< */,
                                  xcb_randr_create_lease_reply_t  *reply);

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
xcb_randr_free_lease_checked (xcb_connection_t  *c,
                              xcb_randr_lease_t  lid,
                              uint8_t            terminate);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_randr_free_lease (xcb_connection_t  *c,
                      xcb_randr_lease_t  lid,
                      uint8_t            terminate);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_randr_lease_notify_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_randr_lease_notify_t)
 */
void
xcb_randr_lease_notify_next (xcb_randr_lease_notify_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_randr_lease_notify_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_randr_lease_notify_end (xcb_randr_lease_notify_iterator_t i);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_randr_notify_data_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_randr_notify_data_t)
 */
void
xcb_randr_notify_data_next (xcb_randr_notify_data_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_randr_notify_data_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_randr_notify_data_end (xcb_randr_notify_data_iterator_t i);


#ifdef __cplusplus
}
#endif

#endif

/**
 * @}
 */
