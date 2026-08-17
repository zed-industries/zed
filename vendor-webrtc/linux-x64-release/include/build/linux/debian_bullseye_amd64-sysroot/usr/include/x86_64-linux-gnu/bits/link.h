/* Copyright (C) 2004-2020 Free Software Foundation, Inc.
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

#ifndef	_LINK_H
# error "Never include <bits/link.h> directly; use <link.h> instead."
#endif


#ifndef __x86_64__
/* Registers for entry into PLT on IA-32.  */
typedef struct La_i86_regs
{
  uint32_t lr_edx;
  uint32_t lr_ecx;
  uint32_t lr_eax;
  uint32_t lr_ebp;
  uint32_t lr_esp;
} La_i86_regs;

/* Return values for calls from PLT on IA-32.  */
typedef struct La_i86_retval
{
  uint32_t lrv_eax;
  uint32_t lrv_edx;
  long double lrv_st0;
  long double lrv_st1;
  uint64_t lrv_bnd0;
  uint64_t lrv_bnd1;
} La_i86_retval;


__BEGIN_DECLS

extern Elf32_Addr la_i86_gnu_pltenter (Elf32_Sym *__sym, unsigned int __ndx,
				       uintptr_t *__refcook,
				       uintptr_t *__defcook,
				       La_i86_regs *__regs,
				       unsigned int *__flags,
				       const char *__symname,
				       long int *__framesizep);
extern unsigned int la_i86_gnu_pltexit (Elf32_Sym *__sym, unsigned int __ndx,
					uintptr_t *__refcook,
					uintptr_t *__defcook,
					const La_i86_regs *__inregs,
					La_i86_retval *__outregs,
					const char *symname);

__END_DECLS

#else

/* Registers for entry into PLT on x86-64.  */
# if __GNUC_PREREQ (4,0)
typedef float La_x86_64_xmm __attribute__ ((__vector_size__ (16)));
typedef float La_x86_64_ymm
    __attribute__ ((__vector_size__ (32), __aligned__ (16)));
typedef double La_x86_64_zmm
    __attribute__ ((__vector_size__ (64), __aligned__ (16)));
# else
typedef float La_x86_64_xmm __attribute__ ((__mode__ (__V4SF__)));
# endif

typedef union
{
# if __GNUC_PREREQ (4,0)
  La_x86_64_ymm ymm[2];
  La_x86_64_zmm zmm[1];
# endif
  La_x86_64_xmm xmm[4];
} La_x86_64_vector __attribute__ ((__aligned__ (16)));

typedef struct La_x86_64_regs
{
  uint64_t lr_rdx;
  uint64_t lr_r8;
  uint64_t lr_r9;
  uint64_t lr_rcx;
  uint64_t lr_rsi;
  uint64_t lr_rdi;
  uint64_t lr_rbp;
  uint64_t lr_rsp;
  La_x86_64_xmm lr_xmm[8];
  La_x86_64_vector lr_vector[8];
#ifndef __ILP32__
  __int128_t lr_bnd[4];
#endif
} La_x86_64_regs;

/* Return values for calls from PLT on x86-64.  */
typedef struct La_x86_64_retval
{
  uint64_t lrv_rax;
  uint64_t lrv_rdx;
  La_x86_64_xmm lrv_xmm0;
  La_x86_64_xmm lrv_xmm1;
  long double lrv_st0;
  long double lrv_st1;
  La_x86_64_vector lrv_vector0;
  La_x86_64_vector lrv_vector1;
#ifndef __ILP32__
  __int128_t lrv_bnd0;
  __int128_t lrv_bnd1;
#endif
} La_x86_64_retval;

#define La_x32_regs La_x86_64_regs
#define La_x32_retval La_x86_64_retval

__BEGIN_DECLS

extern Elf64_Addr la_x86_64_gnu_pltenter (Elf64_Sym *__sym,
					  unsigned int __ndx,
					  uintptr_t *__refcook,
					  uintptr_t *__defcook,
					  La_x86_64_regs *__regs,
					  unsigned int *__flags,
					  const char *__symname,
					  long int *__framesizep);
extern unsigned int la_x86_64_gnu_pltexit (Elf64_Sym *__sym,
					   unsigned int __ndx,
					   uintptr_t *__refcook,
					   uintptr_t *__defcook,
					   const La_x86_64_regs *__inregs,
					   La_x86_64_retval *__outregs,
					   const char *__symname);

extern Elf32_Addr la_x32_gnu_pltenter (Elf32_Sym *__sym,
				       unsigned int __ndx,
				       uintptr_t *__refcook,
				       uintptr_t *__defcook,
				       La_x32_regs *__regs,
				       unsigned int *__flags,
				       const char *__symname,
				       long int *__framesizep);
extern unsigned int la_x32_gnu_pltexit (Elf32_Sym *__sym,
					unsigned int __ndx,
					uintptr_t *__refcook,
					uintptr_t *__defcook,
					const La_x32_regs *__inregs,
					La_x32_retval *__outregs,
					const char *__symname);

__END_DECLS

#endif
