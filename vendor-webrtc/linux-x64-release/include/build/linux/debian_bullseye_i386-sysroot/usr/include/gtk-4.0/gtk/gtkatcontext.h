/* gtkatcontext.h: Assistive technology context
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

#include <gtk/gtktypes.h>
#include <gtk/gtkenums.h>
#include <gtk/gtkaccessible.h>

G_BEGIN_DECLS

#define GTK_TYPE_AT_CONTEXT (gtk_at_context_get_type())

GDK_AVAILABLE_IN_ALL
GDK_DECLARE_INTERNAL_TYPE (GtkATContext, gtk_at_context, GTK, AT_CONTEXT, GObject)

GDK_AVAILABLE_IN_ALL
GtkAccessible *         gtk_at_context_get_accessible           (GtkATContext      *self);
GDK_AVAILABLE_IN_ALL
GtkAccessibleRole       gtk_at_context_get_accessible_role      (GtkATContext      *self);

GDK_AVAILABLE_IN_ALL
GtkATContext *          gtk_at_context_create                   (GtkAccessibleRole  accessible_role,
                                                                 GtkAccessible     *accessible,
                                                                 GdkDisplay        *display);

G_END_DECLS
