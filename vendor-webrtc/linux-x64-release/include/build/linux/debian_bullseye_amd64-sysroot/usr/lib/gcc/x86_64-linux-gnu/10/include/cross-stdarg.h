/* Copyright (C) 2002-2020 Free Software Foundation, Inc.

   This file is part of GCC.

   GCC is free software; you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation; either version 3, or (at your option)
   any later version.

   GCC is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   Under Section 7 of GPL version 3, you are granted additional
   permissions described in the GCC Runtime Library Exception, version
   3.1, as published by the Free Software Foundation.

   You should have received a copy of the GNU General Public License and
   a copy of the GCC Runtime Library Exception along with this program;
   see the files COPYING3 and COPYING.RUNTIME respectively.  If not, see
   <http://www.gnu.org/licenses/>.  */

#ifndef __CROSS_STDARG_H_INCLUDED
#define __CROSS_STDARG_H_INCLUDED

/* Make sure that for non x64 targets cross builtins are defined.  */
#ifndef __x86_64__
/* Call abi ms_abi.  */
#define __builtin_ms_va_list __builtin_va_list
#define __builtin_ms_va_copy __builtin_va_copy
#define __builtin_ms_va_start __builtin_va_start
#define __builtin_ms_va_end __builtin_va_end

/* Call abi sysv_abi.  */
#define __builtin_sysv_va_list __builtin_va_list
#define __builtin_sysv_va_copy __builtin_va_copy
#define __builtin_sysv_va_start __builtin_va_start
#define __builtin_sysv_va_end __builtin_va_end
#endif

#define __ms_va_copy(__d,__s) __builtin_ms_va_copy(__d,__s)
#define __ms_va_start(__v,__l) __builtin_ms_va_start(__v,__l)
#define __ms_va_arg(__v,__l)	__builtin_va_arg(__v,__l)
#define __ms_va_end(__v) __builtin_ms_va_end(__v)

#define __sysv_va_copy(__d,__s) __builtin_sysv_va_copy(__d,__s)
#define __sysv_va_start(__v,__l) __builtin_sysv_va_start(__v,__l)
#define __sysv_va_arg(__v,__l)	__builtin_va_arg(__v,__l)
#define __sysv_va_end(__v) __builtin_sysv_va_end(__v)

#ifndef __GNUC_SYSV_VA_LIST
#define __GNUC_SYSV_VA_LIST
  typedef __builtin_sysv_va_list __gnuc_sysv_va_list;
#endif

#ifndef _SYSV_VA_LIST_DEFINED
#define _SYSV_VA_LIST_DEFINED
  typedef __gnuc_sysv_va_list sysv_va_list;
#endif

#ifndef __GNUC_MS_VA_LIST
#define __GNUC_MS_VA_LIST
  typedef __builtin_ms_va_list __gnuc_ms_va_list;
#endif

#ifndef _MS_VA_LIST_DEFINED
#define _MS_VA_LIST_DEFINED
  typedef __gnuc_ms_va_list ms_va_list;
#endif

#endif /* __CROSS_STDARG_H_INCLUDED */
