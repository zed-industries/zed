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

#if !defined (__GDKX_H_INSIDE__) && !defined (GDK_COMPILATION)
#error "Only <gdk/gdkx.h> can be included directly."
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

#ifdef GDK_COMPILATION
typedef struct _GdkX11Screen GdkX11Screen;
#else
typedef GdkScreen GdkX11Screen;
#endif
typedef struct _GdkX11ScreenClass GdkX11ScreenClass;

GDK_AVAILABLE_IN_ALL
GType    gdk_x11_screen_get_type          (void);

GDK_AVAILABLE_IN_ALL
Screen * gdk_x11_screen_get_xscreen       (GdkScreen   *screen);
GDK_AVAILABLE_IN_ALL
int      gdk_x11_screen_get_screen_number (GdkScreen   *screen);

GDK_AVAILABLE_IN_ALL
const char* gdk_x11_screen_get_window_manager_name (GdkScreen *screen);

GDK_AVAILABLE_IN_ALL
gint     gdk_x11_get_default_screen       (void);

/**
 * GDK_SCREEN_XDISPLAY:
 * @screen: a #GdkScreen
 *
 * Returns the display of a X11 #GdkScreen.
 *
 * Returns: an Xlib Display*.
 */
#define GDK_SCREEN_XDISPLAY(screen) (gdk_x11_display_get_xdisplay (gdk_screen_get_display (screen)))

/**
 * GDK_SCREEN_XSCREEN:
 * @screen: a #GdkScreen
 *
 * Returns the screen of a X11 #GdkScreen.
 *
 * Returns: an Xlib Screen*
 */
#define GDK_SCREEN_XSCREEN(screen) (gdk_x11_screen_get_xscreen (screen))

/**
 * GDK_SCREEN_XNUMBER:
 * @screen: a #GdkScreen
 *
 * Returns the index of a X11 #GdkScreen.
 *
 * Returns: the position of @screen among the screens of its display
 */
#define GDK_SCREEN_XNUMBER(screen) (gdk_x11_screen_get_screen_number (screen))

GDK_AVAILABLE_IN_ALL
gboolean gdk_x11_screen_supports_net_wm_hint (GdkScreen *screen,
                                              GdkAtom    property);

GDK_AVAILABLE_IN_ALL
XID      gdk_x11_screen_get_monitor_output   (GdkScreen *screen,
                                              gint       monitor_num);

GDK_AVAILABLE_IN_3_10
guint32  gdk_x11_screen_get_number_of_desktops (GdkScreen *screen);
GDK_AVAILABLE_IN_3_10
guint32  gdk_x11_screen_get_current_desktop    (GdkScreen *screen);

G_END_DECLS

#endif /* __GDK_X11_SCREEN_H__ */
