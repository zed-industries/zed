/* gtkcellview.h
 * Copyright (C) 2002, 2003  Kristian Rietveld <kris@gtk.org>
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

#ifndef __GTK_CELL_VIEW_H__
#define __GTK_CELL_VIEW_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkwidget.h>
#include <gtk/gtkcellrenderer.h>
#include <gtk/gtkcellarea.h>
#include <gtk/gtkcellareacontext.h>
#include <gtk/gtktreemodel.h>

G_BEGIN_DECLS

#define GTK_TYPE_CELL_VIEW                (gtk_cell_view_get_type ())
#define GTK_CELL_VIEW(obj)                (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_CELL_VIEW, GtkCellView))
#define GTK_IS_CELL_VIEW(obj)             (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_CELL_VIEW))

typedef struct _GtkCellView             GtkCellView;

GDK_AVAILABLE_IN_ALL
GType             gtk_cell_view_get_type                (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
GtkWidget        *gtk_cell_view_new                     (void);
GDK_AVAILABLE_IN_ALL
GtkWidget        *gtk_cell_view_new_with_context        (GtkCellArea        *area,
                                                         GtkCellAreaContext *context);
GDK_AVAILABLE_IN_ALL
GtkWidget        *gtk_cell_view_new_with_text           (const char      *text);
GDK_AVAILABLE_IN_ALL
GtkWidget        *gtk_cell_view_new_with_markup         (const char      *markup);
GDK_AVAILABLE_IN_ALL
GtkWidget        *gtk_cell_view_new_with_texture        (GdkTexture      *texture);
GDK_AVAILABLE_IN_ALL
void              gtk_cell_view_set_model               (GtkCellView     *cell_view,
                                                         GtkTreeModel    *model);
GDK_AVAILABLE_IN_ALL
GtkTreeModel     *gtk_cell_view_get_model               (GtkCellView     *cell_view);
GDK_AVAILABLE_IN_ALL
void              gtk_cell_view_set_displayed_row       (GtkCellView     *cell_view,
                                                         GtkTreePath     *path);
GDK_AVAILABLE_IN_ALL
GtkTreePath      *gtk_cell_view_get_displayed_row       (GtkCellView     *cell_view);
GDK_AVAILABLE_IN_ALL
gboolean          gtk_cell_view_get_draw_sensitive      (GtkCellView     *cell_view);
GDK_AVAILABLE_IN_ALL
void              gtk_cell_view_set_draw_sensitive      (GtkCellView     *cell_view,
                                                         gboolean         draw_sensitive);
GDK_AVAILABLE_IN_ALL
gboolean          gtk_cell_view_get_fit_model           (GtkCellView     *cell_view);
GDK_AVAILABLE_IN_ALL
void              gtk_cell_view_set_fit_model           (GtkCellView     *cell_view,
                                                         gboolean         fit_model);

G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkCellView, g_object_unref)

G_END_DECLS

#endif /* __GTK_CELL_VIEW_H__ */
