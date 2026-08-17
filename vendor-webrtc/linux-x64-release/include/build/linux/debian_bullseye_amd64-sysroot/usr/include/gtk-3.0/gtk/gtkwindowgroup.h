/* GTK - The GIMP Toolkit
 * Copyright (C) 1995-1997 Peter Mattis, Spencer Kimball and Josh MacDonald
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

#ifndef __GTK_WINDOW_GROUP_H__
#define __GTK_WINDOW_GROUP_H__


#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include "gtkwindow.h"

G_BEGIN_DECLS

#define GTK_TYPE_WINDOW_GROUP             (gtk_window_group_get_type ())
#define GTK_WINDOW_GROUP(object)          (G_TYPE_CHECK_INSTANCE_CAST ((object), GTK_TYPE_WINDOW_GROUP, GtkWindowGroup))
#define GTK_WINDOW_GROUP_CLASS(klass)     (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_WINDOW_GROUP, GtkWindowGroupClass))
#define GTK_IS_WINDOW_GROUP(object)       (G_TYPE_CHECK_INSTANCE_TYPE ((object), GTK_TYPE_WINDOW_GROUP))
#define GTK_IS_WINDOW_GROUP_CLASS(klass)  (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_WINDOW_GROUP))
#define GTK_WINDOW_GROUP_GET_CLASS(obj)   (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_WINDOW_GROUP, GtkWindowGroupClass))

struct _GtkWindowGroup
{
  GObject parent_instance;

  GtkWindowGroupPrivate *priv;
};

struct _GtkWindowGroupClass
{
  GObjectClass parent_class;

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};


/* Window groups
 */
GDK_AVAILABLE_IN_ALL
GType            gtk_window_group_get_type      (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
GtkWindowGroup * gtk_window_group_new           (void);
GDK_AVAILABLE_IN_ALL
void             gtk_window_group_add_window    (GtkWindowGroup     *window_group,
                                                 GtkWindow          *window);
GDK_AVAILABLE_IN_ALL
void             gtk_window_group_remove_window (GtkWindowGroup     *window_group,
                                                 GtkWindow          *window);
GDK_AVAILABLE_IN_ALL
GList *          gtk_window_group_list_windows  (GtkWindowGroup     *window_group);

GDK_AVAILABLE_IN_ALL
GtkWidget *      gtk_window_group_get_current_grab (GtkWindowGroup *window_group);
GDK_AVAILABLE_IN_ALL
GtkWidget *      gtk_window_group_get_current_device_grab (GtkWindowGroup *window_group,
                                                           GdkDevice      *device);


G_END_DECLS

#endif /* __GTK_WINDOW_GROUP_H__ */

