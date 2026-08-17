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
#define GTK_SIZE_GROUP_CLASS(klass)    (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_SIZE_GROUP, GtkSizeGroupClass))
#define GTK_IS_SIZE_GROUP(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_SIZE_GROUP))
#define GTK_IS_SIZE_GROUP_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_SIZE_GROUP))
#define GTK_SIZE_GROUP_GET_CLASS(obj)  (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_SIZE_GROUP, GtkSizeGroupClass))


typedef struct _GtkSizeGroup              GtkSizeGroup;
typedef struct _GtkSizeGroupPrivate       GtkSizeGroupPrivate;
typedef struct _GtkSizeGroupClass         GtkSizeGroupClass;

struct _GtkSizeGroup
{
  GObject parent_instance;

  /*< private >*/
  GtkSizeGroupPrivate *priv;
};

struct _GtkSizeGroupClass
{
  GObjectClass parent_class;

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
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
GDK_DEPRECATED_IN_3_22
void             gtk_size_group_set_ignore_hidden (GtkSizeGroup *size_group,
						   gboolean      ignore_hidden);
GDK_DEPRECATED_IN_3_22
gboolean         gtk_size_group_get_ignore_hidden (GtkSizeGroup *size_group);
GDK_AVAILABLE_IN_ALL
void             gtk_size_group_add_widget    (GtkSizeGroup     *size_group,
					       GtkWidget        *widget);
GDK_AVAILABLE_IN_ALL
void             gtk_size_group_remove_widget (GtkSizeGroup     *size_group,
					       GtkWidget        *widget);
GDK_AVAILABLE_IN_ALL
GSList *         gtk_size_group_get_widgets   (GtkSizeGroup     *size_group);

G_END_DECLS

#endif /* __GTK_SIZE_GROUP_H__ */
