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

#ifndef __GTK_SHORTCUT_H__
#define __GTK_SHORTCUT_H__

#include <gtk/gtktypes.h>

G_BEGIN_DECLS

#define GTK_TYPE_SHORTCUT         (gtk_shortcut_get_type ())

GDK_AVAILABLE_IN_ALL
G_DECLARE_FINAL_TYPE (GtkShortcut, gtk_shortcut, GTK, SHORTCUT, GObject)

GDK_AVAILABLE_IN_ALL
GtkShortcut *   gtk_shortcut_new                                (GtkShortcutTrigger     *trigger,
                                                                 GtkShortcutAction      *action);
GDK_AVAILABLE_IN_ALL
GtkShortcut *   gtk_shortcut_new_with_arguments                 (GtkShortcutTrigger     *trigger,
                                                                 GtkShortcutAction      *action,
                                                                 const char             *format_string,
                                                                 ...);

GDK_AVAILABLE_IN_ALL
GtkShortcutTrigger *
                gtk_shortcut_get_trigger                        (GtkShortcut            *self);
GDK_AVAILABLE_IN_ALL
void            gtk_shortcut_set_trigger                        (GtkShortcut            *self,
                                                                 GtkShortcutTrigger     *trigger);
GDK_AVAILABLE_IN_ALL
GtkShortcutAction *
                gtk_shortcut_get_action                         (GtkShortcut            *self);
GDK_AVAILABLE_IN_ALL
void            gtk_shortcut_set_action                         (GtkShortcut            *self,
                                                                 GtkShortcutAction      *action);

GDK_AVAILABLE_IN_ALL
GVariant *      gtk_shortcut_get_arguments                      (GtkShortcut            *self);
GDK_AVAILABLE_IN_ALL
void            gtk_shortcut_set_arguments                      (GtkShortcut            *self,
                                                                 GVariant               *args);

G_END_DECLS

#endif  /* __GTK_SHORTCUT_H__ */
