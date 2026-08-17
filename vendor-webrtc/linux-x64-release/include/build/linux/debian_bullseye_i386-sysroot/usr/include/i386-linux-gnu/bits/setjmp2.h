/* Checking macros for setjmp functions.
   Copyright (C) 2009-2020 Free Software Foundation, Inc.
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

#ifndef _SETJMP_H
# error "Never include <bits/setjmp2.h> directly; use <setjmp.h> instead."
#endif

/* Variant of the longjmp functions which perform some sanity checking.  */
#ifdef __REDIRECT_NTH
extern void __REDIRECT_NTHNL (longjmp,
			      (struct __jmp_buf_tag __env[1], int __val),
			      __longjmp_chk) __attribute__ ((__noreturn__));
extern void __REDIRECT_NTHNL (_longjmp,
			      (struct __jmp_buf_tag __env[1], int __val),
			      __longjmp_chk) __attribute__ ((__noreturn__));
extern void __REDIRECT_NTHNL (siglongjmp,
			      (struct __jmp_buf_tag __env[1], int __val),
			      __longjmp_chk) __attribute__ ((__noreturn__));
#else
extern void __longjmp_chk (struct __jmp_buf_tag __env[1], int __val),
     __THROWNL __attribute__ ((__noreturn__));
# define longjmp __longjmp_chk
# define _longjmp __longjmp_chk
# define siglongjmp __longjmp_chk
#endif
