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
 * Modified by the GTK+ Team and others 1997-1999.  See the AUTHORS
 * file for a list of people on the GTK+ Team.  See the ChangeLog
 * files for a list of changes.  These files are distributed with
 * GTK+ at ftp://ftp.gtk.org/pub/gtk/.
 */

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#ifndef __GTK_VERSION_H__
#define __GTK_VERSION_H__

#include <gdk/gdk.h>

G_BEGIN_DECLS

/**
 * GTK_MAJOR_VERSION:
 *
 * Like [func@get_major_version], but from the headers used at
 * application compile time, rather than from the library linked
 * against at application run time.
 */
#define GTK_MAJOR_VERSION (4)

/**
 * GTK_MINOR_VERSION:
 *
 * Like [func@get_minor_version], but from the headers used at
 * application compile time, rather than from the library linked
 * against at application run time.
 */
#define GTK_MINOR_VERSION (8)

/**
 * GTK_MICRO_VERSION:
 *
 * Like [func@get_micro_version], but from the headers used at
 * application compile time, rather than from the library linked
 * against at application run time.
 */
#define GTK_MICRO_VERSION (3)

/**
 * GTK_BINARY_AGE:
 *
 * Like [func@get_binary_age], but from the headers used at
 * application compile time, rather than from the library linked
 * against at application run time.
 */
#define GTK_BINARY_AGE    (803)

/**
 * GTK_INTERFACE_AGE:
 *
 * Like [func@get_interface_age], but from the headers used at
 * application compile time, rather than from the library linked
 * against at application run time.
 */
#define GTK_INTERFACE_AGE (3)

/**
 * GTK_CHECK_VERSION:
 * @major: major version (e.g. 1 for version 1.2.5)
 * @minor: minor version (e.g. 2 for version 1.2.5)
 * @micro: micro version (e.g. 5 for version 1.2.5)
 *
 * Returns %TRUE if the version of the GTK header files
 * is the same as or newer than the passed-in version.
 *
 * Returns: %TRUE if GTK headers are new enough
 */
#define GTK_CHECK_VERSION(major,minor,micro)                          \
    (GTK_MAJOR_VERSION > (major) ||                                   \
     (GTK_MAJOR_VERSION == (major) && GTK_MINOR_VERSION > (minor)) || \
     (GTK_MAJOR_VERSION == (major) && GTK_MINOR_VERSION == (minor) && \
      GTK_MICRO_VERSION >= (micro)))

GDK_AVAILABLE_IN_ALL
guint gtk_get_major_version (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
guint gtk_get_minor_version (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
guint gtk_get_micro_version (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
guint gtk_get_binary_age    (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
guint gtk_get_interface_age (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
const char * gtk_check_version (guint   required_major,
                                guint   required_minor,
                                guint   required_micro);

G_END_DECLS

#endif /* __GTK_VERSION_H__ */
