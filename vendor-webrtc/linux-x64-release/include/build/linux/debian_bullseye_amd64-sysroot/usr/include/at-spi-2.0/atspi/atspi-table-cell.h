/*
 * AT-SPI - Assistive Technology Service Provider Interface
 * (Gnome Accessibility Project; http://developer.gnome.org/projects/gap)
 *
 * Copyright 2002 Ximian, Inc.
 *           2002 Sun Microsystems Inc.
 * Copyright 2010, 2011 Novell, Inc.
 *           
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library; if not, write to the
 * Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 */

#ifndef _ATSPI_TABLE_CELL_H_
#define _ATSPI_TABLE_CELL_H_

#include "glib-object.h"

#include "atspi-constants.h"

#include "atspi-types.h"

G_BEGIN_DECLS

#define ATSPI_TYPE_TABLE_CELL                    (atspi_table_cell_get_type ())
#define ATSPI_IS_TABLE_CELL(obj)                 G_TYPE_CHECK_INSTANCE_TYPE ((obj), ATSPI_TYPE_TABLE_CELL)
#define ATSPI_TABLE_CELL(obj)                    G_TYPE_CHECK_INSTANCE_CAST ((obj), ATSPI_TYPE_TABLE_CELL, AtspiTableCell)
#define ATSPI_TABLE_CELL_GET_IFACE(obj)          (G_TYPE_INSTANCE_GET_INTERFACE ((obj), ATSPI_TYPE_TABLE_CELL, AtspiTableCell))

GType atspi_table_cell_get_type ();

struct _AtspiTableCell
{
  GTypeInterface parent;
};

gint atspi_table_cell_get_column_span (AtspiTableCell *obj, GError **error);

GPtrArray *atspi_table_cell_get_column_header_cells (AtspiTableCell *obj,
                                                     GError **error);

gint atspi_table_cell_get_column_index (AtspiTableCell *obj, GError **error);

gint atspi_table_cell_get_row_span (AtspiTableCell *obj, GError **error);

GPtrArray *atspi_table_cell_get_row_header_cells (AtspiTableCell *obj,
                                                  GError **error);

gint atspi_table_cell_get_position (AtspiTableCell *obj,
                                    gint           *row,
                                    gint           *column,
                                    GError        **error);

void atspi_table_cell_get_row_column_span (AtspiTableCell *obj,
                                              gint *row,
                                              gint *column,
                                              gint *row_span,
                                              gint *column_span,
                                              GError **error);

AtspiAccessible *atspi_table_cell_get_table (AtspiTableCell *obj,
                                             GError **error);
G_END_DECLS

#endif	/* _ATSPI_TABLE_CELL_H_ */
