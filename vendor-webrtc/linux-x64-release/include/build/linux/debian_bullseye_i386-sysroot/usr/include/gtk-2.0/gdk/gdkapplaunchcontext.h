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
 * License along with this library; if not, write to the
 * Free Software Foundation, Inc., 59 Temple Place - Suite 330,
 * Boston, MA 02111-1307, USA.
 *
 * Author: Alexander Larsson <alexl@redhat.com>
 */

#ifndef __GDK_APP_LAUNCH_CONTEXT_H__
#define __GDK_APP_LAUNCH_CONTEXT_H__

#if !defined (__GDK_H_INSIDE__) && !defined (GDK_COMPILATION)
#error "Only <gdk/gdk.h> can be included directly."
#endif

#include <gio/gio.h>
#include <gdk/gdkscreen.h>

G_BEGIN_DECLS

#define GDK_TYPE_APP_LAUNCH_CONTEXT         (gdk_app_launch_context_get_type ())
#define GDK_APP_LAUNCH_CONTEXT(o)           (G_TYPE_CHECK_INSTANCE_CAST ((o), GDK_TYPE_APP_LAUNCH_CONTEXT, GdkAppLaunchContext))
#define GDK_APP_LAUNCH_CONTEXT_CLASS(k)     (G_TYPE_CHECK_CLASS_CAST((k), GDK_TYPE_APP_LAUNCH_CONTEXT, GdkAppLaunchContextClass))
#define GDK_IS_APP_LAUNCH_CONTEXT(o)        (G_TYPE_CHECK_INSTANCE_TYPE ((o), GDK_TYPE_APP_LAUNCH_CONTEXT))
#define GDK_IS_APP_LAUNCH_CONTEXT_CLASS(k)  (G_TYPE_CHECK_CLASS_TYPE ((k), GDK_TYPE_APP_LAUNCH_CONTEXT))
#define GDK_APP_LAUNCH_CONTEXT_GET_CLASS(o) (G_TYPE_INSTANCE_GET_CLASS ((o), GDK_TYPE_APP_LAUNCH_CONTEXT, GdkAppLaunchContextClass))

typedef struct GdkAppLaunchContext	      GdkAppLaunchContext;
typedef struct GdkAppLaunchContextClass       GdkAppLaunchContextClass;
typedef struct GdkAppLaunchContextPrivate     GdkAppLaunchContextPrivate;

struct GdkAppLaunchContext
{
  GAppLaunchContext parent_instance;

  GdkAppLaunchContextPrivate *priv;
};

struct GdkAppLaunchContextClass
{
  GAppLaunchContextClass parent_class;
};

GType                gdk_app_launch_context_get_type      (void);

GdkAppLaunchContext *gdk_app_launch_context_new           (void);
void                 gdk_app_launch_context_set_display   (GdkAppLaunchContext *context,
							   GdkDisplay          *display);
void                 gdk_app_launch_context_set_screen    (GdkAppLaunchContext *context,
							   GdkScreen           *screen);
void                 gdk_app_launch_context_set_desktop   (GdkAppLaunchContext *context,
							   gint                 desktop);
void                 gdk_app_launch_context_set_timestamp (GdkAppLaunchContext *context,
							   guint32              timestamp);
void                 gdk_app_launch_context_set_icon      (GdkAppLaunchContext *context,
							   GIcon               *icon);
void                 gdk_app_launch_context_set_icon_name (GdkAppLaunchContext *context,
							   const char          *icon_name);

G_END_DECLS

#endif /* __GDK_APP_LAUNCH_CONTEXT_H__ */
