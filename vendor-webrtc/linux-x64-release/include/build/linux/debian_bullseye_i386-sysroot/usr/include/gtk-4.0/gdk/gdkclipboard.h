/* GDK - The GIMP Drawing Kit
 *
 * Copyright (C) 2017 Benjamin Otte <otte@gnome.org>
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

#ifndef __GDK_CLIPBOARD_H__
#define __GDK_CLIPBOARD_H__

#if !defined (__GDK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gdk/gdk.h> can be included directly."
#endif

#include <gdk/gdkversionmacros.h>
#include <gdk/gdktypes.h>
#include <gio/gio.h>


G_BEGIN_DECLS

#define GDK_TYPE_CLIPBOARD            (gdk_clipboard_get_type ())
#define GDK_CLIPBOARD(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), GDK_TYPE_CLIPBOARD, GdkClipboard))
#define GDK_IS_CLIPBOARD(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GDK_TYPE_CLIPBOARD))

GDK_AVAILABLE_IN_ALL
GType                   gdk_clipboard_get_type          (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
GdkDisplay *            gdk_clipboard_get_display       (GdkClipboard          *clipboard);
GDK_AVAILABLE_IN_ALL
GdkContentFormats *     gdk_clipboard_get_formats       (GdkClipboard          *clipboard);
GDK_AVAILABLE_IN_ALL
gboolean                gdk_clipboard_is_local          (GdkClipboard          *clipboard);
GDK_AVAILABLE_IN_ALL
GdkContentProvider *    gdk_clipboard_get_content       (GdkClipboard          *clipboard);

GDK_AVAILABLE_IN_ALL
void                    gdk_clipboard_store_async       (GdkClipboard          *clipboard,
                                                         int                    io_priority,
                                                         GCancellable          *cancellable,
                                                         GAsyncReadyCallback    callback,
                                                         gpointer               user_data);
GDK_AVAILABLE_IN_ALL
gboolean                gdk_clipboard_store_finish      (GdkClipboard          *clipboard,
                                                         GAsyncResult          *result,
                                                         GError               **error);

GDK_AVAILABLE_IN_ALL
void                    gdk_clipboard_read_async        (GdkClipboard          *clipboard,
                                                         const char           **mime_types,
                                                         int                    io_priority,
                                                         GCancellable          *cancellable,
                                                         GAsyncReadyCallback    callback,
                                                         gpointer               user_data);
GDK_AVAILABLE_IN_ALL
GInputStream *          gdk_clipboard_read_finish       (GdkClipboard          *clipboard,
                                                         GAsyncResult          *result,
                                                         const char           **out_mime_type,
                                                         GError               **error);
GDK_AVAILABLE_IN_ALL
void                    gdk_clipboard_read_value_async  (GdkClipboard          *clipboard,
                                                         GType                  type,
                                                         int                    io_priority,
                                                         GCancellable          *cancellable,
                                                         GAsyncReadyCallback    callback,
                                                         gpointer               user_data);
GDK_AVAILABLE_IN_ALL
const GValue *          gdk_clipboard_read_value_finish (GdkClipboard          *clipboard,
                                                         GAsyncResult          *result,
                                                         GError               **error);
GDK_AVAILABLE_IN_ALL
void                    gdk_clipboard_read_texture_async (GdkClipboard          *clipboard,
                                                          GCancellable          *cancellable,
                                                          GAsyncReadyCallback    callback,
                                                          gpointer               user_data);
GDK_AVAILABLE_IN_ALL
GdkTexture *            gdk_clipboard_read_texture_finish (GdkClipboard         *clipboard,
                                                           GAsyncResult          *result,
                                                           GError               **error);
GDK_AVAILABLE_IN_ALL
void                    gdk_clipboard_read_text_async   (GdkClipboard          *clipboard,
                                                         GCancellable          *cancellable,
                                                         GAsyncReadyCallback    callback,
                                                         gpointer               user_data);
GDK_AVAILABLE_IN_ALL
char *                  gdk_clipboard_read_text_finish  (GdkClipboard          *clipboard,
                                                         GAsyncResult          *result,
                                                         GError               **error);

GDK_AVAILABLE_IN_ALL
gboolean                gdk_clipboard_set_content       (GdkClipboard          *clipboard,
                                                         GdkContentProvider    *provider);
GDK_AVAILABLE_IN_ALL
void                    gdk_clipboard_set               (GdkClipboard          *clipboard,
                                                         GType                  type,
                                                         ...);
GDK_AVAILABLE_IN_ALL
void                    gdk_clipboard_set_valist        (GdkClipboard          *clipboard,
                                                         GType                  type,
                                                         va_list                args);
GDK_AVAILABLE_IN_ALL
void                    gdk_clipboard_set_value         (GdkClipboard          *clipboard,
                                                         const GValue          *value);
GDK_AVAILABLE_IN_ALL
void                    gdk_clipboard_set_text          (GdkClipboard          *clipboard,
                                                         const char            *text);
GDK_AVAILABLE_IN_ALL
void                    gdk_clipboard_set_texture       (GdkClipboard          *clipboard,
                                                         GdkTexture            *texture);

G_DEFINE_AUTOPTR_CLEANUP_FUNC(GdkClipboard, g_object_unref)

G_END_DECLS

#endif /* __GDK_CLIPBOARD_H__ */
