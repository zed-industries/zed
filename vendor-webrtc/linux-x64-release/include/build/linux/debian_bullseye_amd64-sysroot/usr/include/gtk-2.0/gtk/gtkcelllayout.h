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
 * License along with this library; if not, write to the
 * Free Software Foundation, Inc., 59 Temple Place - Suite 330,
 * Boston, MA 02111-1307, USA.
 */

#ifndef __GTK_CELL_LAYOUT_H__
#define __GTK_CELL_LAYOUT_H__

#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkcellrenderer.h>
#include <gtk/gtktreeviewcolumn.h>
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
typedef void (* GtkCellLayoutDataFunc) (GtkCellLayout   *cell_layout,
                                        GtkCellRenderer *cell,
                                        GtkTreeModel    *tree_model,
                                        GtkTreeIter     *iter,
                                        gpointer         data);

struct _GtkCellLayoutIface
{
  GTypeInterface g_iface;

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
};

GType gtk_cell_layout_get_type           (void) G_GNUC_CONST;
void  gtk_cell_layout_pack_start         (GtkCellLayout         *cell_layout,
                                          GtkCellRenderer       *cell,
                                          gboolean               expand);
void  gtk_cell_layout_pack_end           (GtkCellLayout         *cell_layout,
                                          GtkCellRenderer       *cell,
                                          gboolean               expand);
GList *gtk_cell_layout_get_cells         (GtkCellLayout         *cell_layout);
void  gtk_cell_layout_clear              (GtkCellLayout         *cell_layout);
void  gtk_cell_layout_set_attributes     (GtkCellLayout         *cell_layout,
                                          GtkCellRenderer       *cell,
                                          ...) G_GNUC_NULL_TERMINATED;
void  gtk_cell_layout_add_attribute      (GtkCellLayout         *cell_layout,
                                          GtkCellRenderer       *cell,
                                          const gchar           *attribute,
                                          gint                   column);
void  gtk_cell_layout_set_cell_data_func (GtkCellLayout         *cell_layout,
                                          GtkCellRenderer       *cell,
                                          GtkCellLayoutDataFunc  func,
                                          gpointer               func_data,
                                          GDestroyNotify         destroy);
void  gtk_cell_layout_clear_attributes   (GtkCellLayout         *cell_layout,
                                          GtkCellRenderer       *cell);
void  gtk_cell_layout_reorder            (GtkCellLayout         *cell_layout,
                                          GtkCellRenderer       *cell,
                                          gint                   position);
gboolean _gtk_cell_layout_buildable_custom_tag_start (GtkBuildable  *buildable,
						      GtkBuilder    *builder,
						      GObject       *child,
						      const gchar   *tagname,
						      GMarkupParser *parser,
						      gpointer      *data);
void _gtk_cell_layout_buildable_custom_tag_end       (GtkBuildable  *buildable,
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
