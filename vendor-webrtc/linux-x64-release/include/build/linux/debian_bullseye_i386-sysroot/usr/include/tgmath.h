/* Copyright (C) 1997-2020 Free Software Foundation, Inc.
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
 *	ISO C99 Standard: 7.22 Type-generic math	<tgmath.h>
 */

#ifndef _TGMATH_H
#define _TGMATH_H	1

#define __GLIBC_INTERNAL_STARTING_HEADER_IMPLEMENTATION
#include <bits/libc-header-start.h>

/* Include the needed headers.  */
#include <bits/floatn.h>
#include <math.h>
#include <complex.h>


/* There are two variant implementations of type-generic macros in
   this file: one for GCC 8 and later, using __builtin_tgmath and
   where each macro expands each of its arguments only once, and one
   for older GCC, using other compiler extensions but with macros
   expanding their arguments many times (so resulting in exponential
   blowup of the size of expansions when calls to such macros are
   nested inside arguments to such macros).  */

#define __HAVE_BUILTIN_TGMATH __GNUC_PREREQ (8, 0)

#if __GNUC_PREREQ (2, 7)

/* Certain cases of narrowing macros only need to call a single
   function so cannot use __builtin_tgmath and do not need any
   complicated logic.  */
# if __HAVE_FLOAT128X
#  error "Unsupported _Float128x type for <tgmath.h>."
# endif
# if ((__HAVE_FLOAT64X && !__HAVE_FLOAT128)		\
      || (__HAVE_FLOAT128 && !__HAVE_FLOAT64X))
#  error "Unsupported combination of types for <tgmath.h>."
# endif
# define __TGMATH_2_NARROW_D(F, X, Y)		\
  (F ## l (X, Y))
# define __TGMATH_2_NARROW_F64X(F, X, Y)	\
  (F ## f128 (X, Y))
# if !__HAVE_FLOAT128
#  define __TGMATH_2_NARROW_F32X(F, X, Y)	\
  (F ## f64 (X, Y))
# endif

# if __HAVE_BUILTIN_TGMATH

#  if __HAVE_FLOAT16 && __GLIBC_USE (IEC_60559_TYPES_EXT)
#   define __TG_F16_ARG(X) X ## f16,
#  else
#   define __TG_F16_ARG(X)
#  endif
#  if __HAVE_FLOAT32 && __GLIBC_USE (IEC_60559_TYPES_EXT)
#   define __TG_F32_ARG(X) X ## f32,
#  else
#   define __TG_F32_ARG(X)
#  endif
#  if __HAVE_FLOAT64 && __GLIBC_USE (IEC_60559_TYPES_EXT)
#   define __TG_F64_ARG(X) X ## f64,
#  else
#   define __TG_F64_ARG(X)
#  endif
#  if __HAVE_FLOAT128 && __GLIBC_USE (IEC_60559_TYPES_EXT)
#   define __TG_F128_ARG(X) X ## f128,
#  else
#   define __TG_F128_ARG(X)
#  endif
#  if __HAVE_FLOAT32X && __GLIBC_USE (IEC_60559_TYPES_EXT)
#   define __TG_F32X_ARG(X) X ## f32x,
#  else
#   define __TG_F32X_ARG(X)
#  endif
#  if __HAVE_FLOAT64X && __GLIBC_USE (IEC_60559_TYPES_EXT)
#   define __TG_F64X_ARG(X) X ## f64x,
#  else
#   define __TG_F64X_ARG(X)
#  endif
#  if __HAVE_FLOAT128X && __GLIBC_USE (IEC_60559_TYPES_EXT)
#   define __TG_F128X_ARG(X) X ## f128x,
#  else
#   define __TG_F128X_ARG(X)
#  endif

#  define __TGMATH_FUNCS(X) X ## f, X, X ## l,				\
    __TG_F16_ARG (X) __TG_F32_ARG (X) __TG_F64_ARG (X) __TG_F128_ARG (X) \
    __TG_F32X_ARG (X) __TG_F64X_ARG (X) __TG_F128X_ARG (X)
#  define __TGMATH_RCFUNCS(F, C) __TGMATH_FUNCS (F) __TGMATH_FUNCS (C)
#  define __TGMATH_1(F, X) __builtin_tgmath (__TGMATH_FUNCS (F) (X))
#  define __TGMATH_2(F, X, Y) __builtin_tgmath (__TGMATH_FUNCS (F) (X), (Y))
#  define __TGMATH_2STD(F, X, Y) __builtin_tgmath (F ## f, F, F ## l, (X), (Y))
#  define __TGMATH_3(F, X, Y, Z) __builtin_tgmath (__TGMATH_FUNCS (F)	\
						   (X), (Y), (Z))
#  define __TGMATH_1C(F, C, X) __builtin_tgmath (__TGMATH_RCFUNCS (F, C) (X))
#  define __TGMATH_2C(F, C, X, Y) __builtin_tgmath (__TGMATH_RCFUNCS (F, C) \
						    (X), (Y))

#  define __TGMATH_NARROW_FUNCS_F(X) X, X ## l,
#  define __TGMATH_NARROW_FUNCS_F16(X)				\
    __TG_F32_ARG (X) __TG_F64_ARG (X) __TG_F128_ARG (X)		\
    __TG_F32X_ARG (X) __TG_F64X_ARG (X) __TG_F128X_ARG (X)
#  define __TGMATH_NARROW_FUNCS_F32(X)				\
    __TG_F64_ARG (X) __TG_F128_ARG (X)				\
    __TG_F32X_ARG (X) __TG_F64X_ARG (X) __TG_F128X_ARG (X)
#  define __TGMATH_NARROW_FUNCS_F64(X)		\
    __TG_F128_ARG (X)				\
    __TG_F64X_ARG (X) __TG_F128X_ARG (X)
#  define __TGMATH_NARROW_FUNCS_F32X(X)		\
    __TG_F64X_ARG (X) __TG_F128X_ARG (X)	\
    __TG_F64_ARG (X) __TG_F128_ARG (X)

#  define __TGMATH_2_NARROW_F(F, X, Y)				\
  __builtin_tgmath (__TGMATH_NARROW_FUNCS_F (F) (X), (Y))
#  define __TGMATH_2_NARROW_F16(F, X, Y)			\
  __builtin_tgmath (__TGMATH_NARROW_FUNCS_F16 (F) (X), (Y))
#  define __TGMATH_2_NARROW_F32(F, X, Y)			\
  __builtin_tgmath (__TGMATH_NARROW_FUNCS_F32 (F) (X), (Y))
#  define __TGMATH_2_NARROW_F64(F, X, Y)			\
  __builtin_tgmath (__TGMATH_NARROW_FUNCS_F64 (F) (X), (Y))
#  if __HAVE_FLOAT128
#   define __TGMATH_2_NARROW_F32X(F, X, Y)			\
  __builtin_tgmath (__TGMATH_NARROW_FUNCS_F32X (F) (X), (Y))
#  endif

# else /* !__HAVE_BUILTIN_TGMATH.  */

#  ifdef __NO_LONG_DOUBLE_MATH
#   define __tgml(fct) fct
#  else
#   define __tgml(fct) fct ## l
#  endif

/* __floating_type expands to 1 if TYPE is a floating type (including
   complex floating types), 0 if TYPE is an integer type (including
   complex integer types).  __real_integer_type expands to 1 if TYPE
   is a real integer type.  __complex_integer_type expands to 1 if
   TYPE is a complex integer type.  All these macros expand to integer
   constant expressions.  All these macros can assume their argument
   has an arithmetic type (not vector, decimal floating-point or
   fixed-point), valid to pass to tgmath.h macros.  */
#  if __GNUC_PREREQ (3, 1)
/* __builtin_classify_type expands to an integer constant expression
   in GCC 3.1 and later.  Default conversions applied to the argument
   of __builtin_classify_type mean it always returns 1 for real
   integer types rather than ever returning different values for
   character, boolean or enumerated types.  */
#   define __floating_type(type)				\
  (__builtin_classify_type (__real__ ((type) 0)) == 8)
#   define __real_integer_type(type)		\
  (__builtin_classify_type ((type) 0) == 1)
#   define __complex_integer_type(type)				\
  (__builtin_classify_type ((type) 0) == 9			\
   && __builtin_classify_type (__real__ ((type) 0)) == 1)
#  else
/* GCC versions predating __builtin_classify_type are also looser on
   what counts as an integer constant expression.  */
#   define __floating_type(type) (((type) 1.25) != 1)
#   define __real_integer_type(type) (((type) (1.25 + _Complex_I)) == 1)
#   define __complex_integer_type(type)			\
  (((type) (1.25 + _Complex_I)) == (1 + _Complex_I))
#  endif

/* Whether an expression (of arithmetic type) has a real type.  */
#  define __expr_is_real(E) (__builtin_classify_type (E) != 9)

/* The tgmath real type for T, where E is 0 if T is an integer type
   and 1 for a floating type.  If T has a complex type, it is
   unspecified whether the return type is real or complex (but it has
   the correct corresponding real type).  */
#  define __tgmath_real_type_sub(T, E) \
  __typeof__ (*(0 ? (__typeof__ (0 ? (double *) 0 : (void *) (E))) 0	      \
		  : (__typeof__ (0 ? (T *) 0 : (void *) (!(E)))) 0))

/* The tgmath real type of EXPR.  */
#  define __tgmath_real_type(expr) \
  __tgmath_real_type_sub (__typeof__ ((__typeof__ (+(expr))) 0),	      \
			  __floating_type (__typeof__ (+(expr))))

/* The tgmath complex type for T, where E1 is 1 if T has a floating
   type and 0 otherwise, E2 is 1 if T has a real integer type and 0
   otherwise, and E3 is 1 if T has a complex type and 0 otherwise.  */
#  define __tgmath_complex_type_sub(T, E1, E2, E3)			\
  __typeof__ (*(0							\
		? (__typeof__ (0 ? (T *) 0 : (void *) (!(E1)))) 0	\
		: (__typeof__ (0					\
			       ? (__typeof__ (0				\
					      ? (double *) 0		\
					      : (void *) (!(E2)))) 0	\
			       : (__typeof__ (0				\
					      ? (_Complex double *) 0	\
					      : (void *) (!(E3)))) 0)) 0))

/* The tgmath complex type of EXPR.  */
#  define __tgmath_complex_type(expr)					\
  __tgmath_complex_type_sub (__typeof__ ((__typeof__ (+(expr))) 0),	\
			     __floating_type (__typeof__ (+(expr))),	\
			     __real_integer_type (__typeof__ (+(expr))), \
			     __complex_integer_type (__typeof__ (+(expr))))

#  if (__HAVE_DISTINCT_FLOAT16			\
      || __HAVE_DISTINCT_FLOAT32		\
      || __HAVE_DISTINCT_FLOAT64		\
      || __HAVE_DISTINCT_FLOAT32X		\
      || __HAVE_DISTINCT_FLOAT64X		\
      || __HAVE_DISTINCT_FLOAT128X)
#   error "Unsupported _FloatN or _FloatNx types for <tgmath.h>."
#  endif

/* Expand to text that checks if ARG_COMB has type _Float128, and if
   so calls the appropriately suffixed FCT (which may include a cast),
   or FCT and CFCT for complex functions, with arguments ARG_CALL.  */
#  if __HAVE_DISTINCT_FLOAT128 && __GLIBC_USE (IEC_60559_TYPES_EXT)
#   if (!__HAVE_FLOAT64X			\
       || __HAVE_FLOAT64X_LONG_DOUBLE		\
       || !__HAVE_FLOATN_NOT_TYPEDEF)
#    define __TGMATH_F128(arg_comb, fct, arg_call)			\
  __builtin_types_compatible_p (__typeof (+(arg_comb)), _Float128)	\
  ? fct ## f128 arg_call :
#    define __TGMATH_CF128(arg_comb, fct, cfct, arg_call)		\
  __builtin_types_compatible_p (__typeof (+__real__ (arg_comb)), _Float128) \
  ? (__expr_is_real (arg_comb)						\
     ? fct ## f128 arg_call						\
     : cfct ## f128 arg_call) :
#   else
/* _Float64x is a distinct type at the C language level, which must be
   handled like _Float128.  */
#    define __TGMATH_F128(arg_comb, fct, arg_call)			\
  (__builtin_types_compatible_p (__typeof (+(arg_comb)), _Float128)	\
   || __builtin_types_compatible_p (__typeof (+(arg_comb)), _Float64x)) \
  ? fct ## f128 arg_call :
#    define __TGMATH_CF128(arg_comb, fct, cfct, arg_call)		\
  (__builtin_types_compatible_p (__typeof (+__real__ (arg_comb)), _Float128) \
   || __builtin_types_compatible_p (__typeof (+__real__ (arg_comb)),	\
				    _Float64x))				\
  ? (__expr_is_real (arg_comb)						\
     ? fct ## f128 arg_call						\
     : cfct ## f128 arg_call) :
#   endif
#  else
#   define __TGMATH_F128(arg_comb, fct, arg_call) /* Nothing.  */
#   define __TGMATH_CF128(arg_comb, fct, cfct, arg_call) /* Nothing.  */
#  endif

# endif /* !__HAVE_BUILTIN_TGMATH.  */

/* We have two kinds of generic macros: to support functions which are
   only defined on real valued parameters and those which are defined
   for complex functions as well.  */
# if __HAVE_BUILTIN_TGMATH

#  define __TGMATH_UNARY_REAL_ONLY(Val, Fct) __TGMATH_1 (Fct, (Val))
#  define __TGMATH_UNARY_REAL_RET_ONLY(Val, Fct) __TGMATH_1 (Fct, (Val))
#  define __TGMATH_BINARY_FIRST_REAL_ONLY(Val1, Val2, Fct)	\
  __TGMATH_2 (Fct, (Val1), (Val2))
#  define __TGMATH_BINARY_FIRST_REAL_STD_ONLY(Val1, Val2, Fct)	\
  __TGMATH_2STD (Fct, (Val1), (Val2))
#  define __TGMATH_BINARY_REAL_ONLY(Val1, Val2, Fct)	\
  __TGMATH_2 (Fct, (Val1), (Val2))
#  define __TGMATH_BINARY_REAL_STD_ONLY(Val1, Val2, Fct)	\
  __TGMATH_2STD (Fct, (Val1), (Val2))
#  define __TGMATH_TERNARY_FIRST_SECOND_REAL_ONLY(Val1, Val2, Val3, Fct) \
  __TGMATH_3 (Fct, (Val1), (Val2), (Val3))
#  define __TGMATH_TERNARY_REAL_ONLY(Val1, Val2, Val3, Fct)	\
  __TGMATH_3 (Fct, (Val1), (Val2), (Val3))
#  define __TGMATH_TERNARY_FIRST_REAL_RET_ONLY(Val1, Val2, Val3, Fct)	\
  __TGMATH_3 (Fct, (Val1), (Val2), (Val3))
#  define __TGMATH_UNARY_REAL_IMAG(Val, Fct, Cfct)	\
  __TGMATH_1C (Fct, Cfct, (Val))
#  define __TGMATH_UNARY_IMAG(Val, Cfct) __TGMATH_1 (Cfct, (Val))
#  define __TGMATH_UNARY_REAL_IMAG_RET_REAL(Val, Fct, Cfct)	\
  __TGMATH_1C (Fct, Cfct, (Val))
#  define __TGMATH_UNARY_REAL_IMAG_RET_REAL_SAME(Val, Cfct)	\
  __TGMATH_1 (Cfct, (Val))
#  define __TGMATH_BINARY_REAL_IMAG(Val1, Val2, Fct, Cfct)	\
  __TGMATH_2C (Fct, Cfct, (Val1), (Val2))

# else /* !__HAVE_BUILTIN_TGMATH.  */

#  define __TGMATH_UNARY_REAL_ONLY(Val, Fct)				\
  (__extension__ ((sizeof (+(Val)) == sizeof (double)			      \
		      || __builtin_classify_type (Val) != 8)		      \
		     ? (__tgmath_real_type (Val)) Fct (Val)		      \
		     : (sizeof (+(Val)) == sizeof (float))		      \
		     ? (__tgmath_real_type (Val)) Fct##f (Val)		      \
		     : __TGMATH_F128 ((Val), (__tgmath_real_type (Val)) Fct,  \
				      (Val))				      \
		     (__tgmath_real_type (Val)) __tgml(Fct) (Val)))

#  define __TGMATH_UNARY_REAL_RET_ONLY(Val, Fct) \
     (__extension__ ((sizeof (+(Val)) == sizeof (double)		      \
		      || __builtin_classify_type (Val) != 8)		      \
		     ? Fct (Val)					      \
		     : (sizeof (+(Val)) == sizeof (float))		      \
		     ? Fct##f (Val)					      \
		     : __TGMATH_F128 ((Val), Fct, (Val))		      \
		     __tgml(Fct) (Val)))

#  define __TGMATH_BINARY_FIRST_REAL_ONLY(Val1, Val2, Fct) \
     (__extension__ ((sizeof (+(Val1)) == sizeof (double)		      \
		      || __builtin_classify_type (Val1) != 8)		      \
		     ? (__tgmath_real_type (Val1)) Fct (Val1, Val2)	      \
		     : (sizeof (+(Val1)) == sizeof (float))		      \
		     ? (__tgmath_real_type (Val1)) Fct##f (Val1, Val2)	      \
		     : __TGMATH_F128 ((Val1), (__tgmath_real_type (Val1)) Fct, \
				    (Val1, Val2))			      \
		     (__tgmath_real_type (Val1)) __tgml(Fct) (Val1, Val2)))

#  define __TGMATH_BINARY_FIRST_REAL_STD_ONLY(Val1, Val2, Fct) \
     (__extension__ ((sizeof (+(Val1)) == sizeof (double)		      \
		      || __builtin_classify_type (Val1) != 8)		      \
		     ? (__tgmath_real_type (Val1)) Fct (Val1, Val2)	      \
		     : (sizeof (+(Val1)) == sizeof (float))		      \
		     ? (__tgmath_real_type (Val1)) Fct##f (Val1, Val2)	      \
		     : (__tgmath_real_type (Val1)) __tgml(Fct) (Val1, Val2)))

#  define __TGMATH_BINARY_REAL_ONLY(Val1, Val2, Fct) \
     (__extension__ ((sizeof ((Val1) + (Val2)) > sizeof (double)	      \
		      && __builtin_classify_type ((Val1) + (Val2)) == 8)      \
		     ? __TGMATH_F128 ((Val1) + (Val2),			      \
				      (__typeof				      \
				       ((__tgmath_real_type (Val1)) 0	      \
					+ (__tgmath_real_type (Val2)) 0)) Fct, \
				      (Val1, Val2))			      \
		     (__typeof ((__tgmath_real_type (Val1)) 0		      \
				+ (__tgmath_real_type (Val2)) 0))	      \
		     __tgml(Fct) (Val1, Val2)				      \
		     : (sizeof (+(Val1)) == sizeof (double)		      \
			|| sizeof (+(Val2)) == sizeof (double)		      \
			|| __builtin_classify_type (Val1) != 8		      \
			|| __builtin_classify_type (Val2) != 8)		      \
		     ? (__typeof ((__tgmath_real_type (Val1)) 0		      \
				   + (__tgmath_real_type (Val2)) 0))	      \
		       Fct (Val1, Val2)					      \
		     : (__typeof ((__tgmath_real_type (Val1)) 0		      \
				   + (__tgmath_real_type (Val2)) 0))	      \
		       Fct##f (Val1, Val2)))

#  define __TGMATH_BINARY_REAL_STD_ONLY(Val1, Val2, Fct) \
     (__extension__ ((sizeof ((Val1) + (Val2)) > sizeof (double)	      \
		      && __builtin_classify_type ((Val1) + (Val2)) == 8)      \
		     ? (__typeof ((__tgmath_real_type (Val1)) 0		      \
				  + (__tgmath_real_type (Val2)) 0))	      \
		       __tgml(Fct) (Val1, Val2)				      \
		     : (sizeof (+(Val1)) == sizeof (double)		      \
			|| sizeof (+(Val2)) == sizeof (double)		      \
			|| __builtin_classify_type (Val1) != 8		      \
			|| __builtin_classify_type (Val2) != 8)		      \
		     ? (__typeof ((__tgmath_real_type (Val1)) 0		      \
				   + (__tgmath_real_type (Val2)) 0))	      \
		       Fct (Val1, Val2)					      \
		     : (__typeof ((__tgmath_real_type (Val1)) 0		      \
				   + (__tgmath_real_type (Val2)) 0))	      \
		       Fct##f (Val1, Val2)))

#  define __TGMATH_TERNARY_FIRST_SECOND_REAL_ONLY(Val1, Val2, Val3, Fct) \
     (__extension__ ((sizeof ((Val1) + (Val2)) > sizeof (double)	      \
		      && __builtin_classify_type ((Val1) + (Val2)) == 8)      \
		     ? __TGMATH_F128 ((Val1) + (Val2),			      \
				      (__typeof				      \
				       ((__tgmath_real_type (Val1)) 0	      \
					+ (__tgmath_real_type (Val2)) 0)) Fct, \
				      (Val1, Val2, Val3))		      \
		     (__typeof ((__tgmath_real_type (Val1)) 0		      \
				+ (__tgmath_real_type (Val2)) 0))	      \
		     __tgml(Fct) (Val1, Val2, Val3)			      \
		     : (sizeof (+(Val1)) == sizeof (double)		      \
			|| sizeof (+(Val2)) == sizeof (double)		      \
			|| __builtin_classify_type (Val1) != 8		      \
			|| __builtin_classify_type (Val2) != 8)		      \
		     ? (__typeof ((__tgmath_real_type (Val1)) 0		      \
				   + (__tgmath_real_type (Val2)) 0))	      \
		       Fct (Val1, Val2, Val3)				      \
		     : (__typeof ((__tgmath_real_type (Val1)) 0		      \
				   + (__tgmath_real_type (Val2)) 0))	      \
		       Fct##f (Val1, Val2, Val3)))

#  define __TGMATH_TERNARY_REAL_ONLY(Val1, Val2, Val3, Fct) \
     (__extension__ ((sizeof ((Val1) + (Val2) + (Val3)) > sizeof (double)     \
		      && __builtin_classify_type ((Val1) + (Val2) + (Val3))   \
			 == 8)						      \
		     ? __TGMATH_F128 ((Val1) + (Val2) + (Val3),		      \
				      (__typeof				      \
				       ((__tgmath_real_type (Val1)) 0	      \
					+ (__tgmath_real_type (Val2)) 0	      \
					+ (__tgmath_real_type (Val3)) 0)) Fct, \
				      (Val1, Val2, Val3))		      \
		     (__typeof ((__tgmath_real_type (Val1)) 0		      \
				+ (__tgmath_real_type (Val2)) 0		      \
				+ (__tgmath_real_type (Val3)) 0))	      \
		       __tgml(Fct) (Val1, Val2, Val3)			      \
		     : (sizeof (+(Val1)) == sizeof (double)		      \
			|| sizeof (+(Val2)) == sizeof (double)		      \
			|| sizeof (+(Val3)) == sizeof (double)		      \
			|| __builtin_classify_type (Val1) != 8		      \
			|| __builtin_classify_type (Val2) != 8		      \
			|| __builtin_classify_type (Val3) != 8)		      \
		     ? (__typeof ((__tgmath_real_type (Val1)) 0		      \
				   + (__tgmath_real_type (Val2)) 0	      \
				   + (__tgmath_real_type (Val3)) 0))	      \
		       Fct (Val1, Val2, Val3)				      \
		     : (__typeof ((__tgmath_real_type (Val1)) 0		      \
				   + (__tgmath_real_type (Val2)) 0	      \
				   + (__tgmath_real_type (Val3)) 0))	      \
		       Fct##f (Val1, Val2, Val3)))

#  define __TGMATH_TERNARY_FIRST_REAL_RET_ONLY(Val1, Val2, Val3, Fct) \
     (__extension__ ((sizeof (+(Val1)) == sizeof (double)		\
		      || __builtin_classify_type (Val1) != 8)		\
		     ? Fct (Val1, Val2, Val3)				\
		     : (sizeof (+(Val1)) == sizeof (float))		\
		     ? Fct##f (Val1, Val2, Val3)			\
		     : __TGMATH_F128 ((Val1), Fct, (Val1, Val2, Val3))	\
		     __tgml(Fct) (Val1, Val2, Val3)))

/* XXX This definition has to be changed as soon as the compiler understands
   the imaginary keyword.  */
#  define __TGMATH_UNARY_REAL_IMAG(Val, Fct, Cfct) \
     (__extension__ ((sizeof (+__real__ (Val)) == sizeof (double)	      \
		      || __builtin_classify_type (__real__ (Val)) != 8)	      \
		     ? (__expr_is_real (Val)				      \
			? (__tgmath_complex_type (Val)) Fct (Val)	      \
			: (__tgmath_complex_type (Val)) Cfct (Val))	      \
		     : (sizeof (+__real__ (Val)) == sizeof (float))	      \
		     ? (__expr_is_real (Val)				      \
			? (__tgmath_complex_type (Val)) Fct##f (Val)	      \
			: (__tgmath_complex_type (Val)) Cfct##f (Val))	      \
		     : __TGMATH_CF128 ((Val),				      \
				       (__tgmath_complex_type (Val)) Fct,     \
				       (__tgmath_complex_type (Val)) Cfct,    \
				       (Val))				      \
		     (__expr_is_real (Val)				      \
		      ? (__tgmath_complex_type (Val)) __tgml(Fct) (Val)	      \
		      : (__tgmath_complex_type (Val)) __tgml(Cfct) (Val))))

#  define __TGMATH_UNARY_IMAG(Val, Cfct) \
     (__extension__ ((sizeof (+__real__ (Val)) == sizeof (double)	      \
		      || __builtin_classify_type (__real__ (Val)) != 8)	      \
		     ? (__typeof__ ((__tgmath_real_type (Val)) 0	      \
				    + _Complex_I)) Cfct (Val)		      \
		     : (sizeof (+__real__ (Val)) == sizeof (float))	      \
		     ? (__typeof__ ((__tgmath_real_type (Val)) 0	      \
				    + _Complex_I)) Cfct##f (Val)	      \
		     : __TGMATH_F128 (__real__ (Val),			      \
				      (__typeof__			      \
				       ((__tgmath_real_type (Val)) 0	      \
					+ _Complex_I)) Cfct, (Val))	      \
		     (__typeof__ ((__tgmath_real_type (Val)) 0		      \
				  + _Complex_I)) __tgml(Cfct) (Val)))

/* XXX This definition has to be changed as soon as the compiler understands
   the imaginary keyword.  */
#  define __TGMATH_UNARY_REAL_IMAG_RET_REAL(Val, Fct, Cfct) \
     (__extension__ ((sizeof (+__real__ (Val)) == sizeof (double)	      \
		      || __builtin_classify_type (__real__ (Val)) != 8)	      \
		     ? (__expr_is_real (Val)				      \
			? (__typeof__ (__real__ (__tgmath_real_type (Val)) 0))\
			  Fct (Val)					      \
			: (__typeof__ (__real__ (__tgmath_real_type (Val)) 0))\
			  Cfct (Val))					      \
		     : (sizeof (+__real__ (Val)) == sizeof (float))	      \
		     ? (__expr_is_real (Val)				      \
			? (__typeof__ (__real__ (__tgmath_real_type (Val)) 0))\
			  Fct##f (Val)					      \
			: (__typeof__ (__real__ (__tgmath_real_type (Val)) 0))\
			  Cfct##f (Val))				      \
		     : __TGMATH_CF128 ((Val), \
				       (__typeof__			      \
					(__real__			      \
					 (__tgmath_real_type (Val)) 0)) Fct,  \
				       (__typeof__			      \
					(__real__			      \
					 (__tgmath_real_type (Val)) 0)) Cfct, \
				       (Val))				      \
		     (__expr_is_real (Val)				      \
		      ? (__typeof__ (__real__ (__tgmath_real_type (Val)) 0))  \
		      __tgml(Fct) (Val)					      \
		      : (__typeof__ (__real__ (__tgmath_real_type (Val)) 0))  \
		      __tgml(Cfct) (Val))))
#  define __TGMATH_UNARY_REAL_IMAG_RET_REAL_SAME(Val, Cfct)	\
  __TGMATH_UNARY_REAL_IMAG_RET_REAL ((Val), Cfct, Cfct)

/* XXX This definition has to be changed as soon as the compiler understands
   the imaginary keyword.  */
#  define __TGMATH_BINARY_REAL_IMAG(Val1, Val2, Fct, Cfct) \
     (__extension__ ((sizeof (__real__ (Val1)				      \
			      + __real__ (Val2)) > sizeof (double)	      \
		      && __builtin_classify_type (__real__ (Val1)	      \
						  + __real__ (Val2)) == 8)    \
		     ? __TGMATH_CF128 ((Val1) + (Val2),			      \
				       (__typeof			      \
					((__tgmath_complex_type (Val1)) 0     \
					 + (__tgmath_complex_type (Val2)) 0)) \
				       Fct,				      \
				       (__typeof			      \
					((__tgmath_complex_type (Val1)) 0     \
					 + (__tgmath_complex_type (Val2)) 0)) \
				       Cfct,				      \
				       (Val1, Val2))			      \
		     (__expr_is_real ((Val1) + (Val2))			      \
		      ? (__typeof ((__tgmath_complex_type (Val1)) 0	      \
				   + (__tgmath_complex_type (Val2)) 0))	      \
		      __tgml(Fct) (Val1, Val2)				      \
		      : (__typeof ((__tgmath_complex_type (Val1)) 0	      \
				   + (__tgmath_complex_type (Val2)) 0))	      \
		      __tgml(Cfct) (Val1, Val2))			      \
		     : (sizeof (+__real__ (Val1)) == sizeof (double)	      \
			|| sizeof (+__real__ (Val2)) == sizeof (double)	      \
			|| __builtin_classify_type (__real__ (Val1)) != 8     \
			|| __builtin_classify_type (__real__ (Val2)) != 8)    \
		     ? (__expr_is_real ((Val1) + (Val2))		      \
			? (__typeof ((__tgmath_complex_type (Val1)) 0	      \
				   + (__tgmath_complex_type (Val2)) 0))	      \
			  Fct (Val1, Val2)				      \
			: (__typeof ((__tgmath_complex_type (Val1)) 0	      \
				   + (__tgmath_complex_type (Val2)) 0))	      \
			  Cfct (Val1, Val2))				      \
		     : (__expr_is_real ((Val1) + (Val2))		      \
			? (__typeof ((__tgmath_complex_type (Val1)) 0	      \
				   + (__tgmath_complex_type (Val2)) 0))	      \
			  Fct##f (Val1, Val2)				      \
			: (__typeof ((__tgmath_complex_type (Val1)) 0	      \
				   + (__tgmath_complex_type (Val2)) 0))	      \
			  Cfct##f (Val1, Val2))))

#  define __TGMATH_2_NARROW_F(F, X, Y)					\
  (__extension__ (sizeof ((__tgmath_real_type (X)) 0			\
			  + (__tgmath_real_type (Y)) 0) > sizeof (double) \
		  ? F ## l (X, Y)					\
		  : F (X, Y)))
/* In most cases, these narrowing macro definitions based on sizeof
   ensure that the function called has the right argument format, as
   for other <tgmath.h> macros for compilers before GCC 8, but may not
   have exactly the argument type (among the types with that format)
   specified in the standard logic.

   In the case of macros for _Float32x return type, when _Float64x
   exists, _Float64 arguments should result in the *f64 function being
   called while _Float32x arguments should result in the *f64x
   function being called.  These cases cannot be distinguished using
   sizeof (or at all if the types are typedefs rather than different
   types).  However, for these functions it is OK (does not affect the
   final result) to call a function with any argument format at least
   as wide as all the floating-point arguments, unless that affects
   rounding of integer arguments.  Integer arguments are considered to
   have type _Float64, so the *f64 functions are preferred for f32x*
   macros when no argument has a wider floating-point type.  */
#  if __HAVE_FLOAT64X_LONG_DOUBLE && __HAVE_DISTINCT_FLOAT128
#   define __TGMATH_2_NARROW_F32(F, X, Y)				\
  (__extension__ (sizeof ((__tgmath_real_type (X)) 0			\
			  + (__tgmath_real_type (Y)) 0) > sizeof (_Float64) \
		  ? __TGMATH_F128 ((X) + (Y), F, (X, Y))		\
		  F ## f64x (X, Y)					\
		  : F ## f64 (X, Y)))
#   define __TGMATH_2_NARROW_F64(F, X, Y)				\
  (__extension__ (sizeof ((__tgmath_real_type (X)) 0			\
			  + (__tgmath_real_type (Y)) 0) > sizeof (_Float64) \
		  ? __TGMATH_F128 ((X) + (Y), F, (X, Y))		\
		  F ## f64x (X, Y)					\
		  : F ## f128 (X, Y)))
#   define __TGMATH_2_NARROW_F32X(F, X, Y)				\
  (__extension__ (sizeof ((__tgmath_real_type (X)) 0			\
			  + (__tgmath_real_type (Y)) 0) > sizeof (_Float64) \
		  ? __TGMATH_F128 ((X) + (Y), F, (X, Y))		\
		  F ## f64x (X, Y)					\
		  : F ## f64 (X, Y)))
#  elif __HAVE_FLOAT128
#   define __TGMATH_2_NARROW_F32(F, X, Y)				\
  (__extension__ (sizeof ((__tgmath_real_type (X)) 0			\
			  + (__tgmath_real_type (Y)) 0) > sizeof (_Float64) \
		  ? F ## f128 (X, Y)					\
		  : F ## f64 (X, Y)))
#   define __TGMATH_2_NARROW_F64(F, X, Y)	\
  (F ## f128 (X, Y))
#   define __TGMATH_2_NARROW_F32X(F, X, Y)				\
  (__extension__ (sizeof ((__tgmath_real_type (X)) 0			\
			  + (__tgmath_real_type (Y)) 0) > sizeof (_Float32x) \
		  ? F ## f64x (X, Y)					\
		  : F ## f64 (X, Y)))
#  else
#   define __TGMATH_2_NARROW_F32(F, X, Y)	\
  (F ## f64 (X, Y))
#  endif
# endif /* !__HAVE_BUILTIN_TGMATH.  */
#else
# error "Unsupported compiler; you cannot use <tgmath.h>"
#endif


/* Unary functions defined for real and complex values.  */


/* Trigonometric functions.  */

/* Arc cosine of X.  */
#define acos(Val) __TGMATH_UNARY_REAL_IMAG (Val, acos, cacos)
/* Arc sine of X.  */
#define asin(Val) __TGMATH_UNARY_REAL_IMAG (Val, asin, casin)
/* Arc tangent of X.  */
#define atan(Val) __TGMATH_UNARY_REAL_IMAG (Val, atan, catan)
/* Arc tangent of Y/X.  */
#define atan2(Val1, Val2) __TGMATH_BINARY_REAL_ONLY (Val1, Val2, atan2)

/* Cosine of X.  */
#define cos(Val) __TGMATH_UNARY_REAL_IMAG (Val, cos, ccos)
/* Sine of X.  */
#define sin(Val) __TGMATH_UNARY_REAL_IMAG (Val, sin, csin)
/* Tangent of X.  */
#define tan(Val) __TGMATH_UNARY_REAL_IMAG (Val, tan, ctan)


/* Hyperbolic functions.  */

/* Hyperbolic arc cosine of X.  */
#define acosh(Val) __TGMATH_UNARY_REAL_IMAG (Val, acosh, cacosh)
/* Hyperbolic arc sine of X.  */
#define asinh(Val) __TGMATH_UNARY_REAL_IMAG (Val, asinh, casinh)
/* Hyperbolic arc tangent of X.  */
#define atanh(Val) __TGMATH_UNARY_REAL_IMAG (Val, atanh, catanh)

/* Hyperbolic cosine of X.  */
#define cosh(Val) __TGMATH_UNARY_REAL_IMAG (Val, cosh, ccosh)
/* Hyperbolic sine of X.  */
#define sinh(Val) __TGMATH_UNARY_REAL_IMAG (Val, sinh, csinh)
/* Hyperbolic tangent of X.  */
#define tanh(Val) __TGMATH_UNARY_REAL_IMAG (Val, tanh, ctanh)


/* Exponential and logarithmic functions.  */

/* Exponential function of X.  */
#define exp(Val) __TGMATH_UNARY_REAL_IMAG (Val, exp, cexp)

/* Break VALUE into a normalized fraction and an integral power of 2.  */
#define frexp(Val1, Val2) __TGMATH_BINARY_FIRST_REAL_ONLY (Val1, Val2, frexp)

/* X times (two to the EXP power).  */
#define ldexp(Val1, Val2) __TGMATH_BINARY_FIRST_REAL_ONLY (Val1, Val2, ldexp)

/* Natural logarithm of X.  */
#define log(Val) __TGMATH_UNARY_REAL_IMAG (Val, log, clog)

/* Base-ten logarithm of X.  */
#ifdef __USE_GNU
# define log10(Val) __TGMATH_UNARY_REAL_IMAG (Val, log10, clog10)
#else
# define log10(Val) __TGMATH_UNARY_REAL_ONLY (Val, log10)
#endif

/* Return exp(X) - 1.  */
#define expm1(Val) __TGMATH_UNARY_REAL_ONLY (Val, expm1)

/* Return log(1 + X).  */
#define log1p(Val) __TGMATH_UNARY_REAL_ONLY (Val, log1p)

/* Return the base 2 signed integral exponent of X.  */
#define logb(Val) __TGMATH_UNARY_REAL_ONLY (Val, logb)

/* Compute base-2 exponential of X.  */
#define exp2(Val) __TGMATH_UNARY_REAL_ONLY (Val, exp2)

/* Compute base-2 logarithm of X.  */
#define log2(Val) __TGMATH_UNARY_REAL_ONLY (Val, log2)


/* Power functions.  */

/* Return X to the Y power.  */
#define pow(Val1, Val2) __TGMATH_BINARY_REAL_IMAG (Val1, Val2, pow, cpow)

/* Return the square root of X.  */
#define sqrt(Val) __TGMATH_UNARY_REAL_IMAG (Val, sqrt, csqrt)

/* Return `sqrt(X*X + Y*Y)'.  */
#define hypot(Val1, Val2) __TGMATH_BINARY_REAL_ONLY (Val1, Val2, hypot)

/* Return the cube root of X.  */
#define cbrt(Val) __TGMATH_UNARY_REAL_ONLY (Val, cbrt)


/* Nearest integer, absolute value, and remainder functions.  */

/* Smallest integral value not less than X.  */
#define ceil(Val) __TGMATH_UNARY_REAL_ONLY (Val, ceil)

/* Absolute value of X.  */
#define fabs(Val) __TGMATH_UNARY_REAL_IMAG_RET_REAL (Val, fabs, cabs)

/* Largest integer not greater than X.  */
#define floor(Val) __TGMATH_UNARY_REAL_ONLY (Val, floor)

/* Floating-point modulo remainder of X/Y.  */
#define fmod(Val1, Val2) __TGMATH_BINARY_REAL_ONLY (Val1, Val2, fmod)

/* Round X to integral valuein floating-point format using current
   rounding direction, but do not raise inexact exception.  */
#define nearbyint(Val) __TGMATH_UNARY_REAL_ONLY (Val, nearbyint)

/* Round X to nearest integral value, rounding halfway cases away from
   zero.  */
#define round(Val) __TGMATH_UNARY_REAL_ONLY (Val, round)

/* Round X to the integral value in floating-point format nearest but
   not larger in magnitude.  */
#define trunc(Val) __TGMATH_UNARY_REAL_ONLY (Val, trunc)

/* Compute remainder of X and Y and put in *QUO a value with sign of x/y
   and magnitude congruent `mod 2^n' to the magnitude of the integral
   quotient x/y, with n >= 3.  */
#define remquo(Val1, Val2, Val3) \
     __TGMATH_TERNARY_FIRST_SECOND_REAL_ONLY (Val1, Val2, Val3, remquo)

/* Round X to nearest integral value according to current rounding
   direction.  */
#define lrint(Val) __TGMATH_UNARY_REAL_RET_ONLY (Val, lrint)
#define llrint(Val) __TGMATH_UNARY_REAL_RET_ONLY (Val, llrint)

/* Round X to nearest integral value, rounding halfway cases away from
   zero.  */
#define lround(Val) __TGMATH_UNARY_REAL_RET_ONLY (Val, lround)
#define llround(Val) __TGMATH_UNARY_REAL_RET_ONLY (Val, llround)


/* Return X with its signed changed to Y's.  */
#define copysign(Val1, Val2) __TGMATH_BINARY_REAL_ONLY (Val1, Val2, copysign)

/* Error and gamma functions.  */
#define erf(Val) __TGMATH_UNARY_REAL_ONLY (Val, erf)
#define erfc(Val) __TGMATH_UNARY_REAL_ONLY (Val, erfc)
#define tgamma(Val) __TGMATH_UNARY_REAL_ONLY (Val, tgamma)
#define lgamma(Val) __TGMATH_UNARY_REAL_ONLY (Val, lgamma)


/* Return the integer nearest X in the direction of the
   prevailing rounding mode.  */
#define rint(Val) __TGMATH_UNARY_REAL_ONLY (Val, rint)

#if __GLIBC_USE (IEC_60559_BFP_EXT_C2X)
/* Return X - epsilon.  */
# define nextdown(Val) __TGMATH_UNARY_REAL_ONLY (Val, nextdown)
/* Return X + epsilon.  */
# define nextup(Val) __TGMATH_UNARY_REAL_ONLY (Val, nextup)
#endif

/* Return X + epsilon if X < Y, X - epsilon if X > Y.  */
#define nextafter(Val1, Val2) __TGMATH_BINARY_REAL_ONLY (Val1, Val2, nextafter)
#define nexttoward(Val1, Val2) \
     __TGMATH_BINARY_FIRST_REAL_STD_ONLY (Val1, Val2, nexttoward)

/* Return the remainder of integer divison X / Y with infinite precision.  */
#define remainder(Val1, Val2) __TGMATH_BINARY_REAL_ONLY (Val1, Val2, remainder)

/* Return X times (2 to the Nth power).  */
#ifdef __USE_MISC
# define scalb(Val1, Val2) __TGMATH_BINARY_REAL_STD_ONLY (Val1, Val2, scalb)
#endif

/* Return X times (2 to the Nth power).  */
#define scalbn(Val1, Val2) __TGMATH_BINARY_FIRST_REAL_ONLY (Val1, Val2, scalbn)

/* Return X times (2 to the Nth power).  */
#define scalbln(Val1, Val2) \
     __TGMATH_BINARY_FIRST_REAL_ONLY (Val1, Val2, scalbln)

/* Return the binary exponent of X, which must be nonzero.  */
#define ilogb(Val) __TGMATH_UNARY_REAL_RET_ONLY (Val, ilogb)


/* Return positive difference between X and Y.  */
#define fdim(Val1, Val2) __TGMATH_BINARY_REAL_ONLY (Val1, Val2, fdim)

/* Return maximum numeric value from X and Y.  */
#define fmax(Val1, Val2) __TGMATH_BINARY_REAL_ONLY (Val1, Val2, fmax)

/* Return minimum numeric value from X and Y.  */
#define fmin(Val1, Val2) __TGMATH_BINARY_REAL_ONLY (Val1, Val2, fmin)


/* Multiply-add function computed as a ternary operation.  */
#define fma(Val1, Val2, Val3) \
     __TGMATH_TERNARY_REAL_ONLY (Val1, Val2, Val3, fma)

#if __GLIBC_USE (IEC_60559_BFP_EXT_C2X)
/* Round X to nearest integer value, rounding halfway cases to even.  */
# define roundeven(Val) __TGMATH_UNARY_REAL_ONLY (Val, roundeven)

# define fromfp(Val1, Val2, Val3)					\
  __TGMATH_TERNARY_FIRST_REAL_RET_ONLY (Val1, Val2, Val3, fromfp)

# define ufromfp(Val1, Val2, Val3)					\
  __TGMATH_TERNARY_FIRST_REAL_RET_ONLY (Val1, Val2, Val3, ufromfp)

# define fromfpx(Val1, Val2, Val3)					\
  __TGMATH_TERNARY_FIRST_REAL_RET_ONLY (Val1, Val2, Val3, fromfpx)

# define ufromfpx(Val1, Val2, Val3)					\
  __TGMATH_TERNARY_FIRST_REAL_RET_ONLY (Val1, Val2, Val3, ufromfpx)

/* Like ilogb, but returning long int.  */
# define llogb(Val) __TGMATH_UNARY_REAL_RET_ONLY (Val, llogb)

/* Return value with maximum magnitude.  */
# define fmaxmag(Val1, Val2) __TGMATH_BINARY_REAL_ONLY (Val1, Val2, fmaxmag)

/* Return value with minimum magnitude.  */
# define fminmag(Val1, Val2) __TGMATH_BINARY_REAL_ONLY (Val1, Val2, fminmag)
#endif


/* Absolute value, conjugates, and projection.  */

/* Argument value of Z.  */
#define carg(Val) __TGMATH_UNARY_REAL_IMAG_RET_REAL_SAME (Val, carg)

/* Complex conjugate of Z.  */
#define conj(Val) __TGMATH_UNARY_IMAG (Val, conj)

/* Projection of Z onto the Riemann sphere.  */
#define cproj(Val) __TGMATH_UNARY_IMAG (Val, cproj)


/* Decomposing complex values.  */

/* Imaginary part of Z.  */
#define cimag(Val) __TGMATH_UNARY_REAL_IMAG_RET_REAL_SAME (Val, cimag)

/* Real part of Z.  */
#define creal(Val) __TGMATH_UNARY_REAL_IMAG_RET_REAL_SAME (Val, creal)


/* Narrowing functions.  */

#if __GLIBC_USE (IEC_60559_BFP_EXT_C2X)

/* Add.  */
# define fadd(Val1, Val2) __TGMATH_2_NARROW_F (fadd, Val1, Val2)
# define dadd(Val1, Val2) __TGMATH_2_NARROW_D (dadd, Val1, Val2)

/* Divide.  */
# define fdiv(Val1, Val2) __TGMATH_2_NARROW_F (fdiv, Val1, Val2)
# define ddiv(Val1, Val2) __TGMATH_2_NARROW_D (ddiv, Val1, Val2)

/* Multiply.  */
# define fmul(Val1, Val2) __TGMATH_2_NARROW_F (fmul, Val1, Val2)
# define dmul(Val1, Val2) __TGMATH_2_NARROW_D (dmul, Val1, Val2)

/* Subtract.  */
# define fsub(Val1, Val2) __TGMATH_2_NARROW_F (fsub, Val1, Val2)
# define dsub(Val1, Val2) __TGMATH_2_NARROW_D (dsub, Val1, Val2)

#endif

#if __GLIBC_USE (IEC_60559_TYPES_EXT)

# if __HAVE_FLOAT16
#  define f16add(Val1, Val2) __TGMATH_2_NARROW_F16 (f16add, Val1, Val2)
#  define f16div(Val1, Val2) __TGMATH_2_NARROW_F16 (f16div, Val1, Val2)
#  define f16mul(Val1, Val2) __TGMATH_2_NARROW_F16 (f16mul, Val1, Val2)
#  define f16sub(Val1, Val2) __TGMATH_2_NARROW_F16 (f16sub, Val1, Val2)
# endif

# if __HAVE_FLOAT32
#  define f32add(Val1, Val2) __TGMATH_2_NARROW_F32 (f32add, Val1, Val2)
#  define f32div(Val1, Val2) __TGMATH_2_NARROW_F32 (f32div, Val1, Val2)
#  define f32mul(Val1, Val2) __TGMATH_2_NARROW_F32 (f32mul, Val1, Val2)
#  define f32sub(Val1, Val2) __TGMATH_2_NARROW_F32 (f32sub, Val1, Val2)
# endif

# if __HAVE_FLOAT64 && (__HAVE_FLOAT64X || __HAVE_FLOAT128)
#  define f64add(Val1, Val2) __TGMATH_2_NARROW_F64 (f64add, Val1, Val2)
#  define f64div(Val1, Val2) __TGMATH_2_NARROW_F64 (f64div, Val1, Val2)
#  define f64mul(Val1, Val2) __TGMATH_2_NARROW_F64 (f64mul, Val1, Val2)
#  define f64sub(Val1, Val2) __TGMATH_2_NARROW_F64 (f64sub, Val1, Val2)
# endif

# if __HAVE_FLOAT32X
#  define f32xadd(Val1, Val2) __TGMATH_2_NARROW_F32X (f32xadd, Val1, Val2)
#  define f32xdiv(Val1, Val2) __TGMATH_2_NARROW_F32X (f32xdiv, Val1, Val2)
#  define f32xmul(Val1, Val2) __TGMATH_2_NARROW_F32X (f32xmul, Val1, Val2)
#  define f32xsub(Val1, Val2) __TGMATH_2_NARROW_F32X (f32xsub, Val1, Val2)
# endif

# if __HAVE_FLOAT64X && (__HAVE_FLOAT128X || __HAVE_FLOAT128)
#  define f64xadd(Val1, Val2) __TGMATH_2_NARROW_F64X (f64xadd, Val1, Val2)
#  define f64xdiv(Val1, Val2) __TGMATH_2_NARROW_F64X (f64xdiv, Val1, Val2)
#  define f64xmul(Val1, Val2) __TGMATH_2_NARROW_F64X (f64xmul, Val1, Val2)
#  define f64xsub(Val1, Val2) __TGMATH_2_NARROW_F64X (f64xsub, Val1, Val2)
# endif

#endif

#endif /* tgmath.h */
