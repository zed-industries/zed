/*
 * Copyright © 2018 Benjamin Otte
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library. If not, see <http://www.gnu.org/licenses/>.
 *
 * Authors: Benjamin Otte <otte@gnome.org>
 */

#ifndef __GDK_SNAPSHOT_H__
#define __GDK_SNAPSHOT_H__

#if !defined (__GDK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gdk/gdk.h> can be included directly."
#endif

#include <gdk/gdktypes.h>
#include <gdk/gdkversionmacros.h>

G_BEGIN_DECLS

typedef struct _GdkSnapshotClass        GdkSnapshotClass;

#define GDK_TYPE_SNAPSHOT               (gdk_snapshot_get_type ())

#define GDK_SNAPSHOT(obj)               (G_TYPE_CHECK_INSTANCE_CAST ((obj), GDK_TYPE_SNAPSHOT, GdkSnapshot))
#define GDK_IS_SNAPSHOT(obj)            (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GDK_TYPE_SNAPSHOT))

GDK_AVAILABLE_IN_ALL
GType           gdk_snapshot_get_type   (void) G_GNUC_CONST;

G_DEFINE_AUTOPTR_CLEANUP_FUNC(GdkSnapshot, g_object_unref)

G_END_DECLS

#endif /* __GDK_SNAPSHOT_H__ */
