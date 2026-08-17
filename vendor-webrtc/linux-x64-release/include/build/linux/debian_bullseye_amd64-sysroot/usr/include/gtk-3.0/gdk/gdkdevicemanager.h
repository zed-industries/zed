/* GDK - The GIMP Drawing Kit
 * Copyright (C) 2009 Carlos Garnacho <carlosg@gnome.org>
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

#ifndef __GDK_DEVICE_MANAGER_H__
#define __GDK_DEVICE_MANAGER_H__

#if !defined (__GDK_H_INSIDE__) && !defined (GDK_COMPILATION)
#error "Only <gdk/gdk.h> can be included directly."
#endif

#include <gdk/gdktypes.h>
#include <gdk/gdkdevice.h>

G_BEGIN_DECLS

#define GDK_TYPE_DEVICE_MANAGER         (gdk_device_manager_get_type ())
#define GDK_DEVICE_MANAGER(o)           (G_TYPE_CHECK_INSTANCE_CAST ((o), GDK_TYPE_DEVICE_MANAGER, GdkDeviceManager))
#define GDK_IS_DEVICE_MANAGER(o)        (G_TYPE_CHECK_INSTANCE_TYPE ((o), GDK_TYPE_DEVICE_MANAGER))


GDK_AVAILABLE_IN_ALL
GType        gdk_device_manager_get_type           (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
GdkDisplay * gdk_device_manager_get_display        (GdkDeviceManager *device_manager);
GDK_DEPRECATED_IN_3_20
GList *      gdk_device_manager_list_devices       (GdkDeviceManager *device_manager,
                                                    GdkDeviceType     type);
GDK_DEPRECATED_IN_3_20
GdkDevice *  gdk_device_manager_get_client_pointer (GdkDeviceManager *device_manager);

G_END_DECLS

#endif /* __GDK_DEVICE_MANAGER_H__ */
