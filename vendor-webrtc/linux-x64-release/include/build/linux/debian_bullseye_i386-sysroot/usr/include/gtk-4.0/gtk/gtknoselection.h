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

#ifndef __GTK_NO_SELECTION_H__
#define __GTK_NO_SELECTION_H__

#include <gtk/gtktypes.h>

G_BEGIN_DECLS

#define GTK_TYPE_NO_SELECTION (gtk_no_selection_get_type ())

GDK_AVAILABLE_IN_ALL
G_DECLARE_FINAL_TYPE (GtkNoSelection, gtk_no_selection, GTK, NO_SELECTION, GObject)

GDK_AVAILABLE_IN_ALL
GtkNoSelection *        gtk_no_selection_new                    (GListModel             *model);

GDK_AVAILABLE_IN_ALL
GListModel *            gtk_no_selection_get_model              (GtkNoSelection         *self);
GDK_AVAILABLE_IN_ALL
void                    gtk_no_selection_set_model              (GtkNoSelection         *self,
                                                                 GListModel             *model);

G_END_DECLS

#endif /* __GTK_NO_SELECTION_H__ */
