/* GTK - The GIMP Toolkit
 * Copyright © 2014 Red Hat Inc.
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
 *
 * Author: Carlos Garnacho <carlosg@gnome.org>
 */

#ifndef __GTK_POPOVER_ACCESSIBLE_H__
#define __GTK_POPOVER_ACCESSIBLE_H__

#if !defined (__GTK_A11Y_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk-a11y.h> can be included directly."
#endif

#include <gtk/a11y/gtkcontaineraccessible.h>

G_BEGIN_DECLS

#define GTK_TYPE_POPOVER_ACCESSIBLE                      (gtk_popover_accessible_get_type ())
#define GTK_POPOVER_ACCESSIBLE(obj)                      (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_POPOVER_ACCESSIBLE, GtkPopoverAccessible))
#define GTK_POPOVER_ACCESSIBLE_CLASS(klass)              (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_POPOVER_ACCESSIBLE, GtkPopoverAccessibleClass))
#define GTK_IS_POPOVER_ACCESSIBLE(obj)                   (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_POPOVER_ACCESSIBLE))
#define GTK_IS_POPOVER_ACCESSIBLE_CLASS(klass)           (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_POPOVER_ACCESSIBLE))
#define GTK_POPOVER_ACCESSIBLE_GET_CLASS(obj)            (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_POPOVER_ACCESSIBLE, GtkPopoverAccessibleClass))

typedef struct _GtkPopoverAccessible        GtkPopoverAccessible;
typedef struct _GtkPopoverAccessibleClass   GtkPopoverAccessibleClass;

struct _GtkPopoverAccessible
{
  GtkContainerAccessible parent;
};

struct _GtkPopoverAccessibleClass
{
  GtkContainerAccessibleClass parent_class;
};

GDK_AVAILABLE_IN_3_12
GType gtk_popover_accessible_get_type (void);

G_END_DECLS

#endif /* __GTK_POPOVER_ACCESSIBLE_H__ */
