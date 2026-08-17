/* GDK - The GIMP Drawing Kit
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

#ifndef __GDK_WAYLAND_SELECTION_H__
#define __GDK_WAYLAND_SELECTION_H__

#if !defined (__GDKWAYLAND_H_INSIDE__) && !defined (GDK_COMPILATION)
#error "Only <gdk/gdkwayland.h> can be included directly."
#endif

#include <gdk/gdk.h>

G_BEGIN_DECLS

#if defined (GTK_COMPILATION) || defined (GDK_COMPILATION)
#define gdk_wayland_selection_add_targets gdk_wayland_selection_add_targets_libgtk_only
GDK_AVAILABLE_IN_ALL
void
gdk_wayland_selection_add_targets (GdkWindow *window,
                                   GdkAtom    selection,
                                   guint      ntargets,
                                   GdkAtom   *targets);

#define gdk_wayland_selection_clear_targets gdk_wayland_selection_clear_targets_libgtk_only
GDK_AVAILABLE_IN_ALL
void
gdk_wayland_selection_clear_targets (GdkDisplay *display, GdkAtom selection);

#endif

G_END_DECLS

#endif /* __GDK_WAYLAND_SELECTION_H__ */
