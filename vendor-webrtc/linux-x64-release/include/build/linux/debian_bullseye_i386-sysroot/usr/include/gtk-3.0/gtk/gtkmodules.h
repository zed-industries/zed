/* GTK - The GIMP Toolkit
 * Copyright 1998-2002 Tim Janik, Red Hat, Inc., and others.
 * Copyright (C) 2003 Alex Graveley
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

#ifndef __GTK_MODULES_H__
#define __GTK_MODULES_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gdk/gdk.h>

G_BEGIN_DECLS

/**
 * GtkModuleInitFunc:
 * @argc: (allow-none): GTK+ always passes %NULL for this argument
 * @argv: (allow-none) (array length=argc): GTK+ always passes %NULL for this argument
 *
 * Each GTK+ module must have a function gtk_module_init() with this prototype.
 * This function is called after loading the module.
 */
typedef void     (*GtkModuleInitFunc)        (gint        *argc,
                                              gchar      ***argv);

/**
 * GtkModuleDisplayInitFunc:
 * @display: an open #GdkDisplay
 *
 * A multihead-aware GTK+ module may have a gtk_module_display_init() function
 * with this prototype. GTK+ calls this function for each opened display.
 *
 * Since: 2.2
 */
typedef void     (*GtkModuleDisplayInitFunc) (GdkDisplay   *display);


G_END_DECLS


#endif /* __GTK_MODULES_H__ */
