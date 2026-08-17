/* GTK - The GIMP Toolkit
 * Copyright (C) 2017 Benjamin Otte
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

#ifndef __GTK_CONTENT_DESERIALIZER_H__
#define __GTK_CONTENT_DESERIALIZER_H__

#if !defined (__GDK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gdk/gdk.h> can be included directly."
#endif


#include <gdk/gdkversionmacros.h>
#include <gdk/gdktypes.h>

G_BEGIN_DECLS

#define GDK_TYPE_CONTENT_DESERIALIZER         (gdk_content_deserializer_get_type ())
#define GDK_CONTENT_DESERIALIZER(o)           (G_TYPE_CHECK_INSTANCE_CAST ((o), GDK_TYPE_CONTENT_DESERIALIZER, GdkContentDeserializer))
#define GDK_IS_CONTENT_DESERIALIZER(o)        (G_TYPE_CHECK_INSTANCE_TYPE ((o), GDK_TYPE_CONTENT_DESERIALIZER))

typedef struct _GdkContentDeserializer GdkContentDeserializer;

/**
 * GdkContentDeserializeFunc:
 * @deserializer: a `GdkContentDeserializer`
 *
 * The type of a function that can be registered with gdk_content_register_deserializer().
 *
 * When the function gets called to operate on content, it can call functions on the
 * @deserializer object to obtain the mime type, input stream, user data, etc. for its
 * operation.
 */
typedef void (* GdkContentDeserializeFunc) (GdkContentDeserializer *deserializer);

GDK_AVAILABLE_IN_ALL
GType                   gdk_content_deserializer_get_type               (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
const char *            gdk_content_deserializer_get_mime_type          (GdkContentDeserializer *deserializer);
GDK_AVAILABLE_IN_ALL
GType                   gdk_content_deserializer_get_gtype              (GdkContentDeserializer *deserializer);
GDK_AVAILABLE_IN_ALL
GValue *                gdk_content_deserializer_get_value              (GdkContentDeserializer *deserializer);
GDK_AVAILABLE_IN_ALL
GInputStream *          gdk_content_deserializer_get_input_stream       (GdkContentDeserializer *deserializer);
GDK_AVAILABLE_IN_ALL
int                     gdk_content_deserializer_get_priority           (GdkContentDeserializer *deserializer);
GDK_AVAILABLE_IN_ALL
GCancellable *          gdk_content_deserializer_get_cancellable        (GdkContentDeserializer *deserializer);
GDK_AVAILABLE_IN_ALL
gpointer                gdk_content_deserializer_get_user_data          (GdkContentDeserializer *deserializer);
GDK_AVAILABLE_IN_ALL
void                    gdk_content_deserializer_set_task_data          (GdkContentDeserializer *deserializer,
                                                                         gpointer                data,
                                                                         GDestroyNotify          notify);
GDK_AVAILABLE_IN_ALL
gpointer                gdk_content_deserializer_get_task_data          (GdkContentDeserializer *deserializer);

GDK_AVAILABLE_IN_ALL
void                    gdk_content_deserializer_return_success         (GdkContentDeserializer *deserializer);
GDK_AVAILABLE_IN_ALL
void                    gdk_content_deserializer_return_error           (GdkContentDeserializer *deserializer,
                                                                         GError                 *error);

GDK_AVAILABLE_IN_ALL
GdkContentFormats *     gdk_content_formats_union_deserialize_gtypes    (GdkContentFormats      *formats);
GDK_AVAILABLE_IN_ALL
GdkContentFormats *     gdk_content_formats_union_deserialize_mime_types(GdkContentFormats      *formats);

GDK_AVAILABLE_IN_ALL
void                    gdk_content_register_deserializer               (const char             *mime_type,
                                                                         GType                   type,
                                                                         GdkContentDeserializeFunc deserialize,
                                                                         gpointer                data,
                                                                         GDestroyNotify          notify);
GDK_AVAILABLE_IN_ALL
void                    gdk_content_deserialize_async                   (GInputStream           *stream,
                                                                         const char             *mime_type,
                                                                         GType                   type,
                                                                         int                     io_priority,
                                                                         GCancellable           *cancellable,
                                                                         GAsyncReadyCallback     callback,
                                                                         gpointer                user_data);
GDK_AVAILABLE_IN_ALL
gboolean                gdk_content_deserialize_finish                  (GAsyncResult           *result,
                                                                         GValue                 *value,
                                                                         GError                **error);


G_END_DECLS

#endif /* __GDK_CONTENT_DESERIALIZER_H__ */
