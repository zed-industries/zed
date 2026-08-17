/*
 * Copyright © 2018 Red Hat, Inc.
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
 * Authors: Matthias Clasen
 */

#ifndef __GTK_DROP_DOWN_H__
#define __GTK_DROP_DOWN_H__

#include <gtk/gtkwidget.h>
#include <gtk/gtkexpression.h>

G_BEGIN_DECLS

#define GTK_TYPE_DROP_DOWN         (gtk_drop_down_get_type ())

GDK_AVAILABLE_IN_ALL
G_DECLARE_FINAL_TYPE (GtkDropDown, gtk_drop_down, GTK, DROP_DOWN, GtkWidget)

GDK_AVAILABLE_IN_ALL
GtkWidget *     gtk_drop_down_new                               (GListModel             *model,
                                                                 GtkExpression          *expression);

GDK_AVAILABLE_IN_ALL
GtkWidget *     gtk_drop_down_new_from_strings                  (const char * const *    strings);

GDK_AVAILABLE_IN_ALL
void            gtk_drop_down_set_model                         (GtkDropDown            *self,
                                                                 GListModel             *model);
GDK_AVAILABLE_IN_ALL
GListModel *    gtk_drop_down_get_model                         (GtkDropDown            *self);

GDK_AVAILABLE_IN_ALL
void            gtk_drop_down_set_selected                      (GtkDropDown            *self,
                                                                 guint                   position);
GDK_AVAILABLE_IN_ALL
guint           gtk_drop_down_get_selected                      (GtkDropDown            *self);

GDK_AVAILABLE_IN_ALL
gpointer        gtk_drop_down_get_selected_item                 (GtkDropDown            *self);

GDK_AVAILABLE_IN_ALL
void            gtk_drop_down_set_factory                       (GtkDropDown            *self,
                                                                 GtkListItemFactory     *factory);
GDK_AVAILABLE_IN_ALL
GtkListItemFactory *
                gtk_drop_down_get_factory                       (GtkDropDown            *self);

GDK_AVAILABLE_IN_ALL
void            gtk_drop_down_set_list_factory                  (GtkDropDown            *self,
                                                                 GtkListItemFactory     *factory);
GDK_AVAILABLE_IN_ALL
GtkListItemFactory *
                gtk_drop_down_get_list_factory                  (GtkDropDown            *self);

GDK_AVAILABLE_IN_ALL
void            gtk_drop_down_set_expression                    (GtkDropDown            *self,
                                                                 GtkExpression          *expression);
GDK_AVAILABLE_IN_ALL
GtkExpression * gtk_drop_down_get_expression                    (GtkDropDown            *self);

GDK_AVAILABLE_IN_ALL
void            gtk_drop_down_set_enable_search                 (GtkDropDown            *self,
                                                                 gboolean                enable_search);
GDK_AVAILABLE_IN_ALL
gboolean        gtk_drop_down_get_enable_search                 (GtkDropDown            *self);
GDK_AVAILABLE_IN_4_6
void            gtk_drop_down_set_show_arrow                    (GtkDropDown            *self,
                                                                 gboolean                show_arrow);
GDK_AVAILABLE_IN_4_6
gboolean        gtk_drop_down_get_show_arrow                    (GtkDropDown            *self);

G_END_DECLS

#endif  /* __GTK_DROP_DOWN_H__ */
