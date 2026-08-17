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
 * ISO C99 7.6: Floating-point environment	<fenv.h>
 */

#ifndef _FENV_H
#define _FENV_H	1

#define __GLIBC_INTERNAL_STARTING_HEADER_IMPLEMENTATION
#include <bits/libc-header-start.h>

/* Get the architecture dependend definitions.  The following definitions
   are expected to be done:

   fenv_t	type for object representing an entire floating-point
		environment

   FE_DFL_ENV	macro of type pointer to fenv_t to be used as the argument
		to functions taking an argument of type fenv_t; in this
		case the default environment will be used

   fexcept_t	type for object representing the floating-point exception
		flags including status associated with the flags

   femode_t	type for object representing floating-point control modes

   FE_DFL_MODE	macro of type pointer to const femode_t to be used as the
		argument to fesetmode; in this case the default control
		modes will be used

   The following macros are defined iff the implementation supports this
   kind of exception.
   FE_INEXACT		inexact result
   FE_DIVBYZERO		division by zero
   FE_UNDERFLOW		result not representable due to underflow
   FE_OVERFLOW		result not representable due to overflow
   FE_INVALID		invalid operation

   FE_ALL_EXCEPT	bitwise OR of all supported exceptions

   The next macros are defined iff the appropriate rounding mode is
   supported by the implementation.
   FE_TONEAREST		round to nearest
   FE_UPWARD		round toward +Inf
   FE_DOWNWARD		round toward -Inf
   FE_TOWARDZERO	round toward 0
*/
#include <bits/fenv.h>

__BEGIN_DECLS

/* Floating-point exception handling.  */

/* Clear the supported exceptions represented by EXCEPTS.  */
extern int feclearexcept (int __excepts) __THROW;

/* Store implementation-defined representation of the exception flags
   indicated by EXCEPTS in the object pointed to by FLAGP.  */
extern int fegetexceptflag (fexcept_t *__flagp, int __excepts) __THROW;

/* Raise the supported exceptions represented by EXCEPTS.  */
extern int feraiseexcept (int __excepts) __THROW;

#if __GLIBC_USE (IEC_60559_BFP_EXT_C2X)
/* Set the supported exception flags represented by EXCEPTS, without
   causing enabled traps to be taken.  */
extern int fesetexcept (int __excepts) __THROW;
#endif

/* Set complete status for exceptions indicated by EXCEPTS according to
   the representation in the object pointed to by FLAGP.  */
extern int fesetexceptflag (const fexcept_t *__flagp, int __excepts) __THROW;

/* Determine which of subset of the exceptions specified by EXCEPTS are
   currently set.  */
extern int fetestexcept (int __excepts) __THROW;

#if __GLIBC_USE (IEC_60559_BFP_EXT_C2X)
/* Determine which of subset of the exceptions specified by EXCEPTS
   are set in *FLAGP.  */
extern int fetestexceptflag (const fexcept_t *__flagp, int __excepts) __THROW;
#endif


/* Rounding control.  */

/* Get current rounding direction.  */
extern int fegetround (void) __THROW __attribute_pure__;

/* Establish the rounding direction represented by ROUND.  */
extern int fesetround (int __rounding_direction) __THROW;


/* Floating-point environment.  */

/* Store the current floating-point environment in the object pointed
   to by ENVP.  */
extern int fegetenv (fenv_t *__envp) __THROW;

/* Save the current environment in the object pointed to by ENVP, clear
   exception flags and install a non-stop mode (if available) for all
   exceptions.  */
extern int feholdexcept (fenv_t *__envp) __THROW;

/* Establish the floating-point environment represented by the object
   pointed to by ENVP.  */
extern int fesetenv (const fenv_t *__envp) __THROW;

/* Save current exceptions in temporary storage, install environment
   represented by object pointed to by ENVP and raise exceptions
   according to saved exceptions.  */
extern int feupdateenv (const fenv_t *__envp) __THROW;


/* Control modes.  */

#if __GLIBC_USE (IEC_60559_BFP_EXT_C2X)
/* Store the current floating-point control modes in the object
   pointed to by MODEP.  */
extern int fegetmode (femode_t *__modep) __THROW;

/* Establish the floating-point control modes represented by the
   object pointed to by MODEP.  */
extern int fesetmode (const femode_t *__modep) __THROW;
#endif

/* Include optimization.  */
#ifdef __OPTIMIZE__
# include <bits/fenvinline.h>
#endif

/* NaN support.  */

#if (__GLIBC_USE (IEC_60559_BFP_EXT_C2X)		\
     && defined FE_INVALID			\
     && defined __SUPPORT_SNAN__)
# define FE_SNANS_ALWAYS_SIGNAL	1
#endif

#ifdef __USE_GNU

/* Enable individual exceptions.  Will not enable more exceptions than
   EXCEPTS specifies.  Returns the previous enabled exceptions if all
   exceptions are successfully set, otherwise returns -1.  */
extern int feenableexcept (int __excepts) __THROW;

/* Disable individual exceptions.  Will not disable more exceptions than
   EXCEPTS specifies.  Returns the previous enabled exceptions if all
   exceptions are successfully disabled, otherwise returns -1.  */
extern int fedisableexcept (int __excepts) __THROW;

/* Return enabled exceptions.  */
extern int fegetexcept (void) __THROW;
#endif

__END_DECLS

#endif /* fenv.h */
