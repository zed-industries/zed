/*
 * gtkappchooserbutton.h: an app-chooser button
 *
 * Copyright (C) 2010 Red Hat, Inc.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public License as
 * published by the Free Software Foundation; either version 2 of the
 * License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public
 * License along with this library. If not, see <http://www.gnu.org/licenses/>.
 *
 * Authors: Cosimo Cecchi <ccecchi@redhat.com>
 */

#ifndef __GTK_APP_CHOOSER_BUTTON_H__
#define __GTK_APP_CHOOSER_BUTTON_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkwidget.h>
#include <gio/gio.h>

G_BEGIN_DECLS

#define GTK_TYPE_APP_CHOOSER_BUTTON            (gtk_app_chooser_button_get_type ())
#define GTK_APP_CHOOSER_BUTTON(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_APP_CHOOSER_BUTTON, GtkAppChooserButton))
#define GTK_IS_APP_CHOOSER_BUTTON(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_APP_CHOOSER_BUTTON))

typedef struct _GtkAppChooserButton        GtkAppChooserButton;

GDK_AVAILABLE_IN_ALL
GType       gtk_app_chooser_button_get_type           (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
GtkWidget * gtk_app_chooser_button_new                (const char          *content_type);

GDK_AVAILABLE_IN_ALL
void        gtk_app_chooser_button_append_separator   (GtkAppChooserButton *self);
GDK_AVAILABLE_IN_ALL
void        gtk_app_chooser_button_append_custom_item (GtkAppChooserButton *self,
                                                       const char          *name,
                                                       const char          *label,
                                                       GIcon               *icon);
GDK_AVAILABLE_IN_ALL
void     gtk_app_chooser_button_set_active_custom_item (GtkAppChooserButton *self,
                                                        const char          *name);

GDK_AVAILABLE_IN_ALL
void     gtk_app_chooser_button_set_show_dialog_item  (GtkAppChooserButton *self,
                                                       gboolean             setting);
GDK_AVAILABLE_IN_ALL
gboolean gtk_app_chooser_button_get_show_dialog_item  (GtkAppChooserButton *self);
GDK_AVAILABLE_IN_ALL
void     gtk_app_chooser_button_set_heading           (GtkAppChooserButton *self,
                                                       const char          *heading);
GDK_AVAILABLE_IN_ALL
const char *
         gtk_app_chooser_button_get_heading           (GtkAppChooserButton *self);
GDK_AVAILABLE_IN_ALL
void     gtk_app_chooser_button_set_show_default_item (GtkAppChooserButton *self,
                                                       gboolean             setting);
GDK_AVAILABLE_IN_ALL
gboolean gtk_app_chooser_button_get_show_default_item (GtkAppChooserButton *self);

GDK_AVAILABLE_IN_ALL
gboolean gtk_app_chooser_button_get_modal             (GtkAppChooserButton *self);
GDK_AVAILABLE_IN_ALL
void     gtk_app_chooser_button_set_modal             (GtkAppChooserButton *self,
                                                       gboolean             modal);

G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkAppChooserButton, g_object_unref)

G_END_DECLS

#endif /* __GTK_APP_CHOOSER_BUTTON_H__ */
