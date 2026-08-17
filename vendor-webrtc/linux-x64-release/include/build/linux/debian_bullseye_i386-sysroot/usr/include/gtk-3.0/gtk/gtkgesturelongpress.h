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
#ifndef __GTK_GESTURE_LONG_PRESS_H__
#define __GTK_GESTURE_LONG_PRESS_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkgesturesingle.h>

G_BEGIN_DECLS

#define GTK_TYPE_GESTURE_LONG_PRESS         (gtk_gesture_long_press_get_type ())
#define GTK_GESTURE_LONG_PRESS(o)           (G_TYPE_CHECK_INSTANCE_CAST ((o), GTK_TYPE_GESTURE_LONG_PRESS, GtkGestureLongPress))
#define GTK_GESTURE_LONG_PRESS_CLASS(k)     (G_TYPE_CHECK_CLASS_CAST ((k), GTK_TYPE_GESTURE_LONG_PRESS, GtkGestureLongPressClass))
#define GTK_IS_GESTURE_LONG_PRESS(o)        (G_TYPE_CHECK_INSTANCE_TYPE ((o), GTK_TYPE_GESTURE_LONG_PRESS))
#define GTK_IS_GESTURE_LONG_PRESS_CLASS(k)  (G_TYPE_CHECK_CLASS_TYPE ((k), GTK_TYPE_GESTURE_LONG_PRESS))
#define GTK_GESTURE_LONG_PRESS_GET_CLASS(o) (G_TYPE_INSTANCE_GET_CLASS ((o), GTK_TYPE_GESTURE_LONG_PRESS, GtkGestureLongPressClass))

typedef struct _GtkGestureLongPress GtkGestureLongPress;
typedef struct _GtkGestureLongPressClass GtkGestureLongPressClass;

GDK_AVAILABLE_IN_3_14
GType        gtk_gesture_long_press_get_type   (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_3_14
GtkGesture * gtk_gesture_long_press_new        (GtkWidget *widget);

G_END_DECLS

#endif /* __GTK_GESTURE_LONG_PRESS_H__ */
