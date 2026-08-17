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

#ifndef __GDK_DEVICE_H__
#define __GDK_DEVICE_H__

#if !defined (__GDK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gdk/gdk.h> can be included directly."
#endif

#include <gdk/gdkversionmacros.h>
#include <gdk/gdktypes.h>
#include <gdk/gdkdevicetool.h>
#include <gdk/gdkenums.h>


G_BEGIN_DECLS

#define GDK_TYPE_DEVICE         (gdk_device_get_type ())
#define GDK_DEVICE(o)           (G_TYPE_CHECK_INSTANCE_CAST ((o), GDK_TYPE_DEVICE, GdkDevice))
#define GDK_IS_DEVICE(o)        (G_TYPE_CHECK_INSTANCE_TYPE ((o), GDK_TYPE_DEVICE))

typedef struct _GdkTimeCoord GdkTimeCoord;

/**
 * GdkInputSource:
 * @GDK_SOURCE_MOUSE: the device is a mouse. (This will be reported for the core
 *   pointer, even if it is something else, such as a trackball.)
 * @GDK_SOURCE_PEN: the device is a stylus of a graphics tablet or similar device.
 * @GDK_SOURCE_KEYBOARD: the device is a keyboard.
 * @GDK_SOURCE_TOUCHSCREEN: the device is a direct-input touch device, such
 *   as a touchscreen or tablet
 * @GDK_SOURCE_TOUCHPAD: the device is an indirect touch device, such
 *   as a touchpad
 * @GDK_SOURCE_TRACKPOINT: the device is a trackpoint
 * @GDK_SOURCE_TABLET_PAD: the device is a "pad", a collection of buttons,
 *   rings and strips found in drawing tablets
 *
 * An enumeration describing the type of an input device in general terms.
 */
typedef enum
{
  GDK_SOURCE_MOUSE,
  GDK_SOURCE_PEN,
  GDK_SOURCE_KEYBOARD,
  GDK_SOURCE_TOUCHSCREEN,
  GDK_SOURCE_TOUCHPAD,
  GDK_SOURCE_TRACKPOINT,
  GDK_SOURCE_TABLET_PAD
} GdkInputSource;

/**
 * GdkTimeCoord:
 * @time: The timestamp for this event
 * @flags: Flags indicating what axes are present, see [flags@Gdk.AxisFlags]
 * @axes: (array fixed-size=12): axis values, indexed by [enum@Gdk.AxisUse]
 *
 * A `GdkTimeCoord` stores a single event in a motion history.
 *
 * To check whether an axis is present, check whether the corresponding
 * flag from the [flags@Gdk.AxisFlags] enumeration is set in the @flags
 * To access individual axis values, use the values of the values of
 * the [enum@Gdk.AxisUse] enumerations as indices.
 */
struct _GdkTimeCoord
{
  guint32 time;
  GdkAxisFlags flags;
  double axes[GDK_AXIS_LAST];
};

GDK_AVAILABLE_IN_ALL
GType               gdk_device_get_type                 (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
const char *        gdk_device_get_name                 (GdkDevice *device);
GDK_AVAILABLE_IN_ALL
const char *        gdk_device_get_vendor_id            (GdkDevice *device);
GDK_AVAILABLE_IN_ALL
const char *        gdk_device_get_product_id           (GdkDevice *device);

GDK_AVAILABLE_IN_ALL
GdkDisplay *        gdk_device_get_display              (GdkDevice *device);
GDK_AVAILABLE_IN_ALL
GdkSeat *           gdk_device_get_seat                 (GdkDevice *device);
GDK_AVAILABLE_IN_ALL
GdkDeviceTool *     gdk_device_get_device_tool          (GdkDevice *device);

GDK_AVAILABLE_IN_ALL
GdkInputSource      gdk_device_get_source               (GdkDevice *device);
GDK_AVAILABLE_IN_ALL
gboolean            gdk_device_get_has_cursor           (GdkDevice *device);
GDK_AVAILABLE_IN_ALL
guint               gdk_device_get_num_touches          (GdkDevice *device);
GDK_AVAILABLE_IN_ALL
GdkModifierType     gdk_device_get_modifier_state       (GdkDevice *device);
GDK_AVAILABLE_IN_ALL
PangoDirection      gdk_device_get_direction            (GdkDevice *device);
GDK_AVAILABLE_IN_ALL
gboolean            gdk_device_has_bidi_layouts         (GdkDevice *device);
GDK_AVAILABLE_IN_ALL
gboolean            gdk_device_get_caps_lock_state      (GdkDevice *device);
GDK_AVAILABLE_IN_ALL
gboolean            gdk_device_get_num_lock_state       (GdkDevice *device);
GDK_AVAILABLE_IN_ALL
gboolean            gdk_device_get_scroll_lock_state    (GdkDevice *device);

GDK_AVAILABLE_IN_ALL
GdkSurface *        gdk_device_get_surface_at_position  (GdkDevice *device,
                                                         double    *win_x,
                                                         double    *win_y);

GDK_AVAILABLE_IN_4_2
guint32             gdk_device_get_timestamp            (GdkDevice *device);

G_DEFINE_AUTOPTR_CLEANUP_FUNC(GdkDevice, g_object_unref)

G_END_DECLS

#endif /* __GDK_DEVICE_H__ */
