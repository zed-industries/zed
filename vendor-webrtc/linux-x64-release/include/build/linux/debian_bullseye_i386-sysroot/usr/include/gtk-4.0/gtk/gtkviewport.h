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

#ifndef __GTK_VIEWPORT_H__
#define __GTK_VIEWPORT_H__


#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkwidget.h>


G_BEGIN_DECLS


#define GTK_TYPE_VIEWPORT            (gtk_viewport_get_type ())
#define GTK_VIEWPORT(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_VIEWPORT, GtkViewport))
#define GTK_IS_VIEWPORT(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_VIEWPORT))


typedef struct _GtkViewport              GtkViewport;


GDK_AVAILABLE_IN_ALL
GType          gtk_viewport_get_type        (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
GtkWidget*     gtk_viewport_new             (GtkAdjustment *hadjustment,
                                             GtkAdjustment *vadjustment);

GDK_AVAILABLE_IN_ALL
gboolean       gtk_viewport_get_scroll_to_focus (GtkViewport *viewport);
GDK_AVAILABLE_IN_ALL
void           gtk_viewport_set_scroll_to_focus (GtkViewport *viewport,
                                                 gboolean     scroll_to_focus);

GDK_AVAILABLE_IN_ALL
void           gtk_viewport_set_child           (GtkViewport *viewport,
                                                 GtkWidget   *child);
GDK_AVAILABLE_IN_ALL
GtkWidget *    gtk_viewport_get_child           (GtkViewport *viewport);

G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkViewport, g_object_unref)

G_END_DECLS


#endif /* __GTK_VIEWPORT_H__ */
