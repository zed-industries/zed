/* GTK - The GIMP Toolkit
 * Copyright (C) 2011      Alberto Ruiz <aruiz@gnome.org>
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

#ifndef __GTK_FONT_CHOOSER_WIDGET_H__
#define __GTK_FONT_CHOOSER_WIDGET_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkwidget.h>

G_BEGIN_DECLS

#define GTK_TYPE_FONT_CHOOSER_WIDGET              (gtk_font_chooser_widget_get_type ())
#define GTK_FONT_CHOOSER_WIDGET(obj)              (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_FONT_CHOOSER_WIDGET, GtkFontChooserWidget))
#define GTK_IS_FONT_CHOOSER_WIDGET(obj)           (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_FONT_CHOOSER_WIDGET))

typedef struct _GtkFontChooserWidget              GtkFontChooserWidget;

GDK_AVAILABLE_IN_ALL
GType        gtk_font_chooser_widget_get_type                 (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
GtkWidget*   gtk_font_chooser_widget_new                      (void);

G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkFontChooserWidget, g_object_unref)

G_END_DECLS

#endif /* __GTK_FONT_CHOOSER_WIDGET_H__ */
