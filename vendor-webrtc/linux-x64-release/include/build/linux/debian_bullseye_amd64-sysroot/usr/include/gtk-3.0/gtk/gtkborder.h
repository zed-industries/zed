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
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.	 See the GNU
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

#ifndef __GTK_BORDER_H__
#define __GTK_BORDER_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <glib-object.h>
#include <gdk/gdk.h>

G_BEGIN_DECLS

typedef struct _GtkBorder GtkBorder;

#define GTK_TYPE_BORDER (gtk_border_get_type ())

/**
 * GtkBorder:
 * @left: The width of the left border
 * @right: The width of the right border
 * @top: The width of the top border
 * @bottom: The width of the bottom border
 *
 * A struct that specifies a border around a rectangular area
 * that can be of different width on each side.
 */
struct _GtkBorder
{
  gint16 left;
  gint16 right;
  gint16 top;
  gint16 bottom;
};

GDK_AVAILABLE_IN_ALL
GType      gtk_border_get_type (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
GtkBorder *gtk_border_new      (void) G_GNUC_MALLOC;
GDK_AVAILABLE_IN_ALL
GtkBorder *gtk_border_copy     (const GtkBorder *border_);
GDK_AVAILABLE_IN_ALL
void       gtk_border_free     (GtkBorder       *border_);

G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkBorder, gtk_border_free)

G_END_DECLS

#endif /* __GTK_BORDER_H__ */
