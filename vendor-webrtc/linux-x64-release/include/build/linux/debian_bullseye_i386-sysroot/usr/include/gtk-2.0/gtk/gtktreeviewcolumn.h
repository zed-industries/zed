/* gtktreeviewcolumn.h
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

#ifndef __GTK_TREE_VIEW_COLUMN_H__
#define __GTK_TREE_VIEW_COLUMN_H__

#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkcellrenderer.h>
#include <gtk/gtktreemodel.h>
#include <gtk/gtktreesortable.h>

/* Not needed, retained for compatibility -Yosh */
#include <gtk/gtkobject.h>


G_BEGIN_DECLS


#define GTK_TYPE_TREE_VIEW_COLUMN	     (gtk_tree_view_column_get_type ())
#define GTK_TREE_VIEW_COLUMN(obj)	     (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_TREE_VIEW_COLUMN, GtkTreeViewColumn))
#define GTK_TREE_VIEW_COLUMN_CLASS(klass)    (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_TREE_VIEW_COLUMN, GtkTreeViewColumnClass))
#define GTK_IS_TREE_VIEW_COLUMN(obj)	     (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_TREE_VIEW_COLUMN))
#define GTK_IS_TREE_VIEW_COLUMN_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_TREE_VIEW_COLUMN))
#define GTK_TREE_VIEW_COLUMN_GET_CLASS(obj)  (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_TREE_VIEW_COLUMN, GtkTreeViewColumnClass))

typedef enum
{
  GTK_TREE_VIEW_COLUMN_GROW_ONLY,
  GTK_TREE_VIEW_COLUMN_AUTOSIZE,
  GTK_TREE_VIEW_COLUMN_FIXED
} GtkTreeViewColumnSizing;

typedef struct _GtkTreeViewColumn      GtkTreeViewColumn;
typedef struct _GtkTreeViewColumnClass GtkTreeViewColumnClass;

typedef void (* GtkTreeCellDataFunc) (GtkTreeViewColumn *tree_column,
				      GtkCellRenderer   *cell,
				      GtkTreeModel      *tree_model,
				      GtkTreeIter       *iter,
				      gpointer           data);


struct _GtkTreeViewColumn
{
  GtkObject parent;

  GtkWidget *GSEAL (tree_view);
  GtkWidget *GSEAL (button);
  GtkWidget *GSEAL (child);
  GtkWidget *GSEAL (arrow);
  GtkWidget *GSEAL (alignment);
  GdkWindow *GSEAL (window);
  GtkCellEditable *GSEAL (editable_widget);
  gfloat GSEAL (xalign);
  guint GSEAL (property_changed_signal);
  gint GSEAL (spacing);

  /* Sizing fields */
  /* see gtk+/doc/tree-column-sizing.txt for more information on them */
  GtkTreeViewColumnSizing GSEAL (column_type);
  gint GSEAL (requested_width);
  gint GSEAL (button_request);
  gint GSEAL (resized_width);
  gint GSEAL (width);
  gint GSEAL (fixed_width);
  gint GSEAL (min_width);
  gint GSEAL (max_width);

  /* dragging columns */
  gint GSEAL (drag_x);
  gint GSEAL (drag_y);

  gchar *GSEAL (title);
  GList *GSEAL (cell_list);

  /* Sorting */
  guint GSEAL (sort_clicked_signal);
  guint GSEAL (sort_column_changed_signal);
  gint GSEAL (sort_column_id);
  GtkSortType GSEAL (sort_order);

  /* Flags */
  guint GSEAL (visible)             : 1;
  guint GSEAL (resizable)           : 1;
  guint GSEAL (clickable)           : 1;
  guint GSEAL (dirty)               : 1;
  guint GSEAL (show_sort_indicator) : 1;
  guint GSEAL (maybe_reordered)     : 1;
  guint GSEAL (reorderable)         : 1;
  guint GSEAL (use_resized_width)   : 1;
  guint GSEAL (expand)              : 1;
};

struct _GtkTreeViewColumnClass
{
  GtkObjectClass parent_class;

  void (*clicked) (GtkTreeViewColumn *tree_column);

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};

GType                   gtk_tree_view_column_get_type            (void) G_GNUC_CONST;
GtkTreeViewColumn      *gtk_tree_view_column_new                 (void);
GtkTreeViewColumn      *gtk_tree_view_column_new_with_attributes (const gchar             *title,
								  GtkCellRenderer         *cell,
								  ...) G_GNUC_NULL_TERMINATED;
void                    gtk_tree_view_column_pack_start          (GtkTreeViewColumn       *tree_column,
								  GtkCellRenderer         *cell,
								  gboolean                 expand);
void                    gtk_tree_view_column_pack_end            (GtkTreeViewColumn       *tree_column,
								  GtkCellRenderer         *cell,
								  gboolean                 expand);
void                    gtk_tree_view_column_clear               (GtkTreeViewColumn       *tree_column);
#ifndef GTK_DISABLE_DEPRECATED
GList                  *gtk_tree_view_column_get_cell_renderers  (GtkTreeViewColumn       *tree_column);
#endif
void                    gtk_tree_view_column_add_attribute       (GtkTreeViewColumn       *tree_column,
								  GtkCellRenderer         *cell_renderer,
								  const gchar             *attribute,
								  gint                     column);
void                    gtk_tree_view_column_set_attributes      (GtkTreeViewColumn       *tree_column,
								  GtkCellRenderer         *cell_renderer,
								  ...) G_GNUC_NULL_TERMINATED;
void                    gtk_tree_view_column_set_cell_data_func  (GtkTreeViewColumn       *tree_column,
								  GtkCellRenderer         *cell_renderer,
								  GtkTreeCellDataFunc      func,
								  gpointer                 func_data,
								  GDestroyNotify           destroy);
void                    gtk_tree_view_column_clear_attributes    (GtkTreeViewColumn       *tree_column,
								  GtkCellRenderer         *cell_renderer);
void                    gtk_tree_view_column_set_spacing         (GtkTreeViewColumn       *tree_column,
								  gint                     spacing);
gint                    gtk_tree_view_column_get_spacing         (GtkTreeViewColumn       *tree_column);
void                    gtk_tree_view_column_set_visible         (GtkTreeViewColumn       *tree_column,
								  gboolean                 visible);
gboolean                gtk_tree_view_column_get_visible         (GtkTreeViewColumn       *tree_column);
void                    gtk_tree_view_column_set_resizable       (GtkTreeViewColumn       *tree_column,
								  gboolean                 resizable);
gboolean                gtk_tree_view_column_get_resizable       (GtkTreeViewColumn       *tree_column);
void                    gtk_tree_view_column_set_sizing          (GtkTreeViewColumn       *tree_column,
								  GtkTreeViewColumnSizing  type);
GtkTreeViewColumnSizing gtk_tree_view_column_get_sizing          (GtkTreeViewColumn       *tree_column);
gint                    gtk_tree_view_column_get_width           (GtkTreeViewColumn       *tree_column);
gint                    gtk_tree_view_column_get_fixed_width     (GtkTreeViewColumn       *tree_column);
void                    gtk_tree_view_column_set_fixed_width     (GtkTreeViewColumn       *tree_column,
								  gint                     fixed_width);
void                    gtk_tree_view_column_set_min_width       (GtkTreeViewColumn       *tree_column,
								  gint                     min_width);
gint                    gtk_tree_view_column_get_min_width       (GtkTreeViewColumn       *tree_column);
void                    gtk_tree_view_column_set_max_width       (GtkTreeViewColumn       *tree_column,
								  gint                     max_width);
gint                    gtk_tree_view_column_get_max_width       (GtkTreeViewColumn       *tree_column);
void                    gtk_tree_view_column_clicked             (GtkTreeViewColumn       *tree_column);



/* Options for manipulating the column headers
 */
void                    gtk_tree_view_column_set_title           (GtkTreeViewColumn       *tree_column,
								  const gchar             *title);
const gchar *           gtk_tree_view_column_get_title           (GtkTreeViewColumn       *tree_column);
void                    gtk_tree_view_column_set_expand          (GtkTreeViewColumn       *tree_column,
								  gboolean                 expand);
gboolean                gtk_tree_view_column_get_expand          (GtkTreeViewColumn       *tree_column);
void                    gtk_tree_view_column_set_clickable       (GtkTreeViewColumn       *tree_column,
								  gboolean                 clickable);
gboolean                gtk_tree_view_column_get_clickable       (GtkTreeViewColumn       *tree_column);
void                    gtk_tree_view_column_set_widget          (GtkTreeViewColumn       *tree_column,
								  GtkWidget               *widget);
GtkWidget              *gtk_tree_view_column_get_widget          (GtkTreeViewColumn       *tree_column);
void                    gtk_tree_view_column_set_alignment       (GtkTreeViewColumn       *tree_column,
								  gfloat                   xalign);
gfloat                  gtk_tree_view_column_get_alignment       (GtkTreeViewColumn       *tree_column);
void                    gtk_tree_view_column_set_reorderable     (GtkTreeViewColumn       *tree_column,
								  gboolean                 reorderable);
gboolean                gtk_tree_view_column_get_reorderable     (GtkTreeViewColumn       *tree_column);



/* You probably only want to use gtk_tree_view_column_set_sort_column_id.  The
 * other sorting functions exist primarily to let others do their own custom sorting.
 */
void                    gtk_tree_view_column_set_sort_column_id  (GtkTreeViewColumn       *tree_column,
								  gint                     sort_column_id);
gint                    gtk_tree_view_column_get_sort_column_id  (GtkTreeViewColumn       *tree_column);
void                    gtk_tree_view_column_set_sort_indicator  (GtkTreeViewColumn       *tree_column,
								  gboolean                 setting);
gboolean                gtk_tree_view_column_get_sort_indicator  (GtkTreeViewColumn       *tree_column);
void                    gtk_tree_view_column_set_sort_order      (GtkTreeViewColumn       *tree_column,
								  GtkSortType              order);
GtkSortType             gtk_tree_view_column_get_sort_order      (GtkTreeViewColumn       *tree_column);


/* These functions are meant primarily for interaction between the GtkTreeView and the column.
 */
void                    gtk_tree_view_column_cell_set_cell_data  (GtkTreeViewColumn       *tree_column,
								  GtkTreeModel            *tree_model,
								  GtkTreeIter             *iter,
								  gboolean                 is_expander,
								  gboolean                 is_expanded);
void                    gtk_tree_view_column_cell_get_size       (GtkTreeViewColumn       *tree_column,
								  const GdkRectangle      *cell_area,
								  gint                    *x_offset,
								  gint                    *y_offset,
								  gint                    *width,
								  gint                    *height);
gboolean                gtk_tree_view_column_cell_is_visible     (GtkTreeViewColumn       *tree_column);
void                    gtk_tree_view_column_focus_cell          (GtkTreeViewColumn       *tree_column,
								  GtkCellRenderer         *cell);
gboolean                gtk_tree_view_column_cell_get_position   (GtkTreeViewColumn       *tree_column,
					                          GtkCellRenderer         *cell_renderer,
					                          gint                    *start_pos,
					                          gint                    *width);
void                    gtk_tree_view_column_queue_resize        (GtkTreeViewColumn       *tree_column);
GtkWidget              *gtk_tree_view_column_get_tree_view       (GtkTreeViewColumn       *tree_column);


G_END_DECLS


#endif /* __GTK_TREE_VIEW_COLUMN_H__ */
