/* gtktreeselection.h
 * Copyright (C) 2000  Red Hat, Inc.,  Jonathan Blandford <jrb@redhat.com>
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
 * License along with this library. If not, see <http://www.gnu.org/licenses/>.
 */

#ifndef __GTK_TREE_SELECTION_H__
#define __GTK_TREE_SELECTION_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtktreeview.h>

G_BEGIN_DECLS


#define GTK_TYPE_TREE_SELECTION			(gtk_tree_selection_get_type ())
#define GTK_TREE_SELECTION(obj)			(G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_TREE_SELECTION, GtkTreeSelection))
#define GTK_TREE_SELECTION_CLASS(klass)		(G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_TREE_SELECTION, GtkTreeSelectionClass))
#define GTK_IS_TREE_SELECTION(obj)		(G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_TREE_SELECTION))
#define GTK_IS_TREE_SELECTION_CLASS(klass)	(G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_TREE_SELECTION))
#define GTK_TREE_SELECTION_GET_CLASS(obj)	(G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_TREE_SELECTION, GtkTreeSelectionClass))

typedef struct _GtkTreeSelectionPrivate      GtkTreeSelectionPrivate;

/**
 * GtkTreeSelectionFunc:
 * @selection: A #GtkTreeSelection
 * @model: A #GtkTreeModel being viewed
 * @path: The #GtkTreePath of the row in question
 * @path_currently_selected: %TRUE, if the path is currently selected
 * @data: (closure): user data
 *
 * A function used by gtk_tree_selection_set_select_function() to filter
 * whether or not a row may be selected.  It is called whenever a row's
 * state might change.  A return value of %TRUE indicates to @selection
 * that it is okay to change the selection.
 *
 * Returns: %TRUE, if the selection state of the row can be toggled
 */
typedef gboolean (* GtkTreeSelectionFunc)    (GtkTreeSelection  *selection,
					      GtkTreeModel      *model,
					      GtkTreePath       *path,
                                              gboolean           path_currently_selected,
					      gpointer           data);

/**
 * GtkTreeSelectionForeachFunc:
 * @model: The #GtkTreeModel being viewed
 * @path: The #GtkTreePath of a selected row
 * @iter: A #GtkTreeIter pointing to a selected row
 * @data: (closure): user data
 *
 * A function used by gtk_tree_selection_selected_foreach() to map all
 * selected rows.  It will be called on every selected row in the view.
 */
typedef void (* GtkTreeSelectionForeachFunc) (GtkTreeModel      *model,
					      GtkTreePath       *path,
					      GtkTreeIter       *iter,
					      gpointer           data);

struct _GtkTreeSelection
{
  /*< private >*/
  GObject parent;

  GtkTreeSelectionPrivate *priv;
};

/**
 * GtkTreeSelectionClass:
 * @parent_class: The parent class.
 * @changed: Signal emitted whenever the selection has (possibly) changed.
 */
struct _GtkTreeSelectionClass
{
  GObjectClass parent_class;

  /*< public >*/

  void (* changed) (GtkTreeSelection *selection);

  /*< private >*/

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};


GDK_AVAILABLE_IN_ALL
GType            gtk_tree_selection_get_type            (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
void             gtk_tree_selection_set_mode            (GtkTreeSelection            *selection,
							 GtkSelectionMode             type);
GDK_AVAILABLE_IN_ALL
GtkSelectionMode gtk_tree_selection_get_mode        (GtkTreeSelection            *selection);
GDK_AVAILABLE_IN_ALL
void             gtk_tree_selection_set_select_function (GtkTreeSelection            *selection,
							 GtkTreeSelectionFunc         func,
							 gpointer                     data,
							 GDestroyNotify               destroy);
GDK_AVAILABLE_IN_ALL
gpointer         gtk_tree_selection_get_user_data       (GtkTreeSelection            *selection);
GDK_AVAILABLE_IN_ALL
GtkTreeView*     gtk_tree_selection_get_tree_view       (GtkTreeSelection            *selection);

GDK_AVAILABLE_IN_ALL
GtkTreeSelectionFunc gtk_tree_selection_get_select_function (GtkTreeSelection        *selection);

/* Only meaningful if GTK_SELECTION_SINGLE or GTK_SELECTION_BROWSE is set */
/* Use selected_foreach or get_selected_rows for GTK_SELECTION_MULTIPLE */
GDK_AVAILABLE_IN_ALL
gboolean         gtk_tree_selection_get_selected        (GtkTreeSelection            *selection,
							 GtkTreeModel               **model,
							 GtkTreeIter                 *iter);
GDK_AVAILABLE_IN_ALL
GList *          gtk_tree_selection_get_selected_rows   (GtkTreeSelection            *selection,
                                                         GtkTreeModel               **model);
GDK_AVAILABLE_IN_ALL
gint             gtk_tree_selection_count_selected_rows (GtkTreeSelection            *selection);
GDK_AVAILABLE_IN_ALL
void             gtk_tree_selection_selected_foreach    (GtkTreeSelection            *selection,
							 GtkTreeSelectionForeachFunc  func,
							 gpointer                     data);
GDK_AVAILABLE_IN_ALL
void             gtk_tree_selection_select_path         (GtkTreeSelection            *selection,
							 GtkTreePath                 *path);
GDK_AVAILABLE_IN_ALL
void             gtk_tree_selection_unselect_path       (GtkTreeSelection            *selection,
							 GtkTreePath                 *path);
GDK_AVAILABLE_IN_ALL
void             gtk_tree_selection_select_iter         (GtkTreeSelection            *selection,
							 GtkTreeIter                 *iter);
GDK_AVAILABLE_IN_ALL
void             gtk_tree_selection_unselect_iter       (GtkTreeSelection            *selection,
							 GtkTreeIter                 *iter);
GDK_AVAILABLE_IN_ALL
gboolean         gtk_tree_selection_path_is_selected    (GtkTreeSelection            *selection,
							 GtkTreePath                 *path);
GDK_AVAILABLE_IN_ALL
gboolean         gtk_tree_selection_iter_is_selected    (GtkTreeSelection            *selection,
							 GtkTreeIter                 *iter);
GDK_AVAILABLE_IN_ALL
void             gtk_tree_selection_select_all          (GtkTreeSelection            *selection);
GDK_AVAILABLE_IN_ALL
void             gtk_tree_selection_unselect_all        (GtkTreeSelection            *selection);
GDK_AVAILABLE_IN_ALL
void             gtk_tree_selection_select_range        (GtkTreeSelection            *selection,
							 GtkTreePath                 *start_path,
							 GtkTreePath                 *end_path);
GDK_AVAILABLE_IN_ALL
void             gtk_tree_selection_unselect_range      (GtkTreeSelection            *selection,
                                                         GtkTreePath                 *start_path,
							 GtkTreePath                 *end_path);


G_END_DECLS

#endif /* __GTK_TREE_SELECTION_H__ */
