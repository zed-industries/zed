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

/**
 * SECTION:gtkfeatures
 * @Short_description: Variables and functions to check the GTK+ version
 * @Title: Version Information
 *
 * GTK+ provides version information, primarily useful in configure checks
 * for builds that have a configure script. Applications will not typically
 * use the features described here.
 */

/**
 * GTK_MAJOR_VERSION:
 *
 * Like gtk_get_major_version(), but from the headers used at
 * application compile time, rather than from the library linked
 * against at application run time.
 */
#define GTK_MAJOR_VERSION (3)

/**
 * GTK_MINOR_VERSION:
 *
 * Like gtk_get_minor_version(), but from the headers used at
 * application compile time, rather than from the library linked
 * against at application run time.
 */
#define GTK_MINOR_VERSION (24)

/**
 * GTK_MICRO_VERSION:
 *
 * Like gtk_get_micro_version(), but from the headers used at
 * application compile time, rather than from the library linked
 * against at application run time.
 */
#define GTK_MICRO_VERSION (24)

/**
 * GTK_BINARY_AGE:
 *
 * Like gtk_get_binary_age(), but from the headers used at
 * application compile time, rather than from the library linked
 * against at application run time.
 */
#define GTK_BINARY_AGE    (2424)

/**
 * GTK_INTERFACE_AGE:
 *
 * Like gtk_get_interface_age(), but from the headers used at
 * application compile time, rather than from the library linked
 * against at application run time.
 */
#define GTK_INTERFACE_AGE (20)

/**
 * GTK_CHECK_VERSION:
 * @major: major version (e.g. 1 for version 1.2.5)
 * @minor: minor version (e.g. 2 for version 1.2.5)
 * @micro: micro version (e.g. 5 for version 1.2.5)
 *
 * Returns %TRUE if the version of the GTK+ header files
 * is the same as or newer than the passed-in version.
 *
 * Returns: %TRUE if GTK+ headers are new enough
 */
#define GTK_CHECK_VERSION(major,minor,micro)                          \
    (GTK_MAJOR_VERSION > (major) ||                                   \
     (GTK_MAJOR_VERSION == (major) && GTK_MINOR_VERSION > (minor)) || \
     (GTK_MAJOR_VERSION == (major) && GTK_MINOR_VERSION == (minor) && \
      GTK_MICRO_VERSION >= (micro)))

#endif /* __GTK_VERSION_H__ */
