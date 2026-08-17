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

#if !defined (__GDK_H_INSIDE__) && !defined (GDK_COMPILATION)
#error "Only <gdk/gdk.h> can be included directly."
#endif

#include <gdk/gdkversionmacros.h>
#include <gdk/gdktypes.h>


G_BEGIN_DECLS

#define GDK_TYPE_DEVICE         (gdk_device_get_type ())
#define GDK_DEVICE(o)           (G_TYPE_CHECK_INSTANCE_CAST ((o), GDK_TYPE_DEVICE, GdkDevice))
#define GDK_IS_DEVICE(o)        (G_TYPE_CHECK_INSTANCE_TYPE ((o), GDK_TYPE_DEVICE))

typedef struct _GdkTimeCoord GdkTimeCoord;

/**
 * GdkInputSource:
 * @GDK_SOURCE_MOUSE: the device is a mouse. (This will be reported for the core
 *                    pointer, even if it is something else, such as a trackball.)
 * @GDK_SOURCE_PEN: the device is a stylus of a graphics tablet or similar device.
 * @GDK_SOURCE_ERASER: the device is an eraser. Typically, this would be the other end
 *                     of a stylus on a graphics tablet.
 * @GDK_SOURCE_CURSOR: the device is a graphics tablet “puck” or similar device.
 * @GDK_SOURCE_KEYBOARD: the device is a keyboard.
 * @GDK_SOURCE_TOUCHSCREEN: the device is a direct-input touch device, such
 *     as a touchscreen or tablet. This device type has been added in 3.4.
 * @GDK_SOURCE_TOUCHPAD: the device is an indirect touch device, such
 *     as a touchpad. This device type has been added in 3.4.
 * @GDK_SOURCE_TRACKPOINT: the device is a trackpoint. This device type has been
 *     added in 3.22
 * @GDK_SOURCE_TABLET_PAD: the device is a "pad", a collection of buttons,
 *     rings and strips found in drawing tablets. This device type has been
 *     added in 3.22.
 *
 * An enumeration describing the type of an input device in general terms.
 */
typedef enum
{
  GDK_SOURCE_MOUSE,
  GDK_SOURCE_PEN,
  GDK_SOURCE_ERASER,
  GDK_SOURCE_CURSOR,
  GDK_SOURCE_KEYBOARD,
  GDK_SOURCE_TOUCHSCREEN,
  GDK_SOURCE_TOUCHPAD,
  GDK_SOURCE_TRACKPOINT,
  GDK_SOURCE_TABLET_PAD
} GdkInputSource;

/**
 * GdkInputMode:
 * @GDK_MODE_DISABLED: the device is disabled and will not report any events.
 * @GDK_MODE_SCREEN: the device is enabled. The device’s coordinate space
 *                   maps to the entire screen.
 * @GDK_MODE_WINDOW: the device is enabled. The device’s coordinate space
 *                   is mapped to a single window. The manner in which this window
 *                   is chosen is undefined, but it will typically be the same
 *                   way in which the focus window for key events is determined.
 *
 * An enumeration that describes the mode of an input device.
 */
typedef enum
{
  GDK_MODE_DISABLED,
  GDK_MODE_SCREEN,
  GDK_MODE_WINDOW
} GdkInputMode;

/**
 * GdkDeviceType:
 * @GDK_DEVICE_TYPE_MASTER: Device is a master (or virtual) device. There will
 *                          be an associated focus indicator on the screen.
 * @GDK_DEVICE_TYPE_SLAVE: Device is a slave (or physical) device.
 * @GDK_DEVICE_TYPE_FLOATING: Device is a physical device, currently not attached to
 *                            any virtual device.
 *
 * Indicates the device type. See [above][GdkDeviceManager.description]
 * for more information about the meaning of these device types.
 */
typedef enum {
  GDK_DEVICE_TYPE_MASTER,
  GDK_DEVICE_TYPE_SLAVE,
  GDK_DEVICE_TYPE_FLOATING
} GdkDeviceType;

/* We don't allocate each coordinate this big, but we use it to
 * be ANSI compliant and avoid accessing past the defined limits.
 */
#define GDK_MAX_TIMECOORD_AXES 128

/**
 * GdkTimeCoord:
 * @time: The timestamp for this event.
 * @axes: the values of the device’s axes.
 *
 * A #GdkTimeCoord stores a single event in a motion history.
 */
struct _GdkTimeCoord
{
  guint32 time;
  gdouble axes[GDK_MAX_TIMECOORD_AXES];
};

GDK_AVAILABLE_IN_ALL
GType                 gdk_device_get_type       (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
const gchar *         gdk_device_get_name       (GdkDevice *device);
GDK_AVAILABLE_IN_ALL
gboolean              gdk_device_get_has_cursor (GdkDevice *device);

/* Functions to configure a device */
GDK_AVAILABLE_IN_ALL
GdkInputSource gdk_device_get_source    (GdkDevice      *device);

GDK_AVAILABLE_IN_ALL
GdkInputMode   gdk_device_get_mode      (GdkDevice      *device);
GDK_AVAILABLE_IN_ALL
gboolean       gdk_device_set_mode      (GdkDevice      *device,
                                         GdkInputMode    mode);

GDK_AVAILABLE_IN_ALL
gint           gdk_device_get_n_keys    (GdkDevice       *device);
GDK_AVAILABLE_IN_ALL
gboolean       gdk_device_get_key       (GdkDevice       *device,
                                         guint            index_,
                                         guint           *keyval,
                                         GdkModifierType *modifiers);
GDK_AVAILABLE_IN_ALL
void           gdk_device_set_key       (GdkDevice      *device,
                                         guint           index_,
                                         guint           keyval,
                                         GdkModifierType modifiers);

GDK_AVAILABLE_IN_ALL
GdkAxisUse     gdk_device_get_axis_use  (GdkDevice         *device,
                                         guint              index_);
GDK_AVAILABLE_IN_ALL
void           gdk_device_set_axis_use  (GdkDevice         *device,
                                         guint              index_,
                                         GdkAxisUse         use);


GDK_AVAILABLE_IN_ALL
void     gdk_device_get_state    (GdkDevice         *device,
                                  GdkWindow         *window,
                                  gdouble           *axes,
                                  GdkModifierType   *mask);
GDK_AVAILABLE_IN_ALL
void     gdk_device_get_position (GdkDevice         *device,
                                  GdkScreen        **screen,
                                  gint              *x,
                                  gint              *y);
GDK_AVAILABLE_IN_ALL
GdkWindow *
         gdk_device_get_window_at_position
                                 (GdkDevice         *device,
                                  gint              *win_x,
                                  gint              *win_y);
GDK_AVAILABLE_IN_3_10
void     gdk_device_get_position_double (GdkDevice         *device,
                                         GdkScreen        **screen,
                                         gdouble           *x,
                                         gdouble           *y);
GDK_AVAILABLE_IN_3_10
GdkWindow *
         gdk_device_get_window_at_position_double
                                 (GdkDevice         *device,
                                  gdouble           *win_x,
                                  gdouble           *win_y);
GDK_AVAILABLE_IN_ALL
gboolean gdk_device_get_history  (GdkDevice         *device,
                                  GdkWindow         *window,
                                  guint32            start,
                                  guint32            stop,
                                  GdkTimeCoord    ***events,
                                  gint              *n_events);
GDK_AVAILABLE_IN_ALL
void     gdk_device_free_history (GdkTimeCoord     **events,
                                  gint               n_events);

GDK_AVAILABLE_IN_ALL
gint     gdk_device_get_n_axes     (GdkDevice       *device);
GDK_AVAILABLE_IN_ALL
GList *  gdk_device_list_axes      (GdkDevice       *device);
GDK_AVAILABLE_IN_ALL
gboolean gdk_device_get_axis_value (GdkDevice       *device,
                                    gdouble         *axes,
                                    GdkAtom          axis_label,
                                    gdouble         *value);

GDK_AVAILABLE_IN_ALL
gboolean gdk_device_get_axis     (GdkDevice         *device,
                                  gdouble           *axes,
                                  GdkAxisUse         use,
                                  gdouble           *value);
GDK_AVAILABLE_IN_ALL
GdkDisplay * gdk_device_get_display (GdkDevice      *device);

GDK_AVAILABLE_IN_ALL
GdkDevice  * gdk_device_get_associated_device (GdkDevice     *device);
GDK_AVAILABLE_IN_ALL
GList *      gdk_device_list_slave_devices    (GdkDevice     *device);

GDK_AVAILABLE_IN_ALL
GdkDeviceType gdk_device_get_device_type (GdkDevice *device);

GDK_DEPRECATED_IN_3_20_FOR(gdk_seat_grab)
GdkGrabStatus gdk_device_grab        (GdkDevice        *device,
                                      GdkWindow        *window,
                                      GdkGrabOwnership  grab_ownership,
                                      gboolean          owner_events,
                                      GdkEventMask      event_mask,
                                      GdkCursor        *cursor,
                                      guint32           time_);

GDK_DEPRECATED_IN_3_20_FOR(gdk_seat_ungrab)
void          gdk_device_ungrab      (GdkDevice        *device,
                                      guint32           time_);

GDK_AVAILABLE_IN_ALL
void          gdk_device_warp        (GdkDevice        *device,
                                      GdkScreen        *screen,
                                      gint              x,
                                      gint              y);

GDK_DEPRECATED_IN_3_16
gboolean gdk_device_grab_info_libgtk_only (GdkDisplay  *display,
                                           GdkDevice   *device,
                                           GdkWindow  **grab_window,
                                           gboolean    *owner_events);

GDK_AVAILABLE_IN_3_12
GdkWindow *gdk_device_get_last_event_window (GdkDevice *device);

GDK_AVAILABLE_IN_3_16
const gchar *gdk_device_get_vendor_id       (GdkDevice *device);
GDK_AVAILABLE_IN_3_16
const gchar *gdk_device_get_product_id      (GdkDevice *device);

GDK_AVAILABLE_IN_3_20
GdkSeat     *gdk_device_get_seat            (GdkDevice *device);

GDK_AVAILABLE_IN_3_22
GdkAxisFlags gdk_device_get_axes            (GdkDevice *device);

G_END_DECLS

#endif /* __GDK_DEVICE_H__ */
