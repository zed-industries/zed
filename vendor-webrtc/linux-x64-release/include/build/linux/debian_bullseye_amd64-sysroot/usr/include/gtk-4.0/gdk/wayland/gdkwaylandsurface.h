/* GDK - The GIMP Drawing Kit
 * Copyright (C) 2013 Jan Arne Petersen
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

#ifndef __GDK_WAYLAND_SURFACE_H__
#define __GDK_WAYLAND_SURFACE_H__

#if !defined (__GDKWAYLAND_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gdk/wayland/gdkwayland.h> can be included directly."
#endif

#include <gdk/gdk.h>

#include <wayland-client.h>

G_BEGIN_DECLS

#ifdef GTK_COMPILATION
typedef struct _GdkWaylandSurface GdkWaylandSurface;
typedef struct _GdkWaylandToplevel GdkWaylandToplevel;
typedef struct _GdkWaylandPopup GdkWaylandPopup;
#else
typedef GdkSurface GdkWaylandSurface;
typedef GdkToplevel GdkWaylandToplevel;
typedef GdkPopup GdkWaylandPopup;
#endif

#define GDK_TYPE_WAYLAND_SURFACE              (gdk_wayland_surface_get_type())
#define GDK_WAYLAND_SURFACE(object)           (G_TYPE_CHECK_INSTANCE_CAST ((object), GDK_TYPE_WAYLAND_SURFACE, GdkWaylandSurface))
#define GDK_IS_WAYLAND_SURFACE(object)        (G_TYPE_CHECK_INSTANCE_TYPE ((object), GDK_TYPE_WAYLAND_SURFACE))

#define GDK_TYPE_WAYLAND_TOPLEVEL             (gdk_wayland_toplevel_get_type())
#define GDK_WAYLAND_TOPLEVEL(object)          (G_TYPE_CHECK_INSTANCE_CAST ((object), GDK_TYPE_WAYLAND_TOPLEVEL, GdkWaylandToplevel))
#define GDK_IS_WAYLAND_TOPLEVEL(object)       (G_TYPE_CHECK_INSTANCE_TYPE ((object), GDK_TYPE_WAYLAND_TOPLEVEL))

#define GDK_TYPE_WAYLAND_POPUP                (gdk_wayland_popup_get_type())
#define GDK_WAYLAND_POPUP(object)             (G_TYPE_CHECK_INSTANCE_CAST ((object), GDK_TYPE_WAYLAND_POPUP, GdkWaylandPopup))
#define GDK_IS_WAYLAND_POPUP(object)          (G_TYPE_CHECK_INSTANCE_TYPE ((object), GDK_TYPE_WAYLAND_POPUP))

GDK_AVAILABLE_IN_ALL
GType                    gdk_wayland_surface_get_type             (void);

GDK_AVAILABLE_IN_ALL
GType                    gdk_wayland_toplevel_get_type            (void);

GDK_AVAILABLE_IN_ALL
GType                    gdk_wayland_popup_get_type               (void);

GDK_AVAILABLE_IN_ALL
struct wl_surface       *gdk_wayland_surface_get_wl_surface       (GdkSurface *surface);

typedef void (*GdkWaylandToplevelExported) (GdkToplevel *toplevel,
                                            const char  *handle,
                                            gpointer     user_data);

GDK_AVAILABLE_IN_ALL
gboolean                 gdk_wayland_toplevel_export_handle (GdkToplevel              *toplevel,
                                                             GdkWaylandToplevelExported callback,
                                                             gpointer                 user_data,
                                                             GDestroyNotify           destroy_func);

GDK_AVAILABLE_IN_ALL
void                     gdk_wayland_toplevel_unexport_handle (GdkToplevel *toplevel);

GDK_AVAILABLE_IN_ALL
gboolean                 gdk_wayland_toplevel_set_transient_for_exported (GdkToplevel *toplevel,
                                                                         const char   *parent_handle_str);

GDK_AVAILABLE_IN_ALL
void                     gdk_wayland_toplevel_set_application_id (GdkToplevel *toplevel,
                                                                  const char  *application_id);

G_END_DECLS

#endif /* __GDK_WAYLAND_SURFACE_H__ */
