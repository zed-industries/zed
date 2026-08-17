/* GTK - The GIMP Toolkit
 * Copyright (C) 2014 Red Hat, Inc.
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
#ifndef __GTK_GESTURE_CLICK_H__
#define __GTK_GESTURE_CLICK_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkwidget.h>
#include <gtk/gtkgesturesingle.h>

G_BEGIN_DECLS

#define GTK_TYPE_GESTURE_CLICK         (gtk_gesture_click_get_type ())
#define GTK_GESTURE_CLICK(o)           (G_TYPE_CHECK_INSTANCE_CAST ((o), GTK_TYPE_GESTURE_CLICK, GtkGestureClick))
#define GTK_GESTURE_CLICK_CLASS(k)     (G_TYPE_CHECK_CLASS_CAST ((k), GTK_TYPE_GESTURE_CLICK, GtkGestureClickClass))
#define GTK_IS_GESTURE_CLICK(o)        (G_TYPE_CHECK_INSTANCE_TYPE ((o), GTK_TYPE_GESTURE_CLICK))
#define GTK_IS_GESTURE_CLICK_CLASS(k)  (G_TYPE_CHECK_CLASS_TYPE ((k), GTK_TYPE_GESTURE_CLICK))
#define GTK_GESTURE_CLICK_GET_CLASS(o) (G_TYPE_INSTANCE_GET_CLASS ((o), GTK_TYPE_GESTURE_CLICK, GtkGestureClickClass))

typedef struct _GtkGestureClick GtkGestureClick;
typedef struct _GtkGestureClickClass GtkGestureClickClass;

GDK_AVAILABLE_IN_ALL
GType        gtk_gesture_click_get_type (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
GtkGesture * gtk_gesture_click_new      (void);

G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkGestureClick, g_object_unref)

G_END_DECLS

#endif /* __GTK_GESTURE_CLICK_H__ */
