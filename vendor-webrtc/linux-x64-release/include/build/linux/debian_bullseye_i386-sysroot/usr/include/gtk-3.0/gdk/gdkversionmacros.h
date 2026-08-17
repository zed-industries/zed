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

#if !defined (__GDK_H_INSIDE__) && !defined (GDK_COMPILATION)
#error "Only <gdk/gdk.h> can be included directly."
#endif

#ifndef __GDK_VERSION_MACROS_H__
#define __GDK_VERSION_MACROS_H__

#include <glib.h>

#define GDK_MAJOR_VERSION (3)
#define GDK_MINOR_VERSION (24)
#define GDK_MICRO_VERSION (24)

#ifndef _GDK_EXTERN
#define _GDK_EXTERN extern
#endif

/**
 * GDK_DISABLE_DEPRECATION_WARNINGS:
 *
 * A macro that should be defined before including the gdk.h header.
 * If it is defined, no compiler warnings will be produced for uses
 * of deprecated GDK and GTK+ APIs.
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
 * GDK_VERSION_3_0:
 *
 * A macro that evaluates to the 3.0 version of GDK, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 3.4
 */
#define GDK_VERSION_3_0         (G_ENCODE_VERSION (3, 0))

/**
 * GDK_VERSION_3_2:
 *
 * A macro that evaluates to the 3.2 version of GDK, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 3.4
 */
#define GDK_VERSION_3_2         (G_ENCODE_VERSION (3, 2))

/**
 * GDK_VERSION_3_4:
 *
 * A macro that evaluates to the 3.4 version of GDK, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 3.4
 */
#define GDK_VERSION_3_4         (G_ENCODE_VERSION (3, 4))

/**
 * GDK_VERSION_3_6:
 *
 * A macro that evaluates to the 3.6 version of GDK, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 3.6
 */
#define GDK_VERSION_3_6         (G_ENCODE_VERSION (3, 6))

/**
 * GDK_VERSION_3_8:
 *
 * A macro that evaluates to the 3.8 version of GDK, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 3.8
 */
#define GDK_VERSION_3_8         (G_ENCODE_VERSION (3, 8))

/**
 * GDK_VERSION_3_10:
 *
 * A macro that evaluates to the 3.10 version of GDK, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 3.10
 */
#define GDK_VERSION_3_10        (G_ENCODE_VERSION (3, 10))

/**
 * GDK_VERSION_3_12:
 *
 * A macro that evaluates to the 3.12 version of GDK, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 3.12
 */
#define GDK_VERSION_3_12        (G_ENCODE_VERSION (3, 12))

/**
 * GDK_VERSION_3_14:
 *
 * A macro that evaluates to the 3.14 version of GDK, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 3.14
 */
#define GDK_VERSION_3_14        (G_ENCODE_VERSION (3, 14))

/**
 * GDK_VERSION_3_16:
 *
 * A macro that evaluates to the 3.16 version of GDK, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 3.16
 */
#define GDK_VERSION_3_16        (G_ENCODE_VERSION (3, 16))

/**
 * GDK_VERSION_3_18:
 *
 * A macro that evaluates to the 3.18 version of GDK, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 3.18
 */
#define GDK_VERSION_3_18        (G_ENCODE_VERSION (3, 18))

/**
 * GDK_VERSION_3_20:
 *
 * A macro that evaluates to the 3.20 version of GDK, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 3.18
 */
#define GDK_VERSION_3_20        (G_ENCODE_VERSION (3, 20))

/**
 * GDK_VERSION_3_22:
 *
 * A macro that evaluates to the 3.22 version of GDK, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 3.20
 */
#define GDK_VERSION_3_22        (G_ENCODE_VERSION (3, 22))

/**
 * GDK_VERSION_3_24:
 *
 * A macro that evaluates to the 3.24 version of GDK, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 3.24
 */
#define GDK_VERSION_3_24        (G_ENCODE_VERSION (3, 24))

/* evaluates to the current stable version; for development cycles,
 * this means the next stable target
 */
#if (GDK_MINOR_VERSION % 2)
#define GDK_VERSION_CUR_STABLE         (G_ENCODE_VERSION (GDK_MAJOR_VERSION, GDK_MINOR_VERSION + 1))
#else
#define GDK_VERSION_CUR_STABLE         (G_ENCODE_VERSION (GDK_MAJOR_VERSION, GDK_MINOR_VERSION))
#endif

/* evaluates to the previous stable version */
#if (GDK_MINOR_VERSION % 2)
#define GDK_VERSION_PREV_STABLE        (G_ENCODE_VERSION (GDK_MAJOR_VERSION, GDK_MINOR_VERSION - 1))
#else
#define GDK_VERSION_PREV_STABLE        (G_ENCODE_VERSION (GDK_MAJOR_VERSION, GDK_MINOR_VERSION - 2))
#endif

/**
 * GDK_VERSION_MIN_REQUIRED:
 *
 * A macro that should be defined by the user prior to including
 * the gdk.h header.
 * The definition should be one of the predefined GDK version
 * macros: %GDK_VERSION_3_0, %GDK_VERSION_3_2,...
 *
 * This macro defines the lower bound for the GDK API to use.
 *
 * If a function has been deprecated in a newer version of GDK,
 * it is possible to use this symbol to avoid the compiler warnings
 * without disabling warning for every deprecated function.
 *
 * Since: 3.4
 */
#ifndef GDK_VERSION_MIN_REQUIRED
# define GDK_VERSION_MIN_REQUIRED      (GDK_VERSION_CUR_STABLE)
#endif

/**
 * GDK_VERSION_MAX_ALLOWED:
 *
 * A macro that should be defined by the user prior to including
 * the gdk.h header.
 * The definition should be one of the predefined GDK version
 * macros: %GDK_VERSION_3_0, %GDK_VERSION_3_2,...
 *
 * This macro defines the upper bound for the GDK API to use.
 *
 * If a function has been introduced in a newer version of GDK,
 * it is possible to use this symbol to get compiler warnings when
 * trying to use that function.
 *
 * Since: 3.4
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
#error "GDK_VERSION_MAX_ALLOWED must be >= GDK_VERSION_MIN_REQUIRED"
#endif
#if GDK_VERSION_MIN_REQUIRED < GDK_VERSION_3_0
#error "GDK_VERSION_MIN_REQUIRED must be >= GDK_VERSION_3_0"
#endif

#define GDK_AVAILABLE_IN_ALL                  _GDK_EXTERN

/* XXX: Every new stable minor release should add a set of macros here */

#if GDK_VERSION_MIN_REQUIRED >= GDK_VERSION_3_0
# define GDK_DEPRECATED_IN_3_0                GDK_DEPRECATED
# define GDK_DEPRECATED_IN_3_0_FOR(f)         GDK_DEPRECATED_FOR(f)
#else
# define GDK_DEPRECATED_IN_3_0                _GDK_EXTERN
# define GDK_DEPRECATED_IN_3_0_FOR(f)         _GDK_EXTERN
#endif

#if GDK_VERSION_MAX_ALLOWED < GDK_VERSION_3_0
# define GDK_AVAILABLE_IN_3_0                 GDK_UNAVAILABLE(3, 0)
#else
# define GDK_AVAILABLE_IN_3_0                 _GDK_EXTERN
#endif

#if GDK_VERSION_MIN_REQUIRED >= GDK_VERSION_3_2
# define GDK_DEPRECATED_IN_3_2                GDK_DEPRECATED
# define GDK_DEPRECATED_IN_3_2_FOR(f)         GDK_DEPRECATED_FOR(f)
#else
# define GDK_DEPRECATED_IN_3_2                _GDK_EXTERN
# define GDK_DEPRECATED_IN_3_2_FOR(f)         _GDK_EXTERN
#endif

#if GDK_VERSION_MAX_ALLOWED < GDK_VERSION_3_2
# define GDK_AVAILABLE_IN_3_2                 GDK_UNAVAILABLE(3, 2)
#else
# define GDK_AVAILABLE_IN_3_2                 _GDK_EXTERN
#endif

#if GDK_VERSION_MIN_REQUIRED >= GDK_VERSION_3_4
# define GDK_DEPRECATED_IN_3_4                GDK_DEPRECATED
# define GDK_DEPRECATED_IN_3_4_FOR(f)         GDK_DEPRECATED_FOR(f)
#else
# define GDK_DEPRECATED_IN_3_4                _GDK_EXTERN
# define GDK_DEPRECATED_IN_3_4_FOR(f)         _GDK_EXTERN
#endif

#if GDK_VERSION_MAX_ALLOWED < GDK_VERSION_3_4
# define GDK_AVAILABLE_IN_3_4                 GDK_UNAVAILABLE(3, 4)
#else
# define GDK_AVAILABLE_IN_3_4                 _GDK_EXTERN
#endif

#if GDK_VERSION_MIN_REQUIRED >= GDK_VERSION_3_6
# define GDK_DEPRECATED_IN_3_6                GDK_DEPRECATED
# define GDK_DEPRECATED_IN_3_6_FOR(f)         GDK_DEPRECATED_FOR(f)
#else
# define GDK_DEPRECATED_IN_3_6                _GDK_EXTERN
# define GDK_DEPRECATED_IN_3_6_FOR(f)         _GDK_EXTERN
#endif

#if GDK_VERSION_MAX_ALLOWED < GDK_VERSION_3_6
# define GDK_AVAILABLE_IN_3_6                 GDK_UNAVAILABLE(3, 6)
#else
# define GDK_AVAILABLE_IN_3_6                 _GDK_EXTERN
#endif

#if GDK_VERSION_MIN_REQUIRED >= GDK_VERSION_3_8
# define GDK_DEPRECATED_IN_3_8                GDK_DEPRECATED
# define GDK_DEPRECATED_IN_3_8_FOR(f)         GDK_DEPRECATED_FOR(f)
#else
# define GDK_DEPRECATED_IN_3_8                _GDK_EXTERN
# define GDK_DEPRECATED_IN_3_8_FOR(f)         _GDK_EXTERN
#endif

#if GDK_VERSION_MAX_ALLOWED < GDK_VERSION_3_8
# define GDK_AVAILABLE_IN_3_8                 GDK_UNAVAILABLE(3, 8)
#else
# define GDK_AVAILABLE_IN_3_8                 _GDK_EXTERN
#endif

#if GDK_VERSION_MIN_REQUIRED >= GDK_VERSION_3_10
# define GDK_DEPRECATED_IN_3_10               GDK_DEPRECATED
# define GDK_DEPRECATED_IN_3_10_FOR(f)        GDK_DEPRECATED_FOR(f)
#else
# define GDK_DEPRECATED_IN_3_10               _GDK_EXTERN
# define GDK_DEPRECATED_IN_3_10_FOR(f)        _GDK_EXTERN
#endif

#if GDK_VERSION_MAX_ALLOWED < GDK_VERSION_3_10
# define GDK_AVAILABLE_IN_3_10                GDK_UNAVAILABLE(3, 10)
#else
# define GDK_AVAILABLE_IN_3_10                _GDK_EXTERN
#endif

#if GDK_VERSION_MIN_REQUIRED >= GDK_VERSION_3_12
# define GDK_DEPRECATED_IN_3_12               GDK_DEPRECATED
# define GDK_DEPRECATED_IN_3_12_FOR(f)        GDK_DEPRECATED_FOR(f)
#else
# define GDK_DEPRECATED_IN_3_12               _GDK_EXTERN
# define GDK_DEPRECATED_IN_3_12_FOR(f)        _GDK_EXTERN
#endif

#if GDK_VERSION_MAX_ALLOWED < GDK_VERSION_3_12
# define GDK_AVAILABLE_IN_3_12                GDK_UNAVAILABLE(3, 12)
#else
# define GDK_AVAILABLE_IN_3_12                _GDK_EXTERN
#endif

#if GDK_VERSION_MIN_REQUIRED >= GDK_VERSION_3_14
# define GDK_DEPRECATED_IN_3_14               GDK_DEPRECATED
# define GDK_DEPRECATED_IN_3_14_FOR(f)        GDK_DEPRECATED_FOR(f)
#else
# define GDK_DEPRECATED_IN_3_14               _GDK_EXTERN
# define GDK_DEPRECATED_IN_3_14_FOR(f)        _GDK_EXTERN
#endif

#if GDK_VERSION_MAX_ALLOWED < GDK_VERSION_3_14
# define GDK_AVAILABLE_IN_3_14                GDK_UNAVAILABLE(3, 14)
#else
# define GDK_AVAILABLE_IN_3_14                _GDK_EXTERN
#endif

#if GDK_VERSION_MIN_REQUIRED >= GDK_VERSION_3_16
# define GDK_DEPRECATED_IN_3_16               GDK_DEPRECATED
# define GDK_DEPRECATED_IN_3_16_FOR(f)        GDK_DEPRECATED_FOR(f)
#else
# define GDK_DEPRECATED_IN_3_16               _GDK_EXTERN
# define GDK_DEPRECATED_IN_3_16_FOR(f)        _GDK_EXTERN
#endif

#if GDK_VERSION_MAX_ALLOWED < GDK_VERSION_3_16
# define GDK_AVAILABLE_IN_3_16                GDK_UNAVAILABLE(3, 16)
#else
# define GDK_AVAILABLE_IN_3_16                _GDK_EXTERN
#endif

#if GDK_VERSION_MIN_REQUIRED >= GDK_VERSION_3_18
# define GDK_DEPRECATED_IN_3_18               GDK_DEPRECATED
# define GDK_DEPRECATED_IN_3_18_FOR(f)        GDK_DEPRECATED_FOR(f)
#else
# define GDK_DEPRECATED_IN_3_18               _GDK_EXTERN
# define GDK_DEPRECATED_IN_3_18_FOR(f)        _GDK_EXTERN
#endif

#if GDK_VERSION_MAX_ALLOWED < GDK_VERSION_3_18
# define GDK_AVAILABLE_IN_3_18                GDK_UNAVAILABLE(3, 18)
#else
# define GDK_AVAILABLE_IN_3_18                _GDK_EXTERN
#endif

#if GDK_VERSION_MIN_REQUIRED >= GDK_VERSION_3_20
# define GDK_DEPRECATED_IN_3_20               GDK_DEPRECATED
# define GDK_DEPRECATED_IN_3_20_FOR(f)        GDK_DEPRECATED_FOR(f)
#else
# define GDK_DEPRECATED_IN_3_20               _GDK_EXTERN
# define GDK_DEPRECATED_IN_3_20_FOR(f)        _GDK_EXTERN
#endif

#if GDK_VERSION_MAX_ALLOWED < GDK_VERSION_3_20
# define GDK_AVAILABLE_IN_3_20                GDK_UNAVAILABLE(3, 20)
#else
# define GDK_AVAILABLE_IN_3_20                _GDK_EXTERN
#endif

#if GDK_VERSION_MIN_REQUIRED >= GDK_VERSION_3_22
# define GDK_DEPRECATED_IN_3_22               GDK_DEPRECATED
# define GDK_DEPRECATED_IN_3_22_FOR(f)        GDK_DEPRECATED_FOR(f)
#else
# define GDK_DEPRECATED_IN_3_22               _GDK_EXTERN
# define GDK_DEPRECATED_IN_3_22_FOR(f)        _GDK_EXTERN
#endif

#if GDK_VERSION_MAX_ALLOWED < GDK_VERSION_3_22
# define GDK_AVAILABLE_IN_3_22                GDK_UNAVAILABLE(3, 22)
#else
# define GDK_AVAILABLE_IN_3_22                _GDK_EXTERN
#endif

#if GDK_VERSION_MIN_REQUIRED >= GDK_VERSION_3_24
# define GDK_DEPRECATED_IN_3_24               GDK_DEPRECATED
# define GDK_DEPRECATED_IN_3_24_FOR(f)        GDK_DEPRECATED_FOR(f)
#else
# define GDK_DEPRECATED_IN_3_24               _GDK_EXTERN
# define GDK_DEPRECATED_IN_3_24_FOR(f)        _GDK_EXTERN
#endif

#if GDK_VERSION_MAX_ALLOWED < GDK_VERSION_3_24
# define GDK_AVAILABLE_IN_3_24                GDK_UNAVAILABLE(3, 24)
#else
# define GDK_AVAILABLE_IN_3_24                _GDK_EXTERN
#endif

#endif  /* __GDK_VERSION_MACROS_H__ */

