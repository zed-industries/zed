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

#ifndef __GTK_FILTER_LIST_MODEL_H__
#define __GTK_FILTER_LIST_MODEL_H__


#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gio/gio.h>
#include <gtk/gtkfilter.h>


G_BEGIN_DECLS

#define GTK_TYPE_FILTER_LIST_MODEL (gtk_filter_list_model_get_type ())

GDK_AVAILABLE_IN_ALL
G_DECLARE_FINAL_TYPE (GtkFilterListModel, gtk_filter_list_model, GTK, FILTER_LIST_MODEL, GObject)

GDK_AVAILABLE_IN_ALL
GtkFilterListModel *    gtk_filter_list_model_new               (GListModel             *model,
                                                                 GtkFilter              *filter);

GDK_AVAILABLE_IN_ALL
void                    gtk_filter_list_model_set_filter        (GtkFilterListModel     *self,
                                                                 GtkFilter              *filter);
GDK_AVAILABLE_IN_ALL
GtkFilter *             gtk_filter_list_model_get_filter        (GtkFilterListModel     *self);
GDK_AVAILABLE_IN_ALL
void                    gtk_filter_list_model_set_model         (GtkFilterListModel     *self,
                                                                 GListModel             *model);
GDK_AVAILABLE_IN_ALL
GListModel *            gtk_filter_list_model_get_model         (GtkFilterListModel     *self);
GDK_AVAILABLE_IN_ALL
void                    gtk_filter_list_model_set_incremental   (GtkFilterListModel     *self,
                                                                 gboolean                incremental);
GDK_AVAILABLE_IN_ALL
gboolean                gtk_filter_list_model_get_incremental   (GtkFilterListModel     *self);
GDK_AVAILABLE_IN_ALL
guint                   gtk_filter_list_model_get_pending       (GtkFilterListModel     *self);


G_END_DECLS

#endif /* __GTK_FILTER_LIST_MODEL_H__ */
