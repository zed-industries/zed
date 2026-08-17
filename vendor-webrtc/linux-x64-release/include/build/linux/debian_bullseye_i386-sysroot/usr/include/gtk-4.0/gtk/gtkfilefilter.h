/* GTK - The GIMP Toolkit
 * gtkfilefilter.h: Filters for selecting a file subset
 * Copyright (C) 2003, Red Hat, Inc.
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

#ifndef __GTK_FILE_FILTER_H__
#define __GTK_FILE_FILTER_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <glib-object.h>
#include <gdk/gdk.h>

G_BEGIN_DECLS

#define GTK_TYPE_FILE_FILTER              (gtk_file_filter_get_type ())
#define GTK_FILE_FILTER(obj)              (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_FILE_FILTER, GtkFileFilter))
#define GTK_IS_FILE_FILTER(obj)           (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_FILE_FILTER))

typedef struct _GtkFileFilter     GtkFileFilter;

GDK_AVAILABLE_IN_ALL
GType           gtk_file_filter_get_type           (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
GtkFileFilter * gtk_file_filter_new                (void);
GDK_AVAILABLE_IN_ALL
void            gtk_file_filter_set_name           (GtkFileFilter *filter,
                                                    const char    *name);
GDK_AVAILABLE_IN_ALL
const char *    gtk_file_filter_get_name           (GtkFileFilter *filter);

GDK_AVAILABLE_IN_ALL
void            gtk_file_filter_add_mime_type      (GtkFileFilter *filter,
                                                    const char    *mime_type);

GDK_AVAILABLE_IN_ALL
void            gtk_file_filter_add_pattern        (GtkFileFilter *filter,
                                                    const char    *pattern);

GDK_AVAILABLE_IN_4_4
void            gtk_file_filter_add_suffix         (GtkFileFilter *filter,
                                                    const char    *suffix);

GDK_AVAILABLE_IN_ALL
void            gtk_file_filter_add_pixbuf_formats (GtkFileFilter *filter);

GDK_AVAILABLE_IN_ALL
const char **   gtk_file_filter_get_attributes     (GtkFileFilter *filter);

GDK_AVAILABLE_IN_ALL
GVariant      * gtk_file_filter_to_gvariant        (GtkFileFilter *filter);
GDK_AVAILABLE_IN_ALL
GtkFileFilter * gtk_file_filter_new_from_gvariant  (GVariant      *variant);

G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkFileFilter, g_object_unref)

G_END_DECLS

#endif /* __GTK_FILE_FILTER_H__ */
