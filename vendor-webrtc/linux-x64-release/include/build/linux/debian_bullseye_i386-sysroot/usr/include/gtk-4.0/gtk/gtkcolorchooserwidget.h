/* GTK - The GIMP Toolkit
 * Copyright (C) 2012 Red Hat, Inc.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.	 See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library. If not, see <http://www.gnu.org/licenses/>.
 */

#ifndef __GTK_COLOR_CHOOSER_WIDGET_H__
#define __GTK_COLOR_CHOOSER_WIDGET_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkwidget.h>

G_BEGIN_DECLS

#define GTK_TYPE_COLOR_CHOOSER_WIDGET              (gtk_color_chooser_widget_get_type ())
#define GTK_COLOR_CHOOSER_WIDGET(obj)              (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_COLOR_CHOOSER_WIDGET, GtkColorChooserWidget))
#define GTK_IS_COLOR_CHOOSER_WIDGET(obj)           (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_COLOR_CHOOSER_WIDGET))

typedef struct _GtkColorChooserWidget        GtkColorChooserWidget;

GDK_AVAILABLE_IN_ALL
GType       gtk_color_chooser_widget_get_type (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
GtkWidget * gtk_color_chooser_widget_new      (void);

G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkColorChooserWidget, g_object_unref)

G_END_DECLS

#endif /* __GTK_COLOR_CHOOSER_WIDGET_H__ */
