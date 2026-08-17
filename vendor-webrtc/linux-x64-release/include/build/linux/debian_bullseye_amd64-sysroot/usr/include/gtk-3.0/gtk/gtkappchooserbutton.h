/*
 * gtkappchooserbutton.h: an app-chooser combobox
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

#include <gtk/gtkcombobox.h>
#include <gio/gio.h>

G_BEGIN_DECLS

#define GTK_TYPE_APP_CHOOSER_BUTTON            (gtk_app_chooser_button_get_type ())
#define GTK_APP_CHOOSER_BUTTON(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_APP_CHOOSER_BUTTON, GtkAppChooserButton))
#define GTK_APP_CHOOSER_BUTTON_CLASS(klass)    (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_APP_CHOOSER_BUTTON, GtkAppChooserButtonClass))
#define GTK_IS_APP_CHOOSER_BUTTON(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_APP_CHOOSER_BUTTON))
#define GTK_IS_APP_CHOOSER_BUTTON_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_APP_CHOOSER_BUTTON))
#define GTK_APP_CHOOSER_BUTTON_GET_CLASS(obj)  (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_APP_CHOOSER_BUTTON, GtkAppChooserButtonClass))

typedef struct _GtkAppChooserButton        GtkAppChooserButton;
typedef struct _GtkAppChooserButtonClass   GtkAppChooserButtonClass;
typedef struct _GtkAppChooserButtonPrivate GtkAppChooserButtonPrivate;

struct _GtkAppChooserButton {
  GtkComboBox parent;

  /*< private >*/
  GtkAppChooserButtonPrivate *priv;
};

/**
 * GtkAppChooserButtonClass:
 * @parent_class: The parent class.
 * @custom_item_activated: Signal emitted when a custom item,
 *    previously added with gtk_app_chooser_button_append_custom_item(),
 *    is activated from the dropdown menu.
 */
struct _GtkAppChooserButtonClass {
  GtkComboBoxClass parent_class;

  /*< public >*/

  void (* custom_item_activated) (GtkAppChooserButton *self,
                                  const gchar *item_name);

  /*< private >*/

  /* padding for future class expansion */
  gpointer padding[16];
};

GDK_AVAILABLE_IN_ALL
GType       gtk_app_chooser_button_get_type           (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
GtkWidget * gtk_app_chooser_button_new                (const gchar         *content_type);

GDK_AVAILABLE_IN_ALL
void        gtk_app_chooser_button_append_separator   (GtkAppChooserButton *self);
GDK_AVAILABLE_IN_ALL
void        gtk_app_chooser_button_append_custom_item (GtkAppChooserButton *self,
                                                       const gchar         *name,
                                                       const gchar         *label,
                                                       GIcon               *icon);
GDK_AVAILABLE_IN_ALL
void     gtk_app_chooser_button_set_active_custom_item (GtkAppChooserButton *self,
                                                        const gchar         *name);

GDK_AVAILABLE_IN_ALL
void     gtk_app_chooser_button_set_show_dialog_item  (GtkAppChooserButton *self,
                                                       gboolean             setting);
GDK_AVAILABLE_IN_ALL
gboolean gtk_app_chooser_button_get_show_dialog_item  (GtkAppChooserButton *self);
GDK_AVAILABLE_IN_ALL
void     gtk_app_chooser_button_set_heading           (GtkAppChooserButton *self,
                                                       const gchar         *heading);
GDK_AVAILABLE_IN_ALL
const gchar *
         gtk_app_chooser_button_get_heading           (GtkAppChooserButton *self);
GDK_AVAILABLE_IN_3_2
void     gtk_app_chooser_button_set_show_default_item (GtkAppChooserButton *self,
                                                       gboolean             setting);
GDK_AVAILABLE_IN_3_2
gboolean gtk_app_chooser_button_get_show_default_item (GtkAppChooserButton *self);

G_END_DECLS

#endif /* __GTK_APP_CHOOSER_BUTTON_H__ */
