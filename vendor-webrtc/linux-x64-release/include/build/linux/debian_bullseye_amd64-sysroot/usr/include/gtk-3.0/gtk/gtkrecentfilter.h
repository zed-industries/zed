/* GTK - The GIMP Toolkit
 * gtkrecentfilter.h - Filter object for recently used resources
 * Copyright (C) 2006, Emmanuele Bassi
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

#ifndef __GTK_RECENT_FILTER_H__
#define __GTK_RECENT_FILTER_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <glib-object.h>
#include <gdk/gdk.h>

G_BEGIN_DECLS

#define GTK_TYPE_RECENT_FILTER		(gtk_recent_filter_get_type ())
#define GTK_RECENT_FILTER(obj)		(G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_RECENT_FILTER, GtkRecentFilter))
#define GTK_IS_RECENT_FILTER(obj)	(G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_RECENT_FILTER))

typedef struct _GtkRecentFilter		GtkRecentFilter;
typedef struct _GtkRecentFilterInfo	GtkRecentFilterInfo;

/**
 * GtkRecentFilterFlags:
 * @GTK_RECENT_FILTER_URI: the URI of the file being tested
 * @GTK_RECENT_FILTER_DISPLAY_NAME: the string that will be used to
 *  display the file in the recent chooser
 * @GTK_RECENT_FILTER_MIME_TYPE: the mime type of the file
 * @GTK_RECENT_FILTER_APPLICATION: the list of applications that have
 *  registered the file
 * @GTK_RECENT_FILTER_GROUP: the groups to which the file belongs to
 * @GTK_RECENT_FILTER_AGE: the number of days elapsed since the file
 *  has been registered
 *
 * These flags indicate what parts of a #GtkRecentFilterInfo struct
 * are filled or need to be filled.
 */
typedef enum {
  GTK_RECENT_FILTER_URI          = 1 << 0,
  GTK_RECENT_FILTER_DISPLAY_NAME = 1 << 1,
  GTK_RECENT_FILTER_MIME_TYPE    = 1 << 2,
  GTK_RECENT_FILTER_APPLICATION  = 1 << 3,
  GTK_RECENT_FILTER_GROUP        = 1 << 4,
  GTK_RECENT_FILTER_AGE          = 1 << 5
} GtkRecentFilterFlags;

/**
 * GtkRecentFilterFunc:
 * @filter_info: a #GtkRecentFilterInfo that is filled according
 *  to the @needed flags passed to gtk_recent_filter_add_custom()
 * @user_data: user data passed to gtk_recent_filter_add_custom()
 *
 * The type of function that is used with custom filters,
 * see gtk_recent_filter_add_custom().
 *
 * Returns: %TRUE if the file should be displayed
 */
typedef gboolean (*GtkRecentFilterFunc) (const GtkRecentFilterInfo *filter_info,
					 gpointer                   user_data);


/**
 * GtkRecentFilterInfo:
 * @contains: #GtkRecentFilterFlags to indicate which fields are set.
 * @uri: (nullable): The URI of the file being tested.
 * @display_name: (nullable): The string that will be used to display
 *    the file in the recent chooser.
 * @mime_type: (nullable): MIME type of the file.
 * @applications: (nullable) (array zero-terminated=1): The list of
 *    applications that have registered the file.
 * @groups: (nullable) (array zero-terminated=1): The groups to which
 *    the file belongs to.
 * @age: The number of days elapsed since the file has been
 *    registered.
 *
 * A GtkRecentFilterInfo struct is used
 * to pass information about the tested file to gtk_recent_filter_filter().
 */
struct _GtkRecentFilterInfo
{
  GtkRecentFilterFlags contains;

  const gchar *uri;
  const gchar *display_name;
  const gchar *mime_type;
  const gchar **applications;
  const gchar **groups;

  gint age;
};

GDK_AVAILABLE_IN_ALL
GType                 gtk_recent_filter_get_type (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
GtkRecentFilter *     gtk_recent_filter_new      (void);
GDK_AVAILABLE_IN_ALL
void                  gtk_recent_filter_set_name (GtkRecentFilter *filter,
						  const gchar     *name);
GDK_AVAILABLE_IN_ALL
const gchar *         gtk_recent_filter_get_name (GtkRecentFilter *filter);

GDK_AVAILABLE_IN_ALL
void gtk_recent_filter_add_mime_type      (GtkRecentFilter      *filter,
					   const gchar          *mime_type);
GDK_AVAILABLE_IN_ALL
void gtk_recent_filter_add_pattern        (GtkRecentFilter      *filter,
					   const gchar          *pattern);
GDK_AVAILABLE_IN_ALL
void gtk_recent_filter_add_pixbuf_formats (GtkRecentFilter      *filter);
GDK_AVAILABLE_IN_ALL
void gtk_recent_filter_add_application    (GtkRecentFilter      *filter,
					   const gchar          *application);
GDK_AVAILABLE_IN_ALL
void gtk_recent_filter_add_group          (GtkRecentFilter      *filter,
					   const gchar          *group);
GDK_AVAILABLE_IN_ALL
void gtk_recent_filter_add_age            (GtkRecentFilter      *filter,
					   gint                  days);
GDK_AVAILABLE_IN_ALL
void gtk_recent_filter_add_custom         (GtkRecentFilter      *filter,
					   GtkRecentFilterFlags  needed,
					   GtkRecentFilterFunc   func,
					   gpointer              data,
					   GDestroyNotify        data_destroy);

GDK_AVAILABLE_IN_ALL
GtkRecentFilterFlags gtk_recent_filter_get_needed (GtkRecentFilter           *filter);
GDK_AVAILABLE_IN_ALL
gboolean             gtk_recent_filter_filter     (GtkRecentFilter           *filter,
						   const GtkRecentFilterInfo *filter_info);

G_END_DECLS

#endif /* ! __GTK_RECENT_FILTER_H__ */
