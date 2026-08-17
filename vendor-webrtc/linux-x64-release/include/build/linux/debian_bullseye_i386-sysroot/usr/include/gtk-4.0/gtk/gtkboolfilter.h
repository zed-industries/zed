/*
 * Copyright © 2020 Benjamin Otte
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

#ifndef __GTK_BOOL_FILTER_H__
#define __GTK_BOOL_FILTER_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkexpression.h>
#include <gtk/gtkfilter.h>

G_BEGIN_DECLS

#define GTK_TYPE_BOOL_FILTER             (gtk_bool_filter_get_type ())
GDK_AVAILABLE_IN_ALL
G_DECLARE_FINAL_TYPE (GtkBoolFilter, gtk_bool_filter, GTK, BOOL_FILTER, GtkFilter)

GDK_AVAILABLE_IN_ALL
GtkBoolFilter *         gtk_bool_filter_new                     (GtkExpression          *expression);

GDK_AVAILABLE_IN_ALL
GtkExpression *         gtk_bool_filter_get_expression          (GtkBoolFilter          *self);
GDK_AVAILABLE_IN_ALL
void                    gtk_bool_filter_set_expression          (GtkBoolFilter          *self,
                                                                 GtkExpression          *expression);
GDK_AVAILABLE_IN_ALL
gboolean                gtk_bool_filter_get_invert              (GtkBoolFilter          *self);
GDK_AVAILABLE_IN_ALL
void                    gtk_bool_filter_set_invert              (GtkBoolFilter          *self,
                                                                 gboolean                invert);


G_END_DECLS

#endif /* __GTK_BOOL_FILTER_H__ */
