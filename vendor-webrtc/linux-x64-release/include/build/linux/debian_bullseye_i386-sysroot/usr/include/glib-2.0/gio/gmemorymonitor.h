/* GIO - GLib Input, Output and Streaming Library
 *
 * Copyright 2019 Red Hat, Inc.
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
 * You should have received a copy of the GNU Lesser General
 * Public License along with this library; if not, see <http://www.gnu.org/licenses/>.
 */

#ifndef __G_MEMORY_MONITOR_H__
#define __G_MEMORY_MONITOR_H__

#if !defined (__GIO_GIO_H_INSIDE__) && !defined (GIO_COMPILATION)
#error "Only <gio/gio.h> can be included directly."
#endif

#include <gio/giotypes.h>

G_BEGIN_DECLS

/**
 * G_MEMORY_MONITOR_EXTENSION_POINT_NAME:
 *
 * Extension point for memory usage monitoring functionality.
 * See [Extending GIO][extending-gio].
 *
 * Since: 2.64
 */
#define G_MEMORY_MONITOR_EXTENSION_POINT_NAME "gio-memory-monitor"

#define G_TYPE_MEMORY_MONITOR             (g_memory_monitor_get_type ())
GLIB_AVAILABLE_IN_2_64
G_DECLARE_INTERFACE(GMemoryMonitor, g_memory_monitor, g, memory_monitor, GObject)

#define G_MEMORY_MONITOR(o)               (G_TYPE_CHECK_INSTANCE_CAST ((o), G_TYPE_MEMORY_MONITOR, GMemoryMonitor))
#define G_IS_MEMORY_MONITOR(o)            (G_TYPE_CHECK_INSTANCE_TYPE ((o), G_TYPE_MEMORY_MONITOR))
#define G_MEMORY_MONITOR_GET_INTERFACE(o) (G_TYPE_INSTANCE_GET_INTERFACE ((o), G_TYPE_MEMORY_MONITOR, GMemoryMonitorInterface))

struct _GMemoryMonitorInterface {
  /*< private >*/
  GTypeInterface g_iface;

  /*< public >*/
  void     (*low_memory_warning)  (GMemoryMonitor             *monitor,
                                   GMemoryMonitorWarningLevel  level);
};

GLIB_AVAILABLE_IN_2_64
GMemoryMonitor      *g_memory_monitor_dup_default           (void);

G_END_DECLS

#endif /* __G_MEMORY_MONITOR_H__ */
