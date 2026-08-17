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
#ifndef __GTK_GESTURE_ZOOM_H__
#define __GTK_GESTURE_ZOOM_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkgesture.h>
#include <gtk/gtkwidget.h>

G_BEGIN_DECLS

#define GTK_TYPE_GESTURE_ZOOM         (gtk_gesture_zoom_get_type ())
#define GTK_GESTURE_ZOOM(o)           (G_TYPE_CHECK_INSTANCE_CAST ((o), GTK_TYPE_GESTURE_ZOOM, GtkGestureZoom))
#define GTK_GESTURE_ZOOM_CLASS(k)     (G_TYPE_CHECK_CLASS_CAST ((k), GTK_TYPE_GESTURE_ZOOM, GtkGestureZoomClass))
#define GTK_IS_GESTURE_ZOOM(o)        (G_TYPE_CHECK_INSTANCE_TYPE ((o), GTK_TYPE_GESTURE_ZOOM))
#define GTK_IS_GESTURE_ZOOM_CLASS(k)  (G_TYPE_CHECK_CLASS_TYPE ((k), GTK_TYPE_GESTURE_ZOOM))
#define GTK_GESTURE_ZOOM_GET_CLASS(o) (G_TYPE_INSTANCE_GET_CLASS ((o), GTK_TYPE_GESTURE_ZOOM, GtkGestureZoomClass))

typedef struct _GtkGestureZoom GtkGestureZoom;
typedef struct _GtkGestureZoomClass GtkGestureZoomClass;

GDK_AVAILABLE_IN_3_14
GType        gtk_gesture_zoom_get_type        (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_3_14
GtkGesture * gtk_gesture_zoom_new             (GtkWidget      *widget);

GDK_AVAILABLE_IN_3_14
gdouble      gtk_gesture_zoom_get_scale_delta (GtkGestureZoom *gesture);

G_END_DECLS

#endif /* __GTK_GESTURE_ZOOM_H__ */
