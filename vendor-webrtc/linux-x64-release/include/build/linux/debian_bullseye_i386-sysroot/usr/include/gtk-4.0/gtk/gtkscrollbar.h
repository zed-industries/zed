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

#ifndef __GTK_SCROLLBAR_H__
#define __GTK_SCROLLBAR_H__


#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkwidget.h>


G_BEGIN_DECLS

#define GTK_TYPE_SCROLLBAR            (gtk_scrollbar_get_type ())
#define GTK_SCROLLBAR(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_SCROLLBAR, GtkScrollbar))
#define GTK_IS_SCROLLBAR(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_SCROLLBAR))


typedef struct _GtkScrollbar        GtkScrollbar;


GDK_AVAILABLE_IN_ALL
GType       gtk_scrollbar_get_type (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
GtkWidget * gtk_scrollbar_new      (GtkOrientation  orientation,
                                    GtkAdjustment  *adjustment);
GDK_AVAILABLE_IN_ALL
void           gtk_scrollbar_set_adjustment (GtkScrollbar  *self,
                                             GtkAdjustment *adjustment);
GDK_AVAILABLE_IN_ALL
GtkAdjustment *gtk_scrollbar_get_adjustment (GtkScrollbar  *self);

G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkScrollbar, g_object_unref)

G_END_DECLS

#endif /* __GTK_SCROLLBAR_H__ */
