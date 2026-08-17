/*
 * copyright (c) 2006 Michael Niedermayer <michaelni@gmx.at>
 *
 * This file is part of FFmpeg.
 *
 * FFmpeg is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * FFmpeg is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with FFmpeg; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA
 */

/**
 * @file
 * common internal API header
 */

#ifndef AVUTIL_INTERNAL_H
#define AVUTIL_INTERNAL_H

#if !defined(DEBUG) && !defined(NDEBUG)
#    define NDEBUG
#endif

// This can be enabled to allow detection of additional integer overflows with ubsan
//#define CHECKED

#include <limits.h>
#include <stdint.h>
#include <stddef.h>
#include <assert.h>
#include <stdio.h>
#include "config.h"
#include "attributes.h"
#include "libm.h"
#include "macros.h"

#ifndef attribute_align_arg
#if ARCH_X86_32 && AV_GCC_VERSION_AT_LEAST(4,2)
#    define attribute_align_arg __attribute__((force_align_arg_pointer))
#else
#    define attribute_align_arg
#endif
#endif

#if defined(_WIN32) && CONFIG_SHARED && !defined(BUILDING_avutil)
#    define av_export_avutil __declspec(dllimport)
#else
#    define av_export_avutil
#endif

#if HAVE_PRAGMA_DEPRECATED
#    if defined(__ICL) || defined (__INTEL_COMPILER)
#        define FF_DISABLE_DEPRECATION_WARNINGS __pragma(warning(push)) __pragma(warning(disable:1478))
#        define FF_ENABLE_DEPRECATION_WARNINGS  __pragma(warning(pop))
#    elif defined(_MSC_VER)
#        define FF_DISABLE_DEPRECATION_WARNINGS __pragma(warning(push)) __pragma(warning(disable:4996))
#        define FF_ENABLE_DEPRECATION_WARNINGS  __pragma(warning(pop))
#    else
#        define FF_DISABLE_DEPRECATION_WARNINGS _Pragma("GCC diagnostic push") _Pragma("GCC diagnostic ignored \"-Wdeprecated-declarations\"")
#        define FF_ENABLE_DEPRECATION_WARNINGS  _Pragma("GCC diagnostic pop")
#    endif
#else
#    define FF_DISABLE_DEPRECATION_WARNINGS
#    define FF_ENABLE_DEPRECATION_WARNINGS
#endif


#define FF_ALLOC_TYPED_ARRAY(p, nelem)  (p = av_malloc_array(nelem, sizeof(*p)))
#define FF_ALLOCZ_TYPED_ARRAY(p, nelem) (p = av_calloc(nelem, sizeof(*p)))

#define FF_PTR_ADD(ptr, off) ((off) ? (ptr) + (off) : (ptr))

/**
 * Access a field in a structure by its offset.
 */
#define FF_FIELD_AT(type, off, obj) (*(type *)((char *)&(obj) + (off)))

/**
 * Return NULL if CONFIG_SMALL is true, otherwise the argument
 * without modification. Used to disable the definition of strings.
 */
#if CONFIG_SMALL
#   define NULL_IF_CONFIG_SMALL(x) NULL
#else
#   define NULL_IF_CONFIG_SMALL(x) x
#endif

/**
 * Log a generic warning message about a missing feature.
 *
 * @param[in] avc a pointer to an arbitrary struct of which the first
 *                field is a pointer to an AVClass struct
 * @param[in] msg string containing the name of the missing feature
 */
#if defined(CHROMIUM_NO_LOGGING)
#define avpriv_report_missing_feature(...)
#else
void avpriv_report_missing_feature(void *avc,
                                   const char *msg, ...) av_printf_format(2, 3);
#endif

/**
 * Log a generic warning message about a missing feature.
 * Additionally request that a sample showcasing the feature be uploaded.
 *
 * @param[in] avc a pointer to an arbitrary struct of which the first field is
 *                a pointer to an AVClass struct
 * @param[in] msg string containing the name of the missing feature
 */
#if defined(CHROMIUM_NO_LOGGING)
#define avpriv_request_sample(...)
#else
void avpriv_request_sample(void *avc,
                           const char *msg, ...) av_printf_format(2, 3);
#endif

#if HAVE_LIBC_MSVCRT
#include <crtversion.h>
#if defined(_VC_CRT_MAJOR_VERSION) && _VC_CRT_MAJOR_VERSION < 14
#pragma comment(linker, "/include:" EXTERN_PREFIX "avpriv_strtod")
#pragma comment(linker, "/include:" EXTERN_PREFIX "avpriv_snprintf")
#endif

#define PTRDIFF_SPECIFIER "Id"
#define SIZE_SPECIFIER "Iu"
#else
#define PTRDIFF_SPECIFIER "td"
#define SIZE_SPECIFIER "zu"
#endif

#ifdef DEBUG
#   define ff_dlog(ctx, ...) av_log(ctx, AV_LOG_DEBUG, __VA_ARGS__)
#elif defined(CHROMIUM_NO_LOGGING)
#   define ff_dlog(ctx, ...) do { } while(0) // Prevent a [-Wempty-body] error.
#else
#   define ff_dlog(ctx, ...) do { if (0) av_log(ctx, AV_LOG_DEBUG, __VA_ARGS__); } while (0)
#endif

#ifdef TRACE
#   define ff_tlog(ctx, ...) av_log(ctx, AV_LOG_TRACE, __VA_ARGS__)
#else
#   define ff_tlog(ctx, ...) do { } while(0)
#endif

// For debuging we use signed operations so overflows can be detected (by ubsan)
// For production we use unsigned so there are no undefined operations
#ifdef CHECKED
#define SUINT   int
#define SUINT32 int32_t
#else
#define SUINT   unsigned
#define SUINT32 uint32_t
#endif

static av_always_inline av_const int avpriv_mirror(int x, int w)
{
    if (!w)
        return 0;

    while ((unsigned)x > (unsigned)w) {
        x = -x;
        if (x < 0)
            x += 2 * w;
    }
    return x;
}

#endif /* AVUTIL_INTERNAL_H */
