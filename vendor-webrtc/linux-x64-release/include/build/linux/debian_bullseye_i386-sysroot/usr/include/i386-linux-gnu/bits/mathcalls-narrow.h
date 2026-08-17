/* Declare functions returning a narrower type.
   Copyright (C) 2018-2020 Free Software Foundation, Inc.
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
# error "Never include <bits/mathcalls-narrow.h> directly; include <math.h> instead."
#endif

/* Add.  */
__MATHCALL_NARROW (__MATHCALL_NAME (add), __MATHCALL_REDIR_NAME (add), 2);

/* Divide.  */
__MATHCALL_NARROW (__MATHCALL_NAME (div), __MATHCALL_REDIR_NAME (div), 2);

/* Multiply.  */
__MATHCALL_NARROW (__MATHCALL_NAME (mul), __MATHCALL_REDIR_NAME (mul), 2);

/* Subtract.  */
__MATHCALL_NARROW (__MATHCALL_NAME (sub), __MATHCALL_REDIR_NAME (sub), 2);
