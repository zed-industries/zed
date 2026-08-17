/* gtktreeview.h
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

#ifndef __GTK_TREE_VIEW_H__
#define __GTK_TREE_VIEW_H__

#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkcontainer.h>
#include <gtk/gtktreemodel.h>
#include <gtk/gtktreeviewcolumn.h>
#include <gtk/gtkdnd.h>
#include <gtk/gtkentry.h>

G_BEGIN_DECLS


typedef enum
{
  /* drop before/after this row */
  GTK_TREE_VIEW_DROP_BEFORE,
  GTK_TREE_VIEW_DROP_AFTER,
  /* drop as a child of this row (with fallback to before or after
   * if into is not possible)
   */
  GTK_TREE_VIEW_DROP_INTO_OR_BEFORE,
  GTK_TREE_VIEW_DROP_INTO_OR_AFTER
} GtkTreeViewDropPosition;

#define GTK_TYPE_TREE_VIEW		(gtk_tree_view_get_type ())
#define GTK_TREE_VIEW(obj)		(G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_TREE_VIEW, GtkTreeView))
#define GTK_TREE_VIEW_CLASS(klass)	(G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_TREE_VIEW, GtkTreeViewClass))
#define GTK_IS_TREE_VIEW(obj)		(G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_TREE_VIEW))
#define GTK_IS_TREE_VIEW_CLASS(klass)	(G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_TREE_VIEW))
#define GTK_TREE_VIEW_GET_CLASS(obj)    (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_TREE_VIEW, GtkTreeViewClass))

typedef struct _GtkTreeView           GtkTreeView;
typedef struct _GtkTreeViewClass      GtkTreeViewClass;
typedef struct _GtkTreeViewPrivate    GtkTreeViewPrivate;
typedef struct _GtkTreeSelection      GtkTreeSelection;
typedef struct _GtkTreeSelectionClass GtkTreeSelectionClass;

struct _GtkTreeView
{
  GtkContainer parent;

  GtkTreeViewPrivate *GSEAL (priv);
};

struct _GtkTreeViewClass
{
  GtkContainerClass parent_class;

  void     (* set_scroll_adjustments)     (GtkTreeView       *tree_view,
				           GtkAdjustment     *hadjustment,
				           GtkAdjustment     *vadjustment);
  void     (* row_activated)              (GtkTreeView       *tree_view,
				           GtkTreePath       *path,
					   GtkTreeViewColumn *column);
  gboolean (* test_expand_row)            (GtkTreeView       *tree_view,
				           GtkTreeIter       *iter,
				           GtkTreePath       *path);
  gboolean (* test_collapse_row)          (GtkTreeView       *tree_view,
				           GtkTreeIter       *iter,
				           GtkTreePath       *path);
  void     (* row_expanded)               (GtkTreeView       *tree_view,
				           GtkTreeIter       *iter,
				           GtkTreePath       *path);
  void     (* row_collapsed)              (GtkTreeView       *tree_view,
				           GtkTreeIter       *iter,
				           GtkTreePath       *path);
  void     (* columns_changed)            (GtkTreeView       *tree_view);
  void     (* cursor_changed)             (GtkTreeView       *tree_view);

  /* Key Binding signals */
  gboolean (* move_cursor)                (GtkTreeView       *tree_view,
				           GtkMovementStep    step,
				           gint               count);
  gboolean (* select_all)                 (GtkTreeView       *tree_view);
  gboolean (* unselect_all)               (GtkTreeView       *tree_view);
  gboolean (* select_cursor_row)          (GtkTreeView       *tree_view,
					   gboolean           start_editing);
  gboolean (* toggle_cursor_row)          (GtkTreeView       *tree_view);
  gboolean (* expand_collapse_cursor_row) (GtkTreeView       *tree_view,
					   gboolean           logical,
					   gboolean           expand,
					   gboolean           open_all);
  gboolean (* select_cursor_parent)       (GtkTreeView       *tree_view);
  gboolean (* start_interactive_search)   (GtkTreeView       *tree_view);

  /* Padding for future expansion */
  void (*_gtk_reserved0) (void);
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};


typedef gboolean (* GtkTreeViewColumnDropFunc) (GtkTreeView             *tree_view,
						GtkTreeViewColumn       *column,
						GtkTreeViewColumn       *prev_column,
						GtkTreeViewColumn       *next_column,
						gpointer                 data);
typedef void     (* GtkTreeViewMappingFunc)    (GtkTreeView             *tree_view,
						GtkTreePath             *path,
						gpointer                 user_data);
typedef gboolean (*GtkTreeViewSearchEqualFunc) (GtkTreeModel            *model,
						gint                     column,
						const gchar             *key,
						GtkTreeIter             *iter,
						gpointer                 search_data);
typedef gboolean (*GtkTreeViewRowSeparatorFunc) (GtkTreeModel      *model,
						 GtkTreeIter       *iter,
						 gpointer           data);
typedef void     (*GtkTreeViewSearchPositionFunc) (GtkTreeView  *tree_view,
						   GtkWidget    *search_dialog,
						   gpointer      user_data);


/* Creators */
GType                  gtk_tree_view_get_type                      (void) G_GNUC_CONST;
GtkWidget             *gtk_tree_view_new                           (void);
GtkWidget             *gtk_tree_view_new_with_model                (GtkTreeModel              *model);

/* Accessors */
GtkTreeModel          *gtk_tree_view_get_model                     (GtkTreeView               *tree_view);
void                   gtk_tree_view_set_model                     (GtkTreeView               *tree_view,
								    GtkTreeModel              *model);
GtkTreeSelection      *gtk_tree_view_get_selection                 (GtkTreeView               *tree_view);
GtkAdjustment         *gtk_tree_view_get_hadjustment               (GtkTreeView               *tree_view);
void                   gtk_tree_view_set_hadjustment               (GtkTreeView               *tree_view,
								    GtkAdjustment             *adjustment);
GtkAdjustment         *gtk_tree_view_get_vadjustment               (GtkTreeView               *tree_view);
void                   gtk_tree_view_set_vadjustment               (GtkTreeView               *tree_view,
								    GtkAdjustment             *adjustment);
gboolean               gtk_tree_view_get_headers_visible           (GtkTreeView               *tree_view);
void                   gtk_tree_view_set_headers_visible           (GtkTreeView               *tree_view,
								    gboolean                   headers_visible);
void                   gtk_tree_view_columns_autosize              (GtkTreeView               *tree_view);
gboolean               gtk_tree_view_get_headers_clickable         (GtkTreeView *tree_view);
void                   gtk_tree_view_set_headers_clickable         (GtkTreeView               *tree_view,
								    gboolean                   setting);
void                   gtk_tree_view_set_rules_hint                (GtkTreeView               *tree_view,
								    gboolean                   setting);
gboolean               gtk_tree_view_get_rules_hint                (GtkTreeView               *tree_view);

/* Column funtions */
gint                   gtk_tree_view_append_column                 (GtkTreeView               *tree_view,
								    GtkTreeViewColumn         *column);
gint                   gtk_tree_view_remove_column                 (GtkTreeView               *tree_view,
								    GtkTreeViewColumn         *column);
gint                   gtk_tree_view_insert_column                 (GtkTreeView               *tree_view,
								    GtkTreeViewColumn         *column,
								    gint                       position);
gint                   gtk_tree_view_insert_column_with_attributes (GtkTreeView               *tree_view,
								    gint                       position,
								    const gchar               *title,
								    GtkCellRenderer           *cell,
								    ...) G_GNUC_NULL_TERMINATED;
gint                   gtk_tree_view_insert_column_with_data_func  (GtkTreeView               *tree_view,
								    gint                       position,
								    const gchar               *title,
								    GtkCellRenderer           *cell,
                                                                    GtkTreeCellDataFunc        func,
                                                                    gpointer                   data,
                                                                    GDestroyNotify             dnotify);
GtkTreeViewColumn     *gtk_tree_view_get_column                    (GtkTreeView               *tree_view,
								    gint                       n);
GList                 *gtk_tree_view_get_columns                   (GtkTreeView               *tree_view);
void                   gtk_tree_view_move_column_after             (GtkTreeView               *tree_view,
								    GtkTreeViewColumn         *column,
								    GtkTreeViewColumn         *base_column);
void                   gtk_tree_view_set_expander_column           (GtkTreeView               *tree_view,
								    GtkTreeViewColumn         *column);
GtkTreeViewColumn     *gtk_tree_view_get_expander_column           (GtkTreeView               *tree_view);
void                   gtk_tree_view_set_column_drag_function      (GtkTreeView               *tree_view,
								    GtkTreeViewColumnDropFunc  func,
								    gpointer                   user_data,
								    GDestroyNotify             destroy);

/* Actions */
void                   gtk_tree_view_scroll_to_point               (GtkTreeView               *tree_view,
								    gint                       tree_x,
								    gint                       tree_y);
void                   gtk_tree_view_scroll_to_cell                (GtkTreeView               *tree_view,
								    GtkTreePath               *path,
								    GtkTreeViewColumn         *column,
								    gboolean                   use_align,
								    gfloat                     row_align,
								    gfloat                     col_align);
void                   gtk_tree_view_row_activated                 (GtkTreeView               *tree_view,
								    GtkTreePath               *path,
								    GtkTreeViewColumn         *column);
void                   gtk_tree_view_expand_all                    (GtkTreeView               *tree_view);
void                   gtk_tree_view_collapse_all                  (GtkTreeView               *tree_view);
void                   gtk_tree_view_expand_to_path                (GtkTreeView               *tree_view,
								    GtkTreePath               *path);
gboolean               gtk_tree_view_expand_row                    (GtkTreeView               *tree_view,
								    GtkTreePath               *path,
								    gboolean                   open_all);
gboolean               gtk_tree_view_collapse_row                  (GtkTreeView               *tree_view,
								    GtkTreePath               *path);
void                   gtk_tree_view_map_expanded_rows             (GtkTreeView               *tree_view,
								    GtkTreeViewMappingFunc     func,
								    gpointer                   data);
gboolean               gtk_tree_view_row_expanded                  (GtkTreeView               *tree_view,
								    GtkTreePath               *path);
void                   gtk_tree_view_set_reorderable               (GtkTreeView               *tree_view,
								    gboolean                   reorderable);
gboolean               gtk_tree_view_get_reorderable               (GtkTreeView               *tree_view);
void                   gtk_tree_view_set_cursor                    (GtkTreeView               *tree_view,
								    GtkTreePath               *path,
								    GtkTreeViewColumn         *focus_column,
								    gboolean                   start_editing);
void                   gtk_tree_view_set_cursor_on_cell            (GtkTreeView               *tree_view,
								    GtkTreePath               *path,
								    GtkTreeViewColumn         *focus_column,
								    GtkCellRenderer           *focus_cell,
								    gboolean                   start_editing);
void                   gtk_tree_view_get_cursor                    (GtkTreeView               *tree_view,
								    GtkTreePath              **path,
								    GtkTreeViewColumn        **focus_column);


/* Layout information */
GdkWindow             *gtk_tree_view_get_bin_window                (GtkTreeView               *tree_view);
gboolean               gtk_tree_view_get_path_at_pos               (GtkTreeView               *tree_view,
								    gint                       x,
								    gint                       y,
								    GtkTreePath              **path,
								    GtkTreeViewColumn        **column,
								    gint                      *cell_x,
								    gint                      *cell_y);
void                   gtk_tree_view_get_cell_area                 (GtkTreeView               *tree_view,
								    GtkTreePath               *path,
								    GtkTreeViewColumn         *column,
								    GdkRectangle              *rect);
void                   gtk_tree_view_get_background_area           (GtkTreeView               *tree_view,
								    GtkTreePath               *path,
								    GtkTreeViewColumn         *column,
								    GdkRectangle              *rect);
void                   gtk_tree_view_get_visible_rect              (GtkTreeView               *tree_view,
								    GdkRectangle              *visible_rect);

#ifndef GTK_DISABLE_DEPRECATED
void                   gtk_tree_view_widget_to_tree_coords         (GtkTreeView               *tree_view,
								    gint                       wx,
								    gint                       wy,
								    gint                      *tx,
								    gint                      *ty);
void                   gtk_tree_view_tree_to_widget_coords         (GtkTreeView               *tree_view,
								    gint                       tx,
								    gint                       ty,
								    gint                      *wx,
								    gint                      *wy);
#endif /* !GTK_DISABLE_DEPRECATED */
gboolean               gtk_tree_view_get_visible_range             (GtkTreeView               *tree_view,
								    GtkTreePath              **start_path,
								    GtkTreePath              **end_path);

/* Drag-and-Drop support */
void                   gtk_tree_view_enable_model_drag_source      (GtkTreeView               *tree_view,
								    GdkModifierType            start_button_mask,
								    const GtkTargetEntry      *targets,
								    gint                       n_targets,
								    GdkDragAction              actions);
void                   gtk_tree_view_enable_model_drag_dest        (GtkTreeView               *tree_view,
								    const GtkTargetEntry      *targets,
								    gint                       n_targets,
								    GdkDragAction              actions);
void                   gtk_tree_view_unset_rows_drag_source        (GtkTreeView               *tree_view);
void                   gtk_tree_view_unset_rows_drag_dest          (GtkTreeView               *tree_view);


/* These are useful to implement your own custom stuff. */
void                   gtk_tree_view_set_drag_dest_row             (GtkTreeView               *tree_view,
								    GtkTreePath               *path,
								    GtkTreeViewDropPosition    pos);
void                   gtk_tree_view_get_drag_dest_row             (GtkTreeView               *tree_view,
								    GtkTreePath              **path,
								    GtkTreeViewDropPosition   *pos);
gboolean               gtk_tree_view_get_dest_row_at_pos           (GtkTreeView               *tree_view,
								    gint                       drag_x,
								    gint                       drag_y,
								    GtkTreePath              **path,
								    GtkTreeViewDropPosition   *pos);
GdkPixmap             *gtk_tree_view_create_row_drag_icon          (GtkTreeView               *tree_view,
								    GtkTreePath               *path);

/* Interactive search */
void                       gtk_tree_view_set_enable_search     (GtkTreeView                *tree_view,
								gboolean                    enable_search);
gboolean                   gtk_tree_view_get_enable_search     (GtkTreeView                *tree_view);
gint                       gtk_tree_view_get_search_column     (GtkTreeView                *tree_view);
void                       gtk_tree_view_set_search_column     (GtkTreeView                *tree_view,
								gint                        column);
GtkTreeViewSearchEqualFunc gtk_tree_view_get_search_equal_func (GtkTreeView                *tree_view);
void                       gtk_tree_view_set_search_equal_func (GtkTreeView                *tree_view,
								GtkTreeViewSearchEqualFunc  search_equal_func,
								gpointer                    search_user_data,
								GDestroyNotify              search_destroy);

GtkEntry                     *gtk_tree_view_get_search_entry         (GtkTreeView                   *tree_view);
void                          gtk_tree_view_set_search_entry         (GtkTreeView                   *tree_view,
								      GtkEntry                      *entry);
GtkTreeViewSearchPositionFunc gtk_tree_view_get_search_position_func (GtkTreeView                   *tree_view);
void                          gtk_tree_view_set_search_position_func (GtkTreeView                   *tree_view,
								      GtkTreeViewSearchPositionFunc  func,
								      gpointer                       data,
								      GDestroyNotify                 destroy);

/* Convert between the different coordinate systems */
void gtk_tree_view_convert_widget_to_tree_coords       (GtkTreeView *tree_view,
							gint         wx,
							gint         wy,
							gint        *tx,
							gint        *ty);
void gtk_tree_view_convert_tree_to_widget_coords       (GtkTreeView *tree_view,
							gint         tx,
							gint         ty,
							gint        *wx,
							gint        *wy);
void gtk_tree_view_convert_widget_to_bin_window_coords (GtkTreeView *tree_view,
							gint         wx,
							gint         wy,
							gint        *bx,
							gint        *by);
void gtk_tree_view_convert_bin_window_to_widget_coords (GtkTreeView *tree_view,
							gint         bx,
							gint         by,
							gint        *wx,
							gint        *wy);
void gtk_tree_view_convert_tree_to_bin_window_coords   (GtkTreeView *tree_view,
							gint         tx,
							gint         ty,
							gint        *bx,
							gint        *by);
void gtk_tree_view_convert_bin_window_to_tree_coords   (GtkTreeView *tree_view,
							gint         bx,
							gint         by,
							gint        *tx,
							gint        *ty);

/* This function should really never be used.  It is just for use by ATK.
 */
typedef void (* GtkTreeDestroyCountFunc)  (GtkTreeView             *tree_view,
					   GtkTreePath             *path,
					   gint                     children,
					   gpointer                 user_data);
void gtk_tree_view_set_destroy_count_func (GtkTreeView             *tree_view,
					   GtkTreeDestroyCountFunc  func,
					   gpointer                 data,
					   GDestroyNotify           destroy);

void     gtk_tree_view_set_fixed_height_mode (GtkTreeView          *tree_view,
					      gboolean              enable);
gboolean gtk_tree_view_get_fixed_height_mode (GtkTreeView          *tree_view);
void     gtk_tree_view_set_hover_selection   (GtkTreeView          *tree_view,
					      gboolean              hover);
gboolean gtk_tree_view_get_hover_selection   (GtkTreeView          *tree_view);
void     gtk_tree_view_set_hover_expand      (GtkTreeView          *tree_view,
					      gboolean              expand);
gboolean gtk_tree_view_get_hover_expand      (GtkTreeView          *tree_view);
void     gtk_tree_view_set_rubber_banding    (GtkTreeView          *tree_view,
					      gboolean              enable);
gboolean gtk_tree_view_get_rubber_banding    (GtkTreeView          *tree_view);

gboolean gtk_tree_view_is_rubber_banding_active (GtkTreeView       *tree_view);

GtkTreeViewRowSeparatorFunc gtk_tree_view_get_row_separator_func (GtkTreeView               *tree_view);
void                        gtk_tree_view_set_row_separator_func (GtkTreeView                *tree_view,
								  GtkTreeViewRowSeparatorFunc func,
								  gpointer                    data,
								  GDestroyNotify              destroy);

GtkTreeViewGridLines        gtk_tree_view_get_grid_lines         (GtkTreeView                *tree_view);
void                        gtk_tree_view_set_grid_lines         (GtkTreeView                *tree_view,
								  GtkTreeViewGridLines        grid_lines);
gboolean                    gtk_tree_view_get_enable_tree_lines  (GtkTreeView                *tree_view);
void                        gtk_tree_view_set_enable_tree_lines  (GtkTreeView                *tree_view,
								  gboolean                    enabled);
void                        gtk_tree_view_set_show_expanders     (GtkTreeView                *tree_view,
								  gboolean                    enabled);
gboolean                    gtk_tree_view_get_show_expanders     (GtkTreeView                *tree_view);
void                        gtk_tree_view_set_level_indentation  (GtkTreeView                *tree_view,
								  gint                        indentation);
gint                        gtk_tree_view_get_level_indentation  (GtkTreeView                *tree_view);

/* Convenience functions for setting tooltips */
void          gtk_tree_view_set_tooltip_row    (GtkTreeView       *tree_view,
						GtkTooltip        *tooltip,
						GtkTreePath       *path);
void          gtk_tree_view_set_tooltip_cell   (GtkTreeView       *tree_view,
						GtkTooltip        *tooltip,
						GtkTreePath       *path,
						GtkTreeViewColumn *column,
						GtkCellRenderer   *cell);
gboolean      gtk_tree_view_get_tooltip_context(GtkTreeView       *tree_view,
						gint              *x,
						gint              *y,
						gboolean           keyboard_tip,
						GtkTreeModel     **model,
						GtkTreePath      **path,
						GtkTreeIter       *iter);
void          gtk_tree_view_set_tooltip_column (GtkTreeView       *tree_view,
					        gint               column);
gint          gtk_tree_view_get_tooltip_column (GtkTreeView       *tree_view);

G_END_DECLS


#endif /* __GTK_TREE_VIEW_H__ */
