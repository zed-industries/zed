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

#ifndef __GDK_RECTANGLE_H__
#define __GDK_RECTANGLE_H__

#if !defined (__GDK_H_INSIDE__) && !defined (GDK_COMPILATION)
#error "Only <gdk/gdk.h> can be included directly."
#endif

#include <gdk/gdktypes.h>
#include <gdk/gdkversionmacros.h>

G_BEGIN_DECLS

/* Rectangle utilities
 */
GDK_AVAILABLE_IN_ALL
gboolean gdk_rectangle_intersect (const GdkRectangle *src1,
                                  const GdkRectangle *src2,
                                  GdkRectangle       *dest);
GDK_AVAILABLE_IN_ALL
void     gdk_rectangle_union     (const GdkRectangle *src1,
                                  const GdkRectangle *src2,
                                  GdkRectangle       *dest);

GDK_AVAILABLE_IN_3_20
gboolean gdk_rectangle_equal     (const GdkRectangle *rect1,
                                  const GdkRectangle *rect2);

GDK_AVAILABLE_IN_ALL
GType gdk_rectangle_get_type (void) G_GNUC_CONST;

#define GDK_TYPE_RECTANGLE (gdk_rectangle_get_type ())

G_END_DECLS

#endif /* __GDK__RECTANGLE_H__ */
