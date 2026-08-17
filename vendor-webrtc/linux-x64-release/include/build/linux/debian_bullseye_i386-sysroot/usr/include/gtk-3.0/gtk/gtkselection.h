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

#ifndef __GTK_SELECTION_H__
#define __GTK_SELECTION_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkwidget.h>
#include <gtk/gtktextiter.h>

G_BEGIN_DECLS

typedef struct _GtkTargetPair GtkTargetPair;

/**
 * GtkTargetPair:
 * @target: #GdkAtom representation of the target type
 * @flags: #GtkTargetFlags for DND
 * @info: an application-assigned integer ID which will
 *     get passed as a parameter to e.g the #GtkWidget::selection-get
 *     signal. It allows the application to identify the target
 *     type without extensive string compares.
 *
 * A #GtkTargetPair is used to represent the same
 * information as a table of #GtkTargetEntry, but in
 * an efficient form.
 */
struct _GtkTargetPair
{
  GdkAtom   target;
  guint     flags;
  guint     info;
};

/**
 * GtkTargetList:
 *
 * A #GtkTargetList-struct is a reference counted list
 * of #GtkTargetPair and should be treated as
 * opaque.
 */
typedef struct _GtkTargetList  GtkTargetList;
typedef struct _GtkTargetEntry GtkTargetEntry;

#define GTK_TYPE_SELECTION_DATA (gtk_selection_data_get_type ())
#define GTK_TYPE_TARGET_LIST    (gtk_target_list_get_type ())

/**
 * GtkTargetFlags:
 * @GTK_TARGET_SAME_APP: If this is set, the target will only be selected
 *   for drags within a single application.
 * @GTK_TARGET_SAME_WIDGET: If this is set, the target will only be selected
 *   for drags within a single widget.
 * @GTK_TARGET_OTHER_APP: If this is set, the target will not be selected
 *   for drags within a single application.
 * @GTK_TARGET_OTHER_WIDGET: If this is set, the target will not be selected
 *   for drags withing a single widget.
 *
 * The #GtkTargetFlags enumeration is used to specify
 * constraints on a #GtkTargetEntry.
 */
typedef enum {
  GTK_TARGET_SAME_APP = 1 << 0,    /*< nick=same-app >*/
  GTK_TARGET_SAME_WIDGET = 1 << 1, /*< nick=same-widget >*/
  GTK_TARGET_OTHER_APP = 1 << 2,   /*< nick=other-app >*/
  GTK_TARGET_OTHER_WIDGET = 1 << 3 /*< nick=other-widget >*/
} GtkTargetFlags;

/**
 * GtkTargetEntry:
 * @target: a string representation of the target type
 * @flags: #GtkTargetFlags for DND
 * @info: an application-assigned integer ID which will
 *     get passed as a parameter to e.g the #GtkWidget::selection-get
 *     signal. It allows the application to identify the target
 *     type without extensive string compares.
 *
 * A #GtkTargetEntry represents a single type of
 * data than can be supplied for by a widget for a selection
 * or for supplied or received during drag-and-drop.
 */
struct _GtkTargetEntry
{
  gchar *target;
  guint  flags;
  guint  info;
};

GDK_AVAILABLE_IN_ALL
GType          gtk_target_list_get_type  (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
GtkTargetList *gtk_target_list_new       (const GtkTargetEntry *targets,
                                          guint                 ntargets);
GDK_AVAILABLE_IN_ALL
GtkTargetList *gtk_target_list_ref       (GtkTargetList  *list);
GDK_AVAILABLE_IN_ALL
void           gtk_target_list_unref     (GtkTargetList  *list);
GDK_AVAILABLE_IN_ALL
void           gtk_target_list_add       (GtkTargetList  *list,
                                          GdkAtom         target,
                                          guint           flags,
                                          guint           info);
GDK_AVAILABLE_IN_ALL
void           gtk_target_list_add_text_targets      (GtkTargetList  *list,
                                                      guint           info);
GDK_AVAILABLE_IN_ALL
void           gtk_target_list_add_rich_text_targets (GtkTargetList  *list,
                                                      guint           info,
                                                      gboolean        deserializable,
                                                      GtkTextBuffer  *buffer);
GDK_AVAILABLE_IN_ALL
void           gtk_target_list_add_image_targets     (GtkTargetList  *list,
                                                      guint           info,
                                                      gboolean        writable);
GDK_AVAILABLE_IN_ALL
void           gtk_target_list_add_uri_targets       (GtkTargetList  *list,
                                                      guint           info);
GDK_AVAILABLE_IN_ALL
void           gtk_target_list_add_table (GtkTargetList        *list,
                                          const GtkTargetEntry *targets,
                                          guint                 ntargets);
GDK_AVAILABLE_IN_ALL
void           gtk_target_list_remove    (GtkTargetList  *list,
                                          GdkAtom         target);
GDK_AVAILABLE_IN_ALL
gboolean       gtk_target_list_find      (GtkTargetList  *list,
                                          GdkAtom         target,
                                          guint          *info);

GDK_AVAILABLE_IN_ALL
GtkTargetEntry * gtk_target_table_new_from_list (GtkTargetList  *list,
                                                 gint           *n_targets);
GDK_AVAILABLE_IN_ALL
void             gtk_target_table_free          (GtkTargetEntry *targets,
                                                 gint            n_targets);

GDK_AVAILABLE_IN_ALL
gboolean gtk_selection_owner_set             (GtkWidget  *widget,
                                              GdkAtom     selection,
                                              guint32     time_);
GDK_AVAILABLE_IN_ALL
gboolean gtk_selection_owner_set_for_display (GdkDisplay *display,
                                              GtkWidget  *widget,
                                              GdkAtom     selection,
                                              guint32     time_);

GDK_AVAILABLE_IN_ALL
void     gtk_selection_add_target    (GtkWidget            *widget,
                                      GdkAtom               selection,
                                      GdkAtom               target,
                                      guint                 info);
GDK_AVAILABLE_IN_ALL
void     gtk_selection_add_targets   (GtkWidget            *widget,
                                      GdkAtom               selection,
                                      const GtkTargetEntry *targets,
                                      guint                 ntargets);
GDK_AVAILABLE_IN_ALL
void     gtk_selection_clear_targets (GtkWidget            *widget,
                                      GdkAtom               selection);
GDK_AVAILABLE_IN_ALL
gboolean gtk_selection_convert       (GtkWidget            *widget,
                                      GdkAtom               selection,
                                      GdkAtom               target,
                                      guint32               time_);
GDK_AVAILABLE_IN_ALL
void     gtk_selection_remove_all    (GtkWidget             *widget);

GDK_AVAILABLE_IN_ALL
GdkAtom       gtk_selection_data_get_selection (const GtkSelectionData *selection_data);
GDK_AVAILABLE_IN_ALL
GdkAtom       gtk_selection_data_get_target    (const GtkSelectionData *selection_data);
GDK_AVAILABLE_IN_ALL
GdkAtom       gtk_selection_data_get_data_type (const GtkSelectionData *selection_data);
GDK_AVAILABLE_IN_ALL
gint          gtk_selection_data_get_format    (const GtkSelectionData *selection_data);
GDK_AVAILABLE_IN_ALL
const guchar *gtk_selection_data_get_data      (const GtkSelectionData *selection_data);
GDK_AVAILABLE_IN_ALL
gint          gtk_selection_data_get_length    (const GtkSelectionData *selection_data);
GDK_AVAILABLE_IN_ALL
const guchar *gtk_selection_data_get_data_with_length
                                               (const GtkSelectionData *selection_data,
                                                gint                   *length);

GDK_AVAILABLE_IN_ALL
GdkDisplay   *gtk_selection_data_get_display   (const GtkSelectionData *selection_data);

GDK_AVAILABLE_IN_ALL
void     gtk_selection_data_set      (GtkSelectionData     *selection_data,
                                      GdkAtom               type,
                                      gint                  format,
                                      const guchar         *data,
                                      gint                  length);
GDK_AVAILABLE_IN_ALL
gboolean gtk_selection_data_set_text (GtkSelectionData     *selection_data,
                                      const gchar          *str,
                                      gint                  len);
GDK_AVAILABLE_IN_ALL
guchar * gtk_selection_data_get_text (const GtkSelectionData     *selection_data);
GDK_AVAILABLE_IN_ALL
gboolean gtk_selection_data_set_pixbuf   (GtkSelectionData  *selection_data,
                                          GdkPixbuf         *pixbuf);
GDK_AVAILABLE_IN_ALL
GdkPixbuf *gtk_selection_data_get_pixbuf (const GtkSelectionData  *selection_data);
GDK_AVAILABLE_IN_ALL
gboolean gtk_selection_data_set_uris (GtkSelectionData     *selection_data,
                                      gchar               **uris);
GDK_AVAILABLE_IN_ALL
gchar  **gtk_selection_data_get_uris (const GtkSelectionData     *selection_data);

GDK_AVAILABLE_IN_ALL
gboolean gtk_selection_data_get_targets          (const GtkSelectionData  *selection_data,
                                                  GdkAtom          **targets,
                                                  gint              *n_atoms);
GDK_AVAILABLE_IN_ALL
gboolean gtk_selection_data_targets_include_text (const GtkSelectionData  *selection_data);
GDK_AVAILABLE_IN_ALL
gboolean gtk_selection_data_targets_include_rich_text (const GtkSelectionData *selection_data,
                                                       GtkTextBuffer    *buffer);
GDK_AVAILABLE_IN_ALL
gboolean gtk_selection_data_targets_include_image (const GtkSelectionData  *selection_data,
                                                   gboolean           writable);
GDK_AVAILABLE_IN_ALL
gboolean gtk_selection_data_targets_include_uri  (const GtkSelectionData  *selection_data);
GDK_AVAILABLE_IN_ALL
gboolean gtk_targets_include_text                (GdkAtom       *targets,
                                                  gint           n_targets);
GDK_AVAILABLE_IN_ALL
gboolean gtk_targets_include_rich_text           (GdkAtom       *targets,
                                                  gint           n_targets,
                                                  GtkTextBuffer *buffer);
GDK_AVAILABLE_IN_ALL
gboolean gtk_targets_include_image               (GdkAtom       *targets,
                                                  gint           n_targets,
                                                  gboolean       writable);
GDK_AVAILABLE_IN_ALL
gboolean gtk_targets_include_uri                 (GdkAtom       *targets,
                                                  gint           n_targets);


GDK_AVAILABLE_IN_ALL
GType             gtk_selection_data_get_type (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
GtkSelectionData *gtk_selection_data_copy     (const GtkSelectionData *data);
GDK_AVAILABLE_IN_ALL
void              gtk_selection_data_free     (GtkSelectionData *data);

GDK_AVAILABLE_IN_ALL
GType             gtk_target_entry_get_type    (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
GtkTargetEntry   *gtk_target_entry_new        (const gchar    *target,
                                               guint           flags,
                                               guint           info);
GDK_AVAILABLE_IN_ALL
GtkTargetEntry   *gtk_target_entry_copy       (GtkTargetEntry *data);
GDK_AVAILABLE_IN_ALL
void              gtk_target_entry_free       (GtkTargetEntry *data);

G_END_DECLS

#endif /* __GTK_SELECTION_H__ */
