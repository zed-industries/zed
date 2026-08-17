/* gtkcelllayout.h
 * Copyright (C) 2003  Kristian Rietveld  <kris@gtk.org>
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

#ifndef __GTK_CELL_LAYOUT_H__
#define __GTK_CELL_LAYOUT_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkcellrenderer.h>
#include <gtk/gtkcellarea.h>
#include <gtk/gtkbuildable.h>
#include <gtk/gtkbuilder.h>

G_BEGIN_DECLS

#define GTK_TYPE_CELL_LAYOUT            (gtk_cell_layout_get_type ())
#define GTK_CELL_LAYOUT(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_CELL_LAYOUT, GtkCellLayout))
#define GTK_IS_CELL_LAYOUT(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_CELL_LAYOUT))
#define GTK_CELL_LAYOUT_GET_IFACE(obj)  (G_TYPE_INSTANCE_GET_INTERFACE ((obj), GTK_TYPE_CELL_LAYOUT, GtkCellLayoutIface))

typedef struct _GtkCellLayout           GtkCellLayout; /* dummy typedef */
typedef struct _GtkCellLayoutIface      GtkCellLayoutIface;

/* keep in sync with GtkTreeCellDataFunc */
/**
 * GtkCellLayoutDataFunc:
 * @cell_layout: a #GtkCellLayout
 * @cell: the cell renderer whose value is to be set
 * @tree_model: the model
 * @iter: a #GtkTreeIter indicating the row to set the value for
 * @data: (closure): user data passed to gtk_cell_layout_set_cell_data_func()
 *
 * A function which should set the value of @cell_layout’s cell renderer(s)
 * as appropriate. 
 */
typedef void (* GtkCellLayoutDataFunc) (GtkCellLayout   *cell_layout,
                                        GtkCellRenderer *cell,
                                        GtkTreeModel    *tree_model,
                                        GtkTreeIter     *iter,
                                        gpointer         data);

/**
 * GtkCellLayoutIface:
 * @pack_start: Packs the cell into the beginning of cell_layout.
 * @pack_end: Adds the cell to the end of cell_layout.
 * @clear: Unsets all the mappings on all renderers on cell_layout and
 *    removes all renderers from cell_layout.
 * @add_attribute: Adds an attribute mapping to the list in
 *    cell_layout.
 * @set_cell_data_func: Sets the #GtkCellLayoutDataFunc to use for
 *    cell_layout.
 * @clear_attributes: Clears all existing attributes previously set
 *    with gtk_cell_layout_set_attributes().
 * @reorder: Re-inserts cell at position.
 * @get_cells: Get the cell renderers which have been added to
 *    cell_layout.
 * @get_area: Get the underlying #GtkCellArea which might be
 *    cell_layout if called on a #GtkCellArea or might be NULL if no
 *    #GtkCellArea is used by cell_layout.
 */
struct _GtkCellLayoutIface
{
  /*< private >*/
  GTypeInterface g_iface;

  /*< public >*/

  /* Virtual Table */
  void (* pack_start)         (GtkCellLayout         *cell_layout,
                               GtkCellRenderer       *cell,
                               gboolean               expand);
  void (* pack_end)           (GtkCellLayout         *cell_layout,
                               GtkCellRenderer       *cell,
                               gboolean               expand);
  void (* clear)              (GtkCellLayout         *cell_layout);
  void (* add_attribute)      (GtkCellLayout         *cell_layout,
                               GtkCellRenderer       *cell,
                               const gchar           *attribute,
                               gint                   column);
  void (* set_cell_data_func) (GtkCellLayout         *cell_layout,
                               GtkCellRenderer       *cell,
                               GtkCellLayoutDataFunc  func,
                               gpointer               func_data,
                               GDestroyNotify         destroy);
  void (* clear_attributes)   (GtkCellLayout         *cell_layout,
                               GtkCellRenderer       *cell);
  void (* reorder)            (GtkCellLayout         *cell_layout,
                               GtkCellRenderer       *cell,
                               gint                   position);
  GList* (* get_cells)        (GtkCellLayout         *cell_layout);

  GtkCellArea *(* get_area)   (GtkCellLayout         *cell_layout);
};

GDK_AVAILABLE_IN_ALL
GType gtk_cell_layout_get_type           (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
void  gtk_cell_layout_pack_start         (GtkCellLayout         *cell_layout,
                                          GtkCellRenderer       *cell,
                                          gboolean               expand);
GDK_AVAILABLE_IN_ALL
void  gtk_cell_layout_pack_end           (GtkCellLayout         *cell_layout,
                                          GtkCellRenderer       *cell,
                                          gboolean               expand);
GDK_AVAILABLE_IN_ALL
GList *gtk_cell_layout_get_cells         (GtkCellLayout         *cell_layout);
GDK_AVAILABLE_IN_ALL
void  gtk_cell_layout_clear              (GtkCellLayout         *cell_layout);
GDK_AVAILABLE_IN_ALL
void  gtk_cell_layout_set_attributes     (GtkCellLayout         *cell_layout,
                                          GtkCellRenderer       *cell,
                                          ...) G_GNUC_NULL_TERMINATED;
GDK_AVAILABLE_IN_ALL
void  gtk_cell_layout_add_attribute      (GtkCellLayout         *cell_layout,
                                          GtkCellRenderer       *cell,
                                          const gchar           *attribute,
                                          gint                   column);
GDK_AVAILABLE_IN_ALL
void  gtk_cell_layout_set_cell_data_func (GtkCellLayout         *cell_layout,
                                          GtkCellRenderer       *cell,
                                          GtkCellLayoutDataFunc  func,
                                          gpointer               func_data,
                                          GDestroyNotify         destroy);
GDK_AVAILABLE_IN_ALL
void  gtk_cell_layout_clear_attributes   (GtkCellLayout         *cell_layout,
                                          GtkCellRenderer       *cell);
GDK_AVAILABLE_IN_ALL
void  gtk_cell_layout_reorder            (GtkCellLayout         *cell_layout,
                                          GtkCellRenderer       *cell,
                                          gint                   position);
GDK_AVAILABLE_IN_ALL
GtkCellArea *gtk_cell_layout_get_area    (GtkCellLayout         *cell_layout);

gboolean _gtk_cell_layout_buildable_custom_tag_start (GtkBuildable  *buildable,
						      GtkBuilder    *builder,
						      GObject       *child,
						      const gchar   *tagname,
						      GMarkupParser *parser,
						      gpointer      *data);
gboolean _gtk_cell_layout_buildable_custom_tag_end   (GtkBuildable  *buildable,
						      GtkBuilder    *builder,
						      GObject       *child,
						      const gchar   *tagname,
						      gpointer      *data);
void _gtk_cell_layout_buildable_add_child            (GtkBuildable  *buildable,
						      GtkBuilder    *builder,
						      GObject       *child,
						      const gchar   *type);

G_END_DECLS

#endif /* __GTK_CELL_LAYOUT_H__ */
