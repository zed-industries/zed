/* GTK - The GIMP Toolkit
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

#ifndef __GTK_IMAGE_H__
#define __GTK_IMAGE_H__


#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gio/gio.h>
#include <gtk/gtkwidget.h>


G_BEGIN_DECLS

#define GTK_TYPE_IMAGE                  (gtk_image_get_type ())
#define GTK_IMAGE(obj)                  (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_IMAGE, GtkImage))
#define GTK_IS_IMAGE(obj)               (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_IMAGE))

typedef struct _GtkImage              GtkImage;

/**
 * GtkImageType:
 * @GTK_IMAGE_EMPTY: there is no image displayed by the widget
 * @GTK_IMAGE_ICON_NAME: the widget contains a named icon
 * @GTK_IMAGE_GICON: the widget contains a `GIcon`
 * @GTK_IMAGE_PAINTABLE: the widget contains a `GdkPaintable`
 *
 * Describes the image data representation used by a [class@Gtk.Image].
 *
 * If you want to get the image from the widget, you can only get the
 * currently-stored representation; for instance, if the gtk_image_get_storage_type()
 * returns %GTK_IMAGE_PAINTABLE, then you can call gtk_image_get_paintable().
 *
 * For empty images, you can request any storage type (call any of the "get"
 * functions), but they will all return %NULL values.
 */
typedef enum
{
  GTK_IMAGE_EMPTY,
  GTK_IMAGE_ICON_NAME,
  GTK_IMAGE_GICON,
  GTK_IMAGE_PAINTABLE
} GtkImageType;

GDK_AVAILABLE_IN_ALL
GType      gtk_image_get_type (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
GtkWidget* gtk_image_new                (void);
GDK_AVAILABLE_IN_ALL
GtkWidget* gtk_image_new_from_file      (const char      *filename);
GDK_AVAILABLE_IN_ALL
GtkWidget* gtk_image_new_from_resource  (const char      *resource_path);
GDK_AVAILABLE_IN_ALL
GtkWidget* gtk_image_new_from_pixbuf    (GdkPixbuf       *pixbuf);
GDK_AVAILABLE_IN_ALL
GtkWidget* gtk_image_new_from_paintable (GdkPaintable    *paintable);
GDK_AVAILABLE_IN_ALL
GtkWidget* gtk_image_new_from_icon_name (const char      *icon_name);
GDK_AVAILABLE_IN_ALL
GtkWidget* gtk_image_new_from_gicon     (GIcon           *icon);

GDK_AVAILABLE_IN_ALL
void gtk_image_clear              (GtkImage        *image);
GDK_AVAILABLE_IN_ALL
void gtk_image_set_from_file      (GtkImage        *image,
                                   const char      *filename);
GDK_AVAILABLE_IN_ALL
void gtk_image_set_from_resource  (GtkImage        *image,
                                   const char      *resource_path);
GDK_AVAILABLE_IN_ALL
void gtk_image_set_from_pixbuf    (GtkImage        *image,
                                   GdkPixbuf       *pixbuf);
GDK_AVAILABLE_IN_ALL
void gtk_image_set_from_paintable (GtkImage        *image,
                                   GdkPaintable    *paintable);
GDK_AVAILABLE_IN_ALL
void gtk_image_set_from_icon_name (GtkImage        *image,
				   const char      *icon_name);
GDK_AVAILABLE_IN_ALL
void gtk_image_set_from_gicon     (GtkImage        *image,
				   GIcon           *icon);
GDK_AVAILABLE_IN_ALL
void gtk_image_set_pixel_size     (GtkImage        *image,
				   int              pixel_size);
GDK_AVAILABLE_IN_ALL
void gtk_image_set_icon_size      (GtkImage        *image,
                                   GtkIconSize      icon_size);

GDK_AVAILABLE_IN_ALL
GtkImageType gtk_image_get_storage_type (GtkImage   *image);

GDK_AVAILABLE_IN_ALL
GdkPaintable *gtk_image_get_paintable (GtkImage       *image);

GDK_AVAILABLE_IN_ALL
const char *gtk_image_get_icon_name (GtkImage     *image);
GDK_AVAILABLE_IN_ALL
GIcon *    gtk_image_get_gicon     (GtkImage              *image);
GDK_AVAILABLE_IN_ALL
int        gtk_image_get_pixel_size (GtkImage             *image);
GDK_AVAILABLE_IN_ALL
GtkIconSize gtk_image_get_icon_size (GtkImage             *image);

G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkImage, g_object_unref)

G_END_DECLS

#endif /* __GTK_IMAGE_H__ */
