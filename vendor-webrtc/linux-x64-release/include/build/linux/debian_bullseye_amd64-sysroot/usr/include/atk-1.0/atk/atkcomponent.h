/* ATK -  Accessibility Toolkit
 * Copyright 2001 Sun Microsystems Inc.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public
 * License along with this library; if not, write to the
 * Free Software Foundation, Inc., 59 Temple Place - Suite 330,
 * Boston, MA 02111-1307, USA.
 */

#ifndef __ATK_COMPONENT_H__
#define __ATK_COMPONENT_H__

#if defined(ATK_DISABLE_SINGLE_INCLUDES) && !defined (__ATK_H_INSIDE__) && !defined (ATK_COMPILATION)
#error "Only <atk/atk.h> can be included directly."
#endif

#include <atk/atkobject.h>
#include <atk/atkutil.h>

G_BEGIN_DECLS

/**
 * AtkScrollType:
 * @ATK_SCROLL_TOP_LEFT: Scroll the object vertically and horizontally to bring
 *   its top left corner to the top left corner of the window.
 * @ATK_SCROLL_BOTTOM_RIGHT: Scroll the object vertically and horizontally to
 *   bring its bottom right corner to the bottom right corner of the window.
 * @ATK_SCROLL_TOP_EDGE: Scroll the object vertically to bring its top edge to
 *   the top edge of the window.
 * @ATK_SCROLL_BOTTOM_EDGE: Scroll the object vertically to bring its bottom
 *   edge to the bottom edge of the window.
 * @ATK_SCROLL_LEFT_EDGE: Scroll the object vertically and horizontally to bring
 *   its left edge to the left edge of the window.
 * @ATK_SCROLL_RIGHT_EDGE: Scroll the object vertically and horizontally to
 *   bring its right edge to the right edge of the window.
 * @ATK_SCROLL_ANYWHERE: Scroll the object vertically and horizontally so that
 *   as much as possible of the object becomes visible. The exact placement is
 *   determined by the application.
 *
 * Specifies where an object should be placed on the screen when using scroll_to.
 *
 * Since: 2.30
 */
typedef enum {
  ATK_SCROLL_TOP_LEFT,
  ATK_SCROLL_BOTTOM_RIGHT,
  ATK_SCROLL_TOP_EDGE,
  ATK_SCROLL_BOTTOM_EDGE,
  ATK_SCROLL_LEFT_EDGE,
  ATK_SCROLL_RIGHT_EDGE,
  ATK_SCROLL_ANYWHERE
} AtkScrollType;

#define ATK_TYPE_COMPONENT                    (atk_component_get_type ())
#define ATK_IS_COMPONENT(obj)                 G_TYPE_CHECK_INSTANCE_TYPE ((obj), ATK_TYPE_COMPONENT)
#define ATK_COMPONENT(obj)                    G_TYPE_CHECK_INSTANCE_CAST ((obj), ATK_TYPE_COMPONENT, AtkComponent)
#define ATK_COMPONENT_GET_IFACE(obj)          (G_TYPE_INSTANCE_GET_INTERFACE ((obj), ATK_TYPE_COMPONENT, AtkComponentIface))

#ifndef _TYPEDEF_ATK_COMPONENT_
#define _TYPEDEF_ATK_COMPONENT_
typedef struct _AtkComponent AtkComponent;
#endif
typedef struct _AtkComponentIface  AtkComponentIface;

/**
 * AtkFocusHandler:
 * @object: the #AtkObject that receives/lose the focus
 * @focus_in: TRUE if the object receives the focus
 *
 * The type of callback function used for
 * atk_component_add_focus_handler() and
 * atk_component_remove_focus_handler()
 *
 * Deprecated: 2.9.4: Deprecated with atk_component_add_focus_handler()
 * and atk_component_remove_focus_handler(). See those
 * methods for more information.
 */
typedef void (*AtkFocusHandler) (AtkObject* object, gboolean focus_in);

typedef struct _AtkRectangle       AtkRectangle;

/**
 * AtkRectangle:
 * @x: X coordinate of the left side of the rectangle.
 * @y: Y coordinate of the top side of the rectangle.
 * @width: width of the rectangle.
 * @height: height of the rectangle.
 *
 * A data structure for holding a rectangle. Those coordinates are
 * relative to the component top-level parent.
 */
struct _AtkRectangle
{
  gint x;
  gint y;
  gint width;
  gint height;
};

ATK_AVAILABLE_IN_ALL
GType atk_rectangle_get_type (void);

#define ATK_TYPE_RECTANGLE (atk_rectangle_get_type ())

/**
 * AtkComponentIface:
 * @add_focus_handler: This virtual function is deprecated since 2.9.4
 *   and it should not be overriden. See atk_component_add_focus_handler()
 *   for more information.
 * @get_position: This virtual function is deprecated since 2.12 and
 *   it should not be overriden. Use @AtkComponentIface.get_extents instead.
 * @get_size: This virtual function is deprecated since 2.12 and it
 *   should not be overriden. Use @AtkComponentIface.get_extents instead.
 * @remove_focus_handler: This virtual function is deprecated since
 *   2.9.4 and it should not be overriden. See atk_component_remove_focus_handler()
 *   for more information.
 * @contains:
 * @ref_accessible_at_point:
 * @get_extents:
 * @grab_focus:
 * @set_extents:
 * @set_position:
 * @set_size:
 * @get_layer:
 * @get_mdi_zorder:
 * @bounds_changed:
 * @get_alpha:
 * @scroll_to:
 * @scroll_to_point:
 *
 * The AtkComponent interface should be supported by any object that is
 * rendered on the screen. The interface provides the standard mechanism
 * for an assistive technology to determine and set the graphical
 * representation of an object.
 */
struct _AtkComponentIface
{
  /*< private >*/
  GTypeInterface parent;

  /*< public >*/
  guint          (* add_focus_handler)  (AtkComponent          *component,
                                         AtkFocusHandler        handler);

  gboolean       (* contains)           (AtkComponent          *component,
                                         gint                   x,
                                         gint                   y,
                                         AtkCoordType           coord_type);

  AtkObject*    (* ref_accessible_at_point)  (AtkComponent     *component,
                                         gint                   x,
                                         gint                   y,
                                         AtkCoordType           coord_type);
  void          (* get_extents)         (AtkComponent          *component,
                                         gint                  *x,
                                         gint                  *y,
                                         gint                  *width,
                                         gint                  *height,
                                         AtkCoordType          coord_type);
  void                     (* get_position)     (AtkComponent   *component,
                                                 gint           *x,
                                                 gint           *y,
                                                 AtkCoordType   coord_type);
  void                     (* get_size)                 (AtkComponent   *component,
                                                         gint           *width,
                                                         gint           *height);
  gboolean                 (* grab_focus)               (AtkComponent   *component);
  void                     (* remove_focus_handler)      (AtkComponent  *component,
                                                          guint         handler_id);
  gboolean                 (* set_extents)      (AtkComponent   *component,
                                                 gint           x,
                                                 gint           y,
                                                 gint           width,
                                                 gint           height,
                                                 AtkCoordType   coord_type);
  gboolean                 (* set_position)     (AtkComponent   *component,
                                                 gint           x,
                                                 gint           y,
                                                 AtkCoordType   coord_type);
  gboolean                 (* set_size)         (AtkComponent   *component,
                                                 gint           width,
                                                 gint           height);
  	
  AtkLayer                 (* get_layer)        (AtkComponent   *component);
  gint                     (* get_mdi_zorder)   (AtkComponent   *component);

  /*
   * signal handlers
   */
  void                     (* bounds_changed)   (AtkComponent   *component,
                                                 AtkRectangle   *bounds);
  gdouble                  (* get_alpha)        (AtkComponent   *component);

  /*
   * Scrolls this object so it becomes visible on the screen.
   *
   * scroll_to lets the implementation compute an appropriate target
   * position on the screen, with type used as a positioning hint.
   *
   * scroll_to_point lets the client specify a precise target position
   * on the screen for the top-left of the object.
   *
   * Since ATK 2.30
   */
  gboolean                (*scroll_to)          (AtkComponent   *component,
                                                 AtkScrollType   type);

  gboolean                (*scroll_to_point)    (AtkComponent   *component,
                                                 AtkCoordType    coords,
                                                 gint            x,
                                                 gint            y);
};

ATK_AVAILABLE_IN_ALL
GType atk_component_get_type (void);

/* convenience functions */
ATK_DEPRECATED_IN_2_10
guint                atk_component_add_focus_handler      (AtkComponent    *component,
                                                           AtkFocusHandler handler);
ATK_AVAILABLE_IN_ALL
gboolean              atk_component_contains               (AtkComponent    *component,
                                                            gint            x,
                                                            gint            y,
                                                            AtkCoordType    coord_type);
ATK_AVAILABLE_IN_ALL
AtkObject*            atk_component_ref_accessible_at_point(AtkComponent    *component,
                                                            gint            x,
                                                            gint            y,
                                                            AtkCoordType    coord_type);
ATK_AVAILABLE_IN_ALL
void                  atk_component_get_extents            (AtkComponent    *component,
                                                            gint            *x,
                                                            gint            *y,
                                                            gint            *width,
                                                            gint            *height,
                                                            AtkCoordType    coord_type);
ATK_DEPRECATED_IN_2_12_FOR(atk_component_get_extents)
void                  atk_component_get_position           (AtkComponent    *component,
                                                            gint            *x,
                                                            gint            *y,
                                                            AtkCoordType    coord_type);
ATK_DEPRECATED_IN_2_12_FOR(atk_component_get_extents)
void                  atk_component_get_size               (AtkComponent    *component,
                                                            gint            *width,
                                                            gint            *height);
ATK_AVAILABLE_IN_ALL
AtkLayer              atk_component_get_layer              (AtkComponent    *component);
ATK_AVAILABLE_IN_ALL
gint                  atk_component_get_mdi_zorder         (AtkComponent    *component);
ATK_AVAILABLE_IN_ALL
gboolean              atk_component_grab_focus             (AtkComponent    *component);
ATK_DEPRECATED_IN_2_10
void                  atk_component_remove_focus_handler   (AtkComponent    *component,
                                                            guint           handler_id);
ATK_AVAILABLE_IN_ALL
gboolean              atk_component_set_extents            (AtkComponent    *component,
                                                            gint            x,
                                                            gint            y,
                                                            gint            width,
                                                            gint            height,
                                                            AtkCoordType    coord_type);
ATK_AVAILABLE_IN_ALL
gboolean              atk_component_set_position           (AtkComponent    *component,
                                                            gint            x,
                                                            gint            y,
                                                            AtkCoordType    coord_type);
ATK_AVAILABLE_IN_ALL
gboolean              atk_component_set_size               (AtkComponent    *component,
                                                            gint            width,
                                                            gint            height);
ATK_AVAILABLE_IN_ALL
gdouble               atk_component_get_alpha              (AtkComponent    *component);

ATK_AVAILABLE_IN_2_30
gboolean              atk_component_scroll_to              (AtkComponent    *component,
                                                            AtkScrollType   type);

ATK_AVAILABLE_IN_2_30
gboolean              atk_component_scroll_to_point        (AtkComponent    *component,
                                                            AtkCoordType    coords,
                                                            gint            x,
                                                            gint            y);

G_END_DECLS

#endif /* __ATK_COMPONENT_H__ */
