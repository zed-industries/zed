/* gtkcellareabox.h
 *
 * Copyright (C) 2010 Openismus GmbH
 *
 * Authors:
 *      Tristan Van Berkom <tristanvb@openismus.com>
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

#ifndef __GTK_CELL_AREA_BOX_H__
#define __GTK_CELL_AREA_BOX_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkcellarea.h>

G_BEGIN_DECLS

#define GTK_TYPE_CELL_AREA_BOX            (gtk_cell_area_box_get_type ())
#define GTK_CELL_AREA_BOX(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_CELL_AREA_BOX, GtkCellAreaBox))
#define GTK_IS_CELL_AREA_BOX(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_CELL_AREA_BOX))

typedef struct _GtkCellAreaBox              GtkCellAreaBox;

GDK_AVAILABLE_IN_ALL
GType        gtk_cell_area_box_get_type    (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
GtkCellArea *gtk_cell_area_box_new         (void);
GDK_AVAILABLE_IN_ALL
void         gtk_cell_area_box_pack_start  (GtkCellAreaBox  *box,
                                            GtkCellRenderer *renderer,
                                            gboolean         expand,
                                            gboolean         align,
                                            gboolean         fixed);
GDK_AVAILABLE_IN_ALL
void         gtk_cell_area_box_pack_end    (GtkCellAreaBox  *box,
                                            GtkCellRenderer *renderer,
                                            gboolean         expand,
                                            gboolean         align,
                                            gboolean         fixed);
GDK_AVAILABLE_IN_ALL
int          gtk_cell_area_box_get_spacing (GtkCellAreaBox  *box);
GDK_AVAILABLE_IN_ALL
void         gtk_cell_area_box_set_spacing (GtkCellAreaBox  *box,
                                            int              spacing);

/* Private interaction with GtkCellAreaBoxContext */
gboolean    _gtk_cell_area_box_group_visible (GtkCellAreaBox  *box,
                                              int              group_idx);

G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkCellAreaBox, g_object_unref)

G_END_DECLS

#endif /* __GTK_CELL_AREA_BOX_H__ */
