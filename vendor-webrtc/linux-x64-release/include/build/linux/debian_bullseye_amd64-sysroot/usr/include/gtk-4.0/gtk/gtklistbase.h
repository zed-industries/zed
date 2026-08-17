/*
 * Copyright © 2019 Benjamin Otte
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

#ifndef __GTK_LIST_BASE_H__
#define __GTK_LIST_BASE_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkwidget.h>
#include <gtk/gtkselectionmodel.h>

G_BEGIN_DECLS

#define GTK_TYPE_LIST_BASE         (gtk_list_base_get_type ())
#define GTK_LIST_BASE(o)           (G_TYPE_CHECK_INSTANCE_CAST ((o), GTK_TYPE_LIST_BASE, GtkListBase))
#define GTK_LIST_BASE_CLASS(k)     (G_TYPE_CHECK_CLASS_CAST ((k), GTK_TYPE_LIST_BASE, GtkListBaseClass))
#define GTK_IS_LIST_BASE(o)        (G_TYPE_CHECK_INSTANCE_TYPE ((o), GTK_TYPE_LIST_BASE))
#define GTK_IS_LIST_BASE_CLASS(k)  (G_TYPE_CHECK_CLASS_TYPE ((k), GTK_TYPE_LIST_BASE))
#define GTK_LIST_BASE_GET_CLASS(o) (G_TYPE_INSTANCE_GET_CLASS ((o), GTK_TYPE_LIST_BASE, GtkListBaseClass))

/**
 * GtkListBase:
 *
 * `GtkListBase` is the abstract base class for GTK's list widgets.
 */
typedef struct _GtkListBase GtkListBase;
typedef struct _GtkListBaseClass GtkListBaseClass;

GDK_AVAILABLE_IN_ALL
GType                   gtk_list_base_get_type                  (void) G_GNUC_CONST;

G_END_DECLS

#endif  /* __GTK_LIST_BASE_H__ */
