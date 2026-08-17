/* GTK - The GIMP Toolkit
 * gtkfilechooserwidget.h: Embeddable file selector widget
 * Copyright (C) 2003, Red Hat, Inc.
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
 * License along with this library; if not, write to the
 * Free Software Foundation, Inc., 59 Temple Place - Suite 330,
 * Boston, MA 02111-1307, USA.
 */

#ifndef __GTK_FILE_CHOOSER_WIDGET_H__
#define __GTK_FILE_CHOOSER_WIDGET_H__

#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkfilechooser.h>
#include <gtk/gtkvbox.h>

G_BEGIN_DECLS

#define GTK_TYPE_FILE_CHOOSER_WIDGET             (gtk_file_chooser_widget_get_type ())
#define GTK_FILE_CHOOSER_WIDGET(obj)             (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_FILE_CHOOSER_WIDGET, GtkFileChooserWidget))
#define GTK_FILE_CHOOSER_WIDGET_CLASS(klass)     (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_FILE_CHOOSER_WIDGET, GtkFileChooserWidgetClass))
#define GTK_IS_FILE_CHOOSER_WIDGET(obj)          (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_FILE_CHOOSER_WIDGET))
#define GTK_IS_FILE_CHOOSER_WIDGET_CLASS(klass)  (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_FILE_CHOOSER_WIDGET))
#define GTK_FILE_CHOOSER_WIDGET_GET_CLASS(obj)   (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_FILE_CHOOSER_WIDGET, GtkFileChooserWidgetClass))

typedef struct _GtkFileChooserWidget        GtkFileChooserWidget;
typedef struct _GtkFileChooserWidgetPrivate GtkFileChooserWidgetPrivate;
typedef struct _GtkFileChooserWidgetClass   GtkFileChooserWidgetClass;

struct _GtkFileChooserWidget
{
  GtkVBox parent_instance;

  GtkFileChooserWidgetPrivate *GSEAL (priv);
};

struct _GtkFileChooserWidgetClass
{
  GtkVBoxClass parent_class;
};

GType      gtk_file_chooser_widget_get_type         (void) G_GNUC_CONST;
GtkWidget *gtk_file_chooser_widget_new              (GtkFileChooserAction  action);


#ifndef GTK_DISABLE_DEPRECATED
GtkWidget *gtk_file_chooser_widget_new_with_backend (GtkFileChooserAction  action,
						     const gchar          *backend);
#endif /* GTK_DISABLE_DEPRECATED */

G_END_DECLS

#endif /* __GTK_FILE_CHOOSER_WIDGET_H__ */
