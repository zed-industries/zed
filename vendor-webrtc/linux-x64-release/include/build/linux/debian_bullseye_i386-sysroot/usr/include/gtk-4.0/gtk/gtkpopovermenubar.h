/* GTK - The GIMP Toolkit
 * Copyright (C) 2019 Red Hat, Inc.
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

#ifndef __GTK_POPOVER_MENU_BAR_H__
#define __GTK_POPOVER_MENU_BAR_H__


#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkpopover.h>


G_BEGIN_DECLS


#define	GTK_TYPE_POPOVER_MENU_BAR               (gtk_popover_menu_bar_get_type ())
#define GTK_POPOVER_MENU_BAR(obj)               (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_POPOVER_MENU_BAR, GtkPopoverMenuBar))
#define GTK_IS_POPOVER_MENU_BAR(obj)            (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_POPOVER_MENU_BAR))

typedef struct _GtkPopoverMenuBar GtkPopoverMenuBar;

GDK_AVAILABLE_IN_ALL
GType        gtk_popover_menu_bar_get_type       (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
GtkWidget *  gtk_popover_menu_bar_new_from_model (GMenuModel        *model);

GDK_AVAILABLE_IN_ALL
void         gtk_popover_menu_bar_set_menu_model (GtkPopoverMenuBar *bar,
                                                  GMenuModel        *model);
GDK_AVAILABLE_IN_ALL
GMenuModel * gtk_popover_menu_bar_get_menu_model (GtkPopoverMenuBar *bar);

GDK_AVAILABLE_IN_ALL
gboolean     gtk_popover_menu_bar_add_child      (GtkPopoverMenuBar *bar,
                                                  GtkWidget         *child,
                                                  const char        *id);

GDK_AVAILABLE_IN_ALL
gboolean     gtk_popover_menu_bar_remove_child   (GtkPopoverMenuBar *bar,
                                                  GtkWidget         *child);

G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkPopoverMenuBar, g_object_unref)

G_END_DECLS


#endif /* __GTK_POPOVER_MENU_BAR_H__ */
