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
#define GTK_IS_POPOVER_MENU(o)          (G_TYPE_CHECK_INSTANCE_TYPE ((o), GTK_TYPE_POPOVER_MENU))

typedef struct _GtkPopoverMenu GtkPopoverMenu;

GDK_AVAILABLE_IN_ALL
GType       gtk_popover_menu_get_type (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
GtkWidget * gtk_popover_menu_new_from_model (GMenuModel *model);

/**
 * GtkPopoverMenuFlags:
 * @GTK_POPOVER_MENU_NESTED: Create submenus as nested
 *    popovers. Without this flag, submenus are created as
 *    sliding pages that replace the main menu.
 *
 * Flags that affect how popover menus are created from
 * a menu model.
 */
typedef enum {
  GTK_POPOVER_MENU_NESTED = 1 << 0
} GtkPopoverMenuFlags;

GDK_AVAILABLE_IN_ALL
GtkWidget * gtk_popover_menu_new_from_model_full (GMenuModel          *model,
                                                  GtkPopoverMenuFlags  flags);

GDK_AVAILABLE_IN_ALL
void        gtk_popover_menu_set_menu_model (GtkPopoverMenu *popover,
                                             GMenuModel     *model);
GDK_AVAILABLE_IN_ALL
GMenuModel *gtk_popover_menu_get_menu_model (GtkPopoverMenu *popover);

GDK_AVAILABLE_IN_ALL
gboolean    gtk_popover_menu_add_child (GtkPopoverMenu *popover,
                                        GtkWidget      *child,
                                        const char     *id);

GDK_AVAILABLE_IN_ALL
gboolean    gtk_popover_menu_remove_child (GtkPopoverMenu *popover,
                                           GtkWidget      *child);

G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkPopoverMenu, g_object_unref)

G_END_DECLS

#endif /* __GTK_POPOVER_MENU_H__ */
