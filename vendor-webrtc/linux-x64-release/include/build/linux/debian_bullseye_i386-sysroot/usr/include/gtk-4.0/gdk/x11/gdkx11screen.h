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

#ifndef __GDK_X11_SCREEN_H__
#define __GDK_X11_SCREEN_H__

#if !defined (__GDKX_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gdk/x11/gdkx.h> can be included directly."
#endif

#include <gdk/gdk.h>

#include <X11/Xlib.h>
#include <X11/Xutil.h>

G_BEGIN_DECLS

#define GDK_TYPE_X11_SCREEN              (gdk_x11_screen_get_type ())
#define GDK_X11_SCREEN(object)           (G_TYPE_CHECK_INSTANCE_CAST ((object), GDK_TYPE_X11_SCREEN, GdkX11Screen))
#define GDK_X11_SCREEN_CLASS(klass)      (G_TYPE_CHECK_CLASS_CAST ((klass), GDK_TYPE_X11_SCREEN, GdkX11ScreenClass))
#define GDK_IS_X11_SCREEN(object)        (G_TYPE_CHECK_INSTANCE_TYPE ((object), GDK_TYPE_X11_SCREEN))
#define GDK_IS_X11_SCREEN_CLASS(klass)   (G_TYPE_CHECK_CLASS_TYPE ((klass), GDK_TYPE_X11_SCREEN))
#define GDK_X11_SCREEN_GET_CLASS(obj)    (G_TYPE_INSTANCE_GET_CLASS ((obj), GDK_TYPE_X11_SCREEN, GdkX11ScreenClass))

typedef struct _GdkX11Screen GdkX11Screen;
typedef struct _GdkX11ScreenClass GdkX11ScreenClass;

GDK_AVAILABLE_IN_ALL
GType    gdk_x11_screen_get_type          (void);

GDK_AVAILABLE_IN_ALL
Screen * gdk_x11_screen_get_xscreen       (GdkX11Screen   *screen);
GDK_AVAILABLE_IN_ALL
int      gdk_x11_screen_get_screen_number (GdkX11Screen   *screen);

GDK_AVAILABLE_IN_ALL
const char* gdk_x11_screen_get_window_manager_name (GdkX11Screen *screen);

GDK_AVAILABLE_IN_ALL
gboolean gdk_x11_screen_supports_net_wm_hint (GdkX11Screen *screen,
                                              const char   *property_name);

GDK_AVAILABLE_IN_ALL
XID      gdk_x11_screen_get_monitor_output   (GdkX11Screen *screen,
                                              int           monitor_num);

GDK_AVAILABLE_IN_ALL
guint32  gdk_x11_screen_get_number_of_desktops (GdkX11Screen *screen);
GDK_AVAILABLE_IN_ALL
guint32  gdk_x11_screen_get_current_desktop    (GdkX11Screen *screen);

G_END_DECLS

#endif /* __GDK_X11_SCREEN_H__ */
