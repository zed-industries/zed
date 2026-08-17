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
 *	ISO C99:  7.3 Complex arithmetic	<complex.h>
 */

#ifndef _COMPLEX_H
#define _COMPLEX_H	1

#define __GLIBC_INTERNAL_STARTING_HEADER_IMPLEMENTATION
#include <bits/libc-header-start.h>

/* Get general and ISO C99 specific information.  */
#include <bits/mathdef.h>

/* Gather machine-dependent _FloatN type support.  */
#include <bits/floatn.h>

__BEGIN_DECLS

/* We might need to add support for more compilers here.  But since ISO
   C99 is out hopefully all maintained compilers will soon provide the data
   types `float complex' and `double complex'.  */
#if __GNUC_PREREQ (2, 7) && !__GNUC_PREREQ (2, 97)
# define _Complex __complex__
#endif

#define complex		_Complex

/* Narrowest imaginary unit.  This depends on the floating-point
   evaluation method.
   XXX This probably has to go into a gcc related file.  */
#define _Complex_I	(__extension__ 1.0iF)

/* Another more descriptive name is `I'.
   XXX Once we have the imaginary support switch this to _Imaginary_I.  */
#undef I
#define I _Complex_I

#if defined __USE_ISOC11 && __GNUC_PREREQ (4, 7)
/* Macros to expand into expression of specified complex type.  */
# define CMPLX(x, y) __builtin_complex ((double) (x), (double) (y))
# define CMPLXF(x, y) __builtin_complex ((float) (x), (float) (y))
# define CMPLXL(x, y) __builtin_complex ((long double) (x), (long double) (y))
#endif

#if __HAVE_FLOAT16 && __GLIBC_USE (IEC_60559_TYPES_EXT)
# define CMPLXF16(x, y) __builtin_complex ((_Float16) (x), (_Float16) (y))
#endif

#if __HAVE_FLOAT32 && __GLIBC_USE (IEC_60559_TYPES_EXT)
# define CMPLXF32(x, y) __builtin_complex ((_Float32) (x), (_Float32) (y))
#endif

#if __HAVE_FLOAT64 && __GLIBC_USE (IEC_60559_TYPES_EXT)
# define CMPLXF64(x, y) __builtin_complex ((_Float64) (x), (_Float64) (y))
#endif

#if __HAVE_FLOAT128 && __GLIBC_USE (IEC_60559_TYPES_EXT)
# define CMPLXF128(x, y) __builtin_complex ((_Float128) (x), (_Float128) (y))
#endif

#if __HAVE_FLOAT32X && __GLIBC_USE (IEC_60559_TYPES_EXT)
# define CMPLXF32X(x, y) __builtin_complex ((_Float32x) (x), (_Float32x) (y))
#endif

#if __HAVE_FLOAT64X && __GLIBC_USE (IEC_60559_TYPES_EXT)
# define CMPLXF64X(x, y) __builtin_complex ((_Float64x) (x), (_Float64x) (y))
#endif

#if __HAVE_FLOAT128X && __GLIBC_USE (IEC_60559_TYPES_EXT)
# define CMPLXF128X(x, y)					\
  __builtin_complex ((_Float128x) (x), (_Float128x) (y))
#endif

/* The file <bits/cmathcalls.h> contains the prototypes for all the
   actual math functions.  These macros are used for those prototypes,
   so we can easily declare each function as both `name' and `__name',
   and can declare the float versions `namef' and `__namef'.  */

#define __MATHCALL(function, args)	\
  __MATHDECL (_Mdouble_complex_,function, args)
#define __MATHDECL(type, function, args) \
  __MATHDECL_1(type, function, args); \
  __MATHDECL_1(type, __CONCAT(__,function), args)
#define __MATHDECL_1(type, function, args) \
  extern type __MATH_PRECNAME(function) args __THROW

#define _Mdouble_ 		double
#define __MATH_PRECNAME(name)	name
#include <bits/cmathcalls.h>
#undef	_Mdouble_
#undef	__MATH_PRECNAME

/* Now the float versions.  */
#define _Mdouble_ 		float
#define __MATH_PRECNAME(name)	name##f
#include <bits/cmathcalls.h>
#undef	_Mdouble_
#undef	__MATH_PRECNAME

/* And the long double versions.  It is non-critical to define them
   here unconditionally since `long double' is required in ISO C99.  */
#if !(defined __NO_LONG_DOUBLE_MATH && defined _LIBC)	\
    || defined __LDBL_COMPAT
# ifdef __LDBL_COMPAT
#  undef __MATHDECL_1
#  define __MATHDECL_1(type, function, args) \
  extern type __REDIRECT_NTH(__MATH_PRECNAME(function), args, function)
# endif

# define _Mdouble_ 		long double
# define __MATH_PRECNAME(name)	name##l
# include <bits/cmathcalls.h>
#endif
#undef	_Mdouble_
#undef	__MATH_PRECNAME

#if (__HAVE_DISTINCT_FLOAT16 || (__HAVE_FLOAT16 && !defined _LIBC)) \
     && __GLIBC_USE (IEC_60559_TYPES_EXT)
# undef _Mdouble_complex_
# define _Mdouble_complex_	__CFLOAT16
# define _Mdouble_		_Float16
# define __MATH_PRECNAME(name)	name##f16
# include <bits/cmathcalls.h>
# undef _Mdouble_
# undef __MATH_PRECNAME
# undef _Mdouble_complex_
#endif

#if (__HAVE_DISTINCT_FLOAT32 || (__HAVE_FLOAT32 && !defined _LIBC)) \
     && __GLIBC_USE (IEC_60559_TYPES_EXT)
# undef _Mdouble_complex_
# define _Mdouble_complex_	__CFLOAT32
# define _Mdouble_		_Float32
# define __MATH_PRECNAME(name)	name##f32
# include <bits/cmathcalls.h>
# undef _Mdouble_
# undef __MATH_PRECNAME
# undef _Mdouble_complex_
#endif

#if (__HAVE_DISTINCT_FLOAT64 || (__HAVE_FLOAT64 && !defined _LIBC)) \
     && __GLIBC_USE (IEC_60559_TYPES_EXT)
# undef _Mdouble_complex_
# define _Mdouble_complex_	__CFLOAT64
# define _Mdouble_		_Float64
# define __MATH_PRECNAME(name)	name##f64
# include <bits/cmathcalls.h>
# undef _Mdouble_
# undef __MATH_PRECNAME
# undef _Mdouble_complex_
#endif

#if (__HAVE_DISTINCT_FLOAT128 || (__HAVE_FLOAT128 && !defined _LIBC)) \
     && __GLIBC_USE (IEC_60559_TYPES_EXT)
# undef _Mdouble_complex_
# define _Mdouble_complex_	__CFLOAT128
# define _Mdouble_		_Float128
# define __MATH_PRECNAME(name)	name##f128
# include <bits/cmathcalls.h>
# undef _Mdouble_
# undef __MATH_PRECNAME
# undef _Mdouble_complex_
#endif

#if (__HAVE_DISTINCT_FLOAT32X || (__HAVE_FLOAT32X && !defined _LIBC)) \
     && __GLIBC_USE (IEC_60559_TYPES_EXT)
# undef _Mdouble_complex_
# define _Mdouble_complex_	__CFLOAT32X
# define _Mdouble_		_Float32x
# define __MATH_PRECNAME(name)	name##f32x
# include <bits/cmathcalls.h>
# undef _Mdouble_
# undef __MATH_PRECNAME
# undef _Mdouble_complex_
#endif

#if (__HAVE_DISTINCT_FLOAT64X || (__HAVE_FLOAT64X && !defined _LIBC)) \
     && __GLIBC_USE (IEC_60559_TYPES_EXT)
# undef _Mdouble_complex_
# define _Mdouble_complex_	__CFLOAT64X
# define _Mdouble_		_Float64x
# define __MATH_PRECNAME(name)	name##f64x
# include <bits/cmathcalls.h>
# undef _Mdouble_
# undef __MATH_PRECNAME
# undef _Mdouble_complex_
#endif

#if (__HAVE_DISTINCT_FLOAT128X || (__HAVE_FLOAT128X && !defined _LIBC)) \
     && __GLIBC_USE (IEC_60559_TYPES_EXT)
# undef _Mdouble_complex_
# define _Mdouble_complex_	__CFLOAT128X
# define _Mdouble_		_Float128x
# define __MATH_PRECNAME(name)	name##f128x
# include <bits/cmathcalls.h>
# undef _Mdouble_
# undef __MATH_PRECNAME
# undef _Mdouble_complex_
#endif

#undef	__MATHDECL_1
#undef	__MATHDECL
#undef	__MATHCALL

__END_DECLS

#endif /* complex.h */
