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
 * License along with this library; if not, write to the
 * Free Software Foundation, Inc., 59 Temple Place - Suite 330,
 * Boston, MA 02111-1307, USA.
 */

#ifndef __GTK_CELL_VIEW_H__
#define __GTK_CELL_VIEW_H__

#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkwidget.h>
#include <gtk/gtkcellrenderer.h>
#include <gtk/gtktreemodel.h>

G_BEGIN_DECLS

#define GTK_TYPE_CELL_VIEW                (gtk_cell_view_get_type ())
#define GTK_CELL_VIEW(obj)                (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_CELL_VIEW, GtkCellView))
#define GTK_CELL_VIEW_CLASS(vtable)       (G_TYPE_CHECK_CLASS_CAST ((vtable), GTK_TYPE_CELL_VIEW, GtkCellViewClass))
#define GTK_IS_CELL_VIEW(obj)             (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_CELL_VIEW))
#define GTK_IS_CELL_VIEW_CLASS(vtable)    (G_TYPE_CHECK_CLASS_TYPE ((vtable), GTK_TYPE_CELL_VIEW))
#define GTK_CELL_VIEW_GET_CLASS(inst)     (G_TYPE_INSTANCE_GET_CLASS ((inst), GTK_TYPE_CELL_VIEW, GtkCellViewClass))

typedef struct _GtkCellView             GtkCellView;
typedef struct _GtkCellViewClass        GtkCellViewClass;
typedef struct _GtkCellViewPrivate      GtkCellViewPrivate;

struct _GtkCellView
{
  GtkWidget parent_instance;

  /*< private >*/
  GtkCellViewPrivate *GSEAL (priv);
};

struct _GtkCellViewClass
{
  GtkWidgetClass parent_class;
};

GType             gtk_cell_view_get_type               (void) G_GNUC_CONST;
GtkWidget        *gtk_cell_view_new                    (void);
GtkWidget        *gtk_cell_view_new_with_text          (const gchar     *text);
GtkWidget        *gtk_cell_view_new_with_markup        (const gchar     *markup);
GtkWidget        *gtk_cell_view_new_with_pixbuf        (GdkPixbuf       *pixbuf);

void              gtk_cell_view_set_model               (GtkCellView     *cell_view,
                                                         GtkTreeModel    *model);
GtkTreeModel     *gtk_cell_view_get_model               (GtkCellView     *cell_view);
void              gtk_cell_view_set_displayed_row       (GtkCellView     *cell_view,
                                                         GtkTreePath     *path);
GtkTreePath      *gtk_cell_view_get_displayed_row       (GtkCellView     *cell_view);
gboolean          gtk_cell_view_get_size_of_row         (GtkCellView     *cell_view,
                                                         GtkTreePath     *path,
                                                         GtkRequisition  *requisition);

void              gtk_cell_view_set_background_color    (GtkCellView     *cell_view,
                                                         const GdkColor  *color);
#ifndef GTK_DISABLE_DEPRECATED
GList            *gtk_cell_view_get_cell_renderers      (GtkCellView     *cell_view);
#endif

G_END_DECLS

#endif /* __GTK_CELL_VIEW_H__ */
