/* GTK - The GIMP Toolkit
 * Copyright (C) 2017-2018, Red Hat, Inc.
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
#ifndef __GTK_GESTURE_STYLUS_H__
#define __GTK_GESTURE_STYLUS_H__

#include <gtk/gtkgesture.h>

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

G_BEGIN_DECLS

#define GTK_TYPE_GESTURE_STYLUS         (gtk_gesture_stylus_get_type ())
#define GTK_GESTURE_STYLUS(o)           (G_TYPE_CHECK_INSTANCE_CAST ((o), GTK_TYPE_GESTURE_STYLUS, GtkGestureStylus))
#define GTK_GESTURE_STYLUS_CLASS(k)     (G_TYPE_CHECK_CLASS_CAST ((k), GTK_TYPE_GESTURE_STYLUS, GtkGestureStylusClass))
#define GTK_IS_GESTURE_STYLUS(o)        (G_TYPE_CHECK_INSTANCE_TYPE ((o), GTK_TYPE_GESTURE_STYLUS))
#define GTK_IS_GESTURE_STYLUS_CLASS(k)  (G_TYPE_CHECK_CLASS_TYPE ((k), GTK_TYPE_GESTURE_STYLUS))
#define GTK_GESTURE_STYLUS_GET_CLASS(o) (G_TYPE_INSTANCE_GET_CLASS ((o), GTK_TYPE_GESTURE_STYLUS, GtkGestureStylusClass))

typedef struct _GtkGestureStylus GtkGestureStylus;
typedef struct _GtkGestureStylusClass GtkGestureStylusClass;

GDK_AVAILABLE_IN_3_24
GType             gtk_gesture_stylus_get_type (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_3_24
GtkGesture *      gtk_gesture_stylus_new      (GtkWidget *widget);

GDK_AVAILABLE_IN_3_24
gboolean          gtk_gesture_stylus_get_axis (GtkGestureStylus *gesture,
					       GdkAxisUse        axis,
					       gdouble          *value);
GDK_AVAILABLE_IN_3_24
gboolean          gtk_gesture_stylus_get_axes (GtkGestureStylus  *gesture,
					       GdkAxisUse         axes[],
					       gdouble          **values);
GDK_AVAILABLE_IN_3_24
GdkDeviceTool *   gtk_gesture_stylus_get_device_tool (GtkGestureStylus *gesture);

G_END_DECLS

#endif /* __GTK_GESTURE_STYLUS_H__ */
