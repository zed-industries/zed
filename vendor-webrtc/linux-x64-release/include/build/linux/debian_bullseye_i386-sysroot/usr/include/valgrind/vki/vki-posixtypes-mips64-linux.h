
/*--------------------------------------------------------------------*/
/*--- mips/Linux-specific kernel interface: posix types.           ---*/
/*---                                vki-posixtypes-mips64-linux.h ---*/
/*--------------------------------------------------------------------*/

/*
   This file is part of Valgrind, a dynamic binary instrumentation
   framework.

   Copyright (C) 2010-2017 RT-RK
      mips-valgrind@rt-rk.com

   This program is free software; you can redistribute it and/or
   modify it under the terms of the GNU General Public License as
   published by the Free Software Foundation; either version 2 of the
   License, or (at your option) any later version.

   This program is distributed in the hope that it will be useful, but
   WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program; if not, see <http://www.gnu.org/licenses/>.
*/

#ifndef __VKI_POSIXTYPES_MIPS64_LINUX_H
#define __VKI_POSIXTYPES_MIPS64_LINUX_H

typedef unsigned long   __vki_kernel_ino_t;
typedef unsigned int    __vki_kernel_mode_t;
#if (_MIPS_SZLONG == 32)
typedef unsigned long   __vki_kernel_nlink_t;
#endif
#if (_MIPS_SZLONG == 64)
typedef unsigned int    __vki_kernel_nlink_t;
#endif
typedef long            __vki_kernel_off_t;
typedef int             __vki_kernel_pid_t;
typedef int             __vki_kernel_ipc_pid_t;
typedef unsigned int    __vki_kernel_uid_t;
typedef unsigned int    __vki_kernel_gid_t;
#if (_MIPS_SZLONG == 32)
typedef unsigned int    __vki_kernel_size_t;
typedef int             __vki_kernel_ssize_t;
typedef int             __vki_kernel_ptrdiff_t;
#endif
#if (_MIPS_SZLONG == 64)
typedef unsigned long   __vki_kernel_size_t;
typedef long            __vki_kernel_ssize_t;
typedef long            __vki_kernel_ptrdiff_t;
#endif
typedef long            __vki_kernel_time_t;
typedef long            __vki_kernel_suseconds_t;
typedef long            __vki_kernel_clock_t;
typedef int             __vki_kernel_timer_t;
typedef int             __vki_kernel_clockid_t;
typedef long            __vki_kernel_daddr_t;
typedef char *          __vki_kernel_caddr_t;

typedef unsigned short  __vki_kernel_uid16_t;
typedef unsigned short  __vki_kernel_gid16_t;
typedef unsigned int    __vki_kernel_uid32_t;
typedef unsigned int    __vki_kernel_gid32_t;
typedef __vki_kernel_uid_t  __vki_kernel_old_uid_t;
typedef __vki_kernel_gid_t  __vki_kernel_old_gid_t;
typedef unsigned int    __vki_kernel_old_dev_t;

typedef long long      __vki_kernel_loff_t;

typedef struct {
#if (_MIPS_SZLONG == 32)
        long    val[2];
#endif
#if (_MIPS_SZLONG == 64)
        int     val[2];
#endif
} __vki_kernel_fsid_t;

#endif // __VKI_POSIXTYPES_MIPS64_LINUX_H

/*--------------------------------------------------------------------*/
/*--- end                              vki-posixtypes-mips-linux.h ---*/
/*--------------------------------------------------------------------*/
