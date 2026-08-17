/* Definitions of macros to access `dev_t' values.
   Copyright (C) 1996-2020 Free Software Foundation, Inc.
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

#ifndef _SYS_SYSMACROS_H
#define _SYS_SYSMACROS_H 1

#include <features.h>
#include <bits/types.h>
#include <bits/sysmacros.h>

#define __SYSMACROS_DECL_TEMPL(rtype, name, proto)			     \
  extern rtype gnu_dev_##name proto __THROW __attribute_const__;

#define __SYSMACROS_IMPL_TEMPL(rtype, name, proto)			     \
  __extension__ __extern_inline __attribute_const__ rtype		     \
  __NTH (gnu_dev_##name proto)

__BEGIN_DECLS

__SYSMACROS_DECLARE_MAJOR (__SYSMACROS_DECL_TEMPL)
__SYSMACROS_DECLARE_MINOR (__SYSMACROS_DECL_TEMPL)
__SYSMACROS_DECLARE_MAKEDEV (__SYSMACROS_DECL_TEMPL)

#ifdef __USE_EXTERN_INLINES

__SYSMACROS_DEFINE_MAJOR (__SYSMACROS_IMPL_TEMPL)
__SYSMACROS_DEFINE_MINOR (__SYSMACROS_IMPL_TEMPL)
__SYSMACROS_DEFINE_MAKEDEV (__SYSMACROS_IMPL_TEMPL)

#endif

__END_DECLS

#ifndef __SYSMACROS_NEED_IMPLEMENTATION
# undef __SYSMACROS_DECL_TEMPL
# undef __SYSMACROS_IMPL_TEMPL
# undef __SYSMACROS_DECLARE_MAJOR
# undef __SYSMACROS_DECLARE_MINOR
# undef __SYSMACROS_DECLARE_MAKEDEV
# undef __SYSMACROS_DEFINE_MAJOR
# undef __SYSMACROS_DEFINE_MINOR
# undef __SYSMACROS_DEFINE_MAKEDEV
#endif

#define major(dev) gnu_dev_major (dev)
#define minor(dev) gnu_dev_minor (dev)
#define makedev(maj, min) gnu_dev_makedev (maj, min)

#endif /* sys/sysmacros.h */
