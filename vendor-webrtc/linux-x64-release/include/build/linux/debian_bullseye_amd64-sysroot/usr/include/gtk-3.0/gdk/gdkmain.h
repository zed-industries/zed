/* GDK - The GIMP Drawing Kit
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

/*
 * Modified by the GTK+ Team and others 1997-2000.  See the AUTHORS
 * file for a list of people on the GTK+ Team.  See the ChangeLog
 * files for a list of changes.  These files are distributed with
 * GTK+ at ftp://ftp.gtk.org/pub/gtk/.
 */

#ifndef __GDK_MAIN_H__
#define __GDK_MAIN_H__

#if !defined (__GDK_H_INSIDE__) && !defined (GDK_COMPILATION)
#error "Only <gdk/gdk.h> can be included directly."
#endif

#include <gdk/gdkversionmacros.h>
#include <gdk/gdktypes.h>

G_BEGIN_DECLS


/* Initialization, exit and events
 */

#define GDK_PRIORITY_EVENTS (G_PRIORITY_DEFAULT)

GDK_AVAILABLE_IN_ALL
void                  gdk_parse_args                      (gint           *argc,
                                                           gchar        ***argv);
GDK_AVAILABLE_IN_ALL
void                  gdk_init                            (gint           *argc,
                                                           gchar        ***argv);
GDK_AVAILABLE_IN_ALL
gboolean              gdk_init_check                      (gint           *argc,
                                                           gchar        ***argv);
GDK_DEPRECATED_IN_3_16
void                  gdk_add_option_entries_libgtk_only  (GOptionGroup   *group);
GDK_DEPRECATED_IN_3_16
void                  gdk_pre_parse_libgtk_only           (void);

GDK_AVAILABLE_IN_ALL
const gchar *         gdk_get_program_class               (void);
GDK_AVAILABLE_IN_ALL
void                  gdk_set_program_class               (const gchar    *program_class);

GDK_AVAILABLE_IN_ALL
void                  gdk_notify_startup_complete         (void);
GDK_AVAILABLE_IN_ALL
void                  gdk_notify_startup_complete_with_id (const gchar* startup_id);

/* Push and pop error handlers for X errors
 */
GDK_DEPRECATED_IN_3_22_FOR(gdk_x11_display_error_trap_push)
void                           gdk_error_trap_push        (void);
/* warn unused because you could use pop_ignored otherwise */
GDK_DEPRECATED_IN_3_22_FOR(gdk_x11_display_error_trap_pop)
G_GNUC_WARN_UNUSED_RESULT gint gdk_error_trap_pop         (void);
GDK_DEPRECATED_IN_3_22_FOR(gdk_x11_display_error_trap_pop_ignored)
void                           gdk_error_trap_pop_ignored (void);


GDK_AVAILABLE_IN_ALL
const gchar *         gdk_get_display_arg_name (void);

GDK_DEPRECATED_IN_3_8_FOR(gdk_display_get_name (gdk_display_get_default ()))
gchar*        gdk_get_display        (void);

#ifndef GDK_MULTIDEVICE_SAFE
GDK_DEPRECATED_IN_3_0_FOR(gdk_device_grab)
GdkGrabStatus gdk_pointer_grab       (GdkWindow    *window,
                                      gboolean      owner_events,
                                      GdkEventMask  event_mask,
                                      GdkWindow    *confine_to,
                                      GdkCursor    *cursor,
                                      guint32       time_);
GDK_DEPRECATED_IN_3_0_FOR(gdk_device_grab)
GdkGrabStatus gdk_keyboard_grab      (GdkWindow    *window,
                                      gboolean      owner_events,
                                      guint32       time_);
#endif /* GDK_MULTIDEVICE_SAFE */

#ifndef GDK_MULTIDEVICE_SAFE
GDK_DEPRECATED_IN_3_0_FOR(gdk_device_ungrab)
void          gdk_pointer_ungrab     (guint32       time_);
GDK_DEPRECATED_IN_3_0_FOR(gdk_device_ungrab)
void          gdk_keyboard_ungrab    (guint32       time_);
GDK_DEPRECATED_IN_3_0_FOR(gdk_display_device_is_grabbed)
gboolean      gdk_pointer_is_grabbed (void);
#endif /* GDK_MULTIDEVICE_SAFE */

GDK_DEPRECATED_IN_3_22
gint gdk_screen_width  (void) G_GNUC_CONST;
GDK_DEPRECATED_IN_3_22
gint gdk_screen_height (void) G_GNUC_CONST;

GDK_DEPRECATED_IN_3_22
gint gdk_screen_width_mm  (void) G_GNUC_CONST;
GDK_DEPRECATED_IN_3_22
gint gdk_screen_height_mm (void) G_GNUC_CONST;

GDK_DEPRECATED_IN_3_22_FOR(gdk_display_set_double_click_time)
void gdk_set_double_click_time (guint msec);

GDK_DEPRECATED_IN_3_22_FOR(gdk_display_beep)
void gdk_beep (void);

GDK_DEPRECATED_IN_3_22_FOR(gdk_display_flush)
void gdk_flush (void);

GDK_AVAILABLE_IN_ALL
void gdk_disable_multidevice (void);

GDK_AVAILABLE_IN_3_10
void gdk_set_allowed_backends (const gchar *backends);

G_END_DECLS

#endif /* __GDK_MAIN_H__ */
