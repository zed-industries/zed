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

#ifndef __GTK_SINGLE_SELECTION_H__
#define __GTK_SINGLE_SELECTION_H__

#include <gtk/gtktypes.h>

G_BEGIN_DECLS

#define GTK_TYPE_SINGLE_SELECTION (gtk_single_selection_get_type ())

GDK_AVAILABLE_IN_ALL
G_DECLARE_FINAL_TYPE (GtkSingleSelection, gtk_single_selection, GTK, SINGLE_SELECTION, GObject)

GDK_AVAILABLE_IN_ALL
GtkSingleSelection *    gtk_single_selection_new                (GListModel             *model);

GDK_AVAILABLE_IN_ALL
GListModel *            gtk_single_selection_get_model          (GtkSingleSelection     *self);
GDK_AVAILABLE_IN_ALL
void                    gtk_single_selection_set_model          (GtkSingleSelection     *self,
                                                                 GListModel             *model);
GDK_AVAILABLE_IN_ALL
guint                   gtk_single_selection_get_selected       (GtkSingleSelection     *self);
GDK_AVAILABLE_IN_ALL
void                    gtk_single_selection_set_selected       (GtkSingleSelection     *self,
                                                                 guint                   position);
GDK_AVAILABLE_IN_ALL
gpointer                gtk_single_selection_get_selected_item  (GtkSingleSelection     *self);
GDK_AVAILABLE_IN_ALL
gboolean                gtk_single_selection_get_autoselect     (GtkSingleSelection     *self);
GDK_AVAILABLE_IN_ALL
void                    gtk_single_selection_set_autoselect     (GtkSingleSelection     *self,
                                                                 gboolean                autoselect);
GDK_AVAILABLE_IN_ALL
gboolean                gtk_single_selection_get_can_unselect   (GtkSingleSelection     *self);
GDK_AVAILABLE_IN_ALL
void                    gtk_single_selection_set_can_unselect   (GtkSingleSelection     *self,
                                                                 gboolean                can_unselect);

G_END_DECLS

#endif /* __GTK_SINGLE_SELECTION_H__ */
