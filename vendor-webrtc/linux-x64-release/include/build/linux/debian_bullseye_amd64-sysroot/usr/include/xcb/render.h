/*
 * This file generated automatically from render.xml by c_client.py.
 * Edit at your peril.
 */

/**
 * @defgroup XCB_Render_API XCB Render API
 * @brief Render XCB Protocol Implementation.
 * @{
 **/

#ifndef __RENDER_H
#define __RENDER_H

#include "xcb.h"
#include "xproto.h"

#ifdef __cplusplus
extern "C" {
#endif

#define XCB_RENDER_MAJOR_VERSION 0
#define XCB_RENDER_MINOR_VERSION 11

extern xcb_extension_t xcb_render_id;

typedef enum xcb_render_pict_type_t {
    XCB_RENDER_PICT_TYPE_INDEXED = 0,
    XCB_RENDER_PICT_TYPE_DIRECT = 1
} xcb_render_pict_type_t;

typedef enum xcb_render_picture_enum_t {
    XCB_RENDER_PICTURE_NONE = 0
} xcb_render_picture_enum_t;

typedef enum xcb_render_pict_op_t {
    XCB_RENDER_PICT_OP_CLEAR = 0,
    XCB_RENDER_PICT_OP_SRC = 1,
    XCB_RENDER_PICT_OP_DST = 2,
    XCB_RENDER_PICT_OP_OVER = 3,
    XCB_RENDER_PICT_OP_OVER_REVERSE = 4,
    XCB_RENDER_PICT_OP_IN = 5,
    XCB_RENDER_PICT_OP_IN_REVERSE = 6,
    XCB_RENDER_PICT_OP_OUT = 7,
    XCB_RENDER_PICT_OP_OUT_REVERSE = 8,
    XCB_RENDER_PICT_OP_ATOP = 9,
    XCB_RENDER_PICT_OP_ATOP_REVERSE = 10,
    XCB_RENDER_PICT_OP_XOR = 11,
    XCB_RENDER_PICT_OP_ADD = 12,
    XCB_RENDER_PICT_OP_SATURATE = 13,
    XCB_RENDER_PICT_OP_DISJOINT_CLEAR = 16,
    XCB_RENDER_PICT_OP_DISJOINT_SRC = 17,
    XCB_RENDER_PICT_OP_DISJOINT_DST = 18,
    XCB_RENDER_PICT_OP_DISJOINT_OVER = 19,
    XCB_RENDER_PICT_OP_DISJOINT_OVER_REVERSE = 20,
    XCB_RENDER_PICT_OP_DISJOINT_IN = 21,
    XCB_RENDER_PICT_OP_DISJOINT_IN_REVERSE = 22,
    XCB_RENDER_PICT_OP_DISJOINT_OUT = 23,
    XCB_RENDER_PICT_OP_DISJOINT_OUT_REVERSE = 24,
    XCB_RENDER_PICT_OP_DISJOINT_ATOP = 25,
    XCB_RENDER_PICT_OP_DISJOINT_ATOP_REVERSE = 26,
    XCB_RENDER_PICT_OP_DISJOINT_XOR = 27,
    XCB_RENDER_PICT_OP_CONJOINT_CLEAR = 32,
    XCB_RENDER_PICT_OP_CONJOINT_SRC = 33,
    XCB_RENDER_PICT_OP_CONJOINT_DST = 34,
    XCB_RENDER_PICT_OP_CONJOINT_OVER = 35,
    XCB_RENDER_PICT_OP_CONJOINT_OVER_REVERSE = 36,
    XCB_RENDER_PICT_OP_CONJOINT_IN = 37,
    XCB_RENDER_PICT_OP_CONJOINT_IN_REVERSE = 38,
    XCB_RENDER_PICT_OP_CONJOINT_OUT = 39,
    XCB_RENDER_PICT_OP_CONJOINT_OUT_REVERSE = 40,
    XCB_RENDER_PICT_OP_CONJOINT_ATOP = 41,
    XCB_RENDER_PICT_OP_CONJOINT_ATOP_REVERSE = 42,
    XCB_RENDER_PICT_OP_CONJOINT_XOR = 43,
    XCB_RENDER_PICT_OP_MULTIPLY = 48,
    XCB_RENDER_PICT_OP_SCREEN = 49,
    XCB_RENDER_PICT_OP_OVERLAY = 50,
    XCB_RENDER_PICT_OP_DARKEN = 51,
    XCB_RENDER_PICT_OP_LIGHTEN = 52,
    XCB_RENDER_PICT_OP_COLOR_DODGE = 53,
    XCB_RENDER_PICT_OP_COLOR_BURN = 54,
    XCB_RENDER_PICT_OP_HARD_LIGHT = 55,
    XCB_RENDER_PICT_OP_SOFT_LIGHT = 56,
    XCB_RENDER_PICT_OP_DIFFERENCE = 57,
    XCB_RENDER_PICT_OP_EXCLUSION = 58,
    XCB_RENDER_PICT_OP_HSL_HUE = 59,
    XCB_RENDER_PICT_OP_HSL_SATURATION = 60,
    XCB_RENDER_PICT_OP_HSL_COLOR = 61,
    XCB_RENDER_PICT_OP_HSL_LUMINOSITY = 62
} xcb_render_pict_op_t;

typedef enum xcb_render_poly_edge_t {
    XCB_RENDER_POLY_EDGE_SHARP = 0,
    XCB_RENDER_POLY_EDGE_SMOOTH = 1
} xcb_render_poly_edge_t;

typedef enum xcb_render_poly_mode_t {
    XCB_RENDER_POLY_MODE_PRECISE = 0,
    XCB_RENDER_POLY_MODE_IMPRECISE = 1
} xcb_render_poly_mode_t;

typedef enum xcb_render_cp_t {
    XCB_RENDER_CP_REPEAT = 1,
    XCB_RENDER_CP_ALPHA_MAP = 2,
    XCB_RENDER_CP_ALPHA_X_ORIGIN = 4,
    XCB_RENDER_CP_ALPHA_Y_ORIGIN = 8,
    XCB_RENDER_CP_CLIP_X_ORIGIN = 16,
    XCB_RENDER_CP_CLIP_Y_ORIGIN = 32,
    XCB_RENDER_CP_CLIP_MASK = 64,
    XCB_RENDER_CP_GRAPHICS_EXPOSURE = 128,
    XCB_RENDER_CP_SUBWINDOW_MODE = 256,
    XCB_RENDER_CP_POLY_EDGE = 512,
    XCB_RENDER_CP_POLY_MODE = 1024,
    XCB_RENDER_CP_DITHER = 2048,
    XCB_RENDER_CP_COMPONENT_ALPHA = 4096
} xcb_render_cp_t;

typedef enum xcb_render_sub_pixel_t {
    XCB_RENDER_SUB_PIXEL_UNKNOWN = 0,
    XCB_RENDER_SUB_PIXEL_HORIZONTAL_RGB = 1,
    XCB_RENDER_SUB_PIXEL_HORIZONTAL_BGR = 2,
    XCB_RENDER_SUB_PIXEL_VERTICAL_RGB = 3,
    XCB_RENDER_SUB_PIXEL_VERTICAL_BGR = 4,
    XCB_RENDER_SUB_PIXEL_NONE = 5
} xcb_render_sub_pixel_t;

typedef enum xcb_render_repeat_t {
    XCB_RENDER_REPEAT_NONE = 0,
    XCB_RENDER_REPEAT_NORMAL = 1,
    XCB_RENDER_REPEAT_PAD = 2,
    XCB_RENDER_REPEAT_REFLECT = 3
} xcb_render_repeat_t;

typedef uint32_t xcb_render_glyph_t;

/**
 * @brief xcb_render_glyph_iterator_t
 **/
typedef struct xcb_render_glyph_iterator_t {
    xcb_render_glyph_t *data;
    int                 rem;
    int                 index;
} xcb_render_glyph_iterator_t;

typedef uint32_t xcb_render_glyphset_t;

/**
 * @brief xcb_render_glyphset_iterator_t
 **/
typedef struct xcb_render_glyphset_iterator_t {
    xcb_render_glyphset_t *data;
    int                    rem;
    int                    index;
} xcb_render_glyphset_iterator_t;

typedef uint32_t xcb_render_picture_t;

/**
 * @brief xcb_render_picture_iterator_t
 **/
typedef struct xcb_render_picture_iterator_t {
    xcb_render_picture_t *data;
    int                   rem;
    int                   index;
} xcb_render_picture_iterator_t;

typedef uint32_t xcb_render_pictformat_t;

/**
 * @brief xcb_render_pictformat_iterator_t
 **/
typedef struct xcb_render_pictformat_iterator_t {
    xcb_render_pictformat_t *data;
    int                      rem;
    int                      index;
} xcb_render_pictformat_iterator_t;

typedef int32_t xcb_render_fixed_t;

/**
 * @brief xcb_render_fixed_iterator_t
 **/
typedef struct xcb_render_fixed_iterator_t {
    xcb_render_fixed_t *data;
    int                 rem;
    int                 index;
} xcb_render_fixed_iterator_t;

/** Opcode for xcb_render_pict_format. */
#define XCB_RENDER_PICT_FORMAT 0

/**
 * @brief xcb_render_pict_format_error_t
 **/
typedef struct xcb_render_pict_format_error_t {
    uint8_t  response_type;
    uint8_t  error_code;
    uint16_t sequence;
} xcb_render_pict_format_error_t;

/** Opcode for xcb_render_picture. */
#define XCB_RENDER_PICTURE 1

/**
 * @brief xcb_render_picture_error_t
 **/
typedef struct xcb_render_picture_error_t {
    uint8_t  response_type;
    uint8_t  error_code;
    uint16_t sequence;
} xcb_render_picture_error_t;

/** Opcode for xcb_render_pict_op. */
#define XCB_RENDER_PICT_OP 2

/**
 * @brief xcb_render_pict_op_error_t
 **/
typedef struct xcb_render_pict_op_error_t {
    uint8_t  response_type;
    uint8_t  error_code;
    uint16_t sequence;
} xcb_render_pict_op_error_t;

/** Opcode for xcb_render_glyph_set. */
#define XCB_RENDER_GLYPH_SET 3

/**
 * @brief xcb_render_glyph_set_error_t
 **/
typedef struct xcb_render_glyph_set_error_t {
    uint8_t  response_type;
    uint8_t  error_code;
    uint16_t sequence;
} xcb_render_glyph_set_error_t;

/** Opcode for xcb_render_glyph. */
#define XCB_RENDER_GLYPH 4

/**
 * @brief xcb_render_glyph_error_t
 **/
typedef struct xcb_render_glyph_error_t {
    uint8_t  response_type;
    uint8_t  error_code;
    uint16_t sequence;
} xcb_render_glyph_error_t;

/**
 * @brief xcb_render_directformat_t
 **/
typedef struct xcb_render_directformat_t {
    uint16_t red_shift;
    uint16_t red_mask;
    uint16_t green_shift;
    uint16_t green_mask;
    uint16_t blue_shift;
    uint16_t blue_mask;
    uint16_t alpha_shift;
    uint16_t alpha_mask;
} xcb_render_directformat_t;

/**
 * @brief xcb_render_directformat_iterator_t
 **/
typedef struct xcb_render_directformat_iterator_t {
    xcb_render_directformat_t *data;
    int                        rem;
    int                        index;
} xcb_render_directformat_iterator_t;

/**
 * @brief xcb_render_pictforminfo_t
 **/
typedef struct xcb_render_pictforminfo_t {
    xcb_render_pictformat_t   id;
    uint8_t                   type;
    uint8_t                   depth;
    uint8_t                   pad0[2];
    xcb_render_directformat_t direct;
    xcb_colormap_t            colormap;
} xcb_render_pictforminfo_t;

/**
 * @brief xcb_render_pictforminfo_iterator_t
 **/
typedef struct xcb_render_pictforminfo_iterator_t {
    xcb_render_pictforminfo_t *data;
    int                        rem;
    int                        index;
} xcb_render_pictforminfo_iterator_t;

/**
 * @brief xcb_render_pictvisual_t
 **/
typedef struct xcb_render_pictvisual_t {
    xcb_visualid_t          visual;
    xcb_render_pictformat_t format;
} xcb_render_pictvisual_t;

/**
 * @brief xcb_render_pictvisual_iterator_t
 **/
typedef struct xcb_render_pictvisual_iterator_t {
    xcb_render_pictvisual_t *data;
    int                      rem;
    int                      index;
} xcb_render_pictvisual_iterator_t;

/**
 * @brief xcb_render_pictdepth_t
 **/
typedef struct xcb_render_pictdepth_t {
    uint8_t  depth;
    uint8_t  pad0;
    uint16_t num_visuals;
    uint8_t  pad1[4];
} xcb_render_pictdepth_t;

/**
 * @brief xcb_render_pictdepth_iterator_t
 **/
typedef struct xcb_render_pictdepth_iterator_t {
    xcb_render_pictdepth_t *data;
    int                     rem;
    int                     index;
} xcb_render_pictdepth_iterator_t;

/**
 * @brief xcb_render_pictscreen_t
 **/
typedef struct xcb_render_pictscreen_t {
    uint32_t                num_depths;
    xcb_render_pictformat_t fallback;
} xcb_render_pictscreen_t;

/**
 * @brief xcb_render_pictscreen_iterator_t
 **/
typedef struct xcb_render_pictscreen_iterator_t {
    xcb_render_pictscreen_t *data;
    int                      rem;
    int                      index;
} xcb_render_pictscreen_iterator_t;

/**
 * @brief xcb_render_indexvalue_t
 **/
typedef struct xcb_render_indexvalue_t {
    uint32_t pixel;
    uint16_t red;
    uint16_t green;
    uint16_t blue;
    uint16_t alpha;
} xcb_render_indexvalue_t;

/**
 * @brief xcb_render_indexvalue_iterator_t
 **/
typedef struct xcb_render_indexvalue_iterator_t {
    xcb_render_indexvalue_t *data;
    int                      rem;
    int                      index;
} xcb_render_indexvalue_iterator_t;

/**
 * @brief xcb_render_color_t
 **/
typedef struct xcb_render_color_t {
    uint16_t red;
    uint16_t green;
    uint16_t blue;
    uint16_t alpha;
} xcb_render_color_t;

/**
 * @brief xcb_render_color_iterator_t
 **/
typedef struct xcb_render_color_iterator_t {
    xcb_render_color_t *data;
    int                 rem;
    int                 index;
} xcb_render_color_iterator_t;

/**
 * @brief xcb_render_pointfix_t
 **/
typedef struct xcb_render_pointfix_t {
    xcb_render_fixed_t x;
    xcb_render_fixed_t y;
} xcb_render_pointfix_t;

/**
 * @brief xcb_render_pointfix_iterator_t
 **/
typedef struct xcb_render_pointfix_iterator_t {
    xcb_render_pointfix_t *data;
    int                    rem;
    int                    index;
} xcb_render_pointfix_iterator_t;

/**
 * @brief xcb_render_linefix_t
 **/
typedef struct xcb_render_linefix_t {
    xcb_render_pointfix_t p1;
    xcb_render_pointfix_t p2;
} xcb_render_linefix_t;

/**
 * @brief xcb_render_linefix_iterator_t
 **/
typedef struct xcb_render_linefix_iterator_t {
    xcb_render_linefix_t *data;
    int                   rem;
    int                   index;
} xcb_render_linefix_iterator_t;

/**
 * @brief xcb_render_triangle_t
 **/
typedef struct xcb_render_triangle_t {
    xcb_render_pointfix_t p1;
    xcb_render_pointfix_t p2;
    xcb_render_pointfix_t p3;
} xcb_render_triangle_t;

/**
 * @brief xcb_render_triangle_iterator_t
 **/
typedef struct xcb_render_triangle_iterator_t {
    xcb_render_triangle_t *data;
    int                    rem;
    int                    index;
} xcb_render_triangle_iterator_t;

/**
 * @brief xcb_render_trapezoid_t
 **/
typedef struct xcb_render_trapezoid_t {
    xcb_render_fixed_t   top;
    xcb_render_fixed_t   bottom;
    xcb_render_linefix_t left;
    xcb_render_linefix_t right;
} xcb_render_trapezoid_t;

/**
 * @brief xcb_render_trapezoid_iterator_t
 **/
typedef struct xcb_render_trapezoid_iterator_t {
    xcb_render_trapezoid_t *data;
    int                     rem;
    int                     index;
} xcb_render_trapezoid_iterator_t;

/**
 * @brief xcb_render_glyphinfo_t
 **/
typedef struct xcb_render_glyphinfo_t {
    uint16_t width;
    uint16_t height;
    int16_t  x;
    int16_t  y;
    int16_t  x_off;
    int16_t  y_off;
} xcb_render_glyphinfo_t;

/**
 * @brief xcb_render_glyphinfo_iterator_t
 **/
typedef struct xcb_render_glyphinfo_iterator_t {
    xcb_render_glyphinfo_t *data;
    int                     rem;
    int                     index;
} xcb_render_glyphinfo_iterator_t;

/**
 * @brief xcb_render_query_version_cookie_t
 **/
typedef struct xcb_render_query_version_cookie_t {
    unsigned int sequence;
} xcb_render_query_version_cookie_t;

/** Opcode for xcb_render_query_version. */
#define XCB_RENDER_QUERY_VERSION 0

/**
 * @brief xcb_render_query_version_request_t
 **/
typedef struct xcb_render_query_version_request_t {
    uint8_t  major_opcode;
    uint8_t  minor_opcode;
    uint16_t length;
    uint32_t client_major_version;
    uint32_t client_minor_version;
} xcb_render_query_version_request_t;

/**
 * @brief xcb_render_query_version_reply_t
 **/
typedef struct xcb_render_query_version_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint32_t major_version;
    uint32_t minor_version;
    uint8_t  pad1[16];
} xcb_render_query_version_reply_t;

/**
 * @brief xcb_render_query_pict_formats_cookie_t
 **/
typedef struct xcb_render_query_pict_formats_cookie_t {
    unsigned int sequence;
} xcb_render_query_pict_formats_cookie_t;

/** Opcode for xcb_render_query_pict_formats. */
#define XCB_RENDER_QUERY_PICT_FORMATS 1

/**
 * @brief xcb_render_query_pict_formats_request_t
 **/
typedef struct xcb_render_query_pict_formats_request_t {
    uint8_t  major_opcode;
    uint8_t  minor_opcode;
    uint16_t length;
} xcb_render_query_pict_formats_request_t;

/**
 * @brief xcb_render_query_pict_formats_reply_t
 **/
typedef struct xcb_render_query_pict_formats_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint32_t num_formats;
    uint32_t num_screens;
    uint32_t num_depths;
    uint32_t num_visuals;
    uint32_t num_subpixel;
    uint8_t  pad1[4];
} xcb_render_query_pict_formats_reply_t;

/**
 * @brief xcb_render_query_pict_index_values_cookie_t
 **/
typedef struct xcb_render_query_pict_index_values_cookie_t {
    unsigned int sequence;
} xcb_render_query_pict_index_values_cookie_t;

/** Opcode for xcb_render_query_pict_index_values. */
#define XCB_RENDER_QUERY_PICT_INDEX_VALUES 2

/**
 * @brief xcb_render_query_pict_index_values_request_t
 **/
typedef struct xcb_render_query_pict_index_values_request_t {
    uint8_t                 major_opcode;
    uint8_t                 minor_opcode;
    uint16_t                length;
    xcb_render_pictformat_t format;
} xcb_render_query_pict_index_values_request_t;

/**
 * @brief xcb_render_query_pict_index_values_reply_t
 **/
typedef struct xcb_render_query_pict_index_values_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint32_t num_values;
    uint8_t  pad1[20];
} xcb_render_query_pict_index_values_reply_t;

/**
 * @brief xcb_render_create_picture_value_list_t
 **/
typedef struct xcb_render_create_picture_value_list_t {
    uint32_t             repeat;
    xcb_render_picture_t alphamap;
    int32_t              alphaxorigin;
    int32_t              alphayorigin;
    int32_t              clipxorigin;
    int32_t              clipyorigin;
    xcb_pixmap_t         clipmask;
    uint32_t             graphicsexposure;
    uint32_t             subwindowmode;
    uint32_t             polyedge;
    uint32_t             polymode;
    xcb_atom_t           dither;
    uint32_t             componentalpha;
} xcb_render_create_picture_value_list_t;

/** Opcode for xcb_render_create_picture. */
#define XCB_RENDER_CREATE_PICTURE 4

/**
 * @brief xcb_render_create_picture_request_t
 **/
typedef struct xcb_render_create_picture_request_t {
    uint8_t                 major_opcode;
    uint8_t                 minor_opcode;
    uint16_t                length;
    xcb_render_picture_t    pid;
    xcb_drawable_t          drawable;
    xcb_render_pictformat_t format;
    uint32_t                value_mask;
} xcb_render_create_picture_request_t;

/**
 * @brief xcb_render_change_picture_value_list_t
 **/
typedef struct xcb_render_change_picture_value_list_t {
    uint32_t             repeat;
    xcb_render_picture_t alphamap;
    int32_t              alphaxorigin;
    int32_t              alphayorigin;
    int32_t              clipxorigin;
    int32_t              clipyorigin;
    xcb_pixmap_t         clipmask;
    uint32_t             graphicsexposure;
    uint32_t             subwindowmode;
    uint32_t             polyedge;
    uint32_t             polymode;
    xcb_atom_t           dither;
    uint32_t             componentalpha;
} xcb_render_change_picture_value_list_t;

/** Opcode for xcb_render_change_picture. */
#define XCB_RENDER_CHANGE_PICTURE 5

/**
 * @brief xcb_render_change_picture_request_t
 **/
typedef struct xcb_render_change_picture_request_t {
    uint8_t              major_opcode;
    uint8_t              minor_opcode;
    uint16_t             length;
    xcb_render_picture_t picture;
    uint32_t             value_mask;
} xcb_render_change_picture_request_t;

/** Opcode for xcb_render_set_picture_clip_rectangles. */
#define XCB_RENDER_SET_PICTURE_CLIP_RECTANGLES 6

/**
 * @brief xcb_render_set_picture_clip_rectangles_request_t
 **/
typedef struct xcb_render_set_picture_clip_rectangles_request_t {
    uint8_t              major_opcode;
    uint8_t              minor_opcode;
    uint16_t             length;
    xcb_render_picture_t picture;
    int16_t              clip_x_origin;
    int16_t              clip_y_origin;
} xcb_render_set_picture_clip_rectangles_request_t;

/** Opcode for xcb_render_free_picture. */
#define XCB_RENDER_FREE_PICTURE 7

/**
 * @brief xcb_render_free_picture_request_t
 **/
typedef struct xcb_render_free_picture_request_t {
    uint8_t              major_opcode;
    uint8_t              minor_opcode;
    uint16_t             length;
    xcb_render_picture_t picture;
} xcb_render_free_picture_request_t;

/** Opcode for xcb_render_composite. */
#define XCB_RENDER_COMPOSITE 8

/**
 * @brief xcb_render_composite_request_t
 **/
typedef struct xcb_render_composite_request_t {
    uint8_t              major_opcode;
    uint8_t              minor_opcode;
    uint16_t             length;
    uint8_t              op;
    uint8_t              pad0[3];
    xcb_render_picture_t src;
    xcb_render_picture_t mask;
    xcb_render_picture_t dst;
    int16_t              src_x;
    int16_t              src_y;
    int16_t              mask_x;
    int16_t              mask_y;
    int16_t              dst_x;
    int16_t              dst_y;
    uint16_t             width;
    uint16_t             height;
} xcb_render_composite_request_t;

/** Opcode for xcb_render_trapezoids. */
#define XCB_RENDER_TRAPEZOIDS 10

/**
 * @brief xcb_render_trapezoids_request_t
 **/
typedef struct xcb_render_trapezoids_request_t {
    uint8_t                 major_opcode;
    uint8_t                 minor_opcode;
    uint16_t                length;
    uint8_t                 op;
    uint8_t                 pad0[3];
    xcb_render_picture_t    src;
    xcb_render_picture_t    dst;
    xcb_render_pictformat_t mask_format;
    int16_t                 src_x;
    int16_t                 src_y;
} xcb_render_trapezoids_request_t;

/** Opcode for xcb_render_triangles. */
#define XCB_RENDER_TRIANGLES 11

/**
 * @brief xcb_render_triangles_request_t
 **/
typedef struct xcb_render_triangles_request_t {
    uint8_t                 major_opcode;
    uint8_t                 minor_opcode;
    uint16_t                length;
    uint8_t                 op;
    uint8_t                 pad0[3];
    xcb_render_picture_t    src;
    xcb_render_picture_t    dst;
    xcb_render_pictformat_t mask_format;
    int16_t                 src_x;
    int16_t                 src_y;
} xcb_render_triangles_request_t;

/** Opcode for xcb_render_tri_strip. */
#define XCB_RENDER_TRI_STRIP 12

/**
 * @brief xcb_render_tri_strip_request_t
 **/
typedef struct xcb_render_tri_strip_request_t {
    uint8_t                 major_opcode;
    uint8_t                 minor_opcode;
    uint16_t                length;
    uint8_t                 op;
    uint8_t                 pad0[3];
    xcb_render_picture_t    src;
    xcb_render_picture_t    dst;
    xcb_render_pictformat_t mask_format;
    int16_t                 src_x;
    int16_t                 src_y;
} xcb_render_tri_strip_request_t;

/** Opcode for xcb_render_tri_fan. */
#define XCB_RENDER_TRI_FAN 13

/**
 * @brief xcb_render_tri_fan_request_t
 **/
typedef struct xcb_render_tri_fan_request_t {
    uint8_t                 major_opcode;
    uint8_t                 minor_opcode;
    uint16_t                length;
    uint8_t                 op;
    uint8_t                 pad0[3];
    xcb_render_picture_t    src;
    xcb_render_picture_t    dst;
    xcb_render_pictformat_t mask_format;
    int16_t                 src_x;
    int16_t                 src_y;
} xcb_render_tri_fan_request_t;

/** Opcode for xcb_render_create_glyph_set. */
#define XCB_RENDER_CREATE_GLYPH_SET 17

/**
 * @brief xcb_render_create_glyph_set_request_t
 **/
typedef struct xcb_render_create_glyph_set_request_t {
    uint8_t                 major_opcode;
    uint8_t                 minor_opcode;
    uint16_t                length;
    xcb_render_glyphset_t   gsid;
    xcb_render_pictformat_t format;
} xcb_render_create_glyph_set_request_t;

/** Opcode for xcb_render_reference_glyph_set. */
#define XCB_RENDER_REFERENCE_GLYPH_SET 18

/**
 * @brief xcb_render_reference_glyph_set_request_t
 **/
typedef struct xcb_render_reference_glyph_set_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_render_glyphset_t gsid;
    xcb_render_glyphset_t existing;
} xcb_render_reference_glyph_set_request_t;

/** Opcode for xcb_render_free_glyph_set. */
#define XCB_RENDER_FREE_GLYPH_SET 19

/**
 * @brief xcb_render_free_glyph_set_request_t
 **/
typedef struct xcb_render_free_glyph_set_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_render_glyphset_t glyphset;
} xcb_render_free_glyph_set_request_t;

/** Opcode for xcb_render_add_glyphs. */
#define XCB_RENDER_ADD_GLYPHS 20

/**
 * @brief xcb_render_add_glyphs_request_t
 **/
typedef struct xcb_render_add_glyphs_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_render_glyphset_t glyphset;
    uint32_t              glyphs_len;
} xcb_render_add_glyphs_request_t;

/** Opcode for xcb_render_free_glyphs. */
#define XCB_RENDER_FREE_GLYPHS 22

/**
 * @brief xcb_render_free_glyphs_request_t
 **/
typedef struct xcb_render_free_glyphs_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_render_glyphset_t glyphset;
} xcb_render_free_glyphs_request_t;

/** Opcode for xcb_render_composite_glyphs_8. */
#define XCB_RENDER_COMPOSITE_GLYPHS_8 23

/**
 * @brief xcb_render_composite_glyphs_8_request_t
 **/
typedef struct xcb_render_composite_glyphs_8_request_t {
    uint8_t                 major_opcode;
    uint8_t                 minor_opcode;
    uint16_t                length;
    uint8_t                 op;
    uint8_t                 pad0[3];
    xcb_render_picture_t    src;
    xcb_render_picture_t    dst;
    xcb_render_pictformat_t mask_format;
    xcb_render_glyphset_t   glyphset;
    int16_t                 src_x;
    int16_t                 src_y;
} xcb_render_composite_glyphs_8_request_t;

/** Opcode for xcb_render_composite_glyphs_16. */
#define XCB_RENDER_COMPOSITE_GLYPHS_16 24

/**
 * @brief xcb_render_composite_glyphs_16_request_t
 **/
typedef struct xcb_render_composite_glyphs_16_request_t {
    uint8_t                 major_opcode;
    uint8_t                 minor_opcode;
    uint16_t                length;
    uint8_t                 op;
    uint8_t                 pad0[3];
    xcb_render_picture_t    src;
    xcb_render_picture_t    dst;
    xcb_render_pictformat_t mask_format;
    xcb_render_glyphset_t   glyphset;
    int16_t                 src_x;
    int16_t                 src_y;
} xcb_render_composite_glyphs_16_request_t;

/** Opcode for xcb_render_composite_glyphs_32. */
#define XCB_RENDER_COMPOSITE_GLYPHS_32 25

/**
 * @brief xcb_render_composite_glyphs_32_request_t
 **/
typedef struct xcb_render_composite_glyphs_32_request_t {
    uint8_t                 major_opcode;
    uint8_t                 minor_opcode;
    uint16_t                length;
    uint8_t                 op;
    uint8_t                 pad0[3];
    xcb_render_picture_t    src;
    xcb_render_picture_t    dst;
    xcb_render_pictformat_t mask_format;
    xcb_render_glyphset_t   glyphset;
    int16_t                 src_x;
    int16_t                 src_y;
} xcb_render_composite_glyphs_32_request_t;

/** Opcode for xcb_render_fill_rectangles. */
#define XCB_RENDER_FILL_RECTANGLES 26

/**
 * @brief xcb_render_fill_rectangles_request_t
 **/
typedef struct xcb_render_fill_rectangles_request_t {
    uint8_t              major_opcode;
    uint8_t              minor_opcode;
    uint16_t             length;
    uint8_t              op;
    uint8_t              pad0[3];
    xcb_render_picture_t dst;
    xcb_render_color_t   color;
} xcb_render_fill_rectangles_request_t;

/** Opcode for xcb_render_create_cursor. */
#define XCB_RENDER_CREATE_CURSOR 27

/**
 * @brief xcb_render_create_cursor_request_t
 **/
typedef struct xcb_render_create_cursor_request_t {
    uint8_t              major_opcode;
    uint8_t              minor_opcode;
    uint16_t             length;
    xcb_cursor_t         cid;
    xcb_render_picture_t source;
    uint16_t             x;
    uint16_t             y;
} xcb_render_create_cursor_request_t;

/**
 * @brief xcb_render_transform_t
 **/
typedef struct xcb_render_transform_t {
    xcb_render_fixed_t matrix11;
    xcb_render_fixed_t matrix12;
    xcb_render_fixed_t matrix13;
    xcb_render_fixed_t matrix21;
    xcb_render_fixed_t matrix22;
    xcb_render_fixed_t matrix23;
    xcb_render_fixed_t matrix31;
    xcb_render_fixed_t matrix32;
    xcb_render_fixed_t matrix33;
} xcb_render_transform_t;

/**
 * @brief xcb_render_transform_iterator_t
 **/
typedef struct xcb_render_transform_iterator_t {
    xcb_render_transform_t *data;
    int                     rem;
    int                     index;
} xcb_render_transform_iterator_t;

/** Opcode for xcb_render_set_picture_transform. */
#define XCB_RENDER_SET_PICTURE_TRANSFORM 28

/**
 * @brief xcb_render_set_picture_transform_request_t
 **/
typedef struct xcb_render_set_picture_transform_request_t {
    uint8_t                major_opcode;
    uint8_t                minor_opcode;
    uint16_t               length;
    xcb_render_picture_t   picture;
    xcb_render_transform_t transform;
} xcb_render_set_picture_transform_request_t;

/**
 * @brief xcb_render_query_filters_cookie_t
 **/
typedef struct xcb_render_query_filters_cookie_t {
    unsigned int sequence;
} xcb_render_query_filters_cookie_t;

/** Opcode for xcb_render_query_filters. */
#define XCB_RENDER_QUERY_FILTERS 29

/**
 * @brief xcb_render_query_filters_request_t
 **/
typedef struct xcb_render_query_filters_request_t {
    uint8_t        major_opcode;
    uint8_t        minor_opcode;
    uint16_t       length;
    xcb_drawable_t drawable;
} xcb_render_query_filters_request_t;

/**
 * @brief xcb_render_query_filters_reply_t
 **/
typedef struct xcb_render_query_filters_reply_t {
    uint8_t  response_type;
    uint8_t  pad0;
    uint16_t sequence;
    uint32_t length;
    uint32_t num_aliases;
    uint32_t num_filters;
    uint8_t  pad1[16];
} xcb_render_query_filters_reply_t;

/** Opcode for xcb_render_set_picture_filter. */
#define XCB_RENDER_SET_PICTURE_FILTER 30

/**
 * @brief xcb_render_set_picture_filter_request_t
 **/
typedef struct xcb_render_set_picture_filter_request_t {
    uint8_t              major_opcode;
    uint8_t              minor_opcode;
    uint16_t             length;
    xcb_render_picture_t picture;
    uint16_t             filter_len;
    uint8_t              pad0[2];
} xcb_render_set_picture_filter_request_t;

/**
 * @brief xcb_render_animcursorelt_t
 **/
typedef struct xcb_render_animcursorelt_t {
    xcb_cursor_t cursor;
    uint32_t     delay;
} xcb_render_animcursorelt_t;

/**
 * @brief xcb_render_animcursorelt_iterator_t
 **/
typedef struct xcb_render_animcursorelt_iterator_t {
    xcb_render_animcursorelt_t *data;
    int                         rem;
    int                         index;
} xcb_render_animcursorelt_iterator_t;

/** Opcode for xcb_render_create_anim_cursor. */
#define XCB_RENDER_CREATE_ANIM_CURSOR 31

/**
 * @brief xcb_render_create_anim_cursor_request_t
 **/
typedef struct xcb_render_create_anim_cursor_request_t {
    uint8_t      major_opcode;
    uint8_t      minor_opcode;
    uint16_t     length;
    xcb_cursor_t cid;
} xcb_render_create_anim_cursor_request_t;

/**
 * @brief xcb_render_spanfix_t
 **/
typedef struct xcb_render_spanfix_t {
    xcb_render_fixed_t l;
    xcb_render_fixed_t r;
    xcb_render_fixed_t y;
} xcb_render_spanfix_t;

/**
 * @brief xcb_render_spanfix_iterator_t
 **/
typedef struct xcb_render_spanfix_iterator_t {
    xcb_render_spanfix_t *data;
    int                   rem;
    int                   index;
} xcb_render_spanfix_iterator_t;

/**
 * @brief xcb_render_trap_t
 **/
typedef struct xcb_render_trap_t {
    xcb_render_spanfix_t top;
    xcb_render_spanfix_t bot;
} xcb_render_trap_t;

/**
 * @brief xcb_render_trap_iterator_t
 **/
typedef struct xcb_render_trap_iterator_t {
    xcb_render_trap_t *data;
    int                rem;
    int                index;
} xcb_render_trap_iterator_t;

/** Opcode for xcb_render_add_traps. */
#define XCB_RENDER_ADD_TRAPS 32

/**
 * @brief xcb_render_add_traps_request_t
 **/
typedef struct xcb_render_add_traps_request_t {
    uint8_t              major_opcode;
    uint8_t              minor_opcode;
    uint16_t             length;
    xcb_render_picture_t picture;
    int16_t              x_off;
    int16_t              y_off;
} xcb_render_add_traps_request_t;

/** Opcode for xcb_render_create_solid_fill. */
#define XCB_RENDER_CREATE_SOLID_FILL 33

/**
 * @brief xcb_render_create_solid_fill_request_t
 **/
typedef struct xcb_render_create_solid_fill_request_t {
    uint8_t              major_opcode;
    uint8_t              minor_opcode;
    uint16_t             length;
    xcb_render_picture_t picture;
    xcb_render_color_t   color;
} xcb_render_create_solid_fill_request_t;

/** Opcode for xcb_render_create_linear_gradient. */
#define XCB_RENDER_CREATE_LINEAR_GRADIENT 34

/**
 * @brief xcb_render_create_linear_gradient_request_t
 **/
typedef struct xcb_render_create_linear_gradient_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_render_picture_t  picture;
    xcb_render_pointfix_t p1;
    xcb_render_pointfix_t p2;
    uint32_t              num_stops;
} xcb_render_create_linear_gradient_request_t;

/** Opcode for xcb_render_create_radial_gradient. */
#define XCB_RENDER_CREATE_RADIAL_GRADIENT 35

/**
 * @brief xcb_render_create_radial_gradient_request_t
 **/
typedef struct xcb_render_create_radial_gradient_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_render_picture_t  picture;
    xcb_render_pointfix_t inner;
    xcb_render_pointfix_t outer;
    xcb_render_fixed_t    inner_radius;
    xcb_render_fixed_t    outer_radius;
    uint32_t              num_stops;
} xcb_render_create_radial_gradient_request_t;

/** Opcode for xcb_render_create_conical_gradient. */
#define XCB_RENDER_CREATE_CONICAL_GRADIENT 36

/**
 * @brief xcb_render_create_conical_gradient_request_t
 **/
typedef struct xcb_render_create_conical_gradient_request_t {
    uint8_t               major_opcode;
    uint8_t               minor_opcode;
    uint16_t              length;
    xcb_render_picture_t  picture;
    xcb_render_pointfix_t center;
    xcb_render_fixed_t    angle;
    uint32_t              num_stops;
} xcb_render_create_conical_gradient_request_t;

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_render_glyph_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_render_glyph_t)
 */
void
xcb_render_glyph_next (xcb_render_glyph_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_render_glyph_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_render_glyph_end (xcb_render_glyph_iterator_t i);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_render_glyphset_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_render_glyphset_t)
 */
void
xcb_render_glyphset_next (xcb_render_glyphset_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_render_glyphset_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_render_glyphset_end (xcb_render_glyphset_iterator_t i);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_render_picture_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_render_picture_t)
 */
void
xcb_render_picture_next (xcb_render_picture_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_render_picture_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_render_picture_end (xcb_render_picture_iterator_t i);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_render_pictformat_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_render_pictformat_t)
 */
void
xcb_render_pictformat_next (xcb_render_pictformat_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_render_pictformat_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_render_pictformat_end (xcb_render_pictformat_iterator_t i);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_render_fixed_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_render_fixed_t)
 */
void
xcb_render_fixed_next (xcb_render_fixed_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_render_fixed_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_render_fixed_end (xcb_render_fixed_iterator_t i);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_render_directformat_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_render_directformat_t)
 */
void
xcb_render_directformat_next (xcb_render_directformat_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_render_directformat_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_render_directformat_end (xcb_render_directformat_iterator_t i);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_render_pictforminfo_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_render_pictforminfo_t)
 */
void
xcb_render_pictforminfo_next (xcb_render_pictforminfo_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_render_pictforminfo_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_render_pictforminfo_end (xcb_render_pictforminfo_iterator_t i);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_render_pictvisual_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_render_pictvisual_t)
 */
void
xcb_render_pictvisual_next (xcb_render_pictvisual_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_render_pictvisual_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_render_pictvisual_end (xcb_render_pictvisual_iterator_t i);

int
xcb_render_pictdepth_sizeof (const void  *_buffer);

xcb_render_pictvisual_t *
xcb_render_pictdepth_visuals (const xcb_render_pictdepth_t *R);

int
xcb_render_pictdepth_visuals_length (const xcb_render_pictdepth_t *R);

xcb_render_pictvisual_iterator_t
xcb_render_pictdepth_visuals_iterator (const xcb_render_pictdepth_t *R);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_render_pictdepth_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_render_pictdepth_t)
 */
void
xcb_render_pictdepth_next (xcb_render_pictdepth_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_render_pictdepth_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_render_pictdepth_end (xcb_render_pictdepth_iterator_t i);

int
xcb_render_pictscreen_sizeof (const void  *_buffer);

int
xcb_render_pictscreen_depths_length (const xcb_render_pictscreen_t *R);

xcb_render_pictdepth_iterator_t
xcb_render_pictscreen_depths_iterator (const xcb_render_pictscreen_t *R);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_render_pictscreen_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_render_pictscreen_t)
 */
void
xcb_render_pictscreen_next (xcb_render_pictscreen_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_render_pictscreen_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_render_pictscreen_end (xcb_render_pictscreen_iterator_t i);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_render_indexvalue_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_render_indexvalue_t)
 */
void
xcb_render_indexvalue_next (xcb_render_indexvalue_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_render_indexvalue_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_render_indexvalue_end (xcb_render_indexvalue_iterator_t i);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_render_color_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_render_color_t)
 */
void
xcb_render_color_next (xcb_render_color_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_render_color_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_render_color_end (xcb_render_color_iterator_t i);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_render_pointfix_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_render_pointfix_t)
 */
void
xcb_render_pointfix_next (xcb_render_pointfix_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_render_pointfix_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_render_pointfix_end (xcb_render_pointfix_iterator_t i);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_render_linefix_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_render_linefix_t)
 */
void
xcb_render_linefix_next (xcb_render_linefix_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_render_linefix_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_render_linefix_end (xcb_render_linefix_iterator_t i);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_render_triangle_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_render_triangle_t)
 */
void
xcb_render_triangle_next (xcb_render_triangle_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_render_triangle_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_render_triangle_end (xcb_render_triangle_iterator_t i);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_render_trapezoid_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_render_trapezoid_t)
 */
void
xcb_render_trapezoid_next (xcb_render_trapezoid_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_render_trapezoid_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_render_trapezoid_end (xcb_render_trapezoid_iterator_t i);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_render_glyphinfo_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_render_glyphinfo_t)
 */
void
xcb_render_glyphinfo_next (xcb_render_glyphinfo_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_render_glyphinfo_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_render_glyphinfo_end (xcb_render_glyphinfo_iterator_t i);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_render_query_version_cookie_t
xcb_render_query_version (xcb_connection_t *c,
                          uint32_t          client_major_version,
                          uint32_t          client_minor_version);

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
xcb_render_query_version_cookie_t
xcb_render_query_version_unchecked (xcb_connection_t *c,
                                    uint32_t          client_major_version,
                                    uint32_t          client_minor_version);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_render_query_version_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_render_query_version_reply_t *
xcb_render_query_version_reply (xcb_connection_t                   *c,
                                xcb_render_query_version_cookie_t   cookie  /**< */,
                                xcb_generic_error_t               **e);

int
xcb_render_query_pict_formats_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_render_query_pict_formats_cookie_t
xcb_render_query_pict_formats (xcb_connection_t *c);

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
xcb_render_query_pict_formats_cookie_t
xcb_render_query_pict_formats_unchecked (xcb_connection_t *c);

xcb_render_pictforminfo_t *
xcb_render_query_pict_formats_formats (const xcb_render_query_pict_formats_reply_t *R);

int
xcb_render_query_pict_formats_formats_length (const xcb_render_query_pict_formats_reply_t *R);

xcb_render_pictforminfo_iterator_t
xcb_render_query_pict_formats_formats_iterator (const xcb_render_query_pict_formats_reply_t *R);

int
xcb_render_query_pict_formats_screens_length (const xcb_render_query_pict_formats_reply_t *R);

xcb_render_pictscreen_iterator_t
xcb_render_query_pict_formats_screens_iterator (const xcb_render_query_pict_formats_reply_t *R);

uint32_t *
xcb_render_query_pict_formats_subpixels (const xcb_render_query_pict_formats_reply_t *R);

int
xcb_render_query_pict_formats_subpixels_length (const xcb_render_query_pict_formats_reply_t *R);

xcb_generic_iterator_t
xcb_render_query_pict_formats_subpixels_end (const xcb_render_query_pict_formats_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_render_query_pict_formats_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_render_query_pict_formats_reply_t *
xcb_render_query_pict_formats_reply (xcb_connection_t                        *c,
                                     xcb_render_query_pict_formats_cookie_t   cookie  /**< */,
                                     xcb_generic_error_t                    **e);

int
xcb_render_query_pict_index_values_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_render_query_pict_index_values_cookie_t
xcb_render_query_pict_index_values (xcb_connection_t        *c,
                                    xcb_render_pictformat_t  format);

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
xcb_render_query_pict_index_values_cookie_t
xcb_render_query_pict_index_values_unchecked (xcb_connection_t        *c,
                                              xcb_render_pictformat_t  format);

xcb_render_indexvalue_t *
xcb_render_query_pict_index_values_values (const xcb_render_query_pict_index_values_reply_t *R);

int
xcb_render_query_pict_index_values_values_length (const xcb_render_query_pict_index_values_reply_t *R);

xcb_render_indexvalue_iterator_t
xcb_render_query_pict_index_values_values_iterator (const xcb_render_query_pict_index_values_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_render_query_pict_index_values_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_render_query_pict_index_values_reply_t *
xcb_render_query_pict_index_values_reply (xcb_connection_t                             *c,
                                          xcb_render_query_pict_index_values_cookie_t   cookie  /**< */,
                                          xcb_generic_error_t                         **e);

int
xcb_render_create_picture_value_list_serialize (void                                         **_buffer,
                                                uint32_t                                       value_mask,
                                                const xcb_render_create_picture_value_list_t  *_aux);

int
xcb_render_create_picture_value_list_unpack (const void                              *_buffer,
                                             uint32_t                                 value_mask,
                                             xcb_render_create_picture_value_list_t  *_aux);

int
xcb_render_create_picture_value_list_sizeof (const void  *_buffer,
                                             uint32_t     value_mask);

int
xcb_render_create_picture_sizeof (const void  *_buffer);

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
xcb_render_create_picture_checked (xcb_connection_t        *c,
                                   xcb_render_picture_t     pid,
                                   xcb_drawable_t           drawable,
                                   xcb_render_pictformat_t  format,
                                   uint32_t                 value_mask,
                                   const void              *value_list);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_render_create_picture (xcb_connection_t        *c,
                           xcb_render_picture_t     pid,
                           xcb_drawable_t           drawable,
                           xcb_render_pictformat_t  format,
                           uint32_t                 value_mask,
                           const void              *value_list);

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
xcb_render_create_picture_aux_checked (xcb_connection_t                             *c,
                                       xcb_render_picture_t                          pid,
                                       xcb_drawable_t                                drawable,
                                       xcb_render_pictformat_t                       format,
                                       uint32_t                                      value_mask,
                                       const xcb_render_create_picture_value_list_t *value_list);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_render_create_picture_aux (xcb_connection_t                             *c,
                               xcb_render_picture_t                          pid,
                               xcb_drawable_t                                drawable,
                               xcb_render_pictformat_t                       format,
                               uint32_t                                      value_mask,
                               const xcb_render_create_picture_value_list_t *value_list);

void *
xcb_render_create_picture_value_list (const xcb_render_create_picture_request_t *R);

int
xcb_render_change_picture_value_list_serialize (void                                         **_buffer,
                                                uint32_t                                       value_mask,
                                                const xcb_render_change_picture_value_list_t  *_aux);

int
xcb_render_change_picture_value_list_unpack (const void                              *_buffer,
                                             uint32_t                                 value_mask,
                                             xcb_render_change_picture_value_list_t  *_aux);

int
xcb_render_change_picture_value_list_sizeof (const void  *_buffer,
                                             uint32_t     value_mask);

int
xcb_render_change_picture_sizeof (const void  *_buffer);

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
xcb_render_change_picture_checked (xcb_connection_t     *c,
                                   xcb_render_picture_t  picture,
                                   uint32_t              value_mask,
                                   const void           *value_list);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_render_change_picture (xcb_connection_t     *c,
                           xcb_render_picture_t  picture,
                           uint32_t              value_mask,
                           const void           *value_list);

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
xcb_render_change_picture_aux_checked (xcb_connection_t                             *c,
                                       xcb_render_picture_t                          picture,
                                       uint32_t                                      value_mask,
                                       const xcb_render_change_picture_value_list_t *value_list);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_render_change_picture_aux (xcb_connection_t                             *c,
                               xcb_render_picture_t                          picture,
                               uint32_t                                      value_mask,
                               const xcb_render_change_picture_value_list_t *value_list);

void *
xcb_render_change_picture_value_list (const xcb_render_change_picture_request_t *R);

int
xcb_render_set_picture_clip_rectangles_sizeof (const void  *_buffer,
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
xcb_render_set_picture_clip_rectangles_checked (xcb_connection_t      *c,
                                                xcb_render_picture_t   picture,
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
xcb_render_set_picture_clip_rectangles (xcb_connection_t      *c,
                                        xcb_render_picture_t   picture,
                                        int16_t                clip_x_origin,
                                        int16_t                clip_y_origin,
                                        uint32_t               rectangles_len,
                                        const xcb_rectangle_t *rectangles);

xcb_rectangle_t *
xcb_render_set_picture_clip_rectangles_rectangles (const xcb_render_set_picture_clip_rectangles_request_t *R);

int
xcb_render_set_picture_clip_rectangles_rectangles_length (const xcb_render_set_picture_clip_rectangles_request_t *R);

xcb_rectangle_iterator_t
xcb_render_set_picture_clip_rectangles_rectangles_iterator (const xcb_render_set_picture_clip_rectangles_request_t *R);

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
xcb_render_free_picture_checked (xcb_connection_t     *c,
                                 xcb_render_picture_t  picture);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_render_free_picture (xcb_connection_t     *c,
                         xcb_render_picture_t  picture);

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
xcb_render_composite_checked (xcb_connection_t     *c,
                              uint8_t               op,
                              xcb_render_picture_t  src,
                              xcb_render_picture_t  mask,
                              xcb_render_picture_t  dst,
                              int16_t               src_x,
                              int16_t               src_y,
                              int16_t               mask_x,
                              int16_t               mask_y,
                              int16_t               dst_x,
                              int16_t               dst_y,
                              uint16_t              width,
                              uint16_t              height);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_render_composite (xcb_connection_t     *c,
                      uint8_t               op,
                      xcb_render_picture_t  src,
                      xcb_render_picture_t  mask,
                      xcb_render_picture_t  dst,
                      int16_t               src_x,
                      int16_t               src_y,
                      int16_t               mask_x,
                      int16_t               mask_y,
                      int16_t               dst_x,
                      int16_t               dst_y,
                      uint16_t              width,
                      uint16_t              height);

int
xcb_render_trapezoids_sizeof (const void  *_buffer,
                              uint32_t     traps_len);

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
xcb_render_trapezoids_checked (xcb_connection_t             *c,
                               uint8_t                       op,
                               xcb_render_picture_t          src,
                               xcb_render_picture_t          dst,
                               xcb_render_pictformat_t       mask_format,
                               int16_t                       src_x,
                               int16_t                       src_y,
                               uint32_t                      traps_len,
                               const xcb_render_trapezoid_t *traps);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_render_trapezoids (xcb_connection_t             *c,
                       uint8_t                       op,
                       xcb_render_picture_t          src,
                       xcb_render_picture_t          dst,
                       xcb_render_pictformat_t       mask_format,
                       int16_t                       src_x,
                       int16_t                       src_y,
                       uint32_t                      traps_len,
                       const xcb_render_trapezoid_t *traps);

xcb_render_trapezoid_t *
xcb_render_trapezoids_traps (const xcb_render_trapezoids_request_t *R);

int
xcb_render_trapezoids_traps_length (const xcb_render_trapezoids_request_t *R);

xcb_render_trapezoid_iterator_t
xcb_render_trapezoids_traps_iterator (const xcb_render_trapezoids_request_t *R);

int
xcb_render_triangles_sizeof (const void  *_buffer,
                             uint32_t     triangles_len);

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
xcb_render_triangles_checked (xcb_connection_t            *c,
                              uint8_t                      op,
                              xcb_render_picture_t         src,
                              xcb_render_picture_t         dst,
                              xcb_render_pictformat_t      mask_format,
                              int16_t                      src_x,
                              int16_t                      src_y,
                              uint32_t                     triangles_len,
                              const xcb_render_triangle_t *triangles);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_render_triangles (xcb_connection_t            *c,
                      uint8_t                      op,
                      xcb_render_picture_t         src,
                      xcb_render_picture_t         dst,
                      xcb_render_pictformat_t      mask_format,
                      int16_t                      src_x,
                      int16_t                      src_y,
                      uint32_t                     triangles_len,
                      const xcb_render_triangle_t *triangles);

xcb_render_triangle_t *
xcb_render_triangles_triangles (const xcb_render_triangles_request_t *R);

int
xcb_render_triangles_triangles_length (const xcb_render_triangles_request_t *R);

xcb_render_triangle_iterator_t
xcb_render_triangles_triangles_iterator (const xcb_render_triangles_request_t *R);

int
xcb_render_tri_strip_sizeof (const void  *_buffer,
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
xcb_render_tri_strip_checked (xcb_connection_t            *c,
                              uint8_t                      op,
                              xcb_render_picture_t         src,
                              xcb_render_picture_t         dst,
                              xcb_render_pictformat_t      mask_format,
                              int16_t                      src_x,
                              int16_t                      src_y,
                              uint32_t                     points_len,
                              const xcb_render_pointfix_t *points);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_render_tri_strip (xcb_connection_t            *c,
                      uint8_t                      op,
                      xcb_render_picture_t         src,
                      xcb_render_picture_t         dst,
                      xcb_render_pictformat_t      mask_format,
                      int16_t                      src_x,
                      int16_t                      src_y,
                      uint32_t                     points_len,
                      const xcb_render_pointfix_t *points);

xcb_render_pointfix_t *
xcb_render_tri_strip_points (const xcb_render_tri_strip_request_t *R);

int
xcb_render_tri_strip_points_length (const xcb_render_tri_strip_request_t *R);

xcb_render_pointfix_iterator_t
xcb_render_tri_strip_points_iterator (const xcb_render_tri_strip_request_t *R);

int
xcb_render_tri_fan_sizeof (const void  *_buffer,
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
xcb_render_tri_fan_checked (xcb_connection_t            *c,
                            uint8_t                      op,
                            xcb_render_picture_t         src,
                            xcb_render_picture_t         dst,
                            xcb_render_pictformat_t      mask_format,
                            int16_t                      src_x,
                            int16_t                      src_y,
                            uint32_t                     points_len,
                            const xcb_render_pointfix_t *points);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_render_tri_fan (xcb_connection_t            *c,
                    uint8_t                      op,
                    xcb_render_picture_t         src,
                    xcb_render_picture_t         dst,
                    xcb_render_pictformat_t      mask_format,
                    int16_t                      src_x,
                    int16_t                      src_y,
                    uint32_t                     points_len,
                    const xcb_render_pointfix_t *points);

xcb_render_pointfix_t *
xcb_render_tri_fan_points (const xcb_render_tri_fan_request_t *R);

int
xcb_render_tri_fan_points_length (const xcb_render_tri_fan_request_t *R);

xcb_render_pointfix_iterator_t
xcb_render_tri_fan_points_iterator (const xcb_render_tri_fan_request_t *R);

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
xcb_render_create_glyph_set_checked (xcb_connection_t        *c,
                                     xcb_render_glyphset_t    gsid,
                                     xcb_render_pictformat_t  format);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_render_create_glyph_set (xcb_connection_t        *c,
                             xcb_render_glyphset_t    gsid,
                             xcb_render_pictformat_t  format);

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
xcb_render_reference_glyph_set_checked (xcb_connection_t      *c,
                                        xcb_render_glyphset_t  gsid,
                                        xcb_render_glyphset_t  existing);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_render_reference_glyph_set (xcb_connection_t      *c,
                                xcb_render_glyphset_t  gsid,
                                xcb_render_glyphset_t  existing);

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
xcb_render_free_glyph_set_checked (xcb_connection_t      *c,
                                   xcb_render_glyphset_t  glyphset);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_render_free_glyph_set (xcb_connection_t      *c,
                           xcb_render_glyphset_t  glyphset);

int
xcb_render_add_glyphs_sizeof (const void  *_buffer,
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
xcb_render_add_glyphs_checked (xcb_connection_t             *c,
                               xcb_render_glyphset_t         glyphset,
                               uint32_t                      glyphs_len,
                               const uint32_t               *glyphids,
                               const xcb_render_glyphinfo_t *glyphs,
                               uint32_t                      data_len,
                               const uint8_t                *data);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_render_add_glyphs (xcb_connection_t             *c,
                       xcb_render_glyphset_t         glyphset,
                       uint32_t                      glyphs_len,
                       const uint32_t               *glyphids,
                       const xcb_render_glyphinfo_t *glyphs,
                       uint32_t                      data_len,
                       const uint8_t                *data);

uint32_t *
xcb_render_add_glyphs_glyphids (const xcb_render_add_glyphs_request_t *R);

int
xcb_render_add_glyphs_glyphids_length (const xcb_render_add_glyphs_request_t *R);

xcb_generic_iterator_t
xcb_render_add_glyphs_glyphids_end (const xcb_render_add_glyphs_request_t *R);

xcb_render_glyphinfo_t *
xcb_render_add_glyphs_glyphs (const xcb_render_add_glyphs_request_t *R);

int
xcb_render_add_glyphs_glyphs_length (const xcb_render_add_glyphs_request_t *R);

xcb_render_glyphinfo_iterator_t
xcb_render_add_glyphs_glyphs_iterator (const xcb_render_add_glyphs_request_t *R);

uint8_t *
xcb_render_add_glyphs_data (const xcb_render_add_glyphs_request_t *R);

int
xcb_render_add_glyphs_data_length (const xcb_render_add_glyphs_request_t *R);

xcb_generic_iterator_t
xcb_render_add_glyphs_data_end (const xcb_render_add_glyphs_request_t *R);

int
xcb_render_free_glyphs_sizeof (const void  *_buffer,
                               uint32_t     glyphs_len);

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
xcb_render_free_glyphs_checked (xcb_connection_t         *c,
                                xcb_render_glyphset_t     glyphset,
                                uint32_t                  glyphs_len,
                                const xcb_render_glyph_t *glyphs);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_render_free_glyphs (xcb_connection_t         *c,
                        xcb_render_glyphset_t     glyphset,
                        uint32_t                  glyphs_len,
                        const xcb_render_glyph_t *glyphs);

xcb_render_glyph_t *
xcb_render_free_glyphs_glyphs (const xcb_render_free_glyphs_request_t *R);

int
xcb_render_free_glyphs_glyphs_length (const xcb_render_free_glyphs_request_t *R);

xcb_generic_iterator_t
xcb_render_free_glyphs_glyphs_end (const xcb_render_free_glyphs_request_t *R);

int
xcb_render_composite_glyphs_8_sizeof (const void  *_buffer,
                                      uint32_t     glyphcmds_len);

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
xcb_render_composite_glyphs_8_checked (xcb_connection_t        *c,
                                       uint8_t                  op,
                                       xcb_render_picture_t     src,
                                       xcb_render_picture_t     dst,
                                       xcb_render_pictformat_t  mask_format,
                                       xcb_render_glyphset_t    glyphset,
                                       int16_t                  src_x,
                                       int16_t                  src_y,
                                       uint32_t                 glyphcmds_len,
                                       const uint8_t           *glyphcmds);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_render_composite_glyphs_8 (xcb_connection_t        *c,
                               uint8_t                  op,
                               xcb_render_picture_t     src,
                               xcb_render_picture_t     dst,
                               xcb_render_pictformat_t  mask_format,
                               xcb_render_glyphset_t    glyphset,
                               int16_t                  src_x,
                               int16_t                  src_y,
                               uint32_t                 glyphcmds_len,
                               const uint8_t           *glyphcmds);

uint8_t *
xcb_render_composite_glyphs_8_glyphcmds (const xcb_render_composite_glyphs_8_request_t *R);

int
xcb_render_composite_glyphs_8_glyphcmds_length (const xcb_render_composite_glyphs_8_request_t *R);

xcb_generic_iterator_t
xcb_render_composite_glyphs_8_glyphcmds_end (const xcb_render_composite_glyphs_8_request_t *R);

int
xcb_render_composite_glyphs_16_sizeof (const void  *_buffer,
                                       uint32_t     glyphcmds_len);

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
xcb_render_composite_glyphs_16_checked (xcb_connection_t        *c,
                                        uint8_t                  op,
                                        xcb_render_picture_t     src,
                                        xcb_render_picture_t     dst,
                                        xcb_render_pictformat_t  mask_format,
                                        xcb_render_glyphset_t    glyphset,
                                        int16_t                  src_x,
                                        int16_t                  src_y,
                                        uint32_t                 glyphcmds_len,
                                        const uint8_t           *glyphcmds);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_render_composite_glyphs_16 (xcb_connection_t        *c,
                                uint8_t                  op,
                                xcb_render_picture_t     src,
                                xcb_render_picture_t     dst,
                                xcb_render_pictformat_t  mask_format,
                                xcb_render_glyphset_t    glyphset,
                                int16_t                  src_x,
                                int16_t                  src_y,
                                uint32_t                 glyphcmds_len,
                                const uint8_t           *glyphcmds);

uint8_t *
xcb_render_composite_glyphs_16_glyphcmds (const xcb_render_composite_glyphs_16_request_t *R);

int
xcb_render_composite_glyphs_16_glyphcmds_length (const xcb_render_composite_glyphs_16_request_t *R);

xcb_generic_iterator_t
xcb_render_composite_glyphs_16_glyphcmds_end (const xcb_render_composite_glyphs_16_request_t *R);

int
xcb_render_composite_glyphs_32_sizeof (const void  *_buffer,
                                       uint32_t     glyphcmds_len);

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
xcb_render_composite_glyphs_32_checked (xcb_connection_t        *c,
                                        uint8_t                  op,
                                        xcb_render_picture_t     src,
                                        xcb_render_picture_t     dst,
                                        xcb_render_pictformat_t  mask_format,
                                        xcb_render_glyphset_t    glyphset,
                                        int16_t                  src_x,
                                        int16_t                  src_y,
                                        uint32_t                 glyphcmds_len,
                                        const uint8_t           *glyphcmds);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_render_composite_glyphs_32 (xcb_connection_t        *c,
                                uint8_t                  op,
                                xcb_render_picture_t     src,
                                xcb_render_picture_t     dst,
                                xcb_render_pictformat_t  mask_format,
                                xcb_render_glyphset_t    glyphset,
                                int16_t                  src_x,
                                int16_t                  src_y,
                                uint32_t                 glyphcmds_len,
                                const uint8_t           *glyphcmds);

uint8_t *
xcb_render_composite_glyphs_32_glyphcmds (const xcb_render_composite_glyphs_32_request_t *R);

int
xcb_render_composite_glyphs_32_glyphcmds_length (const xcb_render_composite_glyphs_32_request_t *R);

xcb_generic_iterator_t
xcb_render_composite_glyphs_32_glyphcmds_end (const xcb_render_composite_glyphs_32_request_t *R);

int
xcb_render_fill_rectangles_sizeof (const void  *_buffer,
                                   uint32_t     rects_len);

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
xcb_render_fill_rectangles_checked (xcb_connection_t      *c,
                                    uint8_t                op,
                                    xcb_render_picture_t   dst,
                                    xcb_render_color_t     color,
                                    uint32_t               rects_len,
                                    const xcb_rectangle_t *rects);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_render_fill_rectangles (xcb_connection_t      *c,
                            uint8_t                op,
                            xcb_render_picture_t   dst,
                            xcb_render_color_t     color,
                            uint32_t               rects_len,
                            const xcb_rectangle_t *rects);

xcb_rectangle_t *
xcb_render_fill_rectangles_rects (const xcb_render_fill_rectangles_request_t *R);

int
xcb_render_fill_rectangles_rects_length (const xcb_render_fill_rectangles_request_t *R);

xcb_rectangle_iterator_t
xcb_render_fill_rectangles_rects_iterator (const xcb_render_fill_rectangles_request_t *R);

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
xcb_render_create_cursor_checked (xcb_connection_t     *c,
                                  xcb_cursor_t          cid,
                                  xcb_render_picture_t  source,
                                  uint16_t              x,
                                  uint16_t              y);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_render_create_cursor (xcb_connection_t     *c,
                          xcb_cursor_t          cid,
                          xcb_render_picture_t  source,
                          uint16_t              x,
                          uint16_t              y);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_render_transform_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_render_transform_t)
 */
void
xcb_render_transform_next (xcb_render_transform_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_render_transform_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_render_transform_end (xcb_render_transform_iterator_t i);

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
xcb_render_set_picture_transform_checked (xcb_connection_t       *c,
                                          xcb_render_picture_t    picture,
                                          xcb_render_transform_t  transform);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_render_set_picture_transform (xcb_connection_t       *c,
                                  xcb_render_picture_t    picture,
                                  xcb_render_transform_t  transform);

int
xcb_render_query_filters_sizeof (const void  *_buffer);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_render_query_filters_cookie_t
xcb_render_query_filters (xcb_connection_t *c,
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
xcb_render_query_filters_cookie_t
xcb_render_query_filters_unchecked (xcb_connection_t *c,
                                    xcb_drawable_t    drawable);

uint16_t *
xcb_render_query_filters_aliases (const xcb_render_query_filters_reply_t *R);

int
xcb_render_query_filters_aliases_length (const xcb_render_query_filters_reply_t *R);

xcb_generic_iterator_t
xcb_render_query_filters_aliases_end (const xcb_render_query_filters_reply_t *R);

int
xcb_render_query_filters_filters_length (const xcb_render_query_filters_reply_t *R);

xcb_str_iterator_t
xcb_render_query_filters_filters_iterator (const xcb_render_query_filters_reply_t *R);

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The xcb_generic_error_t supplied
 *
 * Returns the reply of the request asked by
 *
 * The parameter @p e supplied to this function must be NULL if
 * xcb_render_query_filters_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
xcb_render_query_filters_reply_t *
xcb_render_query_filters_reply (xcb_connection_t                   *c,
                                xcb_render_query_filters_cookie_t   cookie  /**< */,
                                xcb_generic_error_t               **e);

int
xcb_render_set_picture_filter_sizeof (const void  *_buffer,
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
xcb_render_set_picture_filter_checked (xcb_connection_t         *c,
                                       xcb_render_picture_t      picture,
                                       uint16_t                  filter_len,
                                       const char               *filter,
                                       uint32_t                  values_len,
                                       const xcb_render_fixed_t *values);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_render_set_picture_filter (xcb_connection_t         *c,
                               xcb_render_picture_t      picture,
                               uint16_t                  filter_len,
                               const char               *filter,
                               uint32_t                  values_len,
                               const xcb_render_fixed_t *values);

char *
xcb_render_set_picture_filter_filter (const xcb_render_set_picture_filter_request_t *R);

int
xcb_render_set_picture_filter_filter_length (const xcb_render_set_picture_filter_request_t *R);

xcb_generic_iterator_t
xcb_render_set_picture_filter_filter_end (const xcb_render_set_picture_filter_request_t *R);

xcb_render_fixed_t *
xcb_render_set_picture_filter_values (const xcb_render_set_picture_filter_request_t *R);

int
xcb_render_set_picture_filter_values_length (const xcb_render_set_picture_filter_request_t *R);

xcb_generic_iterator_t
xcb_render_set_picture_filter_values_end (const xcb_render_set_picture_filter_request_t *R);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_render_animcursorelt_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_render_animcursorelt_t)
 */
void
xcb_render_animcursorelt_next (xcb_render_animcursorelt_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_render_animcursorelt_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_render_animcursorelt_end (xcb_render_animcursorelt_iterator_t i);

int
xcb_render_create_anim_cursor_sizeof (const void  *_buffer,
                                      uint32_t     cursors_len);

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
xcb_render_create_anim_cursor_checked (xcb_connection_t                 *c,
                                       xcb_cursor_t                      cid,
                                       uint32_t                          cursors_len,
                                       const xcb_render_animcursorelt_t *cursors);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_render_create_anim_cursor (xcb_connection_t                 *c,
                               xcb_cursor_t                      cid,
                               uint32_t                          cursors_len,
                               const xcb_render_animcursorelt_t *cursors);

xcb_render_animcursorelt_t *
xcb_render_create_anim_cursor_cursors (const xcb_render_create_anim_cursor_request_t *R);

int
xcb_render_create_anim_cursor_cursors_length (const xcb_render_create_anim_cursor_request_t *R);

xcb_render_animcursorelt_iterator_t
xcb_render_create_anim_cursor_cursors_iterator (const xcb_render_create_anim_cursor_request_t *R);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_render_spanfix_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_render_spanfix_t)
 */
void
xcb_render_spanfix_next (xcb_render_spanfix_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_render_spanfix_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_render_spanfix_end (xcb_render_spanfix_iterator_t i);

/**
 * Get the next element of the iterator
 * @param i Pointer to a xcb_render_trap_iterator_t
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(xcb_render_trap_t)
 */
void
xcb_render_trap_next (xcb_render_trap_iterator_t *i);

/**
 * Return the iterator pointing to the last element
 * @param i An xcb_render_trap_iterator_t
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
xcb_generic_iterator_t
xcb_render_trap_end (xcb_render_trap_iterator_t i);

int
xcb_render_add_traps_sizeof (const void  *_buffer,
                             uint32_t     traps_len);

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
xcb_render_add_traps_checked (xcb_connection_t        *c,
                              xcb_render_picture_t     picture,
                              int16_t                  x_off,
                              int16_t                  y_off,
                              uint32_t                 traps_len,
                              const xcb_render_trap_t *traps);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_render_add_traps (xcb_connection_t        *c,
                      xcb_render_picture_t     picture,
                      int16_t                  x_off,
                      int16_t                  y_off,
                      uint32_t                 traps_len,
                      const xcb_render_trap_t *traps);

xcb_render_trap_t *
xcb_render_add_traps_traps (const xcb_render_add_traps_request_t *R);

int
xcb_render_add_traps_traps_length (const xcb_render_add_traps_request_t *R);

xcb_render_trap_iterator_t
xcb_render_add_traps_traps_iterator (const xcb_render_add_traps_request_t *R);

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
xcb_render_create_solid_fill_checked (xcb_connection_t     *c,
                                      xcb_render_picture_t  picture,
                                      xcb_render_color_t    color);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_render_create_solid_fill (xcb_connection_t     *c,
                              xcb_render_picture_t  picture,
                              xcb_render_color_t    color);

int
xcb_render_create_linear_gradient_sizeof (const void  *_buffer);

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
xcb_render_create_linear_gradient_checked (xcb_connection_t         *c,
                                           xcb_render_picture_t      picture,
                                           xcb_render_pointfix_t     p1,
                                           xcb_render_pointfix_t     p2,
                                           uint32_t                  num_stops,
                                           const xcb_render_fixed_t *stops,
                                           const xcb_render_color_t *colors);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_render_create_linear_gradient (xcb_connection_t         *c,
                                   xcb_render_picture_t      picture,
                                   xcb_render_pointfix_t     p1,
                                   xcb_render_pointfix_t     p2,
                                   uint32_t                  num_stops,
                                   const xcb_render_fixed_t *stops,
                                   const xcb_render_color_t *colors);

xcb_render_fixed_t *
xcb_render_create_linear_gradient_stops (const xcb_render_create_linear_gradient_request_t *R);

int
xcb_render_create_linear_gradient_stops_length (const xcb_render_create_linear_gradient_request_t *R);

xcb_generic_iterator_t
xcb_render_create_linear_gradient_stops_end (const xcb_render_create_linear_gradient_request_t *R);

xcb_render_color_t *
xcb_render_create_linear_gradient_colors (const xcb_render_create_linear_gradient_request_t *R);

int
xcb_render_create_linear_gradient_colors_length (const xcb_render_create_linear_gradient_request_t *R);

xcb_render_color_iterator_t
xcb_render_create_linear_gradient_colors_iterator (const xcb_render_create_linear_gradient_request_t *R);

int
xcb_render_create_radial_gradient_sizeof (const void  *_buffer);

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
xcb_render_create_radial_gradient_checked (xcb_connection_t         *c,
                                           xcb_render_picture_t      picture,
                                           xcb_render_pointfix_t     inner,
                                           xcb_render_pointfix_t     outer,
                                           xcb_render_fixed_t        inner_radius,
                                           xcb_render_fixed_t        outer_radius,
                                           uint32_t                  num_stops,
                                           const xcb_render_fixed_t *stops,
                                           const xcb_render_color_t *colors);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_render_create_radial_gradient (xcb_connection_t         *c,
                                   xcb_render_picture_t      picture,
                                   xcb_render_pointfix_t     inner,
                                   xcb_render_pointfix_t     outer,
                                   xcb_render_fixed_t        inner_radius,
                                   xcb_render_fixed_t        outer_radius,
                                   uint32_t                  num_stops,
                                   const xcb_render_fixed_t *stops,
                                   const xcb_render_color_t *colors);

xcb_render_fixed_t *
xcb_render_create_radial_gradient_stops (const xcb_render_create_radial_gradient_request_t *R);

int
xcb_render_create_radial_gradient_stops_length (const xcb_render_create_radial_gradient_request_t *R);

xcb_generic_iterator_t
xcb_render_create_radial_gradient_stops_end (const xcb_render_create_radial_gradient_request_t *R);

xcb_render_color_t *
xcb_render_create_radial_gradient_colors (const xcb_render_create_radial_gradient_request_t *R);

int
xcb_render_create_radial_gradient_colors_length (const xcb_render_create_radial_gradient_request_t *R);

xcb_render_color_iterator_t
xcb_render_create_radial_gradient_colors_iterator (const xcb_render_create_radial_gradient_request_t *R);

int
xcb_render_create_conical_gradient_sizeof (const void  *_buffer);

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
xcb_render_create_conical_gradient_checked (xcb_connection_t         *c,
                                            xcb_render_picture_t      picture,
                                            xcb_render_pointfix_t     center,
                                            xcb_render_fixed_t        angle,
                                            uint32_t                  num_stops,
                                            const xcb_render_fixed_t *stops,
                                            const xcb_render_color_t *colors);

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 *
 */
xcb_void_cookie_t
xcb_render_create_conical_gradient (xcb_connection_t         *c,
                                    xcb_render_picture_t      picture,
                                    xcb_render_pointfix_t     center,
                                    xcb_render_fixed_t        angle,
                                    uint32_t                  num_stops,
                                    const xcb_render_fixed_t *stops,
                                    const xcb_render_color_t *colors);

xcb_render_fixed_t *
xcb_render_create_conical_gradient_stops (const xcb_render_create_conical_gradient_request_t *R);

int
xcb_render_create_conical_gradient_stops_length (const xcb_render_create_conical_gradient_request_t *R);

xcb_generic_iterator_t
xcb_render_create_conical_gradient_stops_end (const xcb_render_create_conical_gradient_request_t *R);

xcb_render_color_t *
xcb_render_create_conical_gradient_colors (const xcb_render_create_conical_gradient_request_t *R);

int
xcb_render_create_conical_gradient_colors_length (const xcb_render_create_conical_gradient_request_t *R);

xcb_render_color_iterator_t
xcb_render_create_conical_gradient_colors_iterator (const xcb_render_create_conical_gradient_request_t *R);


#ifdef __cplusplus
}
#endif

#endif

/**
 * @}
 */
