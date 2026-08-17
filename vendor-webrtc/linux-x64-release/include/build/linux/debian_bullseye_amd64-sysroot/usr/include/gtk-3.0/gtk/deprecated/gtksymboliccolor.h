/* GTK - The GIMP Toolkit
 * Copyright (C) 2010 Carlos Garnacho <carlosg@gnome.org>
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

#ifndef __GTK_SYMBOLIC_COLOR_H__
#define __GTK_SYMBOLIC_COLOR_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gdk/gdk.h>
#include <gtk/deprecated/gtkstyleproperties.h>

G_BEGIN_DECLS

#define GTK_TYPE_SYMBOLIC_COLOR (gtk_symbolic_color_get_type ())

GDK_DEPRECATED_IN_3_8
GType              gtk_symbolic_color_get_type    (void) G_GNUC_CONST;

GDK_DEPRECATED_IN_3_8
GtkSymbolicColor * gtk_symbolic_color_new_literal (const GdkRGBA      *color);
GDK_DEPRECATED_IN_3_8
GtkSymbolicColor * gtk_symbolic_color_new_name    (const gchar        *name);
GDK_DEPRECATED_IN_3_8
GtkSymbolicColor * gtk_symbolic_color_new_shade   (GtkSymbolicColor   *color,
                                                   gdouble             factor);
GDK_DEPRECATED_IN_3_8
GtkSymbolicColor * gtk_symbolic_color_new_alpha   (GtkSymbolicColor   *color,
                                                   gdouble             factor);
GDK_DEPRECATED_IN_3_8
GtkSymbolicColor * gtk_symbolic_color_new_mix     (GtkSymbolicColor   *color1,
                                                   GtkSymbolicColor   *color2,
                                                   gdouble             factor);
GDK_DEPRECATED_IN_3_8
GtkSymbolicColor * gtk_symbolic_color_new_win32   (const gchar        *theme_class,
                                                   gint                id);

GDK_DEPRECATED_IN_3_8
GtkSymbolicColor * gtk_symbolic_color_ref         (GtkSymbolicColor   *color);
GDK_DEPRECATED_IN_3_8
void               gtk_symbolic_color_unref       (GtkSymbolicColor   *color);

GDK_DEPRECATED_IN_3_8
char *             gtk_symbolic_color_to_string   (GtkSymbolicColor   *color);

GDK_DEPRECATED_IN_3_8
gboolean           gtk_symbolic_color_resolve     (GtkSymbolicColor   *color,
                                                   GtkStyleProperties *props,
                                                   GdkRGBA            *resolved_color);

G_END_DECLS

#endif /* __GTK_SYMBOLIC_COLOR_H__ */
