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
typedef struct _GtkFileFilterInfo GtkFileFilterInfo;

/**
 * GtkFileFilterFlags:
 * @GTK_FILE_FILTER_FILENAME: the filename of the file being tested
 * @GTK_FILE_FILTER_URI: the URI for the file being tested
 * @GTK_FILE_FILTER_DISPLAY_NAME: the string that will be used to 
 *   display the file in the file chooser
 * @GTK_FILE_FILTER_MIME_TYPE: the mime type of the file
 * 
 * These flags indicate what parts of a #GtkFileFilterInfo struct
 * are filled or need to be filled. 
 */
typedef enum {
  GTK_FILE_FILTER_FILENAME     = 1 << 0,
  GTK_FILE_FILTER_URI          = 1 << 1,
  GTK_FILE_FILTER_DISPLAY_NAME = 1 << 2,
  GTK_FILE_FILTER_MIME_TYPE    = 1 << 3
} GtkFileFilterFlags;

/**
 * GtkFileFilterFunc:
 * @filter_info: a #GtkFileFilterInfo that is filled according
 *   to the @needed flags passed to gtk_file_filter_add_custom()
 * @data: (closure): user data passed to gtk_file_filter_add_custom()
 *
 * The type of function that is used with custom filters, see
 * gtk_file_filter_add_custom().
 *
 * Returns: %TRUE if the file should be displayed
 */
typedef gboolean (*GtkFileFilterFunc) (const GtkFileFilterInfo *filter_info,
				       gpointer                 data);

/**
 * GtkFileFilterInfo:
 * @contains: Flags indicating which of the following fields need
 *   are filled
 * @filename: the filename of the file being tested
 * @uri: the URI for the file being tested
 * @display_name: the string that will be used to display the file
 *   in the file chooser
 * @mime_type: the mime type of the file
 *
 * A #GtkFileFilterInfo-struct is used to pass information about the
 * tested file to gtk_file_filter_filter().
 */
struct _GtkFileFilterInfo
{
  GtkFileFilterFlags contains;

  const gchar *filename;
  const gchar *uri;
  const gchar *display_name;
  const gchar *mime_type;
};

GDK_AVAILABLE_IN_ALL
GType gtk_file_filter_get_type (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
GtkFileFilter *       gtk_file_filter_new      (void);
GDK_AVAILABLE_IN_ALL
void                  gtk_file_filter_set_name (GtkFileFilter *filter,
						const gchar   *name);
GDK_AVAILABLE_IN_ALL
const gchar *         gtk_file_filter_get_name (GtkFileFilter *filter);

GDK_AVAILABLE_IN_ALL
void gtk_file_filter_add_mime_type      (GtkFileFilter      *filter,
					 const gchar        *mime_type);
GDK_AVAILABLE_IN_ALL
void gtk_file_filter_add_pattern        (GtkFileFilter      *filter,
					 const gchar        *pattern);
GDK_AVAILABLE_IN_ALL
void gtk_file_filter_add_pixbuf_formats (GtkFileFilter      *filter);
GDK_AVAILABLE_IN_ALL
void gtk_file_filter_add_custom         (GtkFileFilter      *filter,
					 GtkFileFilterFlags  needed,
					 GtkFileFilterFunc   func,
					 gpointer            data,
					 GDestroyNotify      notify);

GDK_AVAILABLE_IN_ALL
GtkFileFilterFlags gtk_file_filter_get_needed (GtkFileFilter           *filter);
GDK_AVAILABLE_IN_ALL
gboolean           gtk_file_filter_filter     (GtkFileFilter           *filter,
					       const GtkFileFilterInfo *filter_info);

GDK_AVAILABLE_IN_3_22
GVariant      *gtk_file_filter_to_gvariant       (GtkFileFilter *filter);
GDK_AVAILABLE_IN_3_22
GtkFileFilter *gtk_file_filter_new_from_gvariant (GVariant      *variant);

G_END_DECLS

#endif /* __GTK_FILE_FILTER_H__ */
