/* GTK - The GIMP Toolkit
 * gtksizegroup.h:
 * Copyright (C) 2000 Red Hat Software
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
 */

#ifndef __GTK_SIZE_GROUP_H__
#define __GTK_SIZE_GROUP_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkwidget.h>

G_BEGIN_DECLS

#define GTK_TYPE_SIZE_GROUP            (gtk_size_group_get_type ())
#define GTK_SIZE_GROUP(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_SIZE_GROUP, GtkSizeGroup))
#define GTK_IS_SIZE_GROUP(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_SIZE_GROUP))

typedef struct _GtkSizeGroup              GtkSizeGroup;

struct _GtkSizeGroup
{
  GObject parent_instance;
};

GDK_AVAILABLE_IN_ALL
GType            gtk_size_group_get_type      (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
GtkSizeGroup *   gtk_size_group_new           (GtkSizeGroupMode  mode);
GDK_AVAILABLE_IN_ALL
void             gtk_size_group_set_mode      (GtkSizeGroup     *size_group,
					       GtkSizeGroupMode  mode);
GDK_AVAILABLE_IN_ALL
GtkSizeGroupMode gtk_size_group_get_mode      (GtkSizeGroup     *size_group);
GDK_AVAILABLE_IN_ALL
void             gtk_size_group_add_widget    (GtkSizeGroup     *size_group,
					       GtkWidget        *widget);
GDK_AVAILABLE_IN_ALL
void             gtk_size_group_remove_widget (GtkSizeGroup     *size_group,
					       GtkWidget        *widget);
GDK_AVAILABLE_IN_ALL
GSList *         gtk_size_group_get_widgets   (GtkSizeGroup     *size_group);

G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkSizeGroup, g_object_unref)

G_END_DECLS

#endif /* __GTK_SIZE_GROUP_H__ */
