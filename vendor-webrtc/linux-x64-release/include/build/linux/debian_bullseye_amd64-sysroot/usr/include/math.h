/* Declarations for math functions.
   Copyright (C) 1991-2020 Free Software Foundation, Inc.
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

/*
 *	ISO C99 Standard: 7.12 Mathematics	<math.h>
 */

#ifndef	_MATH_H
#define	_MATH_H	1

#define __GLIBC_INTERNAL_STARTING_HEADER_IMPLEMENTATION
#include <bits/libc-header-start.h>

#if defined log && defined __GNUC__
# warning A macro called log was already defined when <math.h> was included.
# warning This will cause compilation problems.
#endif

__BEGIN_DECLS

/* Get definitions of __intmax_t and __uintmax_t.  */
#include <bits/types.h>

/* Get machine-dependent vector math functions declarations.  */
#include <bits/math-vector.h>

/* Gather machine dependent type support.  */
#include <bits/floatn.h>

/* Value returned on overflow.  With IEEE 754 floating point, this is
   +Infinity, otherwise the largest representable positive value.  */
#if __GNUC_PREREQ (3, 3)
# define HUGE_VAL (__builtin_huge_val ())
#else
/* This may provoke compiler warnings, and may not be rounded to
   +Infinity in all IEEE 754 rounding modes, but is the best that can
   be done in ISO C while remaining a constant expression.  10,000 is
   greater than the maximum (decimal) exponent for all supported
   floating-point formats and widths.  */
# define HUGE_VAL 1e10000
#endif
#ifdef __USE_ISOC99
# if __GNUC_PREREQ (3, 3)
#  define HUGE_VALF (__builtin_huge_valf ())
#  define HUGE_VALL (__builtin_huge_vall ())
# else
#  define HUGE_VALF 1e10000f
#  define HUGE_VALL 1e10000L
# endif
#endif
#if __HAVE_FLOAT16 && __GLIBC_USE (IEC_60559_TYPES_EXT)
# define HUGE_VAL_F16 (__builtin_huge_valf16 ())
#endif
#if __HAVE_FLOAT32 && __GLIBC_USE (IEC_60559_TYPES_EXT)
# define HUGE_VAL_F32 (__builtin_huge_valf32 ())
#endif
#if __HAVE_FLOAT64 && __GLIBC_USE (IEC_60559_TYPES_EXT)
# define HUGE_VAL_F64 (__builtin_huge_valf64 ())
#endif
#if __HAVE_FLOAT128 && __GLIBC_USE (IEC_60559_TYPES_EXT)
# define HUGE_VAL_F128 (__builtin_huge_valf128 ())
#endif
#if __HAVE_FLOAT32X && __GLIBC_USE (IEC_60559_TYPES_EXT)
# define HUGE_VAL_F32X (__builtin_huge_valf32x ())
#endif
#if __HAVE_FLOAT64X && __GLIBC_USE (IEC_60559_TYPES_EXT)
# define HUGE_VAL_F64X (__builtin_huge_valf64x ())
#endif
#if __HAVE_FLOAT128X && __GLIBC_USE (IEC_60559_TYPES_EXT)
# define HUGE_VAL_F128X (__builtin_huge_valf128x ())
#endif

#ifdef __USE_ISOC99
/* IEEE positive infinity.  */
# if __GNUC_PREREQ (3, 3)
#  define INFINITY (__builtin_inff ())
# else
#  define INFINITY HUGE_VALF
# endif

/* IEEE Not A Number.  */
# if __GNUC_PREREQ (3, 3)
#  define NAN (__builtin_nanf (""))
# else
/* This will raise an "invalid" exception outside static initializers,
   but is the best that can be done in ISO C while remaining a
   constant expression.  */
#  define NAN (0.0f / 0.0f)
# endif
#endif /* __USE_ISOC99 */

#if __GLIBC_USE (IEC_60559_BFP_EXT_C2X)
/* Signaling NaN macros, if supported.  */
# if __GNUC_PREREQ (3, 3)
#  define SNANF (__builtin_nansf (""))
#  define SNAN (__builtin_nans (""))
#  define SNANL (__builtin_nansl (""))
# endif
#endif
#if __HAVE_FLOAT16 && __GLIBC_USE (IEC_60559_TYPES_EXT)
# define SNANF16 (__builtin_nansf16 (""))
#endif
#if __HAVE_FLOAT32 && __GLIBC_USE (IEC_60559_TYPES_EXT)
# define SNANF32 (__builtin_nansf32 (""))
#endif
#if __HAVE_FLOAT64 && __GLIBC_USE (IEC_60559_TYPES_EXT)
# define SNANF64 (__builtin_nansf64 (""))
#endif
#if __HAVE_FLOAT128 && __GLIBC_USE (IEC_60559_TYPES_EXT)
# define SNANF128 (__builtin_nansf128 (""))
#endif
#if __HAVE_FLOAT32X && __GLIBC_USE (IEC_60559_TYPES_EXT)
# define SNANF32X (__builtin_nansf32x (""))
#endif
#if __HAVE_FLOAT64X && __GLIBC_USE (IEC_60559_TYPES_EXT)
# define SNANF64X (__builtin_nansf64x (""))
#endif
#if __HAVE_FLOAT128X && __GLIBC_USE (IEC_60559_TYPES_EXT)
# define SNANF128X (__builtin_nansf128x (""))
#endif

/* Get __GLIBC_FLT_EVAL_METHOD.  */
#include <bits/flt-eval-method.h>

#ifdef __USE_ISOC99
/* Define the following typedefs.

    float_t	floating-point type at least as wide as `float' used
		to evaluate `float' expressions
    double_t	floating-point type at least as wide as `double' used
		to evaluate `double' expressions
*/
# if __GLIBC_FLT_EVAL_METHOD == 0 || __GLIBC_FLT_EVAL_METHOD == 16
typedef float float_t;
typedef double double_t;
# elif __GLIBC_FLT_EVAL_METHOD == 1
typedef double float_t;
typedef double double_t;
# elif __GLIBC_FLT_EVAL_METHOD == 2
typedef long double float_t;
typedef long double double_t;
# elif __GLIBC_FLT_EVAL_METHOD == 32
typedef _Float32 float_t;
typedef double double_t;
# elif __GLIBC_FLT_EVAL_METHOD == 33
typedef _Float32x float_t;
typedef _Float32x double_t;
# elif __GLIBC_FLT_EVAL_METHOD == 64
typedef _Float64 float_t;
typedef _Float64 double_t;
# elif __GLIBC_FLT_EVAL_METHOD == 65
typedef _Float64x float_t;
typedef _Float64x double_t;
# elif __GLIBC_FLT_EVAL_METHOD == 128
typedef _Float128 float_t;
typedef _Float128 double_t;
# elif __GLIBC_FLT_EVAL_METHOD == 129
typedef _Float128x float_t;
typedef _Float128x double_t;
# else
#  error "Unknown __GLIBC_FLT_EVAL_METHOD"
# endif
#endif

/* Define macros for the return values of ilogb and llogb, based on
   __FP_LOGB0_IS_MIN and __FP_LOGBNAN_IS_MIN.

    FP_ILOGB0	Expands to a value returned by `ilogb (0.0)'.
    FP_ILOGBNAN	Expands to a value returned by `ilogb (NAN)'.
    FP_LLOGB0	Expands to a value returned by `llogb (0.0)'.
    FP_LLOGBNAN	Expands to a value returned by `llogb (NAN)'.

*/

#include <bits/fp-logb.h>
#ifdef __USE_ISOC99
# if __FP_LOGB0_IS_MIN
#  define FP_ILOGB0	(-2147483647 - 1)
# else
#  define FP_ILOGB0	(-2147483647)
# endif
# if __FP_LOGBNAN_IS_MIN
#  define FP_ILOGBNAN	(-2147483647 - 1)
# else
#  define FP_ILOGBNAN	2147483647
# endif
#endif
#if __GLIBC_USE (IEC_60559_BFP_EXT_C2X)
# if __WORDSIZE == 32
#  define __FP_LONG_MAX 0x7fffffffL
# else
#  define __FP_LONG_MAX 0x7fffffffffffffffL
# endif
# if __FP_LOGB0_IS_MIN
#  define FP_LLOGB0	(-__FP_LONG_MAX - 1)
# else
#  define FP_LLOGB0	(-__FP_LONG_MAX)
# endif
# if __FP_LOGBNAN_IS_MIN
#  define FP_LLOGBNAN	(-__FP_LONG_MAX - 1)
# else
#  define FP_LLOGBNAN	__FP_LONG_MAX
# endif
#endif

/* Get the architecture specific values describing the floating-point
   evaluation.  The following symbols will get defined:

    FP_FAST_FMA
    FP_FAST_FMAF
    FP_FAST_FMAL
		If defined it indicates that the `fma' function
		generally executes about as fast as a multiply and an add.
		This macro is defined only iff the `fma' function is
		implemented directly with a hardware multiply-add instructions.
*/

#include <bits/fp-fast.h>

#if __GLIBC_USE (IEC_60559_BFP_EXT_C2X)
/* Rounding direction macros for fromfp functions.  */
enum
  {
    FP_INT_UPWARD =
# define FP_INT_UPWARD 0
      FP_INT_UPWARD,
    FP_INT_DOWNWARD =
# define FP_INT_DOWNWARD 1
      FP_INT_DOWNWARD,
    FP_INT_TOWARDZERO =
# define FP_INT_TOWARDZERO 2
      FP_INT_TOWARDZERO,
    FP_INT_TONEARESTFROMZERO =
# define FP_INT_TONEARESTFROMZERO 3
      FP_INT_TONEARESTFROMZERO,
    FP_INT_TONEAREST =
# define FP_INT_TONEAREST 4
      FP_INT_TONEAREST,
  };
#endif

/* The file <bits/mathcalls.h> contains the prototypes for all the
   actual math functions.  These macros are used for those prototypes,
   so we can easily declare each function as both `name' and `__name',
   and can declare the float versions `namef' and `__namef'.  */

#define __SIMD_DECL(function) __CONCAT (__DECL_SIMD_, function)

#define __MATHCALL_VEC(function, suffix, args) 	\
  __SIMD_DECL (__MATH_PRECNAME (function, suffix)) \
  __MATHCALL (function, suffix, args)

#define __MATHDECL_VEC(type, function,suffix, args) \
  __SIMD_DECL (__MATH_PRECNAME (function, suffix)) \
  __MATHDECL(type, function,suffix, args)

#define __MATHCALL(function,suffix, args)	\
  __MATHDECL (_Mdouble_,function,suffix, args)
#define __MATHDECL(type, function,suffix, args) \
  __MATHDECL_1(type, function,suffix, args); \
  __MATHDECL_1(type, __CONCAT(__,function),suffix, args)
#define __MATHCALLX(function,suffix, args, attrib)	\
  __MATHDECLX (_Mdouble_,function,suffix, args, attrib)
#define __MATHDECLX(type, function,suffix, args, attrib) \
  __MATHDECL_1(type, function,suffix, args) __attribute__ (attrib); \
  __MATHDECL_1(type, __CONCAT(__,function),suffix, args) __attribute__ (attrib)
#define __MATHDECL_1(type, function,suffix, args) \
  extern type __MATH_PRECNAME(function,suffix) args __THROW

#define _Mdouble_		double
#define __MATH_PRECNAME(name,r)	__CONCAT(name,r)
#define __MATH_DECLARING_DOUBLE  1
#define __MATH_DECLARING_FLOATN  0
#include <bits/mathcalls-helper-functions.h>
#include <bits/mathcalls.h>
#undef	_Mdouble_
#undef	__MATH_PRECNAME
#undef __MATH_DECLARING_DOUBLE
#undef __MATH_DECLARING_FLOATN

#ifdef __USE_ISOC99


/* Include the file of declarations again, this time using `float'
   instead of `double' and appending f to each function name.  */

# define _Mdouble_		float
# define __MATH_PRECNAME(name,r) name##f##r
# define __MATH_DECLARING_DOUBLE  0
# define __MATH_DECLARING_FLOATN  0
# include <bits/mathcalls-helper-functions.h>
# include <bits/mathcalls.h>
# undef	_Mdouble_
# undef	__MATH_PRECNAME
# undef __MATH_DECLARING_DOUBLE
# undef __MATH_DECLARING_FLOATN

# if !(defined __NO_LONG_DOUBLE_MATH && defined _LIBC) \
     || defined __LDBL_COMPAT \
     || defined _LIBC_TEST
#  ifdef __LDBL_COMPAT

#   ifdef __USE_ISOC99
extern float __nldbl_nexttowardf (float __x, long double __y)
				  __THROW __attribute__ ((__const__));
#    ifdef __REDIRECT_NTH
extern float __REDIRECT_NTH (nexttowardf, (float __x, long double __y),
			     __nldbl_nexttowardf)
     __attribute__ ((__const__));
extern double __REDIRECT_NTH (nexttoward, (double __x, long double __y),
			      nextafter) __attribute__ ((__const__));
extern long double __REDIRECT_NTH (nexttowardl,
				   (long double __x, long double __y),
				   nextafter) __attribute__ ((__const__));
#    endif
#   endif

#   undef __MATHDECL_1
#   define __MATHDECL_2(type, function,suffix, args, alias) \
  extern type __REDIRECT_NTH(__MATH_PRECNAME(function,suffix), \
			     args, alias)
#   define __MATHDECL_1(type, function,suffix, args) \
  __MATHDECL_2(type, function,suffix, args, __CONCAT(function,suffix))
#  endif

/* Include the file of declarations again, this time using `long double'
   instead of `double' and appending l to each function name.  */

#  define _Mdouble_		long double
#  define __MATH_PRECNAME(name,r) name##l##r
#  define __MATH_DECLARING_DOUBLE  0
#  define __MATH_DECLARING_FLOATN  0
#  define __MATH_DECLARE_LDOUBLE   1
#  include <bits/mathcalls-helper-functions.h>
#  include <bits/mathcalls.h>
#  undef _Mdouble_
#  undef __MATH_PRECNAME
#  undef __MATH_DECLARING_DOUBLE
#  undef __MATH_DECLARING_FLOATN

# endif /* !(__NO_LONG_DOUBLE_MATH && _LIBC) || __LDBL_COMPAT */

#endif	/* Use ISO C99.  */

/* Include the file of declarations for _FloatN and _FloatNx
   types.  */

#if __HAVE_DISTINCT_FLOAT16 || (__HAVE_FLOAT16 && !defined _LIBC)
# define _Mdouble_		_Float16
# define __MATH_PRECNAME(name,r) name##f16##r
# define __MATH_DECLARING_DOUBLE  0
# define __MATH_DECLARING_FLOATN  1
# if __HAVE_DISTINCT_FLOAT16
#  include <bits/mathcalls-helper-functions.h>
# endif
# if __GLIBC_USE (IEC_60559_TYPES_EXT)
#  include <bits/mathcalls.h>
# endif
# undef _Mdouble_
# undef __MATH_PRECNAME
# undef __MATH_DECLARING_DOUBLE
# undef __MATH_DECLARING_FLOATN
#endif /* __HAVE_DISTINCT_FLOAT16 || (__HAVE_FLOAT16 && !_LIBC).  */

#if __HAVE_DISTINCT_FLOAT32 || (__HAVE_FLOAT32 && !defined _LIBC)
# define _Mdouble_		_Float32
# define __MATH_PRECNAME(name,r) name##f32##r
# define __MATH_DECLARING_DOUBLE  0
# define __MATH_DECLARING_FLOATN  1
# if __HAVE_DISTINCT_FLOAT32
#  include <bits/mathcalls-helper-functions.h>
# endif
# if __GLIBC_USE (IEC_60559_TYPES_EXT)
#  include <bits/mathcalls.h>
# endif
# undef _Mdouble_
# undef __MATH_PRECNAME
# undef __MATH_DECLARING_DOUBLE
# undef __MATH_DECLARING_FLOATN
#endif /* __HAVE_DISTINCT_FLOAT32 || (__HAVE_FLOAT32 && !_LIBC).  */

#if __HAVE_DISTINCT_FLOAT64 || (__HAVE_FLOAT64 && !defined _LIBC)
# define _Mdouble_		_Float64
# define __MATH_PRECNAME(name,r) name##f64##r
# define __MATH_DECLARING_DOUBLE  0
# define __MATH_DECLARING_FLOATN  1
# if __HAVE_DISTINCT_FLOAT64
#  include <bits/mathcalls-helper-functions.h>
# endif
# if __GLIBC_USE (IEC_60559_TYPES_EXT)
#  include <bits/mathcalls.h>
# endif
# undef _Mdouble_
# undef __MATH_PRECNAME
# undef __MATH_DECLARING_DOUBLE
# undef __MATH_DECLARING_FLOATN
#endif /* __HAVE_DISTINCT_FLOAT64 || (__HAVE_FLOAT64 && !_LIBC).  */

#if __HAVE_DISTINCT_FLOAT128 || (__HAVE_FLOAT128 && !defined _LIBC)
# define _Mdouble_		_Float128
# define __MATH_PRECNAME(name,r) name##f128##r
# define __MATH_DECLARING_DOUBLE  0
# define __MATH_DECLARING_FLOATN  1
# if __HAVE_DISTINCT_FLOAT128
#  include <bits/mathcalls-helper-functions.h>
# endif
# if __GLIBC_USE (IEC_60559_TYPES_EXT)
#  include <bits/mathcalls.h>
# endif
# undef _Mdouble_
# undef __MATH_PRECNAME
# undef __MATH_DECLARING_DOUBLE
# undef __MATH_DECLARING_FLOATN
#endif /* __HAVE_DISTINCT_FLOAT128 || (__HAVE_FLOAT128 && !_LIBC).  */

#if __HAVE_DISTINCT_FLOAT32X || (__HAVE_FLOAT32X && !defined _LIBC)
# define _Mdouble_		_Float32x
# define __MATH_PRECNAME(name,r) name##f32x##r
# define __MATH_DECLARING_DOUBLE  0
# define __MATH_DECLARING_FLOATN  1
# if __HAVE_DISTINCT_FLOAT32X
#  include <bits/mathcalls-helper-functions.h>
# endif
# if __GLIBC_USE (IEC_60559_TYPES_EXT)
#  include <bits/mathcalls.h>
# endif
# undef _Mdouble_
# undef __MATH_PRECNAME
# undef __MATH_DECLARING_DOUBLE
# undef __MATH_DECLARING_FLOATN
#endif /* __HAVE_DISTINCT_FLOAT32X || (__HAVE_FLOAT32X && !_LIBC).  */

#if __HAVE_DISTINCT_FLOAT64X || (__HAVE_FLOAT64X && !defined _LIBC)
# define _Mdouble_		_Float64x
# define __MATH_PRECNAME(name,r) name##f64x##r
# define __MATH_DECLARING_DOUBLE  0
# define __MATH_DECLARING_FLOATN  1
# if __HAVE_DISTINCT_FLOAT64X
#  include <bits/mathcalls-helper-functions.h>
# endif
# if __GLIBC_USE (IEC_60559_TYPES_EXT)
#  include <bits/mathcalls.h>
# endif
# undef _Mdouble_
# undef __MATH_PRECNAME
# undef __MATH_DECLARING_DOUBLE
# undef __MATH_DECLARING_FLOATN
#endif /* __HAVE_DISTINCT_FLOAT64X || (__HAVE_FLOAT64X && !_LIBC).  */

#if __HAVE_DISTINCT_FLOAT128X || (__HAVE_FLOAT128X && !defined _LIBC)
# define _Mdouble_		_Float128x
# define __MATH_PRECNAME(name,r) name##f128x##r
# define __MATH_DECLARING_DOUBLE  0
# define __MATH_DECLARING_FLOATN  1
# if __HAVE_DISTINCT_FLOAT128X
#  include <bits/mathcalls-helper-functions.h>
# endif
# if __GLIBC_USE (IEC_60559_TYPES_EXT)
#  include <bits/mathcalls.h>
# endif
# undef _Mdouble_
# undef __MATH_PRECNAME
# undef __MATH_DECLARING_DOUBLE
# undef __MATH_DECLARING_FLOATN
#endif /* __HAVE_DISTINCT_FLOAT128X || (__HAVE_FLOAT128X && !_LIBC).  */

#undef	__MATHDECL_1
#undef	__MATHDECL
#undef	__MATHCALL

/* Declare functions returning a narrower type.  */
#define __MATHCALL_NARROW_ARGS_1 (_Marg_ __x)
#define __MATHCALL_NARROW_ARGS_2 (_Marg_ __x, _Marg_ __y)
#define __MATHCALL_NARROW_ARGS_3 (_Marg_ __x, _Marg_ __y, _Marg_ __z)
#define __MATHCALL_NARROW_NORMAL(func, nargs)			\
  extern _Mret_ func __MATHCALL_NARROW_ARGS_ ## nargs __THROW
#define __MATHCALL_NARROW_REDIR(func, redir, nargs)			\
  extern _Mret_ __REDIRECT_NTH (func, __MATHCALL_NARROW_ARGS_ ## nargs, \
				redir)
#define __MATHCALL_NARROW(func, redir, nargs)	\
  __MATHCALL_NARROW_NORMAL (func, nargs)

#if __GLIBC_USE (IEC_60559_BFP_EXT_C2X)

# define _Mret_ float
# define _Marg_ double
# define __MATHCALL_NAME(name) f ## name
# include <bits/mathcalls-narrow.h>
# undef _Mret_
# undef _Marg_
# undef __MATHCALL_NAME

# define _Mret_ float
# define _Marg_ long double
# define __MATHCALL_NAME(name) f ## name ## l
# ifdef __LDBL_COMPAT
#  define __MATHCALL_REDIR_NAME(name) f ## name
#  undef __MATHCALL_NARROW
#  define __MATHCALL_NARROW(func, redir, nargs) \
  __MATHCALL_NARROW_REDIR (func, redir, nargs)
# endif
# include <bits/mathcalls-narrow.h>
# undef _Mret_
# undef _Marg_
# undef __MATHCALL_NAME
# ifdef __LDBL_COMPAT
#  undef __MATHCALL_REDIR_NAME
#  undef __MATHCALL_NARROW
#  define __MATHCALL_NARROW(func, redir, nargs) \
  __MATHCALL_NARROW_NORMAL (func, nargs)
# endif

# define _Mret_ double
# define _Marg_ long double
# define __MATHCALL_NAME(name) d ## name ## l
# ifdef __LDBL_COMPAT
#  define __MATHCALL_REDIR_NAME(name) __nldbl_d ## name ## l
#  undef __MATHCALL_NARROW
#  define __MATHCALL_NARROW(func, redir, nargs) \
  __MATHCALL_NARROW_REDIR (func, redir, nargs)
# endif
# include <bits/mathcalls-narrow.h>
# undef _Mret_
# undef _Marg_
# undef __MATHCALL_NAME
# ifdef __LDBL_COMPAT
#  undef __MATHCALL_REDIR_NAME
#  undef __MATHCALL_NARROW
#  define __MATHCALL_NARROW(func, redir, nargs) \
  __MATHCALL_NARROW_NORMAL (func, nargs)
# endif

#endif

#if __GLIBC_USE (IEC_60559_TYPES_EXT)

# if __HAVE_FLOAT16 && __HAVE_FLOAT32
#  define _Mret_ _Float16
#  define _Marg_ _Float32
#  define __MATHCALL_NAME(name) f16 ## name ## f32
#  include <bits/mathcalls-narrow.h>
#  undef _Mret_
#  undef _Marg_
#  undef __MATHCALL_NAME
# endif

# if __HAVE_FLOAT16 && __HAVE_FLOAT32X
#  define _Mret_ _Float16
#  define _Marg_ _Float32x
#  define __MATHCALL_NAME(name) f16 ## name ## f32x
#  include <bits/mathcalls-narrow.h>
#  undef _Mret_
#  undef _Marg_
#  undef __MATHCALL_NAME
# endif

# if __HAVE_FLOAT16 && __HAVE_FLOAT64
#  define _Mret_ _Float16
#  define _Marg_ _Float64
#  define __MATHCALL_NAME(name) f16 ## name ## f64
#  include <bits/mathcalls-narrow.h>
#  undef _Mret_
#  undef _Marg_
#  undef __MATHCALL_NAME
# endif

# if __HAVE_FLOAT16 && __HAVE_FLOAT64X
#  define _Mret_ _Float16
#  define _Marg_ _Float64x
#  define __MATHCALL_NAME(name) f16 ## name ## f64x
#  include <bits/mathcalls-narrow.h>
#  undef _Mret_
#  undef _Marg_
#  undef __MATHCALL_NAME
# endif

# if __HAVE_FLOAT16 && __HAVE_FLOAT128
#  define _Mret_ _Float16
#  define _Marg_ _Float128
#  define __MATHCALL_NAME(name) f16 ## name ## f128
#  include <bits/mathcalls-narrow.h>
#  undef _Mret_
#  undef _Marg_
#  undef __MATHCALL_NAME
# endif

# if __HAVE_FLOAT16 && __HAVE_FLOAT128X
#  define _Mret_ _Float16
#  define _Marg_ _Float128x
#  define __MATHCALL_NAME(name) f16 ## name ## f128x
#  include <bits/mathcalls-narrow.h>
#  undef _Mret_
#  undef _Marg_
#  undef __MATHCALL_NAME
# endif

# if __HAVE_FLOAT32 && __HAVE_FLOAT32X
#  define _Mret_ _Float32
#  define _Marg_ _Float32x
#  define __MATHCALL_NAME(name) f32 ## name ## f32x
#  include <bits/mathcalls-narrow.h>
#  undef _Mret_
#  undef _Marg_
#  undef __MATHCALL_NAME
# endif

# if __HAVE_FLOAT32 && __HAVE_FLOAT64
#  define _Mret_ _Float32
#  define _Marg_ _Float64
#  define __MATHCALL_NAME(name) f32 ## name ## f64
#  include <bits/mathcalls-narrow.h>
#  undef _Mret_
#  undef _Marg_
#  undef __MATHCALL_NAME
# endif

# if __HAVE_FLOAT32 && __HAVE_FLOAT64X
#  define _Mret_ _Float32
#  define _Marg_ _Float64x
#  define __MATHCALL_NAME(name) f32 ## name ## f64x
#  include <bits/mathcalls-narrow.h>
#  undef _Mret_
#  undef _Marg_
#  undef __MATHCALL_NAME
# endif

# if __HAVE_FLOAT32 && __HAVE_FLOAT128
#  define _Mret_ _Float32
#  define _Marg_ _Float128
#  define __MATHCALL_NAME(name) f32 ## name ## f128
#  include <bits/mathcalls-narrow.h>
#  undef _Mret_
#  undef _Marg_
#  undef __MATHCALL_NAME
# endif

# if __HAVE_FLOAT32 && __HAVE_FLOAT128X
#  define _Mret_ _Float32
#  define _Marg_ _Float128x
#  define __MATHCALL_NAME(name) f32 ## name ## f128x
#  include <bits/mathcalls-narrow.h>
#  undef _Mret_
#  undef _Marg_
#  undef __MATHCALL_NAME
# endif

# if __HAVE_FLOAT32X && __HAVE_FLOAT64
#  define _Mret_ _Float32x
#  define _Marg_ _Float64
#  define __MATHCALL_NAME(name) f32x ## name ## f64
#  include <bits/mathcalls-narrow.h>
#  undef _Mret_
#  undef _Marg_
#  undef __MATHCALL_NAME
# endif

# if __HAVE_FLOAT32X && __HAVE_FLOAT64X
#  define _Mret_ _Float32x
#  define _Marg_ _Float64x
#  define __MATHCALL_NAME(name) f32x ## name ## f64x
#  include <bits/mathcalls-narrow.h>
#  undef _Mret_
#  undef _Marg_
#  undef __MATHCALL_NAME
# endif

# if __HAVE_FLOAT32X && __HAVE_FLOAT128
#  define _Mret_ _Float32x
#  define _Marg_ _Float128
#  define __MATHCALL_NAME(name) f32x ## name ## f128
#  include <bits/mathcalls-narrow.h>
#  undef _Mret_
#  undef _Marg_
#  undef __MATHCALL_NAME
# endif

# if __HAVE_FLOAT32X && __HAVE_FLOAT128X
#  define _Mret_ _Float32x
#  define _Marg_ _Float128x
#  define __MATHCALL_NAME(name) f32x ## name ## f128x
#  include <bits/mathcalls-narrow.h>
#  undef _Mret_
#  undef _Marg_
#  undef __MATHCALL_NAME
# endif

# if __HAVE_FLOAT64 && __HAVE_FLOAT64X
#  define _Mret_ _Float64
#  define _Marg_ _Float64x
#  define __MATHCALL_NAME(name) f64 ## name ## f64x
#  include <bits/mathcalls-narrow.h>
#  undef _Mret_
#  undef _Marg_
#  undef __MATHCALL_NAME
# endif

# if __HAVE_FLOAT64 && __HAVE_FLOAT128
#  define _Mret_ _Float64
#  define _Marg_ _Float128
#  define __MATHCALL_NAME(name) f64 ## name ## f128
#  include <bits/mathcalls-narrow.h>
#  undef _Mret_
#  undef _Marg_
#  undef __MATHCALL_NAME
# endif

# if __HAVE_FLOAT64 && __HAVE_FLOAT128X
#  define _Mret_ _Float64
#  define _Marg_ _Float128x
#  define __MATHCALL_NAME(name) f64 ## name ## f128x
#  include <bits/mathcalls-narrow.h>
#  undef _Mret_
#  undef _Marg_
#  undef __MATHCALL_NAME
# endif

# if __HAVE_FLOAT64X && __HAVE_FLOAT128
#  define _Mret_ _Float64x
#  define _Marg_ _Float128
#  define __MATHCALL_NAME(name) f64x ## name ## f128
#  include <bits/mathcalls-narrow.h>
#  undef _Mret_
#  undef _Marg_
#  undef __MATHCALL_NAME
# endif

# if __HAVE_FLOAT64X && __HAVE_FLOAT128X
#  define _Mret_ _Float64x
#  define _Marg_ _Float128x
#  define __MATHCALL_NAME(name) f64x ## name ## f128x
#  include <bits/mathcalls-narrow.h>
#  undef _Mret_
#  undef _Marg_
#  undef __MATHCALL_NAME
# endif

# if __HAVE_FLOAT128 && __HAVE_FLOAT128X
#  define _Mret_ _Float128
#  define _Marg_ _Float128x
#  define __MATHCALL_NAME(name) f128 ## name ## f128x
#  include <bits/mathcalls-narrow.h>
#  undef _Mret_
#  undef _Marg_
#  undef __MATHCALL_NAME
# endif

#endif

#undef __MATHCALL_NARROW_ARGS_1
#undef __MATHCALL_NARROW_ARGS_2
#undef __MATHCALL_NARROW_ARGS_3
#undef __MATHCALL_NARROW_NORMAL
#undef __MATHCALL_NARROW_REDIR
#undef __MATHCALL_NARROW

#if defined __USE_MISC || defined __USE_XOPEN
/* This variable is used by `gamma' and `lgamma'.  */
extern int signgam;
#endif

#if (__HAVE_DISTINCT_FLOAT16			\
     || __HAVE_DISTINCT_FLOAT32			\
     || __HAVE_DISTINCT_FLOAT64			\
     || __HAVE_DISTINCT_FLOAT32X		\
     || __HAVE_DISTINCT_FLOAT64X		\
     || __HAVE_DISTINCT_FLOAT128X)
# error "Unsupported _FloatN or _FloatNx types for <math.h>."
#endif

/* Depending on the type of TG_ARG, call an appropriately suffixed
   version of FUNC with arguments (including parentheses) ARGS.
   Suffixed functions may not exist for long double if it has the same
   format as double, or for other types with the same format as float,
   double or long double.  The behavior is undefined if the argument
   does not have a real floating type.  The definition may use a
   conditional expression, so all suffixed versions of FUNC must
   return the same type (FUNC may include a cast if necessary rather
   than being a single identifier).  */
#ifdef __NO_LONG_DOUBLE_MATH
# if __HAVE_DISTINCT_FLOAT128
#  error "Distinct _Float128 without distinct long double not supported."
# endif
# define __MATH_TG(TG_ARG, FUNC, ARGS)					\
  (sizeof (TG_ARG) == sizeof (float) ? FUNC ## f ARGS : FUNC ARGS)
#elif __HAVE_DISTINCT_FLOAT128
# if __HAVE_GENERIC_SELECTION
#  if __HAVE_FLOATN_NOT_TYPEDEF && __HAVE_FLOAT32
#   define __MATH_TG_F32(FUNC, ARGS) _Float32: FUNC ## f ARGS,
#  else
#   define __MATH_TG_F32(FUNC, ARGS)
#  endif
#  if __HAVE_FLOATN_NOT_TYPEDEF && __HAVE_FLOAT64X
#   if __HAVE_FLOAT64X_LONG_DOUBLE
#    define __MATH_TG_F64X(FUNC, ARGS) _Float64x: FUNC ## l ARGS,
#   else
#    define __MATH_TG_F64X(FUNC, ARGS) _Float64x: FUNC ## f128 ARGS,
#   endif
#  else
#   define __MATH_TG_F64X(FUNC, ARGS)
#  endif
#  define __MATH_TG(TG_ARG, FUNC, ARGS)	\
     _Generic ((TG_ARG),			\
	       float: FUNC ## f ARGS,		\
	       __MATH_TG_F32 (FUNC, ARGS)	\
	       default: FUNC ARGS,		\
	       long double: FUNC ## l ARGS,	\
	       __MATH_TG_F64X (FUNC, ARGS)	\
	       _Float128: FUNC ## f128 ARGS)
# else
#  if __HAVE_FLOATN_NOT_TYPEDEF
#   error "Non-typedef _FloatN but no _Generic."
#  endif
#  define __MATH_TG(TG_ARG, FUNC, ARGS)					\
     __builtin_choose_expr						\
     (__builtin_types_compatible_p (__typeof (TG_ARG), float),		\
      FUNC ## f ARGS,							\
      __builtin_choose_expr						\
      (__builtin_types_compatible_p (__typeof (TG_ARG), double),	\
       FUNC ARGS,							\
       __builtin_choose_expr						\
       (__builtin_types_compatible_p (__typeof (TG_ARG), long double),	\
	FUNC ## l ARGS,							\
	FUNC ## f128 ARGS)))
# endif
#else
# define __MATH_TG(TG_ARG, FUNC, ARGS)		\
  (sizeof (TG_ARG) == sizeof (float)		\
   ? FUNC ## f ARGS				\
   : sizeof (TG_ARG) == sizeof (double)		\
   ? FUNC ARGS					\
   : FUNC ## l ARGS)
#endif

/* ISO C99 defines some generic macros which work on any data type.  */
#ifdef __USE_ISOC99

/* All floating-point numbers can be put in one of these categories.  */
enum
  {
    FP_NAN =
# define FP_NAN 0
      FP_NAN,
    FP_INFINITE =
# define FP_INFINITE 1
      FP_INFINITE,
    FP_ZERO =
# define FP_ZERO 2
      FP_ZERO,
    FP_SUBNORMAL =
# define FP_SUBNORMAL 3
      FP_SUBNORMAL,
    FP_NORMAL =
# define FP_NORMAL 4
      FP_NORMAL
  };

/* GCC bug 66462 means we cannot use the math builtins with -fsignaling-nan,
   so disable builtins if this is enabled.  When fixed in a newer GCC,
   the __SUPPORT_SNAN__ check may be skipped for those versions.  */

/* Return number of classification appropriate for X.  */
# if ((__GNUC_PREREQ (4,4) && !defined __SUPPORT_SNAN__)		      \
      || __glibc_clang_prereq (2,8))					      \
     && (!defined __OPTIMIZE_SIZE__ || defined __cplusplus)
     /* The check for __cplusplus allows the use of the builtin, even
	when optimization for size is on.  This is provided for
	libstdc++, only to let its configure test work when it is built
	with -Os.  No further use of this definition of fpclassify is
	expected in C++ mode, since libstdc++ provides its own version
	of fpclassify in cmath (which undefines fpclassify).  */
#  define fpclassify(x) __builtin_fpclassify (FP_NAN, FP_INFINITE,	      \
     FP_NORMAL, FP_SUBNORMAL, FP_ZERO, x)
# else
#  define fpclassify(x) __MATH_TG ((x), __fpclassify, (x))
# endif

/* Return nonzero value if sign of X is negative.  */
# if __GNUC_PREREQ (6,0) || __glibc_clang_prereq (3,3)
#  define signbit(x) __builtin_signbit (x)
# elif defined __cplusplus
  /* In C++ mode, __MATH_TG cannot be used, because it relies on
     __builtin_types_compatible_p, which is a C-only builtin.
     The check for __cplusplus allows the use of the builtin instead of
     __MATH_TG. This is provided for libstdc++, only to let its configure
     test work. No further use of this definition of signbit is expected
     in C++ mode, since libstdc++ provides its own version of signbit
     in cmath (which undefines signbit). */
#  define signbit(x) __builtin_signbitl (x)
# elif __GNUC_PREREQ (4,0)
#  define signbit(x) __MATH_TG ((x), __builtin_signbit, (x))
# else
#  define signbit(x) __MATH_TG ((x), __signbit, (x))
# endif

/* Return nonzero value if X is not +-Inf or NaN.  */
# if (__GNUC_PREREQ (4,4) && !defined __SUPPORT_SNAN__) \
     || __glibc_clang_prereq (2,8)
#  define isfinite(x) __builtin_isfinite (x)
# else
#  define isfinite(x) __MATH_TG ((x), __finite, (x))
# endif

/* Return nonzero value if X is neither zero, subnormal, Inf, nor NaN.  */
# if (__GNUC_PREREQ (4,4) && !defined __SUPPORT_SNAN__) \
     || __glibc_clang_prereq (2,8)
#  define isnormal(x) __builtin_isnormal (x)
# else
#  define isnormal(x) (fpclassify (x) == FP_NORMAL)
# endif

/* Return nonzero value if X is a NaN.  We could use `fpclassify' but
   we already have this functions `__isnan' and it is faster.  */
# if (__GNUC_PREREQ (4,4) && !defined __SUPPORT_SNAN__) \
     || __glibc_clang_prereq (2,8)
#  define isnan(x) __builtin_isnan (x)
# else
#  define isnan(x) __MATH_TG ((x), __isnan, (x))
# endif

/* Return nonzero value if X is positive or negative infinity.  */
# if __HAVE_DISTINCT_FLOAT128 && !__GNUC_PREREQ (7,0) \
     && !defined __SUPPORT_SNAN__ && !defined __cplusplus
   /* Since __builtin_isinf_sign is broken for float128 before GCC 7.0,
      use the helper function, __isinff128, with older compilers.  This is
      only provided for C mode, because in C++ mode, GCC has no support
      for __builtin_types_compatible_p (and when in C++ mode, this macro is
      not used anyway, because libstdc++ headers undefine it).  */
#  define isinf(x) \
    (__builtin_types_compatible_p (__typeof (x), _Float128) \
     ? __isinff128 (x) : __builtin_isinf_sign (x))
# elif (__GNUC_PREREQ (4,4) && !defined __SUPPORT_SNAN__) \
       || __glibc_clang_prereq (3,7)
#  define isinf(x) __builtin_isinf_sign (x)
# else
#  define isinf(x) __MATH_TG ((x), __isinf, (x))
# endif

/* Bitmasks for the math_errhandling macro.  */
# define MATH_ERRNO	1	/* errno set by math functions.  */
# define MATH_ERREXCEPT	2	/* Exceptions raised by math functions.  */

/* By default all math functions support both errno and exception handling
   (except for soft floating point implementations which may only support
   errno handling).  If errno handling is disabled, exceptions are still
   supported by GLIBC.  Set math_errhandling to 0 with -ffast-math (this is
   nonconforming but it is more useful than leaving it undefined).  */
# ifdef __FAST_MATH__
#  define math_errhandling	0
# elif defined __NO_MATH_ERRNO__
#  define math_errhandling	(MATH_ERREXCEPT)
# else
#  define math_errhandling	(MATH_ERRNO | MATH_ERREXCEPT)
# endif

#endif /* Use ISO C99.  */

#if __GLIBC_USE (IEC_60559_BFP_EXT_C2X)
# include <bits/iscanonical.h>

/* Return nonzero value if X is a signaling NaN.  */
# ifndef __cplusplus
#  define issignaling(x) __MATH_TG ((x), __issignaling, (x))
# else
   /* In C++ mode, __MATH_TG cannot be used, because it relies on
      __builtin_types_compatible_p, which is a C-only builtin.  On the
      other hand, overloading provides the means to distinguish between
      the floating-point types.  The overloading resolution will match
      the correct parameter (regardless of type qualifiers (i.e.: const
      and volatile)).  */
extern "C++" {
inline int issignaling (float __val) { return __issignalingf (__val); }
inline int issignaling (double __val) { return __issignaling (__val); }
inline int
issignaling (long double __val)
{
#  ifdef __NO_LONG_DOUBLE_MATH
  return __issignaling (__val);
#  else
  return __issignalingl (__val);
#  endif
}
#  if __HAVE_FLOAT128_UNLIKE_LDBL
/* When using an IEEE 128-bit long double, _Float128 is defined as long double
   in C++.  */
inline int issignaling (_Float128 __val) { return __issignalingf128 (__val); }
#  endif
} /* extern C++ */
# endif

/* Return nonzero value if X is subnormal.  */
# define issubnormal(x) (fpclassify (x) == FP_SUBNORMAL)

/* Return nonzero value if X is zero.  */
# ifndef __cplusplus
#  ifdef __SUPPORT_SNAN__
#   define iszero(x) (fpclassify (x) == FP_ZERO)
#  else
#   define iszero(x) (((__typeof (x)) (x)) == 0)
#  endif
# else	/* __cplusplus */
extern "C++" {
#  ifdef __SUPPORT_SNAN__
inline int
iszero (float __val)
{
  return __fpclassifyf (__val) == FP_ZERO;
}
inline int
iszero (double __val)
{
  return __fpclassify (__val) == FP_ZERO;
}
inline int
iszero (long double __val)
{
#   ifdef __NO_LONG_DOUBLE_MATH
  return __fpclassify (__val) == FP_ZERO;
#   else
  return __fpclassifyl (__val) == FP_ZERO;
#   endif
}
#   if __HAVE_FLOAT128_UNLIKE_LDBL
  /* When using an IEEE 128-bit long double, _Float128 is defined as long double
     in C++.  */
inline int
iszero (_Float128 __val)
{
  return __fpclassifyf128 (__val) == FP_ZERO;
}
#   endif
#  else
template <class __T> inline bool
iszero (__T __val)
{
  return __val == 0;
}
#  endif
} /* extern C++ */
# endif	/* __cplusplus */
#endif /* Use IEC_60559_BFP_EXT.  */

#ifdef __USE_XOPEN
/* X/Open wants another strange constant.  */
# define MAXFLOAT	3.40282347e+38F
#endif


/* Some useful constants.  */
#if defined __USE_MISC || defined __USE_XOPEN
# define M_E		2.7182818284590452354	/* e */
# define M_LOG2E	1.4426950408889634074	/* log_2 e */
# define M_LOG10E	0.43429448190325182765	/* log_10 e */
# define M_LN2		0.69314718055994530942	/* log_e 2 */
# define M_LN10		2.30258509299404568402	/* log_e 10 */
# define M_PI		3.14159265358979323846	/* pi */
# define M_PI_2		1.57079632679489661923	/* pi/2 */
# define M_PI_4		0.78539816339744830962	/* pi/4 */
# define M_1_PI		0.31830988618379067154	/* 1/pi */
# define M_2_PI		0.63661977236758134308	/* 2/pi */
# define M_2_SQRTPI	1.12837916709551257390	/* 2/sqrt(pi) */
# define M_SQRT2	1.41421356237309504880	/* sqrt(2) */
# define M_SQRT1_2	0.70710678118654752440	/* 1/sqrt(2) */
#endif

/* The above constants are not adequate for computation using `long double's.
   Therefore we provide as an extension constants with similar names as a
   GNU extension.  Provide enough digits for the 128-bit IEEE quad.  */
#ifdef __USE_GNU
# define M_El		2.718281828459045235360287471352662498L /* e */
# define M_LOG2El	1.442695040888963407359924681001892137L /* log_2 e */
# define M_LOG10El	0.434294481903251827651128918916605082L /* log_10 e */
# define M_LN2l		0.693147180559945309417232121458176568L /* log_e 2 */
# define M_LN10l	2.302585092994045684017991454684364208L /* log_e 10 */
# define M_PIl		3.141592653589793238462643383279502884L /* pi */
# define M_PI_2l	1.570796326794896619231321691639751442L /* pi/2 */
# define M_PI_4l	0.785398163397448309615660845819875721L /* pi/4 */
# define M_1_PIl	0.318309886183790671537767526745028724L /* 1/pi */
# define M_2_PIl	0.636619772367581343075535053490057448L /* 2/pi */
# define M_2_SQRTPIl	1.128379167095512573896158903121545172L /* 2/sqrt(pi) */
# define M_SQRT2l	1.414213562373095048801688724209698079L /* sqrt(2) */
# define M_SQRT1_2l	0.707106781186547524400844362104849039L /* 1/sqrt(2) */
#endif

#if __HAVE_FLOAT16 && defined __USE_GNU
# define M_Ef16		__f16 (2.718281828459045235360287471352662498) /* e */
# define M_LOG2Ef16	__f16 (1.442695040888963407359924681001892137) /* log_2 e */
# define M_LOG10Ef16	__f16 (0.434294481903251827651128918916605082) /* log_10 e */
# define M_LN2f16	__f16 (0.693147180559945309417232121458176568) /* log_e 2 */
# define M_LN10f16	__f16 (2.302585092994045684017991454684364208) /* log_e 10 */
# define M_PIf16	__f16 (3.141592653589793238462643383279502884) /* pi */
# define M_PI_2f16	__f16 (1.570796326794896619231321691639751442) /* pi/2 */
# define M_PI_4f16	__f16 (0.785398163397448309615660845819875721) /* pi/4 */
# define M_1_PIf16	__f16 (0.318309886183790671537767526745028724) /* 1/pi */
# define M_2_PIf16	__f16 (0.636619772367581343075535053490057448) /* 2/pi */
# define M_2_SQRTPIf16	__f16 (1.128379167095512573896158903121545172) /* 2/sqrt(pi) */
# define M_SQRT2f16	__f16 (1.414213562373095048801688724209698079) /* sqrt(2) */
# define M_SQRT1_2f16	__f16 (0.707106781186547524400844362104849039) /* 1/sqrt(2) */
#endif

#if __HAVE_FLOAT32 && defined __USE_GNU
# define M_Ef32		__f32 (2.718281828459045235360287471352662498) /* e */
# define M_LOG2Ef32	__f32 (1.442695040888963407359924681001892137) /* log_2 e */
# define M_LOG10Ef32	__f32 (0.434294481903251827651128918916605082) /* log_10 e */
# define M_LN2f32	__f32 (0.693147180559945309417232121458176568) /* log_e 2 */
# define M_LN10f32	__f32 (2.302585092994045684017991454684364208) /* log_e 10 */
# define M_PIf32	__f32 (3.141592653589793238462643383279502884) /* pi */
# define M_PI_2f32	__f32 (1.570796326794896619231321691639751442) /* pi/2 */
# define M_PI_4f32	__f32 (0.785398163397448309615660845819875721) /* pi/4 */
# define M_1_PIf32	__f32 (0.318309886183790671537767526745028724) /* 1/pi */
# define M_2_PIf32	__f32 (0.636619772367581343075535053490057448) /* 2/pi */
# define M_2_SQRTPIf32	__f32 (1.128379167095512573896158903121545172) /* 2/sqrt(pi) */
# define M_SQRT2f32	__f32 (1.414213562373095048801688724209698079) /* sqrt(2) */
# define M_SQRT1_2f32	__f32 (0.707106781186547524400844362104849039) /* 1/sqrt(2) */
#endif

#if __HAVE_FLOAT64 && defined __USE_GNU
# define M_Ef64		__f64 (2.718281828459045235360287471352662498) /* e */
# define M_LOG2Ef64	__f64 (1.442695040888963407359924681001892137) /* log_2 e */
# define M_LOG10Ef64	__f64 (0.434294481903251827651128918916605082) /* log_10 e */
# define M_LN2f64	__f64 (0.693147180559945309417232121458176568) /* log_e 2 */
# define M_LN10f64	__f64 (2.302585092994045684017991454684364208) /* log_e 10 */
# define M_PIf64	__f64 (3.141592653589793238462643383279502884) /* pi */
# define M_PI_2f64	__f64 (1.570796326794896619231321691639751442) /* pi/2 */
# define M_PI_4f64	__f64 (0.785398163397448309615660845819875721) /* pi/4 */
# define M_1_PIf64	__f64 (0.318309886183790671537767526745028724) /* 1/pi */
# define M_2_PIf64	__f64 (0.636619772367581343075535053490057448) /* 2/pi */
# define M_2_SQRTPIf64	__f64 (1.128379167095512573896158903121545172) /* 2/sqrt(pi) */
# define M_SQRT2f64	__f64 (1.414213562373095048801688724209698079) /* sqrt(2) */
# define M_SQRT1_2f64	__f64 (0.707106781186547524400844362104849039) /* 1/sqrt(2) */
#endif

#if __HAVE_FLOAT128 && defined __USE_GNU
# define M_Ef128	__f128 (2.718281828459045235360287471352662498) /* e */
# define M_LOG2Ef128	__f128 (1.442695040888963407359924681001892137) /* log_2 e */
# define M_LOG10Ef128	__f128 (0.434294481903251827651128918916605082) /* log_10 e */
# define M_LN2f128	__f128 (0.693147180559945309417232121458176568) /* log_e 2 */
# define M_LN10f128	__f128 (2.302585092994045684017991454684364208) /* log_e 10 */
# define M_PIf128	__f128 (3.141592653589793238462643383279502884) /* pi */
# define M_PI_2f128	__f128 (1.570796326794896619231321691639751442) /* pi/2 */
# define M_PI_4f128	__f128 (0.785398163397448309615660845819875721) /* pi/4 */
# define M_1_PIf128	__f128 (0.318309886183790671537767526745028724) /* 1/pi */
# define M_2_PIf128	__f128 (0.636619772367581343075535053490057448) /* 2/pi */
# define M_2_SQRTPIf128	__f128 (1.128379167095512573896158903121545172) /* 2/sqrt(pi) */
# define M_SQRT2f128	__f128 (1.414213562373095048801688724209698079) /* sqrt(2) */
# define M_SQRT1_2f128	__f128 (0.707106781186547524400844362104849039) /* 1/sqrt(2) */
#endif

#if __HAVE_FLOAT32X && defined __USE_GNU
# define M_Ef32x	__f32x (2.718281828459045235360287471352662498) /* e */
# define M_LOG2Ef32x	__f32x (1.442695040888963407359924681001892137) /* log_2 e */
# define M_LOG10Ef32x	__f32x (0.434294481903251827651128918916605082) /* log_10 e */
# define M_LN2f32x	__f32x (0.693147180559945309417232121458176568) /* log_e 2 */
# define M_LN10f32x	__f32x (2.302585092994045684017991454684364208) /* log_e 10 */
# define M_PIf32x	__f32x (3.141592653589793238462643383279502884) /* pi */
# define M_PI_2f32x	__f32x (1.570796326794896619231321691639751442) /* pi/2 */
# define M_PI_4f32x	__f32x (0.785398163397448309615660845819875721) /* pi/4 */
# define M_1_PIf32x	__f32x (0.318309886183790671537767526745028724) /* 1/pi */
# define M_2_PIf32x	__f32x (0.636619772367581343075535053490057448) /* 2/pi */
# define M_2_SQRTPIf32x	__f32x (1.128379167095512573896158903121545172) /* 2/sqrt(pi) */
# define M_SQRT2f32x	__f32x (1.414213562373095048801688724209698079) /* sqrt(2) */
# define M_SQRT1_2f32x	__f32x (0.707106781186547524400844362104849039) /* 1/sqrt(2) */
#endif

#if __HAVE_FLOAT64X && defined __USE_GNU
# define M_Ef64x	__f64x (2.718281828459045235360287471352662498) /* e */
# define M_LOG2Ef64x	__f64x (1.442695040888963407359924681001892137) /* log_2 e */
# define M_LOG10Ef64x	__f64x (0.434294481903251827651128918916605082) /* log_10 e */
# define M_LN2f64x	__f64x (0.693147180559945309417232121458176568) /* log_e 2 */
# define M_LN10f64x	__f64x (2.302585092994045684017991454684364208) /* log_e 10 */
# define M_PIf64x	__f64x (3.141592653589793238462643383279502884) /* pi */
# define M_PI_2f64x	__f64x (1.570796326794896619231321691639751442) /* pi/2 */
# define M_PI_4f64x	__f64x (0.785398163397448309615660845819875721) /* pi/4 */
# define M_1_PIf64x	__f64x (0.318309886183790671537767526745028724) /* 1/pi */
# define M_2_PIf64x	__f64x (0.636619772367581343075535053490057448) /* 2/pi */
# define M_2_SQRTPIf64x	__f64x (1.128379167095512573896158903121545172) /* 2/sqrt(pi) */
# define M_SQRT2f64x	__f64x (1.414213562373095048801688724209698079) /* sqrt(2) */
# define M_SQRT1_2f64x	__f64x (0.707106781186547524400844362104849039) /* 1/sqrt(2) */
#endif

#if __HAVE_FLOAT128X && defined __USE_GNU
# error "M_* values needed for _Float128x"
#endif

/* When compiling in strict ISO C compatible mode we must not use the
   inline functions since they, among other things, do not set the
   `errno' variable correctly.  */
#if defined __STRICT_ANSI__ && !defined __NO_MATH_INLINES
# define __NO_MATH_INLINES	1
#endif

#ifdef __USE_ISOC99
# if __GNUC_PREREQ (3, 1)
/* ISO C99 defines some macros to compare number while taking care for
   unordered numbers.  Many FPUs provide special instructions to support
   these operations.  Generic support in GCC for these as builtins went
   in 2.97, but not all cpus added their patterns until 3.1.  Therefore
   we enable the builtins from 3.1 onwards and use a generic implementation
   othwerwise.  */
#  define isgreater(x, y)	__builtin_isgreater(x, y)
#  define isgreaterequal(x, y)	__builtin_isgreaterequal(x, y)
#  define isless(x, y)		__builtin_isless(x, y)
#  define islessequal(x, y)	__builtin_islessequal(x, y)
#  define islessgreater(x, y)	__builtin_islessgreater(x, y)
#  define isunordered(x, y)	__builtin_isunordered(x, y)
# else
#  define isgreater(x, y) \
  (__extension__ ({ __typeof__ (x) __x = (x); __typeof__ (y) __y = (y); \
		    !isunordered (__x, __y) && __x > __y; }))
#  define isgreaterequal(x, y) \
  (__extension__ ({ __typeof__ (x) __x = (x); __typeof__ (y) __y = (y); \
		    !isunordered (__x, __y) && __x >= __y; }))
#  define isless(x, y) \
  (__extension__ ({ __typeof__ (x) __x = (x); __typeof__ (y) __y = (y); \
		    !isunordered (__x, __y) && __x < __y; }))
#  define islessequal(x, y) \
  (__extension__ ({ __typeof__ (x) __x = (x); __typeof__ (y) __y = (y); \
		    !isunordered (__x, __y) && __x <= __y; }))
#  define islessgreater(x, y) \
  (__extension__ ({ __typeof__ (x) __x = (x); __typeof__ (y) __y = (y); \
		    !isunordered (__x, __y) && __x != __y; }))
/* isunordered must always check both operands first for signaling NaNs.  */
#  define isunordered(x, y) \
  (__extension__ ({ __typeof__ (x) __u = (x); __typeof__ (y) __v = (y); \
		    __u != __v && (__u != __u || __v != __v); }))
# endif
#endif

/* Get machine-dependent inline versions (if there are any).  */
#ifdef __USE_EXTERN_INLINES
# include <bits/mathinline.h>
#endif


#if __GLIBC_USE (IEC_60559_BFP_EXT_C2X)
/* An expression whose type has the widest of the evaluation formats
   of X and Y (which are of floating-point types).  */
# if __FLT_EVAL_METHOD__ == 2 || __FLT_EVAL_METHOD__ > 64
#  define __MATH_EVAL_FMT2(x, y) ((x) + (y) + 0.0L)
# elif __FLT_EVAL_METHOD__ == 1 || __FLT_EVAL_METHOD__ > 32
#  define __MATH_EVAL_FMT2(x, y) ((x) + (y) + 0.0)
# elif __FLT_EVAL_METHOD__ == 0 || __FLT_EVAL_METHOD__ == 32
#  define __MATH_EVAL_FMT2(x, y) ((x) + (y) + 0.0f)
# else
#  define __MATH_EVAL_FMT2(x, y) ((x) + (y))
# endif

/* Return X == Y but raising "invalid" and setting errno if X or Y is
   a NaN.  */
# if !defined __cplusplus || (__cplusplus < 201103L && !defined __GNUC__)
#  define iseqsig(x, y) \
   __MATH_TG (__MATH_EVAL_FMT2 (x, y), __iseqsig, ((x), (y)))
# else
/* In C++ mode, __MATH_TG cannot be used, because it relies on
   __builtin_types_compatible_p, which is a C-only builtin.  Moreover,
   the comparison macros from ISO C take two floating-point arguments,
   which need not have the same type.  Choosing what underlying function
   to call requires evaluating the formats of the arguments, then
   selecting which is wider.  The macro __MATH_EVAL_FMT2 provides this
   information, however, only the type of the macro expansion is
   relevant (actually evaluating the expression would be incorrect).
   Thus, the type is used as a template parameter for __iseqsig_type,
   which calls the appropriate underlying function.  */
extern "C++" {
template<typename> struct __iseqsig_type;

template<> struct __iseqsig_type<float>
{
  static int __call (float __x, float __y) throw ()
  {
    return __iseqsigf (__x, __y);
  }
};

template<> struct __iseqsig_type<double>
{
  static int __call (double __x, double __y) throw ()
  {
    return __iseqsig (__x, __y);
  }
};

template<> struct __iseqsig_type<long double>
{
  static int __call (long double __x, long double __y) throw ()
  {
#  ifndef __NO_LONG_DOUBLE_MATH
    return __iseqsigl (__x, __y);
#  else
    return __iseqsig (__x, __y);
#  endif
  }
};

#  if __HAVE_FLOAT128_UNLIKE_LDBL
  /* When using an IEEE 128-bit long double, _Float128 is defined as long double
     in C++.  */
template<> struct __iseqsig_type<_Float128>
{
  static int __call (_Float128 __x, _Float128 __y) throw ()
  {
    return __iseqsigf128 (__x, __y);
  }
};
#  endif

template<typename _T1, typename _T2>
inline int
iseqsig (_T1 __x, _T2 __y) throw ()
{
#  if __cplusplus >= 201103L
  typedef decltype (__MATH_EVAL_FMT2 (__x, __y)) _T3;
#  else
  typedef __typeof (__MATH_EVAL_FMT2 (__x, __y)) _T3;
#  endif
  return __iseqsig_type<_T3>::__call (__x, __y);
}

} /* extern "C++" */
# endif /* __cplusplus */

#endif

__END_DECLS


#endif /* math.h  */
