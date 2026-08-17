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

#ifndef __GDK_X11_UTILS_H__
#define __GDK_X11_UTILS_H__

#if !defined (__GDKX_H_INSIDE__) && !defined (GDK_COMPILATION)
#error "Only <gdk/gdkx.h> can be included directly."
#endif

#include <gdk/gdk.h>

#include <X11/Xlib.h>
#include <X11/Xutil.h>

G_BEGIN_DECLS

GDK_AVAILABLE_IN_ALL
Window   gdk_x11_get_default_root_xwindow (void);
GDK_AVAILABLE_IN_ALL
Display *gdk_x11_get_default_xdisplay     (void);

/**
 * GDK_ROOT_WINDOW:
 *
 * Obtains the Xlib window id of the root window of the current screen.
 */
#define GDK_ROOT_WINDOW()             (gdk_x11_get_default_root_xwindow ())

/**
 * GDK_XID_TO_POINTER:
 * @xid: XID to stuff into the pointer
 *
 * Converts an XID into a @gpointer. This is useful with data structures
 * that use pointer arguments such as #GHashTable. Use GDK_POINTER_TO_XID()
 * to convert the argument back to an XID.
 */
#define GDK_XID_TO_POINTER(xid) GUINT_TO_POINTER(xid)

/**
 * GDK_POINTER_TO_XID:
 * @pointer: pointer to extract an XID from
 *
 * Converts a @gpointer back to an XID that was previously converted
 * using GDK_XID_TO_POINTER().
 */
#define GDK_POINTER_TO_XID(pointer) GPOINTER_TO_UINT(pointer)

GDK_AVAILABLE_IN_ALL
void          gdk_x11_grab_server    (void);
GDK_AVAILABLE_IN_ALL
void          gdk_x11_ungrab_server  (void);

GDK_DEPRECATED_IN_3_24
cairo_pattern_t *gdk_x11_get_parent_relative_pattern (void);

G_END_DECLS

#endif /* __GDK_X11_UTILS_H__ */
