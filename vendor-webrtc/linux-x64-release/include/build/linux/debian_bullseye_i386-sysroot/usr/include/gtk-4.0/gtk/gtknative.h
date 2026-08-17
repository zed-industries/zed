/*
 * Copyright © 2019 Red Hat, Inc.
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
 * Authors: Matthias Clasen <mclasen@redhat.com>
 */

#ifndef __GTK_NATIVE_H__
#define __GTK_NATIVE_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gdk/gdk.h>
#include <gtk/gtkwidget.h>

G_BEGIN_DECLS

#define GTK_TYPE_NATIVE               (gtk_native_get_type ())

GDK_AVAILABLE_IN_ALL
G_DECLARE_INTERFACE (GtkNative, gtk_native, GTK, NATIVE, GtkWidget)

GDK_AVAILABLE_IN_ALL
void        gtk_native_realize         (GtkNative *self);

GDK_AVAILABLE_IN_ALL
void        gtk_native_unrealize       (GtkNative *self);

GDK_AVAILABLE_IN_ALL
GtkNative * gtk_native_get_for_surface (GdkSurface *surface);

GDK_AVAILABLE_IN_ALL
GdkSurface *gtk_native_get_surface     (GtkNative *self);

GDK_AVAILABLE_IN_ALL
GskRenderer *gtk_native_get_renderer   (GtkNative *self);

GDK_AVAILABLE_IN_ALL
void         gtk_native_get_surface_transform (GtkNative *self,
                                               double    *x,
                                               double    *y);

G_END_DECLS

#endif /* __GTK_NATIVE_H__ */
