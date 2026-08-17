/*
 * gdkx11monitor.h
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

#ifndef __GDK_X11_MONITOR_H__
#define __GDK_X11_MONITOR_H__

#if !defined (__GDKX_H_INSIDE__) && !defined (GDK_COMPILATION)
#error "Only <gdk/gdkx.h> can be included directly."
#endif

#include <gdk/gdkmonitor.h>

G_BEGIN_DECLS

#define GDK_TYPE_X11_MONITOR           (gdk_x11_monitor_get_type ())
#define GDK_X11_MONITOR(object)        (G_TYPE_CHECK_INSTANCE_CAST ((object), GDK_TYPE_X11_MONITOR, GdkX11Monitor))
#define GDK_IS_X11_MONITOR(object)     (G_TYPE_CHECK_INSTANCE_TYPE ((object), GDK_TYPE_X11_MONITOR))

typedef struct _GdkX11Monitor      GdkX11Monitor;
typedef struct _GdkX11MonitorClass GdkX11MonitorClass;

GDK_AVAILABLE_IN_3_22
GType             gdk_x11_monitor_get_type            (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_3_22
XID               gdk_x11_monitor_get_output          (GdkMonitor *monitor);

G_END_DECLS

#endif  /* __GDK_X11_MONITOR_H__ */
