/* Define FP_FAST_* macros.
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
# error "Never use <bits/fp-fast.h> directly; include <math.h> instead."
#endif

#ifdef __USE_ISOC99

/* The GCC 4.6 compiler will define __FP_FAST_FMA{,F,L} if the fma{,f,l}
   builtins are supported.  */
# ifdef __FP_FAST_FMA
#  define FP_FAST_FMA 1
# endif

# ifdef __FP_FAST_FMAF
#  define FP_FAST_FMAF 1
# endif

# ifdef __FP_FAST_FMAL
#  define FP_FAST_FMAL 1
# endif

#endif
