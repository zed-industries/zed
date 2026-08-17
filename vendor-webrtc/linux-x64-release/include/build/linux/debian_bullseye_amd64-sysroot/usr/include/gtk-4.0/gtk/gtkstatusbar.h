/* GTK - The GIMP Toolkit
 * Copyright (C) 1995-1997 Peter Mattis, Spencer Kimball and Josh MacDonald
 * GtkStatusbar Copyright (C) 1998 Shawn T. Amundson
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

#ifndef __GTK_STATUSBAR_H__
#define __GTK_STATUSBAR_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkwidget.h>

G_BEGIN_DECLS

#define GTK_TYPE_STATUSBAR            (gtk_statusbar_get_type ())
#define GTK_STATUSBAR(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_STATUSBAR, GtkStatusbar))
#define GTK_IS_STATUSBAR(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_STATUSBAR))

typedef struct _GtkStatusbar GtkStatusbar;

GDK_AVAILABLE_IN_ALL
GType      gtk_statusbar_get_type     	(void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
GtkWidget* gtk_statusbar_new          	(void);
GDK_AVAILABLE_IN_ALL
guint	   gtk_statusbar_get_context_id	(GtkStatusbar *statusbar,
					 const char   *context_description);
GDK_AVAILABLE_IN_ALL
guint      gtk_statusbar_push          	(GtkStatusbar *statusbar,
					 guint	       context_id,
					 const char   *text);
GDK_AVAILABLE_IN_ALL
void       gtk_statusbar_pop          	(GtkStatusbar *statusbar,
					 guint	       context_id);
GDK_AVAILABLE_IN_ALL
void       gtk_statusbar_remove        	(GtkStatusbar *statusbar,
					 guint	       context_id,
					 guint         message_id);
GDK_AVAILABLE_IN_ALL
void       gtk_statusbar_remove_all    	(GtkStatusbar *statusbar,
					 guint	       context_id);

G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkStatusbar, g_object_unref)

G_END_DECLS

#endif /* __GTK_STATUSBAR_H__ */
