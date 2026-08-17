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

#ifndef __GTK_PICTURE_H__
#define __GTK_PICTURE_H__


#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gio/gio.h>
#include <gtk/gtkwidget.h>


G_BEGIN_DECLS

#define GTK_TYPE_PICTURE (gtk_picture_get_type ())

GDK_AVAILABLE_IN_ALL
G_DECLARE_FINAL_TYPE (GtkPicture, gtk_picture, GTK, PICTURE, GtkWidget)

GDK_AVAILABLE_IN_ALL
GtkWidget*      gtk_picture_new                         (void);
GDK_AVAILABLE_IN_ALL
GtkWidget*      gtk_picture_new_for_paintable           (GdkPaintable           *paintable);
GDK_AVAILABLE_IN_ALL
GtkWidget*      gtk_picture_new_for_pixbuf              (GdkPixbuf              *pixbuf);
GDK_AVAILABLE_IN_ALL
GtkWidget*      gtk_picture_new_for_file                (GFile                  *file);
GDK_AVAILABLE_IN_ALL
GtkWidget*      gtk_picture_new_for_filename            (const char             *filename);
GDK_AVAILABLE_IN_ALL
GtkWidget*      gtk_picture_new_for_resource            (const char             *resource_path);

GDK_AVAILABLE_IN_ALL
void            gtk_picture_set_paintable               (GtkPicture             *self,
                                                         GdkPaintable           *paintable);
GDK_AVAILABLE_IN_ALL
GdkPaintable *  gtk_picture_get_paintable               (GtkPicture             *self);
GDK_AVAILABLE_IN_ALL
void            gtk_picture_set_file                    (GtkPicture             *self,
                                                         GFile                  *file);
GDK_AVAILABLE_IN_ALL
GFile *         gtk_picture_get_file                    (GtkPicture             *self);
GDK_AVAILABLE_IN_ALL
void            gtk_picture_set_filename                (GtkPicture             *self,
                                                         const char             *filename);
GDK_AVAILABLE_IN_ALL
void            gtk_picture_set_resource                (GtkPicture             *self,
                                                         const char             *resource_path);
GDK_AVAILABLE_IN_ALL
void            gtk_picture_set_pixbuf                  (GtkPicture             *self,
                                                         GdkPixbuf              *pixbuf);

GDK_DEPRECATED_IN_4_8_FOR(gtk_picture_set_content_fit)
void            gtk_picture_set_keep_aspect_ratio       (GtkPicture             *self,
                                                         gboolean                keep_aspect_ratio);
GDK_DEPRECATED_IN_4_8_FOR(gtk_picture_get_content_fit)
gboolean        gtk_picture_get_keep_aspect_ratio       (GtkPicture             *self);
GDK_AVAILABLE_IN_ALL
void            gtk_picture_set_can_shrink              (GtkPicture             *self,
                                                         gboolean                can_shrink);
GDK_AVAILABLE_IN_ALL
gboolean        gtk_picture_get_can_shrink              (GtkPicture             *self);

GDK_AVAILABLE_IN_4_8
void            gtk_picture_set_content_fit             (GtkPicture             *self,
                                                         GtkContentFit           content_fit);
GDK_AVAILABLE_IN_4_8
GtkContentFit   gtk_picture_get_content_fit             (GtkPicture             *self);

GDK_AVAILABLE_IN_ALL
void            gtk_picture_set_alternative_text        (GtkPicture             *self,
                                                         const char             *alternative_text);
GDK_AVAILABLE_IN_ALL
const char *    gtk_picture_get_alternative_text        (GtkPicture             *self);


G_END_DECLS

#endif /* __GTK_PICTURE_H__ */
