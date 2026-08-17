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
 * License along with this library; if not, write to the
 * Free Software Foundation, Inc., 59 Temple Place - Suite 330,
 * Boston, MA 02111-1307, USA.
 */

#ifndef __GTK_TEXT_BUFFER_RICH_TEXT_H__
#define __GTK_TEXT_BUFFER_RICH_TEXT_H__

#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtktextbuffer.h>

G_BEGIN_DECLS

typedef guint8 * (* GtkTextBufferSerializeFunc)   (GtkTextBuffer     *register_buffer,
                                                   GtkTextBuffer     *content_buffer,
                                                   const GtkTextIter *start,
                                                   const GtkTextIter *end,
                                                   gsize             *length,
                                                   gpointer           user_data);
typedef gboolean (* GtkTextBufferDeserializeFunc) (GtkTextBuffer     *register_buffer,
                                                   GtkTextBuffer     *content_buffer,
                                                   GtkTextIter       *iter,
                                                   const guint8      *data,
                                                   gsize              length,
                                                   gboolean           create_tags,
                                                   gpointer           user_data,
                                                   GError           **error);

GdkAtom   gtk_text_buffer_register_serialize_format   (GtkTextBuffer                *buffer,
                                                       const gchar                  *mime_type,
                                                       GtkTextBufferSerializeFunc    function,
                                                       gpointer                      user_data,
                                                       GDestroyNotify                user_data_destroy);
GdkAtom   gtk_text_buffer_register_serialize_tagset   (GtkTextBuffer                *buffer,
                                                       const gchar                  *tagset_name);

GdkAtom   gtk_text_buffer_register_deserialize_format (GtkTextBuffer                *buffer,
                                                       const gchar                  *mime_type,
                                                       GtkTextBufferDeserializeFunc  function,
                                                       gpointer                      user_data,
                                                       GDestroyNotify                user_data_destroy);
GdkAtom   gtk_text_buffer_register_deserialize_tagset (GtkTextBuffer                *buffer,
                                                       const gchar                  *tagset_name);

void    gtk_text_buffer_unregister_serialize_format   (GtkTextBuffer                *buffer,
                                                       GdkAtom                       format);
void    gtk_text_buffer_unregister_deserialize_format (GtkTextBuffer                *buffer,
                                                       GdkAtom                       format);

void     gtk_text_buffer_deserialize_set_can_create_tags (GtkTextBuffer             *buffer,
                                                          GdkAtom                    format,
                                                          gboolean                   can_create_tags);
gboolean gtk_text_buffer_deserialize_get_can_create_tags (GtkTextBuffer             *buffer,
                                                          GdkAtom                    format);

GdkAtom * gtk_text_buffer_get_serialize_formats       (GtkTextBuffer                *buffer,
                                                       gint                         *n_formats);
GdkAtom * gtk_text_buffer_get_deserialize_formats     (GtkTextBuffer                *buffer,
                                                       gint                         *n_formats);

guint8  * gtk_text_buffer_serialize                   (GtkTextBuffer                *register_buffer,
                                                       GtkTextBuffer                *content_buffer,
                                                       GdkAtom                       format,
                                                       const GtkTextIter            *start,
                                                       const GtkTextIter            *end,
                                                       gsize                        *length);
gboolean  gtk_text_buffer_deserialize                 (GtkTextBuffer                *register_buffer,
                                                       GtkTextBuffer                *content_buffer,
                                                       GdkAtom                       format,
                                                       GtkTextIter                  *iter,
                                                       const guint8                 *data,
                                                       gsize                         length,
                                                       GError                      **error);

G_END_DECLS

#endif /* __GTK_TEXT_BUFFER_RICH_TEXT_H__ */
