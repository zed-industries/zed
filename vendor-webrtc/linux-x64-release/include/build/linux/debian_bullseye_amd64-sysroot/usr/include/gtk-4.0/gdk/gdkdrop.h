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


#ifndef __GDK_DROP_H__
#define __GDK_DROP_H__

#if !defined (__GDK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gdk/gdk.h> can be included directly."
#endif

#include <gdk/gdkenums.h>
#include <gdk/gdktypes.h>
#include <gdk/gdkversionmacros.h>

G_BEGIN_DECLS

#define GDK_TYPE_DROP              (gdk_drop_get_type ())
#define GDK_DROP(object)           (G_TYPE_CHECK_INSTANCE_CAST ((object), GDK_TYPE_DROP, GdkDrop))
#define GDK_IS_DROP(object)        (G_TYPE_CHECK_INSTANCE_TYPE ((object), GDK_TYPE_DROP))

GDK_AVAILABLE_IN_ALL
GType                   gdk_drop_get_type               (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
GdkDisplay *            gdk_drop_get_display            (GdkDrop                *self);
GDK_AVAILABLE_IN_ALL
GdkDevice *             gdk_drop_get_device             (GdkDrop                *self);
GDK_AVAILABLE_IN_ALL
GdkSurface *            gdk_drop_get_surface            (GdkDrop                *self);
GDK_AVAILABLE_IN_ALL
GdkContentFormats *     gdk_drop_get_formats            (GdkDrop                *self);
GDK_AVAILABLE_IN_ALL
GdkDragAction           gdk_drop_get_actions            (GdkDrop                *self);
GDK_AVAILABLE_IN_ALL
GdkDrag *               gdk_drop_get_drag               (GdkDrop                *self);

GDK_AVAILABLE_IN_ALL
void                    gdk_drop_status                 (GdkDrop                *self,
                                                         GdkDragAction           actions,
                                                         GdkDragAction           preferred);
GDK_AVAILABLE_IN_ALL
void                    gdk_drop_finish                 (GdkDrop                *self,
                                                         GdkDragAction           action);

GDK_AVAILABLE_IN_ALL
void                    gdk_drop_read_async             (GdkDrop                *self,
                                                         const char            **mime_types,
                                                         int                     io_priority,
                                                         GCancellable           *cancellable,
                                                         GAsyncReadyCallback     callback,
                                                         gpointer                user_data);
GDK_AVAILABLE_IN_ALL
GInputStream *          gdk_drop_read_finish            (GdkDrop                *self,
                                                         GAsyncResult           *result,
                                                         const char            **out_mime_type,
                                                         GError                **error);
GDK_AVAILABLE_IN_ALL
void                    gdk_drop_read_value_async       (GdkDrop                *self,
                                                         GType                   type,
                                                         int                     io_priority,
                                                         GCancellable           *cancellable,
                                                         GAsyncReadyCallback     callback,
                                                         gpointer                user_data);
GDK_AVAILABLE_IN_ALL
const GValue *          gdk_drop_read_value_finish      (GdkDrop                *self,
                                                         GAsyncResult           *result,
                                                         GError                **error);

G_DEFINE_AUTOPTR_CLEANUP_FUNC(GdkDrop, g_object_unref)

G_END_DECLS

#endif /* __GDK_DROP_H__ */
