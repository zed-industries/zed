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

#ifndef _FENV_H
# error "Never use <bits/fenv.h> directly; include <fenv.h> instead."
#endif

/* Define bits representing the exception.  We use the bit positions
   of the appropriate bits in the FPU control word.  */
enum
  {
    FE_INVALID =
#define FE_INVALID	0x01
      FE_INVALID,
    __FE_DENORM = 0x02,
    FE_DIVBYZERO =
#define FE_DIVBYZERO	0x04
      FE_DIVBYZERO,
    FE_OVERFLOW =
#define FE_OVERFLOW	0x08
      FE_OVERFLOW,
    FE_UNDERFLOW =
#define FE_UNDERFLOW	0x10
      FE_UNDERFLOW,
    FE_INEXACT =
#define FE_INEXACT	0x20
      FE_INEXACT
  };

#define FE_ALL_EXCEPT \
	(FE_INEXACT | FE_DIVBYZERO | FE_UNDERFLOW | FE_OVERFLOW | FE_INVALID)

/* The ix87 FPU supports all of the four defined rounding modes.  We
   use again the bit positions in the FPU control word as the values
   for the appropriate macros.  */
enum
  {
    FE_TONEAREST =
#define FE_TONEAREST	0
      FE_TONEAREST,
    FE_DOWNWARD =
#define FE_DOWNWARD	0x400
      FE_DOWNWARD,
    FE_UPWARD =
#define FE_UPWARD	0x800
      FE_UPWARD,
    FE_TOWARDZERO =
#define FE_TOWARDZERO	0xc00
      FE_TOWARDZERO
  };


/* Type representing exception flags.  */
typedef unsigned short int fexcept_t;


/* Type representing floating-point environment.  This structure
   corresponds to the layout of the block written by the `fstenv'
   instruction and has additional fields for the contents of the MXCSR
   register as written by the `stmxcsr' instruction.  */
typedef struct
  {
    unsigned short int __control_word;
    unsigned short int __glibc_reserved1;
    unsigned short int __status_word;
    unsigned short int __glibc_reserved2;
    unsigned short int __tags;
    unsigned short int __glibc_reserved3;
    unsigned int __eip;
    unsigned short int __cs_selector;
    unsigned int __opcode:11;
    unsigned int __glibc_reserved4:5;
    unsigned int __data_offset;
    unsigned short int __data_selector;
    unsigned short int __glibc_reserved5;
#ifdef __x86_64__
    unsigned int __mxcsr;
#endif
  }
fenv_t;

/* If the default argument is used we use this value.  */
#define FE_DFL_ENV	((const fenv_t *) -1)

#ifdef __USE_GNU
/* Floating-point environment where none of the exception is masked.  */
# define FE_NOMASK_ENV	((const fenv_t *) -2)
#endif

#if __GLIBC_USE (IEC_60559_BFP_EXT_C2X)
/* Type representing floating-point control modes.  */
typedef struct
  {
    unsigned short int __control_word;
    unsigned short int __glibc_reserved;
    unsigned int __mxcsr;
  }
femode_t;

/* Default floating-point control modes.  */
# define FE_DFL_MODE	((const femode_t *) -1L)
#endif


#ifdef __USE_EXTERN_INLINES
__BEGIN_DECLS

/* Optimized versions.  */
#ifndef _LIBC
extern int __REDIRECT_NTH (__feraiseexcept_renamed, (int), feraiseexcept);
#endif
__extern_always_inline void
__NTH (__feraiseexcept_invalid_divbyzero (int __excepts))
{
  if ((FE_INVALID & __excepts) != 0)
    {
      /* One example of an invalid operation is 0.0 / 0.0.  */
      float __f = 0.0;

# ifdef __SSE_MATH__
      __asm__ __volatile__ ("divss %0, %0 " : : "x" (__f));
# else
      __asm__ __volatile__ ("fdiv %%st, %%st(0); fwait"
			    : "=t" (__f) : "0" (__f));
# endif
      (void) &__f;
    }
  if ((FE_DIVBYZERO & __excepts) != 0)
    {
      float __f = 1.0;
      float __g = 0.0;

# ifdef __SSE_MATH__
      __asm__ __volatile__ ("divss %1, %0" : : "x" (__f), "x" (__g));
# else
      __asm__ __volatile__ ("fdivp %%st, %%st(1); fwait"
			    : "=t" (__f) : "0" (__f), "u" (__g) : "st(1)");
# endif
      (void) &__f;
    }
}
__extern_inline int
__NTH (feraiseexcept (int __excepts))
{
  if (__builtin_constant_p (__excepts)
      && (__excepts & ~(FE_INVALID | FE_DIVBYZERO)) == 0)
    {
      __feraiseexcept_invalid_divbyzero (__excepts);
      return 0;
    }

  return __feraiseexcept_renamed (__excepts);
}

__END_DECLS
#endif
