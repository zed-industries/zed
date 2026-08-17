/*
 * Copyright © 2018 Benjamin Otte
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

#ifndef __GTK_SORT_LIST_MODEL_H__
#define __GTK_SORT_LIST_MODEL_H__


#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gio/gio.h>
#include <gtk/gtkwidget.h>
#include <gtk/gtksorter.h>


G_BEGIN_DECLS

#define GTK_TYPE_SORT_LIST_MODEL (gtk_sort_list_model_get_type ())

GDK_AVAILABLE_IN_ALL
G_DECLARE_FINAL_TYPE (GtkSortListModel, gtk_sort_list_model, GTK, SORT_LIST_MODEL, GObject)

GDK_AVAILABLE_IN_ALL
GtkSortListModel *      gtk_sort_list_model_new                 (GListModel            *model,
                                                                 GtkSorter             *sorter);
GDK_AVAILABLE_IN_ALL
void                    gtk_sort_list_model_set_sorter          (GtkSortListModel       *self,
                                                                 GtkSorter              *sorter);
GDK_AVAILABLE_IN_ALL
GtkSorter *             gtk_sort_list_model_get_sorter          (GtkSortListModel       *self);

GDK_AVAILABLE_IN_ALL
void                    gtk_sort_list_model_set_model           (GtkSortListModel       *self,
                                                                 GListModel             *model);
GDK_AVAILABLE_IN_ALL
GListModel *            gtk_sort_list_model_get_model           (GtkSortListModel       *self);

GDK_AVAILABLE_IN_ALL
void                    gtk_sort_list_model_set_incremental     (GtkSortListModel       *self,
                                                                 gboolean                incremental);
GDK_AVAILABLE_IN_ALL
gboolean                gtk_sort_list_model_get_incremental     (GtkSortListModel       *self);

GDK_AVAILABLE_IN_ALL
guint                   gtk_sort_list_model_get_pending         (GtkSortListModel       *self);

G_END_DECLS

#endif /* __GTK_SORT_LIST_MODEL_H__ */
