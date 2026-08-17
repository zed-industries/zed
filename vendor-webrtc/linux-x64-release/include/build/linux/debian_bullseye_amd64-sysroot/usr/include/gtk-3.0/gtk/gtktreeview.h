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
 * License along with this library. If not, see <http://www.gnu.org/licenses/>.
 */

#ifndef __GTK_TREE_VIEW_H__
#define __GTK_TREE_VIEW_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkcontainer.h>
#include <gtk/gtktreemodel.h>
#include <gtk/gtktreeviewcolumn.h>
#include <gtk/gtkdnd.h>
#include <gtk/gtkentry.h>

G_BEGIN_DECLS

/**
 * GtkTreeViewDropPosition:
 * @GTK_TREE_VIEW_DROP_BEFORE: dropped row is inserted before
 * @GTK_TREE_VIEW_DROP_AFTER: dropped row is inserted after
 * @GTK_TREE_VIEW_DROP_INTO_OR_BEFORE: dropped row becomes a child or is inserted before
 * @GTK_TREE_VIEW_DROP_INTO_OR_AFTER: dropped row becomes a child or is inserted after
 *
 * An enum for determining where a dropped row goes.
 */
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

  /*< private >*/
  GtkTreeViewPrivate *priv;
};

struct _GtkTreeViewClass
{
  GtkContainerClass parent_class;

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
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
  void (*_gtk_reserved5) (void);
  void (*_gtk_reserved6) (void);
  void (*_gtk_reserved7) (void);
  void (*_gtk_reserved8) (void);
};

/**
 * GtkTreeViewColumnDropFunc:
 * @tree_view: A #GtkTreeView
 * @column: The #GtkTreeViewColumn being dragged
 * @prev_column: A #GtkTreeViewColumn on one side of @column
 * @next_column: A #GtkTreeViewColumn on the other side of @column
 * @data: (closure): user data
 *
 * Function type for determining whether @column can be dropped in a
 * particular spot (as determined by @prev_column and @next_column).  In
 * left to right locales, @prev_column is on the left of the potential drop
 * spot, and @next_column is on the right.  In right to left mode, this is
 * reversed.  This function should return %TRUE if the spot is a valid drop
 * spot.  Please note that returning %TRUE does not actually indicate that
 * the column drop was made, but is meant only to indicate a possible drop
 * spot to the user.
 *
 * Returns: %TRUE, if @column can be dropped in this spot
 */
typedef gboolean (* GtkTreeViewColumnDropFunc) (GtkTreeView             *tree_view,
						GtkTreeViewColumn       *column,
						GtkTreeViewColumn       *prev_column,
						GtkTreeViewColumn       *next_column,
						gpointer                 data);

/**
 * GtkTreeViewMappingFunc:
 * @tree_view: A #GtkTreeView
 * @path: The path that’s expanded
 * @user_data: user data
 *
 * Function used for gtk_tree_view_map_expanded_rows().
 */
typedef void     (* GtkTreeViewMappingFunc)    (GtkTreeView             *tree_view,
						GtkTreePath             *path,
						gpointer                 user_data);

/**
 * GtkTreeViewSearchEqualFunc:
 * @model: the #GtkTreeModel being searched
 * @column: the search column set by gtk_tree_view_set_search_column()
 * @key: the key string to compare with
 * @iter: a #GtkTreeIter pointing the row of @model that should be compared
 *  with @key.
 * @search_data: (closure): user data from gtk_tree_view_set_search_equal_func()
 *
 * A function used for checking whether a row in @model matches
 * a search key string entered by the user. Note the return value
 * is reversed from what you would normally expect, though it
 * has some similarity to strcmp() returning 0 for equal strings.
 *
 * Returns: %FALSE if the row matches, %TRUE otherwise.
 */
typedef gboolean (*GtkTreeViewSearchEqualFunc) (GtkTreeModel            *model,
						gint                     column,
						const gchar             *key,
						GtkTreeIter             *iter,
						gpointer                 search_data);

/**
 * GtkTreeViewRowSeparatorFunc:
 * @model: the #GtkTreeModel
 * @iter: a #GtkTreeIter pointing at a row in @model
 * @data: (closure): user data
 *
 * Function type for determining whether the row pointed to by @iter should
 * be rendered as a separator. A common way to implement this is to have a
 * boolean column in the model, whose values the #GtkTreeViewRowSeparatorFunc
 * returns.
 *
 * Returns: %TRUE if the row is a separator
 */
typedef gboolean (*GtkTreeViewRowSeparatorFunc) (GtkTreeModel      *model,
						 GtkTreeIter       *iter,
						 gpointer           data);
typedef void     (*GtkTreeViewSearchPositionFunc) (GtkTreeView  *tree_view,
						   GtkWidget    *search_dialog,
						   gpointer      user_data);


/* Creators */
GDK_AVAILABLE_IN_ALL
GType                  gtk_tree_view_get_type                      (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
GtkWidget             *gtk_tree_view_new                           (void);
GDK_AVAILABLE_IN_ALL
GtkWidget             *gtk_tree_view_new_with_model                (GtkTreeModel              *model);

/* Accessors */
GDK_AVAILABLE_IN_ALL
GtkTreeModel          *gtk_tree_view_get_model                     (GtkTreeView               *tree_view);
GDK_AVAILABLE_IN_ALL
void                   gtk_tree_view_set_model                     (GtkTreeView               *tree_view,
								    GtkTreeModel              *model);
GDK_AVAILABLE_IN_ALL
GtkTreeSelection      *gtk_tree_view_get_selection                 (GtkTreeView               *tree_view);

GDK_DEPRECATED_IN_3_0_FOR(gtk_scrollable_get_hadjustment)
GtkAdjustment         *gtk_tree_view_get_hadjustment               (GtkTreeView               *tree_view);
GDK_DEPRECATED_IN_3_0_FOR(gtk_scrollable_set_hadjustment)
void                   gtk_tree_view_set_hadjustment               (GtkTreeView               *tree_view,
								    GtkAdjustment             *adjustment);
GDK_DEPRECATED_IN_3_0_FOR(gtk_scrollable_get_vadjustment)
GtkAdjustment         *gtk_tree_view_get_vadjustment               (GtkTreeView               *tree_view);
GDK_DEPRECATED_IN_3_0_FOR(gtk_scrollable_set_vadjustment)
void                   gtk_tree_view_set_vadjustment               (GtkTreeView               *tree_view,
								    GtkAdjustment             *adjustment);


GDK_AVAILABLE_IN_ALL
gboolean               gtk_tree_view_get_headers_visible           (GtkTreeView               *tree_view);
GDK_AVAILABLE_IN_ALL
void                   gtk_tree_view_set_headers_visible           (GtkTreeView               *tree_view,
								    gboolean                   headers_visible);
GDK_AVAILABLE_IN_ALL
void                   gtk_tree_view_columns_autosize              (GtkTreeView               *tree_view);
GDK_AVAILABLE_IN_ALL
gboolean               gtk_tree_view_get_headers_clickable         (GtkTreeView *tree_view);
GDK_AVAILABLE_IN_ALL
void                   gtk_tree_view_set_headers_clickable         (GtkTreeView               *tree_view,
								    gboolean                   setting);
GDK_DEPRECATED_IN_3_14
void                   gtk_tree_view_set_rules_hint                (GtkTreeView               *tree_view,
								    gboolean                   setting);
GDK_DEPRECATED_IN_3_14
gboolean               gtk_tree_view_get_rules_hint                (GtkTreeView               *tree_view);
GDK_AVAILABLE_IN_3_8
gboolean               gtk_tree_view_get_activate_on_single_click  (GtkTreeView               *tree_view);
GDK_AVAILABLE_IN_3_8
void                   gtk_tree_view_set_activate_on_single_click  (GtkTreeView               *tree_view,
								    gboolean                   single);

/* Column funtions */
GDK_AVAILABLE_IN_ALL
gint                   gtk_tree_view_append_column                 (GtkTreeView               *tree_view,
								    GtkTreeViewColumn         *column);
GDK_AVAILABLE_IN_ALL
gint                   gtk_tree_view_remove_column                 (GtkTreeView               *tree_view,
								    GtkTreeViewColumn         *column);
GDK_AVAILABLE_IN_ALL
gint                   gtk_tree_view_insert_column                 (GtkTreeView               *tree_view,
								    GtkTreeViewColumn         *column,
								    gint                       position);
GDK_AVAILABLE_IN_ALL
gint                   gtk_tree_view_insert_column_with_attributes (GtkTreeView               *tree_view,
								    gint                       position,
								    const gchar               *title,
								    GtkCellRenderer           *cell,
								    ...) G_GNUC_NULL_TERMINATED;
GDK_AVAILABLE_IN_ALL
gint                   gtk_tree_view_insert_column_with_data_func  (GtkTreeView               *tree_view,
								    gint                       position,
								    const gchar               *title,
								    GtkCellRenderer           *cell,
                                                                    GtkTreeCellDataFunc        func,
                                                                    gpointer                   data,
                                                                    GDestroyNotify             dnotify);

GDK_AVAILABLE_IN_3_4
guint                  gtk_tree_view_get_n_columns                 (GtkTreeView               *tree_view);
GDK_AVAILABLE_IN_ALL
GtkTreeViewColumn     *gtk_tree_view_get_column                    (GtkTreeView               *tree_view,
								    gint                       n);
GDK_AVAILABLE_IN_ALL
GList                 *gtk_tree_view_get_columns                   (GtkTreeView               *tree_view);
GDK_AVAILABLE_IN_ALL
void                   gtk_tree_view_move_column_after             (GtkTreeView               *tree_view,
								    GtkTreeViewColumn         *column,
								    GtkTreeViewColumn         *base_column);
GDK_AVAILABLE_IN_ALL
void                   gtk_tree_view_set_expander_column           (GtkTreeView               *tree_view,
								    GtkTreeViewColumn         *column);
GDK_AVAILABLE_IN_ALL
GtkTreeViewColumn     *gtk_tree_view_get_expander_column           (GtkTreeView               *tree_view);
GDK_AVAILABLE_IN_ALL
void                   gtk_tree_view_set_column_drag_function      (GtkTreeView               *tree_view,
								    GtkTreeViewColumnDropFunc  func,
								    gpointer                   user_data,
								    GDestroyNotify             destroy);

/* Actions */
GDK_AVAILABLE_IN_ALL
void                   gtk_tree_view_scroll_to_point               (GtkTreeView               *tree_view,
								    gint                       tree_x,
								    gint                       tree_y);
GDK_AVAILABLE_IN_ALL
void                   gtk_tree_view_scroll_to_cell                (GtkTreeView               *tree_view,
								    GtkTreePath               *path,
								    GtkTreeViewColumn         *column,
								    gboolean                   use_align,
								    gfloat                     row_align,
								    gfloat                     col_align);
GDK_AVAILABLE_IN_ALL
void                   gtk_tree_view_row_activated                 (GtkTreeView               *tree_view,
								    GtkTreePath               *path,
								    GtkTreeViewColumn         *column);
GDK_AVAILABLE_IN_ALL
void                   gtk_tree_view_expand_all                    (GtkTreeView               *tree_view);
GDK_AVAILABLE_IN_ALL
void                   gtk_tree_view_collapse_all                  (GtkTreeView               *tree_view);
GDK_AVAILABLE_IN_ALL
void                   gtk_tree_view_expand_to_path                (GtkTreeView               *tree_view,
								    GtkTreePath               *path);
GDK_AVAILABLE_IN_ALL
gboolean               gtk_tree_view_expand_row                    (GtkTreeView               *tree_view,
								    GtkTreePath               *path,
								    gboolean                   open_all);
GDK_AVAILABLE_IN_ALL
gboolean               gtk_tree_view_collapse_row                  (GtkTreeView               *tree_view,
								    GtkTreePath               *path);
GDK_AVAILABLE_IN_ALL
void                   gtk_tree_view_map_expanded_rows             (GtkTreeView               *tree_view,
								    GtkTreeViewMappingFunc     func,
								    gpointer                   data);
GDK_AVAILABLE_IN_ALL
gboolean               gtk_tree_view_row_expanded                  (GtkTreeView               *tree_view,
								    GtkTreePath               *path);
GDK_AVAILABLE_IN_ALL
void                   gtk_tree_view_set_reorderable               (GtkTreeView               *tree_view,
								    gboolean                   reorderable);
GDK_AVAILABLE_IN_ALL
gboolean               gtk_tree_view_get_reorderable               (GtkTreeView               *tree_view);
GDK_AVAILABLE_IN_ALL
void                   gtk_tree_view_set_cursor                    (GtkTreeView               *tree_view,
								    GtkTreePath               *path,
								    GtkTreeViewColumn         *focus_column,
								    gboolean                   start_editing);
GDK_AVAILABLE_IN_ALL
void                   gtk_tree_view_set_cursor_on_cell            (GtkTreeView               *tree_view,
								    GtkTreePath               *path,
								    GtkTreeViewColumn         *focus_column,
								    GtkCellRenderer           *focus_cell,
								    gboolean                   start_editing);
GDK_AVAILABLE_IN_ALL
void                   gtk_tree_view_get_cursor                    (GtkTreeView               *tree_view,
								    GtkTreePath              **path,
								    GtkTreeViewColumn        **focus_column);


/* Layout information */
GDK_AVAILABLE_IN_ALL
GdkWindow             *gtk_tree_view_get_bin_window                (GtkTreeView               *tree_view);
GDK_AVAILABLE_IN_ALL
gboolean               gtk_tree_view_get_path_at_pos               (GtkTreeView               *tree_view,
								    gint                       x,
								    gint                       y,
								    GtkTreePath              **path,
								    GtkTreeViewColumn        **column,
								    gint                      *cell_x,
								    gint                      *cell_y);
GDK_AVAILABLE_IN_ALL
void                   gtk_tree_view_get_cell_area                 (GtkTreeView               *tree_view,
								    GtkTreePath               *path,
								    GtkTreeViewColumn         *column,
								    GdkRectangle              *rect);
GDK_AVAILABLE_IN_ALL
void                   gtk_tree_view_get_background_area           (GtkTreeView               *tree_view,
								    GtkTreePath               *path,
								    GtkTreeViewColumn         *column,
								    GdkRectangle              *rect);
GDK_AVAILABLE_IN_ALL
void                   gtk_tree_view_get_visible_rect              (GtkTreeView               *tree_view,
								    GdkRectangle              *visible_rect);
GDK_AVAILABLE_IN_ALL
gboolean               gtk_tree_view_get_visible_range             (GtkTreeView               *tree_view,
								    GtkTreePath              **start_path,
								    GtkTreePath              **end_path);
GDK_AVAILABLE_IN_ALL
gboolean               gtk_tree_view_is_blank_at_pos               (GtkTreeView               *tree_view,
                                                                    gint                       x,
                                                                    gint                       y,
                                                                    GtkTreePath              **path,
                                                                    GtkTreeViewColumn        **column,
                                                                    gint                      *cell_x,
                                                                    gint                      *cell_y);

/* Drag-and-Drop support */
GDK_AVAILABLE_IN_ALL
void                   gtk_tree_view_enable_model_drag_source      (GtkTreeView               *tree_view,
								    GdkModifierType            start_button_mask,
								    const GtkTargetEntry      *targets,
								    gint                       n_targets,
								    GdkDragAction              actions);
GDK_AVAILABLE_IN_ALL
void                   gtk_tree_view_enable_model_drag_dest        (GtkTreeView               *tree_view,
								    const GtkTargetEntry      *targets,
								    gint                       n_targets,
								    GdkDragAction              actions);
GDK_AVAILABLE_IN_ALL
void                   gtk_tree_view_unset_rows_drag_source        (GtkTreeView               *tree_view);
GDK_AVAILABLE_IN_ALL
void                   gtk_tree_view_unset_rows_drag_dest          (GtkTreeView               *tree_view);


/* These are useful to implement your own custom stuff. */
GDK_AVAILABLE_IN_ALL
void                   gtk_tree_view_set_drag_dest_row             (GtkTreeView               *tree_view,
								    GtkTreePath               *path,
								    GtkTreeViewDropPosition    pos);
GDK_AVAILABLE_IN_ALL
void                   gtk_tree_view_get_drag_dest_row             (GtkTreeView               *tree_view,
								    GtkTreePath              **path,
								    GtkTreeViewDropPosition   *pos);
GDK_AVAILABLE_IN_ALL
gboolean               gtk_tree_view_get_dest_row_at_pos           (GtkTreeView               *tree_view,
								    gint                       drag_x,
								    gint                       drag_y,
								    GtkTreePath              **path,
								    GtkTreeViewDropPosition   *pos);
GDK_AVAILABLE_IN_ALL
cairo_surface_t       *gtk_tree_view_create_row_drag_icon          (GtkTreeView               *tree_view,
								    GtkTreePath               *path);

/* Interactive search */
GDK_AVAILABLE_IN_ALL
void                       gtk_tree_view_set_enable_search     (GtkTreeView                *tree_view,
								gboolean                    enable_search);
GDK_AVAILABLE_IN_ALL
gboolean                   gtk_tree_view_get_enable_search     (GtkTreeView                *tree_view);
GDK_AVAILABLE_IN_ALL
gint                       gtk_tree_view_get_search_column     (GtkTreeView                *tree_view);
GDK_AVAILABLE_IN_ALL
void                       gtk_tree_view_set_search_column     (GtkTreeView                *tree_view,
								gint                        column);
GDK_AVAILABLE_IN_ALL
GtkTreeViewSearchEqualFunc gtk_tree_view_get_search_equal_func (GtkTreeView                *tree_view);
GDK_AVAILABLE_IN_ALL
void                       gtk_tree_view_set_search_equal_func (GtkTreeView                *tree_view,
								GtkTreeViewSearchEqualFunc  search_equal_func,
								gpointer                    search_user_data,
								GDestroyNotify              search_destroy);

GDK_AVAILABLE_IN_ALL
GtkEntry                     *gtk_tree_view_get_search_entry         (GtkTreeView                   *tree_view);
GDK_AVAILABLE_IN_ALL
void                          gtk_tree_view_set_search_entry         (GtkTreeView                   *tree_view,
								      GtkEntry                      *entry);
GDK_AVAILABLE_IN_ALL
GtkTreeViewSearchPositionFunc gtk_tree_view_get_search_position_func (GtkTreeView                   *tree_view);
GDK_AVAILABLE_IN_ALL
void                          gtk_tree_view_set_search_position_func (GtkTreeView                   *tree_view,
								      GtkTreeViewSearchPositionFunc  func,
								      gpointer                       data,
								      GDestroyNotify                 destroy);

/* Convert between the different coordinate systems */
GDK_AVAILABLE_IN_ALL
void gtk_tree_view_convert_widget_to_tree_coords       (GtkTreeView *tree_view,
							gint         wx,
							gint         wy,
							gint        *tx,
							gint        *ty);
GDK_AVAILABLE_IN_ALL
void gtk_tree_view_convert_tree_to_widget_coords       (GtkTreeView *tree_view,
							gint         tx,
							gint         ty,
							gint        *wx,
							gint        *wy);
GDK_AVAILABLE_IN_ALL
void gtk_tree_view_convert_widget_to_bin_window_coords (GtkTreeView *tree_view,
							gint         wx,
							gint         wy,
							gint        *bx,
							gint        *by);
GDK_AVAILABLE_IN_ALL
void gtk_tree_view_convert_bin_window_to_widget_coords (GtkTreeView *tree_view,
							gint         bx,
							gint         by,
							gint        *wx,
							gint        *wy);
GDK_AVAILABLE_IN_ALL
void gtk_tree_view_convert_tree_to_bin_window_coords   (GtkTreeView *tree_view,
							gint         tx,
							gint         ty,
							gint        *bx,
							gint        *by);
GDK_AVAILABLE_IN_ALL
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
GDK_DEPRECATED_IN_3_4
void gtk_tree_view_set_destroy_count_func (GtkTreeView             *tree_view,
					   GtkTreeDestroyCountFunc  func,
					   gpointer                 data,
					   GDestroyNotify           destroy);

GDK_AVAILABLE_IN_ALL
void     gtk_tree_view_set_fixed_height_mode (GtkTreeView          *tree_view,
					      gboolean              enable);
GDK_AVAILABLE_IN_ALL
gboolean gtk_tree_view_get_fixed_height_mode (GtkTreeView          *tree_view);
GDK_AVAILABLE_IN_ALL
void     gtk_tree_view_set_hover_selection   (GtkTreeView          *tree_view,
					      gboolean              hover);
GDK_AVAILABLE_IN_ALL
gboolean gtk_tree_view_get_hover_selection   (GtkTreeView          *tree_view);
GDK_AVAILABLE_IN_ALL
void     gtk_tree_view_set_hover_expand      (GtkTreeView          *tree_view,
					      gboolean              expand);
GDK_AVAILABLE_IN_ALL
gboolean gtk_tree_view_get_hover_expand      (GtkTreeView          *tree_view);
GDK_AVAILABLE_IN_ALL
void     gtk_tree_view_set_rubber_banding    (GtkTreeView          *tree_view,
					      gboolean              enable);
GDK_AVAILABLE_IN_ALL
gboolean gtk_tree_view_get_rubber_banding    (GtkTreeView          *tree_view);

GDK_AVAILABLE_IN_ALL
gboolean gtk_tree_view_is_rubber_banding_active (GtkTreeView       *tree_view);

GDK_AVAILABLE_IN_ALL
GtkTreeViewRowSeparatorFunc gtk_tree_view_get_row_separator_func (GtkTreeView               *tree_view);
GDK_AVAILABLE_IN_ALL
void                        gtk_tree_view_set_row_separator_func (GtkTreeView                *tree_view,
								  GtkTreeViewRowSeparatorFunc func,
								  gpointer                    data,
								  GDestroyNotify              destroy);

GDK_AVAILABLE_IN_ALL
GtkTreeViewGridLines        gtk_tree_view_get_grid_lines         (GtkTreeView                *tree_view);
GDK_AVAILABLE_IN_ALL
void                        gtk_tree_view_set_grid_lines         (GtkTreeView                *tree_view,
								  GtkTreeViewGridLines        grid_lines);
GDK_AVAILABLE_IN_ALL
gboolean                    gtk_tree_view_get_enable_tree_lines  (GtkTreeView                *tree_view);
GDK_AVAILABLE_IN_ALL
void                        gtk_tree_view_set_enable_tree_lines  (GtkTreeView                *tree_view,
								  gboolean                    enabled);
GDK_AVAILABLE_IN_ALL
void                        gtk_tree_view_set_show_expanders     (GtkTreeView                *tree_view,
								  gboolean                    enabled);
GDK_AVAILABLE_IN_ALL
gboolean                    gtk_tree_view_get_show_expanders     (GtkTreeView                *tree_view);
GDK_AVAILABLE_IN_ALL
void                        gtk_tree_view_set_level_indentation  (GtkTreeView                *tree_view,
								  gint                        indentation);
GDK_AVAILABLE_IN_ALL
gint                        gtk_tree_view_get_level_indentation  (GtkTreeView                *tree_view);

/* Convenience functions for setting tooltips */
GDK_AVAILABLE_IN_ALL
void          gtk_tree_view_set_tooltip_row    (GtkTreeView       *tree_view,
						GtkTooltip        *tooltip,
						GtkTreePath       *path);
GDK_AVAILABLE_IN_ALL
void          gtk_tree_view_set_tooltip_cell   (GtkTreeView       *tree_view,
						GtkTooltip        *tooltip,
						GtkTreePath       *path,
						GtkTreeViewColumn *column,
						GtkCellRenderer   *cell);
GDK_AVAILABLE_IN_ALL
gboolean      gtk_tree_view_get_tooltip_context(GtkTreeView       *tree_view,
						gint              *x,
						gint              *y,
						gboolean           keyboard_tip,
						GtkTreeModel     **model,
						GtkTreePath      **path,
						GtkTreeIter       *iter);
GDK_AVAILABLE_IN_ALL
void          gtk_tree_view_set_tooltip_column (GtkTreeView       *tree_view,
					        gint               column);
GDK_AVAILABLE_IN_ALL
gint          gtk_tree_view_get_tooltip_column (GtkTreeView       *tree_view);

G_END_DECLS


#endif /* __GTK_TREE_VIEW_H__ */
