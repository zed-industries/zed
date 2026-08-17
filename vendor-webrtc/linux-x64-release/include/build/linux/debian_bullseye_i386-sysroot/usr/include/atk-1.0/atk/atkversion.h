/* ATK -  Accessibility Toolkit
 *
 * Copyright (C) 2012 Igalia, S.L.
 * Copyright (C) 2014 Chun-wei Fan
 *
 * Author: Alejandro Piñeiro Iglesias <apinheiro@igalia.com>
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
 */

#if defined(ATK_DISABLE_SINGLE_INCLUDES) && !defined (__ATK_H_INSIDE__) && !defined (ATK_COMPILATION)
#error "Only <atk/atk.h> can be included directly."
#endif

#ifndef __ATK_VERSION_H__
#define __ATK_VERSION_H__

#include <glib.h>

/**
 * ATK_MAJOR_VERSION:
 *
 * Like atk_get_major_version(), but from the headers used at
 * application compile time, rather than from the library linked
 * against at application run time.
 *
 * Since: 2.7.4
 */
#define ATK_MAJOR_VERSION (2)

/**
 * ATK_MINOR_VERSION:
 *
 * Like atk_get_minor_version(), but from the headers used at
 * application compile time, rather than from the library linked
 * against at application run time.
 *
 * Since: 2.7.4
 */
#define ATK_MINOR_VERSION (38)

/**
 * ATK_MICRO_VERSION:
 *
 * Like atk_get_micro_version(), but from the headers used at
 * application compile time, rather than from the library linked
 * against at application run time.
 *
 * Since: 2.7.4
 */
#define ATK_MICRO_VERSION (0)

/**
 * ATK_BINARY_AGE:
 *
 * Like atk_get_binary_age(), but from the headers used at
 * application compile time, rather than from the library linked
 * against at application run time.
 *
 * Since: 2.7.4
 */
#define ATK_BINARY_AGE    (23810)

/**
 * ATK_INTERFACE_AGE:
 *
 * Like atk_get_interface_age(), but from the headers used at
 * application compile time, rather than from the library linked
 * against at application run time.
 *
 * Since: 2.7.4
 */
#define ATK_INTERFACE_AGE (1)

/**
 * ATK_CHECK_VERSION:
 * @major: major version (e.g. 1 for version 1.2.5)
 * @minor: minor version (e.g. 2 for version 1.2.5)
 * @micro: micro version (e.g. 5 for version 1.2.5)
 *
 * Returns %TRUE if the version of the ATK header files is the same as
 * or newer than the passed-in version.
 *
 * Since: 2.7.4
 */
#define ATK_CHECK_VERSION(major,minor,micro)                          \
    (ATK_MAJOR_VERSION > (major) ||                                   \
     (ATK_MAJOR_VERSION == (major) && ATK_MINOR_VERSION > (minor)) || \
     (ATK_MAJOR_VERSION == (major) && ATK_MINOR_VERSION == (minor) && \
      ATK_MICRO_VERSION >= (micro)))

#ifndef _ATK_EXTERN
#define _ATK_EXTERN extern
#endif

/**
 * ATK_VERSION_2_2:
 *
 * A macro that evaluates to the 2.2 version of ATK, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 2.14
 */
#define ATK_VERSION_2_2       (G_ENCODE_VERSION (2, 2))

/**
 * ATK_VERSION_2_4:
 *
 * A macro that evaluates to the 2.4 version of ATK, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 2.14
 */
#define ATK_VERSION_2_4       (G_ENCODE_VERSION (2, 4))

/**
 * ATK_VERSION_2_6:
 *
 * A macro that evaluates to the 2.6 version of ATK, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 2.14
 */
#define ATK_VERSION_2_6       (G_ENCODE_VERSION (2, 6))

/**
 * ATK_VERSION_2_8:
 *
 * A macro that evaluates to the 2.8 version of ATK, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 2.14
 */
#define ATK_VERSION_2_8       (G_ENCODE_VERSION (2, 8))

/**
 * ATK_VERSION_2_10:
 *
 * A macro that evaluates to the 2.10 version of ATK, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 2.14
 */

#define ATK_VERSION_2_10       (G_ENCODE_VERSION (2, 10))
/**
 * ATK_VERSION_2_12:
 *
 * A macro that evaluates to the 2.12 version of ATK, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 2.14
 */
#define ATK_VERSION_2_12       (G_ENCODE_VERSION (2, 12))

/**
 * ATK_VERSION_2_14:
 *
 * A macro that evaluates to the 2.14 version of ATK, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 2.14
 */
#define ATK_VERSION_2_14       (G_ENCODE_VERSION (2, 14))

/**
 * ATK_VERSION_2_30:
 *
 * A macro that evaluates to the 2.30 version of ATK, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 2.30
 */
#define ATK_VERSION_2_30       (G_ENCODE_VERSION (2, 30))

/**
 * ATK_VERSION_2_36:
 *
 * A macro that evaluates to the 2.36 version of ATK, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 2.36
 */
#define ATK_VERSION_2_36       (G_ENCODE_VERSION (2, 36))

/* evaluates to the current stable version; for development cycles,
 * this means the next stable target
 */
#if (ATK_MINOR_VERSION % 2)
#define ATK_VERSION_CUR_STABLE         (G_ENCODE_VERSION (ATK_MAJOR_VERSION, ATK_MINOR_VERSION + 1))
#else
#define ATK_VERSION_CUR_STABLE         (G_ENCODE_VERSION (ATK_MAJOR_VERSION, ATK_MINOR_VERSION))
#endif

/* evaluates to the previous stable version */
#if (ATK_MINOR_VERSION % 2)
#define ATK_VERSION_PREV_STABLE        (G_ENCODE_VERSION (ATK_MAJOR_VERSION, ATK_MINOR_VERSION - 1))
#else
#define ATK_VERSION_PREV_STABLE        (G_ENCODE_VERSION (ATK_MAJOR_VERSION, ATK_MINOR_VERSION - 2))
#endif

/**
 * ATK_VERSION_MIN_REQUIRED:
 *
 * A macro that should be defined by the user prior to including
 * the atk/atk.h header.
 * The definition should be one of the predefined ATK version
 * macros: %ATK_VERSION_2_12, %ATK_VERSION_2_14,...
 *
 * This macro defines the earliest version of ATK that the package is
 * required to be able to compile against.
 *
 * If the compiler is configured to warn about the use of deprecated
 * functions, then using functions that were deprecated in version
 * %ATK_VERSION_MIN_REQUIRED or earlier will cause warnings (but
 * using functions deprecated in later releases will not).
 *
 * Since: 2.14
 */
/* If the package sets ATK_VERSION_MIN_REQUIRED to some future
 * ATK_VERSION_X_Y value that we don't know about, it will compare as
 * 0 in preprocessor tests.
 */
#ifndef ATK_VERSION_MIN_REQUIRED
# define ATK_VERSION_MIN_REQUIRED      (ATK_VERSION_CUR_STABLE)
#elif ATK_VERSION_MIN_REQUIRED == 0
# undef  ATK_VERSION_MIN_REQUIRED
# define ATK_VERSION_MIN_REQUIRED      (ATK_VERSION_CUR_STABLE + 2)
#endif

/**
 * ATK_VERSION_MAX_ALLOWED:
 *
 * A macro that should be defined by the user prior to including
 * the atk/atk.h header.
 * The definition should be one of the predefined ATK version
 * macros: %ATK_VERSION_2_12, %ATK_VERSION_2_14,...
 *
 * This macro defines the latest version of the ATK API that the
 * package is allowed to make use of.
 *
 * If the compiler is configured to warn about the use of deprecated
 * functions, then using functions added after version
 * %ATK_VERSION_MAX_ALLOWED will cause warnings.
 *
 * Unless you are using ATK_CHECK_VERSION() or the like to compile
 * different code depending on the ATK version, then this should be
 * set to the same value as %ATK_VERSION_MIN_REQUIRED.
 *
 * Since: 2.14
 */
#if !defined (ATK_VERSION_MAX_ALLOWED) || (ATK_VERSION_MAX_ALLOWED == 0)
# undef ATK_VERSION_MAX_ALLOWED
# define ATK_VERSION_MAX_ALLOWED      (ATK_VERSION_CUR_STABLE)
#endif

/* sanity checks */
#if ATK_VERSION_MIN_REQUIRED > ATK_VERSION_CUR_STABLE
#error "ATK_VERSION_MIN_REQUIRED must be <= ATK_VERSION_CUR_STABLE"
#endif
#if ATK_VERSION_MAX_ALLOWED < ATK_VERSION_MIN_REQUIRED
#error "ATK_VERSION_MAX_ALLOWED must be >= ATK_VERSION_MIN_REQUIRED"
#endif
#if ATK_VERSION_MIN_REQUIRED < ATK_VERSION_2_2
#error "ATK_VERSION_MIN_REQUIRED must be >= ATK_VERSION_2_2"
#endif

/* these macros are used to mark deprecated functions, and thus have to be
 * exposed in a public header.
 *
 * do *not* use them in other libraries depending on Atk: use G_DEPRECATED
 * and G_DEPRECATED_FOR, or use your own wrappers around them.
 */
#ifdef ATK_DISABLE_DEPRECATION_WARNINGS
#define ATK_DEPRECATED _ATK_EXTERN
#define ATK_DEPRECATED_FOR(f) _ATK_EXTERN
#define ATK_UNAVAILABLE(maj,min) _ATK_EXTERN
#else
#define ATK_DEPRECATED G_DEPRECATED _ATK_EXTERN
#define ATK_DEPRECATED_FOR(f) G_DEPRECATED_FOR(f) _ATK_EXTERN
#define ATK_UNAVAILABLE(maj,min) G_UNAVAILABLE(maj,min) _ATK_EXTERN
#endif

#define ATK_AVAILABLE_IN_ALL _ATK_EXTERN

/* XXX: Every new stable minor release should add a set of macros here */

#if ATK_VERSION_MIN_REQUIRED >= ATK_VERSION_2_2
# define ATK_DEPRECATED_IN_2_2                ATK_DEPRECATED
# define ATK_DEPRECATED_IN_2_2_FOR(f)         ATK_DEPRECATED_FOR(f)
#else
# define ATK_DEPRECATED_IN_2_2                _ATK_EXTERN
# define ATK_DEPRECATED_IN_2_2_FOR(f)         _ATK_EXTERN
#endif

#if ATK_VERSION_MAX_ALLOWED < ATK_VERSION_2_2
# define ATK_AVAILABLE_IN_2_2                 ATK_UNAVAILABLE(2, 2)
#else
# define ATK_AVAILABLE_IN_2_2                 _ATK_EXTERN
#endif

#if ATK_VERSION_MIN_REQUIRED >= ATK_VERSION_2_4
# define ATK_DEPRECATED_IN_2_4                ATK_DEPRECATED
# define ATK_DEPRECATED_IN_2_4_FOR(f)         ATK_DEPRECATED_FOR(f)
#else
# define ATK_DEPRECATED_IN_2_4                _ATK_EXTERN
# define ATK_DEPRECATED_IN_2_4_FOR(f)         _ATK_EXTERN
#endif

#if ATK_VERSION_MAX_ALLOWED < ATK_VERSION_2_4
# define ATK_AVAILABLE_IN_2_4                 ATK_UNAVAILABLE(2, 4)
#else
# define ATK_AVAILABLE_IN_2_4                 _ATK_EXTERN
#endif

#if ATK_VERSION_MIN_REQUIRED >= ATK_VERSION_2_6
# define ATK_DEPRECATED_IN_2_6                ATK_DEPRECATED
# define ATK_DEPRECATED_IN_2_6_FOR(f)         ATK_DEPRECATED_FOR(f)
#else
# define ATK_DEPRECATED_IN_2_6                _ATK_EXTERN
# define ATK_DEPRECATED_IN_2_6_FOR(f)         _ATK_EXTERN
#endif

#if ATK_VERSION_MAX_ALLOWED < ATK_VERSION_2_6
# define ATK_AVAILABLE_IN_2_6                 ATK_UNAVAILABLE(2, 6)
#else
# define ATK_AVAILABLE_IN_2_6                 _ATK_EXTERN
#endif

#if ATK_VERSION_MIN_REQUIRED >= ATK_VERSION_2_8
# define ATK_DEPRECATED_IN_2_8                ATK_DEPRECATED
# define ATK_DEPRECATED_IN_2_8_FOR(f)         ATK_DEPRECATED_FOR(f)
#else
# define ATK_DEPRECATED_IN_2_8                _ATK_EXTERN
# define ATK_DEPRECATED_IN_2_8_FOR(f)         _ATK_EXTERN
#endif

#if ATK_VERSION_MAX_ALLOWED < ATK_VERSION_2_8
# define ATK_AVAILABLE_IN_2_8                 ATK_UNAVAILABLE(2, 8)
#else
# define ATK_AVAILABLE_IN_2_8                 _ATK_EXTERN
#endif

#if ATK_VERSION_MIN_REQUIRED >= ATK_VERSION_2_10
# define ATK_DEPRECATED_IN_2_10                ATK_DEPRECATED
# define ATK_DEPRECATED_IN_2_10_FOR(f)         ATK_DEPRECATED_FOR(f)
#else
# define ATK_DEPRECATED_IN_2_10                _ATK_EXTERN
# define ATK_DEPRECATED_IN_2_10_FOR(f)         _ATK_EXTERN
#endif

#if ATK_VERSION_MAX_ALLOWED < ATK_VERSION_2_10
# define ATK_AVAILABLE_IN_2_10                 ATK_UNAVAILABLE(2, 10)
#else
# define ATK_AVAILABLE_IN_2_10                 _ATK_EXTERN
#endif

#if ATK_VERSION_MIN_REQUIRED >= ATK_VERSION_2_12
# define ATK_DEPRECATED_IN_2_12                ATK_DEPRECATED
# define ATK_DEPRECATED_IN_2_12_FOR(f)         ATK_DEPRECATED_FOR(f)
#else
# define ATK_DEPRECATED_IN_2_12                _ATK_EXTERN
# define ATK_DEPRECATED_IN_2_12_FOR(f)         _ATK_EXTERN
#endif

#if ATK_VERSION_MAX_ALLOWED < ATK_VERSION_2_12
# define ATK_AVAILABLE_IN_2_12                 ATK_UNAVAILABLE(2, 12)
#else
# define ATK_AVAILABLE_IN_2_12                 _ATK_EXTERN
#endif

#if ATK_VERSION_MIN_REQUIRED >= ATK_VERSION_2_14
# define ATK_DEPRECATED_IN_2_14                ATK_DEPRECATED
# define ATK_DEPRECATED_IN_2_14_FOR(f)         ATK_DEPRECATED_FOR(f)
#else
# define ATK_DEPRECATED_IN_2_14                _ATK_EXTERN
# define ATK_DEPRECATED_IN_2_14_FOR(f)         _ATK_EXTERN
#endif

#if ATK_VERSION_MAX_ALLOWED < ATK_VERSION_2_14
# define ATK_AVAILABLE_IN_2_14                 ATK_UNAVAILABLE(2, 14)
#else
# define ATK_AVAILABLE_IN_2_14                 _ATK_EXTERN
#endif

#if ATK_VERSION_MIN_REQUIRED >= ATK_VERSION_2_30
# define ATK_DEPRECATED_IN_2_30                ATK_DEPRECATED
# define ATK_DEPRECATED_IN_2_30_FOR(f)         ATK_DEPRECATED_FOR(f)
#else
# define ATK_DEPRECATED_IN_2_30                _ATK_EXTERN
# define ATK_DEPRECATED_IN_2_30_FOR(f)         _ATK_EXTERN
#endif

#if ATK_VERSION_MAX_ALLOWED < ATK_VERSION_2_30
# define ATK_AVAILABLE_IN_2_30                 ATK_UNAVAILABLE(2, 30)
#else
# define ATK_AVAILABLE_IN_2_30                 _ATK_EXTERN
#endif

#if ATK_VERSION_MIN_REQUIRED >= ATK_VERSION_2_36
# define ATK_DEPRECATED_IN_2_36                ATK_DEPRECATED
# define ATK_DEPRECATED_IN_2_36_FOR(f)         ATK_DEPRECATED_FOR(f)
#else
# define ATK_DEPRECATED_IN_2_36                _ATK_EXTERN
# define ATK_DEPRECATED_IN_2_36_FOR(f)         _ATK_EXTERN
#endif

#if ATK_VERSION_MAX_ALLOWED < ATK_VERSION_2_36
# define ATK_AVAILABLE_IN_2_36                 ATK_UNAVAILABLE(2, 36)
#else
# define ATK_AVAILABLE_IN_2_36                 _ATK_EXTERN
#endif

ATK_AVAILABLE_IN_2_8
guint atk_get_major_version (void) G_GNUC_CONST;
ATK_AVAILABLE_IN_2_8
guint atk_get_minor_version (void) G_GNUC_CONST;
ATK_AVAILABLE_IN_2_8
guint atk_get_micro_version (void) G_GNUC_CONST;
ATK_AVAILABLE_IN_2_8
guint atk_get_binary_age    (void) G_GNUC_CONST;
ATK_AVAILABLE_IN_2_8
guint atk_get_interface_age (void) G_GNUC_CONST;

#define atk_major_version atk_get_major_version ()
#define atk_minor_version atk_get_minor_version ()
#define atk_micro_version atk_get_micro_version ()
#define atk_binary_age atk_get_binary_age ()
#define atk_interface_age atk_get_interface_age ()

#endif /* __ATK_VERSION_H__ */
