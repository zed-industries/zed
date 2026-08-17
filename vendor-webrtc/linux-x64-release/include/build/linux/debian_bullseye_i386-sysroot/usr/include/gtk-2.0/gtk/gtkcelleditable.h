/* gtkcelleditable.h
 * Copyright (C) 2001  Red Hat, Inc.
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

#ifndef __GTK_CELL_EDITABLE_H__
#define __GTK_CELL_EDITABLE_H__

#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkwidget.h>

G_BEGIN_DECLS

#define GTK_TYPE_CELL_EDITABLE            (gtk_cell_editable_get_type ())
#define GTK_CELL_EDITABLE(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_CELL_EDITABLE, GtkCellEditable))
#define GTK_CELL_EDITABLE_CLASS(obj)      (G_TYPE_CHECK_CLASS_CAST ((obj), GTK_TYPE_CELL_EDITABLE, GtkCellEditableIface))
#define GTK_IS_CELL_EDITABLE(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_CELL_EDITABLE))
#define GTK_CELL_EDITABLE_GET_IFACE(obj)  (G_TYPE_INSTANCE_GET_INTERFACE ((obj), GTK_TYPE_CELL_EDITABLE, GtkCellEditableIface))

typedef struct _GtkCellEditable      GtkCellEditable; /* Dummy typedef */
typedef struct _GtkCellEditableIface GtkCellEditableIface;

struct _GtkCellEditableIface
{
  GTypeInterface g_iface;

  /* signals */
  void (* editing_done)  (GtkCellEditable *cell_editable);
  void (* remove_widget) (GtkCellEditable *cell_editable);

  /* virtual table */
  void (* start_editing) (GtkCellEditable *cell_editable,
			  GdkEvent        *event);
};


GType gtk_cell_editable_get_type      (void) G_GNUC_CONST;

void  gtk_cell_editable_start_editing (GtkCellEditable *cell_editable,
				       GdkEvent        *event);
void  gtk_cell_editable_editing_done  (GtkCellEditable *cell_editable);
void  gtk_cell_editable_remove_widget (GtkCellEditable *cell_editable);


G_END_DECLS

#endif /* __GTK_CELL_EDITABLE_H__ */
