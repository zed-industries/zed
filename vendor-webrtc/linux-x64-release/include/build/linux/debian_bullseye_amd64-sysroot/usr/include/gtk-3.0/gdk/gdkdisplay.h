/*
 * gdkdisplay.h
 *
 * Copyright 2001 Sun Microsystems Inc.
 *
 * Erwann Chenede <erwann.chenede@sun.com>
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

#ifndef __GDK_DISPLAY_H__
#define __GDK_DISPLAY_H__

#if !defined (__GDK_H_INSIDE__) && !defined (GDK_COMPILATION)
#error "Only <gdk/gdk.h> can be included directly."
#endif

#include <gdk/gdkversionmacros.h>
#include <gdk/gdktypes.h>
#include <gdk/gdkevents.h>
#include <gdk/gdkdevicemanager.h>
#include <gdk/gdkseat.h>
#include <gdk/gdkmonitor.h>

G_BEGIN_DECLS

#define GDK_TYPE_DISPLAY              (gdk_display_get_type ())
#define GDK_DISPLAY(object)           (G_TYPE_CHECK_INSTANCE_CAST ((object), GDK_TYPE_DISPLAY, GdkDisplay))
#define GDK_IS_DISPLAY(object)        (G_TYPE_CHECK_INSTANCE_TYPE ((object), GDK_TYPE_DISPLAY))
#ifndef GDK_DISABLE_DEPRECATED
#define GDK_DISPLAY_OBJECT(object)    GDK_DISPLAY(object)
#endif

GDK_AVAILABLE_IN_ALL
GType       gdk_display_get_type (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
GdkDisplay *gdk_display_open                (const gchar *display_name);

GDK_AVAILABLE_IN_ALL
const gchar * gdk_display_get_name         (GdkDisplay *display);

GDK_DEPRECATED_IN_3_10
gint        gdk_display_get_n_screens      (GdkDisplay  *display);
GDK_DEPRECATED_IN_3_20
GdkScreen * gdk_display_get_screen         (GdkDisplay  *display,
                                            gint         screen_num);
GDK_AVAILABLE_IN_ALL
GdkScreen * gdk_display_get_default_screen (GdkDisplay  *display);

#ifndef GDK_MULTIDEVICE_SAFE
GDK_DEPRECATED_IN_3_0_FOR(gdk_device_ungrab)
void        gdk_display_pointer_ungrab     (GdkDisplay  *display,
                                            guint32      time_);
GDK_DEPRECATED_IN_3_0_FOR(gdk_device_ungrab)
void        gdk_display_keyboard_ungrab    (GdkDisplay  *display,
                                            guint32      time_);
GDK_DEPRECATED_IN_3_0_FOR(gdk_display_device_is_grabbed)
gboolean    gdk_display_pointer_is_grabbed (GdkDisplay  *display);
#endif /* GDK_MULTIDEVICE_SAFE */

GDK_AVAILABLE_IN_ALL
gboolean    gdk_display_device_is_grabbed  (GdkDisplay  *display,
                                            GdkDevice   *device);
GDK_AVAILABLE_IN_ALL
void        gdk_display_beep               (GdkDisplay  *display);
GDK_AVAILABLE_IN_ALL
void        gdk_display_sync               (GdkDisplay  *display);
GDK_AVAILABLE_IN_ALL
void        gdk_display_flush              (GdkDisplay  *display);

GDK_AVAILABLE_IN_ALL
void        gdk_display_close                  (GdkDisplay  *display);
GDK_AVAILABLE_IN_ALL
gboolean    gdk_display_is_closed          (GdkDisplay  *display);

GDK_DEPRECATED_IN_3_0_FOR(gdk_device_manager_list_devices)
GList *     gdk_display_list_devices       (GdkDisplay  *display);

GDK_AVAILABLE_IN_ALL
GdkEvent* gdk_display_get_event  (GdkDisplay     *display);
GDK_AVAILABLE_IN_ALL
GdkEvent* gdk_display_peek_event (GdkDisplay     *display);
GDK_AVAILABLE_IN_ALL
void      gdk_display_put_event  (GdkDisplay     *display,
                                  const GdkEvent *event);
GDK_AVAILABLE_IN_ALL
gboolean  gdk_display_has_pending (GdkDisplay  *display);

GDK_AVAILABLE_IN_ALL
void gdk_display_set_double_click_time     (GdkDisplay   *display,
                                            guint         msec);
GDK_AVAILABLE_IN_ALL
void gdk_display_set_double_click_distance (GdkDisplay   *display,
                                            guint         distance);

GDK_AVAILABLE_IN_ALL
GdkDisplay *gdk_display_get_default (void);

#ifndef GDK_MULTIDEVICE_SAFE
GDK_DEPRECATED_IN_3_0_FOR(gdk_device_get_position)
void             gdk_display_get_pointer           (GdkDisplay             *display,
                                                    GdkScreen             **screen,
                                                    gint                   *x,
                                                    gint                   *y,
                                                    GdkModifierType        *mask);
GDK_DEPRECATED_IN_3_0_FOR(gdk_device_get_window_at_position)
GdkWindow *      gdk_display_get_window_at_pointer (GdkDisplay             *display,
                                                    gint                   *win_x,
                                                    gint                   *win_y);
GDK_DEPRECATED_IN_3_0_FOR(gdk_device_warp)
void             gdk_display_warp_pointer          (GdkDisplay             *display,
                                                    GdkScreen              *screen,
                                                    gint                   x,
                                                    gint                   y);
#endif /* GDK_MULTIDEVICE_SAFE */

GDK_DEPRECATED_IN_3_16
GdkDisplay *gdk_display_open_default_libgtk_only (void);

GDK_AVAILABLE_IN_ALL
gboolean gdk_display_supports_cursor_alpha     (GdkDisplay    *display);
GDK_AVAILABLE_IN_ALL
gboolean gdk_display_supports_cursor_color     (GdkDisplay    *display);
GDK_AVAILABLE_IN_ALL
guint    gdk_display_get_default_cursor_size   (GdkDisplay    *display);
GDK_AVAILABLE_IN_ALL
void     gdk_display_get_maximal_cursor_size   (GdkDisplay    *display,
                                                guint         *width,
                                                guint         *height);

GDK_AVAILABLE_IN_ALL
GdkWindow *gdk_display_get_default_group       (GdkDisplay *display); 

GDK_AVAILABLE_IN_ALL
gboolean gdk_display_supports_selection_notification (GdkDisplay *display);
GDK_AVAILABLE_IN_ALL
gboolean gdk_display_request_selection_notification  (GdkDisplay *display,
                                                      GdkAtom     selection);

GDK_AVAILABLE_IN_ALL
gboolean gdk_display_supports_clipboard_persistence (GdkDisplay    *display);
GDK_AVAILABLE_IN_ALL
void     gdk_display_store_clipboard                (GdkDisplay    *display,
                                                     GdkWindow     *clipboard_window,
                                                     guint32        time_,
                                                     const GdkAtom *targets,
                                                     gint           n_targets);

GDK_AVAILABLE_IN_ALL
gboolean gdk_display_supports_shapes           (GdkDisplay    *display);
GDK_AVAILABLE_IN_ALL
gboolean gdk_display_supports_input_shapes     (GdkDisplay    *display);
GDK_DEPRECATED_IN_3_16
gboolean gdk_display_supports_composite        (GdkDisplay    *display);
GDK_AVAILABLE_IN_ALL
void     gdk_display_notify_startup_complete   (GdkDisplay    *display,
                                                const gchar   *startup_id);

GDK_DEPRECATED_IN_3_20_FOR(gdk_display_get_default_seat)
GdkDeviceManager * gdk_display_get_device_manager (GdkDisplay *display);

GDK_AVAILABLE_IN_ALL
GdkAppLaunchContext *gdk_display_get_app_launch_context (GdkDisplay *display);

GDK_AVAILABLE_IN_3_20
GdkSeat * gdk_display_get_default_seat (GdkDisplay *display);

GDK_AVAILABLE_IN_3_20
GList   * gdk_display_list_seats       (GdkDisplay *display);

GDK_AVAILABLE_IN_3_22
int          gdk_display_get_n_monitors        (GdkDisplay *display);
GDK_AVAILABLE_IN_3_22
GdkMonitor * gdk_display_get_monitor           (GdkDisplay *display,
                                                int         monitor_num);
GDK_AVAILABLE_IN_3_22
GdkMonitor * gdk_display_get_primary_monitor   (GdkDisplay *display);
GDK_AVAILABLE_IN_3_22
GdkMonitor * gdk_display_get_monitor_at_point  (GdkDisplay *display,
                                                int         x,
                                                int         y);
GDK_AVAILABLE_IN_3_22
GdkMonitor * gdk_display_get_monitor_at_window (GdkDisplay *display,
                                                GdkWindow  *window);


G_END_DECLS

#endif  /* __GDK_DISPLAY_H__ */
