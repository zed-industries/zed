/* Macros to control TS 18661-3 glibc features on x86.
   Copyright (C) 2017-2020 Free Software Foundation, Inc.
   This file is part of the GNU C Library.

   The GNU C Library is free software; you can redistribute it and/or
   modify it under the terms of the GNU Lesser General Public
   License as published by the Free Software Foundation; either
   version 2.1 of the License, or (at your option) any later version.

   The GNU C Library is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public
   License along with the GNU C Library; if not, see
   <https://www.gnu.org/licenses/>.  */

#ifndef _BITS_FLOATN_H
#define _BITS_FLOATN_H

#include <features.h>

/* Defined to 1 if the current compiler invocation provides a
   floating-point type with the IEEE 754 binary128 format, and this
   glibc includes corresponding *f128 interfaces for it.  The required
   libgcc support was added some time after the basic compiler
   support, for x86_64 and x86.  */
#if (defined __x86_64__							\
     ? __GNUC_PREREQ (4, 3)						\
     : (defined __GNU__ ? __GNUC_PREREQ (4, 5) : __GNUC_PREREQ (4, 4))) \
     &&  !defined(__CUDACC__) && !defined(__ICC)
# define __HAVE_FLOAT128 1
#else
# define __HAVE_FLOAT128 0
#endif

/* Defined to 1 if __HAVE_FLOAT128 is 1 and the type is ABI-distinct
   from the default float, double and long double types in this glibc.  */
#if __HAVE_FLOAT128
# define __HAVE_DISTINCT_FLOAT128 1
#else
# define __HAVE_DISTINCT_FLOAT128 0
#endif

/* Defined to 1 if the current compiler invocation provides a
   floating-point type with the right format for _Float64x, and this
   glibc includes corresponding *f64x interfaces for it.  */
#define __HAVE_FLOAT64X 1

/* Defined to 1 if __HAVE_FLOAT64X is 1 and _Float64x has the format
   of long double.  Otherwise, if __HAVE_FLOAT64X is 1, _Float64x has
   the format of _Float128, which must be different from that of long
   double.  */
#define __HAVE_FLOAT64X_LONG_DOUBLE 1

#ifndef __ASSEMBLER__

/* Defined to concatenate the literal suffix to be used with _Float128
   types, if __HAVE_FLOAT128 is 1. */
# if __HAVE_FLOAT128
#  if !__GNUC_PREREQ (7, 0) || defined __cplusplus
/* The literal suffix f128 exists only since GCC 7.0.  */
#   define __f128(x) x##q
#  else
#   define __f128(x) x##f128
#  endif
# endif

/* Defined to a complex binary128 type if __HAVE_FLOAT128 is 1.  */
# if __HAVE_FLOAT128
#  if !__GNUC_PREREQ (7, 0) || defined __cplusplus
/* Add a typedef for older GCC compilers which don't natively support
   _Complex _Float128.  */
typedef _Complex float __cfloat128 __attribute__ ((__mode__ (__TC__)));
#   define __CFLOAT128 __cfloat128
#  else
#   define __CFLOAT128 _Complex _Float128
#  endif
# endif

/* The remaining of this file provides support for older compilers.  */
# if __HAVE_FLOAT128

/* The type _Float128 exists only since GCC 7.0.  */
#  if !__GNUC_PREREQ (7, 0) || defined __cplusplus
typedef __float128 _Float128;
#  endif

/* __builtin_huge_valf128 doesn't exist before GCC 7.0.  */
#  if !__GNUC_PREREQ (7, 0)
#   define __builtin_huge_valf128() ((_Float128) __builtin_huge_val ())
#  endif

/* Older GCC has only a subset of built-in functions for _Float128 on
   x86, and __builtin_infq is not usable in static initializers.
   Converting a narrower sNaN to _Float128 produces a quiet NaN, so
   attempts to use _Float128 sNaNs will not work properly with older
   compilers.  */
#  if !__GNUC_PREREQ (7, 0)
#   define __builtin_copysignf128 __builtin_copysignq
#   define __builtin_fabsf128 __builtin_fabsq
#   define __builtin_inff128() ((_Float128) __builtin_inf ())
#   define __builtin_nanf128(x) ((_Float128) __builtin_nan (x))
#   define __builtin_nansf128(x) ((_Float128) __builtin_nans (x))
#  endif

/* In math/math.h, __MATH_TG will expand signbit to __builtin_signbit*,
   e.g.: __builtin_signbitf128, before GCC 6.  However, there has never
   been a __builtin_signbitf128 in GCC and the type-generic builtin is
   only available since GCC 6.  */
#  if !__GNUC_PREREQ (6, 0)
#   define __builtin_signbitf128 __signbitf128
#  endif

# endif

#endif /* !__ASSEMBLER__.  */

#include <bits/floatn-common.h>

#endif /* _BITS_FLOATN_H */
