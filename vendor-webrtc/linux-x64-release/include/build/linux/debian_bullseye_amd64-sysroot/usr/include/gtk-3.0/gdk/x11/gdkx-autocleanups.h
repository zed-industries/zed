/* GTK - The GIMP Toolkit
 * Copyright (C) 1995-1997 Peter Mattis, Spencer Kimball and Josh MacDonald
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

#if !defined (__GDKX_H_INSIDE__) && !defined (GDK_COMPILATION)
#error "Only <gdk/gdkx.h> can be included directly."
#endif

#ifndef __GI_SCANNER__

G_DEFINE_AUTOPTR_CLEANUP_FUNC(GdkX11AppLaunchContext, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GdkX11Cursor, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GdkX11DeviceCore, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GdkX11DeviceManagerCore, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GdkX11DeviceManagerXI2, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GdkX11DeviceXI2, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GdkX11Display, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GdkX11DisplayManager, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GdkX11DragContext, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GdkX11GLContext, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GdkX11Keymap, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GdkX11Screen, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GdkX11Visual, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GdkX11Window, g_object_unref)

#endif
