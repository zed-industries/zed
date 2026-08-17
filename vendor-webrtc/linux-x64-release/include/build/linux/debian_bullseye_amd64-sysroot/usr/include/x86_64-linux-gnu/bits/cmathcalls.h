/* Prototype declarations for complex math functions;
   helper file for <complex.h>.
   Copyright (C) 1997-2020 Free Software Foundation, Inc.
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

/* NOTE: Because of the special way this file is used by <complex.h>, this
   file must NOT be protected from multiple inclusion as header files
   usually are.

   This file provides prototype declarations for the math functions.
   Most functions are declared using the macro:

   __MATHCALL (NAME, (ARGS...));

   This means there is a function `NAME' returning `double' and a function
   `NAMEf' returning `float'.  Each place `_Mdouble_' appears in the
   prototype, that is actually `double' in the prototype for `NAME' and
   `float' in the prototype for `NAMEf'.  Reentrant variant functions are
   called `NAME_r' and `NAMEf_r'.

   Functions returning other types like `int' are declared using the macro:

   __MATHDECL (TYPE, NAME, (ARGS...));

   This is just like __MATHCALL but for a function returning `TYPE'
   instead of `_Mdouble_'.  In all of these cases, there is still
   both a `NAME' and a `NAMEf' that takes `float' arguments.  */

#ifndef _COMPLEX_H
#error "Never use <bits/cmathcalls.h> directly; include <complex.h> instead."
#endif

#ifndef _Mdouble_complex_
# define _Mdouble_complex_ _Mdouble_ _Complex
#endif


/* Trigonometric functions.  */

/* Arc cosine of Z.  */
__MATHCALL (cacos, (_Mdouble_complex_ __z));
/* Arc sine of Z.  */
__MATHCALL (casin, (_Mdouble_complex_ __z));
/* Arc tangent of Z.  */
__MATHCALL (catan, (_Mdouble_complex_ __z));

/* Cosine of Z.  */
__MATHCALL (ccos, (_Mdouble_complex_ __z));
/* Sine of Z.  */
__MATHCALL (csin, (_Mdouble_complex_ __z));
/* Tangent of Z.  */
__MATHCALL (ctan, (_Mdouble_complex_ __z));


/* Hyperbolic functions.  */

/* Hyperbolic arc cosine of Z.  */
__MATHCALL (cacosh, (_Mdouble_complex_ __z));
/* Hyperbolic arc sine of Z.  */
__MATHCALL (casinh, (_Mdouble_complex_ __z));
/* Hyperbolic arc tangent of Z.  */
__MATHCALL (catanh, (_Mdouble_complex_ __z));

/* Hyperbolic cosine of Z.  */
__MATHCALL (ccosh, (_Mdouble_complex_ __z));
/* Hyperbolic sine of Z.  */
__MATHCALL (csinh, (_Mdouble_complex_ __z));
/* Hyperbolic tangent of Z.  */
__MATHCALL (ctanh, (_Mdouble_complex_ __z));


/* Exponential and logarithmic functions.  */

/* Exponential function of Z.  */
__MATHCALL (cexp, (_Mdouble_complex_ __z));

/* Natural logarithm of Z.  */
__MATHCALL (clog, (_Mdouble_complex_ __z));

#ifdef __USE_GNU
/* The base 10 logarithm is not defined by the standard but to implement
   the standard C++ library it is handy.  */
__MATHCALL (clog10, (_Mdouble_complex_ __z));
#endif

/* Power functions.  */

/* Return X to the Y power.  */
__MATHCALL (cpow, (_Mdouble_complex_ __x, _Mdouble_complex_ __y));

/* Return the square root of Z.  */
__MATHCALL (csqrt, (_Mdouble_complex_ __z));


/* Absolute value, conjugates, and projection.  */

/* Absolute value of Z.  */
__MATHDECL (_Mdouble_,cabs, (_Mdouble_complex_ __z));

/* Argument value of Z.  */
__MATHDECL (_Mdouble_,carg, (_Mdouble_complex_ __z));

/* Complex conjugate of Z.  */
__MATHCALL (conj, (_Mdouble_complex_ __z));

/* Projection of Z onto the Riemann sphere.  */
__MATHCALL (cproj, (_Mdouble_complex_ __z));


/* Decomposing complex values.  */

/* Imaginary part of Z.  */
__MATHDECL (_Mdouble_,cimag, (_Mdouble_complex_ __z));

/* Real part of Z.  */
__MATHDECL (_Mdouble_,creal, (_Mdouble_complex_ __z));
