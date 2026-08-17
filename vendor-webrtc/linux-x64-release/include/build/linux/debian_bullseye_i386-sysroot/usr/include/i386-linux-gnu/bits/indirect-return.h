/* Definition of __INDIRECT_RETURN.  x86 version.
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

#ifndef _UCONTEXT_H
# error "Never include <bits/indirect-return.h> directly; use <ucontext.h> instead."
#endif

/* On x86, swapcontext returns via indirect branch when the shadow stack
   is enabled.  Define __INDIRECT_RETURN to indicate whether swapcontext
   returns via indirect branch.  */
#if defined __CET__ && (__CET__ & 2) != 0
# if __glibc_has_attribute (__indirect_return__)
#  define __INDIRECT_RETURN __attribute__ ((__indirect_return__))
# else
/* Newer compilers provide the indirect_return attribute, but without
   it we can use returns_twice to affect the optimizer in the same
   way and avoid unsafe optimizations.  */
#  define __INDIRECT_RETURN __attribute__ ((__returns_twice__))
# endif
#else
# define __INDIRECT_RETURN
#endif
