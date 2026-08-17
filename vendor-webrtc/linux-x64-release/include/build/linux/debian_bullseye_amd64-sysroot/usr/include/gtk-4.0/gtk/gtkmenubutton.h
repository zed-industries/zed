/* GTK - The GIMP Toolkit
 *
 * Copyright (C) 2003 Ricardo Fernandez Pascual
 * Copyright (C) 2004 Paolo Borelli
 * Copyright (C) 2012 Bastien Nocera
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

#ifndef __GTK_MENU_BUTTON_H__
#define __GTK_MENU_BUTTON_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtktogglebutton.h>
#include <gtk/gtkpopover.h>

G_BEGIN_DECLS

#define GTK_TYPE_MENU_BUTTON            (gtk_menu_button_get_type ())
#define GTK_MENU_BUTTON(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_MENU_BUTTON, GtkMenuButton))
#define GTK_IS_MENU_BUTTON(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_MENU_BUTTON))

typedef struct _GtkMenuButton        GtkMenuButton;

/**
 * GtkMenuButtonCreatePopupFunc:
 * @menu_button: the `GtkMenuButton`
 * @user_data: User data passed to gtk_menu_button_set_create_popup_func()
 *
 * User-provided callback function to create a popup for a
 * `GtkMenuButton` on demand.
 *
 * This function is called when the popup of @menu_button is shown,
 * but none has been provided via [method@Gtk.MenuButton.set_popover]
 * or [method@Gtk.MenuButton.set_menu_model].
 */
typedef void  (*GtkMenuButtonCreatePopupFunc) (GtkMenuButton *menu_button,
                                               gpointer       user_data);

GDK_AVAILABLE_IN_ALL
GType        gtk_menu_button_get_type       (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
GtkWidget   *gtk_menu_button_new            (void);

GDK_AVAILABLE_IN_ALL
void         gtk_menu_button_set_popover    (GtkMenuButton *menu_button,
                                             GtkWidget     *popover);
GDK_AVAILABLE_IN_ALL
GtkPopover  *gtk_menu_button_get_popover    (GtkMenuButton *menu_button);

GDK_AVAILABLE_IN_ALL
void         gtk_menu_button_set_direction  (GtkMenuButton *menu_button,
                                             GtkArrowType   direction);
GDK_AVAILABLE_IN_ALL
GtkArrowType gtk_menu_button_get_direction  (GtkMenuButton *menu_button);

GDK_AVAILABLE_IN_ALL
void         gtk_menu_button_set_menu_model (GtkMenuButton *menu_button,
                                             GMenuModel    *menu_model);
GDK_AVAILABLE_IN_ALL
GMenuModel  *gtk_menu_button_get_menu_model (GtkMenuButton *menu_button);

GDK_AVAILABLE_IN_ALL
void         gtk_menu_button_set_icon_name (GtkMenuButton *menu_button,
                                            const char    *icon_name);
GDK_AVAILABLE_IN_ALL
const char * gtk_menu_button_get_icon_name (GtkMenuButton *menu_button);

GDK_AVAILABLE_IN_4_4
void         gtk_menu_button_set_always_show_arrow (GtkMenuButton *menu_button,
                                                    gboolean       always_show_arrow);
GDK_AVAILABLE_IN_4_4
gboolean     gtk_menu_button_get_always_show_arrow (GtkMenuButton *menu_button);

GDK_AVAILABLE_IN_ALL
void         gtk_menu_button_set_label (GtkMenuButton *menu_button,
                                        const char    *label);
GDK_AVAILABLE_IN_ALL
const char * gtk_menu_button_get_label (GtkMenuButton *menu_button);

GDK_AVAILABLE_IN_ALL
void         gtk_menu_button_set_use_underline (GtkMenuButton *menu_button,
                                                gboolean       use_underline);
GDK_AVAILABLE_IN_ALL
gboolean     gtk_menu_button_get_use_underline (GtkMenuButton *menu_button);

GDK_AVAILABLE_IN_ALL
void           gtk_menu_button_set_has_frame (GtkMenuButton  *menu_button,
                                              gboolean        has_frame);
GDK_AVAILABLE_IN_ALL
gboolean       gtk_menu_button_get_has_frame (GtkMenuButton  *menu_button);

GDK_AVAILABLE_IN_ALL
void          gtk_menu_button_popup (GtkMenuButton *menu_button);
GDK_AVAILABLE_IN_ALL
void          gtk_menu_button_popdown (GtkMenuButton *menu_button);

GDK_AVAILABLE_IN_ALL
void          gtk_menu_button_set_create_popup_func (GtkMenuButton                *menu_button,
                                                     GtkMenuButtonCreatePopupFunc  func,
                                                     gpointer                      user_data,
                                                     GDestroyNotify                destroy_notify);

GDK_AVAILABLE_IN_4_4
void          gtk_menu_button_set_primary (GtkMenuButton *menu_button,
                                           gboolean       primary);
GDK_AVAILABLE_IN_4_4
gboolean      gtk_menu_button_get_primary (GtkMenuButton *menu_button);

GDK_AVAILABLE_IN_4_6
void          gtk_menu_button_set_child   (GtkMenuButton *menu_button,
                                           GtkWidget     *child);
GDK_AVAILABLE_IN_4_6
GtkWidget *   gtk_menu_button_get_child   (GtkMenuButton *menu_button);

G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkMenuButton, g_object_unref)

G_END_DECLS

#endif /* __GTK_MENU_BUTTON_H__ */
