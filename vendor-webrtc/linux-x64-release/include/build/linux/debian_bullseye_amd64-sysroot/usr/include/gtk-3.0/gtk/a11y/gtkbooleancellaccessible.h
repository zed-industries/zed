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
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.	 See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public
 * License along with this library. If not, see <http://www.gnu.org/licenses/>.
 */

#ifndef __GTK_BOOLEAN_CELL_ACCESSIBLE_H__
#define __GTK_BOOLEAN_CELL_ACCESSIBLE_H__

#if !defined (__GTK_A11Y_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk-a11y.h> can be included directly."
#endif

#include <atk/atk.h>
#include <gtk/a11y/gtkrenderercellaccessible.h>

G_BEGIN_DECLS

#define GTK_TYPE_BOOLEAN_CELL_ACCESSIBLE            (gtk_boolean_cell_accessible_get_type ())
#define GTK_BOOLEAN_CELL_ACCESSIBLE(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_BOOLEAN_CELL_ACCESSIBLE, GtkBooleanCellAccessible))
#define GTK_BOOLEAN_CELL_ACCESSIBLE_CLASS(klass)    (G_TYPE_CHECK_CLASS_CAST ((klass), GAIL_BOOLEAN_CELL, GtkBooleanCellAccessibleClass))
#define GTK_IS_BOOLEAN_CELL_ACCESSIBLE(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_BOOLEAN_CELL_ACCESSIBLE))
#define GTK_IS_BOOLEAN_CELL_ACCESSIBLE_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_BOOLEAN_CELL_ACCESSIBLE))
#define GTK_BOOLEAN_CELL_ACCESSIBLE_GET_CLASS(obj)  (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_BOOLEAN_CELL_ACCESSIBLE, GtkBooleanCellAccessibleClass))

typedef struct _GtkBooleanCellAccessible        GtkBooleanCellAccessible;
typedef struct _GtkBooleanCellAccessibleClass   GtkBooleanCellAccessibleClass;
typedef struct _GtkBooleanCellAccessiblePrivate GtkBooleanCellAccessiblePrivate;

struct _GtkBooleanCellAccessible
{
  GtkRendererCellAccessible parent;

  GtkBooleanCellAccessiblePrivate *priv;
};

struct _GtkBooleanCellAccessibleClass
{
  GtkRendererCellAccessibleClass parent_class;
};

GDK_AVAILABLE_IN_ALL
GType      gtk_boolean_cell_accessible_get_type (void);

G_END_DECLS

#endif /* __GAIL_TREE_VIEW_BOOLEAN_CELL_H__ */
