/* gtktreemodel.h
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

#ifndef __GTK_TREE_MODEL_H__
#define __GTK_TREE_MODEL_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <glib-object.h>
#include <gdk/gdk.h>

G_BEGIN_DECLS

#define GTK_TYPE_TREE_MODEL            (gtk_tree_model_get_type ())
#define GTK_TREE_MODEL(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_TREE_MODEL, GtkTreeModel))
#define GTK_IS_TREE_MODEL(obj)	       (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_TREE_MODEL))
#define GTK_TREE_MODEL_GET_IFACE(obj)  (G_TYPE_INSTANCE_GET_INTERFACE ((obj), GTK_TYPE_TREE_MODEL, GtkTreeModelIface))

#define GTK_TYPE_TREE_ITER             (gtk_tree_iter_get_type ())
#define GTK_TYPE_TREE_PATH             (gtk_tree_path_get_type ())
#define GTK_TYPE_TREE_ROW_REFERENCE    (gtk_tree_row_reference_get_type ())

typedef struct _GtkTreeIter         GtkTreeIter;
typedef struct _GtkTreePath         GtkTreePath;
typedef struct _GtkTreeRowReference GtkTreeRowReference;
typedef struct _GtkTreeModel        GtkTreeModel; /* Dummy typedef */
typedef struct _GtkTreeModelIface   GtkTreeModelIface;

/**
 * GtkTreeModelForeachFunc:
 * @model: the #GtkTreeModel being iterated
 * @path: the current #GtkTreePath
 * @iter: the current #GtkTreeIter
 * @data: (closure): The user data passed to gtk_tree_model_foreach()
 *
 * Type of the callback passed to gtk_tree_model_foreach() to
 * iterate over the rows in a tree model.
 *
 * Returns: %TRUE to stop iterating, %FALSE to continue
 *
 */
typedef gboolean (* GtkTreeModelForeachFunc) (GtkTreeModel *model, GtkTreePath *path, GtkTreeIter *iter, gpointer data);

/**
 * GtkTreeModelFlags:
 * @GTK_TREE_MODEL_ITERS_PERSIST: iterators survive all signals
 *     emitted by the tree
 * @GTK_TREE_MODEL_LIST_ONLY: the model is a list only, and never
 *     has children
 *
 * These flags indicate various properties of a #GtkTreeModel.
 *
 * They are returned by gtk_tree_model_get_flags(), and must be
 * static for the lifetime of the object. A more complete description
 * of #GTK_TREE_MODEL_ITERS_PERSIST can be found in the overview of
 * this section.
 */
typedef enum
{
  GTK_TREE_MODEL_ITERS_PERSIST = 1 << 0,
  GTK_TREE_MODEL_LIST_ONLY = 1 << 1
} GtkTreeModelFlags;

/**
 * GtkTreeIter:
 * @stamp: a unique stamp to catch invalid iterators
 * @user_data: model-specific data
 * @user_data2: model-specific data
 * @user_data3: model-specific data
 *
 * The #GtkTreeIter is the primary structure
 * for accessing a #GtkTreeModel. Models are expected to put a unique
 * integer in the @stamp member, and put
 * model-specific data in the three @user_data
 * members.
 */
struct _GtkTreeIter
{
  gint stamp;
  gpointer user_data;
  gpointer user_data2;
  gpointer user_data3;
};

/**
 * GtkTreeModelIface:
 * @row_changed: Signal emitted when a row in the model has changed.
 * @row_inserted: Signal emitted when a new row has been inserted in
 *    the model.
 * @row_has_child_toggled: Signal emitted when a row has gotten the
 *    first child row or lost its last child row.
 * @row_deleted: Signal emitted when a row has been deleted.
 * @rows_reordered: Signal emitted when the children of a node in the
 *    GtkTreeModel have been reordered.
 * @get_flags: Get #GtkTreeModelFlags supported by this interface.
 * @get_n_columns: Get the number of columns supported by the model.
 * @get_column_type: Get the type of the column.
 * @get_iter: Sets iter to a valid iterator pointing to path.
 * @get_path: Gets a newly-created #GtkTreePath referenced by iter.
 * @get_value: Initializes and sets value to that at column.
 * @iter_next: Sets iter to point to the node following it at the
 *    current level.
 * @iter_previous: Sets iter to point to the previous node at the
 *    current level.
 * @iter_children: Sets iter to point to the first child of parent.
 * @iter_has_child: %TRUE if iter has children, %FALSE otherwise.
 * @iter_n_children: Gets the number of children that iter has.
 * @iter_nth_child: Sets iter to be the child of parent, using the
 *    given index.
 * @iter_parent: Sets iter to be the parent of child.
 * @ref_node: Lets the tree ref the node.
 * @unref_node: Lets the tree unref the node.
 */
struct _GtkTreeModelIface
{
  /*< private >*/
  GTypeInterface g_iface;

  /*< public >*/

  /* Signals */
  void         (* row_changed)           (GtkTreeModel *tree_model,
					  GtkTreePath  *path,
					  GtkTreeIter  *iter);
  void         (* row_inserted)          (GtkTreeModel *tree_model,
					  GtkTreePath  *path,
					  GtkTreeIter  *iter);
  void         (* row_has_child_toggled) (GtkTreeModel *tree_model,
					  GtkTreePath  *path,
					  GtkTreeIter  *iter);
  void         (* row_deleted)           (GtkTreeModel *tree_model,
					  GtkTreePath  *path);
  void         (* rows_reordered)        (GtkTreeModel *tree_model,
					  GtkTreePath  *path,
					  GtkTreeIter  *iter,
					  gint         *new_order);

  /* Virtual Table */
  GtkTreeModelFlags (* get_flags)  (GtkTreeModel *tree_model);

  gint         (* get_n_columns)   (GtkTreeModel *tree_model);
  GType        (* get_column_type) (GtkTreeModel *tree_model,
				    gint          index_);
  gboolean     (* get_iter)        (GtkTreeModel *tree_model,
				    GtkTreeIter  *iter,
				    GtkTreePath  *path);
  GtkTreePath *(* get_path)        (GtkTreeModel *tree_model,
				    GtkTreeIter  *iter);
  void         (* get_value)       (GtkTreeModel *tree_model,
				    GtkTreeIter  *iter,
				    gint          column,
				    GValue       *value);
  gboolean     (* iter_next)       (GtkTreeModel *tree_model,
				    GtkTreeIter  *iter);
  gboolean     (* iter_previous)   (GtkTreeModel *tree_model,
				    GtkTreeIter  *iter);
  gboolean     (* iter_children)   (GtkTreeModel *tree_model,
				    GtkTreeIter  *iter,
				    GtkTreeIter  *parent);
  gboolean     (* iter_has_child)  (GtkTreeModel *tree_model,
				    GtkTreeIter  *iter);
  gint         (* iter_n_children) (GtkTreeModel *tree_model,
				    GtkTreeIter  *iter);
  gboolean     (* iter_nth_child)  (GtkTreeModel *tree_model,
				    GtkTreeIter  *iter,
				    GtkTreeIter  *parent,
				    gint          n);
  gboolean     (* iter_parent)     (GtkTreeModel *tree_model,
				    GtkTreeIter  *iter,
				    GtkTreeIter  *child);
  void         (* ref_node)        (GtkTreeModel *tree_model,
				    GtkTreeIter  *iter);
  void         (* unref_node)      (GtkTreeModel *tree_model,
				    GtkTreeIter  *iter);
};


/* GtkTreePath operations */
GDK_AVAILABLE_IN_ALL
GtkTreePath *gtk_tree_path_new              (void);
GDK_AVAILABLE_IN_ALL
GtkTreePath *gtk_tree_path_new_from_string  (const gchar       *path);
GDK_AVAILABLE_IN_ALL
GtkTreePath *gtk_tree_path_new_from_indices (gint               first_index,
					     ...);
GDK_AVAILABLE_IN_3_12
GtkTreePath *gtk_tree_path_new_from_indicesv (gint             *indices,
					      gsize             length);
GDK_AVAILABLE_IN_ALL
gchar       *gtk_tree_path_to_string        (GtkTreePath       *path);
GDK_AVAILABLE_IN_ALL
GtkTreePath *gtk_tree_path_new_first        (void);
GDK_AVAILABLE_IN_ALL
void         gtk_tree_path_append_index     (GtkTreePath       *path,
					     gint               index_);
GDK_AVAILABLE_IN_ALL
void         gtk_tree_path_prepend_index    (GtkTreePath       *path,
					     gint               index_);
GDK_AVAILABLE_IN_ALL
gint         gtk_tree_path_get_depth        (GtkTreePath       *path);
GDK_AVAILABLE_IN_ALL
gint        *gtk_tree_path_get_indices      (GtkTreePath       *path);

GDK_AVAILABLE_IN_ALL
gint        *gtk_tree_path_get_indices_with_depth (GtkTreePath *path,
						   gint        *depth);

GDK_AVAILABLE_IN_ALL
void         gtk_tree_path_free             (GtkTreePath       *path);
GDK_AVAILABLE_IN_ALL
GtkTreePath *gtk_tree_path_copy             (const GtkTreePath *path);
GDK_AVAILABLE_IN_ALL
GType        gtk_tree_path_get_type         (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
gint         gtk_tree_path_compare          (const GtkTreePath *a,
					     const GtkTreePath *b);
GDK_AVAILABLE_IN_ALL
void         gtk_tree_path_next             (GtkTreePath       *path);
GDK_AVAILABLE_IN_ALL
gboolean     gtk_tree_path_prev             (GtkTreePath       *path);
GDK_AVAILABLE_IN_ALL
gboolean     gtk_tree_path_up               (GtkTreePath       *path);
GDK_AVAILABLE_IN_ALL
void         gtk_tree_path_down             (GtkTreePath       *path);

GDK_AVAILABLE_IN_ALL
gboolean     gtk_tree_path_is_ancestor      (GtkTreePath       *path,
                                             GtkTreePath       *descendant);
GDK_AVAILABLE_IN_ALL
gboolean     gtk_tree_path_is_descendant    (GtkTreePath       *path,
                                             GtkTreePath       *ancestor);

/**
 * GtkTreeRowReference:
 *
 * A GtkTreeRowReference tracks model changes so that it always refers to the
 * same row (a #GtkTreePath refers to a position, not a fixed row). Create a
 * new GtkTreeRowReference with gtk_tree_row_reference_new().
 */

GDK_AVAILABLE_IN_ALL
GType                gtk_tree_row_reference_get_type (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
GtkTreeRowReference *gtk_tree_row_reference_new       (GtkTreeModel        *model,
						       GtkTreePath         *path);
GDK_AVAILABLE_IN_ALL
GtkTreeRowReference *gtk_tree_row_reference_new_proxy (GObject             *proxy,
						       GtkTreeModel        *model,
						       GtkTreePath         *path);
GDK_AVAILABLE_IN_ALL
GtkTreePath         *gtk_tree_row_reference_get_path  (GtkTreeRowReference *reference);
GDK_AVAILABLE_IN_ALL
GtkTreeModel        *gtk_tree_row_reference_get_model (GtkTreeRowReference *reference);
GDK_AVAILABLE_IN_ALL
gboolean             gtk_tree_row_reference_valid     (GtkTreeRowReference *reference);
GDK_AVAILABLE_IN_ALL
GtkTreeRowReference *gtk_tree_row_reference_copy      (GtkTreeRowReference *reference);
GDK_AVAILABLE_IN_ALL
void                 gtk_tree_row_reference_free      (GtkTreeRowReference *reference);
/* These two functions are only needed if you created the row reference with a
 * proxy object */
GDK_AVAILABLE_IN_ALL
void                 gtk_tree_row_reference_inserted  (GObject     *proxy,
						       GtkTreePath *path);
GDK_AVAILABLE_IN_ALL
void                 gtk_tree_row_reference_deleted   (GObject     *proxy,
						       GtkTreePath *path);
GDK_AVAILABLE_IN_ALL
void                 gtk_tree_row_reference_reordered (GObject     *proxy,
						       GtkTreePath *path,
						       GtkTreeIter *iter,
						       gint        *new_order);

/* GtkTreeIter operations */
GDK_AVAILABLE_IN_ALL
GtkTreeIter *     gtk_tree_iter_copy             (GtkTreeIter  *iter);
GDK_AVAILABLE_IN_ALL
void              gtk_tree_iter_free             (GtkTreeIter  *iter);
GDK_AVAILABLE_IN_ALL
GType             gtk_tree_iter_get_type         (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
GType             gtk_tree_model_get_type        (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
GtkTreeModelFlags gtk_tree_model_get_flags       (GtkTreeModel *tree_model);
GDK_AVAILABLE_IN_ALL
gint              gtk_tree_model_get_n_columns   (GtkTreeModel *tree_model);
GDK_AVAILABLE_IN_ALL
GType             gtk_tree_model_get_column_type (GtkTreeModel *tree_model,
						  gint          index_);


/* Iterator movement */
GDK_AVAILABLE_IN_ALL
gboolean          gtk_tree_model_get_iter        (GtkTreeModel *tree_model,
						  GtkTreeIter  *iter,
						  GtkTreePath  *path);
GDK_AVAILABLE_IN_ALL
gboolean          gtk_tree_model_get_iter_from_string (GtkTreeModel *tree_model,
						       GtkTreeIter  *iter,
						       const gchar  *path_string);
GDK_AVAILABLE_IN_ALL
gchar *           gtk_tree_model_get_string_from_iter (GtkTreeModel *tree_model,
                                                       GtkTreeIter  *iter);
GDK_AVAILABLE_IN_ALL
gboolean          gtk_tree_model_get_iter_first  (GtkTreeModel *tree_model,
						  GtkTreeIter  *iter);
GDK_AVAILABLE_IN_ALL
GtkTreePath *     gtk_tree_model_get_path        (GtkTreeModel *tree_model,
						  GtkTreeIter  *iter);
GDK_AVAILABLE_IN_ALL
void              gtk_tree_model_get_value       (GtkTreeModel *tree_model,
						  GtkTreeIter  *iter,
						  gint          column,
						  GValue       *value);
GDK_AVAILABLE_IN_ALL
gboolean          gtk_tree_model_iter_previous   (GtkTreeModel *tree_model,
						  GtkTreeIter  *iter);
GDK_AVAILABLE_IN_ALL
gboolean          gtk_tree_model_iter_next       (GtkTreeModel *tree_model,
						  GtkTreeIter  *iter);
GDK_AVAILABLE_IN_ALL
gboolean          gtk_tree_model_iter_children   (GtkTreeModel *tree_model,
						  GtkTreeIter  *iter,
						  GtkTreeIter  *parent);
GDK_AVAILABLE_IN_ALL
gboolean          gtk_tree_model_iter_has_child  (GtkTreeModel *tree_model,
						  GtkTreeIter  *iter);
GDK_AVAILABLE_IN_ALL
gint              gtk_tree_model_iter_n_children (GtkTreeModel *tree_model,
						  GtkTreeIter  *iter);
GDK_AVAILABLE_IN_ALL
gboolean          gtk_tree_model_iter_nth_child  (GtkTreeModel *tree_model,
						  GtkTreeIter  *iter,
						  GtkTreeIter  *parent,
						  gint          n);
GDK_AVAILABLE_IN_ALL
gboolean          gtk_tree_model_iter_parent     (GtkTreeModel *tree_model,
						  GtkTreeIter  *iter,
						  GtkTreeIter  *child);
GDK_AVAILABLE_IN_ALL
void              gtk_tree_model_ref_node        (GtkTreeModel *tree_model,
						  GtkTreeIter  *iter);
GDK_AVAILABLE_IN_ALL
void              gtk_tree_model_unref_node      (GtkTreeModel *tree_model,
						  GtkTreeIter  *iter);
GDK_AVAILABLE_IN_ALL
void              gtk_tree_model_get             (GtkTreeModel *tree_model,
						  GtkTreeIter  *iter,
						  ...);
GDK_AVAILABLE_IN_ALL
void              gtk_tree_model_get_valist      (GtkTreeModel *tree_model,
						  GtkTreeIter  *iter,
						  va_list       var_args);


GDK_AVAILABLE_IN_ALL
void              gtk_tree_model_foreach         (GtkTreeModel            *model,
						  GtkTreeModelForeachFunc  func,
						  gpointer                 user_data);

/* Signals */
GDK_AVAILABLE_IN_ALL
void gtk_tree_model_row_changed           (GtkTreeModel *tree_model,
					   GtkTreePath  *path,
					   GtkTreeIter  *iter);
GDK_AVAILABLE_IN_ALL
void gtk_tree_model_row_inserted          (GtkTreeModel *tree_model,
					   GtkTreePath  *path,
					   GtkTreeIter  *iter);
GDK_AVAILABLE_IN_ALL
void gtk_tree_model_row_has_child_toggled (GtkTreeModel *tree_model,
					   GtkTreePath  *path,
					   GtkTreeIter  *iter);
GDK_AVAILABLE_IN_ALL
void gtk_tree_model_row_deleted           (GtkTreeModel *tree_model,
					   GtkTreePath  *path);
GDK_AVAILABLE_IN_ALL
void gtk_tree_model_rows_reordered        (GtkTreeModel *tree_model,
					   GtkTreePath  *path,
					   GtkTreeIter  *iter,
					   gint         *new_order);
GDK_AVAILABLE_IN_3_10
void gtk_tree_model_rows_reordered_with_length (GtkTreeModel *tree_model,
						GtkTreePath  *path,
						GtkTreeIter  *iter,
						gint         *new_order,
						gint          length);

G_END_DECLS

#endif /* __GTK_TREE_MODEL_H__ */
