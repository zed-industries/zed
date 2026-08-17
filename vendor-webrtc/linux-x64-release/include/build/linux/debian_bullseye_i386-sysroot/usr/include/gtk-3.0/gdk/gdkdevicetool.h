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

#ifndef __GDK_DEVICE_TOOL_H__
#define __GDK_DEVICE_TOOL_H__

#if !defined (__GDK_H_INSIDE__) && !defined (GDK_COMPILATION)
#error "Only <gdk/gdk.h> can be included directly."
#endif

#include <gdk/gdkversionmacros.h>
#include <gdk/gdktypes.h>


G_BEGIN_DECLS

#define GDK_TYPE_DEVICE_TOOL    (gdk_device_tool_get_type ())
#define GDK_DEVICE_TOOL(o)      (G_TYPE_CHECK_INSTANCE_CAST ((o), GDK_TYPE_DEVICE_TOOL, GdkDeviceTool))
#define GDK_IS_DEVICE_TOOL(o)   (G_TYPE_CHECK_INSTANCE_TYPE ((o), GDK_TYPE_DEVICE_TOOL))

typedef struct _GdkDeviceTool GdkDeviceTool;

/**
 * GdkDeviceToolType:
 * @GDK_DEVICE_TOOL_TYPE_UNKNOWN: Tool is of an unknown type.
 * @GDK_DEVICE_TOOL_TYPE_PEN: Tool is a standard tablet stylus.
 * @GDK_DEVICE_TOOL_TYPE_ERASER: Tool is standard tablet eraser.
 * @GDK_DEVICE_TOOL_TYPE_BRUSH: Tool is a brush stylus.
 * @GDK_DEVICE_TOOL_TYPE_PENCIL: Tool is a pencil stylus.
 * @GDK_DEVICE_TOOL_TYPE_AIRBRUSH: Tool is an airbrush stylus.
 * @GDK_DEVICE_TOOL_TYPE_MOUSE: Tool is a mouse.
 * @GDK_DEVICE_TOOL_TYPE_LENS: Tool is a lens cursor.
 *
 * Indicates the specific type of tool being used being a tablet. Such as an
 * airbrush, pencil, etc.
 *
 * Since: 3.22
 */
typedef enum {
  GDK_DEVICE_TOOL_TYPE_UNKNOWN,
  GDK_DEVICE_TOOL_TYPE_PEN,
  GDK_DEVICE_TOOL_TYPE_ERASER,
  GDK_DEVICE_TOOL_TYPE_BRUSH,
  GDK_DEVICE_TOOL_TYPE_PENCIL,
  GDK_DEVICE_TOOL_TYPE_AIRBRUSH,
  GDK_DEVICE_TOOL_TYPE_MOUSE,
  GDK_DEVICE_TOOL_TYPE_LENS,
} GdkDeviceToolType;

GDK_AVAILABLE_IN_3_22
GType gdk_device_tool_get_type (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_3_22
guint64 gdk_device_tool_get_serial (GdkDeviceTool *tool);

GDK_AVAILABLE_IN_3_22
guint64 gdk_device_tool_get_hardware_id (GdkDeviceTool *tool);

GDK_AVAILABLE_IN_3_22
GdkDeviceToolType gdk_device_tool_get_tool_type (GdkDeviceTool *tool);

G_END_DECLS

#endif /* __GDK_DEVICE_TOOL_H__ */
