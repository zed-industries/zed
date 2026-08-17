/* gtkfixedlayout.h: Fixed positioning layout manager
 *
 * SPDX-License-Identifier: LGPL-2.1-or-later
 *
 * Copyright 2019 GNOME Foundation
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

#pragma once

#include <gtk/gtklayoutmanager.h>

G_BEGIN_DECLS

#define GTK_TYPE_FIXED_LAYOUT (gtk_fixed_layout_get_type ())
#define GTK_TYPE_FIXED_LAYOUT_CHILD (gtk_fixed_layout_child_get_type ())

/* GtkFixedLayout */

GDK_AVAILABLE_IN_ALL
G_DECLARE_FINAL_TYPE (GtkFixedLayout, gtk_fixed_layout, GTK, FIXED_LAYOUT, GtkLayoutManager)

GDK_AVAILABLE_IN_ALL
GtkLayoutManager *      gtk_fixed_layout_new    (void);

/* GtkFixedLayoutChild */

GDK_AVAILABLE_IN_ALL
G_DECLARE_FINAL_TYPE (GtkFixedLayoutChild, gtk_fixed_layout_child, GTK, FIXED_LAYOUT_CHILD, GtkLayoutChild)

GDK_AVAILABLE_IN_ALL
void            gtk_fixed_layout_child_set_transform    (GtkFixedLayoutChild *child,
                                                         GskTransform        *transform);
GDK_AVAILABLE_IN_ALL
GskTransform *  gtk_fixed_layout_child_get_transform    (GtkFixedLayoutChild *child);

G_END_DECLS
