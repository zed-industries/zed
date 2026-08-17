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

#ifndef __GDK_X11_VISUAL_H__
#define __GDK_X11_VISUAL_H__

#if !defined (__GDKX_H_INSIDE__) && !defined (GDK_COMPILATION)
#error "Only <gdk/gdkx.h> can be included directly."
#endif

#include <gdk/gdk.h>

#include <X11/Xlib.h>
#include <X11/Xutil.h>

G_BEGIN_DECLS

#define GDK_TYPE_X11_VISUAL              (gdk_x11_visual_get_type ())
#define GDK_X11_VISUAL(object)           (G_TYPE_CHECK_INSTANCE_CAST ((object), GDK_TYPE_X11_VISUAL, GdkX11Visual))
#define GDK_X11_VISUAL_CLASS(klass)      (G_TYPE_CHECK_CLASS_CAST ((klass), GDK_TYPE_X11_VISUAL, GdkX11VisualClass))
#define GDK_IS_X11_VISUAL(object)        (G_TYPE_CHECK_INSTANCE_TYPE ((object), GDK_TYPE_X11_VISUAL))
#define GDK_IS_X11_VISUAL_CLASS(klass)   (G_TYPE_CHECK_CLASS_TYPE ((klass), GDK_TYPE_X11_VISUAL))
#define GDK_X11_VISUAL_GET_CLASS(obj)    (G_TYPE_INSTANCE_GET_CLASS ((obj), GDK_TYPE_X11_VISUAL, GdkX11VisualClass))

#ifdef GDK_COMPILATION
typedef struct _GdkX11Visual GdkX11Visual;
#else
typedef GdkVisual GdkX11Visual;
#endif
typedef struct _GdkX11VisualClass GdkX11VisualClass;

GDK_AVAILABLE_IN_ALL
GType    gdk_x11_visual_get_type          (void);

GDK_AVAILABLE_IN_ALL
Visual * gdk_x11_visual_get_xvisual       (GdkVisual   *visual);

#define GDK_VISUAL_XVISUAL(visual)    (gdk_x11_visual_get_xvisual (visual))

GDK_AVAILABLE_IN_ALL
GdkVisual* gdk_x11_screen_lookup_visual (GdkScreen *screen,
                                         VisualID   xvisualid);

G_END_DECLS

#endif /* __GDK_X11_VISUAL_H__ */
