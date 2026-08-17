/* GTK+ - accessibility implementations
 * Copyright 2019 Samuel Thibault <sthibault@hypra.fr>
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

#ifndef __GTK_SOCKET_ACCESSIBLE_H__
#define __GTK_SOCKET_ACCESSIBLE_H__

#if !defined (__GTK_A11Y_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk-a11y.h> can be included directly."
#endif

#include <gtk/a11y/gtkcontaineraccessible.h>

G_BEGIN_DECLS

#define GTK_TYPE_SOCKET_ACCESSIBLE                         (gtk_socket_accessible_get_type ())
#define GTK_SOCKET_ACCESSIBLE(obj)                         (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_SOCKET_ACCESSIBLE, GtkSocketAccessible))
#define GTK_SOCKET_ACCESSIBLE_CLASS(klass)                 (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_SOCKET_ACCESSIBLE, GtkSocketAccessibleClass))
#define GTK_IS_SOCKET_ACCESSIBLE(obj)                      (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_SOCKET_ACCESSIBLE))
#define GTK_IS_SOCKET_ACCESSIBLE_CLASS(klass)              (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_SOCKET_ACCESSIBLE))
#define GTK_SOCKET_ACCESSIBLE_GET_CLASS(obj)               (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_SOCKET_ACCESSIBLE, GtkSocketAccessibleClass))

typedef struct _GtkSocketAccessible        GtkSocketAccessible;
typedef struct _GtkSocketAccessibleClass   GtkSocketAccessibleClass;
typedef struct _GtkSocketAccessiblePrivate GtkSocketAccessiblePrivate;

struct _GtkSocketAccessible
{
  GtkContainerAccessible parent;

  GtkSocketAccessiblePrivate *priv;
};

struct _GtkSocketAccessibleClass
{
  GtkContainerAccessibleClass parent_class;
};

GDK_AVAILABLE_IN_ALL
GType gtk_socket_accessible_get_type (void);

GDK_AVAILABLE_IN_ALL
void gtk_socket_accessible_embed (GtkSocketAccessible *socket, gchar *path);

G_END_DECLS

#endif /* __GTK_SOCKET_ACCESSIBLE_H__ */
