/* gdkapplaunchcontext.h - Gtk+ implementation for GAppLaunchContext
 *
 * Copyright (C) 2007 Red Hat, Inc.
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
 *
 * Author: Alexander Larsson <alexl@redhat.com>
 */

#ifndef __GDK_APP_LAUNCH_CONTEXT_H__
#define __GDK_APP_LAUNCH_CONTEXT_H__

#if !defined (__GDK_H_INSIDE__) && !defined (GDK_COMPILATION)
#error "Only <gdk/gdk.h> can be included directly."
#endif

#include <gio/gio.h>
#include <gdk/gdkversionmacros.h>
#include <gdk/gdktypes.h>
#include <gdk/gdkscreen.h>

G_BEGIN_DECLS

#define GDK_TYPE_APP_LAUNCH_CONTEXT         (gdk_app_launch_context_get_type ())
#define GDK_APP_LAUNCH_CONTEXT(o)           (G_TYPE_CHECK_INSTANCE_CAST ((o), GDK_TYPE_APP_LAUNCH_CONTEXT, GdkAppLaunchContext))
        #define GDK_IS_APP_LAUNCH_CONTEXT(o)        (G_TYPE_CHECK_INSTANCE_TYPE ((o), GDK_TYPE_APP_LAUNCH_CONTEXT))


GDK_AVAILABLE_IN_ALL
GType                gdk_app_launch_context_get_type      (void);

GDK_DEPRECATED_IN_3_0_FOR(gdk_display_get_app_launch_context)
GdkAppLaunchContext *gdk_app_launch_context_new           (void);
GDK_DEPRECATED_IN_3_0_FOR(gdk_display_get_app_launch_context)
void                 gdk_app_launch_context_set_display   (GdkAppLaunchContext *context,
                                                           GdkDisplay          *display);
GDK_AVAILABLE_IN_ALL
void                 gdk_app_launch_context_set_screen    (GdkAppLaunchContext *context,
                                                           GdkScreen           *screen);
GDK_AVAILABLE_IN_ALL
void                 gdk_app_launch_context_set_desktop   (GdkAppLaunchContext *context,
                                                           gint                 desktop);
GDK_AVAILABLE_IN_ALL
void                 gdk_app_launch_context_set_timestamp (GdkAppLaunchContext *context,
                                                           guint32              timestamp);
GDK_AVAILABLE_IN_ALL
void                 gdk_app_launch_context_set_icon      (GdkAppLaunchContext *context,
                                                           GIcon               *icon);
GDK_AVAILABLE_IN_ALL
void                 gdk_app_launch_context_set_icon_name (GdkAppLaunchContext *context,
                                                           const char          *icon_name);

G_END_DECLS

#endif /* __GDK_APP_LAUNCH_CONTEXT_H__ */
