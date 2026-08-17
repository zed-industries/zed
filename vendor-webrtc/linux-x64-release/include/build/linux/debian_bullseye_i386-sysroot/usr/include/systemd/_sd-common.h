/* SPDX-License-Identifier: LGPL-2.1-or-later */
#ifndef foosdcommonhfoo
#define foosdcommonhfoo

/***
  systemd is free software; you can redistribute it and/or modify it
  under the terms of the GNU Lesser General Public License as published by
  the Free Software Foundation; either version 2.1 of the License, or
  (at your option) any later version.

  systemd is distributed in the hope that it will be useful, but
  WITHOUT ANY WARRANTY; without even the implied warranty of
  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
  Lesser General Public License for more details.

  You should have received a copy of the GNU Lesser General Public License
  along with systemd; If not, see <https://www.gnu.org/licenses/>.
***/

/* This is a private header; never even think of including this directly! */

#if defined(__INCLUDE_LEVEL__) && __INCLUDE_LEVEL__ <= 1 && !defined(__COVERITY__)
#  error "Do not include _sd-common.h directly; it is a private header."
#endif

typedef void (*_sd_destroy_t)(void *userdata);

#ifndef _sd_printf_
#  if __GNUC__ >= 4
#    define _sd_printf_(a,b) __attribute__((__format__(printf, a, b)))
#  else
#    define _sd_printf_(a,b)
#  endif
#endif

#ifndef _sd_sentinel_
#  define _sd_sentinel_ __attribute__((__sentinel__))
#endif

#ifndef _sd_packed_
#  define _sd_packed_ __attribute__((__packed__))
#endif

#ifndef _sd_pure_
#  define _sd_pure_ __attribute__((__pure__))
#endif

/* Note that strictly speaking __deprecated__ has been available before GCC 6. However, starting with GCC 6
 * it also works on enum values, which we are interested in. Since this is a developer-facing feature anyway
 * (as opposed to build engineer-facing), let's hence conditionalize this to gcc 6, given that the developers
 * are probably going to use something newer anyway. */
#ifndef _sd_deprecated_
#  if __GNUC__ >= 6
#    define _sd_deprecated_ __attribute__((__deprecated__))
#  else
#    define _sd_deprecated_
#  endif
#endif

#ifndef _SD_STRINGIFY
#  define _SD_XSTRINGIFY(x) #x
#  define _SD_STRINGIFY(x) _SD_XSTRINGIFY(x)
#endif

#ifndef _SD_BEGIN_DECLARATIONS
#  ifdef __cplusplus
#    define _SD_BEGIN_DECLARATIONS                              \
        extern "C" {                                            \
        struct _sd_useless_struct_to_allow_trailing_semicolon_
#  else
#    define _SD_BEGIN_DECLARATIONS                              \
        struct _sd_useless_struct_to_allow_trailing_semicolon_
#  endif
#endif

#ifndef _SD_END_DECLARATIONS
#  ifdef __cplusplus
#    define _SD_END_DECLARATIONS                                \
        }                                                       \
        struct _sd_useless_cpp_struct_to_allow_trailing_semicolon_
#  else
#    define _SD_END_DECLARATIONS                                \
        struct _sd_useless_struct_to_allow_trailing_semicolon_
#  endif
#endif

#ifndef _SD_ARRAY_STATIC
#  if __STDC_VERSION__ >= 199901L && !defined(__cplusplus)
#    define _SD_ARRAY_STATIC static
#  else
#    define _SD_ARRAY_STATIC
#  endif
#endif

#define _SD_DEFINE_POINTER_CLEANUP_FUNC(type, func)             \
        static __inline__ void func##p(type **p) {              \
                if (*p)                                         \
                        func(*p);                               \
        }                                                       \
        struct _sd_useless_struct_to_allow_trailing_semicolon_

/* The following macro should be used in all public enums, to force 64bit wideness on them, so that we can
 * freely extend them later on, without breaking compatibility. */
#define _SD_ENUM_FORCE_S64(id)               \
        _SD_##id##_INT64_MIN = INT64_MIN,    \
        _SD_##id##_INT64_MAX = INT64_MAX

#endif
