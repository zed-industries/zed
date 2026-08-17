/* gtktreemodelfilter.h
 * Copyright (C) 2000,2001  Red Hat, Inc., Jonathan Blandford <jrb@redhat.com>
 * Copyright (C) 2001-2003  Kristian Rietveld <kris@gtk.org>
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

#ifndef __GTK_TREE_MODEL_FILTER_H__
#define __GTK_TREE_MODEL_FILTER_H__

#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gdkconfig.h>
#include <gtk/gtktreemodel.h>

G_BEGIN_DECLS

#define GTK_TYPE_TREE_MODEL_FILTER              (gtk_tree_model_filter_get_type ())
#define GTK_TREE_MODEL_FILTER(obj)              (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_TREE_MODEL_FILTER, GtkTreeModelFilter))
#define GTK_TREE_MODEL_FILTER_CLASS(vtable)     (G_TYPE_CHECK_CLASS_CAST ((vtable), GTK_TYPE_TREE_MODEL_FILTER, GtkTreeModelFilterClass))
#define GTK_IS_TREE_MODEL_FILTER(obj)           (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_TREE_MODEL_FILTER))
#define GTK_IS_TREE_MODEL_FILTER_CLASS(vtable)  (G_TYPE_CHECK_CLASS_TYPE ((vtable), GTK_TYPE_TREE_MODEL_FILTER))
#define GTK_TREE_MODEL_FILTER_GET_CLASS(obj)   (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_TREE_MODEL_FILTER, GtkTreeModelFilterClass))

typedef gboolean (* GtkTreeModelFilterVisibleFunc) (GtkTreeModel *model,
                                                    GtkTreeIter  *iter,
                                                    gpointer      data);
typedef void (* GtkTreeModelFilterModifyFunc) (GtkTreeModel *model,
                                               GtkTreeIter  *iter,
                                               GValue       *value,
                                               gint          column,
                                               gpointer      data);

typedef struct _GtkTreeModelFilter          GtkTreeModelFilter;
typedef struct _GtkTreeModelFilterClass     GtkTreeModelFilterClass;
typedef struct _GtkTreeModelFilterPrivate   GtkTreeModelFilterPrivate;

struct _GtkTreeModelFilter
{
  GObject parent;

  /*< private >*/
  GtkTreeModelFilterPrivate *GSEAL (priv);
};

struct _GtkTreeModelFilterClass
{
  GObjectClass parent_class;

  /* Padding for future expansion */
  void (*_gtk_reserved0) (void);
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
};

/* base */
GType         gtk_tree_model_filter_get_type                   (void) G_GNUC_CONST;
GtkTreeModel *gtk_tree_model_filter_new                        (GtkTreeModel                 *child_model,
                                                                GtkTreePath                  *root);
void          gtk_tree_model_filter_set_visible_func           (GtkTreeModelFilter           *filter,
                                                                GtkTreeModelFilterVisibleFunc func,
                                                                gpointer                      data,
                                                                GDestroyNotify                destroy);
void          gtk_tree_model_filter_set_modify_func            (GtkTreeModelFilter           *filter,
                                                                gint                          n_columns,
                                                                GType                        *types,
                                                                GtkTreeModelFilterModifyFunc  func,
                                                                gpointer                      data,
                                                                GDestroyNotify                destroy);
void          gtk_tree_model_filter_set_visible_column         (GtkTreeModelFilter           *filter,
                                                                gint                          column);

GtkTreeModel *gtk_tree_model_filter_get_model                  (GtkTreeModelFilter           *filter);

/* conversion */
gboolean      gtk_tree_model_filter_convert_child_iter_to_iter (GtkTreeModelFilter           *filter,
                                                                GtkTreeIter                  *filter_iter,
                                                                GtkTreeIter                  *child_iter);
void          gtk_tree_model_filter_convert_iter_to_child_iter (GtkTreeModelFilter           *filter,
                                                                GtkTreeIter                  *child_iter,
                                                                GtkTreeIter                  *filter_iter);
GtkTreePath  *gtk_tree_model_filter_convert_child_path_to_path (GtkTreeModelFilter           *filter,
                                                                GtkTreePath                  *child_path);
GtkTreePath  *gtk_tree_model_filter_convert_path_to_child_path (GtkTreeModelFilter           *filter,
                                                                GtkTreePath                  *filter_path);

/* extras */
void          gtk_tree_model_filter_refilter                   (GtkTreeModelFilter           *filter);
void          gtk_tree_model_filter_clear_cache                (GtkTreeModelFilter           *filter);

G_END_DECLS

#endif /* __GTK_TREE_MODEL_FILTER_H__ */
