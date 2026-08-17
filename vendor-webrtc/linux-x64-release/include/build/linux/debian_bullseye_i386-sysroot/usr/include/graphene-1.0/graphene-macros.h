/* graphene-macros.h: Public macros
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

#ifndef _GRAPHENE_PUBLIC
#define _GRAPHENE_PUBLIC        extern
#endif

#ifdef GRAPHENE_COMPILATION
# define GRAPHENE_PRIVATE_FIELD(type,name)      type name
#else
# define GRAPHENE_PRIVATE_FIELD(type,name)      type __graphene_private_##name
#endif

#if defined(__GNUC__)
# define GRAPHENE_ALIGN16  __attribute__((aligned(16)))
# define GRAPHENE_ALIGNED_DECL(x,val)   x __attribute__((aligned(val)))
#elif defined(_MSC_VER)
# define GRAPHENE_ALIGN16  __declspec(align(16))
# define GRAPHENE_ALIGNED_DECL(x,val)   __declspec(align(val)) x
#else
# define GRAPHENE_ALIGN16
# define GRAPHENE_ALIGNED_DECL(x,val)   x
#endif

#if defined(_MSC_VER) && (_MSC_VER >= 1910)
# ifdef _M_IX86
/* Use __vectorcall to enable SSE intrinsics on 32-bit builds on MSVC 2013 and later */
#  define GRAPHENE_VECTORCALL   __vectorcall
# else
#  define GRAPHENE_VECTORCALL
# endif
#else
# define GRAPHENE_VECTORCALL
#endif

#ifdef _MSC_VER
# if !defined (__cplusplus) && defined (_MSC_VER) && (_MSC_VER < 1900)
#  define inline __inline
# endif
#endif

#ifdef __cplusplus
# define GRAPHENE_BEGIN_DECLS   extern "C" {
# define GRAPHENE_END_DECLS     }
#else
# define GRAPHENE_BEGIN_DECLS
# define GRAPHENE_END_DECLS
#endif

#if defined(_MSC_VER) && !defined(__bool_true_false_are_defined) && (_MSC_VER < 1800)
# ifndef __GI_SCANNER__
typedef int bool;
#  define false 0
#  define true 1
# endif
#else
# include <stdbool.h>
#endif

#if __GNUC__ > 3 || (__GNUC__ == 3 && __GNUC_MINOR__ >= 1)
#define _GRAPHENE_DEPRECATED __attribute__((__deprecated__))
#elif defined(_MSC_VER) && (_MSC_VER >= 1300)
#define _GRAPHENE_DEPRECATED __declspec(deprecated)
#else
#define _GRAPHENE_DEPRECATED
#endif

#if __GNUC__ > 4 || (__GNUC__ == 4 && __GNUC_MINOR__ >= 5)
#define _GRAPHENE_DEPRECATED_FOR(f) __attribute__((__deprecated__("Use '" #f "' instead")))
#elif defined(_MSC_FULL_VER) && (_MSC_FULL_VER > 140050320)
#define _GRAPHENE_DEPRECATED_FOR(f) __declspec(deprecated("is deprecated. Use '" #f "' instead"))
#else
#define _GRAPHENE_DEPRECATED_FOR(f) _GRAPHENE_DEPRECATED
#endif

#if __GNUC__ > 4 || (__GNUC__ == 4 && __GNUC_MINOR__ >= 5)
#define _GRAPHENE_UNAVAILABLE(maj,min) __attribute__((deprecated("Not available before " #maj "." #min)))
#elif defined(_MSC_FULL_VER) && (_MSC_FULL_VER > 140050320)
#define _GRAPHENE_UNAVAILABLE(maj,min) __declspec(deprecated("is not available before " #maj "." #min))
#else
#define _GRAPHENE_UNAVAILABLE(maj,min) _GRAPHENE_DEPRECATED
#endif

#define GRAPHENE_PI             3.1415926535897932384626434f
#define GRAPHENE_PI_2           1.5707963267948966192313217f
