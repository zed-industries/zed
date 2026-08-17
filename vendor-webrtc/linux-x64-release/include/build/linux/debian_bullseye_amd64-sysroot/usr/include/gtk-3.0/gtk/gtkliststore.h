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
 * License along with this library. If not, see <http://www.gnu.org/licenses/>.
 */

#ifndef __GTK_LIST_STORE_H__
#define __GTK_LIST_STORE_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gdk/gdk.h>
#include <gtk/gtktreemodel.h>
#include <gtk/gtktreesortable.h>


G_BEGIN_DECLS


#define GTK_TYPE_LIST_STORE	       (gtk_list_store_get_type ())
#define GTK_LIST_STORE(obj)	       (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_LIST_STORE, GtkListStore))
#define GTK_LIST_STORE_CLASS(klass)    (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_LIST_STORE, GtkListStoreClass))
#define GTK_IS_LIST_STORE(obj)	       (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_LIST_STORE))
#define GTK_IS_LIST_STORE_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_LIST_STORE))
#define GTK_LIST_STORE_GET_CLASS(obj)  (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_LIST_STORE, GtkListStoreClass))

typedef struct _GtkListStore              GtkListStore;
typedef struct _GtkListStorePrivate       GtkListStorePrivate;
typedef struct _GtkListStoreClass         GtkListStoreClass;

struct _GtkListStore
{
  GObject parent;

  /*< private >*/
  GtkListStorePrivate *priv;
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


GDK_AVAILABLE_IN_ALL
GType         gtk_list_store_get_type         (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
GtkListStore *gtk_list_store_new              (gint          n_columns,
					       ...);
GDK_AVAILABLE_IN_ALL
GtkListStore *gtk_list_store_newv             (gint          n_columns,
					       GType        *types);
GDK_AVAILABLE_IN_ALL
void          gtk_list_store_set_column_types (GtkListStore *list_store,
					       gint          n_columns,
					       GType        *types);

/* NOTE: use gtk_tree_model_get to get values from a GtkListStore */

GDK_AVAILABLE_IN_ALL
void          gtk_list_store_set_value        (GtkListStore *list_store,
					       GtkTreeIter  *iter,
					       gint          column,
					       GValue       *value);
GDK_AVAILABLE_IN_ALL
void          gtk_list_store_set              (GtkListStore *list_store,
					       GtkTreeIter  *iter,
					       ...);
GDK_AVAILABLE_IN_ALL
void          gtk_list_store_set_valuesv      (GtkListStore *list_store,
					       GtkTreeIter  *iter,
					       gint         *columns,
					       GValue       *values,
					       gint          n_values);
GDK_AVAILABLE_IN_ALL
void          gtk_list_store_set_valist       (GtkListStore *list_store,
					       GtkTreeIter  *iter,
					       va_list       var_args);
GDK_AVAILABLE_IN_ALL
gboolean      gtk_list_store_remove           (GtkListStore *list_store,
					       GtkTreeIter  *iter);
GDK_AVAILABLE_IN_ALL
void          gtk_list_store_insert           (GtkListStore *list_store,
					       GtkTreeIter  *iter,
					       gint          position);
GDK_AVAILABLE_IN_ALL
void          gtk_list_store_insert_before    (GtkListStore *list_store,
					       GtkTreeIter  *iter,
					       GtkTreeIter  *sibling);
GDK_AVAILABLE_IN_ALL
void          gtk_list_store_insert_after     (GtkListStore *list_store,
					       GtkTreeIter  *iter,
					       GtkTreeIter  *sibling);
GDK_AVAILABLE_IN_ALL
void          gtk_list_store_insert_with_values  (GtkListStore *list_store,
						  GtkTreeIter  *iter,
						  gint          position,
						  ...);
GDK_AVAILABLE_IN_ALL
void          gtk_list_store_insert_with_valuesv (GtkListStore *list_store,
						  GtkTreeIter  *iter,
						  gint          position,
						  gint         *columns,
						  GValue       *values,
						  gint          n_values);
GDK_AVAILABLE_IN_ALL
void          gtk_list_store_prepend          (GtkListStore *list_store,
					       GtkTreeIter  *iter);
GDK_AVAILABLE_IN_ALL
void          gtk_list_store_append           (GtkListStore *list_store,
					       GtkTreeIter  *iter);
GDK_AVAILABLE_IN_ALL
void          gtk_list_store_clear            (GtkListStore *list_store);
GDK_AVAILABLE_IN_ALL
gboolean      gtk_list_store_iter_is_valid    (GtkListStore *list_store,
                                               GtkTreeIter  *iter);
GDK_AVAILABLE_IN_ALL
void          gtk_list_store_reorder          (GtkListStore *store,
                                               gint         *new_order);
GDK_AVAILABLE_IN_ALL
void          gtk_list_store_swap             (GtkListStore *store,
                                               GtkTreeIter  *a,
                                               GtkTreeIter  *b);
GDK_AVAILABLE_IN_ALL
void          gtk_list_store_move_after       (GtkListStore *store,
                                               GtkTreeIter  *iter,
                                               GtkTreeIter  *position);
GDK_AVAILABLE_IN_ALL
void          gtk_list_store_move_before      (GtkListStore *store,
                                               GtkTreeIter  *iter,
                                               GtkTreeIter  *position);


G_END_DECLS


#endif /* __GTK_LIST_STORE_H__ */
