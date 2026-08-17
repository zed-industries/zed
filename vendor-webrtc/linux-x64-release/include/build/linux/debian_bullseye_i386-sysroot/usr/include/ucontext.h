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

/* System V ABI compliant user-level context switching support.  */

#ifndef _UCONTEXT_H
#define _UCONTEXT_H	1

#include <features.h>

/* Get definition of __INDIRECT_RETURN.  */
#include <bits/indirect-return.h>

/* Get machine dependent definition of data structures.  */
#include <sys/ucontext.h>

__BEGIN_DECLS

/* Get user context and store it in variable pointed to by UCP.  */
extern int getcontext (ucontext_t *__ucp) __THROWNL;

/* Set user context from information of variable pointed to by UCP.  */
extern int setcontext (const ucontext_t *__ucp) __THROWNL;

/* Save current context in context variable pointed to by OUCP and set
   context from variable pointed to by UCP.  */
extern int swapcontext (ucontext_t *__restrict __oucp,
			const ucontext_t *__restrict __ucp)
  __THROWNL __INDIRECT_RETURN;

/* Manipulate user context UCP to continue with calling functions FUNC
   and the ARGC-1 parameters following ARGC when the context is used
   the next time in `setcontext' or `swapcontext'.

   We cannot say anything about the parameters FUNC takes; `void'
   is as good as any other choice.  */
extern void makecontext (ucontext_t *__ucp, void (*__func) (void),
			 int __argc, ...) __THROW;

__END_DECLS

#endif /* ucontext.h */
