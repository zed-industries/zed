/* GCC Quad-Precision Math Library
   Copyright (C) 2010-2019 Free Software Foundation, Inc.
   Written by Francois-Xavier Coudert  <fxcoudert@gcc.gnu.org>

This file is part of the libquadmath library.
Libquadmath is free software; you can redistribute it and/or
modify it under the terms of the GNU Library General Public
License as published by the Free Software Foundation; either
version 2 of the License, or (at your option) any later version.

Libquadmath is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
Library General Public License for more details.

You should have received a copy of the GNU Library General Public
License along with libquadmath; see the file COPYING.LIB.  If
not, write to the Free Software Foundation, Inc., 51 Franklin Street - Fifth Floor,
Boston, MA 02110-1301, USA.  */

#ifndef QUADMATH_H
#define QUADMATH_H

#include <stdlib.h>

#ifdef __cplusplus
extern "C" {
#endif

/* Define the complex type corresponding to __float128
   ("_Complex __float128" is not allowed) */
#if (!defined(_ARCH_PPC)) || defined(__LONG_DOUBLE_IEEE128__)
typedef _Complex float __attribute__((mode(TC))) __complex128;
#else
typedef _Complex float __attribute__((mode(KC))) __complex128;
#endif

#ifdef __cplusplus
# define __quadmath_throw throw ()
# define __quadmath_nth(fct) fct throw ()
#else
# define __quadmath_throw __attribute__((__nothrow__))
# define __quadmath_nth(fct) __attribute__((__nothrow__)) fct
#endif

/* Prototypes for real functions */
extern __float128 acosq (__float128) __quadmath_throw;
extern __float128 acoshq (__float128) __quadmath_throw;
extern __float128 asinq (__float128) __quadmath_throw;
extern __float128 asinhq (__float128) __quadmath_throw;
extern __float128 atanq (__float128) __quadmath_throw;
extern __float128 atanhq (__float128) __quadmath_throw;
extern __float128 atan2q (__float128, __float128) __quadmath_throw;
extern __float128 cbrtq (__float128) __quadmath_throw;
extern __float128 ceilq (__float128) __quadmath_throw;
extern __float128 copysignq (__float128, __float128) __quadmath_throw;
extern __float128 coshq (__float128) __quadmath_throw;
extern __float128 cosq (__float128) __quadmath_throw;
extern __float128 erfq (__float128) __quadmath_throw;
extern __float128 erfcq (__float128) __quadmath_throw;
extern __float128 exp2q (__float128) __quadmath_throw;
extern __float128 expq (__float128) __quadmath_throw;
extern __float128 expm1q (__float128) __quadmath_throw;
extern __float128 fabsq (__float128) __quadmath_throw;
extern __float128 fdimq (__float128, __float128) __quadmath_throw;
extern int finiteq (__float128) __quadmath_throw;
extern __float128 floorq (__float128) __quadmath_throw;
extern __float128 fmaq (__float128, __float128, __float128) __quadmath_throw;
extern __float128 fmaxq (__float128, __float128) __quadmath_throw;
extern __float128 fminq (__float128, __float128) __quadmath_throw;
extern __float128 fmodq (__float128, __float128) __quadmath_throw;
extern __float128 frexpq (__float128, int *) __quadmath_throw;
extern __float128 hypotq (__float128, __float128) __quadmath_throw;
extern int isinfq (__float128) __quadmath_throw;
extern int ilogbq (__float128) __quadmath_throw;
extern int isnanq (__float128) __quadmath_throw;
extern int issignalingq (__float128) __quadmath_throw;
extern __float128 j0q (__float128) __quadmath_throw;
extern __float128 j1q (__float128) __quadmath_throw;
extern __float128 jnq (int, __float128) __quadmath_throw;
extern __float128 ldexpq (__float128, int) __quadmath_throw;
extern __float128 lgammaq (__float128) __quadmath_throw;
extern long long int llrintq (__float128) __quadmath_throw;
extern long long int llroundq (__float128) __quadmath_throw;
extern __float128 logbq (__float128) __quadmath_throw;
extern __float128 logq (__float128) __quadmath_throw;
extern __float128 log10q (__float128) __quadmath_throw;
extern __float128 log2q (__float128) __quadmath_throw;
extern __float128 log1pq (__float128) __quadmath_throw;
extern long int lrintq (__float128) __quadmath_throw;
extern long int lroundq (__float128) __quadmath_throw;
extern __float128 modfq (__float128, __float128 *) __quadmath_throw;
extern __float128 nanq (const char *) __quadmath_throw;
extern __float128 nearbyintq (__float128) __quadmath_throw;
extern __float128 nextafterq (__float128, __float128) __quadmath_throw;
extern __float128 powq (__float128, __float128) __quadmath_throw;
extern __float128 remainderq (__float128, __float128) __quadmath_throw;
extern __float128 remquoq (__float128, __float128, int *) __quadmath_throw;
extern __float128 rintq (__float128) __quadmath_throw;
extern __float128 roundq (__float128) __quadmath_throw;
extern __float128 scalblnq (__float128, long int) __quadmath_throw;
extern __float128 scalbnq (__float128, int) __quadmath_throw;
extern int signbitq (__float128) __quadmath_throw;
extern void sincosq (__float128, __float128 *, __float128 *) __quadmath_throw;
extern __float128 sinhq (__float128) __quadmath_throw;
extern __float128 sinq (__float128) __quadmath_throw;
extern __float128 sqrtq (__float128) __quadmath_throw;
extern __float128 tanq (__float128) __quadmath_throw;
extern __float128 tanhq (__float128) __quadmath_throw;
extern __float128 tgammaq (__float128) __quadmath_throw;
extern __float128 truncq (__float128) __quadmath_throw;
extern __float128 y0q (__float128) __quadmath_throw;
extern __float128 y1q (__float128) __quadmath_throw;
extern __float128 ynq (int, __float128) __quadmath_throw;


/* Prototypes for complex functions */
extern __float128 cabsq (__complex128) __quadmath_throw;
extern __float128 cargq (__complex128) __quadmath_throw;
extern __float128 cimagq (__complex128) __quadmath_throw;
extern __float128 crealq (__complex128) __quadmath_throw;
extern __complex128 cacosq (__complex128) __quadmath_throw;
extern __complex128 cacoshq (__complex128) __quadmath_throw;
extern __complex128 casinq (__complex128) __quadmath_throw;
extern __complex128 casinhq (__complex128) __quadmath_throw;
extern __complex128 catanq (__complex128) __quadmath_throw;
extern __complex128 catanhq (__complex128) __quadmath_throw;
extern __complex128 ccosq (__complex128) __quadmath_throw;
extern __complex128 ccoshq (__complex128) __quadmath_throw;
extern __complex128 cexpq (__complex128) __quadmath_throw;
extern __complex128 cexpiq (__float128) __quadmath_throw;
extern __complex128 clogq (__complex128) __quadmath_throw;
extern __complex128 clog10q (__complex128) __quadmath_throw;
extern __complex128 conjq (__complex128) __quadmath_throw;
extern __complex128 cpowq (__complex128, __complex128) __quadmath_throw;
extern __complex128 cprojq (__complex128) __quadmath_throw;
extern __complex128 csinq (__complex128) __quadmath_throw;
extern __complex128 csinhq (__complex128) __quadmath_throw;
extern __complex128 csqrtq (__complex128) __quadmath_throw;
extern __complex128 ctanq (__complex128) __quadmath_throw;
extern __complex128 ctanhq (__complex128) __quadmath_throw;


/* Prototypes for string <-> __float128 conversion functions */
extern __float128 strtoflt128 (const char *, char **) __quadmath_throw;
extern int quadmath_snprintf (char *str, size_t size,
			      const char *format, ...) __quadmath_throw;


/* Macros */
#define FLT128_MAX 1.18973149535723176508575932662800702e4932Q
#define FLT128_MIN 3.36210314311209350626267781732175260e-4932Q
#define FLT128_EPSILON 1.92592994438723585305597794258492732e-34Q
#define FLT128_DENORM_MIN 6.475175119438025110924438958227646552e-4966Q
#define FLT128_MANT_DIG 113
#define FLT128_MIN_EXP (-16381)
#define FLT128_MAX_EXP 16384
#define FLT128_DIG 33
#define FLT128_MIN_10_EXP (-4931)
#define FLT128_MAX_10_EXP 4932


#define HUGE_VALQ __builtin_huge_valq()
/* The following alternative is valid, but brings the warning:
   (floating constant exceeds range of ‘__float128’)  */
/* #define HUGE_VALQ (__extension__ 0x1.0p32767Q) */

#define M_Eq		2.718281828459045235360287471352662498Q  /* e */
#define M_LOG2Eq	1.442695040888963407359924681001892137Q  /* log_2 e */
#define M_LOG10Eq	0.434294481903251827651128918916605082Q  /* log_10 e */
#define M_LN2q		0.693147180559945309417232121458176568Q  /* log_e 2 */
#define M_LN10q		2.302585092994045684017991454684364208Q  /* log_e 10 */
#define M_PIq		3.141592653589793238462643383279502884Q  /* pi */
#define M_PI_2q		1.570796326794896619231321691639751442Q  /* pi/2 */
#define M_PI_4q		0.785398163397448309615660845819875721Q  /* pi/4 */
#define M_1_PIq		0.318309886183790671537767526745028724Q  /* 1/pi */
#define M_2_PIq		0.636619772367581343075535053490057448Q  /* 2/pi */
#define M_2_SQRTPIq	1.128379167095512573896158903121545172Q  /* 2/sqrt(pi) */
#define M_SQRT2q	1.414213562373095048801688724209698079Q  /* sqrt(2) */
#define M_SQRT1_2q	0.707106781186547524400844362104849039Q  /* 1/sqrt(2) */

#define __quadmath_extern_inline \
  extern inline __attribute__ ((__gnu_inline__))

__quadmath_extern_inline __float128
__quadmath_nth (cimagq (__complex128 __z))
{
  return __imag__ __z;
}

__quadmath_extern_inline __float128
__quadmath_nth (crealq (__complex128 __z))
{
  return __real__ __z;
}

__quadmath_extern_inline __complex128
__quadmath_nth (conjq (__complex128 __z))
{
  return __extension__ ~__z;
}

#ifdef __cplusplus
}
#endif

#endif
