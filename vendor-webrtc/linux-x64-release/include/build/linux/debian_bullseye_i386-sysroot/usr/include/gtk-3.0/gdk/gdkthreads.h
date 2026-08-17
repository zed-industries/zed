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
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.	 See the GNU
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

#ifndef __GDK_THREADS_H__
#define __GDK_THREADS_H__

#if !defined (__GDK_H_INSIDE__) && !defined (GDK_COMPILATION)
#error "Only <gdk/gdk.h> can be included directly."
#endif

#include <gdk/gdktypes.h>
#include <gdk/gdkversionmacros.h>

G_BEGIN_DECLS

#if defined(GDK_COMPILATION) || defined(GTK_COMPILATION)
#define GDK_THREADS_DEPRECATED _GDK_EXTERN
#else
#define GDK_THREADS_DEPRECATED GDK_DEPRECATED_IN_3_6
#endif

GDK_THREADS_DEPRECATED
void     gdk_threads_init                     (void);
GDK_THREADS_DEPRECATED
void     gdk_threads_enter                    (void);
GDK_THREADS_DEPRECATED
void     gdk_threads_leave                    (void);
GDK_THREADS_DEPRECATED
void     gdk_threads_set_lock_functions       (GCallback enter_fn,
                                               GCallback leave_fn);

GDK_AVAILABLE_IN_ALL
guint    gdk_threads_add_idle_full            (gint           priority,
                                               GSourceFunc    function,
                                               gpointer       data,
                                               GDestroyNotify notify);
GDK_AVAILABLE_IN_ALL
guint    gdk_threads_add_idle                 (GSourceFunc    function,
                                               gpointer       data);
GDK_AVAILABLE_IN_ALL
guint    gdk_threads_add_timeout_full         (gint           priority,
                                               guint          interval,
                                               GSourceFunc    function,
                                               gpointer       data,
                                               GDestroyNotify notify);
GDK_AVAILABLE_IN_ALL
guint    gdk_threads_add_timeout              (guint          interval,
                                               GSourceFunc    function,
                                               gpointer       data);
GDK_AVAILABLE_IN_ALL
guint    gdk_threads_add_timeout_seconds_full (gint           priority,
                                               guint          interval,
                                               GSourceFunc    function,
                                               gpointer       data,
                                               GDestroyNotify notify);
GDK_AVAILABLE_IN_ALL
guint    gdk_threads_add_timeout_seconds      (guint          interval,
                                               GSourceFunc    function,
                                               gpointer       data);


/**
 * GDK_THREADS_ENTER:
 *
 * This macro marks the beginning of a critical section in which GDK and
 * GTK+ functions can be called safely and without causing race
 * conditions.  Only one thread at a time can be in such a critial
 * section. The macro expands to a no-op if #G_THREADS_ENABLED has not
 * been defined. Typically gdk_threads_enter() should be used instead of
 * this macro.
 *
 * Deprecated:3.6: Use g_main_context_invoke(), g_idle_add() and related
 *     functions if you need to schedule GTK+ calls from other threads.
 */
#define GDK_THREADS_ENTER() gdk_threads_enter()

/**
 * GDK_THREADS_LEAVE:
 *
 * This macro marks the end of a critical section
 * begun with #GDK_THREADS_ENTER.
 *
 * Deprecated:3.6: Deprecated in 3.6.
 */
#define GDK_THREADS_LEAVE() gdk_threads_leave()

#undef GDK_THREADS_DEPRECATED

G_END_DECLS

#endif /* __GDK_THREADS_H__ */
