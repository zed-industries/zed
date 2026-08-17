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
#include <gtk/gtkmenu.h>
#include <gtk/gtkpopover.h>

G_BEGIN_DECLS

#define GTK_TYPE_MENU_BUTTON            (gtk_menu_button_get_type ())
#define GTK_MENU_BUTTON(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_MENU_BUTTON, GtkMenuButton))
#define GTK_MENU_BUTTON_CLASS(klass)    (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_MENU_BUTTON, GtkMenuButtonClass))
#define GTK_IS_MENU_BUTTON(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_MENU_BUTTON))
#define GTK_IS_MENU_BUTTON_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_MENU_BUTTON))
#define GTK_MENU_BUTTON_GET_CLASS(obj)  (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_MENU_BUTTON, GtkMenuButtonClass))

typedef struct _GtkMenuButton        GtkMenuButton;
typedef struct _GtkMenuButtonClass   GtkMenuButtonClass;
typedef struct _GtkMenuButtonPrivate GtkMenuButtonPrivate;

struct _GtkMenuButton
{
  GtkToggleButton parent;

  /*< private >*/
  GtkMenuButtonPrivate *priv;
};

struct _GtkMenuButtonClass
{
  GtkToggleButtonClass parent_class;

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};

GDK_AVAILABLE_IN_3_6
GType        gtk_menu_button_get_type       (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_3_6
GtkWidget   *gtk_menu_button_new            (void);

GDK_AVAILABLE_IN_3_6
void         gtk_menu_button_set_popup      (GtkMenuButton *menu_button,
                                             GtkWidget     *menu);
GDK_AVAILABLE_IN_3_6
GtkMenu     *gtk_menu_button_get_popup      (GtkMenuButton *menu_button);

GDK_AVAILABLE_IN_3_12
void         gtk_menu_button_set_popover    (GtkMenuButton *menu_button,
                                             GtkWidget     *popover);
GDK_AVAILABLE_IN_3_12
GtkPopover  *gtk_menu_button_get_popover    (GtkMenuButton *menu_button);

GDK_AVAILABLE_IN_3_6
void         gtk_menu_button_set_direction  (GtkMenuButton *menu_button,
                                             GtkArrowType   direction);
GDK_AVAILABLE_IN_3_6
GtkArrowType gtk_menu_button_get_direction  (GtkMenuButton *menu_button);

GDK_AVAILABLE_IN_3_6
void         gtk_menu_button_set_menu_model (GtkMenuButton *menu_button,
                                             GMenuModel    *menu_model);
GDK_AVAILABLE_IN_3_6
GMenuModel  *gtk_menu_button_get_menu_model (GtkMenuButton *menu_button);

GDK_AVAILABLE_IN_3_6
void         gtk_menu_button_set_align_widget (GtkMenuButton *menu_button,
                                               GtkWidget     *align_widget);
GDK_AVAILABLE_IN_3_6
GtkWidget   *gtk_menu_button_get_align_widget (GtkMenuButton *menu_button);

GDK_AVAILABLE_IN_3_12
void         gtk_menu_button_set_use_popover (GtkMenuButton *menu_button,
                                              gboolean       use_popover);

GDK_AVAILABLE_IN_3_12
gboolean     gtk_menu_button_get_use_popover (GtkMenuButton *menu_button);


G_END_DECLS

#endif /* __GTK_MENU_BUTTON_H__ */
