/* Prototype declarations for math functions; helper file for <math.h>.
   Copyright (C) 1996-2020 Free Software Foundation, Inc.
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

/* NOTE: Because of the special way this file is used by <math.h>, this
   file must NOT be protected from multiple inclusion as header files
   usually are.

   This file provides prototype declarations for the math functions.
   Most functions are declared using the macro:

   __MATHCALL (NAME,[_r], (ARGS...));

   This means there is a function `NAME' returning `double' and a function
   `NAMEf' returning `float'.  Each place `_Mdouble_' appears in the
   prototype, that is actually `double' in the prototype for `NAME' and
   `float' in the prototype for `NAMEf'.  Reentrant variant functions are
   called `NAME_r' and `NAMEf_r'.

   Functions returning other types like `int' are declared using the macro:

   __MATHDECL (TYPE, NAME,[_r], (ARGS...));

   This is just like __MATHCALL but for a function returning `TYPE'
   instead of `_Mdouble_'.  In all of these cases, there is still
   both a `NAME' and a `NAMEf' that takes `float' arguments.

   Note that there must be no whitespace before the argument passed for
   NAME, to make token pasting work with -traditional.  */

#ifndef _MATH_H
# error "Never include <bits/mathcalls.h> directly; include <math.h> instead."
#endif


/* Trigonometric functions.  */

/* Arc cosine of X.  */
__MATHCALL (acos,, (_Mdouble_ __x));
/* Arc sine of X.  */
__MATHCALL (asin,, (_Mdouble_ __x));
/* Arc tangent of X.  */
__MATHCALL (atan,, (_Mdouble_ __x));
/* Arc tangent of Y/X.  */
__MATHCALL (atan2,, (_Mdouble_ __y, _Mdouble_ __x));

/* Cosine of X.  */
__MATHCALL_VEC (cos,, (_Mdouble_ __x));
/* Sine of X.  */
__MATHCALL_VEC (sin,, (_Mdouble_ __x));
/* Tangent of X.  */
__MATHCALL (tan,, (_Mdouble_ __x));

/* Hyperbolic functions.  */

/* Hyperbolic cosine of X.  */
__MATHCALL (cosh,, (_Mdouble_ __x));
/* Hyperbolic sine of X.  */
__MATHCALL (sinh,, (_Mdouble_ __x));
/* Hyperbolic tangent of X.  */
__MATHCALL (tanh,, (_Mdouble_ __x));

#ifdef __USE_GNU
/* Cosine and sine of X.  */
__MATHDECL_VEC (void,sincos,,
		(_Mdouble_ __x, _Mdouble_ *__sinx, _Mdouble_ *__cosx));
#endif

#if defined __USE_XOPEN_EXTENDED || defined __USE_ISOC99
/* Hyperbolic arc cosine of X.  */
__MATHCALL (acosh,, (_Mdouble_ __x));
/* Hyperbolic arc sine of X.  */
__MATHCALL (asinh,, (_Mdouble_ __x));
/* Hyperbolic arc tangent of X.  */
__MATHCALL (atanh,, (_Mdouble_ __x));
#endif

/* Exponential and logarithmic functions.  */

/* Exponential function of X.  */
__MATHCALL_VEC (exp,, (_Mdouble_ __x));

/* Break VALUE into a normalized fraction and an integral power of 2.  */
__MATHCALL (frexp,, (_Mdouble_ __x, int *__exponent));

/* X times (two to the EXP power).  */
__MATHCALL (ldexp,, (_Mdouble_ __x, int __exponent));

/* Natural logarithm of X.  */
__MATHCALL_VEC (log,, (_Mdouble_ __x));

/* Base-ten logarithm of X.  */
__MATHCALL (log10,, (_Mdouble_ __x));

/* Break VALUE into integral and fractional parts.  */
__MATHCALL (modf,, (_Mdouble_ __x, _Mdouble_ *__iptr)) __nonnull ((2));

#if __GLIBC_USE (IEC_60559_FUNCS_EXT_C2X)
/* Compute exponent to base ten.  */
__MATHCALL (exp10,, (_Mdouble_ __x));
#endif

#if defined __USE_XOPEN_EXTENDED || defined __USE_ISOC99
/* Return exp(X) - 1.  */
__MATHCALL (expm1,, (_Mdouble_ __x));

/* Return log(1 + X).  */
__MATHCALL (log1p,, (_Mdouble_ __x));

/* Return the base 2 signed integral exponent of X.  */
__MATHCALL (logb,, (_Mdouble_ __x));
#endif

#ifdef __USE_ISOC99
/* Compute base-2 exponential of X.  */
__MATHCALL (exp2,, (_Mdouble_ __x));

/* Compute base-2 logarithm of X.  */
__MATHCALL (log2,, (_Mdouble_ __x));
#endif


/* Power functions.  */

/* Return X to the Y power.  */
__MATHCALL_VEC (pow,, (_Mdouble_ __x, _Mdouble_ __y));

/* Return the square root of X.  */
__MATHCALL (sqrt,, (_Mdouble_ __x));

#if defined __USE_XOPEN || defined __USE_ISOC99
/* Return `sqrt(X*X + Y*Y)'.  */
__MATHCALL (hypot,, (_Mdouble_ __x, _Mdouble_ __y));
#endif

#if defined __USE_XOPEN_EXTENDED || defined __USE_ISOC99
/* Return the cube root of X.  */
__MATHCALL (cbrt,, (_Mdouble_ __x));
#endif


/* Nearest integer, absolute value, and remainder functions.  */

/* Smallest integral value not less than X.  */
__MATHCALLX (ceil,, (_Mdouble_ __x), (__const__));

/* Absolute value of X.  */
__MATHCALLX (fabs,, (_Mdouble_ __x), (__const__));

/* Largest integer not greater than X.  */
__MATHCALLX (floor,, (_Mdouble_ __x), (__const__));

/* Floating-point modulo remainder of X/Y.  */
__MATHCALL (fmod,, (_Mdouble_ __x, _Mdouble_ __y));

#ifdef __USE_MISC
# if ((!defined __cplusplus \
       || __cplusplus < 201103L /* isinf conflicts with C++11.  */ \
       || __MATH_DECLARING_DOUBLE == 0)) /* isinff or isinfl don't.  */ \
      && !__MATH_DECLARING_FLOATN
/* Return 0 if VALUE is finite or NaN, +1 if it
   is +Infinity, -1 if it is -Infinity.  */
__MATHDECL_1 (int,isinf,, (_Mdouble_ __value)) __attribute__ ((__const__));
# endif

# if !__MATH_DECLARING_FLOATN
/* Return nonzero if VALUE is finite and not NaN.  */
__MATHDECL_1 (int,finite,, (_Mdouble_ __value)) __attribute__ ((__const__));

/* Return the remainder of X/Y.  */
__MATHCALL (drem,, (_Mdouble_ __x, _Mdouble_ __y));


/* Return the fractional part of X after dividing out `ilogb (X)'.  */
__MATHCALL (significand,, (_Mdouble_ __x));
# endif

#endif /* Use misc.  */

#ifdef __USE_ISOC99
/* Return X with its signed changed to Y's.  */
__MATHCALLX (copysign,, (_Mdouble_ __x, _Mdouble_ __y), (__const__));
#endif

#ifdef __USE_ISOC99
/* Return representation of qNaN for double type.  */
__MATHCALL (nan,, (const char *__tagb));
#endif


#if defined __USE_MISC || (defined __USE_XOPEN && !defined __USE_XOPEN2K)
# if ((!defined __cplusplus \
       || __cplusplus < 201103L /* isnan conflicts with C++11.  */ \
       || __MATH_DECLARING_DOUBLE == 0)) /* isnanf or isnanl don't.  */ \
      && !__MATH_DECLARING_FLOATN
/* Return nonzero if VALUE is not a number.  */
__MATHDECL_1 (int,isnan,, (_Mdouble_ __value)) __attribute__ ((__const__));
# endif
#endif

#if defined __USE_MISC || (defined __USE_XOPEN && __MATH_DECLARING_DOUBLE)
/* Bessel functions.  */
__MATHCALL (j0,, (_Mdouble_));
__MATHCALL (j1,, (_Mdouble_));
__MATHCALL (jn,, (int, _Mdouble_));
__MATHCALL (y0,, (_Mdouble_));
__MATHCALL (y1,, (_Mdouble_));
__MATHCALL (yn,, (int, _Mdouble_));
#endif


#if defined __USE_XOPEN || defined __USE_ISOC99
/* Error and gamma functions.  */
__MATHCALL (erf,, (_Mdouble_));
__MATHCALL (erfc,, (_Mdouble_));
__MATHCALL (lgamma,, (_Mdouble_));
#endif

#ifdef __USE_ISOC99
/* True gamma function.  */
__MATHCALL (tgamma,, (_Mdouble_));
#endif

#if defined __USE_MISC || (defined __USE_XOPEN && !defined __USE_XOPEN2K)
# if !__MATH_DECLARING_FLOATN
/* Obsolete alias for `lgamma'.  */
__MATHCALL (gamma,, (_Mdouble_));
# endif
#endif

#ifdef __USE_MISC
/* Reentrant version of lgamma.  This function uses the global variable
   `signgam'.  The reentrant version instead takes a pointer and stores
   the value through it.  */
__MATHCALL (lgamma,_r, (_Mdouble_, int *__signgamp));
#endif


#if defined __USE_XOPEN_EXTENDED || defined __USE_ISOC99
/* Return the integer nearest X in the direction of the
   prevailing rounding mode.  */
__MATHCALL (rint,, (_Mdouble_ __x));

/* Return X + epsilon if X < Y, X - epsilon if X > Y.  */
__MATHCALL (nextafter,, (_Mdouble_ __x, _Mdouble_ __y));
# if defined __USE_ISOC99 && !defined __LDBL_COMPAT && !__MATH_DECLARING_FLOATN
__MATHCALL (nexttoward,, (_Mdouble_ __x, long double __y));
# endif

# if __GLIBC_USE (IEC_60559_BFP_EXT_C2X) || __MATH_DECLARING_FLOATN
/* Return X - epsilon.  */
__MATHCALL (nextdown,, (_Mdouble_ __x));
/* Return X + epsilon.  */
__MATHCALL (nextup,, (_Mdouble_ __x));
# endif

/* Return the remainder of integer divison X / Y with infinite precision.  */
__MATHCALL (remainder,, (_Mdouble_ __x, _Mdouble_ __y));

# ifdef __USE_ISOC99
/* Return X times (2 to the Nth power).  */
__MATHCALL (scalbn,, (_Mdouble_ __x, int __n));
# endif

/* Return the binary exponent of X, which must be nonzero.  */
__MATHDECL (int,ilogb,, (_Mdouble_ __x));
#endif

#if __GLIBC_USE (IEC_60559_BFP_EXT_C2X) || __MATH_DECLARING_FLOATN
/* Like ilogb, but returning long int.  */
__MATHDECL (long int, llogb,, (_Mdouble_ __x));
#endif

#ifdef __USE_ISOC99
/* Return X times (2 to the Nth power).  */
__MATHCALL (scalbln,, (_Mdouble_ __x, long int __n));

/* Round X to integral value in floating-point format using current
   rounding direction, but do not raise inexact exception.  */
__MATHCALL (nearbyint,, (_Mdouble_ __x));

/* Round X to nearest integral value, rounding halfway cases away from
   zero.  */
__MATHCALLX (round,, (_Mdouble_ __x), (__const__));

/* Round X to the integral value in floating-point format nearest but
   not larger in magnitude.  */
__MATHCALLX (trunc,, (_Mdouble_ __x), (__const__));

/* Compute remainder of X and Y and put in *QUO a value with sign of x/y
   and magnitude congruent `mod 2^n' to the magnitude of the integral
   quotient x/y, with n >= 3.  */
__MATHCALL (remquo,, (_Mdouble_ __x, _Mdouble_ __y, int *__quo));


/* Conversion functions.  */

/* Round X to nearest integral value according to current rounding
   direction.  */
__MATHDECL (long int,lrint,, (_Mdouble_ __x));
__extension__
__MATHDECL (long long int,llrint,, (_Mdouble_ __x));

/* Round X to nearest integral value, rounding halfway cases away from
   zero.  */
__MATHDECL (long int,lround,, (_Mdouble_ __x));
__extension__
__MATHDECL (long long int,llround,, (_Mdouble_ __x));


/* Return positive difference between X and Y.  */
__MATHCALL (fdim,, (_Mdouble_ __x, _Mdouble_ __y));

/* Return maximum numeric value from X and Y.  */
__MATHCALLX (fmax,, (_Mdouble_ __x, _Mdouble_ __y), (__const__));

/* Return minimum numeric value from X and Y.  */
__MATHCALLX (fmin,, (_Mdouble_ __x, _Mdouble_ __y), (__const__));

/* Multiply-add function computed as a ternary operation.  */
__MATHCALL (fma,, (_Mdouble_ __x, _Mdouble_ __y, _Mdouble_ __z));
#endif /* Use ISO C99.  */

#if __GLIBC_USE (IEC_60559_BFP_EXT_C2X) || __MATH_DECLARING_FLOATN
/* Round X to nearest integer value, rounding halfway cases to even.  */
__MATHCALLX (roundeven,, (_Mdouble_ __x), (__const__));

/* Round X to nearest signed integer value, not raising inexact, with
   control of rounding direction and width of result.  */
__MATHDECL (__intmax_t, fromfp,, (_Mdouble_ __x, int __round,
				  unsigned int __width));

/* Round X to nearest unsigned integer value, not raising inexact,
   with control of rounding direction and width of result.  */
__MATHDECL (__uintmax_t, ufromfp,, (_Mdouble_ __x, int __round,
				    unsigned int __width));

/* Round X to nearest signed integer value, raising inexact for
   non-integers, with control of rounding direction and width of
   result.  */
__MATHDECL (__intmax_t, fromfpx,, (_Mdouble_ __x, int __round,
				   unsigned int __width));

/* Round X to nearest unsigned integer value, raising inexact for
   non-integers, with control of rounding direction and width of
   result.  */
__MATHDECL (__uintmax_t, ufromfpx,, (_Mdouble_ __x, int __round,
				     unsigned int __width));

/* Return value with maximum magnitude.  */
__MATHCALLX (fmaxmag,, (_Mdouble_ __x, _Mdouble_ __y), (__const__));

/* Return value with minimum magnitude.  */
__MATHCALLX (fminmag,, (_Mdouble_ __x, _Mdouble_ __y), (__const__));

/* Canonicalize floating-point representation.  */
__MATHDECL_1 (int, canonicalize,, (_Mdouble_ *__cx, const _Mdouble_ *__x));
#endif

#if __GLIBC_USE (IEC_60559_BFP_EXT) || __MATH_DECLARING_FLOATN
/* Total order operation.  */
__MATHDECL_1 (int, totalorder,, (const _Mdouble_ *__x,
				 const _Mdouble_ *__y))
     __attribute_pure__;

/* Total order operation on absolute values.  */
__MATHDECL_1 (int, totalordermag,, (const _Mdouble_ *__x,
				    const _Mdouble_ *__y))
     __attribute_pure__;

/* Get NaN payload.  */
__MATHCALL (getpayload,, (const _Mdouble_ *__x));

/* Set quiet NaN payload.  */
__MATHDECL_1 (int, setpayload,, (_Mdouble_ *__x, _Mdouble_ __payload));

/* Set signaling NaN payload.  */
__MATHDECL_1 (int, setpayloadsig,, (_Mdouble_ *__x, _Mdouble_ __payload));
#endif

#if (defined __USE_MISC || (defined __USE_XOPEN_EXTENDED \
			    && __MATH_DECLARING_DOUBLE	  \
			    && !defined __USE_XOPEN2K8))  \
     && !__MATH_DECLARING_FLOATN
/* Return X times (2 to the Nth power).  */
__MATHCALL (scalb,, (_Mdouble_ __x, _Mdouble_ __n));
#endif
