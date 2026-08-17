/* GTK - The GIMP Toolkit
 * Copyright (C) 2012, One Laptop Per Child.
 * Copyright (C) 2014, Red Hat, Inc.
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
 * Author(s): Carlos Garnacho <carlosg@gnome.org>
 */
#ifndef __GTK_GESTURE_ROTATE_H__
#define __GTK_GESTURE_ROTATE_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkwidget.h>
#include <gtk/gtkgesture.h>

G_BEGIN_DECLS

#define GTK_TYPE_GESTURE_ROTATE         (gtk_gesture_rotate_get_type ())
#define GTK_GESTURE_ROTATE(o)           (G_TYPE_CHECK_INSTANCE_CAST ((o), GTK_TYPE_GESTURE_ROTATE, GtkGestureRotate))
#define GTK_GESTURE_ROTATE_CLASS(k)     (G_TYPE_CHECK_CLASS_CAST ((k), GTK_TYPE_GESTURE_ROTATE, GtkGestureRotateClass))
#define GTK_IS_GESTURE_ROTATE(o)        (G_TYPE_CHECK_INSTANCE_TYPE ((o), GTK_TYPE_GESTURE_ROTATE))
#define GTK_IS_GESTURE_ROTATE_CLASS(k)  (G_TYPE_CHECK_CLASS_TYPE ((k), GTK_TYPE_GESTURE_ROTATE))
#define GTK_GESTURE_ROTATE_GET_CLASS(o) (G_TYPE_INSTANCE_GET_CLASS ((o), GTK_TYPE_GESTURE_ROTATE, GtkGestureRotateClass))

typedef struct _GtkGestureRotate GtkGestureRotate;
typedef struct _GtkGestureRotateClass GtkGestureRotateClass;

GDK_AVAILABLE_IN_3_14
GType        gtk_gesture_rotate_get_type        (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_3_14
GtkGesture * gtk_gesture_rotate_new             (GtkWidget        *widget);

GDK_AVAILABLE_IN_3_14
gdouble      gtk_gesture_rotate_get_angle_delta (GtkGestureRotate *gesture);

G_END_DECLS

#endif /* __GTK_GESTURE_ROTATE_H__ */
