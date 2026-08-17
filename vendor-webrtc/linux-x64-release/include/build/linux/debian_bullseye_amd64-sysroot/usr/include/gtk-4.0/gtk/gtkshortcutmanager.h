/*
 * Copyright © 2018 Benjamin Otte
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library. If not, see <http://www.gnu.org/licenses/>.
 *
 * Authors: Benjamin Otte <otte@gnome.org>
 */

#ifndef __GTK_SHORTCUT_MANAGER_H__
#define __GTK_SHORTCUT_MANAGER_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkshortcutcontroller.h>
#include <gtk/gtkwidget.h>

G_BEGIN_DECLS

#define GTK_TYPE_SHORTCUT_MANAGER               (gtk_shortcut_manager_get_type ())

GDK_AVAILABLE_IN_ALL
G_DECLARE_INTERFACE (GtkShortcutManager, gtk_shortcut_manager, GTK, SHORTCUT_MANAGER, GtkWidget)

/**
 * GtkShortcutManagerInterface:
 * @add_controller: Add a `GtkShortcutController` to be managed.
 * @remove_controller: Remove a `GtkShortcutController` that had previously
 *   been added
 *
 * The list of functions that can be implemented for the `GtkShortcutManager`
 * interface.
 *
 * Note that no function is mandatory to implement, the default implementation
 * will work fine.
 */
struct _GtkShortcutManagerInterface
{
  /*< private >*/
  GTypeInterface g_iface;

  /*< public >*/
  void                  (* add_controller)              (GtkShortcutManager           *self,
                                                         GtkShortcutController        *controller);
  void                  (* remove_controller)           (GtkShortcutManager           *self,
                                                         GtkShortcutController        *controller);
};


G_END_DECLS

#endif /* __GTK_SHORTCUT_MANAGER_H__ */
