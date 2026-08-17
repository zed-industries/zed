/* -*- Mode: C; c-file-style: "gnu"; tab-width: 8 -*- */
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

#ifndef __GTK_DRAG_SOURCE_H__
#define __GTK_DRAG_SOURCE_H__


#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkwidget.h>


G_BEGIN_DECLS

#define GTK_TYPE_DRAG_SOURCE         (gtk_drag_source_get_type ())
#define GTK_DRAG_SOURCE(o)           (G_TYPE_CHECK_INSTANCE_CAST ((o), GTK_TYPE_DRAG_SOURCE, GtkDragSource))
#define GTK_DRAG_SOURCE_CLASS(k)     (G_TYPE_CHECK_CLASS_CAST ((k), GTK_TYPE_DRAG_SOURCE, GtkDragSourceClass))
#define GTK_IS_DRAG_SOURCE(o)        (G_TYPE_CHECK_INSTANCE_TYPE ((o), GTK_TYPE_DRAG_SOURCE))
#define GTK_IS_DRAG_SOURCE_CLASS(k)  (G_TYPE_CHECK_CLASS_TYPE ((k), GTK_TYPE_DRAG_SOURCE))
#define GTK_DRAG_SOURCE_GET_CLASS(o) (G_TYPE_INSTANCE_GET_CLASS ((o), GTK_TYPE_DRAG_SOURCE, GtkDragSourceClass))

typedef struct _GtkDragSource GtkDragSource;
typedef struct _GtkDragSourceClass GtkDragSourceClass;

GDK_AVAILABLE_IN_ALL
GType              gtk_drag_source_get_type  (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
GtkDragSource      *gtk_drag_source_new        (void);

GDK_AVAILABLE_IN_ALL
void                gtk_drag_source_set_content (GtkDragSource     *source,
                                                 GdkContentProvider *content);
GDK_AVAILABLE_IN_ALL
GdkContentProvider *gtk_drag_source_get_content (GtkDragSource     *source);

GDK_AVAILABLE_IN_ALL
void               gtk_drag_source_set_actions (GtkDragSource     *source,
                                                GdkDragAction      actions);
GDK_AVAILABLE_IN_ALL
GdkDragAction      gtk_drag_source_get_actions (GtkDragSource     *source);

GDK_AVAILABLE_IN_ALL
void               gtk_drag_source_set_icon    (GtkDragSource     *source,
                                                GdkPaintable      *paintable,
                                                int                hot_x,
                                                int                hot_y);
GDK_AVAILABLE_IN_ALL
void               gtk_drag_source_drag_cancel (GtkDragSource     *source);

GDK_AVAILABLE_IN_ALL
GdkDrag *          gtk_drag_source_get_drag    (GtkDragSource     *source);

GDK_AVAILABLE_IN_ALL
gboolean           gtk_drag_check_threshold    (GtkWidget         *widget,
                                                int                start_x,
                                                int                start_y,
                                                int                current_x,
                                                int                current_y);


G_END_DECLS

#endif /* __GTK_DRAG_SOURCE_H__ */
