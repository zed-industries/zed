/*
 * gdkmonitor.h
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

#ifndef __GDK_MONITOR_H__
#define __GDK_MONITOR_H__

#if !defined (__GDK_H_INSIDE__) && !defined (GDK_COMPILATION)
#error "Only <gdk/gdk.h> can be included directly."
#endif

#include <gdk/gdkversionmacros.h>
#include <gdk/gdkrectangle.h>
#include <gdk/gdktypes.h>

G_BEGIN_DECLS

#define GDK_TYPE_MONITOR           (gdk_monitor_get_type ())
#define GDK_MONITOR(object)        (G_TYPE_CHECK_INSTANCE_CAST ((object), GDK_TYPE_MONITOR, GdkMonitor))
#define GDK_IS_MONITOR(object)     (G_TYPE_CHECK_INSTANCE_TYPE ((object), GDK_TYPE_MONITOR))

typedef struct _GdkMonitor      GdkMonitor;
typedef struct _GdkMonitorClass GdkMonitorClass;

/**
 * GdkSubpixelLayout:
 * @GDK_SUBPIXEL_LAYOUT_UNKNOWN: The layout is not known
 * @GDK_SUBPIXEL_LAYOUT_NONE: Not organized in this way
 * @GDK_SUBPIXEL_LAYOUT_HORIZONTAL_RGB: The layout is horizontal, the order is RGB
 * @GDK_SUBPIXEL_LAYOUT_HORIZONTAL_BGR: The layout is horizontal, the order is BGR
 * @GDK_SUBPIXEL_LAYOUT_VERTICAL_RGB: The layout is vertical, the order is RGB
 * @GDK_SUBPIXEL_LAYOUT_VERTICAL_BGR: The layout is vertical, the order is BGR
 *
 * This enumeration describes how the red, green and blue components
 * of physical pixels on an output device are laid out.
 *
 * Since: 3.22
 */
typedef enum {
  GDK_SUBPIXEL_LAYOUT_UNKNOWN,
  GDK_SUBPIXEL_LAYOUT_NONE,
  GDK_SUBPIXEL_LAYOUT_HORIZONTAL_RGB,
  GDK_SUBPIXEL_LAYOUT_HORIZONTAL_BGR,
  GDK_SUBPIXEL_LAYOUT_VERTICAL_RGB,
  GDK_SUBPIXEL_LAYOUT_VERTICAL_BGR
} GdkSubpixelLayout;

GDK_AVAILABLE_IN_3_22
GType             gdk_monitor_get_type            (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_3_22
GdkDisplay  *     gdk_monitor_get_display         (GdkMonitor   *monitor);
GDK_AVAILABLE_IN_3_22
void              gdk_monitor_get_geometry        (GdkMonitor   *monitor,
                                                   GdkRectangle *geometry);
GDK_AVAILABLE_IN_3_22
void              gdk_monitor_get_workarea        (GdkMonitor   *monitor,
                                                   GdkRectangle *workarea);
GDK_AVAILABLE_IN_3_22
int               gdk_monitor_get_width_mm        (GdkMonitor   *monitor);
GDK_AVAILABLE_IN_3_22
int               gdk_monitor_get_height_mm       (GdkMonitor   *monitor);
GDK_AVAILABLE_IN_3_22
const char *      gdk_monitor_get_manufacturer    (GdkMonitor   *monitor);
GDK_AVAILABLE_IN_3_22
const char *      gdk_monitor_get_model           (GdkMonitor   *monitor);
GDK_AVAILABLE_IN_3_22
int               gdk_monitor_get_scale_factor    (GdkMonitor   *monitor);
GDK_AVAILABLE_IN_3_22
int               gdk_monitor_get_refresh_rate    (GdkMonitor   *monitor);
GDK_AVAILABLE_IN_3_22
GdkSubpixelLayout gdk_monitor_get_subpixel_layout (GdkMonitor   *monitor);
GDK_AVAILABLE_IN_3_22
gboolean          gdk_monitor_is_primary          (GdkMonitor   *monitor);

G_END_DECLS

#endif  /* __GDK_MONITOR_H__ */
