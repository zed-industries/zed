/* gtktreemodelsort.h
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
 * License along with this library; if not, write to the
 * Free Software Foundation, Inc., 59 Temple Place - Suite 330,
 * Boston, MA 02111-1307, USA.
 */

#ifndef __GTK_TREE_MODEL_SORT_H__
#define __GTK_TREE_MODEL_SORT_H__

#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gdkconfig.h>
#include <gtk/gtktreemodel.h>
#include <gtk/gtktreesortable.h>

G_BEGIN_DECLS

#define GTK_TYPE_TREE_MODEL_SORT			(gtk_tree_model_sort_get_type ())
#define GTK_TREE_MODEL_SORT(obj)			(G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_TREE_MODEL_SORT, GtkTreeModelSort))
#define GTK_TREE_MODEL_SORT_CLASS(klass)		(G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_TREE_MODEL_SORT, GtkTreeModelSortClass))
#define GTK_IS_TREE_MODEL_SORT(obj)			(G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_TREE_MODEL_SORT))
#define GTK_IS_TREE_MODEL_SORT_CLASS(klass)		(G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_TREE_MODEL_SORT))
#define GTK_TREE_MODEL_SORT_GET_CLASS(obj)		(G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_TREE_MODEL_SORT, GtkTreeModelSortClass))

typedef struct _GtkTreeModelSort       GtkTreeModelSort;
typedef struct _GtkTreeModelSortClass  GtkTreeModelSortClass;

struct _GtkTreeModelSort
{
  GObject parent;

  /* < private > */
  gpointer GSEAL (root);
  gint GSEAL (stamp);
  guint GSEAL (child_flags);
  GtkTreeModel *GSEAL (child_model);
  gint GSEAL (zero_ref_count);

  /* sort information */
  GList *GSEAL (sort_list);
  gint GSEAL (sort_column_id);
  GtkSortType GSEAL (order);

  /* default sort */
  GtkTreeIterCompareFunc GSEAL (default_sort_func);
  gpointer GSEAL (default_sort_data);
  GDestroyNotify GSEAL (default_sort_destroy);

  /* signal ids */
  guint GSEAL (changed_id);
  guint GSEAL (inserted_id);
  guint GSEAL (has_child_toggled_id);
  guint GSEAL (deleted_id);
  guint GSEAL (reordered_id);
};

struct _GtkTreeModelSortClass
{
  GObjectClass parent_class;

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};


GType         gtk_tree_model_sort_get_type                   (void) G_GNUC_CONST;
GtkTreeModel *gtk_tree_model_sort_new_with_model             (GtkTreeModel     *child_model);

GtkTreeModel *gtk_tree_model_sort_get_model                  (GtkTreeModelSort *tree_model);
GtkTreePath  *gtk_tree_model_sort_convert_child_path_to_path (GtkTreeModelSort *tree_model_sort,
							      GtkTreePath      *child_path);
gboolean      gtk_tree_model_sort_convert_child_iter_to_iter (GtkTreeModelSort *tree_model_sort,
							      GtkTreeIter      *sort_iter,
							      GtkTreeIter      *child_iter);
GtkTreePath  *gtk_tree_model_sort_convert_path_to_child_path (GtkTreeModelSort *tree_model_sort,
							      GtkTreePath      *sorted_path);
void          gtk_tree_model_sort_convert_iter_to_child_iter (GtkTreeModelSort *tree_model_sort,
							      GtkTreeIter      *child_iter,
							      GtkTreeIter      *sorted_iter);
void          gtk_tree_model_sort_reset_default_sort_func    (GtkTreeModelSort *tree_model_sort);
void          gtk_tree_model_sort_clear_cache                (GtkTreeModelSort *tree_model_sort);
gboolean      gtk_tree_model_sort_iter_is_valid              (GtkTreeModelSort *tree_model_sort,
                                                              GtkTreeIter      *iter);


G_END_DECLS

#endif /* __GTK_TREE_MODEL_SORT_H__ */
