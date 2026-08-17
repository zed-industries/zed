/* Define iscanonical macro.  ldbl-96 version.
   Copyright (C) 2016-2020 Free Software Foundation, Inc.
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

#ifndef _MATH_H
# error "Never use <bits/iscanonical.h> directly; include <math.h> instead."
#endif

extern int __iscanonicall (long double __x)
     __THROW __attribute__ ((__const__));
#define __iscanonicalf(x) ((void) (__typeof (x)) (x), 1)
#define __iscanonical(x) ((void) (__typeof (x)) (x), 1)
#if __HAVE_DISTINCT_FLOAT128
# define __iscanonicalf128(x) ((void) (__typeof (x)) (x), 1)
#endif

/* Return nonzero value if X is canonical.  In IEEE interchange binary
   formats, all values are canonical, but the argument must still be
   converted to its semantic type for any exceptions arising from the
   conversion, before being discarded; in extended precision, there
   are encodings that are not consistently handled as corresponding to
   any particular value of the type, and we return 0 for those.  */
#ifndef __cplusplus
# define iscanonical(x) __MATH_TG ((x), __iscanonical, (x))
#else
/* In C++ mode, __MATH_TG cannot be used, because it relies on
   __builtin_types_compatible_p, which is a C-only builtin.  On the
   other hand, overloading provides the means to distinguish between
   the floating-point types.  The overloading resolution will match
   the correct parameter (regardless of type qualifiers (i.e.: const
   and volatile)).  */
extern "C++" {
inline int iscanonical (float __val) { return __iscanonicalf (__val); }
inline int iscanonical (double __val) { return __iscanonical (__val); }
inline int iscanonical (long double __val) { return __iscanonicall (__val); }
# if __HAVE_DISTINCT_FLOAT128
inline int iscanonical (_Float128 __val) { return __iscanonicalf128 (__val); }
# endif
}
#endif /* __cplusplus */
