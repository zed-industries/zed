/* GtkCellRendererSpin
 * Copyright (C) 2004 Lorenzo Gil Sanchez
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library. If not, see <http://www.gnu.org/licenses/>.
 */

#ifndef __GTK_CELL_RENDERER_SPIN_H__
#define __GTK_CELL_RENDERER_SPIN_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkcellrenderertext.h>

G_BEGIN_DECLS

#define GTK_TYPE_CELL_RENDERER_SPIN		(gtk_cell_renderer_spin_get_type ())
#define GTK_CELL_RENDERER_SPIN(obj)		(G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_CELL_RENDERER_SPIN, GtkCellRendererSpin))
#define GTK_IS_CELL_RENDERER_SPIN(obj)		(G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_CELL_RENDERER_SPIN))

typedef struct _GtkCellRendererSpin        GtkCellRendererSpin;

GDK_AVAILABLE_IN_ALL
GType            gtk_cell_renderer_spin_get_type (void);
GDK_AVAILABLE_IN_ALL
GtkCellRenderer *gtk_cell_renderer_spin_new      (void);

G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkCellRendererSpin, g_object_unref)

G_END_DECLS

#endif  /* __GTK_CELL_RENDERER_SPIN_H__ */
