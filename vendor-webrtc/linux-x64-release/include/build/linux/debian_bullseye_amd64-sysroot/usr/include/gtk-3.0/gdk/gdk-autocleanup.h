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

#if !defined (__GDK_H_INSIDE__) && !defined (GDK_COMPILATION)
#error "Only <gdk/gdk.h> can be included directly."
#endif

#ifndef __GI_SCANNER__

G_DEFINE_AUTOPTR_CLEANUP_FUNC(GdkAppLaunchContext, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GdkCursor, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GdkDevice, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GdkDeviceManager, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GdkDisplay, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GdkDisplayManager, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GdkDragContext, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GdkFrameClock, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GdkGLContext, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GdkKeymap, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GdkScreen, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GdkVisual, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GdkWindow, g_object_unref)

G_DEFINE_AUTOPTR_CLEANUP_FUNC(GdkEvent, gdk_event_free)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GdkFrameTimings, gdk_frame_timings_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GdkRGBA, gdk_rgba_free)

#endif
