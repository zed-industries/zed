/* Macros to control TS 18661-3 glibc features where the same
   definitions are appropriate for all platforms.
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

#ifndef _BITS_FLOATN_COMMON_H
#define _BITS_FLOATN_COMMON_H

#include <features.h>
#include <bits/long-double.h>

/* This header should be included at the bottom of each bits/floatn.h.
   It defines the following macros for each _FloatN and _FloatNx type,
   where the same definitions, or definitions based only on the macros
   in bits/floatn.h, are appropriate for all glibc configurations.  */

/* Defined to 1 if the current compiler invocation provides a
   floating-point type with the right format for this type, and this
   glibc includes corresponding *fN or *fNx interfaces for it.  */
#define __HAVE_FLOAT16 0
#define __HAVE_FLOAT32 1
#define __HAVE_FLOAT64 1
#define __HAVE_FLOAT32X 1
#define __HAVE_FLOAT128X 0

/* Defined to 1 if the corresponding __HAVE_<type> macro is 1 and the
   type is the first with its format in the sequence of (the default
   choices for) float, double, long double, _Float16, _Float32,
   _Float64, _Float128, _Float32x, _Float64x, _Float128x for this
   glibc; that is, if functions present once per floating-point format
   rather than once per type are present for this type.

   All configurations supported by glibc have _Float32 the same format
   as float, _Float64 and _Float32x the same format as double, the
   _Float64x the same format as either long double or _Float128.  No
   configurations support _Float128x or, as of GCC 7, have compiler
   support for a type meeting the requirements for _Float128x.  */
#define __HAVE_DISTINCT_FLOAT16 __HAVE_FLOAT16
#define __HAVE_DISTINCT_FLOAT32 0
#define __HAVE_DISTINCT_FLOAT64 0
#define __HAVE_DISTINCT_FLOAT32X 0
#define __HAVE_DISTINCT_FLOAT64X 0
#define __HAVE_DISTINCT_FLOAT128X __HAVE_FLOAT128X

/* Defined to 1 if the corresponding _FloatN type is not binary compatible
   with the corresponding ISO C type in the current compilation unit as
   opposed to __HAVE_DISTINCT_FLOATN, which indicates the default types built
   in glibc.  */
#define __HAVE_FLOAT128_UNLIKE_LDBL (__HAVE_DISTINCT_FLOAT128	\
				     && __LDBL_MANT_DIG__ != 113)

/* Defined to 1 if any _FloatN or _FloatNx types that are not
   ABI-distinct are however distinct types at the C language level (so
   for the purposes of __builtin_types_compatible_p and _Generic).  */
#if __GNUC_PREREQ (7, 0) && !defined __cplusplus
# define __HAVE_FLOATN_NOT_TYPEDEF 1
#else
# define __HAVE_FLOATN_NOT_TYPEDEF 0
#endif

#ifndef __ASSEMBLER__

/* Defined to concatenate the literal suffix to be used with _FloatN
   or _FloatNx types, if __HAVE_<type> is 1.  The corresponding
   literal suffixes exist since GCC 7, for C only.  */
# if __HAVE_FLOAT16
#  if !__GNUC_PREREQ (7, 0) || defined __cplusplus
/* No corresponding suffix available for this type.  */
#   define __f16(x) ((_Float16) x##f)
#  else
#   define __f16(x) x##f16
#  endif
# endif

# if __HAVE_FLOAT32
#  if !__GNUC_PREREQ (7, 0) || defined __cplusplus
#   define __f32(x) x##f
#  else
#   define __f32(x) x##f32
#  endif
# endif

# if __HAVE_FLOAT64
#  if !__GNUC_PREREQ (7, 0) || defined __cplusplus
#   ifdef __NO_LONG_DOUBLE_MATH
#    define __f64(x) x##l
#   else
#    define __f64(x) x
#   endif
#  else
#   define __f64(x) x##f64
#  endif
# endif

# if __HAVE_FLOAT32X
#  if !__GNUC_PREREQ (7, 0) || defined __cplusplus
#   define __f32x(x) x
#  else
#   define __f32x(x) x##f32x
#  endif
# endif

# if __HAVE_FLOAT64X
#  if !__GNUC_PREREQ (7, 0) || defined __cplusplus
#   if __HAVE_FLOAT64X_LONG_DOUBLE
#    define __f64x(x) x##l
#   else
#    define __f64x(x) __f128 (x)
#   endif
#  else
#   define __f64x(x) x##f64x
#  endif
# endif

# if __HAVE_FLOAT128X
#  if !__GNUC_PREREQ (7, 0) || defined __cplusplus
#   error "_Float128X supported but no constant suffix"
#  else
#   define __f128x(x) x##f128x
#  endif
# endif

/* Defined to a complex type if __HAVE_<type> is 1.  */
# if __HAVE_FLOAT16
#  if !__GNUC_PREREQ (7, 0) || defined __cplusplus
typedef _Complex float __cfloat16 __attribute__ ((__mode__ (__HC__)));
#   define __CFLOAT16 __cfloat16
#  else
#   define __CFLOAT16 _Complex _Float16
#  endif
# endif

# if __HAVE_FLOAT32
#  if !__GNUC_PREREQ (7, 0) || defined __cplusplus
#   define __CFLOAT32 _Complex float
#  else
#   define __CFLOAT32 _Complex _Float32
#  endif
# endif

# if __HAVE_FLOAT64
#  if !__GNUC_PREREQ (7, 0) || defined __cplusplus
#   ifdef __NO_LONG_DOUBLE_MATH
#    define __CFLOAT64 _Complex long double
#   else
#    define __CFLOAT64 _Complex double
#   endif
#  else
#   define __CFLOAT64 _Complex _Float64
#  endif
# endif

# if __HAVE_FLOAT32X
#  if !__GNUC_PREREQ (7, 0) || defined __cplusplus
#   define __CFLOAT32X _Complex double
#  else
#   define __CFLOAT32X _Complex _Float32x
#  endif
# endif

# if __HAVE_FLOAT64X
#  if !__GNUC_PREREQ (7, 0) || defined __cplusplus
#   if __HAVE_FLOAT64X_LONG_DOUBLE
#    define __CFLOAT64X _Complex long double
#   else
#    define __CFLOAT64X __CFLOAT128
#   endif
#  else
#   define __CFLOAT64X _Complex _Float64x
#  endif
# endif

# if __HAVE_FLOAT128X
#  if !__GNUC_PREREQ (7, 0) || defined __cplusplus
#   error "_Float128X supported but no complex type"
#  else
#   define __CFLOAT128X _Complex _Float128x
#  endif
# endif

/* The remaining of this file provides support for older compilers.  */
# if __HAVE_FLOAT16

#  if !__GNUC_PREREQ (7, 0) || defined __cplusplus
typedef float _Float16 __attribute__ ((__mode__ (__HF__)));
#  endif

#  if !__GNUC_PREREQ (7, 0)
#   define __builtin_huge_valf16() ((_Float16) __builtin_huge_val ())
#   define __builtin_inff16() ((_Float16) __builtin_inf ())
#   define __builtin_nanf16(x) ((_Float16) __builtin_nan (x))
#   define __builtin_nansf16(x) ((_Float16) __builtin_nans (x))
#  endif

# endif

# if __HAVE_FLOAT32

#  if !__GNUC_PREREQ (7, 0) || defined __cplusplus
typedef float _Float32;
#  endif

#  if !__GNUC_PREREQ (7, 0)
#   define __builtin_huge_valf32() (__builtin_huge_valf ())
#   define __builtin_inff32() (__builtin_inff ())
#   define __builtin_nanf32(x) (__builtin_nanf (x))
#   define __builtin_nansf32(x) (__builtin_nansf (x))
#  endif

# endif

# if __HAVE_FLOAT64

/* If double, long double and _Float64 all have the same set of
   values, TS 18661-3 requires the usual arithmetic conversions on
   long double and _Float64 to produce _Float64.  For this to be the
   case when building with a compiler without a distinct _Float64
   type, _Float64 must be a typedef for long double, not for
   double.  */

#  ifdef __NO_LONG_DOUBLE_MATH

#   if !__GNUC_PREREQ (7, 0) || defined __cplusplus
typedef long double _Float64;
#   endif

#   if !__GNUC_PREREQ (7, 0)
#    define __builtin_huge_valf64() (__builtin_huge_vall ())
#    define __builtin_inff64() (__builtin_infl ())
#    define __builtin_nanf64(x) (__builtin_nanl (x))
#    define __builtin_nansf64(x) (__builtin_nansl (x))
#   endif

#  else

#   if !__GNUC_PREREQ (7, 0) || defined __cplusplus
typedef double _Float64;
#   endif

#   if !__GNUC_PREREQ (7, 0)
#    define __builtin_huge_valf64() (__builtin_huge_val ())
#    define __builtin_inff64() (__builtin_inf ())
#    define __builtin_nanf64(x) (__builtin_nan (x))
#    define __builtin_nansf64(x) (__builtin_nans (x))
#   endif

#  endif

# endif

# if __HAVE_FLOAT32X

#  if !__GNUC_PREREQ (7, 0) || defined __cplusplus
typedef double _Float32x;
#  endif

#  if !__GNUC_PREREQ (7, 0)
#   define __builtin_huge_valf32x() (__builtin_huge_val ())
#   define __builtin_inff32x() (__builtin_inf ())
#   define __builtin_nanf32x(x) (__builtin_nan (x))
#   define __builtin_nansf32x(x) (__builtin_nans (x))
#  endif

# endif

# if __HAVE_FLOAT64X

#  if __HAVE_FLOAT64X_LONG_DOUBLE

#   if !__GNUC_PREREQ (7, 0) || defined __cplusplus
typedef long double _Float64x;
#   endif

#   if !__GNUC_PREREQ (7, 0)
#    define __builtin_huge_valf64x() (__builtin_huge_vall ())
#    define __builtin_inff64x() (__builtin_infl ())
#    define __builtin_nanf64x(x) (__builtin_nanl (x))
#    define __builtin_nansf64x(x) (__builtin_nansl (x))
#   endif

#  else

#   if !__GNUC_PREREQ (7, 0) || defined __cplusplus
typedef _Float128 _Float64x;
#   endif

#   if !__GNUC_PREREQ (7, 0)
#    define __builtin_huge_valf64x() (__builtin_huge_valf128 ())
#    define __builtin_inff64x() (__builtin_inff128 ())
#    define __builtin_nanf64x(x) (__builtin_nanf128 (x))
#    define __builtin_nansf64x(x) (__builtin_nansf128 (x))
#   endif

#  endif

# endif

# if __HAVE_FLOAT128X

#  if !__GNUC_PREREQ (7, 0) || defined __cplusplus
#   error "_Float128x supported but no type"
#  endif

#  if !__GNUC_PREREQ (7, 0)
#   define __builtin_huge_valf128x() ((_Float128x) __builtin_huge_val ())
#   define __builtin_inff128x() ((_Float128x) __builtin_inf ())
#   define __builtin_nanf128x(x) ((_Float128x) __builtin_nan (x))
#   define __builtin_nansf128x(x) ((_Float128x) __builtin_nans (x))
#  endif

# endif

#endif /* !__ASSEMBLER__.  */

#endif /* _BITS_FLOATN_COMMON_H */
