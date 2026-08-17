/* Copyright (C) 2002-2020 Free Software Foundation, Inc.

This file is part of GCC.

GCC is free software; you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation; either version 3, or (at your option)
any later version.

GCC is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

Under Section 7 of GPL version 3, you are granted additional
permissions described in the GCC Runtime Library Exception, version
3.1, as published by the Free Software Foundation.

You should have received a copy of the GNU General Public License and
a copy of the GCC Runtime Library Exception along with this program;
see the files COPYING3 and COPYING.RUNTIME respectively.  If not, see
<http://www.gnu.org/licenses/>.  */

/*
 * ISO C Standard:  5.2.4.2.2  Characteristics of floating types <float.h>
 */

#ifndef _FLOAT_H___
#define _FLOAT_H___

/* Radix of exponent representation, b. */
#undef FLT_RADIX
#define FLT_RADIX	__FLT_RADIX__

/* Number of base-FLT_RADIX digits in the significand, p.  */
#undef FLT_MANT_DIG
#undef DBL_MANT_DIG
#undef LDBL_MANT_DIG
#define FLT_MANT_DIG	__FLT_MANT_DIG__
#define DBL_MANT_DIG	__DBL_MANT_DIG__
#define LDBL_MANT_DIG	__LDBL_MANT_DIG__

/* Number of decimal digits, q, such that any floating-point number with q
   decimal digits can be rounded into a floating-point number with p radix b
   digits and back again without change to the q decimal digits,

	p * log10(b)			if b is a power of 10
	floor((p - 1) * log10(b))	otherwise
*/
#undef FLT_DIG
#undef DBL_DIG
#undef LDBL_DIG
#define FLT_DIG		__FLT_DIG__
#define DBL_DIG		__DBL_DIG__
#define LDBL_DIG	__LDBL_DIG__

/* Minimum int x such that FLT_RADIX**(x-1) is a normalized float, emin */
#undef FLT_MIN_EXP
#undef DBL_MIN_EXP
#undef LDBL_MIN_EXP
#define FLT_MIN_EXP	__FLT_MIN_EXP__
#define DBL_MIN_EXP	__DBL_MIN_EXP__
#define LDBL_MIN_EXP	__LDBL_MIN_EXP__

/* Minimum negative integer such that 10 raised to that power is in the
   range of normalized floating-point numbers,

	ceil(log10(b) * (emin - 1))
*/
#undef FLT_MIN_10_EXP
#undef DBL_MIN_10_EXP
#undef LDBL_MIN_10_EXP
#define FLT_MIN_10_EXP	__FLT_MIN_10_EXP__
#define DBL_MIN_10_EXP	__DBL_MIN_10_EXP__
#define LDBL_MIN_10_EXP	__LDBL_MIN_10_EXP__

/* Maximum int x such that FLT_RADIX**(x-1) is a representable float, emax.  */
#undef FLT_MAX_EXP
#undef DBL_MAX_EXP
#undef LDBL_MAX_EXP
#define FLT_MAX_EXP	__FLT_MAX_EXP__
#define DBL_MAX_EXP	__DBL_MAX_EXP__
#define LDBL_MAX_EXP	__LDBL_MAX_EXP__

/* Maximum integer such that 10 raised to that power is in the range of
   representable finite floating-point numbers,

	floor(log10((1 - b**-p) * b**emax))
*/
#undef FLT_MAX_10_EXP
#undef DBL_MAX_10_EXP
#undef LDBL_MAX_10_EXP
#define FLT_MAX_10_EXP	__FLT_MAX_10_EXP__
#define DBL_MAX_10_EXP	__DBL_MAX_10_EXP__
#define LDBL_MAX_10_EXP	__LDBL_MAX_10_EXP__

/* Maximum representable finite floating-point number,

	(1 - b**-p) * b**emax
*/
#undef FLT_MAX
#undef DBL_MAX
#undef LDBL_MAX
#define FLT_MAX		__FLT_MAX__
#define DBL_MAX		__DBL_MAX__
#define LDBL_MAX	__LDBL_MAX__

/* The difference between 1 and the least value greater than 1 that is
   representable in the given floating point type, b**1-p.  */
#undef FLT_EPSILON
#undef DBL_EPSILON
#undef LDBL_EPSILON
#define FLT_EPSILON	__FLT_EPSILON__
#define DBL_EPSILON	__DBL_EPSILON__
#define LDBL_EPSILON	__LDBL_EPSILON__

/* Minimum normalized positive floating-point number, b**(emin - 1).  */
#undef FLT_MIN
#undef DBL_MIN
#undef LDBL_MIN
#define FLT_MIN		__FLT_MIN__
#define DBL_MIN		__DBL_MIN__
#define LDBL_MIN	__LDBL_MIN__

/* Addition rounds to 0: zero, 1: nearest, 2: +inf, 3: -inf, -1: unknown.  */
/* ??? This is supposed to change with calls to fesetround in <fenv.h>.  */
#undef FLT_ROUNDS
#define FLT_ROUNDS 1

#if (defined (__STDC_VERSION__) && __STDC_VERSION__ >= 199901L) \
     || (defined (__cplusplus) && __cplusplus >= 201103L)
/* The floating-point expression evaluation method.  The precise
   definitions of these values are generalised to include support for
   the interchange and extended types defined in ISO/IEC TS 18661-3.
   Prior to this (for C99/C11) the definitions were:

	-1  indeterminate
	 0  evaluate all operations and constants just to the range and
	    precision of the type
	 1  evaluate operations and constants of type float and double
	    to the range and precision of the double type, evaluate
	    long double operations and constants to the range and
	    precision of the long double type
	 2  evaluate all operations and constants to the range and
	    precision of the long double type

   The TS 18661-3 definitions are:

	-1  indeterminate
	 0  evaluate all operations and constants, whose semantic type has
	    at most the range and precision of float, to the range and
	    precision of float; evaluate all other operations and constants
	    to the range and precision of the semantic type.
	 1  evaluate all operations and constants, whose semantic type has
	    at most the range and precision of double, to the range and
	    precision of double; evaluate all other operations and constants
	    to the range and precision of the semantic type.
	 2  evaluate all operations and constants, whose semantic type has
	    at most the range and precision of long double, to the range and
	    precision of long double; evaluate all other operations and
	    constants to the range and precision of the semantic type.
	 N  where _FloatN  is a supported interchange floating type
	    evaluate all operations and constants, whose semantic type has
	    at most the range and precision of the _FloatN type, to the
	    range and precision of the _FloatN type; evaluate all other
	    operations and constants to the range and precision of the
	    semantic type.
	 N + 1, where _FloatNx is a supported extended floating type
	    evaluate operations and constants, whose semantic type has at
	    most the range and precision of the _FloatNx type, to the range
	    and precision of the _FloatNx type; evaluate all other
	    operations and constants to the range and precision of the
	    semantic type.

   The compiler predefines two macros:

      __FLT_EVAL_METHOD__
      Which, depending on the value given for
      -fpermitted-flt-eval-methods, may be limited to only those values
      for FLT_EVAL_METHOD defined in C99/C11.

     __FLT_EVAL_METHOD_TS_18661_3__
      Which always permits the values for FLT_EVAL_METHOD defined in
      ISO/IEC TS 18661-3.

     Here we want to use __FLT_EVAL_METHOD__, unless
     __STDC_WANT_IEC_60559_TYPES_EXT__ is defined, in which case the user
     is specifically asking for the ISO/IEC TS 18661-3 types, so we use
     __FLT_EVAL_METHOD_TS_18661_3__.

   ??? This ought to change with the setting of the fp control word;
   the value provided by the compiler assumes the widest setting.  */
#undef FLT_EVAL_METHOD
#ifdef __STDC_WANT_IEC_60559_TYPES_EXT__
#define FLT_EVAL_METHOD __FLT_EVAL_METHOD_TS_18661_3__
#else
#define FLT_EVAL_METHOD	__FLT_EVAL_METHOD__
#endif

/* Number of decimal digits, n, such that any floating-point number in the
   widest supported floating type with pmax radix b digits can be rounded
   to a floating-point number with n decimal digits and back again without
   change to the value,

	pmax * log10(b)			if b is a power of 10
	ceil(1 + pmax * log10(b))	otherwise
*/
#undef DECIMAL_DIG
#define DECIMAL_DIG	__DECIMAL_DIG__

#endif /* C99 */

#if (defined (__STDC_VERSION__) && __STDC_VERSION__ >= 201112L) \
     || (defined (__cplusplus) && __cplusplus >= 201703L)
/* Versions of DECIMAL_DIG for each floating-point type.  */
#undef FLT_DECIMAL_DIG
#undef DBL_DECIMAL_DIG
#undef LDBL_DECIMAL_DIG
#define FLT_DECIMAL_DIG		__FLT_DECIMAL_DIG__
#define DBL_DECIMAL_DIG		__DBL_DECIMAL_DIG__
#define LDBL_DECIMAL_DIG	__LDBL_DECIMAL_DIG__

/* Whether types support subnormal numbers.  */
#undef FLT_HAS_SUBNORM
#undef DBL_HAS_SUBNORM
#undef LDBL_HAS_SUBNORM
#define FLT_HAS_SUBNORM		__FLT_HAS_DENORM__
#define DBL_HAS_SUBNORM		__DBL_HAS_DENORM__
#define LDBL_HAS_SUBNORM	__LDBL_HAS_DENORM__

/* Minimum positive values, including subnormals.  */
#undef FLT_TRUE_MIN
#undef DBL_TRUE_MIN
#undef LDBL_TRUE_MIN
#define FLT_TRUE_MIN	__FLT_DENORM_MIN__
#define DBL_TRUE_MIN	__DBL_DENORM_MIN__
#define LDBL_TRUE_MIN	__LDBL_DENORM_MIN__

#endif /* C11 */

#if defined __STDC_VERSION__ && __STDC_VERSION__ > 201710L
/* Maximum finite positive value with MANT_DIG digits in the
   significand taking their maximum value.  */
#undef FLT_NORM_MAX
#undef DBL_NORM_MAX
#undef LDBL_NORM_MAX
#define FLT_NORM_MAX	__FLT_NORM_MAX__
#define DBL_NORM_MAX	__DBL_NORM_MAX__
#define LDBL_NORM_MAX	__LDBL_NORM_MAX__

#endif /* C2X */

#ifdef __STDC_WANT_IEC_60559_BFP_EXT__
/* Number of decimal digits for which conversions between decimal
   character strings and binary formats, in both directions, are
   correctly rounded.  */
#define CR_DECIMAL_DIG	__UINTMAX_MAX__
#endif

#ifdef __STDC_WANT_IEC_60559_TYPES_EXT__
/* Constants for _FloatN and _FloatNx types from TS 18661-3.  See
   comments above for their semantics.  */

#ifdef __FLT16_MANT_DIG__
#undef FLT16_MANT_DIG
#define FLT16_MANT_DIG		__FLT16_MANT_DIG__
#undef FLT16_DIG
#define FLT16_DIG		__FLT16_DIG__
#undef FLT16_MIN_EXP
#define FLT16_MIN_EXP		__FLT16_MIN_EXP__
#undef FLT16_MIN_10_EXP
#define FLT16_MIN_10_EXP	__FLT16_MIN_10_EXP__
#undef FLT16_MAX_EXP
#define FLT16_MAX_EXP		__FLT16_MAX_EXP__
#undef FLT16_MAX_10_EXP
#define FLT16_MAX_10_EXP	__FLT16_MAX_10_EXP__
#undef FLT16_MAX
#define FLT16_MAX		__FLT16_MAX__
#undef FLT16_EPSILON
#define FLT16_EPSILON		__FLT16_EPSILON__
#undef FLT16_MIN
#define FLT16_MIN		__FLT16_MIN__
#undef FLT16_DECIMAL_DIG
#define FLT16_DECIMAL_DIG	__FLT16_DECIMAL_DIG__
#undef FLT16_TRUE_MIN
#define FLT16_TRUE_MIN		__FLT16_DENORM_MIN__
#endif /* __FLT16_MANT_DIG__.  */

#ifdef __FLT32_MANT_DIG__
#undef FLT32_MANT_DIG
#define FLT32_MANT_DIG		__FLT32_MANT_DIG__
#undef FLT32_DIG
#define FLT32_DIG		__FLT32_DIG__
#undef FLT32_MIN_EXP
#define FLT32_MIN_EXP		__FLT32_MIN_EXP__
#undef FLT32_MIN_10_EXP
#define FLT32_MIN_10_EXP	__FLT32_MIN_10_EXP__
#undef FLT32_MAX_EXP
#define FLT32_MAX_EXP		__FLT32_MAX_EXP__
#undef FLT32_MAX_10_EXP
#define FLT32_MAX_10_EXP	__FLT32_MAX_10_EXP__
#undef FLT32_MAX
#define FLT32_MAX		__FLT32_MAX__
#undef FLT32_EPSILON
#define FLT32_EPSILON		__FLT32_EPSILON__
#undef FLT32_MIN
#define FLT32_MIN		__FLT32_MIN__
#undef FLT32_DECIMAL_DIG
#define FLT32_DECIMAL_DIG	__FLT32_DECIMAL_DIG__
#undef FLT32_TRUE_MIN
#define FLT32_TRUE_MIN		__FLT32_DENORM_MIN__
#endif /* __FLT32_MANT_DIG__.  */

#ifdef __FLT64_MANT_DIG__
#undef FLT64_MANT_DIG
#define FLT64_MANT_DIG		__FLT64_MANT_DIG__
#undef FLT64_DIG
#define FLT64_DIG		__FLT64_DIG__
#undef FLT64_MIN_EXP
#define FLT64_MIN_EXP		__FLT64_MIN_EXP__
#undef FLT64_MIN_10_EXP
#define FLT64_MIN_10_EXP	__FLT64_MIN_10_EXP__
#undef FLT64_MAX_EXP
#define FLT64_MAX_EXP		__FLT64_MAX_EXP__
#undef FLT64_MAX_10_EXP
#define FLT64_MAX_10_EXP	__FLT64_MAX_10_EXP__
#undef FLT64_MAX
#define FLT64_MAX		__FLT64_MAX__
#undef FLT64_EPSILON
#define FLT64_EPSILON		__FLT64_EPSILON__
#undef FLT64_MIN
#define FLT64_MIN		__FLT64_MIN__
#undef FLT64_DECIMAL_DIG
#define FLT64_DECIMAL_DIG	__FLT64_DECIMAL_DIG__
#undef FLT64_TRUE_MIN
#define FLT64_TRUE_MIN		__FLT64_DENORM_MIN__
#endif /* __FLT64_MANT_DIG__.  */

#ifdef __FLT128_MANT_DIG__
#undef FLT128_MANT_DIG
#define FLT128_MANT_DIG		__FLT128_MANT_DIG__
#undef FLT128_DIG
#define FLT128_DIG		__FLT128_DIG__
#undef FLT128_MIN_EXP
#define FLT128_MIN_EXP		__FLT128_MIN_EXP__
#undef FLT128_MIN_10_EXP
#define FLT128_MIN_10_EXP	__FLT128_MIN_10_EXP__
#undef FLT128_MAX_EXP
#define FLT128_MAX_EXP		__FLT128_MAX_EXP__
#undef FLT128_MAX_10_EXP
#define FLT128_MAX_10_EXP	__FLT128_MAX_10_EXP__
#undef FLT128_MAX
#define FLT128_MAX		__FLT128_MAX__
#undef FLT128_EPSILON
#define FLT128_EPSILON		__FLT128_EPSILON__
#undef FLT128_MIN
#define FLT128_MIN		__FLT128_MIN__
#undef FLT128_DECIMAL_DIG
#define FLT128_DECIMAL_DIG	__FLT128_DECIMAL_DIG__
#undef FLT128_TRUE_MIN
#define FLT128_TRUE_MIN		__FLT128_DENORM_MIN__
#endif /* __FLT128_MANT_DIG__.  */

#ifdef __FLT32X_MANT_DIG__
#undef FLT32X_MANT_DIG
#define FLT32X_MANT_DIG		__FLT32X_MANT_DIG__
#undef FLT32X_DIG
#define FLT32X_DIG		__FLT32X_DIG__
#undef FLT32X_MIN_EXP
#define FLT32X_MIN_EXP		__FLT32X_MIN_EXP__
#undef FLT32X_MIN_10_EXP
#define FLT32X_MIN_10_EXP	__FLT32X_MIN_10_EXP__
#undef FLT32X_MAX_EXP
#define FLT32X_MAX_EXP		__FLT32X_MAX_EXP__
#undef FLT32X_MAX_10_EXP
#define FLT32X_MAX_10_EXP	__FLT32X_MAX_10_EXP__
#undef FLT32X_MAX
#define FLT32X_MAX		__FLT32X_MAX__
#undef FLT32X_EPSILON
#define FLT32X_EPSILON		__FLT32X_EPSILON__
#undef FLT32X_MIN
#define FLT32X_MIN		__FLT32X_MIN__
#undef FLT32X_DECIMAL_DIG
#define FLT32X_DECIMAL_DIG	__FLT32X_DECIMAL_DIG__
#undef FLT32X_TRUE_MIN
#define FLT32X_TRUE_MIN		__FLT32X_DENORM_MIN__
#endif /* __FLT32X_MANT_DIG__.  */

#ifdef __FLT64X_MANT_DIG__
#undef FLT64X_MANT_DIG
#define FLT64X_MANT_DIG		__FLT64X_MANT_DIG__
#undef FLT64X_DIG
#define FLT64X_DIG		__FLT64X_DIG__
#undef FLT64X_MIN_EXP
#define FLT64X_MIN_EXP		__FLT64X_MIN_EXP__
#undef FLT64X_MIN_10_EXP
#define FLT64X_MIN_10_EXP	__FLT64X_MIN_10_EXP__
#undef FLT64X_MAX_EXP
#define FLT64X_MAX_EXP		__FLT64X_MAX_EXP__
#undef FLT64X_MAX_10_EXP
#define FLT64X_MAX_10_EXP	__FLT64X_MAX_10_EXP__
#undef FLT64X_MAX
#define FLT64X_MAX		__FLT64X_MAX__
#undef FLT64X_EPSILON
#define FLT64X_EPSILON		__FLT64X_EPSILON__
#undef FLT64X_MIN
#define FLT64X_MIN		__FLT64X_MIN__
#undef FLT64X_DECIMAL_DIG
#define FLT64X_DECIMAL_DIG	__FLT64X_DECIMAL_DIG__
#undef FLT64X_TRUE_MIN
#define FLT64X_TRUE_MIN		__FLT64X_DENORM_MIN__
#endif /* __FLT64X_MANT_DIG__.  */

#ifdef __FLT128X_MANT_DIG__
#undef FLT128X_MANT_DIG
#define FLT128X_MANT_DIG	__FLT128X_MANT_DIG__
#undef FLT128X_DIG
#define FLT128X_DIG		__FLT128X_DIG__
#undef FLT128X_MIN_EXP
#define FLT128X_MIN_EXP		__FLT128X_MIN_EXP__
#undef FLT128X_MIN_10_EXP
#define FLT128X_MIN_10_EXP	__FLT128X_MIN_10_EXP__
#undef FLT128X_MAX_EXP
#define FLT128X_MAX_EXP		__FLT128X_MAX_EXP__
#undef FLT128X_MAX_10_EXP
#define FLT128X_MAX_10_EXP	__FLT128X_MAX_10_EXP__
#undef FLT128X_MAX
#define FLT128X_MAX		__FLT128X_MAX__
#undef FLT128X_EPSILON
#define FLT128X_EPSILON		__FLT128X_EPSILON__
#undef FLT128X_MIN
#define FLT128X_MIN		__FLT128X_MIN__
#undef FLT128X_DECIMAL_DIG
#define FLT128X_DECIMAL_DIG	__FLT128X_DECIMAL_DIG__
#undef FLT128X_TRUE_MIN
#define FLT128X_TRUE_MIN	__FLT128X_DENORM_MIN__
#endif /* __FLT128X_MANT_DIG__.  */

#endif /* __STDC_WANT_IEC_60559_TYPES_EXT__.  */

#ifdef __DEC32_MANT_DIG__
#if (defined __STDC_WANT_DEC_FP__ \
     || defined __STDC_WANT_IEC_60559_DFP_EXT__ \
     || (defined __STDC_VERSION__ && __STDC_VERSION__ > 201710L))
/* C2X; formerly Technical Report 24732, extension for decimal
   floating-point arithmetic: Characteristic of decimal floating types
   <float.h>, and TS 18661-2.  */

/* Number of base-FLT_RADIX digits in the significand, p.  */
#undef DEC32_MANT_DIG
#undef DEC64_MANT_DIG
#undef DEC128_MANT_DIG
#define DEC32_MANT_DIG	__DEC32_MANT_DIG__
#define DEC64_MANT_DIG	__DEC64_MANT_DIG__
#define DEC128_MANT_DIG	__DEC128_MANT_DIG__

/* Minimum exponent. */
#undef DEC32_MIN_EXP
#undef DEC64_MIN_EXP
#undef DEC128_MIN_EXP
#define DEC32_MIN_EXP	__DEC32_MIN_EXP__
#define DEC64_MIN_EXP	__DEC64_MIN_EXP__
#define DEC128_MIN_EXP	__DEC128_MIN_EXP__

/* Maximum exponent. */
#undef DEC32_MAX_EXP
#undef DEC64_MAX_EXP
#undef DEC128_MAX_EXP
#define DEC32_MAX_EXP	__DEC32_MAX_EXP__
#define DEC64_MAX_EXP	__DEC64_MAX_EXP__
#define DEC128_MAX_EXP	__DEC128_MAX_EXP__

/* Maximum representable finite decimal floating-point number
   (there are 6, 15, and 33 9s after the decimal points respectively). */
#undef DEC32_MAX
#undef DEC64_MAX
#undef DEC128_MAX
#define DEC32_MAX   __DEC32_MAX__
#define DEC64_MAX   __DEC64_MAX__
#define DEC128_MAX  __DEC128_MAX__

/* The difference between 1 and the least value greater than 1 that is
   representable in the given floating point type. */
#undef DEC32_EPSILON
#undef DEC64_EPSILON
#undef DEC128_EPSILON
#define DEC32_EPSILON	__DEC32_EPSILON__
#define DEC64_EPSILON	__DEC64_EPSILON__
#define DEC128_EPSILON	__DEC128_EPSILON__

/* Minimum normalized positive floating-point number. */
#undef DEC32_MIN
#undef DEC64_MIN
#undef DEC128_MIN
#define DEC32_MIN	__DEC32_MIN__
#define DEC64_MIN	__DEC64_MIN__
#define DEC128_MIN	__DEC128_MIN__

/* The floating-point expression evaluation method.
         -1  indeterminate
         0  evaluate all operations and constants just to the range and
            precision of the type
         1  evaluate operations and constants of type _Decimal32 
	    and _Decimal64 to the range and precision of the _Decimal64 
            type, evaluate _Decimal128 operations and constants to the 
	    range and precision of the _Decimal128 type;
	 2  evaluate all operations and constants to the range and
	    precision of the _Decimal128 type.  */

#undef DEC_EVAL_METHOD
#define DEC_EVAL_METHOD	__DEC_EVAL_METHOD__

#endif /* __STDC_WANT_DEC_FP__ || __STDC_WANT_IEC_60559_DFP_EXT__ || C2X.  */

#ifdef __STDC_WANT_DEC_FP__

/* Minimum subnormal positive floating-point number. */
#undef DEC32_SUBNORMAL_MIN
#undef DEC64_SUBNORMAL_MIN
#undef DEC128_SUBNORMAL_MIN
#define DEC32_SUBNORMAL_MIN       __DEC32_SUBNORMAL_MIN__
#define DEC64_SUBNORMAL_MIN       __DEC64_SUBNORMAL_MIN__
#define DEC128_SUBNORMAL_MIN      __DEC128_SUBNORMAL_MIN__

#endif /* __STDC_WANT_DEC_FP__.  */

#if (defined __STDC_WANT_IEC_60559_DFP_EXT__ \
     || (defined __STDC_VERSION__ && __STDC_VERSION__ > 201710L))

/* Minimum subnormal positive floating-point number. */
#undef DEC32_TRUE_MIN
#undef DEC64_TRUE_MIN
#undef DEC128_TRUE_MIN
#define DEC32_TRUE_MIN       __DEC32_SUBNORMAL_MIN__
#define DEC64_TRUE_MIN       __DEC64_SUBNORMAL_MIN__
#define DEC128_TRUE_MIN      __DEC128_SUBNORMAL_MIN__

#endif /* __STDC_WANT_IEC_60559_DFP_EXT__ || C2X.  */

#endif /* __DEC32_MANT_DIG__ */

#endif /* _FLOAT_H___ */
