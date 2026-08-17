/* gtkrichtext.h
 *
 * Copyright (C) 2006 Imendio AB
 * Contact: Michael Natterer <mitch@imendio.com>
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public
 * License along with this library. If not, see <http://www.gnu.org/licenses/>.
 */

#ifndef __GTK_TEXT_BUFFER_RICH_TEXT_H__
#define __GTK_TEXT_BUFFER_RICH_TEXT_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtktextbuffer.h>

G_BEGIN_DECLS

/**
 * GtkTextBufferSerializeFunc:
 * @register_buffer: the #GtkTextBuffer for which the format is registered
 * @content_buffer: the #GtkTextBuffer to serialize
 * @start: start of the block of text to serialize
 * @end: end of the block of text to serialize
 * @length: Return location for the length of the serialized data
 * @user_data: user data that was specified when registering the format
 *
 * A function that is called to serialize the content of a text buffer.
 * It must return the serialized form of the content.
 *
 * Returns: (nullable): a newly-allocated array of guint8 which contains
 * the serialized data, or %NULL if an error occurred
 */
typedef guint8 * (* GtkTextBufferSerializeFunc)   (GtkTextBuffer     *register_buffer,
                                                   GtkTextBuffer     *content_buffer,
                                                   const GtkTextIter *start,
                                                   const GtkTextIter *end,
                                                   gsize             *length,
                                                   gpointer           user_data);

/**
 * GtkTextBufferDeserializeFunc:
 * @register_buffer: the #GtkTextBuffer the format is registered with
 * @content_buffer: the #GtkTextBuffer to deserialize into
 * @iter: insertion point for the deserialized text
 * @data: (array length=length): data to deserialize
 * @length: length of @data
 * @create_tags: %TRUE if deserializing may create tags
 * @user_data: user data that was specified when registering the format
 * @error: return location for a #GError
 *
 * A function that is called to deserialize rich text that has been
 * serialized with gtk_text_buffer_serialize(), and insert it at @iter.
 *
 * Returns: %TRUE on success, %FALSE otherwise
 */
typedef gboolean (* GtkTextBufferDeserializeFunc) (GtkTextBuffer     *register_buffer,
                                                   GtkTextBuffer     *content_buffer,
                                                   GtkTextIter       *iter,
                                                   const guint8      *data,
                                                   gsize              length,
                                                   gboolean           create_tags,
                                                   gpointer           user_data,
                                                   GError           **error);

GDK_AVAILABLE_IN_ALL
GdkAtom   gtk_text_buffer_register_serialize_format   (GtkTextBuffer                *buffer,
                                                       const gchar                  *mime_type,
                                                       GtkTextBufferSerializeFunc    function,
                                                       gpointer                      user_data,
                                                       GDestroyNotify                user_data_destroy);
GDK_AVAILABLE_IN_ALL
GdkAtom   gtk_text_buffer_register_serialize_tagset   (GtkTextBuffer                *buffer,
                                                       const gchar                  *tagset_name);

GDK_AVAILABLE_IN_ALL
GdkAtom   gtk_text_buffer_register_deserialize_format (GtkTextBuffer                *buffer,
                                                       const gchar                  *mime_type,
                                                       GtkTextBufferDeserializeFunc  function,
                                                       gpointer                      user_data,
                                                       GDestroyNotify                user_data_destroy);
GDK_AVAILABLE_IN_ALL
GdkAtom   gtk_text_buffer_register_deserialize_tagset (GtkTextBuffer                *buffer,
                                                       const gchar                  *tagset_name);

GDK_AVAILABLE_IN_ALL
void    gtk_text_buffer_unregister_serialize_format   (GtkTextBuffer                *buffer,
                                                       GdkAtom                       format);
GDK_AVAILABLE_IN_ALL
void    gtk_text_buffer_unregister_deserialize_format (GtkTextBuffer                *buffer,
                                                       GdkAtom                       format);

GDK_AVAILABLE_IN_ALL
void     gtk_text_buffer_deserialize_set_can_create_tags (GtkTextBuffer             *buffer,
                                                          GdkAtom                    format,
                                                          gboolean                   can_create_tags);
GDK_AVAILABLE_IN_ALL
gboolean gtk_text_buffer_deserialize_get_can_create_tags (GtkTextBuffer             *buffer,
                                                          GdkAtom                    format);

GDK_AVAILABLE_IN_ALL
GdkAtom * gtk_text_buffer_get_serialize_formats       (GtkTextBuffer                *buffer,
                                                       gint                         *n_formats);
GDK_AVAILABLE_IN_ALL
GdkAtom * gtk_text_buffer_get_deserialize_formats     (GtkTextBuffer                *buffer,
                                                       gint                         *n_formats);

GDK_AVAILABLE_IN_ALL
guint8  * gtk_text_buffer_serialize                   (GtkTextBuffer                *register_buffer,
                                                       GtkTextBuffer                *content_buffer,
                                                       GdkAtom                       format,
                                                       const GtkTextIter            *start,
                                                       const GtkTextIter            *end,
                                                       gsize                        *length);
GDK_AVAILABLE_IN_ALL
gboolean  gtk_text_buffer_deserialize                 (GtkTextBuffer                *register_buffer,
                                                       GtkTextBuffer                *content_buffer,
                                                       GdkAtom                       format,
                                                       GtkTextIter                  *iter,
                                                       const guint8                 *data,
                                                       gsize                         length,
                                                       GError                      **error);

G_END_DECLS

#endif /* __GTK_TEXT_BUFFER_RICH_TEXT_H__ */
