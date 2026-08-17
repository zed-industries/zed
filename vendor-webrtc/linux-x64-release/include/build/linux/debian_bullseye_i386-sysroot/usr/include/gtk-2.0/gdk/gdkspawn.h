/*
 * Copyright (C) 2003 Sun Microsystems Inc.
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
 * License along with this library; if not, write to the
 * Free Software Foundation, Inc., 59 Temple Place - Suite 330,
 * Boston, MA 02111-1307, USA.
 *
 * Authors: Mark McLoughlin <mark@skynet.ie>
 */

#ifndef __GDK_SPAWN_H__
#define __GDK_SPAWN_H__

#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GDK_H_INSIDE__) && !defined (GDK_COMPILATION)
#error "Only <gdk/gdk.h> can be included directly."
#endif

#include <gdk/gdkscreen.h>

G_BEGIN_DECLS

#ifndef GDK_DISABLE_DEPRECATED
gboolean gdk_spawn_on_screen              (GdkScreen             *screen,
					   const gchar           *working_directory,
					   gchar                **argv,
					   gchar                **envp,
					   GSpawnFlags            flags,
					   GSpawnChildSetupFunc   child_setup,
					   gpointer               user_data,
					   gint                  *child_pid,
					   GError               **error);

gboolean gdk_spawn_on_screen_with_pipes   (GdkScreen             *screen,
					   const gchar           *working_directory,
					   gchar                **argv,
					   gchar                **envp,
					   GSpawnFlags            flags,
					   GSpawnChildSetupFunc   child_setup,
					   gpointer               user_data,
					   gint                  *child_pid,
					   gint                  *standard_input,
					   gint                  *standard_output,
					   gint                  *standard_error,
					   GError               **error);

gboolean gdk_spawn_command_line_on_screen (GdkScreen             *screen,
					   const gchar           *command_line,
					   GError               **error);
#endif

G_END_DECLS

#endif /* __GDK_SPAWN_H__ */
