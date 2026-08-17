/* GDK - The GIMP Drawing Kit
 * Copyright (C) 2000 Red Hat, Inc.
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

#ifndef __GDK_DISPLAY_MANAGER_H__
#define __GDK_DISPLAY_MANAGER_H__

#if !defined (__GDK_H_INSIDE__) && !defined (GDK_COMPILATION)
#error "Only <gdk/gdk.h> can be included directly."
#endif

#include <gdk/gdktypes.h>
#include <gdk/gdkdisplay.h>

G_BEGIN_DECLS


#define GDK_TYPE_DISPLAY_MANAGER              (gdk_display_manager_get_type ())
#define GDK_DISPLAY_MANAGER(object)           (G_TYPE_CHECK_INSTANCE_CAST ((object), GDK_TYPE_DISPLAY_MANAGER, GdkDisplayManager))
#define GDK_IS_DISPLAY_MANAGER(object)        (G_TYPE_CHECK_INSTANCE_TYPE ((object), GDK_TYPE_DISPLAY_MANAGER))


GDK_AVAILABLE_IN_ALL
GType              gdk_display_manager_get_type            (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
GdkDisplayManager *gdk_display_manager_get                 (void);
GDK_AVAILABLE_IN_ALL
GdkDisplay *       gdk_display_manager_get_default_display (GdkDisplayManager *manager);
GDK_AVAILABLE_IN_ALL
void               gdk_display_manager_set_default_display (GdkDisplayManager *manager,
                                                            GdkDisplay        *display);
GDK_AVAILABLE_IN_ALL
GSList *           gdk_display_manager_list_displays       (GdkDisplayManager *manager);
GDK_AVAILABLE_IN_ALL
GdkDisplay *       gdk_display_manager_open_display        (GdkDisplayManager *manager,
                                                            const gchar       *name);

G_END_DECLS

#endif /* __GDK_DISPLAY_MANAGER_H__ */
