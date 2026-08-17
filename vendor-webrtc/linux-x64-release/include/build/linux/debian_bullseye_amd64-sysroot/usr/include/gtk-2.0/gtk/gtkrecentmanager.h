/* GTK - The GIMP Toolkit
 * gtkrecentmanager.h: a manager for the recently used resources
 *
 * Copyright (C) 2006 Emmanuele Bassi
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
 */

#ifndef __GTK_RECENT_MANAGER_H__
#define __GTK_RECENT_MANAGER_H__

#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gdk-pixbuf/gdk-pixbuf.h>
#include <gdk/gdk.h>
#include <time.h>

G_BEGIN_DECLS

#define GTK_TYPE_RECENT_INFO			(gtk_recent_info_get_type ())

#define GTK_TYPE_RECENT_MANAGER			(gtk_recent_manager_get_type ())
#define GTK_RECENT_MANAGER(obj)			(G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_RECENT_MANAGER, GtkRecentManager))
#define GTK_IS_RECENT_MANAGER(obj)		(G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_RECENT_MANAGER))
#define GTK_RECENT_MANAGER_CLASS(klass) 	(G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_RECENT_MANAGER, GtkRecentManagerClass))
#define GTK_IS_RECENT_MANAGER_CLASS(klass)	(G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_RECENT_MANAGER))
#define GTK_RECENT_MANAGER_GET_CLASS(obj)	(G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_RECENT_MANAGER, GtkRecentManagerClass))

typedef struct _GtkRecentInfo		GtkRecentInfo;
typedef struct _GtkRecentData		GtkRecentData;
typedef struct _GtkRecentManager	GtkRecentManager;
typedef struct _GtkRecentManagerClass	GtkRecentManagerClass;
typedef struct _GtkRecentManagerPrivate GtkRecentManagerPrivate;

/**
 * GtkRecentData:
 * @display_name: a UTF-8 encoded string, containing the name of the recently
 *   used resource to be displayed, or %NULL;
 * @description: a UTF-8 encoded string, containing a short description of
 *   the resource, or %NULL;
 * @mime_type: the MIME type of the resource;
 * @app_name: the name of the application that is registering this recently
 *   used resource;
 * @app_exec: command line used to launch this resource; may contain the
 *   "&percnt;f" and "&percnt;u" escape characters which will be expanded
 *   to the resource file path and URI respectively when the command line
 *   is retrieved;
 * @groups: a vector of strings containing groups names;
 * @is_private: whether this resource should be displayed only by the
 *   applications that have registered it or not.
 *
 * Meta-data to be passed to gtk_recent_manager_add_full() when
 * registering a recently used resource.
 **/
struct _GtkRecentData
{
  gchar *display_name;
  gchar *description;

  gchar *mime_type;

  gchar *app_name;
  gchar *app_exec;

  gchar **groups;

  gboolean is_private;
};

struct _GtkRecentManager
{
  /*< private >*/
  GObject parent_instance;

  GtkRecentManagerPrivate *GSEAL (priv);
};

struct _GtkRecentManagerClass
{
  /*< private >*/
  GObjectClass parent_class;

  void (*changed) (GtkRecentManager *manager);

  /* padding for future expansion */
  void (*_gtk_recent1) (void);
  void (*_gtk_recent2) (void);
  void (*_gtk_recent3) (void);
  void (*_gtk_recent4) (void);
};

/**
 * GtkRecentManagerError:
 * @GTK_RECENT_MANAGER_ERROR_NOT_FOUND: the URI specified does not exists in
 *   the recently used resources list.
 * @GTK_RECENT_MANAGER_ERROR_INVALID_URI: the URI specified is not valid.
 * @GTK_RECENT_MANAGER_ERROR_INVALID_ENCODING: the supplied string is not
 *   UTF-8 encoded.
 * @GTK_RECENT_MANAGER_ERROR_NOT_REGISTERED: no application has registered
 *   the specified item.
 * @GTK_RECENT_MANAGER_ERROR_READ: failure while reading the recently used
 *   resources file.
 * @GTK_RECENT_MANAGER_ERROR_WRITE: failure while writing the recently used
 *   resources file.
 * @GTK_RECENT_MANAGER_ERROR_UNKNOWN: unspecified error.
 *
 * Error codes for GtkRecentManager operations
 **/
typedef enum
{
  GTK_RECENT_MANAGER_ERROR_NOT_FOUND,
  GTK_RECENT_MANAGER_ERROR_INVALID_URI,
  GTK_RECENT_MANAGER_ERROR_INVALID_ENCODING,
  GTK_RECENT_MANAGER_ERROR_NOT_REGISTERED,
  GTK_RECENT_MANAGER_ERROR_READ,
  GTK_RECENT_MANAGER_ERROR_WRITE,
  GTK_RECENT_MANAGER_ERROR_UNKNOWN
} GtkRecentManagerError;

#define GTK_RECENT_MANAGER_ERROR	(gtk_recent_manager_error_quark ())
GQuark 	gtk_recent_manager_error_quark (void);


GType 		  gtk_recent_manager_get_type       (void) G_GNUC_CONST;

GtkRecentManager *gtk_recent_manager_new            (void);
GtkRecentManager *gtk_recent_manager_get_default    (void);

#ifndef GTK_DISABLE_DEPRECATED
GtkRecentManager *gtk_recent_manager_get_for_screen (GdkScreen            *screen);
void              gtk_recent_manager_set_screen     (GtkRecentManager     *manager,
						     GdkScreen            *screen);
#endif

gboolean          gtk_recent_manager_add_item       (GtkRecentManager     *manager,
						     const gchar          *uri);
gboolean          gtk_recent_manager_add_full       (GtkRecentManager     *manager,
						     const gchar          *uri,
						     const GtkRecentData  *recent_data);
gboolean          gtk_recent_manager_remove_item    (GtkRecentManager     *manager,
						     const gchar          *uri,
						     GError              **error);
GtkRecentInfo *   gtk_recent_manager_lookup_item    (GtkRecentManager     *manager,
						     const gchar          *uri,
						     GError              **error);
gboolean          gtk_recent_manager_has_item       (GtkRecentManager     *manager,
						     const gchar          *uri);
gboolean          gtk_recent_manager_move_item      (GtkRecentManager     *manager,
						     const gchar          *uri,
						     const gchar          *new_uri,
						     GError              **error);
void              gtk_recent_manager_set_limit      (GtkRecentManager     *manager,
						     gint                  limit);
gint              gtk_recent_manager_get_limit      (GtkRecentManager     *manager);
GList *           gtk_recent_manager_get_items      (GtkRecentManager     *manager);
gint              gtk_recent_manager_purge_items    (GtkRecentManager     *manager,
						     GError              **error);


GType	              gtk_recent_info_get_type             (void) G_GNUC_CONST;

GtkRecentInfo *       gtk_recent_info_ref                  (GtkRecentInfo  *info);
void                  gtk_recent_info_unref                (GtkRecentInfo  *info);

const gchar *         gtk_recent_info_get_uri              (GtkRecentInfo  *info);
const gchar *         gtk_recent_info_get_display_name     (GtkRecentInfo  *info);
const gchar *         gtk_recent_info_get_description      (GtkRecentInfo  *info);
const gchar *         gtk_recent_info_get_mime_type        (GtkRecentInfo  *info);
time_t                gtk_recent_info_get_added            (GtkRecentInfo  *info);
time_t                gtk_recent_info_get_modified         (GtkRecentInfo  *info);
time_t                gtk_recent_info_get_visited          (GtkRecentInfo  *info);
gboolean              gtk_recent_info_get_private_hint     (GtkRecentInfo  *info);
gboolean              gtk_recent_info_get_application_info (GtkRecentInfo  *info,
							    const gchar    *app_name,
							    const gchar   **app_exec,
							    guint          *count,
							    time_t         *time_);
gchar **              gtk_recent_info_get_applications     (GtkRecentInfo  *info,
							    gsize          *length) G_GNUC_MALLOC;
gchar *               gtk_recent_info_last_application     (GtkRecentInfo  *info) G_GNUC_MALLOC;
gboolean              gtk_recent_info_has_application      (GtkRecentInfo  *info,
							    const gchar    *app_name);
gchar **              gtk_recent_info_get_groups           (GtkRecentInfo  *info,
							    gsize          *length) G_GNUC_MALLOC;
gboolean              gtk_recent_info_has_group            (GtkRecentInfo  *info,
							    const gchar    *group_name);
GdkPixbuf *           gtk_recent_info_get_icon             (GtkRecentInfo  *info,
							    gint            size);
gchar *               gtk_recent_info_get_short_name       (GtkRecentInfo  *info) G_GNUC_MALLOC;
gchar *               gtk_recent_info_get_uri_display      (GtkRecentInfo  *info) G_GNUC_MALLOC;
gint                  gtk_recent_info_get_age              (GtkRecentInfo  *info);
gboolean              gtk_recent_info_is_local             (GtkRecentInfo  *info);
gboolean              gtk_recent_info_exists               (GtkRecentInfo  *info);
gboolean              gtk_recent_info_match                (GtkRecentInfo  *info_a,
							    GtkRecentInfo  *info_b);

/* private */
void _gtk_recent_manager_sync (void);

G_END_DECLS

#endif /* __GTK_RECENT_MANAGER_H__ */
