/* graphene-version.h: Versioning macros
 *
 * SPDX-License-Identifier: MIT
 *
 * Copyright 2014  Emmanuele Bassi
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
 * THE SOFTWARE.
 */

#pragma once

#if !defined(GRAPHENE_H_INSIDE) && !defined(GRAPHENE_COMPILATION)
#error "Only graphene.h can be included directly."
#endif

#include "graphene-version.h"

/**
 * GRAPHENE_ENCODE_VERSION:
 * @major: a major version
 * @minor: a minor version
 * @micro: a micro version
 *
 * Encodes the given components into a value that can be used for
 * version checks.
 *
 * Since: 1.0
 */
#define GRAPHENE_ENCODE_VERSION(major,minor,micro) \
  ((major) << 24 | (minor) << 16 | (micro) << 8)

#define _GRAPHENE_ENCODE_VERSION(maj,min) \
  ((maj) << 16 | (min) << 8)

#ifdef GRAPHENE_DISABLE_DEPRECATION_WARNINGS
# define GRAPHENE_DEPRECATED            _GRAPHENE_PUBLIC
# define GRAPHENE_DEPRECATED_FOR(f)     _GRAPHENE_PUBLIC
# define GRAPHENE_UNAVAILABLE(maj,min)  _GRAPHENE_PUBLIC
#else
# define GRAPHENE_DEPRECATED            _GRAPHENE_DEPRECATED _GRAPHENE_PUBLIC
# define GRAPHENE_DEPRECATED_FOR(f)     _GRAPHENE_DEPRECATED_FOR(f) _GRAPHENE_PUBLIC
# define GRAPHENE_UNAVAILABLE(maj,min)  _GRAPHENE_UNAVAILABLE(maj,min) _GRAPHENE_PUBLIC
#endif

/**
 * GRAPHENE_VERSION:
 *
 * The current version of the library, as encoded through
 * the %GRAPHENE_ENCODE_VERSION macro. Can be used for version
 * checking, for instance:
 *
 * |[<!-- language="C" -->
 *   #if GRAPHENE_VERSION >= GRAPHENE_ENCODE_VERSION (1, 2, 3)
 *     // code that uses API introduced after version 1.2.3
 *   #endif
 * ]|
 *
 * Since: 1.0
 */
#define GRAPHENE_VERSION                \
  GRAPHENE_ENCODE_VERSION (GRAPHENE_MAJOR_VERSION, \
                           GRAPHENE_MINOR_VERSION, \
                           GRAPHENE_MICRO_VERSION)

#define GRAPHENE_CHECK_VERSION(major,minor,micro) \
  ((major) > GRAPHENE_MAJOR_VERSION || \
   (major) == GRAPHENE_MAJOR_VERSION && (minor) > GRAPHENE_MINOR_VERSION || \
   (major) == GRAPHENE_MAJOR_VERSION && (minor) == GRAPHENE_MINOR_VERSION && (micro) >= GRAPHENE_MICRO_VERSION)

/* evaluates to the current stable release; for development cycles
 * this means the next stable target.
 */
#if (GRAPHENE_MINOR_VERSION == 99)
# define GRAPHENE_VERSION_CUR_STABLE    (_GRAPHENE_ENCODE_VERSION (GRAPHENE_MAJOR_VERSION + 1, 0))
#elif (GRAPHENE_MINOR_VERSION % 2)
# define GRAPHENE_VERSION_CUR_STABLE    (_GRAPHENE_ENCODE_VERSION (GRAPHENE_MAJOR_VERSION, GRAPHENE_MINOR_VERSION + 1))
#else
# define GRAPHENE_VERSION_CUR_STABLE    (_GRAPHENE_ENCODE_VERSION (GRAPHENE_MAJOR_VERSION, GRAPHENE_MINOR_VERSION))
#endif

/* evaluates to the previous stable version */
#if (GRAPHENE_MINOR_VERSION == 99)
# define GRAPHENE_VERSION_PREV_STABLE   (_GRAPHENE_ENCODE_VERSION (GRAPHENE_MAJOR_VERSION + 1, 0))
#elif (GRAPHENE_MINOR_VERSION % 2)
# define GRAPHENE_VERSION_PREV_STABLE   (_GRAPHENE_ENCODE_VERSION (GRAPHENE_MAJOR_VERSION, GRAPHENE_MINOR_VERSION - 1))
#else
# define GRAPHENE_VERSION_PREV_STABLE   (_GRAPHENE_ENCODE_VERSION (GRAPHENE_MAJOR_VERSION, GRAPHENE_MINOR_VERSION - 2))
#endif

/* version defines
 *
 * remember to add new macros at the beginning of each development cycle
 */

#define GRAPHENE_VERSION_1_0    (_GRAPHENE_ENCODE_VERSION (1, 0))
#define GRAPHENE_VERSION_1_2    (_GRAPHENE_ENCODE_VERSION (1, 2))
#define GRAPHENE_VERSION_1_4    (_GRAPHENE_ENCODE_VERSION (1, 4))
#define GRAPHENE_VERSION_1_6    (_GRAPHENE_ENCODE_VERSION (1, 6))
#define GRAPHENE_VERSION_1_8    (_GRAPHENE_ENCODE_VERSION (1, 8))
#define GRAPHENE_VERSION_1_10   (_GRAPHENE_ENCODE_VERSION (1, 10))

#ifndef GRAPHENE_VERSION_MIN_REQUIRED
# define GRAPHENE_VERSION_MIN_REQUIRED  (GRAPHENE_VERSION_1_0)
#endif

#ifndef GRAPHENE_VERSION_MAX_ALLOWED
# if GRAPHENE_VERSION_MIN_REQUIRED > GRAPHENE_VERSION_PREV_STABLE
#  define GRAPHENE_VERSION_MAX_ALLOWED  (GRAPHENE_VERSION_MIN_REQUIRED)
# else
#  define GRAPHENE_VERSION_MAX_ALLOWED  (GRAPHENE_VERSION_CUR_STABLE)
# endif
#endif

/* sanity checks */
#if GRAPHENE_VERSION_MAX_ALLOWED < GRAPHENE_VERSION_MIN_REQUIRED
# error "GRAPHENE_VERSION_MAX_ALLOWED must be >= GRAPHENE_VERSION_MIN_REQUIRED"
#endif
#if GRAPHENE_VERSION_MIN_REQUIRED < GRAPHENE_VERSION_1_0
# error "GRAPHENE_VERSION_MIN_REQUIRED must be >= GRAPHENE_VERSION_1_0"
#endif

/* unconditional */
#define GRAPHENE_DEPRECATED_IN_1_0              GRAPHENE_DEPRECATED
#define GRAPHENE_DEPRECATED_IN_1_0_FOR(f)       GRAPHENE_DEPRECATED_FOR(f)
#define GRAPHENE_AVAILABLE_IN_1_0               _GRAPHENE_PUBLIC

/* Graphene 1.2 */
#if GRAPHENE_VERSION_MIN_REQUIRED >= GRAPHENE_VERSION_1_2
# define GRAPHENE_DEPRECATED_IN_1_2             GRAPHENE_DEPRECATED
# define GRAPHENE_DEPRECATED_IN_1_2_FOR(f)      GRAPHENE_DEPRECATED_FOR(f)
#else
# define GRAPHENE_DEPRECATED_IN_1_2             _GRAPHENE_PUBLIC
# define GRAPHENE_DEPRECATED_IN_1_2_FOR(f)      _GRAPHENE_PUBLIC
#endif

#if GRAPHENE_VERSION_MAX_ALLOWED < GRAPHENE_VERSION_1_2
# define GRAPHENE_AVAILABLE_IN_1_2              GRAPHENE_UNAVAILABLE(1,2)
#else
# define GRAPHENE_AVAILABLE_IN_1_2              _GRAPHENE_PUBLIC
#endif

/* Graphene 1.4 */
#if GRAPHENE_VERSION_MIN_REQUIRED >= GRAPHENE_VERSION_1_4
# define GRAPHENE_DEPRECATED_IN_1_4             GRAPHENE_DEPRECATED
# define GRAPHENE_DEPRECATED_IN_1_4_FOR(f)      GRAPHENE_DEPRECATED_FOR(f)
#else
# define GRAPHENE_DEPRECATED_IN_1_4             _GRAPHENE_PUBLIC
# define GRAPHENE_DEPRECATED_IN_1_4_FOR(f)      _GRAPHENE_PUBLIC
#endif

#if GRAPHENE_VERSION_MAX_ALLOWED < GRAPHENE_VERSION_1_4
# define GRAPHENE_AVAILABLE_IN_1_4              GRAPHENE_UNAVAILABLE(1,4)
#else
# define GRAPHENE_AVAILABLE_IN_1_4              _GRAPHENE_PUBLIC
#endif

/* Graphene 1.6 */
#if GRAPHENE_VERSION_MIN_REQUIRED >= GRAPHENE_VERSION_1_6
# define GRAPHENE_DEPRECATED_IN_1_6             GRAPHENE_DEPRECATED
# define GRAPHENE_DEPRECATED_IN_1_6_FOR(f)      GRAPHENE_DEPRECATED_FOR(f)
#else
# define GRAPHENE_DEPRECATED_IN_1_6             _GRAPHENE_PUBLIC
# define GRAPHENE_DEPRECATED_IN_1_6_FOR(f)      _GRAPHENE_PUBLIC
#endif

#if GRAPHENE_VERSION_MAX_ALLOWED < GRAPHENE_VERSION_1_6
# define GRAPHENE_AVAILABLE_IN_1_6              GRAPHENE_UNAVAILABLE(1,6)
#else
# define GRAPHENE_AVAILABLE_IN_1_6              _GRAPHENE_PUBLIC
#endif

/* Graphene 1.8 */
#if GRAPHENE_VERSION_MIN_REQUIRED >= GRAPHENE_VERSION_1_8
# define GRAPHENE_DEPRECATED_IN_1_8             GRAPHENE_DEPRECATED
# define GRAPHENE_DEPRECATED_IN_1_8_FOR(f)      GRAPHENE_DEPRECATED_FOR(f)
#else
# define GRAPHENE_DEPRECATED_IN_1_8             _GRAPHENE_PUBLIC
# define GRAPHENE_DEPRECATED_IN_1_8_FOR(f)      _GRAPHENE_PUBLIC
#endif

#if GRAPHENE_VERSION_MAX_ALLOWED < GRAPHENE_VERSION_1_8
# define GRAPHENE_AVAILABLE_IN_1_8              GRAPHENE_UNAVAILABLE(1,8)
#else
# define GRAPHENE_AVAILABLE_IN_1_8              _GRAPHENE_PUBLIC
#endif

/* Graphene 1.10 */
#if GRAPHENE_VERSION_MIN_REQUIRED >= GRAPHENE_VERSION_1_10
# define GRAPHENE_DEPRECATED_IN_1_10            GRAPHENE_DEPRECATED
# define GRAPHENE_DEPRECATED_IN_1_10_FOR(f)     GRAPHENE_DEPRECATED_FOR(f)
#else
# define GRAPHENE_DEPRECATED_IN_1_10            _GRAPHENE_PUBLIC
# define GRAPHENE_DEPRECATED_IN_1_10_FOR(f)     _GRAPHENE_PUBLIC
#endif

#if GRAPHENE_VERSION_MAX_ALLOWED < GRAPHENE_VERSION_1_10
# define GRAPHENE_AVAILABLE_IN_1_10             GRAPHENE_UNAVAILABLE(1,10)
#else
# define GRAPHENE_AVAILABLE_IN_1_10             _GRAPHENE_PUBLIC
#endif
