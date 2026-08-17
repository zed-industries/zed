/* Copyright (C) 1996-2020 Free Software Foundation, Inc.
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

#ifndef	_SYS_SYSCTL_H
#define	_SYS_SYSCTL_H	1

#warning "The <sys/sysctl.h> header is deprecated and will be removed."

#include <features.h>
#define __need_size_t
#include <stddef.h>
/* Prevent more kernel headers than necessary to be included.  */
#ifndef _LINUX_KERNEL_H
# define _LINUX_KERNEL_H	1
# define __undef_LINUX_KERNEL_H
#endif
#ifndef _LINUX_TYPES_H
# define _LINUX_TYPES_H		1
# define __undef_LINUX_TYPES_H
#endif
#ifndef _LINUX_LIST_H
# define _LINUX_LIST_H		1
# define __undef_LINUX_LIST_H
#endif
#ifndef __LINUX_COMPILER_H
# define __LINUX_COMPILER_H	1
# define __user
# define __undef__LINUX_COMPILER_H
#endif

#include <linux/sysctl.h>

#ifdef __undef_LINUX_KERNEL_H
# undef _LINUX_KERNEL_H
# undef __undef_LINUX_KERNEL_H
#endif
#ifdef __undef_LINUX_TYPES_H
# undef _LINUX_TYPES_H
# undef __undef_LINUX_TYPES_H
#endif
#ifdef __undef_LINUX_LIST_H
# undef _LINUX_LIST_H
# undef __undef_LINUX_LIST_H
#endif
#ifdef __undef__LINUX_COMPILER_H
# undef __LINUX_COMPILER_H
# undef __user
# undef __undef__LINUX_COMPILER_H
#endif

#include <bits/sysctl.h>

__BEGIN_DECLS

/* Read or write system parameters.  */
extern int sysctl (int *__name, int __nlen, void *__oldval,
		   size_t *__oldlenp, void *__newval, size_t __newlen) __THROW
  __attribute_deprecated__;

__END_DECLS

#endif	/* _SYS_SYSCTL_H */
