/* GLIB - Library of useful routines for C programming
 * Copyright (C) 1995-1997  Peter Mattis, Spencer Kimball and Josh MacDonald
 *
 * SPDX-License-Identifier: LGPL-2.1-or-later
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library; if not, see <http://www.gnu.org/licenses/>.
 */

/*
 * Modified by the GLib Team and others 1997-2000.  See the AUTHORS
 * file for a list of people on the GLib Team.  See the ChangeLog
 * files for a list of changes.  These files are distributed with
 * GLib at ftp://ftp.gtk.org/pub/gtk/.
 */

#ifndef __G_VERSION_MACROS_H__
#define __G_VERSION_MACROS_H__

#if !defined(__GLIB_H_INSIDE__) && !defined(GLIB_COMPILATION)
#error "Only <glib.h> can be included directly."
#endif

/* Version boundaries checks */

#define G_ENCODE_VERSION(major, minor) ((major) << 16 | (minor) << 8)

/**
 * GLIB_VERSION_2_2:
 *
 * A macro that evaluates to the 2.2 version of GLib, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 2.32
 */
#define GLIB_VERSION_2_2 (G_ENCODE_VERSION(2, 2))
/**
 * GLIB_VERSION_2_4:
 *
 * A macro that evaluates to the 2.4 version of GLib, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 2.32
 */
#define GLIB_VERSION_2_4 (G_ENCODE_VERSION(2, 4))
/**
 * GLIB_VERSION_2_6:
 *
 * A macro that evaluates to the 2.6 version of GLib, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 2.32
 */
#define GLIB_VERSION_2_6 (G_ENCODE_VERSION(2, 6))
/**
 * GLIB_VERSION_2_8:
 *
 * A macro that evaluates to the 2.8 version of GLib, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 2.32
 */
#define GLIB_VERSION_2_8 (G_ENCODE_VERSION(2, 8))
/**
 * GLIB_VERSION_2_10:
 *
 * A macro that evaluates to the 2.10 version of GLib, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 2.32
 */
#define GLIB_VERSION_2_10 (G_ENCODE_VERSION(2, 10))
/**
 * GLIB_VERSION_2_12:
 *
 * A macro that evaluates to the 2.12 version of GLib, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 2.32
 */
#define GLIB_VERSION_2_12 (G_ENCODE_VERSION(2, 12))
/**
 * GLIB_VERSION_2_14:
 *
 * A macro that evaluates to the 2.14 version of GLib, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 2.32
 */
#define GLIB_VERSION_2_14 (G_ENCODE_VERSION(2, 14))
/**
 * GLIB_VERSION_2_16:
 *
 * A macro that evaluates to the 2.16 version of GLib, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 2.32
 */
#define GLIB_VERSION_2_16 (G_ENCODE_VERSION(2, 16))
/**
 * GLIB_VERSION_2_18:
 *
 * A macro that evaluates to the 2.18 version of GLib, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 2.32
 */
#define GLIB_VERSION_2_18 (G_ENCODE_VERSION(2, 18))
/**
 * GLIB_VERSION_2_20:
 *
 * A macro that evaluates to the 2.20 version of GLib, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 2.32
 */
#define GLIB_VERSION_2_20 (G_ENCODE_VERSION(2, 20))
/**
 * GLIB_VERSION_2_22:
 *
 * A macro that evaluates to the 2.22 version of GLib, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 2.32
 */
#define GLIB_VERSION_2_22 (G_ENCODE_VERSION(2, 22))
/**
 * GLIB_VERSION_2_24:
 *
 * A macro that evaluates to the 2.24 version of GLib, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 2.32
 */
#define GLIB_VERSION_2_24 (G_ENCODE_VERSION(2, 24))
/**
 * GLIB_VERSION_2_26:
 *
 * A macro that evaluates to the 2.26 version of GLib, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 2.32
 */
#define GLIB_VERSION_2_26 (G_ENCODE_VERSION(2, 26))
/**
 * GLIB_VERSION_2_28:
 *
 * A macro that evaluates to the 2.28 version of GLib, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 2.32
 */
#define GLIB_VERSION_2_28 (G_ENCODE_VERSION(2, 28))
/**
 * GLIB_VERSION_2_30:
 *
 * A macro that evaluates to the 2.30 version of GLib, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 2.32
 */
#define GLIB_VERSION_2_30 (G_ENCODE_VERSION(2, 30))
/**
 * GLIB_VERSION_2_32:
 *
 * A macro that evaluates to the 2.32 version of GLib, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 2.32
 */
#define GLIB_VERSION_2_32 (G_ENCODE_VERSION(2, 32))
/**
 * GLIB_VERSION_2_34:
 *
 * A macro that evaluates to the 2.34 version of GLib, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 2.34
 */
#define GLIB_VERSION_2_34 (G_ENCODE_VERSION(2, 34))
/**
 * GLIB_VERSION_2_36:
 *
 * A macro that evaluates to the 2.36 version of GLib, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 2.36
 */
#define GLIB_VERSION_2_36 (G_ENCODE_VERSION(2, 36))
/**
 * GLIB_VERSION_2_38:
 *
 * A macro that evaluates to the 2.38 version of GLib, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 2.38
 */
#define GLIB_VERSION_2_38 (G_ENCODE_VERSION(2, 38))
/**
 * GLIB_VERSION_2_40:
 *
 * A macro that evaluates to the 2.40 version of GLib, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 2.40
 */
#define GLIB_VERSION_2_40 (G_ENCODE_VERSION(2, 40))
/**
 * GLIB_VERSION_2_42:
 *
 * A macro that evaluates to the 2.42 version of GLib, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 2.42
 */
#define GLIB_VERSION_2_42 (G_ENCODE_VERSION(2, 42))
/**
 * GLIB_VERSION_2_44:
 *
 * A macro that evaluates to the 2.44 version of GLib, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 2.44
 */
#define GLIB_VERSION_2_44 (G_ENCODE_VERSION(2, 44))
/**
 * GLIB_VERSION_2_46:
 *
 * A macro that evaluates to the 2.46 version of GLib, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 2.46
 */
#define GLIB_VERSION_2_46 (G_ENCODE_VERSION(2, 46))
/**
 * GLIB_VERSION_2_48:
 *
 * A macro that evaluates to the 2.48 version of GLib, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 2.48
 */
#define GLIB_VERSION_2_48 (G_ENCODE_VERSION(2, 48))
/**
 * GLIB_VERSION_2_50:
 *
 * A macro that evaluates to the 2.50 version of GLib, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 2.50
 */
#define GLIB_VERSION_2_50 (G_ENCODE_VERSION(2, 50))
/**
 * GLIB_VERSION_2_52:
 *
 * A macro that evaluates to the 2.52 version of GLib, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 2.52
 */
#define GLIB_VERSION_2_52 (G_ENCODE_VERSION(2, 52))
/**
 * GLIB_VERSION_2_54:
 *
 * A macro that evaluates to the 2.54 version of GLib, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 2.54
 */
#define GLIB_VERSION_2_54 (G_ENCODE_VERSION(2, 54))
/**
 * GLIB_VERSION_2_56:
 *
 * A macro that evaluates to the 2.56 version of GLib, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 2.56
 */
#define GLIB_VERSION_2_56 (G_ENCODE_VERSION(2, 56))
/**
 * GLIB_VERSION_2_58:
 *
 * A macro that evaluates to the 2.58 version of GLib, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 2.58
 */
#define GLIB_VERSION_2_58 (G_ENCODE_VERSION(2, 58))
/**
 * GLIB_VERSION_2_60:
 *
 * A macro that evaluates to the 2.60 version of GLib, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 2.60
 */
#define GLIB_VERSION_2_60 (G_ENCODE_VERSION(2, 60))
/**
 * GLIB_VERSION_2_62:
 *
 * A macro that evaluates to the 2.62 version of GLib, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 2.62
 */
#define GLIB_VERSION_2_62 (G_ENCODE_VERSION(2, 62))
/**
 * GLIB_VERSION_2_64:
 *
 * A macro that evaluates to the 2.64 version of GLib, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 2.64
 */
#define GLIB_VERSION_2_64 (G_ENCODE_VERSION(2, 64))
/**
 * GLIB_VERSION_2_66:
 *
 * A macro that evaluates to the 2.66 version of GLib, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 2.66
 */
#define GLIB_VERSION_2_66 (G_ENCODE_VERSION(2, 66))
/**
 * GLIB_VERSION_2_68:
 *
 * A macro that evaluates to the 2.68 version of GLib, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 2.68
 */
#define GLIB_VERSION_2_68 (G_ENCODE_VERSION(2, 68))
/**
 * GLIB_VERSION_2_70:
 *
 * A macro that evaluates to the 2.70 version of GLib, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 2.70
 */
#define GLIB_VERSION_2_70 (G_ENCODE_VERSION(2, 70))
/**
 * GLIB_VERSION_2_72:
 *
 * A macro that evaluates to the 2.72 version of GLib, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 2.72
 */
#define GLIB_VERSION_2_72 (G_ENCODE_VERSION(2, 72))
/**
 * GLIB_VERSION_2_74:
 *
 * A macro that evaluates to the 2.74 version of GLib, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 2.74
 */
#define GLIB_VERSION_2_74 (G_ENCODE_VERSION(2, 74))
/**
 * GLIB_VERSION_2_76:
 *
 * A macro that evaluates to the 2.76 version of GLib, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 2.76
 */
#define GLIB_VERSION_2_76 (G_ENCODE_VERSION(2, 76))
/**
 * GLIB_VERSION_2_78:
 *
 * A macro that evaluates to the 2.78 version of GLib, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 2.78
 */
#define GLIB_VERSION_2_78 (G_ENCODE_VERSION(2, 78))
/**
 * GLIB_VERSION_2_80:
 *
 * A macro that evaluates to the 2.80 version of GLib, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 2.80
 */
#define GLIB_VERSION_2_80 (G_ENCODE_VERSION(2, 80))
/**
 * GLIB_VERSION_2_82:
 *
 * A macro that evaluates to the 2.82 version of GLib, in a format
 * that can be used by the C pre-processor.
 *
 * Since: 2.82
 */
#define GLIB_VERSION_2_82 (G_ENCODE_VERSION(2, 82))

/**
 * GLIB_VERSION_CUR_STABLE:
 *
 * A macro that evaluates to the current stable version of GLib, in a format
 * that can be used by the C pre-processor.
 *
 * During an unstable development cycle, this evaluates to the next stable
 * (unreleased) version which will be the result of the development cycle.
 *
 * Since: 2.32
 */
#if (GLIB_MINOR_VERSION % 2)
#define GLIB_VERSION_CUR_STABLE \
  (G_ENCODE_VERSION(GLIB_MAJOR_VERSION, GLIB_MINOR_VERSION + 1))
#else
#define GLIB_VERSION_CUR_STABLE \
  (G_ENCODE_VERSION(GLIB_MAJOR_VERSION, GLIB_MINOR_VERSION))
#endif

/**
 * GLIB_VERSION_PREV_STABLE:
 *
 * A macro that evaluates to the previous stable version of GLib, in a format
 * that can be used by the C pre-processor.
 *
 * During an unstable development cycle, this evaluates to the most recent
 * released stable release, which preceded this development cycle.
 *
 * Since: 2.32
 */
#if (GLIB_MINOR_VERSION % 2)
#define GLIB_VERSION_PREV_STABLE \
  (G_ENCODE_VERSION(GLIB_MAJOR_VERSION, GLIB_MINOR_VERSION - 1))
#else
#define GLIB_VERSION_PREV_STABLE \
  (G_ENCODE_VERSION(GLIB_MAJOR_VERSION, GLIB_MINOR_VERSION - 2))
#endif

/**
 * GLIB_VERSION_MIN_REQUIRED:
 *
 * A macro that should be defined by the user prior to including
 * the glib.h header.
 * The definition should be one of the predefined GLib version
 * macros: %GLIB_VERSION_2_26, %GLIB_VERSION_2_28,...
 *
 * This macro defines the earliest version of GLib that the package is
 * required to be able to compile against.
 *
 * If the compiler is configured to warn about the use of deprecated
 * functions, then using functions that were deprecated in version
 * %GLIB_VERSION_MIN_REQUIRED or earlier will cause warnings (but
 * using functions deprecated in later releases will not).
 *
 * Since: 2.32
 */
/* If the package sets GLIB_VERSION_MIN_REQUIRED to some future
 * GLIB_VERSION_X_Y value that we don't know about, it will compare as
 * 0 in preprocessor tests.
 */
#ifndef GLIB_VERSION_MIN_REQUIRED
#define GLIB_VERSION_MIN_REQUIRED (GLIB_VERSION_CUR_STABLE)
#elif GLIB_VERSION_MIN_REQUIRED == 0
#undef GLIB_VERSION_MIN_REQUIRED
#define GLIB_VERSION_MIN_REQUIRED (GLIB_VERSION_CUR_STABLE + 2)
#endif

/**
 * GLIB_VERSION_MAX_ALLOWED:
 *
 * A macro that should be defined by the user prior to including
 * the glib.h header.
 * The definition should be one of the predefined GLib version
 * macros: %GLIB_VERSION_2_26, %GLIB_VERSION_2_28,...
 *
 * This macro defines the latest version of the GLib API that the
 * package is allowed to make use of.
 *
 * If the compiler is configured to warn about the use of deprecated
 * functions, then using functions added after version
 * %GLIB_VERSION_MAX_ALLOWED will cause warnings.
 *
 * Unless you are using GLIB_CHECK_VERSION() or the like to compile
 * different code depending on the GLib version, then this should be
 * set to the same value as %GLIB_VERSION_MIN_REQUIRED.
 *
 * Since: 2.32
 */
#if !defined(GLIB_VERSION_MAX_ALLOWED) || (GLIB_VERSION_MAX_ALLOWED == 0)
#undef GLIB_VERSION_MAX_ALLOWED
#define GLIB_VERSION_MAX_ALLOWED (GLIB_VERSION_CUR_STABLE)
#endif

/* sanity checks */
#if GLIB_VERSION_MIN_REQUIRED > GLIB_VERSION_CUR_STABLE
#error "GLIB_VERSION_MIN_REQUIRED must be <= GLIB_VERSION_CUR_STABLE"
#endif
#if GLIB_VERSION_MAX_ALLOWED < GLIB_VERSION_MIN_REQUIRED
#error "GLIB_VERSION_MAX_ALLOWED must be >= GLIB_VERSION_MIN_REQUIRED"
#endif
#if GLIB_VERSION_MIN_REQUIRED < GLIB_VERSION_2_26
#error "GLIB_VERSION_MIN_REQUIRED must be >= GLIB_VERSION_2_26"
#endif

#endif /*  __G_VERSION_MACROS_H__ */
