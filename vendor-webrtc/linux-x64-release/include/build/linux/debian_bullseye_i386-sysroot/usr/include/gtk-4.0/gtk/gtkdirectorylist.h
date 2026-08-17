/*
 * Copyright © 2019 Benjamin Otte
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

#ifndef __GTK_DIRECTORY_LIST_H__
#define __GTK_DIRECTORY_LIST_H__


#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gio/gio.h>
/* for GDK_AVAILABLE_IN_ALL */
#include <gdk/gdk.h>


G_BEGIN_DECLS

#define GTK_TYPE_DIRECTORY_LIST (gtk_directory_list_get_type ())

GDK_AVAILABLE_IN_ALL
G_DECLARE_FINAL_TYPE (GtkDirectoryList, gtk_directory_list, GTK, DIRECTORY_LIST, GObject)

GDK_AVAILABLE_IN_ALL
GtkDirectoryList *      gtk_directory_list_new                  (const char             *attributes,
                                                                 GFile                  *file);

GDK_AVAILABLE_IN_ALL
void                    gtk_directory_list_set_file             (GtkDirectoryList       *self,
                                                                 GFile                  *file);
GDK_AVAILABLE_IN_ALL
GFile *                 gtk_directory_list_get_file             (GtkDirectoryList       *self);
GDK_AVAILABLE_IN_ALL
void                    gtk_directory_list_set_attributes       (GtkDirectoryList       *self,
                                                                 const char             *attributes);
GDK_AVAILABLE_IN_ALL
const char *            gtk_directory_list_get_attributes       (GtkDirectoryList       *self);
GDK_AVAILABLE_IN_ALL
void                    gtk_directory_list_set_io_priority      (GtkDirectoryList       *self,
                                                                 int                     io_priority);
GDK_AVAILABLE_IN_ALL
int                     gtk_directory_list_get_io_priority      (GtkDirectoryList       *self);

GDK_AVAILABLE_IN_ALL
gboolean                gtk_directory_list_is_loading           (GtkDirectoryList       *self);
GDK_AVAILABLE_IN_ALL
const GError *          gtk_directory_list_get_error            (GtkDirectoryList       *self);

GDK_AVAILABLE_IN_ALL
void                    gtk_directory_list_set_monitored        (GtkDirectoryList       *self,
                                                                 gboolean                monitored);
GDK_AVAILABLE_IN_ALL
gboolean                gtk_directory_list_get_monitored        (GtkDirectoryList       *self);

G_END_DECLS

#endif /* __GTK_DIRECTORY_LIST_H__ */
