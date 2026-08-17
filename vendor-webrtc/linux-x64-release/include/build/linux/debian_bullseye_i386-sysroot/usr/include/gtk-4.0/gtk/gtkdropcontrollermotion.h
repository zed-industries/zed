/*
 * Copyright © 2020 Benjamin Otte
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

#ifndef __GTK_DROP_CONTROLLER_MOTION_H__
#define __GTK_DROP_CONTROLLER_MOTION_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gdk/gdk.h>
#include <gtk/gtkeventcontroller.h>

G_BEGIN_DECLS

#define GTK_TYPE_DROP_CONTROLLER_MOTION         (gtk_drop_controller_motion_get_type ())
#define GTK_DROP_CONTROLLER_MOTION(o)           (G_TYPE_CHECK_INSTANCE_CAST ((o), GTK_TYPE_DROP_CONTROLLER_MOTION, GtkDropControllerMotion))
#define GTK_DROP_CONTROLLER_MOTION_CLASS(k)     (G_TYPE_CHECK_CLASS_CAST ((k), GTK_TYPE_DROP_CONTROLLER_MOTION, GtkDropControllerMotionClass))
#define GTK_IS_DROP_CONTROLLER_MOTION(o)        (G_TYPE_CHECK_INSTANCE_TYPE ((o), GTK_TYPE_DROP_CONTROLLER_MOTION))
#define GTK_IS_DROP_CONTROLLER_MOTION_CLASS(k)  (G_TYPE_CHECK_CLASS_TYPE ((k), GTK_TYPE_DROP_CONTROLLER_MOTION))
#define GTK_DROP_CONTROLLER_MOTION_GET_CLASS(o) (G_TYPE_INSTANCE_GET_CLASS ((o), GTK_TYPE_DROP_CONTROLLER_MOTION, GtkDropControllerMotionClass))

typedef struct _GtkDropControllerMotion GtkDropControllerMotion;
typedef struct _GtkDropControllerMotionClass GtkDropControllerMotionClass;

GDK_AVAILABLE_IN_ALL
GType                   gtk_drop_controller_motion_get_type             (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
GtkEventController *    gtk_drop_controller_motion_new                  (void);

GDK_AVAILABLE_IN_ALL
gboolean                gtk_drop_controller_motion_contains_pointer     (GtkDropControllerMotion        *self);
GDK_AVAILABLE_IN_ALL
GdkDrop *               gtk_drop_controller_motion_get_drop             (GtkDropControllerMotion        *self);
GDK_AVAILABLE_IN_ALL
gboolean                gtk_drop_controller_motion_is_pointer           (GtkDropControllerMotion        *self);

G_END_DECLS

#endif /* __GTK_DROP_CONTROLLER_MOTION_H__ */
