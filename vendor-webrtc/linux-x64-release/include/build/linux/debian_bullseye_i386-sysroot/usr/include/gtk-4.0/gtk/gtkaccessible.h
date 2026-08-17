/* gtkaccessible.h: Accessible interface
 *
 * Copyright 2020  GNOME Foundation
 *
 * SPDX-License-Identifier: LGPL-2.1-or-later
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
 * License along with this library; if not, see <http://www.gnu.org/licenses/>.
 */

#pragma once

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <glib-object.h>
#include <gtk/gtktypes.h>
#include <gtk/gtkenums.h>

G_BEGIN_DECLS

#define GTK_TYPE_ACCESSIBLE (gtk_accessible_get_type())

GDK_AVAILABLE_IN_ALL
G_DECLARE_INTERFACE (GtkAccessible, gtk_accessible, GTK, ACCESSIBLE, GObject)

GDK_AVAILABLE_IN_ALL
GtkAccessibleRole       gtk_accessible_get_accessible_role      (GtkAccessible         *self);

GDK_AVAILABLE_IN_ALL
void                    gtk_accessible_update_state             (GtkAccessible         *self,
                                                                 GtkAccessibleState     first_state,
                                                                 ...);
GDK_AVAILABLE_IN_ALL
void                    gtk_accessible_update_property          (GtkAccessible         *self,
                                                                 GtkAccessibleProperty  first_property,
                                                                 ...);
GDK_AVAILABLE_IN_ALL
void                    gtk_accessible_update_relation          (GtkAccessible         *self,
                                                                 GtkAccessibleRelation  first_relation,
                                                                 ...);
GDK_AVAILABLE_IN_ALL
void                    gtk_accessible_update_state_value       (GtkAccessible         *self,
                                                                 int                    n_states,
                                                                 GtkAccessibleState     states[],
                                                                 const GValue           values[]);
GDK_AVAILABLE_IN_ALL
void                    gtk_accessible_update_property_value    (GtkAccessible         *self,
                                                                 int                    n_properties,
                                                                 GtkAccessibleProperty  properties[],
                                                                 const GValue           values[]);
GDK_AVAILABLE_IN_ALL
void                    gtk_accessible_update_relation_value    (GtkAccessible         *self,
                                                                 int                    n_relations,
                                                                 GtkAccessibleRelation  relations[],
                                                                 const GValue           values[]);

GDK_AVAILABLE_IN_ALL
void                    gtk_accessible_reset_state              (GtkAccessible         *self,
                                                                 GtkAccessibleState     state);
GDK_AVAILABLE_IN_ALL
void                    gtk_accessible_reset_property           (GtkAccessible         *self,
                                                                 GtkAccessibleProperty  property);
GDK_AVAILABLE_IN_ALL
void                    gtk_accessible_reset_relation           (GtkAccessible         *self,
                                                                 GtkAccessibleRelation  relation);

GDK_AVAILABLE_IN_ALL
void                    gtk_accessible_state_init_value         (GtkAccessibleState     state,
                                                                 GValue                *value);
GDK_AVAILABLE_IN_ALL
void                    gtk_accessible_property_init_value      (GtkAccessibleProperty  property,
                                                                 GValue                *value);
GDK_AVAILABLE_IN_ALL
void                    gtk_accessible_relation_init_value      (GtkAccessibleRelation  relation,
                                                                 GValue                *value);

G_END_DECLS
