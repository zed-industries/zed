/* GDK - The GIMP Drawing Kit
 * Copyright (C) 2000 Red Hat, Inc.
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

#ifndef __GDK_KEYS_H__
#define __GDK_KEYS_H__

#if !defined (__GDK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gdk/gdk.h> can be included directly."
#endif

#include <gdk/gdkversionmacros.h>
#include <gdk/gdktypes.h>

G_BEGIN_DECLS

GDK_AVAILABLE_IN_ALL
const char *gdk_keyval_name      (guint        keyval) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
guint    gdk_keyval_from_name    (const char *keyval_name);
GDK_AVAILABLE_IN_ALL
void     gdk_keyval_convert_case (guint        symbol,
				  guint       *lower,
				  guint       *upper);
GDK_AVAILABLE_IN_ALL
guint    gdk_keyval_to_upper     (guint        keyval) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
guint    gdk_keyval_to_lower     (guint        keyval) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
gboolean gdk_keyval_is_upper     (guint        keyval) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
gboolean gdk_keyval_is_lower     (guint        keyval) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
guint32  gdk_keyval_to_unicode   (guint        keyval) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
guint    gdk_unicode_to_keyval   (guint32      wc) G_GNUC_CONST;


G_END_DECLS

#endif /* __GDK_KEYS_H__ */
