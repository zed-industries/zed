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
 * License along with this library; if not, write to the
 * Free Software Foundation, Inc., 59 Temple Place - Suite 330,
 * Boston, MA 02111-1307, USA.
 */

#ifndef __GDK_SCREEN_H__
#define __GDK_SCREEN_H__

#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GDK_H_INSIDE__) && !defined (GDK_COMPILATION)
#error "Only <gdk/gdk.h> can be included directly."
#endif

#include <cairo.h>
#include "gdk/gdktypes.h"
#include "gdk/gdkdisplay.h"

G_BEGIN_DECLS

typedef struct _GdkScreenClass GdkScreenClass;

#define GDK_TYPE_SCREEN            (gdk_screen_get_type ())
#define GDK_SCREEN(object)         (G_TYPE_CHECK_INSTANCE_CAST ((object), GDK_TYPE_SCREEN, GdkScreen))
#define GDK_SCREEN_CLASS(klass)    (G_TYPE_CHECK_CLASS_CAST ((klass), GDK_TYPE_SCREEN, GdkScreenClass))
#define GDK_IS_SCREEN(object)      (G_TYPE_CHECK_INSTANCE_TYPE ((object), GDK_TYPE_SCREEN))
#define GDK_IS_SCREEN_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE ((klass), GDK_TYPE_SCREEN))
#define GDK_SCREEN_GET_CLASS(obj)  (G_TYPE_INSTANCE_GET_CLASS ((obj), GDK_TYPE_SCREEN, GdkScreenClass))

struct _GdkScreen
{
  GObject parent_instance;

  guint GSEAL (closed) : 1;

  GdkGC *GSEAL (normal_gcs[32]);
  GdkGC *GSEAL (exposure_gcs[32]);
  GdkGC *GSEAL (subwindow_gcs[32]);

  cairo_font_options_t *GSEAL (font_options);
  double GSEAL (resolution);	/* pixels/points scale factor for fonts */
};

struct _GdkScreenClass
{
  GObjectClass parent_class;

  void (*size_changed) (GdkScreen *screen);
  void (*composited_changed) (GdkScreen *screen);
  void (*monitors_changed) (GdkScreen *screen);
};

GType        gdk_screen_get_type              (void) G_GNUC_CONST;
GdkColormap *gdk_screen_get_default_colormap  (GdkScreen   *screen);
void         gdk_screen_set_default_colormap  (GdkScreen   *screen,
					       GdkColormap *colormap);
GdkColormap* gdk_screen_get_system_colormap   (GdkScreen   *screen);
GdkVisual*   gdk_screen_get_system_visual     (GdkScreen   *screen);
GdkColormap *gdk_screen_get_rgb_colormap      (GdkScreen   *screen);
GdkVisual *  gdk_screen_get_rgb_visual        (GdkScreen   *screen);
GdkColormap *gdk_screen_get_rgba_colormap     (GdkScreen   *screen);
GdkVisual *  gdk_screen_get_rgba_visual       (GdkScreen   *screen);
gboolean     gdk_screen_is_composited	      (GdkScreen   *screen);

GdkWindow *  gdk_screen_get_root_window       (GdkScreen   *screen);
GdkDisplay * gdk_screen_get_display           (GdkScreen   *screen);
gint         gdk_screen_get_number            (GdkScreen   *screen);
gint         gdk_screen_get_width             (GdkScreen   *screen);
gint         gdk_screen_get_height            (GdkScreen   *screen);
gint         gdk_screen_get_width_mm          (GdkScreen   *screen);
gint         gdk_screen_get_height_mm         (GdkScreen   *screen);

GList *      gdk_screen_list_visuals          (GdkScreen   *screen);
GList *      gdk_screen_get_toplevel_windows  (GdkScreen   *screen);
gchar *      gdk_screen_make_display_name     (GdkScreen   *screen);

gint          gdk_screen_get_n_monitors        (GdkScreen *screen);
gint          gdk_screen_get_primary_monitor   (GdkScreen *screen);
void          gdk_screen_get_monitor_geometry  (GdkScreen *screen,
						gint       monitor_num,
						GdkRectangle *dest);
gint          gdk_screen_get_monitor_at_point  (GdkScreen *screen,
						gint       x,
						gint       y);
gint          gdk_screen_get_monitor_at_window (GdkScreen *screen,
						GdkWindow *window);
gint          gdk_screen_get_monitor_width_mm  (GdkScreen *screen,
                                                gint       monitor_num);
gint          gdk_screen_get_monitor_height_mm (GdkScreen *screen,
                                                gint       monitor_num);
gchar *       gdk_screen_get_monitor_plug_name (GdkScreen *screen,
                                                gint       monitor_num);

void          gdk_screen_broadcast_client_message  (GdkScreen       *screen,
						    GdkEvent        *event);

GdkScreen *gdk_screen_get_default (void);

gboolean   gdk_screen_get_setting (GdkScreen   *screen,
				   const gchar *name,
				   GValue      *value);

void                        gdk_screen_set_font_options (GdkScreen                  *screen,
							 const cairo_font_options_t *options);
const cairo_font_options_t *gdk_screen_get_font_options (GdkScreen                  *screen);

void    gdk_screen_set_resolution (GdkScreen *screen,
				   gdouble    dpi);
gdouble gdk_screen_get_resolution (GdkScreen *screen);

GdkWindow *gdk_screen_get_active_window (GdkScreen *screen);
GList     *gdk_screen_get_window_stack  (GdkScreen *screen);

G_END_DECLS

#endif				/* __GDK_SCREEN_H__ */
