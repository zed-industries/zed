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

#ifndef __GDK_DND_H__
#define __GDK_DND_H__

#if !defined (__GDK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gdk/gdk.h> can be included directly."
#endif

#include <gdk/gdkdevice.h>
#include <gdk/gdkenums.h>
#include <gdk/gdkevents.h>
#include <gdk/gdktypes.h>

G_BEGIN_DECLS

#define GDK_TYPE_DRAG              (gdk_drag_get_type ())
#define GDK_DRAG(object)           (G_TYPE_CHECK_INSTANCE_CAST ((object), GDK_TYPE_DRAG, GdkDrag))
#define GDK_IS_DRAG(object)        (G_TYPE_CHECK_INSTANCE_TYPE ((object), GDK_TYPE_DRAG))

/**
 * GdkDragCancelReason:
 * @GDK_DRAG_CANCEL_NO_TARGET: There is no suitable drop target.
 * @GDK_DRAG_CANCEL_USER_CANCELLED: Drag cancelled by the user
 * @GDK_DRAG_CANCEL_ERROR: Unspecified error.
 *
 * Used in `GdkDrag` to the reason of a cancelled DND operation.
 */
typedef enum {
  GDK_DRAG_CANCEL_NO_TARGET,
  GDK_DRAG_CANCEL_USER_CANCELLED,
  GDK_DRAG_CANCEL_ERROR
} GdkDragCancelReason;

GDK_AVAILABLE_IN_ALL
GType            gdk_drag_get_type             (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
GdkDisplay *     gdk_drag_get_display          (GdkDrag *drag);
GDK_AVAILABLE_IN_ALL
GdkDevice *      gdk_drag_get_device           (GdkDrag *drag);

GDK_AVAILABLE_IN_ALL
GdkContentFormats *gdk_drag_get_formats        (GdkDrag *drag);
GDK_AVAILABLE_IN_ALL
GdkDragAction    gdk_drag_get_actions          (GdkDrag *drag);
GDK_AVAILABLE_IN_ALL
GdkDragAction    gdk_drag_get_selected_action  (GdkDrag *drag);

GDK_AVAILABLE_IN_ALL
gboolean         gdk_drag_action_is_unique     (GdkDragAction   action) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
GdkDrag *        gdk_drag_begin                (GdkSurface         *surface,
                                                GdkDevice          *device,
                                                GdkContentProvider *content,
                                                GdkDragAction       actions,
                                                double              dx,
                                                double              dy);

GDK_AVAILABLE_IN_ALL
void            gdk_drag_drop_done   (GdkDrag  *drag,
                                      gboolean  success);

GDK_AVAILABLE_IN_ALL
GdkSurface      *gdk_drag_get_drag_surface (GdkDrag *drag);

GDK_AVAILABLE_IN_ALL
void            gdk_drag_set_hotspot (GdkDrag *drag,
                                      int      hot_x,
                                      int      hot_y);

GDK_AVAILABLE_IN_ALL
GdkContentProvider *
                gdk_drag_get_content (GdkDrag *drag);

GDK_AVAILABLE_IN_ALL
GdkSurface *    gdk_drag_get_surface (GdkDrag *drag);

G_DEFINE_AUTOPTR_CLEANUP_FUNC(GdkDrag, g_object_unref)

G_END_DECLS

#endif /* __GDK_DND_H__ */
