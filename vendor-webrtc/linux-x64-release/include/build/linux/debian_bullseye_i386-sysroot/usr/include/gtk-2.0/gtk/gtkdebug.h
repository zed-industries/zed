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
 * License along with this library; if not, write to the
 * Free Software Foundation, Inc., 59 Temple Place - Suite 330,
 * Boston, MA 02111-1307, USA.
 */

/*
 * Modified by the GTK+ Team and others 1997-2000.  See the AUTHORS
 * file for a list of people on the GTK+ Team.  See the ChangeLog
 * files for a list of changes.  These files are distributed with
 * GTK+ at ftp://ftp.gtk.org/pub/gtk/.
 */

#ifndef __GTK_DEBUG_H__
#define __GTK_DEBUG_H__

#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <glib.h>

G_BEGIN_DECLS

typedef enum {
  GTK_DEBUG_MISC        = 1 << 0,
  GTK_DEBUG_PLUGSOCKET  = 1 << 1,
  GTK_DEBUG_TEXT        = 1 << 2,
  GTK_DEBUG_TREE        = 1 << 3,
  GTK_DEBUG_UPDATES     = 1 << 4,
  GTK_DEBUG_KEYBINDINGS = 1 << 5,
  GTK_DEBUG_MULTIHEAD   = 1 << 6,
  GTK_DEBUG_MODULES     = 1 << 7,
  GTK_DEBUG_GEOMETRY    = 1 << 8,
  GTK_DEBUG_ICONTHEME   = 1 << 9,
  GTK_DEBUG_PRINTING	= 1 << 10,
  GTK_DEBUG_BUILDER	= 1 << 11
} GtkDebugFlag;

#ifdef G_ENABLE_DEBUG

#define GTK_NOTE(type,action)                G_STMT_START { \
    if (gtk_debug_flags & GTK_DEBUG_##type)                 \
       { action; };                          } G_STMT_END

#else /* !G_ENABLE_DEBUG */

#define GTK_NOTE(type, action)

#endif /* G_ENABLE_DEBUG */

#ifdef G_OS_WIN32
#  ifdef GTK_COMPILATION
#    define GTKVAR extern __declspec(dllexport)
#  else
#    define GTKVAR extern __declspec(dllimport)
#  endif
#else
#  define GTKVAR extern
#endif

GTKVAR guint gtk_debug_flags;

G_END_DECLS

#endif /* __GTK_DEBUG_H__ */
