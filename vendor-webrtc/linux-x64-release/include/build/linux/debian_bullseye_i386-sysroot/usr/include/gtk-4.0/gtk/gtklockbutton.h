/* GTK - The GIMP Toolkit
 * Copyright (C) 2010 Red Hat, Inc.
 * Author: Matthias Clasen
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public
 * License along with this library. If not, see <http://www.gnu.org/licenses/>.
 */

#ifndef __GTK_LOCK_BUTTON_H__
#define __GTK_LOCK_BUTTON_H__

#include <gtk/gtkbutton.h>
#include <gio/gio.h>

G_BEGIN_DECLS

#define GTK_TYPE_LOCK_BUTTON         (gtk_lock_button_get_type ())
#define GTK_LOCK_BUTTON(o)           (G_TYPE_CHECK_INSTANCE_CAST ((o), GTK_TYPE_LOCK_BUTTON, GtkLockButton))
#define GTK_IS_LOCK_BUTTON(o)        (G_TYPE_CHECK_INSTANCE_TYPE ((o), GTK_TYPE_LOCK_BUTTON))

typedef struct _GtkLockButton        GtkLockButton;

GDK_AVAILABLE_IN_ALL
GType        gtk_lock_button_get_type       (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
GtkWidget   *gtk_lock_button_new            (GPermission   *permission);
GDK_AVAILABLE_IN_ALL
GPermission *gtk_lock_button_get_permission (GtkLockButton *button);
GDK_AVAILABLE_IN_ALL
void         gtk_lock_button_set_permission (GtkLockButton *button,
                                             GPermission   *permission);

G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkLockButton, g_object_unref)

G_END_DECLS

#endif  /* __GTK_LOCK_BUTTON_H__ */
