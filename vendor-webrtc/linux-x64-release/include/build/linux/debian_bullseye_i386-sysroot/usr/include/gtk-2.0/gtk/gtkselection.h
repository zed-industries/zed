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
 * License along with this library; if not, write to the
 * Free Software Foundation, Inc., 59 Temple Place - Suite 330,
 * Boston, MA 02111-1307, USA.
 */

/*
 * Modified by the GTK+ Team and others 1997-2000.  See the AUTHORS
 * file for a list of people on the GTK+ Team.  See the ChangeLog
 * files for a list of changes.  These files are distributed with
 * GTK+ at ftp://ftp.gtk.org/pub/gtk/.
 */

#ifndef __GTK_SELECTION_H__
#define __GTK_SELECTION_H__


#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkwidget.h>
#include <gtk/gtktextiter.h>


G_BEGIN_DECLS

typedef struct _GtkTargetList    GtkTargetList;
typedef struct _GtkTargetEntry   GtkTargetEntry;

#define GTK_TYPE_SELECTION_DATA (gtk_selection_data_get_type ())
#define GTK_TYPE_TARGET_LIST    (gtk_target_list_get_type ())

/* The contents of a selection are returned in a GtkSelectionData
 * structure. selection/target identify the request.  type specifies
 * the type of the return; if length < 0, and the data should be
 * ignored. This structure has object semantics - no fields should be
 * modified directly, they should not be created directly, and
 * pointers to them should not be stored beyond the duration of a
 * callback. (If the last is changed, we'll need to add reference
 * counting.) The time field gives the timestamp at which the data was
 * sent.
 */

struct _GtkSelectionData
{
  GdkAtom       GSEAL (selection);
  GdkAtom       GSEAL (target);
  GdkAtom       GSEAL (type);
  gint          GSEAL (format);
  guchar       *GSEAL (data);
  gint          GSEAL (length);
  GdkDisplay   *GSEAL (display);
};

struct _GtkTargetEntry {
  gchar *target;
  guint  flags;
  guint  info;
};

/* These structures not public, and are here only for the convenience of
 * gtkdnd.c
 */

typedef struct _GtkTargetPair GtkTargetPair;

/* This structure is a list of destinations, and associated guint id's */
struct _GtkTargetList {
  GList *list;
  guint ref_count;
};

struct _GtkTargetPair {
  GdkAtom   target;
  guint     flags;
  guint     info;
};

GtkTargetList *gtk_target_list_new       (const GtkTargetEntry *targets,
					  guint                 ntargets);
GtkTargetList *gtk_target_list_ref       (GtkTargetList  *list);
void           gtk_target_list_unref     (GtkTargetList  *list);
void           gtk_target_list_add       (GtkTargetList  *list,
				  	  GdkAtom         target,
					  guint           flags,
					  guint           info);
void           gtk_target_list_add_text_targets      (GtkTargetList  *list,
                                                      guint           info);
void           gtk_target_list_add_rich_text_targets (GtkTargetList  *list,
                                                      guint           info,
                                                      gboolean        deserializable,
                                                      GtkTextBuffer  *buffer);
void           gtk_target_list_add_image_targets     (GtkTargetList  *list,
                                                      guint           info,
                                                      gboolean        writable);
void           gtk_target_list_add_uri_targets       (GtkTargetList  *list,
                                                      guint           info);
void           gtk_target_list_add_table (GtkTargetList        *list,
					  const GtkTargetEntry *targets,
					  guint                 ntargets);
void           gtk_target_list_remove    (GtkTargetList  *list,
					  GdkAtom         target);
gboolean       gtk_target_list_find      (GtkTargetList  *list,
					  GdkAtom         target,
					  guint          *info);

GtkTargetEntry * gtk_target_table_new_from_list (GtkTargetList  *list,
                                                 gint           *n_targets);
void             gtk_target_table_free          (GtkTargetEntry *targets,
                                                 gint            n_targets);

/* Public interface */

gboolean gtk_selection_owner_set             (GtkWidget  *widget,
					      GdkAtom     selection,
					      guint32     time_);
gboolean gtk_selection_owner_set_for_display (GdkDisplay *display,
					      GtkWidget  *widget,
					      GdkAtom     selection,
					      guint32     time_);

void     gtk_selection_add_target    (GtkWidget            *widget,
				      GdkAtom               selection,
				      GdkAtom               target,
				      guint                 info);
void     gtk_selection_add_targets   (GtkWidget            *widget,
				      GdkAtom               selection,
				      const GtkTargetEntry *targets,
				      guint                 ntargets);
void     gtk_selection_clear_targets (GtkWidget            *widget,
				      GdkAtom               selection);
gboolean gtk_selection_convert       (GtkWidget            *widget,
				      GdkAtom               selection,
				      GdkAtom               target,
				      guint32               time_);

GdkAtom       gtk_selection_data_get_selection (GtkSelectionData *selection_data);
GdkAtom       gtk_selection_data_get_target    (GtkSelectionData *selection_data);
GdkAtom       gtk_selection_data_get_data_type (GtkSelectionData *selection_data);
gint          gtk_selection_data_get_format    (GtkSelectionData *selection_data);
const guchar *gtk_selection_data_get_data      (GtkSelectionData *selection_data);
gint          gtk_selection_data_get_length    (GtkSelectionData *selection_data);
GdkDisplay   *gtk_selection_data_get_display   (GtkSelectionData *selection_data);

void     gtk_selection_data_set      (GtkSelectionData     *selection_data,
				      GdkAtom               type,
				      gint                  format,
				      const guchar         *data,
				      gint                  length);
gboolean gtk_selection_data_set_text (GtkSelectionData     *selection_data,
				      const gchar          *str,
				      gint                  len);
guchar * gtk_selection_data_get_text (GtkSelectionData     *selection_data);
gboolean gtk_selection_data_set_pixbuf   (GtkSelectionData  *selection_data,
				          GdkPixbuf         *pixbuf);
GdkPixbuf *gtk_selection_data_get_pixbuf (GtkSelectionData  *selection_data);
gboolean gtk_selection_data_set_uris (GtkSelectionData     *selection_data,
				      gchar               **uris);
gchar  **gtk_selection_data_get_uris (GtkSelectionData     *selection_data);

gboolean gtk_selection_data_get_targets          (GtkSelectionData  *selection_data,
						  GdkAtom          **targets,
						  gint              *n_atoms);
gboolean gtk_selection_data_targets_include_text (GtkSelectionData  *selection_data);
gboolean gtk_selection_data_targets_include_rich_text (GtkSelectionData *selection_data,
                                                       GtkTextBuffer    *buffer);
gboolean gtk_selection_data_targets_include_image (GtkSelectionData  *selection_data,
						   gboolean           writable);
gboolean gtk_selection_data_targets_include_uri  (GtkSelectionData  *selection_data);
gboolean gtk_targets_include_text                (GdkAtom       *targets,
						  gint           n_targets);
gboolean gtk_targets_include_rich_text           (GdkAtom       *targets,
						  gint           n_targets,
                                                  GtkTextBuffer *buffer);
gboolean gtk_targets_include_image               (GdkAtom       *targets,
						  gint           n_targets,
						  gboolean       writable);
gboolean gtk_targets_include_uri                 (GdkAtom       *targets,
						  gint           n_targets);

/* Called when a widget is destroyed */

void gtk_selection_remove_all      (GtkWidget *widget);

/* Event handlers */
#if !defined(GTK_DISABLE_DEPRECATED) || defined (GTK_COMPILATION)
gboolean gtk_selection_clear		  (GtkWidget 	     *widget,
					   GdkEventSelection *event);
#endif
gboolean _gtk_selection_request		  (GtkWidget  	     *widget,
					   GdkEventSelection *event);
gboolean _gtk_selection_incr_event	  (GdkWindow         *window,
					   GdkEventProperty  *event);
gboolean _gtk_selection_notify		  (GtkWidget         *widget,
					   GdkEventSelection *event);
gboolean _gtk_selection_property_notify	  (GtkWidget         *widget,
					   GdkEventProperty  *event);

GType             gtk_selection_data_get_type (void) G_GNUC_CONST;
GtkSelectionData *gtk_selection_data_copy     (GtkSelectionData *data);
void		  gtk_selection_data_free     (GtkSelectionData *data);

GType             gtk_target_list_get_type    (void) G_GNUC_CONST;

G_END_DECLS

#endif /* __GTK_SELECTION_H__ */
