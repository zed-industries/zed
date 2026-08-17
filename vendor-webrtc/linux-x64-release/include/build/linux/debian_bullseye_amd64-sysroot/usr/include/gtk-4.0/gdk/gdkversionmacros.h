/* gdkversionmacros.h - version boundaries checks
 * Copyright (C) 2012 Red Hat, Inc.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.▸ See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library; if not, see <http://www.gnu.org/licenses/>.
 */

#if !defined (__GDK_H_INSIDE__) && !defined (__GTK_CSS_H_INSIDE__) && !defined (GTK_COMPILATION) && !defined (GTK_CSS_COMPILATION)
#error "Only <gdk/gdk.h> can be included directly."
#endif

#ifndef __GDK_VERSION_MACROS_H__
#define __GDK_VERSION_MACROS_H__

#include <glib.h>

/**
 * GDK_MAJOR_VERSION:
 *
 * The major version component of the library's version, e.g. "1" for "1.2.3".
 */
#define GDK_MAJOR_VERSION (4)

/**
 * GDK_MINOR_VERSION:
 *
 * The minor version component of the library's version, e.g. "2" for "1.2.3".
 */
#define GDK_MINOR_VERSION (8)

/**
 * GDK_MICRO_VERSION:
 *
 * The micro version component of the library's version, e.g. "3" for "1.2.3".
 */
#define GDK_MICRO_VERSION (3)

#ifndef _GDK_EXTERN
#define _GDK_EXTERN extern
#endif

/**
 * GDK_DISABLE_DEPRECATION_WARNINGS:
 *
 * A macro that should be defined before including the `gdk.h` header.
 *
 * If this symbol is defined, no compiler warnings will be produced for
 * uses of deprecated GDK and GTK APIs.
 */

#ifdef GDK_DISABLE_DEPRECATION_WARNINGS
#define GDK_DEPRECATED _GDK_EXTERN
#define GDK_DEPRECATED_FOR(f) _GDK_EXTERN
#define GDK_UNAVAILABLE(maj,min) _GDK_EXTERN
#else
#define GDK_DEPRECATED G_DEPRECATED _GDK_EXTERN
#define GDK_DEPRECATED_FOR(f) G_DEPRECATED_FOR(f) _GDK_EXTERN
#define GDK_UNAVAILABLE(maj,min) G_UNAVAILABLE(maj,min) _GDK_EXTERN
#endif

/* XXX: Every new stable minor release bump should add a macro here */

/**
 * GDK_VERSION_4_0:
 *
 * A macro that evaluates to the 4.0 version of GDK, in a format
 * that can be used by the C pre-processor.
 */
#define GDK_VERSION_4_0        (G_ENCODE_VERSION (4, 0))

/**
 * GDK_VERSION_4_2:
 *
 * A macro that evaluates to the 4.2 version of GDK, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 4.2
 */
#define GDK_VERSION_4_2        (G_ENCODE_VERSION (4, 2))

/**
 * GDK_VERSION_4_4:
 *
 * A macro that evaluates to the 4.4 version of GDK, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 4.4
 */
#define GDK_VERSION_4_4        (G_ENCODE_VERSION (4, 4))

/**
 * GDK_VERSION_4_6:
 *
 * A macro that evaluates to the 4.6 version of GDK, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 4.6
 */
#define GDK_VERSION_4_6        (G_ENCODE_VERSION (4, 6))

/**
 * GDK_VERSION_4_8:
 *
 * A macro that evaluates to the 4.8 version of GDK, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 4.8
 */
#define GDK_VERSION_4_8        (G_ENCODE_VERSION (4, 8))


/* evaluates to the current stable version; for development cycles,
 * this means the next stable target, with a hard backstop to the
 * beginning of the stable series
 */
#if GDK_MAJOR_VERSION >= 4 && (GDK_MINOR_VERSION % 2)
# define GDK_VERSION_CUR_STABLE         (G_ENCODE_VERSION (GDK_MAJOR_VERSION, GDK_MINOR_VERSION + 1))
#elif G_ENCODE_VERSION (GDK_MAJOR_VERSION, GDK_MINOR_VERSION) > GDK_VERSION_4_0
# define GDK_VERSION_CUR_STABLE         (G_ENCODE_VERSION (GDK_MAJOR_VERSION, GDK_MINOR_VERSION))
#else
# define GDK_VERSION_CUR_STABLE         GDK_VERSION_4_0
#endif

/* evaluates to the previous stable version, with a hard backstop
 * to the beginning of the stable series
 */
#if GDK_MAJOR_VERSION >= 4 && (GDK_MINOR_VERSION % 2)
# define GDK_VERSION_PREV_STABLE        (G_ENCODE_VERSION (GDK_MAJOR_VERSION, GDK_MINOR_VERSION - 1))
#elif GDK_MAJOR_VERSION >= 4 && GDK_MINOR_VERSION > 2
# define GDK_VERSION_PREV_STABLE        (G_ENCODE_VERSION (GDK_MAJOR_VERSION, GDK_MINOR_VERSION - 2))
#else
# define GDK_VERSION_PREV_STABLE        GDK_VERSION_4_0
#endif

/**
 * GDK_VERSION_MIN_REQUIRED:
 *
 * A macro that should be defined by the user prior to including
 * the `gdk.h` header.
 *
 * The definition should be one of the predefined GDK version
 * macros: %GDK_VERSION_4_0, %GDK_VERSION_4_2,...
 *
 * This macro defines the lower bound for the GDK API to use.
 *
 * If a function has been deprecated in a newer version of GDK,
 * it is possible to use this symbol to avoid the compiler warnings
 * without disabling warning for every deprecated function.
 */
#ifndef GDK_VERSION_MIN_REQUIRED
# define GDK_VERSION_MIN_REQUIRED      (GDK_VERSION_CUR_STABLE)
#endif

/**
 * GDK_VERSION_MAX_ALLOWED:
 *
 * A macro that should be defined by the user prior to including
 * the `gdk.h` header.
 *
 * The definition should be one of the predefined GDK version
 * macros: %GDK_VERSION_4_0, %GDK_VERSION_4_2,...
 *
 * This macro defines the upper bound for the GDK API to use.
 *
 * If a function has been introduced in a newer version of GDK,
 * it is possible to use this symbol to get compiler warnings when
 * trying to use that function.
 */
#ifndef GDK_VERSION_MAX_ALLOWED
# if GDK_VERSION_MIN_REQUIRED > GDK_VERSION_PREV_STABLE
#  define GDK_VERSION_MAX_ALLOWED      GDK_VERSION_MIN_REQUIRED
# else
#  define GDK_VERSION_MAX_ALLOWED      GDK_VERSION_CUR_STABLE
# endif
#endif

/* sanity checks */
#if GDK_VERSION_MAX_ALLOWED < GDK_VERSION_MIN_REQUIRED
# error "GDK_VERSION_MAX_ALLOWED must be >= GDK_VERSION_MIN_REQUIRED"
#endif
#if GDK_VERSION_MIN_REQUIRED < GDK_VERSION_4_0
# error "GDK_VERSION_MIN_REQUIRED must be >= GDK_VERSION_4_0"
#endif

#define GDK_AVAILABLE_IN_ALL                  _GDK_EXTERN

/* XXX: Every new stable minor release should add a set of macros here */

/* This is not really necessary for 4.0, since there can't be an
 * earlier version, and there are no deprecated symbols. We just
 * include it for completeness, and because it's easier to copy
 * this stanza every time a new development cycle starts.
 */
#if GDK_VERSION_MAX_ALLOWED < GDK_VERSION_4_0
# define GDK_AVAILABLE_IN_4_0                   GDK_UNAVAILABLE(4, 0)
#else
# define GDK_AVAILABLE_IN_4_0                   _GDK_EXTERN
#endif

#if GDK_VERSION_MIN_REQUIRED >= GDK_VERSION_4_0
# define GDK_DEPRECATED_IN_4_0                  GDK_DEPRECATED
# define GDK_DEPRECATED_IN_4_0_FOR(f)           GDK_DEPRECATED_FOR(f)
#else
# define GDK_DEPRECATED_IN_4_0                  _GDK_EXTERN
# define GDK_DEPRECATED_IN_4_0_FOR(f)           _GDK_EXTERN
#endif


#if GDK_VERSION_MAX_ALLOWED < GDK_VERSION_4_2
# define GDK_AVAILABLE_IN_4_2                   GDK_UNAVAILABLE(4, 2)
#else
# define GDK_AVAILABLE_IN_4_2                   _GDK_EXTERN
#endif

#if GDK_VERSION_MIN_REQUIRED >= GDK_VERSION_4_2
# define GDK_DEPRECATED_IN_4_2                  GDK_DEPRECATED
# define GDK_DEPRECATED_IN_4_2_FOR(f)           GDK_DEPRECATED_FOR(f)
#else
# define GDK_DEPRECATED_IN_4_2                  _GDK_EXTERN
# define GDK_DEPRECATED_IN_4_2_FOR(f)           _GDK_EXTERN
#endif


#if GDK_VERSION_MAX_ALLOWED < GDK_VERSION_4_4
# define GDK_AVAILABLE_IN_4_4                   GDK_UNAVAILABLE(4, 4)
#else
# define GDK_AVAILABLE_IN_4_4                   _GDK_EXTERN
#endif

#if GDK_VERSION_MIN_REQUIRED >= GDK_VERSION_4_4
# define GDK_DEPRECATED_IN_4_4                  GDK_DEPRECATED
# define GDK_DEPRECATED_IN_4_4_FOR(f)           GDK_DEPRECATED_FOR(f)
#else
# define GDK_DEPRECATED_IN_4_4                  _GDK_EXTERN
# define GDK_DEPRECATED_IN_4_4_FOR(f)           _GDK_EXTERN
#endif

#if GDK_VERSION_MAX_ALLOWED < GDK_VERSION_4_6
# define GDK_AVAILABLE_IN_4_6                   GDK_UNAVAILABLE(4, 6)
#else
# define GDK_AVAILABLE_IN_4_6                   _GDK_EXTERN
#endif

#if GDK_VERSION_MIN_REQUIRED >= GDK_VERSION_4_6
# define GDK_DEPRECATED_IN_4_6                  GDK_DEPRECATED
# define GDK_DEPRECATED_IN_4_6_FOR(f)           GDK_DEPRECATED_FOR(f)
#else
# define GDK_DEPRECATED_IN_4_6                  _GDK_EXTERN
# define GDK_DEPRECATED_IN_4_6_FOR(f)           _GDK_EXTERN
#endif

#if GDK_VERSION_MAX_ALLOWED < GDK_VERSION_4_8
# define GDK_AVAILABLE_IN_4_8                   GDK_UNAVAILABLE(4, 8)
#else
# define GDK_AVAILABLE_IN_4_8                   _GDK_EXTERN
#endif

#if GDK_VERSION_MIN_REQUIRED >= GDK_VERSION_4_8
# define GDK_DEPRECATED_IN_4_8                  GDK_DEPRECATED
# define GDK_DEPRECATED_IN_4_8_FOR(f)           GDK_DEPRECATED_FOR(f)
#else
# define GDK_DEPRECATED_IN_4_8                  _GDK_EXTERN
# define GDK_DEPRECATED_IN_4_8_FOR(f)           _GDK_EXTERN
#endif

#endif  /* __GDK_VERSION_MACROS_H__ */
