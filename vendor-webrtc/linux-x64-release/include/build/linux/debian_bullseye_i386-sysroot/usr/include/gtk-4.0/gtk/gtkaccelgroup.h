/* GTK - The GIMP Toolkit
 * Copyright (C) 1998, 2001 Tim Janik
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

#ifndef __GTK_ACCEL_GROUP_H__
#define __GTK_ACCEL_GROUP_H__


#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gdk/gdk.h>
#include <gtk/gtkenums.h>

G_BEGIN_DECLS


/* --- Accelerators--- */
GDK_AVAILABLE_IN_ALL
gboolean gtk_accelerator_valid		      (guint	        keyval,
					       GdkModifierType  modifiers) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
gboolean gtk_accelerator_parse		      (const char      *accelerator,
					       guint	       *accelerator_key,
					       GdkModifierType *accelerator_mods);
GDK_AVAILABLE_IN_ALL
gboolean gtk_accelerator_parse_with_keycode   (const char      *accelerator,
                                               GdkDisplay      *display,
                                               guint           *accelerator_key,
                                               guint          **accelerator_codes,
                                               GdkModifierType *accelerator_mods);
GDK_AVAILABLE_IN_ALL
char *	 gtk_accelerator_name		      (guint	        accelerator_key,
					       GdkModifierType  accelerator_mods);
GDK_AVAILABLE_IN_ALL
char *	 gtk_accelerator_name_with_keycode    (GdkDisplay      *display,
                                               guint            accelerator_key,
                                               guint            keycode,
                                               GdkModifierType  accelerator_mods);
GDK_AVAILABLE_IN_ALL
char *   gtk_accelerator_get_label            (guint           accelerator_key,
                                               GdkModifierType accelerator_mods);
GDK_AVAILABLE_IN_ALL
char *   gtk_accelerator_get_label_with_keycode (GdkDisplay      *display,
                                                 guint            accelerator_key,
                                                 guint            keycode,
                                                 GdkModifierType  accelerator_mods);
GDK_AVAILABLE_IN_ALL
GdkModifierType gtk_accelerator_get_default_mod_mask (void) G_GNUC_CONST;


G_END_DECLS

#endif /* __GTK_ACCEL_GROUP_H__ */
