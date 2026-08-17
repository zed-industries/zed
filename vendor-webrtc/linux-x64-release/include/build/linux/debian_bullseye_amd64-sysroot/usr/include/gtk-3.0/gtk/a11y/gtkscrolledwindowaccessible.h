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

#ifndef __GTK_SCROLLED_WINDOW_ACCESSIBLE_H__
#define __GTK_SCROLLED_WINDOW_ACCESSIBLE_H__

#if !defined (__GTK_A11Y_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk-a11y.h> can be included directly."
#endif

#include <gtk/a11y/gtkcontaineraccessible.h>

G_BEGIN_DECLS

#define GTK_TYPE_SCROLLED_WINDOW_ACCESSIBLE            (gtk_scrolled_window_accessible_get_type ())
#define GTK_SCROLLED_WINDOW_ACCESSIBLE(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_SCROLLED_WINDOW_ACCESSIBLE, GtkScrolledWindowAccessible))
#define GTK_SCROLLED_WINDOW_ACCESSIBLE_CLASS(klass)    (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_SCROLLED_WINDOW_ACCESSIBLE, GtkScrolledWindowAccessibleClass))
#define GTK_IS_SCROLLED_WINDOW_ACCESSIBLE(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_SCROLLED_WINDOW_ACCESSIBLE))
#define GTK_IS_SCROLLED_WINDOW_ACCESSIBLE_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_SCROLLED_WINDOW_ACCESSIBLE))
#define GTK_SCROLLED_WINDOW_ACCESSIBLE_GET_CLASS(obj)  (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_SCROLLED_WINDOW_ACCESSIBLE, GtkScrolledWindowAccessibleClass))

typedef struct _GtkScrolledWindowAccessible        GtkScrolledWindowAccessible;
typedef struct _GtkScrolledWindowAccessibleClass   GtkScrolledWindowAccessibleClass;
typedef struct _GtkScrolledWindowAccessiblePrivate GtkScrolledWindowAccessiblePrivate;

struct _GtkScrolledWindowAccessible
{
  GtkContainerAccessible parent;

  GtkScrolledWindowAccessiblePrivate *priv;
};

struct _GtkScrolledWindowAccessibleClass
{
  GtkContainerAccessibleClass parent_class;
};

GDK_AVAILABLE_IN_ALL
GType gtk_scrolled_window_accessible_get_type (void);

G_END_DECLS

#endif /* __GTK_SCROLLED_WINDOW_ACCESSIBLE_H__ */
