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

#ifndef __GDK_CONTENT_PROVIDER_H__
#define __GDK_CONTENT_PROVIDER_H__

#if !defined (__GDK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gdk/gdk.h> can be included directly."
#endif

#include <gdk/gdkversionmacros.h>
#include <gdk/gdktypes.h>


G_BEGIN_DECLS

#define GDK_TYPE_CONTENT_PROVIDER            (gdk_content_provider_get_type ())
#define GDK_CONTENT_PROVIDER(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), GDK_TYPE_CONTENT_PROVIDER, GdkContentProvider))
#define GDK_IS_CONTENT_PROVIDER(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GDK_TYPE_CONTENT_PROVIDER))
#define GDK_CONTENT_PROVIDER_CLASS(klass)    (G_TYPE_CHECK_CLASS_CAST ((klass), GDK_TYPE_CONTENT_PROVIDER, GdkContentProviderClass))
#define GDK_IS_CONTENT_PROVIDER_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE ((klass), GDK_TYPE_CONTENT_PROVIDER))
#define GDK_CONTENT_PROVIDER_GET_CLASS(obj)  (G_TYPE_INSTANCE_GET_CLASS ((obj), GDK_TYPE_CONTENT_PROVIDER, GdkContentProviderClass))

typedef struct _GdkContentProviderClass GdkContentProviderClass;

struct _GdkContentProvider
{
  GObject parent;
};

/**
 * GdkContentProviderClass:
 * @content_changed: Signal class closure for `GdkContentProvider::content-changed`
 *
 * Class structure for `GdkContentProvider`.
 */
struct _GdkContentProviderClass
{
  GObjectClass parent_class;

  /* signals */
  void                  (* content_changed)                             (GdkContentProvider     *provider);

  /*< private >*/
  /* vfuncs */
  void                  (* attach_clipboard)                            (GdkContentProvider     *provider,
                                                                         GdkClipboard           *clipboard);
  void                  (* detach_clipboard)                            (GdkContentProvider     *provider,
                                                                         GdkClipboard           *clipboard);

  GdkContentFormats *   (* ref_formats)                                 (GdkContentProvider     *provider);
  GdkContentFormats *   (* ref_storable_formats)                        (GdkContentProvider     *provider);
  void                  (* write_mime_type_async)                       (GdkContentProvider     *provider,
                                                                         const char             *mime_type,
                                                                         GOutputStream          *stream,
                                                                         int                     io_priority,
                                                                         GCancellable           *cancellable,
                                                                         GAsyncReadyCallback     callback,
                                                                         gpointer                user_data);
  gboolean              (* write_mime_type_finish)                      (GdkContentProvider     *provider,
                                                                         GAsyncResult           *result,
                                                                         GError                **error);
  gboolean              (* get_value)                                   (GdkContentProvider     *provider,
                                                                         GValue                 *value,
                                                                         GError                **error);

  /*< private >*/
  gpointer padding[8];
};


GDK_AVAILABLE_IN_ALL
GType                   gdk_content_provider_get_type                   (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
GdkContentFormats *     gdk_content_provider_ref_formats                (GdkContentProvider     *provider);
GDK_AVAILABLE_IN_ALL
GdkContentFormats *     gdk_content_provider_ref_storable_formats       (GdkContentProvider     *provider);

GDK_AVAILABLE_IN_ALL
void                    gdk_content_provider_content_changed            (GdkContentProvider     *provider);

GDK_AVAILABLE_IN_ALL
void                    gdk_content_provider_write_mime_type_async      (GdkContentProvider     *provider,
                                                                         const char             *mime_type,
                                                                         GOutputStream          *stream,
                                                                         int                     io_priority,
                                                                         GCancellable           *cancellable,
                                                                         GAsyncReadyCallback     callback,
                                                                         gpointer                user_data);
GDK_AVAILABLE_IN_ALL
gboolean                gdk_content_provider_write_mime_type_finish     (GdkContentProvider     *provider,
                                                                         GAsyncResult           *result,
                                                                         GError                **error);
GDK_AVAILABLE_IN_ALL
gboolean                gdk_content_provider_get_value                  (GdkContentProvider     *provider,
                                                                         GValue                 *value,
                                                                         GError                **error);

G_DEFINE_AUTOPTR_CLEANUP_FUNC(GdkContentProvider, g_object_unref)

G_END_DECLS

#endif /* __GDK_CONTENT_PROVIDER_H__ */
