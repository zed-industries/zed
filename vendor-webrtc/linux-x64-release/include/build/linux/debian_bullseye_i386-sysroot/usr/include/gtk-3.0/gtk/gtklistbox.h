/*
 * Copyright (c) 2013 Red Hat, Inc.
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or (at your
 * option) any later version.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
 * or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public
 * License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with this program; if not, write to the Free Software Foundation,
 * Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
 *
 * Author: Alexander Larsson <alexl@redhat.com>
 *
 */

#ifndef __GTK_LIST_BOX_H__
#define __GTK_LIST_BOX_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkbin.h>

G_BEGIN_DECLS


#define GTK_TYPE_LIST_BOX (gtk_list_box_get_type ())
#define GTK_LIST_BOX(obj) (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_LIST_BOX, GtkListBox))
#define GTK_LIST_BOX_CLASS(klass) (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_LIST_BOX, GtkListBoxClass))
#define GTK_IS_LIST_BOX(obj) (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_LIST_BOX))
#define GTK_IS_LIST_BOX_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_LIST_BOX))
#define GTK_LIST_BOX_GET_CLASS(obj) (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_LIST_BOX, GtkListBoxClass))

typedef struct _GtkListBox        GtkListBox;
typedef struct _GtkListBoxClass   GtkListBoxClass;

typedef struct _GtkListBoxRow        GtkListBoxRow;
typedef struct _GtkListBoxRowClass   GtkListBoxRowClass;

struct _GtkListBox
{
  GtkContainer parent_instance;
};

/**
 * GtkListBoxClass:
 * @parent_class: The parent class.
 * @row_selected: Class handler for the #GtkListBox::row-selected signal
 * @row_activated: Class handler for the #GtkListBox::row-activated signal
 * @activate_cursor_row: Class handler for the #GtkListBox::activate-cursor-row signal
 * @toggle_cursor_row: Class handler for the #GtkListBox::toggle-cursor-row signal
 * @move_cursor: Class handler for the #GtkListBox::move-cursor signal
 * @selected_rows_changed: Class handler for the #GtkListBox::selected-rows-changed signal
 * @select_all: Class handler for the #GtkListBox::select-all signal
 * @unselect_all: Class handler for the #GtkListBox::unselect-all signal
 */
struct _GtkListBoxClass
{
  GtkContainerClass parent_class;

  /*< public >*/

  void (*row_selected)        (GtkListBox      *box,
                               GtkListBoxRow   *row);
  void (*row_activated)       (GtkListBox      *box,
                               GtkListBoxRow   *row);
  void (*activate_cursor_row) (GtkListBox      *box);
  void (*toggle_cursor_row)   (GtkListBox      *box);
  void (*move_cursor)         (GtkListBox      *box,
                               GtkMovementStep  step,
                               gint             count);
  void (*selected_rows_changed) (GtkListBox    *box);
  void (*select_all)            (GtkListBox    *box);
  void (*unselect_all)          (GtkListBox    *box);

  /*< private >*/

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
};

#define GTK_TYPE_LIST_BOX_ROW            (gtk_list_box_row_get_type ())
#define GTK_LIST_BOX_ROW(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_LIST_BOX_ROW, GtkListBoxRow))
#define GTK_LIST_BOX_ROW_CLASS(klass)    (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_LIST_BOX_ROW, GtkListBoxRowClass))
#define GTK_IS_LIST_BOX_ROW(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_LIST_BOX_ROW))
#define GTK_IS_LIST_BOX_ROW_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_LIST_BOX_ROW))
#define GTK_LIST_BOX_ROW_GET_CLASS(obj)  (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_LIST_BOX_ROW, GtkListBoxRowClass))

struct _GtkListBoxRow
{
  GtkBin parent_instance;
};

/**
 * GtkListBoxRowClass:
 * @parent_class: The parent class.
 * @activate: 
 */
struct _GtkListBoxRowClass
{
  GtkBinClass parent_class;

  /*< public >*/

  void (* activate) (GtkListBoxRow *row);

  /*< private >*/

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
};

/**
 * GtkListBoxFilterFunc:
 * @row: the row that may be filtered
 * @user_data: (closure): user data
 *
 * Will be called whenever the row changes or is added and lets you control
 * if the row should be visible or not.
 *
 * Returns: %TRUE if the row should be visible, %FALSE otherwise
 *
 * Since: 3.10
 */
typedef gboolean (*GtkListBoxFilterFunc) (GtkListBoxRow *row,
                                          gpointer       user_data);

/**
 * GtkListBoxSortFunc:
 * @row1: the first row
 * @row2: the second row
 * @user_data: (closure): user data
 *
 * Compare two rows to determine which should be first.
 *
 * Returns: < 0 if @row1 should be before @row2, 0 if they are
 *     equal and > 0 otherwise
 *
 * Since: 3.10
 */
typedef gint (*GtkListBoxSortFunc) (GtkListBoxRow *row1,
                                    GtkListBoxRow *row2,
                                    gpointer       user_data);

/**
 * GtkListBoxUpdateHeaderFunc:
 * @row: the row to update
 * @before: (allow-none): the row before @row, or %NULL if it is first
 * @user_data: (closure): user data
 *
 * Whenever @row changes or which row is before @row changes this
 * is called, which lets you update the header on @row. You may
 * remove or set a new one via gtk_list_box_row_set_header() or
 * just change the state of the current header widget.
 *
 * Since: 3.10
 */
typedef void (*GtkListBoxUpdateHeaderFunc) (GtkListBoxRow *row,
                                            GtkListBoxRow *before,
                                            gpointer       user_data);

/**
 * GtkListBoxCreateWidgetFunc:
 * @item: (type GObject): the item from the model for which to create a widget for
 * @user_data: (closure): user data
 *
 * Called for list boxes that are bound to a #GListModel with
 * gtk_list_box_bind_model() for each item that gets added to the model.
 *
 * Versions of GTK+ prior to 3.18 called gtk_widget_show_all() on the rows
 * created by the GtkListBoxCreateWidgetFunc, but this forced all widgets
 * inside the row to be shown, and is no longer the case. Applications should
 * be updated to show the desired row widgets.
 *
 * Returns: (transfer full): a #GtkWidget that represents @item
 *
 * Since: 3.16
 */
typedef GtkWidget * (*GtkListBoxCreateWidgetFunc) (gpointer item,
                                                   gpointer user_data);

GDK_AVAILABLE_IN_3_10
GType      gtk_list_box_row_get_type      (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_3_10
GtkWidget* gtk_list_box_row_new           (void);
GDK_AVAILABLE_IN_3_10
GtkWidget* gtk_list_box_row_get_header    (GtkListBoxRow *row);
GDK_AVAILABLE_IN_3_10
void       gtk_list_box_row_set_header    (GtkListBoxRow *row,
                                           GtkWidget     *header);
GDK_AVAILABLE_IN_3_10
gint       gtk_list_box_row_get_index     (GtkListBoxRow *row);
GDK_AVAILABLE_IN_3_10
void       gtk_list_box_row_changed       (GtkListBoxRow *row);

GDK_AVAILABLE_IN_3_14
gboolean   gtk_list_box_row_is_selected   (GtkListBoxRow *row);

GDK_AVAILABLE_IN_3_14
void       gtk_list_box_row_set_selectable (GtkListBoxRow *row,
                                            gboolean       selectable);
GDK_AVAILABLE_IN_3_14
gboolean   gtk_list_box_row_get_selectable (GtkListBoxRow *row);


GDK_AVAILABLE_IN_3_14
void       gtk_list_box_row_set_activatable (GtkListBoxRow *row,
                                             gboolean       activatable);
GDK_AVAILABLE_IN_3_14
gboolean   gtk_list_box_row_get_activatable (GtkListBoxRow *row);

GDK_AVAILABLE_IN_3_10
GType          gtk_list_box_get_type                     (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_3_10
void           gtk_list_box_prepend                      (GtkListBox                    *box,
                                                          GtkWidget                     *child);
GDK_AVAILABLE_IN_3_10
void           gtk_list_box_insert                       (GtkListBox                    *box,
                                                          GtkWidget                     *child,
                                                          gint                           position);
GDK_AVAILABLE_IN_3_10
GtkListBoxRow* gtk_list_box_get_selected_row             (GtkListBox                    *box);
GDK_AVAILABLE_IN_3_10
GtkListBoxRow* gtk_list_box_get_row_at_index             (GtkListBox                    *box,
                                                          gint                           index_);
GDK_AVAILABLE_IN_3_10
GtkListBoxRow* gtk_list_box_get_row_at_y                 (GtkListBox                    *box,
                                                          gint                           y);
GDK_AVAILABLE_IN_3_10
void           gtk_list_box_select_row                   (GtkListBox                    *box,
                                                          GtkListBoxRow                 *row);
GDK_AVAILABLE_IN_3_10
void           gtk_list_box_set_placeholder              (GtkListBox                    *box,
                                                          GtkWidget                     *placeholder);
GDK_AVAILABLE_IN_3_10
void           gtk_list_box_set_adjustment               (GtkListBox                    *box,
                                                          GtkAdjustment                 *adjustment);
GDK_AVAILABLE_IN_3_10
GtkAdjustment *gtk_list_box_get_adjustment               (GtkListBox                    *box);

typedef void (* GtkListBoxForeachFunc) (GtkListBox      *box,
                                        GtkListBoxRow   *row,
                                        gpointer         user_data);

GDK_AVAILABLE_IN_3_14
void           gtk_list_box_selected_foreach             (GtkListBox                    *box,
                                                          GtkListBoxForeachFunc          func,
                                                          gpointer                       data);
GDK_AVAILABLE_IN_3_14
GList         *gtk_list_box_get_selected_rows            (GtkListBox                    *box);
GDK_AVAILABLE_IN_3_14
void           gtk_list_box_unselect_row                 (GtkListBox                    *box,
                                                          GtkListBoxRow                 *row);
GDK_AVAILABLE_IN_3_14
void           gtk_list_box_select_all                   (GtkListBox                    *box);
GDK_AVAILABLE_IN_3_14
void           gtk_list_box_unselect_all                 (GtkListBox                    *box);

GDK_AVAILABLE_IN_3_10
void           gtk_list_box_set_selection_mode           (GtkListBox                    *box,
                                                          GtkSelectionMode               mode);
GDK_AVAILABLE_IN_3_10
GtkSelectionMode gtk_list_box_get_selection_mode         (GtkListBox                    *box);
GDK_AVAILABLE_IN_3_10
void           gtk_list_box_set_filter_func              (GtkListBox                    *box,
                                                          GtkListBoxFilterFunc           filter_func,
                                                          gpointer                       user_data,
                                                          GDestroyNotify                 destroy);
GDK_AVAILABLE_IN_3_10
void           gtk_list_box_set_header_func              (GtkListBox                    *box,
                                                          GtkListBoxUpdateHeaderFunc     update_header,
                                                          gpointer                       user_data,
                                                          GDestroyNotify                 destroy);
GDK_AVAILABLE_IN_3_10
void           gtk_list_box_invalidate_filter            (GtkListBox                    *box);
GDK_AVAILABLE_IN_3_10
void           gtk_list_box_invalidate_sort              (GtkListBox                    *box);
GDK_AVAILABLE_IN_3_10
void           gtk_list_box_invalidate_headers           (GtkListBox                    *box);
GDK_AVAILABLE_IN_3_10
void           gtk_list_box_set_sort_func                (GtkListBox                    *box,
                                                          GtkListBoxSortFunc             sort_func,
                                                          gpointer                       user_data,
                                                          GDestroyNotify                 destroy);
GDK_AVAILABLE_IN_3_10
void           gtk_list_box_set_activate_on_single_click (GtkListBox                    *box,
                                                          gboolean                       single);
GDK_AVAILABLE_IN_3_10
gboolean       gtk_list_box_get_activate_on_single_click (GtkListBox                    *box);
GDK_AVAILABLE_IN_3_10
void           gtk_list_box_drag_unhighlight_row         (GtkListBox                    *box);
GDK_AVAILABLE_IN_3_10
void           gtk_list_box_drag_highlight_row           (GtkListBox                    *box,
                                                          GtkListBoxRow                 *row);
GDK_AVAILABLE_IN_3_10
GtkWidget*     gtk_list_box_new                          (void);


GDK_AVAILABLE_IN_3_16
void           gtk_list_box_bind_model                   (GtkListBox                   *box,
                                                          GListModel                   *model,
                                                          GtkListBoxCreateWidgetFunc    create_widget_func,
                                                          gpointer                      user_data,
                                                          GDestroyNotify                user_data_free_func);

G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkListBox, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkListBoxRow, g_object_unref)

G_END_DECLS

#endif
