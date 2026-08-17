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

#ifndef __GTK_PANED_H__
#define __GTK_PANED_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkwidget.h>

G_BEGIN_DECLS

#define GTK_TYPE_PANED                  (gtk_paned_get_type ())
#define GTK_PANED(obj)                  (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_PANED, GtkPaned))
#define GTK_IS_PANED(obj)               (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_PANED))

typedef struct _GtkPaned GtkPaned;

GDK_AVAILABLE_IN_ALL
GType       gtk_paned_get_type     (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
GtkWidget * gtk_paned_new          (GtkOrientation orientation);
GDK_AVAILABLE_IN_ALL
void        gtk_paned_set_start_child (GtkPaned       *paned,
                                       GtkWidget      *child);
GDK_AVAILABLE_IN_ALL
GtkWidget * gtk_paned_get_start_child (GtkPaned       *paned);
GDK_AVAILABLE_IN_ALL
void        gtk_paned_set_resize_start_child (GtkPaned *paned,
                                              gboolean  resize);
GDK_AVAILABLE_IN_ALL
gboolean    gtk_paned_get_resize_start_child (GtkPaned *paned);

GDK_AVAILABLE_IN_ALL
void        gtk_paned_set_end_child   (GtkPaned       *paned,
                                       GtkWidget      *child);
GDK_AVAILABLE_IN_ALL
GtkWidget * gtk_paned_get_end_child   (GtkPaned       *paned);

GDK_AVAILABLE_IN_ALL
void        gtk_paned_set_shrink_start_child (GtkPaned *paned,
                                              gboolean  resize);
GDK_AVAILABLE_IN_ALL
gboolean    gtk_paned_get_shrink_start_child (GtkPaned *paned);

GDK_AVAILABLE_IN_ALL
void        gtk_paned_set_resize_end_child (GtkPaned *paned,
                                              gboolean  resize);
GDK_AVAILABLE_IN_ALL
gboolean    gtk_paned_get_resize_end_child (GtkPaned *paned);

GDK_AVAILABLE_IN_ALL
void        gtk_paned_set_shrink_end_child (GtkPaned *paned,
                                              gboolean  resize);
GDK_AVAILABLE_IN_ALL
gboolean    gtk_paned_get_shrink_end_child (GtkPaned *paned);

GDK_AVAILABLE_IN_ALL
int         gtk_paned_get_position (GtkPaned       *paned);
GDK_AVAILABLE_IN_ALL
void        gtk_paned_set_position (GtkPaned       *paned,
                                    int             position);

GDK_AVAILABLE_IN_ALL
void        gtk_paned_set_wide_handle (GtkPaned    *paned,
                                       gboolean     wide);
GDK_AVAILABLE_IN_ALL
gboolean    gtk_paned_get_wide_handle (GtkPaned    *paned);

G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkPaned, g_object_unref)

G_END_DECLS

#endif /* __GTK_PANED_H__ */
