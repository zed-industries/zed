/* gtktreestore.h
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

#ifndef __GTK_TREE_STORE_H__
#define __GTK_TREE_STORE_H__

#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gdkconfig.h>
#include <gtk/gtktreemodel.h>
#include <gtk/gtktreesortable.h>
#include <stdarg.h>


G_BEGIN_DECLS


#define GTK_TYPE_TREE_STORE			(gtk_tree_store_get_type ())
#define GTK_TREE_STORE(obj)			(G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_TREE_STORE, GtkTreeStore))
#define GTK_TREE_STORE_CLASS(klass)		(G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_TREE_STORE, GtkTreeStoreClass))
#define GTK_IS_TREE_STORE(obj)			(G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_TREE_STORE))
#define GTK_IS_TREE_STORE_CLASS(klass)		(G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_TREE_STORE))
#define GTK_TREE_STORE_GET_CLASS(obj)		(G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_TREE_STORE, GtkTreeStoreClass))

typedef struct _GtkTreeStore       GtkTreeStore;
typedef struct _GtkTreeStoreClass  GtkTreeStoreClass;

struct _GtkTreeStore
{
  GObject parent;

  gint GSEAL (stamp);
  gpointer GSEAL (root);
  gpointer GSEAL (last);
  gint GSEAL (n_columns);
  gint GSEAL (sort_column_id);
  GList *GSEAL (sort_list);
  GtkSortType GSEAL (order);
  GType *GSEAL (column_headers);
  GtkTreeIterCompareFunc GSEAL (default_sort_func);
  gpointer GSEAL (default_sort_data);
  GDestroyNotify GSEAL (default_sort_destroy);
  guint GSEAL (columns_dirty) : 1;
};

struct _GtkTreeStoreClass
{
  GObjectClass parent_class;

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};


GType         gtk_tree_store_get_type         (void) G_GNUC_CONST;
GtkTreeStore *gtk_tree_store_new              (gint          n_columns,
					       ...);
GtkTreeStore *gtk_tree_store_newv             (gint          n_columns,
					       GType        *types);
void          gtk_tree_store_set_column_types (GtkTreeStore *tree_store,
					       gint          n_columns,
					       GType        *types);

/* NOTE: use gtk_tree_model_get to get values from a GtkTreeStore */

void          gtk_tree_store_set_value        (GtkTreeStore *tree_store,
					       GtkTreeIter  *iter,
					       gint          column,
					       GValue       *value);
void          gtk_tree_store_set              (GtkTreeStore *tree_store,
					       GtkTreeIter  *iter,
					       ...);
void          gtk_tree_store_set_valuesv      (GtkTreeStore *tree_store,
					       GtkTreeIter  *iter,
					       gint         *columns,
					       GValue       *values,
					       gint          n_values);
void          gtk_tree_store_set_valist       (GtkTreeStore *tree_store,
					       GtkTreeIter  *iter,
					       va_list       var_args);
gboolean      gtk_tree_store_remove           (GtkTreeStore *tree_store,
					       GtkTreeIter  *iter);
void          gtk_tree_store_insert           (GtkTreeStore *tree_store,
					       GtkTreeIter  *iter,
					       GtkTreeIter  *parent,
					       gint          position);
void          gtk_tree_store_insert_before    (GtkTreeStore *tree_store,
					       GtkTreeIter  *iter,
					       GtkTreeIter  *parent,
					       GtkTreeIter  *sibling);
void          gtk_tree_store_insert_after     (GtkTreeStore *tree_store,
					       GtkTreeIter  *iter,
					       GtkTreeIter  *parent,
					       GtkTreeIter  *sibling);
void          gtk_tree_store_insert_with_values (GtkTreeStore *tree_store,
						 GtkTreeIter  *iter,
						 GtkTreeIter  *parent,
						 gint          position,
						 ...);
void          gtk_tree_store_insert_with_valuesv (GtkTreeStore *tree_store,
						  GtkTreeIter  *iter,
						  GtkTreeIter  *parent,
						  gint          position,
						  gint         *columns,
						  GValue       *values,
						  gint          n_values);
void          gtk_tree_store_prepend          (GtkTreeStore *tree_store,
					       GtkTreeIter  *iter,
					       GtkTreeIter  *parent);
void          gtk_tree_store_append           (GtkTreeStore *tree_store,
					       GtkTreeIter  *iter,
					       GtkTreeIter  *parent);
gboolean      gtk_tree_store_is_ancestor      (GtkTreeStore *tree_store,
					       GtkTreeIter  *iter,
					       GtkTreeIter  *descendant);
gint          gtk_tree_store_iter_depth       (GtkTreeStore *tree_store,
					       GtkTreeIter  *iter);
void          gtk_tree_store_clear            (GtkTreeStore *tree_store);
gboolean      gtk_tree_store_iter_is_valid    (GtkTreeStore *tree_store,
                                               GtkTreeIter  *iter);
void          gtk_tree_store_reorder          (GtkTreeStore *tree_store,
                                               GtkTreeIter  *parent,
                                               gint         *new_order);
void          gtk_tree_store_swap             (GtkTreeStore *tree_store,
                                               GtkTreeIter  *a,
                                               GtkTreeIter  *b);
void          gtk_tree_store_move_before      (GtkTreeStore *tree_store,
                                               GtkTreeIter  *iter,
                                               GtkTreeIter  *position);
void          gtk_tree_store_move_after       (GtkTreeStore *tree_store,
                                               GtkTreeIter  *iter,
                                               GtkTreeIter  *position);


G_END_DECLS


#endif /* __GTK_TREE_STORE_H__ */
