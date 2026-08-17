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

#ifndef __GTK_FRAME_H__
#define __GTK_FRAME_H__


#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkwidget.h>


G_BEGIN_DECLS


#define GTK_TYPE_FRAME                  (gtk_frame_get_type ())
#define GTK_FRAME(obj)                  (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_FRAME, GtkFrame))
#define GTK_FRAME_CLASS(klass)          (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_FRAME, GtkFrameClass))
#define GTK_IS_FRAME(obj)               (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_FRAME))
#define GTK_IS_FRAME_CLASS(klass)       (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_FRAME))
#define GTK_FRAME_GET_CLASS(obj)        (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_FRAME, GtkFrameClass))

typedef struct _GtkFrame              GtkFrame;
typedef struct _GtkFrameClass         GtkFrameClass;

struct _GtkFrame
{
  GtkWidget parent_instance;
};

/**
 * GtkFrameClass:
 * @parent_class: The parent class.
 * @compute_child_allocation:
 */
struct _GtkFrameClass
{
  GtkWidgetClass parent_class;

  /*< public >*/

  void (*compute_child_allocation) (GtkFrame *frame,
                                    GtkAllocation *allocation);

  /*< private >*/

  gpointer padding[8];
};


GDK_AVAILABLE_IN_ALL
GType      gtk_frame_get_type         (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
GtkWidget* gtk_frame_new              (const char    *label);

GDK_AVAILABLE_IN_ALL
void          gtk_frame_set_label (GtkFrame    *frame,
                                   const char *label);
GDK_AVAILABLE_IN_ALL
const char * gtk_frame_get_label (GtkFrame    *frame);

GDK_AVAILABLE_IN_ALL
void       gtk_frame_set_label_widget (GtkFrame      *frame,
                                       GtkWidget     *label_widget);
GDK_AVAILABLE_IN_ALL
GtkWidget *gtk_frame_get_label_widget (GtkFrame      *frame);
GDK_AVAILABLE_IN_ALL
void       gtk_frame_set_label_align  (GtkFrame      *frame,
                                       float          xalign);
GDK_AVAILABLE_IN_ALL
float      gtk_frame_get_label_align  (GtkFrame      *frame);

GDK_AVAILABLE_IN_ALL
void       gtk_frame_set_child        (GtkFrame      *frame,
                                       GtkWidget     *child);
GDK_AVAILABLE_IN_ALL
GtkWidget *gtk_frame_get_child        (GtkFrame      *frame);

G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkFrame, g_object_unref)

G_END_DECLS


#endif /* __GTK_FRAME_H__ */
