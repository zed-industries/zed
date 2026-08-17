/* gdkwaylandseat.h: Wayland GdkSeat
 *
 * Copyright 2020  GNOME Foundation
 *
 * SPDX-License-Identifier: LGPL-2.1-or-later
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library; if not, see <http://www.gnu.org/licenses/>.
 */
#pragma once

#if !defined (__GDKWAYLAND_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gdk/wayland/gdkwayland.h> can be included directly."
#endif

#include <gdk/gdk.h>
#include <wayland-client.h>

G_BEGIN_DECLS

#ifdef GTK_COMPILATION
typedef struct _GdkWaylandSeat GdkWaylandSeat;
#else
typedef GdkSeat GdkWaylandSeat;
#endif

typedef struct _GdkWaylandSeatClass GdkWaylandSeatClass;

#define GDK_TYPE_WAYLAND_SEAT         (gdk_wayland_seat_get_type ())
#define GDK_WAYLAND_SEAT(o)           (G_TYPE_CHECK_INSTANCE_CAST ((o), GDK_TYPE_WAYLAND_SEAT, GdkWaylandSeat))
#define GDK_IS_WAYLAND_SEAT(o)        (G_TYPE_CHECK_INSTANCE_TYPE ((o), GDK_TYPE_WAYLAND_SEAT))

GDK_AVAILABLE_IN_ALL
GType gdk_wayland_seat_get_type (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
struct wl_seat *        gdk_wayland_seat_get_wl_seat    (GdkSeat *seat);

G_END_DECLS
