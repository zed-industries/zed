/* -mlong-double-64 compatibility mode for stdio functions.
   Copyright (C) 2006-2020 Free Software Foundation, Inc.
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

#ifndef _STDIO_H
# error "Never include <bits/stdio-ldbl.h> directly; use <stdio.h> instead."
#endif

__LDBL_REDIR_DECL (fprintf)
__LDBL_REDIR_DECL (printf)
__LDBL_REDIR_DECL (sprintf)
__LDBL_REDIR_DECL (vfprintf)
__LDBL_REDIR_DECL (vprintf)
__LDBL_REDIR_DECL (vsprintf)
#if !__GLIBC_USE (DEPRECATED_SCANF)
__LDBL_REDIR1_DECL (fscanf, __nldbl___isoc99_fscanf)
__LDBL_REDIR1_DECL (scanf, __nldbl___isoc99_scanf)
__LDBL_REDIR1_DECL (sscanf, __nldbl___isoc99_sscanf)
#else
__LDBL_REDIR_DECL (fscanf)
__LDBL_REDIR_DECL (scanf)
__LDBL_REDIR_DECL (sscanf)
#endif

#if defined __USE_ISOC99 || defined __USE_UNIX98
__LDBL_REDIR_DECL (snprintf)
__LDBL_REDIR_DECL (vsnprintf)
#endif

#ifdef	__USE_ISOC99
# if !__GLIBC_USE (DEPRECATED_SCANF)
__LDBL_REDIR1_DECL (vfscanf, __nldbl___isoc99_vfscanf)
__LDBL_REDIR1_DECL (vscanf, __nldbl___isoc99_vscanf)
__LDBL_REDIR1_DECL (vsscanf, __nldbl___isoc99_vsscanf)
# else
__LDBL_REDIR_DECL (vfscanf)
__LDBL_REDIR_DECL (vsscanf)
__LDBL_REDIR_DECL (vscanf)
# endif
#endif

#ifdef __USE_XOPEN2K8
__LDBL_REDIR_DECL (vdprintf)
__LDBL_REDIR_DECL (dprintf)
#endif

#ifdef __USE_GNU
__LDBL_REDIR_DECL (vasprintf)
__LDBL_REDIR_DECL (__asprintf)
__LDBL_REDIR_DECL (asprintf)
__LDBL_REDIR_DECL (obstack_printf)
__LDBL_REDIR_DECL (obstack_vprintf)
#endif

#if __USE_FORTIFY_LEVEL > 0 && defined __fortify_function
__LDBL_REDIR_DECL (__sprintf_chk)
__LDBL_REDIR_DECL (__vsprintf_chk)
# if defined __USE_ISOC99 || defined __USE_UNIX98
__LDBL_REDIR_DECL (__snprintf_chk)
__LDBL_REDIR_DECL (__vsnprintf_chk)
# endif
# if __USE_FORTIFY_LEVEL > 1
__LDBL_REDIR_DECL (__fprintf_chk)
__LDBL_REDIR_DECL (__printf_chk)
__LDBL_REDIR_DECL (__vfprintf_chk)
__LDBL_REDIR_DECL (__vprintf_chk)
#  ifdef __USE_XOPEN2K8
__LDBL_REDIR_DECL (__dprintf_chk)
__LDBL_REDIR_DECL (__vdprintf_chk)
#  endif
#  ifdef __USE_GNU
__LDBL_REDIR_DECL (__asprintf_chk)
__LDBL_REDIR_DECL (__vasprintf_chk)
__LDBL_REDIR_DECL (__obstack_printf_chk)
__LDBL_REDIR_DECL (__obstack_vprintf_chk)
#  endif
# endif
#endif
