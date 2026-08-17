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

#ifndef __GDK_X11_DEVICE_MANAGER_CORE_H__
#define __GDK_X11_DEVICE_MANAGER_CORE_H__

#include <gdk/gdk.h>

G_BEGIN_DECLS

#define GDK_TYPE_X11_DEVICE_MANAGER_CORE         (gdk_x11_device_manager_core_get_type ())
#define GDK_X11_DEVICE_MANAGER_CORE(o)           (G_TYPE_CHECK_INSTANCE_CAST ((o), GDK_TYPE_X11_DEVICE_MANAGER_CORE, GdkX11DeviceManagerCore))
#define GDK_X11_DEVICE_MANAGER_CORE_CLASS(c)     (G_TYPE_CHECK_CLASS_CAST ((c), GDK_TYPE_X11_DEVICE_MANAGER_CORE, GdkX11DeviceManagerCoreClass))
#define GDK_IS_X11_DEVICE_MANAGER_CORE(o)        (G_TYPE_CHECK_INSTANCE_TYPE ((o), GDK_TYPE_X11_DEVICE_MANAGER_CORE))
#define GDK_IS_X11_DEVICE_MANAGER_CORE_CLASS(c)  (G_TYPE_CHECK_CLASS_TYPE ((c), GDK_TYPE_X11_DEVICE_MANAGER_CORE))
#define GDK_X11_DEVICE_MANAGER_CORE_GET_CLASS(o) (G_TYPE_INSTANCE_GET_CLASS ((o), GDK_TYPE_X11_DEVICE_MANAGER_CORE, GdkX11DeviceManagerCoreClass))

typedef struct _GdkX11DeviceManagerCore GdkX11DeviceManagerCore;
typedef struct _GdkX11DeviceManagerCoreClass GdkX11DeviceManagerCoreClass;


GDK_AVAILABLE_IN_ALL
GType gdk_x11_device_manager_core_get_type (void) G_GNUC_CONST;


G_END_DECLS

#endif /* __GDK_X11_DEVICE_MANAGER_CORE_H__ */
