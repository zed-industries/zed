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

/*
 * Modified by the GTK+ Team and others 1997-2000.  See the AUTHORS
 * file for a list of people on the GTK+ Team.  See the ChangeLog
 * files for a list of changes.  These files are distributed with
 * GTK+ at ftp://ftp.gtk.org/pub/gtk/.
 */

#ifndef __GTK_DEBUG_H__
#define __GTK_DEBUG_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <glib.h>
#include <gdk/gdk.h>

G_BEGIN_DECLS

/**
 * GtkDebugFlags:
 * @GTK_DEBUG_TEXT: Information about GtkTextView
 * @GTK_DEBUG_TREE: Information about GtkTreeView
 * @GTK_DEBUG_KEYBINDINGS: Information about keyboard shortcuts
 * @GTK_DEBUG_MODULES: Information about modules and extensions
 * @GTK_DEBUG_GEOMETRY: Information about size allocation
 * @GTK_DEBUG_ICONTHEME: Information about icon themes
 * @GTK_DEBUG_PRINTING: Information about printing
 * @GTK_DEBUG_BUILDER: Trace GtkBuilder operation
 * @GTK_DEBUG_SIZE_REQUEST: Information about size requests
 * @GTK_DEBUG_NO_CSS_CACHE: Disable the style property cache
 * @GTK_DEBUG_INTERACTIVE: Open the GTK inspector
 * @GTK_DEBUG_TOUCHSCREEN: Pretend the pointer is a touchscreen
 * @GTK_DEBUG_ACTIONS: Information about actions and menu models
 * @GTK_DEBUG_LAYOUT: Information from layout managers
 * @GTK_DEBUG_SNAPSHOT: Include debug render nodes in the generated snapshots
 * @GTK_DEBUG_CONSTRAINTS: Information from the constraints solver
 * @GTK_DEBUG_BUILDER_OBJECTS: Log unused GtkBuilder objects
 * @GTK_DEBUG_A11Y: Information about accessibility state changes
 * @GTK_DEBUG_ICONFALLBACK: Information about icon fallback. Since: 4.2
 *
 * Flags to use with gtk_set_debug_flags().
 *
 * Settings these flags causes GTK to print out different
 * types of debugging information. Some of these flags are
 * only available when GTK has been configured with `-Ddebug=true`.
 */
typedef enum {
  GTK_DEBUG_TEXT            = 1 <<  0,
  GTK_DEBUG_TREE            = 1 <<  1,
  GTK_DEBUG_KEYBINDINGS     = 1 <<  2,
  GTK_DEBUG_MODULES         = 1 <<  3,
  GTK_DEBUG_GEOMETRY        = 1 <<  4,
  GTK_DEBUG_ICONTHEME       = 1 <<  5,
  GTK_DEBUG_PRINTING        = 1 <<  6,
  GTK_DEBUG_BUILDER         = 1 <<  7,
  GTK_DEBUG_SIZE_REQUEST    = 1 <<  8,
  GTK_DEBUG_NO_CSS_CACHE    = 1 <<  9,
  GTK_DEBUG_INTERACTIVE     = 1 << 10,
  GTK_DEBUG_TOUCHSCREEN     = 1 << 11,
  GTK_DEBUG_ACTIONS         = 1 << 12,
  GTK_DEBUG_LAYOUT          = 1 << 13,
  GTK_DEBUG_SNAPSHOT        = 1 << 14,
  GTK_DEBUG_CONSTRAINTS     = 1 << 15,
  GTK_DEBUG_BUILDER_OBJECTS = 1 << 16,
  GTK_DEBUG_A11Y            = 1 << 17,
  GTK_DEBUG_ICONFALLBACK    = 1 << 18,
  GTK_DEBUG_INVERT_TEXT_DIR = 1 << 19,
} GtkDebugFlags;

#ifdef G_ENABLE_DEBUG

#define GTK_DEBUG_CHECK(type) G_UNLIKELY (gtk_get_debug_flags () & GTK_DEBUG_##type)

#else /* !G_ENABLE_DEBUG */

#define GTK_DEBUG_CHECK(type) 0

#endif /* G_ENABLE_DEBUG */

GDK_AVAILABLE_IN_ALL
GtkDebugFlags gtk_get_debug_flags (void);
GDK_AVAILABLE_IN_ALL
void          gtk_set_debug_flags (GtkDebugFlags flags);

G_END_DECLS

#endif /* __GTK_DEBUG_H__ */
