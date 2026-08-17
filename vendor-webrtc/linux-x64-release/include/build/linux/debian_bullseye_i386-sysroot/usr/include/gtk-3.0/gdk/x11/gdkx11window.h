/* GDK - The GIMP Drawing Kit
 * Copyright (C) 1995-1997 Peter Mattis, Spencer Kimball and Josh MacDonald
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library. If not, see <http://www.gnu.org/licenses/>.
 */

/*
 * Modified by the GTK+ Team and others 1997-2000.  See the AUTHORS
 * file for a list of people on the GTK+ Team.  See the ChangeLog
 * files for a list of changes.  These files are distributed with
 * GTK+ at ftp://ftp.gtk.org/pub/gtk/. 
 */

#ifndef __GDK_X11_WINDOW_H__
#define __GDK_X11_WINDOW_H__

#if !defined (__GDKX_H_INSIDE__) && !defined (GDK_COMPILATION)
#error "Only <gdk/gdkx.h> can be included directly."
#endif

#include <gdk/gdk.h>

#include <X11/Xlib.h>
#include <X11/Xutil.h>

G_BEGIN_DECLS

#define GDK_TYPE_X11_WINDOW              (gdk_x11_window_get_type ())
#define GDK_X11_WINDOW(object)           (G_TYPE_CHECK_INSTANCE_CAST ((object), GDK_TYPE_X11_WINDOW, GdkX11Window))
#define GDK_X11_WINDOW_CLASS(klass)      (G_TYPE_CHECK_CLASS_CAST ((klass), GDK_TYPE_X11_WINDOW, GdkX11WindowClass))
#define GDK_IS_X11_WINDOW(object)        (G_TYPE_CHECK_INSTANCE_TYPE ((object), GDK_TYPE_X11_WINDOW))
#define GDK_IS_X11_WINDOW_CLASS(klass)   (G_TYPE_CHECK_CLASS_TYPE ((klass), GDK_TYPE_X11_WINDOW))
#define GDK_X11_WINDOW_GET_CLASS(obj)    (G_TYPE_INSTANCE_GET_CLASS ((obj), GDK_TYPE_X11_WINDOW, GdkX11WindowClass))

#ifdef GDK_COMPILATION
typedef struct _GdkX11Window GdkX11Window;
#else
typedef GdkWindow GdkX11Window;
#endif
typedef struct _GdkX11WindowClass GdkX11WindowClass;

GDK_AVAILABLE_IN_ALL
GType    gdk_x11_window_get_type          (void);

GDK_AVAILABLE_IN_ALL
Window   gdk_x11_window_get_xid           (GdkWindow   *window);
GDK_AVAILABLE_IN_ALL
void     gdk_x11_window_set_user_time     (GdkWindow   *window,
                                           guint32      timestamp);
GDK_AVAILABLE_IN_3_4
void     gdk_x11_window_set_utf8_property    (GdkWindow *window,
					      const gchar *name,
					      const gchar *value);
GDK_AVAILABLE_IN_3_2
void     gdk_x11_window_set_theme_variant (GdkWindow   *window,
                                           char        *variant);
GDK_DEPRECATED_IN_3_12_FOR(gdk_window_set_shadow_width)
void     gdk_x11_window_set_frame_extents (GdkWindow *window,
                                           int        left,
                                           int        right,
                                           int        top,
                                           int        bottom);
GDK_AVAILABLE_IN_3_4
void     gdk_x11_window_set_hide_titlebar_when_maximized (GdkWindow *window,
                                                          gboolean   hide_titlebar_when_maximized);
GDK_AVAILABLE_IN_ALL
void     gdk_x11_window_move_to_current_desktop (GdkWindow   *window);

GDK_AVAILABLE_IN_3_10
guint32  gdk_x11_window_get_desktop             (GdkWindow   *window);
GDK_AVAILABLE_IN_3_10
void     gdk_x11_window_move_to_desktop         (GdkWindow   *window,
                                                 guint32      desktop);

GDK_AVAILABLE_IN_3_8
void     gdk_x11_window_set_frame_sync_enabled (GdkWindow *window,
                                                gboolean   frame_sync_enabled);

/**
 * GDK_WINDOW_XDISPLAY:
 * @win: a #GdkWindow.
 *
 * Returns the display of a #GdkWindow.
 *
 * Returns: an Xlib Display*.
 */
#define GDK_WINDOW_XDISPLAY(win)      (GDK_DISPLAY_XDISPLAY (gdk_window_get_display (win)))

/**
 * GDK_WINDOW_XID:
 * @win: a #GdkWindow.
 *
 * Returns the X window belonging to a #GdkWindow.
 *
 * Returns: the Xlib Window of @win.
 */
#define GDK_WINDOW_XID(win)           (gdk_x11_window_get_xid (win))

GDK_AVAILABLE_IN_ALL
guint32       gdk_x11_get_server_time  (GdkWindow       *window);

GDK_AVAILABLE_IN_ALL
GdkWindow  *gdk_x11_window_foreign_new_for_display (GdkDisplay *display,
                                                    Window      window);
GDK_AVAILABLE_IN_ALL
GdkWindow  *gdk_x11_window_lookup_for_display      (GdkDisplay *display,
                                                    Window      window);

G_END_DECLS

#endif /* __GDK_X11_WINDOW_H__ */
