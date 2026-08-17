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

#include <gtk/gtkbox.h>

G_BEGIN_DECLS

#define GTK_TYPE_FONT_CHOOSER_WIDGET              (gtk_font_chooser_widget_get_type ())
#define GTK_FONT_CHOOSER_WIDGET(obj)              (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_FONT_CHOOSER_WIDGET, GtkFontChooserWidget))
#define GTK_FONT_CHOOSER_WIDGET_CLASS(klass)      (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_FONT_CHOOSER_WIDGET, GtkFontChooserWidgetClass))
#define GTK_IS_FONT_CHOOSER_WIDGET(obj)           (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_FONT_CHOOSER_WIDGET))
#define GTK_IS_FONT_CHOOSER_WIDGET_CLASS(klass)   (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_FONT_CHOOSER_WIDGET))
#define GTK_FONT_CHOOSER_WIDGET_GET_CLASS(obj)    (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_FONT_CHOOSER_WIDGET, GtkFontChooserWidgetClass))

typedef struct _GtkFontChooserWidget              GtkFontChooserWidget;
typedef struct _GtkFontChooserWidgetPrivate       GtkFontChooserWidgetPrivate;
typedef struct _GtkFontChooserWidgetClass         GtkFontChooserWidgetClass;

struct _GtkFontChooserWidget
{
  GtkBox parent_instance;

  /*< private >*/
  GtkFontChooserWidgetPrivate *priv;
};

/**
 * GtkFontChooserWidgetClass:
 * @parent_class: The parent class.
 */
struct _GtkFontChooserWidgetClass
{
  GtkBoxClass parent_class;

  /*< private >*/

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
  void (*_gtk_reserved5) (void);
  void (*_gtk_reserved6) (void);
  void (*_gtk_reserved7) (void);
  void (*_gtk_reserved8) (void);
};

GDK_AVAILABLE_IN_3_2
GType        gtk_font_chooser_widget_get_type                 (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_3_2
GtkWidget*   gtk_font_chooser_widget_new                      (void);

G_END_DECLS

#endif /* __GTK_FONT_CHOOSER_WIDGET_H__ */
