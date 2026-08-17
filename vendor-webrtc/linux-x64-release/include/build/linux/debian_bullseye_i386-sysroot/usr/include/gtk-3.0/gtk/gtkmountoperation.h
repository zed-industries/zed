/* GTK - The GIMP Toolkit
 * Copyright (C) Christian Kellner <gicmo@gnome.org>
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

/*
 * Modified by the GTK+ Team and others 1997-2000.  See the AUTHORS
 * file for a list of people on the GTK+ Team.  See the ChangeLog
 * files for a list of changes.  These files are distributed with
 * GTK+ at ftp://ftp.gtk.org/pub/gtk/.
 */

#ifndef __GTK_MOUNT_OPERATION_H__
#define __GTK_MOUNT_OPERATION_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

G_BEGIN_DECLS

#define GTK_TYPE_MOUNT_OPERATION         (gtk_mount_operation_get_type ())
#define GTK_MOUNT_OPERATION(o)           (G_TYPE_CHECK_INSTANCE_CAST ((o), GTK_TYPE_MOUNT_OPERATION, GtkMountOperation))
#define GTK_MOUNT_OPERATION_CLASS(k)     (G_TYPE_CHECK_CLASS_CAST((k), GTK_TYPE_MOUNT_OPERATION, GtkMountOperationClass))
#define GTK_IS_MOUNT_OPERATION(o)        (G_TYPE_CHECK_INSTANCE_TYPE ((o), GTK_TYPE_MOUNT_OPERATION))
#define GTK_IS_MOUNT_OPERATION_CLASS(k)  (G_TYPE_CHECK_CLASS_TYPE ((k), GTK_TYPE_MOUNT_OPERATION))
#define GTK_MOUNT_OPERATION_GET_CLASS(o) (G_TYPE_INSTANCE_GET_CLASS ((o), GTK_TYPE_MOUNT_OPERATION, GtkMountOperationClass))

typedef struct _GtkMountOperation         GtkMountOperation;
typedef struct _GtkMountOperationClass    GtkMountOperationClass;
typedef struct _GtkMountOperationPrivate  GtkMountOperationPrivate;

/**
 * GtkMountOperation:
 *
 * This should not be accessed directly. Use the accessor functions below.
 */
struct _GtkMountOperation
{
  GMountOperation parent_instance;

  GtkMountOperationPrivate *priv;
};

/**
 * GtkMountOperationClass:
 * @parent_class: The parent class.
 */
struct _GtkMountOperationClass
{
  GMountOperationClass parent_class;

  /*< private >*/

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};


GDK_AVAILABLE_IN_ALL
GType            gtk_mount_operation_get_type   (void);
GDK_AVAILABLE_IN_ALL
GMountOperation *gtk_mount_operation_new        (GtkWindow         *parent);
GDK_AVAILABLE_IN_ALL
gboolean         gtk_mount_operation_is_showing (GtkMountOperation *op);
GDK_AVAILABLE_IN_ALL
void             gtk_mount_operation_set_parent (GtkMountOperation *op,
                                                 GtkWindow         *parent);
GDK_AVAILABLE_IN_ALL
GtkWindow *      gtk_mount_operation_get_parent (GtkMountOperation *op);
GDK_AVAILABLE_IN_ALL
void             gtk_mount_operation_set_screen (GtkMountOperation *op,
                                                 GdkScreen         *screen);
GDK_AVAILABLE_IN_ALL
GdkScreen       *gtk_mount_operation_get_screen (GtkMountOperation *op);

G_END_DECLS

#endif /* __GTK_MOUNT_OPERATION_H__ */
