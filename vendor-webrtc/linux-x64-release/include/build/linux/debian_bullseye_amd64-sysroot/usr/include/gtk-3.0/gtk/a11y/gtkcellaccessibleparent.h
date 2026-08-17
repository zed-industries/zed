/* GTK+ - accessibility implementations
 * Copyright 2001 Sun Microsystems Inc.
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

#ifndef __GTK_CELL_ACCESSIBLE_PARENT_H__
#define __GTK_CELL_ACCESSIBLE_PARENT_H__

#if !defined (__GTK_A11Y_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk-a11y.h> can be included directly."
#endif

#include <atk/atk.h>
#include <gtk/a11y/gtkcellaccessible.h>

G_BEGIN_DECLS

/*
 * The GtkCellAccessibleParent interface should be supported by any object
 * which contains children which are flyweights, i.e. do not have corresponding
 * widgets and the children need help from their parent to provide
 * functionality. One example is GtkTreeViewAccessible where the children
 * GtkCellAccessible need help from the GtkTreeViewAccessible in order to
 * implement atk_component_get_extents().
 */

#define GTK_TYPE_CELL_ACCESSIBLE_PARENT            (gtk_cell_accessible_parent_get_type ())
#define GTK_IS_CELL_ACCESSIBLE_PARENT(obj)         G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_CELL_ACCESSIBLE_PARENT)
#define GTK_CELL_ACCESSIBLE_PARENT(obj)            G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_CELL_ACCESSIBLE_PARENT, GtkCellAccessibleParent)
#define GTK_CELL_ACCESSIBLE_PARENT_GET_IFACE(obj)  (G_TYPE_INSTANCE_GET_INTERFACE ((obj), GTK_TYPE_CELL_ACCESSIBLE_PARENT, GtkCellAccessibleParentIface))

typedef struct _GtkCellAccessibleParent GtkCellAccessibleParent;
typedef struct _GtkCellAccessibleParentIface GtkCellAccessibleParentIface;

struct _GtkCellAccessibleParentIface
{
  GTypeInterface parent;
  void     ( *get_cell_extents) (GtkCellAccessibleParent *parent,
                                 GtkCellAccessible       *cell,
                                 gint                    *x,
                                 gint                    *y,
                                 gint                    *width,
                                 gint                    *height,
                                 AtkCoordType             coord_type);
  void     ( *get_cell_area)    (GtkCellAccessibleParent *parent,
                                 GtkCellAccessible       *cell,
                                 GdkRectangle            *cell_rect);
  gboolean ( *grab_focus)       (GtkCellAccessibleParent *parent,
                                 GtkCellAccessible       *cell);
  int      ( *get_child_index)  (GtkCellAccessibleParent *parent,
                                 GtkCellAccessible       *cell);
  GtkCellRendererState
           ( *get_renderer_state) (GtkCellAccessibleParent *parent,
                                 GtkCellAccessible       *cell);
  /* actions */
  void     ( *expand_collapse)  (GtkCellAccessibleParent *parent,
                                 GtkCellAccessible       *cell);
  void     ( *activate)         (GtkCellAccessibleParent *parent,
                                 GtkCellAccessible       *cell);
  void     ( *edit)             (GtkCellAccessibleParent *parent,
                                 GtkCellAccessible       *cell);
  /* end of actions */
  void     ( *update_relationset) (GtkCellAccessibleParent *parent,
                                 GtkCellAccessible       *cell,
                                 AtkRelationSet          *relationset);
  void     ( *get_cell_position) (GtkCellAccessibleParent *parent,
                                  GtkCellAccessible       *cell,
                                  gint                    *row,
                                  gint                    *column);
  GPtrArray *   ( *get_column_header_cells) (GtkCellAccessibleParent *parent,
                                             GtkCellAccessible       *cell);
  GPtrArray *   ( *get_row_header_cells)    (GtkCellAccessibleParent *parent,
                                             GtkCellAccessible       *cell);
};

GDK_AVAILABLE_IN_ALL
GType    gtk_cell_accessible_parent_get_type         (void);

GDK_AVAILABLE_IN_ALL
void     gtk_cell_accessible_parent_get_cell_extents (GtkCellAccessibleParent *parent,
                                                      GtkCellAccessible       *cell,
                                                      gint                    *x,
                                                      gint                    *y,
                                                      gint                    *width,
                                                      gint                    *height,
                                                      AtkCoordType             coord_type);
GDK_AVAILABLE_IN_ALL
void     gtk_cell_accessible_parent_get_cell_area    (GtkCellAccessibleParent *parent,
                                                      GtkCellAccessible       *cell,
                                                      GdkRectangle            *cell_rect);
GDK_AVAILABLE_IN_ALL
gboolean gtk_cell_accessible_parent_grab_focus       (GtkCellAccessibleParent *parent,
                                                      GtkCellAccessible       *cell);
GDK_AVAILABLE_IN_ALL
int      gtk_cell_accessible_parent_get_child_index  (GtkCellAccessibleParent *parent,
                                                      GtkCellAccessible       *cell);
GDK_AVAILABLE_IN_ALL
GtkCellRendererState
         gtk_cell_accessible_parent_get_renderer_state(GtkCellAccessibleParent *parent,
                                                       GtkCellAccessible       *cell);
GDK_AVAILABLE_IN_ALL
void     gtk_cell_accessible_parent_expand_collapse  (GtkCellAccessibleParent *parent,
                                                      GtkCellAccessible       *cell);
GDK_AVAILABLE_IN_ALL
void     gtk_cell_accessible_parent_activate         (GtkCellAccessibleParent *parent,
                                                      GtkCellAccessible       *cell);
GDK_AVAILABLE_IN_ALL
void     gtk_cell_accessible_parent_edit             (GtkCellAccessibleParent *parent,
                                                      GtkCellAccessible       *cell);
GDK_AVAILABLE_IN_3_12
void     gtk_cell_accessible_parent_update_relationset (GtkCellAccessibleParent *parent,
                                                      GtkCellAccessible       *cell,
                                                      AtkRelationSet          *relationset);
GDK_AVAILABLE_IN_ALL
void     gtk_cell_accessible_parent_get_cell_position(GtkCellAccessibleParent *parent,
                                                      GtkCellAccessible       *cell,
                                                      gint                    *row,
                                                      gint                    *column);
GDK_AVAILABLE_IN_ALL
GPtrArray   *gtk_cell_accessible_parent_get_column_header_cells (GtkCellAccessibleParent *parent,
                                                                 GtkCellAccessible       *cell);
GDK_AVAILABLE_IN_ALL
GPtrArray   *gtk_cell_accessible_parent_get_row_header_cells    (GtkCellAccessibleParent *parent,
                                                                 GtkCellAccessible       *cell);

G_END_DECLS

#endif /* __GTK_CELL_ACCESSIBLE_PARENT_H__ */
