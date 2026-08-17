/*
 * gdkwaylandmonitor.h
 *
 * Copyright 2016 Red Hat, Inc.
 *
 * Matthias Clasen <mclasen@redhat.com>
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public
 * License along with this library. If not, see <http://www.gnu.org/licenses/>.
 */

#ifndef __GDK_WAYLAND_MONITOR_H__
#define __GDK_WAYLAND_MONITOR_H__

#if !defined (__GDKWAYLAND_H_INSIDE__) && !defined (GDK_COMPILATION)
#error "Only <gdk/gdkwayland.h> can be included directly."
#endif

#include <gdk/gdkmonitor.h>

G_BEGIN_DECLS

#define GDK_TYPE_WAYLAND_MONITOR           (gdk_wayland_monitor_get_type ())
#define GDK_WAYLAND_MONITOR(object)        (G_TYPE_CHECK_INSTANCE_CAST ((object), GDK_TYPE_WAYLAND_MONITOR, GdkWaylandMonitor))
#define GDK_IS_WAYLAND_MONITOR(object)     (G_TYPE_CHECK_INSTANCE_TYPE ((object), GDK_TYPE_WAYLAND_MONITOR))

typedef struct _GdkWaylandMonitor      GdkWaylandMonitor;
typedef struct _GdkWaylandMonitorClass GdkWaylandMonitorClass;

GDK_AVAILABLE_IN_3_22
GType             gdk_wayland_monitor_get_type            (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_3_22
struct wl_output *gdk_wayland_monitor_get_wl_output       (GdkMonitor *monitor);

G_END_DECLS

#endif  /* __GDK_WAYLAND_MONITOR_H__ */
