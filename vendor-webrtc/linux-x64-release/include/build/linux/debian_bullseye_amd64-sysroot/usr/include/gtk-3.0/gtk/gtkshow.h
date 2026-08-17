/*
 * GTK - The GIMP Toolkit
 * Copyright (C) 2008  Jaap Haitsma <jaap@haitsma.org>
 *
 * All rights reserved.
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

#ifndef __GTK_SHOW_H__
#define __GTK_SHOW_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkwindow.h>

G_BEGIN_DECLS

GDK_DEPRECATED_IN_3_22_FOR(gtk_show_uri_on_window)
gboolean gtk_show_uri  (GdkScreen   *screen,
                        const gchar *uri,
                        guint32      timestamp,
                        GError     **error);

GDK_AVAILABLE_IN_3_22
gboolean gtk_show_uri_on_window (GtkWindow   *parent,
                                 const char  *uri,
                                 guint32      timestamp,
                                 GError     **error);

G_END_DECLS

#endif /* __GTK_SHOW_H__ */
