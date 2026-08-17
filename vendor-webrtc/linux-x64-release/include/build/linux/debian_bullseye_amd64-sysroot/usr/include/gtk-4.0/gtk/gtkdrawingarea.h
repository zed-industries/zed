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

#ifndef __GTK_DRAWING_AREA_H__
#define __GTK_DRAWING_AREA_H__


#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkwidget.h>


G_BEGIN_DECLS

#define GTK_TYPE_DRAWING_AREA            (gtk_drawing_area_get_type ())
#define GTK_DRAWING_AREA(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_DRAWING_AREA, GtkDrawingArea))
#define GTK_DRAWING_AREA_CLASS(klass)    (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_DRAWING_AREA, GtkDrawingAreaClass))
#define GTK_IS_DRAWING_AREA(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_DRAWING_AREA))
#define GTK_IS_DRAWING_AREA_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_DRAWING_AREA))
#define GTK_DRAWING_AREA_GET_CLASS(obj)  (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_DRAWING_AREA, GtkDrawingAreaClass))

typedef struct _GtkDrawingArea       GtkDrawingArea;
typedef struct _GtkDrawingAreaClass  GtkDrawingAreaClass;

/**
 * GtkDrawingAreaDrawFunc:
 * @drawing_area: the `GtkDrawingArea` to redraw
 * @cr: the context to draw to
 * @width: the actual width of the contents. This value will be at least
 *   as wide as GtkDrawingArea:width.
 * @height: the actual height of the contents. This value will be at least
 *   as wide as GtkDrawingArea:height.
 * @user_data: (closure): user data
 *
 * Whenever @drawing_area needs to redraw, this function will be called.
 *
 * This function should exclusively redraw the contents of the drawing area
 * and must not call any widget functions that cause changes.
 */
typedef void (* GtkDrawingAreaDrawFunc)  (GtkDrawingArea *drawing_area,
                                          cairo_t        *cr,
                                          int             width,
                                          int             height,
                                          gpointer        user_data);

struct _GtkDrawingArea
{
  GtkWidget widget;
};

struct _GtkDrawingAreaClass
{
  GtkWidgetClass parent_class;

  void           (* resize)         (GtkDrawingArea *area,
                                     int             width,
                                     int             height);

  /*< private >*/

  gpointer padding[8];
};


GDK_AVAILABLE_IN_ALL
GType      gtk_drawing_area_get_type (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
GtkWidget* gtk_drawing_area_new      (void);

GDK_AVAILABLE_IN_ALL
void            gtk_drawing_area_set_content_width      (GtkDrawingArea         *self,
                                                         int                     width);
GDK_AVAILABLE_IN_ALL
int             gtk_drawing_area_get_content_width      (GtkDrawingArea         *self);
GDK_AVAILABLE_IN_ALL
void            gtk_drawing_area_set_content_height     (GtkDrawingArea         *self,
                                                         int                     height);
GDK_AVAILABLE_IN_ALL
int             gtk_drawing_area_get_content_height     (GtkDrawingArea         *self);
GDK_AVAILABLE_IN_ALL
void            gtk_drawing_area_set_draw_func          (GtkDrawingArea         *self,
                                                         GtkDrawingAreaDrawFunc  draw_func,
                                                         gpointer                user_data,
                                                         GDestroyNotify          destroy);

G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkDrawingArea, g_object_unref)

G_END_DECLS

#endif /* __GTK_DRAWING_AREA_H__ */
