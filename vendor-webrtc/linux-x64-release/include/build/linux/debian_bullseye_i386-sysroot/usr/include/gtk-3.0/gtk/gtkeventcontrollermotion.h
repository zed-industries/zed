/* GTK - The GIMP Toolkit
 * Copyright (C) 2017, Red Hat, Inc.
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
 * Author(s): Matthias Clasen <mclasen@redhat.com>
 */

#ifndef __GTK_EVENT_CONTROLLER_MOTION_H__
#define __GTK_EVENT_CONTROLLER_MOTION_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gdk/gdk.h>
#include <gtk/gtkeventcontroller.h>

G_BEGIN_DECLS

#define GTK_TYPE_EVENT_CONTROLLER_MOTION         (gtk_event_controller_motion_get_type ())
#define GTK_EVENT_CONTROLLER_MOTION(o)           (G_TYPE_CHECK_INSTANCE_CAST ((o), GTK_TYPE_EVENT_CONTROLLER_MOTION, GtkEventControllerMotion))
#define GTK_EVENT_CONTROLLER_MOTION_CLASS(k)     (G_TYPE_CHECK_CLASS_CAST ((k), GTK_TYPE_EVENT_CONTROLLER_MOTION, GtkEventControllerMotionClass))
#define GTK_IS_EVENT_CONTROLLER_MOTION(o)        (G_TYPE_CHECK_INSTANCE_TYPE ((o), GTK_TYPE_EVENT_CONTROLLER_MOTION))
#define GTK_IS_EVENT_CONTROLLER_MOTION_CLASS(k)  (G_TYPE_CHECK_CLASS_TYPE ((k), GTK_TYPE_EVENT_CONTROLLER_MOTION))
#define GTK_EVENT_CONTROLLER_MOTION_GET_CLASS(o) (G_TYPE_INSTANCE_GET_CLASS ((o), GTK_TYPE_EVENT_CONTROLLER_MOTION, GtkEventControllerMotionClass))

typedef struct _GtkEventControllerMotion GtkEventControllerMotion;
typedef struct _GtkEventControllerMotionClass GtkEventControllerMotionClass;

GDK_AVAILABLE_IN_3_24
GType               gtk_event_controller_motion_get_type (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_3_24
GtkEventController *gtk_event_controller_motion_new      (GtkWidget *widget);

G_END_DECLS

#endif /* __GTK_EVENT_CONTROLLER_MOTION_H__ */
