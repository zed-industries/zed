/* gtkliststore.h
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

#ifndef __GTK_LIST_STORE_H__
#define __GTK_LIST_STORE_H__

#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gdkconfig.h>
#include <gtk/gtktreemodel.h>
#include <gtk/gtktreesortable.h>


G_BEGIN_DECLS


#define GTK_TYPE_LIST_STORE	       (gtk_list_store_get_type ())
#define GTK_LIST_STORE(obj)	       (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_LIST_STORE, GtkListStore))
#define GTK_LIST_STORE_CLASS(klass)    (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_LIST_STORE, GtkListStoreClass))
#define GTK_IS_LIST_STORE(obj)	       (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_LIST_STORE))
#define GTK_IS_LIST_STORE_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_LIST_STORE))
#define GTK_LIST_STORE_GET_CLASS(obj)  (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_LIST_STORE, GtkListStoreClass))

typedef struct _GtkListStore       GtkListStore;
typedef struct _GtkListStoreClass  GtkListStoreClass;

struct _GtkListStore
{
  GObject parent;

  /*< private >*/
  gint GSEAL (stamp);
  gpointer GSEAL (seq);		/* head of the list */
  gpointer GSEAL (_gtk_reserved1);
  GList *GSEAL (sort_list);
  gint GSEAL (n_columns);
  gint GSEAL (sort_column_id);
  GtkSortType GSEAL (order);
  GType *GSEAL (column_headers);
  gint GSEAL (length);
  GtkTreeIterCompareFunc GSEAL (default_sort_func);
  gpointer GSEAL (default_sort_data);
  GDestroyNotify GSEAL (default_sort_destroy);
  guint GSEAL (columns_dirty) : 1;
};

struct _GtkListStoreClass
{
  GObjectClass parent_class;

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};


GType         gtk_list_store_get_type         (void) G_GNUC_CONST;
GtkListStore *gtk_list_store_new              (gint          n_columns,
					       ...);
GtkListStore *gtk_list_store_newv             (gint          n_columns,
					       GType        *types);
void          gtk_list_store_set_column_types (GtkListStore *list_store,
					       gint          n_columns,
					       GType        *types);

/* NOTE: use gtk_tree_model_get to get values from a GtkListStore */

void          gtk_list_store_set_value        (GtkListStore *list_store,
					       GtkTreeIter  *iter,
					       gint          column,
					       GValue       *value);
void          gtk_list_store_set              (GtkListStore *list_store,
					       GtkTreeIter  *iter,
					       ...);
void          gtk_list_store_set_valuesv      (GtkListStore *list_store,
					       GtkTreeIter  *iter,
					       gint         *columns,
					       GValue       *values,
					       gint          n_values);
void          gtk_list_store_set_valist       (GtkListStore *list_store,
					       GtkTreeIter  *iter,
					       va_list       var_args);
gboolean      gtk_list_store_remove           (GtkListStore *list_store,
					       GtkTreeIter  *iter);
void          gtk_list_store_insert           (GtkListStore *list_store,
					       GtkTreeIter  *iter,
					       gint          position);
void          gtk_list_store_insert_before    (GtkListStore *list_store,
					       GtkTreeIter  *iter,
					       GtkTreeIter  *sibling);
void          gtk_list_store_insert_after     (GtkListStore *list_store,
					       GtkTreeIter  *iter,
					       GtkTreeIter  *sibling);
void          gtk_list_store_insert_with_values  (GtkListStore *list_store,
						  GtkTreeIter  *iter,
						  gint          position,
						  ...);
void          gtk_list_store_insert_with_valuesv (GtkListStore *list_store,
						  GtkTreeIter  *iter,
						  gint          position,
						  gint         *columns,
						  GValue       *values,
						  gint          n_values);
void          gtk_list_store_prepend          (GtkListStore *list_store,
					       GtkTreeIter  *iter);
void          gtk_list_store_append           (GtkListStore *list_store,
					       GtkTreeIter  *iter);
void          gtk_list_store_clear            (GtkListStore *list_store);
gboolean      gtk_list_store_iter_is_valid    (GtkListStore *list_store,
                                               GtkTreeIter  *iter);
void          gtk_list_store_reorder          (GtkListStore *store,
                                               gint         *new_order);
void          gtk_list_store_swap             (GtkListStore *store,
                                               GtkTreeIter  *a,
                                               GtkTreeIter  *b);
void          gtk_list_store_move_after       (GtkListStore *store,
                                               GtkTreeIter  *iter,
                                               GtkTreeIter  *position);
void          gtk_list_store_move_before      (GtkListStore *store,
                                               GtkTreeIter  *iter,
                                               GtkTreeIter  *position);


G_END_DECLS


#endif /* __GTK_LIST_STORE_H__ */
