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

#ifndef __GDK_X11_SURFACE_H__
#define __GDK_X11_SURFACE_H__

#if !defined (__GDKX_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gdk/x11/gdkx.h> can be included directly."
#endif

#include <gdk/gdk.h>

#include <X11/Xlib.h>
#include <X11/Xutil.h>

G_BEGIN_DECLS

#define GDK_TYPE_X11_SURFACE              (gdk_x11_surface_get_type ())
#define GDK_X11_SURFACE(object)           (G_TYPE_CHECK_INSTANCE_CAST ((object), GDK_TYPE_X11_SURFACE, GdkX11Surface))
#define GDK_X11_SURFACE_CLASS(klass)      (G_TYPE_CHECK_CLASS_CAST ((klass), GDK_TYPE_X11_SURFACE, GdkX11SurfaceClass))
#define GDK_IS_X11_SURFACE(object)        (G_TYPE_CHECK_INSTANCE_TYPE ((object), GDK_TYPE_X11_SURFACE))
#define GDK_IS_X11_SURFACE_CLASS(klass)   (G_TYPE_CHECK_CLASS_TYPE ((klass), GDK_TYPE_X11_SURFACE))
#define GDK_X11_SURFACE_GET_CLASS(obj)    (G_TYPE_INSTANCE_GET_CLASS ((obj), GDK_TYPE_X11_SURFACE, GdkX11SurfaceClass))

#ifdef GTK_COMPILATION
typedef struct _GdkX11Surface GdkX11Surface;
#else
typedef GdkSurface GdkX11Surface;
#endif
typedef struct _GdkX11SurfaceClass GdkX11SurfaceClass;

GDK_AVAILABLE_IN_ALL
GType    gdk_x11_surface_get_type          (void);

GDK_AVAILABLE_IN_ALL
Window   gdk_x11_surface_get_xid           (GdkSurface   *surface);
GDK_AVAILABLE_IN_ALL
void     gdk_x11_surface_set_user_time     (GdkSurface   *surface,
                                            guint32      timestamp);
GDK_AVAILABLE_IN_ALL
void     gdk_x11_surface_set_utf8_property (GdkSurface *surface,
                                            const char *name,
                                            const char *value);
GDK_AVAILABLE_IN_ALL
void     gdk_x11_surface_set_theme_variant (GdkSurface   *surface,
                                            const char  *variant);
GDK_AVAILABLE_IN_ALL
void     gdk_x11_surface_move_to_current_desktop (GdkSurface   *surface);

GDK_AVAILABLE_IN_ALL
guint32  gdk_x11_surface_get_desktop             (GdkSurface   *surface);
GDK_AVAILABLE_IN_ALL
void     gdk_x11_surface_move_to_desktop         (GdkSurface   *surface,
                                                  guint32      desktop);

GDK_AVAILABLE_IN_ALL
void     gdk_x11_surface_set_frame_sync_enabled (GdkSurface *surface,
                                                 gboolean   frame_sync_enabled);

/**
 * GDK_SURFACE_XDISPLAY:
 * @win: a `GdkSurface`
 *
 * Returns the display of a `GdkSurface`.
 *
 * Returns: an Xlib Display*.
 */
#define GDK_SURFACE_XDISPLAY(win)      (GDK_DISPLAY_XDISPLAY (gdk_surface_get_display (win)))

/**
 * GDK_SURFACE_XID:
 * @win: a `GdkSurface`
 *
 * Returns the X window belonging to a `GdkSurface`.
 *
 * Returns: the Xlib Window of @win.
 */
#define GDK_SURFACE_XID(win)           (gdk_x11_surface_get_xid (win))

GDK_AVAILABLE_IN_ALL
guint32       gdk_x11_get_server_time  (GdkSurface       *surface);

GDK_AVAILABLE_IN_ALL
GdkSurface  *gdk_x11_surface_lookup_for_display      (GdkDisplay *display,
                                                      Window      window);

GDK_AVAILABLE_IN_ALL
void gdk_x11_surface_set_skip_taskbar_hint (GdkSurface *surface,
                                            gboolean    skips_taskbar);
GDK_AVAILABLE_IN_ALL
void gdk_x11_surface_set_skip_pager_hint   (GdkSurface *surface,
                                            gboolean    skips_pager);
GDK_AVAILABLE_IN_ALL
void gdk_x11_surface_set_urgency_hint      (GdkSurface *surface,
                                            gboolean    urgent);

GDK_AVAILABLE_IN_ALL
void          gdk_x11_surface_set_group    (GdkSurface *surface,
                                            GdkSurface *leader);
GDK_AVAILABLE_IN_ALL
GdkSurface *  gdk_x11_surface_get_group    (GdkSurface *surface);


G_END_DECLS

#endif /* __GDK_X11_SURFACE_H__ */
