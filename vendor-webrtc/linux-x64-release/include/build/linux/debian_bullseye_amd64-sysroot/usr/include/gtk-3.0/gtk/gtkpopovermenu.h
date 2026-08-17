/* GTK - The GIMP Toolkit
 * Copyright © 2014 Red Hat, Inc.
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

#ifndef __GTK_POPOVER_MENU_H__
#define __GTK_POPOVER_MENU_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkpopover.h>

G_BEGIN_DECLS

#define GTK_TYPE_POPOVER_MENU           (gtk_popover_menu_get_type ())
#define GTK_POPOVER_MENU(o)             (G_TYPE_CHECK_INSTANCE_CAST ((o), GTK_TYPE_POPOVER_MENU, GtkPopoverMenu))
#define GTK_POPOVER_MENU_CLASS(c)       (G_TYPE_CHECK_CLASS_CAST ((c), GTK_TYPE_POPOVER_MENU, GtkPopoverMenuClass))
#define GTK_IS_POPOVER_MENU(o)          (G_TYPE_CHECK_INSTANCE_TYPE ((o), GTK_TYPE_POPOVER_MENU))
#define GTK_IS_POPOVER_MENU_CLASS(o)    (G_TYPE_CHECK_CLASS_TYPE ((o), GTK_TYPE_POPOVER_MENU))
#define GTK_POPOVER_MENU_GET_CLASS(o)   (G_TYPE_INSTANCE_GET_CLASS ((o), GTK_TYPE_POPOVER_MENU, GtkPopoverMenuClass))

typedef struct _GtkPopoverMenu GtkPopoverMenu;
typedef struct _GtkPopoverMenuClass GtkPopoverMenuClass;

struct _GtkPopoverMenuClass
{
  GtkPopoverClass parent_class;

  /*< private >*/

  /* Padding for future expansion */
  gpointer reserved[10];
};

GDK_AVAILABLE_IN_3_16
GType       gtk_popover_menu_get_type (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_3_16
GtkWidget * gtk_popover_menu_new      (void);

GDK_AVAILABLE_IN_3_16
void        gtk_popover_menu_open_submenu (GtkPopoverMenu *popover,
                                           const gchar    *name);


G_END_DECLS

#endif /* __GTK_POPOVER_MENU_H__ */
