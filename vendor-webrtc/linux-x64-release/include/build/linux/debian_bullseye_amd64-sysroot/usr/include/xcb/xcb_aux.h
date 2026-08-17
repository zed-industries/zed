#ifndef __XCB_AUX_H__
#define __XCB_AUX_H__


#ifdef __cplusplus
extern "C" {
#endif


uint8_t          xcb_aux_get_depth       (xcb_connection_t *c,
                                          xcb_screen_t     *screen);

uint8_t xcb_aux_get_depth_of_visual      (xcb_screen_t *screen,
					  xcb_visualid_t id);

xcb_screen_t     *xcb_aux_get_screen     (xcb_connection_t *c,
                                          int               screen);

xcb_visualtype_t *xcb_aux_get_visualtype (xcb_connection_t *c,
                                          int               screen,
                                          xcb_visualid_t    vid);

xcb_visualtype_t *
xcb_aux_find_visual_by_id (xcb_screen_t *screen,
			   xcb_visualid_t id);

xcb_visualtype_t *
xcb_aux_find_visual_by_attrs (xcb_screen_t *screen,
			      int8_t class_,
			      int8_t depth);

void           xcb_aux_sync              (xcb_connection_t *c);

/* internal helper macro for XCB_AUX_ADD_PARAM
It gives the offset of the field 'param' in the structure pointed to by
'paramsp' in multiples of an uint32_t's size. */
#define XCB_AUX_INTERNAL_OFFSETOF(paramsp, param) \
    ((uint32_t const*)(&((paramsp)->param))-(uint32_t const*)(paramsp))

/* add an optional parameter to an xcb_params_* structure
parameters:
    maskp: pointer to bitmask whos bits mark used parameters
    paramsp: pointer to structure with parameters
    param: parameter to set
    value: value to set the parameter to
*/
#define XCB_AUX_ADD_PARAM(maskp, paramsp, param, value) \
    ((*(maskp)|=1<<XCB_AUX_INTERNAL_OFFSETOF((paramsp),param)), \
     ((paramsp)->param=(value)))

typedef struct {
    uint32_t back_pixmap;
    uint32_t back_pixel;
    uint32_t border_pixmap;
    uint32_t border_pixel;
    uint32_t bit_gravity;
    uint32_t win_gravity;
    uint32_t backing_store;
    uint32_t backing_planes;
    uint32_t backing_pixel;
    uint32_t override_redirect;
    uint32_t save_under;
    uint32_t event_mask;
    uint32_t dont_propagate;
    uint32_t colormap;
    uint32_t cursor;
} xcb_params_cw_t;

xcb_void_cookie_t
xcb_aux_create_window (xcb_connection_t      *c,
                       uint8_t                depth,
                       xcb_window_t           wid,
                       xcb_window_t           parent,
                       int16_t                x,
                       int16_t                y,
                       uint16_t               width,
                       uint16_t               height,
                       uint16_t               border_width,
                       uint16_t               class_,
                       xcb_visualid_t         visual,
                       uint32_t               mask,
                       const xcb_params_cw_t *params);

xcb_void_cookie_t
xcb_aux_create_window_checked (xcb_connection_t       *c,
			       uint8_t                depth,
			       xcb_window_t           wid,
			       xcb_window_t           parent,
			       int16_t                x,
			       int16_t                y,
			       uint16_t               width,
			       uint16_t               height,
			       uint16_t               border_width,
			       uint16_t               class_,
			       xcb_visualid_t         visual,
			       uint32_t               mask,
			       const xcb_params_cw_t *params);

xcb_void_cookie_t
xcb_aux_change_window_attributes (xcb_connection_t      *c,
                                  xcb_window_t           window,
                                  uint32_t               mask,
                                  const xcb_params_cw_t *params);

xcb_void_cookie_t
xcb_aux_change_window_attributes_checked (xcb_connection_t      *c,
                                          xcb_window_t           window,
                                          uint32_t               mask,
                                          const xcb_params_cw_t *params);

typedef struct {
    int32_t  x;
    int32_t  y;
    uint32_t width;
    uint32_t height;
    uint32_t border_width;
    uint32_t sibling;
    uint32_t stack_mode;
} xcb_params_configure_window_t;

xcb_void_cookie_t
xcb_aux_configure_window (xcb_connection_t                    *c,
                          xcb_window_t                         window,
                          uint16_t                             mask,
                          const xcb_params_configure_window_t *params);

typedef struct {
    uint32_t function;
    uint32_t plane_mask;
    uint32_t foreground;
    uint32_t background;
    uint32_t line_width;
    uint32_t line_style;
    uint32_t cap_style;
    uint32_t join_style;
    uint32_t fill_style;
    uint32_t fill_rule;
    uint32_t tile;
    uint32_t stipple;
    uint32_t tile_stipple_origin_x;
    uint32_t tile_stipple_origin_y;
    uint32_t font;
    uint32_t subwindow_mode;
    uint32_t graphics_exposures;
    uint32_t clip_originX;
    uint32_t clip_originY;
    uint32_t mask;
    uint32_t dash_offset;
    uint32_t dash_list;
    uint32_t arc_mode;
} xcb_params_gc_t;

xcb_void_cookie_t
xcb_aux_create_gc (xcb_connection_t      *c,
                   xcb_gcontext_t         cid,
                   xcb_drawable_t         drawable,
                   uint32_t               mask,
                   const xcb_params_gc_t *params);

xcb_void_cookie_t
xcb_aux_create_gc_checked (xcb_connection_t      *c,
			   xcb_gcontext_t         gid,
			   xcb_drawable_t         drawable,
			   uint32_t               mask,
			   const xcb_params_gc_t *params);
xcb_void_cookie_t
xcb_aux_change_gc (xcb_connection_t      *c,
                   xcb_gcontext_t         gc,
                   uint32_t               mask,
                   const xcb_params_gc_t *params);

xcb_void_cookie_t
xcb_aux_change_gc_checked (xcb_connection_t     *c,
			   xcb_gcontext_t        gc,
			   uint32_t              mask,
			   const xcb_params_gc_t *params);
typedef struct {
    uint32_t key_click_percent;
    uint32_t bell_percent;
    uint32_t bell_pitch;
    uint32_t bell_duration;
    uint32_t led;
    uint32_t led_mode;
    uint32_t key;
    uint32_t auto_repeat_mode;
} xcb_params_keyboard_t;

xcb_void_cookie_t
xcb_aux_change_keyboard_control (xcb_connection_t            *c,
                                 uint32_t                     mask,
                                 const xcb_params_keyboard_t *params);

int
xcb_aux_parse_color(const char *color_name,
		    uint16_t *red,  uint16_t *green,  uint16_t *blue);

xcb_void_cookie_t
xcb_aux_set_line_attributes_checked (xcb_connection_t *dpy,
				     xcb_gcontext_t gc,
				     uint16_t linewidth,
				     int32_t linestyle,
				     int32_t capstyle,
				     int32_t joinstyle);

xcb_void_cookie_t
xcb_aux_clear_window(xcb_connection_t *  dpy,
		     xcb_window_t        w);

#ifdef __cplusplus
}
#endif


#endif /* __XCB_AUX_H__ */
