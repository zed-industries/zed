/*
 * gdkscreen.h
 *
 * Copyright 2001 Sun Microsystems Inc.
 *
 * Erwann Chenede <erwann.chenede@sun.com>
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
 * License along with this library. If not, see <http://www.gnu.org/licenses/>.
 */

#ifndef __GDK_SCREEN_H__
#define __GDK_SCREEN_H__

#if !defined (__GDK_H_INSIDE__) && !defined (GDK_COMPILATION)
#error "Only <gdk/gdk.h> can be included directly."
#endif

#include <cairo.h>
#include <gdk/gdkversionmacros.h>
#include <gdk/gdktypes.h>
#include <gdk/gdkdisplay.h>

G_BEGIN_DECLS

#define GDK_TYPE_SCREEN            (gdk_screen_get_type ())
#define GDK_SCREEN(object)         (G_TYPE_CHECK_INSTANCE_CAST ((object), GDK_TYPE_SCREEN, GdkScreen))
#define GDK_IS_SCREEN(object)      (G_TYPE_CHECK_INSTANCE_TYPE ((object), GDK_TYPE_SCREEN))


GDK_AVAILABLE_IN_ALL
GType        gdk_screen_get_type              (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
GdkVisual *  gdk_screen_get_system_visual     (GdkScreen   *screen);
GDK_AVAILABLE_IN_ALL
GdkVisual *  gdk_screen_get_rgba_visual       (GdkScreen   *screen);
GDK_AVAILABLE_IN_ALL
gboolean     gdk_screen_is_composited         (GdkScreen   *screen);

GDK_AVAILABLE_IN_ALL
GdkWindow *  gdk_screen_get_root_window       (GdkScreen   *screen);
GDK_AVAILABLE_IN_ALL
GdkDisplay * gdk_screen_get_display           (GdkScreen   *screen);
GDK_DEPRECATED_IN_3_22
gint         gdk_screen_get_number            (GdkScreen   *screen);
GDK_DEPRECATED_IN_3_22
gint         gdk_screen_get_width             (GdkScreen   *screen);
GDK_DEPRECATED_IN_3_22
gint         gdk_screen_get_height            (GdkScreen   *screen);
GDK_DEPRECATED_IN_3_22
gint         gdk_screen_get_width_mm          (GdkScreen   *screen);
GDK_DEPRECATED_IN_3_22
gint         gdk_screen_get_height_mm         (GdkScreen   *screen);

GDK_AVAILABLE_IN_ALL
GList *      gdk_screen_list_visuals          (GdkScreen   *screen);
GDK_AVAILABLE_IN_ALL
GList *      gdk_screen_get_toplevel_windows  (GdkScreen   *screen);
GDK_DEPRECATED_IN_3_22
gchar *      gdk_screen_make_display_name     (GdkScreen   *screen);

GDK_DEPRECATED_IN_3_22_FOR(gdk_display_get_n_monitors)
gint         gdk_screen_get_n_monitors        (GdkScreen    *screen);
GDK_DEPRECATED_IN_3_22_FOR(gdk_display_get_primary_monitor)
gint         gdk_screen_get_primary_monitor   (GdkScreen    *screen);
GDK_DEPRECATED_IN_3_22_FOR(gdk_monitor_get_geometry)
void         gdk_screen_get_monitor_geometry  (GdkScreen    *screen,
                                               gint          monitor_num,
                                               GdkRectangle *dest);
GDK_DEPRECATED_IN_3_22_FOR(gdk_monitor_get_workarea)
void         gdk_screen_get_monitor_workarea  (GdkScreen    *screen,
                                               gint          monitor_num,
                                               GdkRectangle *dest);

GDK_DEPRECATED_IN_3_22_FOR(gdk_display_get_monitor_at_point)
gint          gdk_screen_get_monitor_at_point  (GdkScreen *screen,
                                                gint       x,
                                                gint       y);
GDK_DEPRECATED_IN_3_22_FOR(gdk_display_get_monitor_at_window)
gint          gdk_screen_get_monitor_at_window (GdkScreen *screen,
                                                GdkWindow *window);
GDK_DEPRECATED_IN_3_22_FOR(gdk_monitor_get_width_mm)
gint          gdk_screen_get_monitor_width_mm  (GdkScreen *screen,
                                                gint       monitor_num);
GDK_DEPRECATED_IN_3_22_FOR(gdk_monitor_get_height_mm)
gint          gdk_screen_get_monitor_height_mm (GdkScreen *screen,
                                                gint       monitor_num);
GDK_DEPRECATED_IN_3_22_FOR(gdk_monitor_get_model)
gchar *       gdk_screen_get_monitor_plug_name (GdkScreen *screen,
                                                gint       monitor_num);
GDK_DEPRECATED_IN_3_22_FOR(gdk_monitor_get_scale_factor)
gint          gdk_screen_get_monitor_scale_factor (GdkScreen *screen,
                                                   gint       monitor_num);

GDK_AVAILABLE_IN_ALL
GdkScreen *gdk_screen_get_default (void);

GDK_AVAILABLE_IN_ALL
gboolean   gdk_screen_get_setting (GdkScreen   *screen,
                                   const gchar *name,
                                   GValue      *value);

GDK_AVAILABLE_IN_ALL
void                        gdk_screen_set_font_options (GdkScreen                  *screen,
                                                         const cairo_font_options_t *options);
GDK_AVAILABLE_IN_ALL
const cairo_font_options_t *gdk_screen_get_font_options (GdkScreen                  *screen);

GDK_AVAILABLE_IN_ALL
void    gdk_screen_set_resolution (GdkScreen *screen,
                                   gdouble    dpi);
GDK_AVAILABLE_IN_ALL
gdouble gdk_screen_get_resolution (GdkScreen *screen);

GDK_DEPRECATED_IN_3_22
GdkWindow *gdk_screen_get_active_window (GdkScreen *screen);
GDK_AVAILABLE_IN_ALL
GList     *gdk_screen_get_window_stack  (GdkScreen *screen);

G_END_DECLS

#endif  /* __GDK_SCREEN_H__ */
