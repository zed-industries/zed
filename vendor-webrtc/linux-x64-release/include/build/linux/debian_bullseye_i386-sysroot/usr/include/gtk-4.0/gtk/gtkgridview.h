/*
 * Copyright © 2019 Benjamin Otte
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
 * License along with this library. If not, see <http://www.gnu.org/licenses/>.
 *
 * Authors: Benjamin Otte <otte@gnome.org>
 */

#ifndef __GTK_GRID_VIEW_H__
#define __GTK_GRID_VIEW_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtklistbase.h>

G_BEGIN_DECLS

#define GTK_TYPE_GRID_VIEW         (gtk_grid_view_get_type ())
#define GTK_GRID_VIEW(o)           (G_TYPE_CHECK_INSTANCE_CAST ((o), GTK_TYPE_GRID_VIEW, GtkGridView))
#define GTK_GRID_VIEW_CLASS(k)     (G_TYPE_CHECK_CLASS_CAST ((k), GTK_TYPE_GRID_VIEW, GtkGridViewClass))
#define GTK_IS_GRID_VIEW(o)        (G_TYPE_CHECK_INSTANCE_TYPE ((o), GTK_TYPE_GRID_VIEW))
#define GTK_IS_GRID_VIEW_CLASS(k)  (G_TYPE_CHECK_CLASS_TYPE ((k), GTK_TYPE_GRID_VIEW))
#define GTK_GRID_VIEW_GET_CLASS(o) (G_TYPE_INSTANCE_GET_CLASS ((o), GTK_TYPE_GRID_VIEW, GtkGridViewClass))

typedef struct _GtkGridView GtkGridView;
typedef struct _GtkGridViewClass GtkGridViewClass;

GDK_AVAILABLE_IN_ALL
GType           gtk_grid_view_get_type                          (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
GtkWidget *     gtk_grid_view_new                               (GtkSelectionModel      *model,
                                                                 GtkListItemFactory     *factory);

GDK_AVAILABLE_IN_ALL
GtkSelectionModel *
                gtk_grid_view_get_model                         (GtkGridView            *self);
GDK_AVAILABLE_IN_ALL
void            gtk_grid_view_set_model                         (GtkGridView            *self,
                                                                 GtkSelectionModel      *model);
GDK_AVAILABLE_IN_ALL
void            gtk_grid_view_set_factory                       (GtkGridView            *self,
                                                                 GtkListItemFactory     *factory);
GDK_AVAILABLE_IN_ALL
GtkListItemFactory *
                gtk_grid_view_get_factory                       (GtkGridView            *self);
GDK_AVAILABLE_IN_ALL
guint           gtk_grid_view_get_min_columns                   (GtkGridView            *self);
GDK_AVAILABLE_IN_ALL
void            gtk_grid_view_set_min_columns                   (GtkGridView            *self,
                                                                 guint                   min_columns);
GDK_AVAILABLE_IN_ALL
guint           gtk_grid_view_get_max_columns                   (GtkGridView            *self);
GDK_AVAILABLE_IN_ALL
void            gtk_grid_view_set_max_columns                   (GtkGridView            *self,
                                                                 guint                   max_columns);
GDK_AVAILABLE_IN_ALL
void            gtk_grid_view_set_enable_rubberband             (GtkGridView            *self,
                                                                 gboolean                enable_rubberband);
GDK_AVAILABLE_IN_ALL
gboolean        gtk_grid_view_get_enable_rubberband             (GtkGridView            *self);

GDK_AVAILABLE_IN_ALL
void            gtk_grid_view_set_single_click_activate         (GtkGridView            *self,
                                                                 gboolean                single_click_activate);
GDK_AVAILABLE_IN_ALL
gboolean        gtk_grid_view_get_single_click_activate         (GtkGridView            *self);

G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkGridView, g_object_unref)

G_END_DECLS

#endif  /* __GTK_GRID_VIEW_H__ */
