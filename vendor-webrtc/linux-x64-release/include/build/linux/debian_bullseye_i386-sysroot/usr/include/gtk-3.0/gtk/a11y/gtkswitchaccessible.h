/* GTK+ - accessibility implementations
 * Copyright 2011 Red Hat, Inc.
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

#ifndef __GTK_SWITCH_ACCESSIBLE_H__
#define __GTK_SWITCH_ACCESSIBLE_H__

#if !defined (__GTK_A11Y_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk-a11y.h> can be included directly."
#endif

#include <gtk/a11y/gtkwidgetaccessible.h>

G_BEGIN_DECLS

#define GTK_TYPE_SWITCH_ACCESSIBLE              (gtk_switch_accessible_get_type ())
#define GTK_SWITCH_ACCESSIBLE(obj)              (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_SWITCH_ACCESSIBLE, GtkSwitchAccessible))
#define GTK_SWITCH_ACCESSIBLE_CLASS(klass)      (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_SWITCH_ACCESSIBLE, GtkSwitchAccessibleClass))
#define GTK_IS_SWITCH_ACCESSIBLE(obj)           (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_SWITCH_ACCESSIBLE))
#define GTK_IS_SWITCH_ACCESSIBLE_CLASS(klass)   (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_SWITCH_ACCESSIBLE))
#define GTK_SWITCH_ACCESSIBLE_GET_CLASS(obj)    (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_SWITCH_ACCESSIBLE, GtkSwitchAccessibleClass))

typedef struct _GtkSwitchAccessible        GtkSwitchAccessible;
typedef struct _GtkSwitchAccessibleClass   GtkSwitchAccessibleClass;
typedef struct _GtkSwitchAccessiblePrivate GtkSwitchAccessiblePrivate;

struct _GtkSwitchAccessible
{
  GtkWidgetAccessible parent;

  GtkSwitchAccessiblePrivate *priv;
};

struct _GtkSwitchAccessibleClass
{
  GtkWidgetAccessibleClass parent_class;
};

GDK_AVAILABLE_IN_ALL
GType gtk_switch_accessible_get_type (void);

G_END_DECLS

#endif /* __GTK_SWITCH_ACCESSIBLE_H__ */
