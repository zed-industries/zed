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
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.	 See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library; if not, write to the
 * Free Software Foundation, Inc., 59 Temple Place - Suite 330,
 * Boston, MA 02111-1307, USA.
 */

/*
 * Modified by the GTK+ Team and others 1997-2000.  See the AUTHORS
 * file for a list of people on the GTK+ Team.  See the ChangeLog
 * files for a list of changes.  These files are distributed with
 * GTK+ at ftp://ftp.gtk.org/pub/gtk/. 
 */

#ifndef __GDK_PRIVATE_H__
#define __GDK_PRIVATE_H__

#include <gdk/gdk.h>

G_BEGIN_DECLS

#define GDK_PARENT_RELATIVE_BG ((GdkPixmap *)1L)
#define GDK_NO_BG ((GdkPixmap *)2L)

#ifndef GDK_COMPILATION
#define GDK_WINDOW_TYPE(d) (gdk_window_get_window_type (GDK_WINDOW (d)))
#define GDK_WINDOW_DESTROYED(d) (gdk_window_is_destroyed (GDK_WINDOW (d)))
#endif

void gdk_window_destroy_notify	     (GdkWindow *window);

void gdk_synthesize_window_state (GdkWindow     *window,
                                  GdkWindowState unset_flags,
                                  GdkWindowState set_flags);

/* Tests whether a pair of x,y may cause overflows when converted to Pango
 * units (multiplied by PANGO_SCALE).  We don't allow the entire range, leave
 * some space for additions afterwards, to be safe...
 */
#define GDK_PANGO_UNITS_OVERFLOWS(x,y) (G_UNLIKELY ( \
	(y) >= PANGO_PIXELS (G_MAXINT-PANGO_SCALE)/2 || \
	(x) >= PANGO_PIXELS (G_MAXINT-PANGO_SCALE)/2 || \
	(y) <=-PANGO_PIXELS (G_MAXINT-PANGO_SCALE)/2 || \
	(x) <=-PANGO_PIXELS (G_MAXINT-PANGO_SCALE)/2))

G_END_DECLS

#endif /* __GDK_PRIVATE_H__ */
