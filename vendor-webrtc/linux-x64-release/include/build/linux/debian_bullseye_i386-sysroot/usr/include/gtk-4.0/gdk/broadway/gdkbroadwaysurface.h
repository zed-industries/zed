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

#ifndef __GDK_BROADWAY_SURFACE_H__
#define __GDK_BROADWAY_SURFACE_H__

#include <gdk/gdk.h>

G_BEGIN_DECLS

#define GDK_TYPE_BROADWAY_SURFACE              (gdk_broadway_surface_get_type ())
#define GDK_BROADWAY_SURFACE(object)           (G_TYPE_CHECK_INSTANCE_CAST ((object), GDK_TYPE_BROADWAY_SURFACE, GdkBroadwaySurface))
#define GDK_BROADWAY_SURFACE_CLASS(klass)      (G_TYPE_CHECK_CLASS_CAST ((klass), GDK_TYPE_BROADWAY_SURFACE, GdkBroadwaySurfaceClass))
#define GDK_IS_BROADWAY_SURFACE(object)        (G_TYPE_CHECK_INSTANCE_TYPE ((object), GDK_TYPE_BROADWAY_SURFACE))
#define GDK_IS_BROADWAY_SURFACE_CLASS(klass)   (G_TYPE_CHECK_CLASS_TYPE ((klass), GDK_TYPE_BROADWAY_SURFACE))
#define GDK_BROADWAY_SURFACE_GET_CLASS(obj)    (G_TYPE_INSTANCE_GET_CLASS ((obj), GDK_TYPE_BROADWAY_SURFACE, GdkBroadwaySurfaceClass))

#ifdef GTK_COMPILATION
typedef struct _GdkBroadwaySurface GdkBroadwaySurface;
#else
typedef GdkSurface GdkBroadwaySurface;
#endif
typedef struct _GdkBroadwaySurfaceClass GdkBroadwaySurfaceClass;

GDK_AVAILABLE_IN_ALL
GType    gdk_broadway_surface_get_type          (void);

G_END_DECLS

#endif /* __GDK_BROADWAY_SURFACE_H__ */
