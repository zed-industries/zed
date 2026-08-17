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

#ifndef _ATSPI_TABLE_H_
#define _ATSPI_TABLE_H_

#include "glib-object.h"

#include "atspi-constants.h"

#include "atspi-types.h"

G_BEGIN_DECLS

#define ATSPI_TYPE_TABLE                    (atspi_table_get_type ())
#define ATSPI_IS_TABLE(obj)                 G_TYPE_CHECK_INSTANCE_TYPE ((obj), ATSPI_TYPE_TABLE)
#define ATSPI_TABLE(obj)                    G_TYPE_CHECK_INSTANCE_CAST ((obj), ATSPI_TYPE_TABLE, AtspiTable)
#define ATSPI_TABLE_GET_IFACE(obj)          (G_TYPE_INSTANCE_GET_INTERFACE ((obj), ATSPI_TYPE_TABLE, AtspiTable))

GType atspi_table_get_type ();

struct _AtspiTable
{
  GTypeInterface parent;
};

AtspiAccessible * atspi_table_get_caption (AtspiTable *obj, GError **error);

AtspiAccessible * atspi_table_get_summary (AtspiTable *obj, GError **error);

gint atspi_table_get_n_rows (AtspiTable *obj, GError **error);

gint atspi_table_get_n_columns (AtspiTable *obj, GError **error);

AtspiAccessible * atspi_table_get_accessible_at (AtspiTable *obj, gint row, gint column, GError **error);

gint atspi_table_get_index_at (AtspiTable *obj, gint row, gint column, GError **error);

gint atspi_table_get_row_at_index (AtspiTable *obj, gint index, GError **error);

gint atspi_table_get_column_at_index (AtspiTable *obj, gint index, GError **error);

gchar * atspi_table_get_row_description (AtspiTable *obj, gint row, GError **error);

gchar * atspi_table_get_column_description (AtspiTable *obj, gint         column, GError **error);

gint
atspi_table_get_row_extent_at (AtspiTable *obj, gint row, gint column, GError **error);

gint
atspi_table_get_column_extent_at (AtspiTable *obj, gint row, gint column, GError **error);

AtspiAccessible * atspi_table_get_row_header (AtspiTable *obj, gint row, GError **error);

AtspiAccessible * atspi_table_get_column_header (AtspiTable *obj, gint column, GError **error);

gint atspi_table_get_n_selected_rows (AtspiTable *obj, GError **error);

GArray *atspi_table_get_selected_rows (AtspiTable *obj, GError **error);

GArray * atspi_table_get_selected_columns (AtspiTable *obj, GError **error);

gint atspi_table_get_n_selected_columns (AtspiTable *obj, GError **error);

gboolean atspi_table_is_row_selected (AtspiTable *obj, gint row, GError **error);

gboolean atspi_table_is_column_selected (AtspiTable *obj, gint column, GError **error);

gboolean atspi_table_add_row_selection (AtspiTable *obj, gint row, GError **error);

gboolean atspi_table_add_column_selection (AtspiTable *obj, gint column, GError **error);

gboolean atspi_table_remove_row_selection (AtspiTable *obj, gint row, GError **error);

gboolean atspi_table_remove_column_selection (AtspiTable *obj, gint column, GError **error);

gboolean atspi_table_get_row_column_extents_at_index (AtspiTable *obj, gint index, gint *row, gint *col, gint *row_extents, gint *col_extents, gboolean *is_selected, GError **error);

gboolean atspi_table_is_selected (AtspiTable *obj, gint row, gint column, GError **error);

G_END_DECLS

#endif	/* _ATSPI_TABLE_H_ */
