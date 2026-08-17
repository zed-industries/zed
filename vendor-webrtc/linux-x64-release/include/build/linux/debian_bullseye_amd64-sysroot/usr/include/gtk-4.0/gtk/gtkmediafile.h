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

#ifndef __GTK_MEDIA_FILE_H__
#define __GTK_MEDIA_FILE_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkmediastream.h>

G_BEGIN_DECLS

#define GTK_MEDIA_FILE_EXTENSION_POINT_NAME "gtk-media-file"

#define GTK_TYPE_MEDIA_FILE             (gtk_media_file_get_type ())

GDK_AVAILABLE_IN_ALL
G_DECLARE_DERIVABLE_TYPE (GtkMediaFile, gtk_media_file, GTK, MEDIA_FILE, GtkMediaStream)

struct _GtkMediaFileClass
{
  GtkMediaStreamClass parent_class;

  void                  (* open)                                (GtkMediaFile *self);
  void                  (* close)                               (GtkMediaFile *self);

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};

GDK_AVAILABLE_IN_ALL
GtkMediaStream *        gtk_media_file_new                      (void);
GDK_AVAILABLE_IN_ALL
GtkMediaStream *        gtk_media_file_new_for_filename         (const char     *filename);
GDK_AVAILABLE_IN_ALL
GtkMediaStream *        gtk_media_file_new_for_resource         (const char     *resource_path);
GDK_AVAILABLE_IN_ALL
GtkMediaStream *        gtk_media_file_new_for_file             (GFile          *file);
GDK_AVAILABLE_IN_ALL
GtkMediaStream *        gtk_media_file_new_for_input_stream     (GInputStream   *stream);

GDK_AVAILABLE_IN_ALL
void                    gtk_media_file_clear                    (GtkMediaFile   *self);

GDK_AVAILABLE_IN_ALL
void                    gtk_media_file_set_filename             (GtkMediaFile   *self,
                                                                 const char     *filename);
GDK_AVAILABLE_IN_ALL
void                    gtk_media_file_set_resource             (GtkMediaFile   *self,
                                                                 const char     *resource_path);
GDK_AVAILABLE_IN_ALL
void                    gtk_media_file_set_file                 (GtkMediaFile   *self,
                                                                 GFile          *file);
GDK_AVAILABLE_IN_ALL
GFile *                 gtk_media_file_get_file                 (GtkMediaFile   *self);
GDK_AVAILABLE_IN_ALL
void                    gtk_media_file_set_input_stream         (GtkMediaFile   *self,
                                                                 GInputStream   *stream);
GDK_AVAILABLE_IN_ALL
GInputStream *          gtk_media_file_get_input_stream         (GtkMediaFile   *self);


G_END_DECLS

#endif /* __GTK_MEDIA_FILE_H__ */
