/* GTK+ - accessibility implementations
 * Copyright 2001 Sun Microsystems Inc.
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

#ifndef __GTK_MENU_SHELL_ACCESSIBLE_H__
#define __GTK_MENU_SHELL_ACCESSIBLE_H__

#if !defined (__GTK_A11Y_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk-a11y.h> can be included directly."
#endif

#include <gtk/a11y/gtkcontaineraccessible.h>

G_BEGIN_DECLS

#define GTK_TYPE_MENU_SHELL_ACCESSIBLE                    (gtk_menu_shell_accessible_get_type ())
#define GTK_MENU_SHELL_ACCESSIBLE(obj)                    (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_MENU_SHELL_ACCESSIBLE, GtkMenuShellAccessible))
#define GTK_MENU_SHELL_ACCESSIBLE_CLASS(klass)            (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_MENU_SHELL_ACCESSIBLE, GtkMenuShellAccessibleClass))
#define GTK_IS_MENU_SHELL_ACCESSIBLE(obj)                 (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_MENU_SHELL_ACCESSIBLE))
#define GTK_IS_MENU_SHELL_ACCESSIBLE_CLASS(klass)         (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_MENU_SHELL_ACCESSIBLE))
#define GTK_MENU_SHELL_ACCESSIBLE_GET_CLASS(obj)          (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_MENU_SHELL_ACCESSIBLE, GtkMenuShellAccessibleClass))

typedef struct _GtkMenuShellAccessible        GtkMenuShellAccessible;
typedef struct _GtkMenuShellAccessibleClass   GtkMenuShellAccessibleClass;
typedef struct _GtkMenuShellAccessiblePrivate GtkMenuShellAccessiblePrivate;

struct _GtkMenuShellAccessible
{
  GtkContainerAccessible parent;

  GtkMenuShellAccessiblePrivate *priv;
};

struct _GtkMenuShellAccessibleClass
{
  GtkContainerAccessibleClass parent_class;
};

GDK_AVAILABLE_IN_ALL
GType gtk_menu_shell_accessible_get_type (void);

G_END_DECLS

#endif /* __GTK_MENU_SHELL_ACCESSIBLE_H__ */
