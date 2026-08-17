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
#define GTK_LOCK_BUTTON_CLASS(k)     (G_TYPE_CHECK_CLASS_CAST ((k), GTK_LOCK_BUTTON,  GtkLockButtonClass))
#define GTK_IS_LOCK_BUTTON(o)        (G_TYPE_CHECK_INSTANCE_TYPE ((o), GTK_TYPE_LOCK_BUTTON))
#define GTK_IS_LOCK_BUTTON_CLASS(k)  (G_TYPE_CHECK_CLASS_TYPE ((k), GTK_TYPE_LOCK_BUTTON))
#define GTK_LOCK_BUTTON_GET_CLASS(o) (G_TYPE_INSTANCE_GET_CLASS ((o), GTK_TYPE_LOCK_BUTTON, GtkLockButtonClass))

typedef struct _GtkLockButton        GtkLockButton;
typedef struct _GtkLockButtonClass   GtkLockButtonClass;
typedef struct _GtkLockButtonPrivate GtkLockButtonPrivate;

struct _GtkLockButton
{
  GtkButton parent;

  GtkLockButtonPrivate *priv;
};

/**
 * GtkLockButtonClass:
 * @parent_class: The parent class.
 */
struct _GtkLockButtonClass
{
  GtkButtonClass parent_class;

  /*< private >*/

  void (*reserved0) (void);
  void (*reserved1) (void);
  void (*reserved2) (void);
  void (*reserved3) (void);
  void (*reserved4) (void);
  void (*reserved5) (void);
  void (*reserved6) (void);
  void (*reserved7) (void);
};

GDK_AVAILABLE_IN_3_2
GType        gtk_lock_button_get_type       (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_3_2
GtkWidget   *gtk_lock_button_new            (GPermission   *permission);
GDK_AVAILABLE_IN_3_2
GPermission *gtk_lock_button_get_permission (GtkLockButton *button);
GDK_AVAILABLE_IN_3_2
void         gtk_lock_button_set_permission (GtkLockButton *button,
                                             GPermission   *permission);


G_END_DECLS

#endif  /* __GTK_LOCK_BUTTON_H__ */
