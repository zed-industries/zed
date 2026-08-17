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

#ifndef __GDK_WAYLAND_DEVICE_H__
#define __GDK_WAYLAND_DEVICE_H__

#if !defined (__GDKWAYLAND_H_INSIDE__) && !defined (GDK_COMPILATION)
#error "Only <gdk/gdkwayland.h> can be included directly."
#endif

#include <gdk/gdk.h>

#include <wayland-client.h>

G_BEGIN_DECLS

#ifdef GDK_COMPILATION
typedef struct _GdkWaylandDevice GdkWaylandDevice;
#else
typedef GdkDevice GdkWaylandDevice;
#endif
typedef struct _GdkWaylandDeviceClass GdkWaylandDeviceClass;

#define GDK_TYPE_WAYLAND_DEVICE         (gdk_wayland_device_get_type ())
#define GDK_WAYLAND_DEVICE(o)           (G_TYPE_CHECK_INSTANCE_CAST ((o), GDK_TYPE_WAYLAND_DEVICE, GdkWaylandDevice))
#define GDK_WAYLAND_DEVICE_CLASS(c)     (G_TYPE_CHECK_CLASS_CAST ((c), GDK_TYPE_WAYLAND_DEVICE, GdkWaylandDeviceClass))
#define GDK_IS_WAYLAND_DEVICE(o)        (G_TYPE_CHECK_INSTANCE_TYPE ((o), GDK_TYPE_WAYLAND_DEVICE))
#define GDK_IS_WAYLAND_DEVICE_CLASS(c)  (G_TYPE_CHECK_CLASS_TYPE ((c), GDK_TYPE_WAYLAND_DEVICE))
#define GDK_WAYLAND_DEVICE_GET_CLASS(o) (G_TYPE_INSTANCE_GET_CLASS ((o), GDK_TYPE_WAYLAND_DEVICE, GdkWaylandDeviceClass))

GDK_AVAILABLE_IN_ALL
GType                gdk_wayland_device_get_type            (void);

GDK_AVAILABLE_IN_ALL
struct wl_seat      *gdk_wayland_device_get_wl_seat         (GdkDevice *device);
GDK_AVAILABLE_IN_ALL
struct wl_pointer   *gdk_wayland_device_get_wl_pointer      (GdkDevice *device);
GDK_AVAILABLE_IN_ALL
struct wl_keyboard  *gdk_wayland_device_get_wl_keyboard     (GdkDevice *device);

GDK_AVAILABLE_IN_3_20
struct wl_seat      *gdk_wayland_seat_get_wl_seat           (GdkSeat   *seat);

GDK_AVAILABLE_IN_3_22
const gchar         *gdk_wayland_device_get_node_path       (GdkDevice *device);

GDK_AVAILABLE_IN_3_22
void                 gdk_wayland_device_pad_set_feedback (GdkDevice           *device,
                                                          GdkDevicePadFeature  element,
                                                          guint                idx,
                                                          const gchar         *label);

G_END_DECLS

#endif /* __GDK_WAYLAND_DEVICE_H__ */
