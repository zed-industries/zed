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

/* This file should define RB_* macros to be used as flag
   bits in the argument to the `reboot' system call.  */

#ifndef _SYS_REBOOT_H
#define _SYS_REBOOT_H	1

#include <features.h>

/* Perform a hard reset now.  */
#define RB_AUTOBOOT	0x01234567

/* Halt the system.  */
#define RB_HALT_SYSTEM	0xcdef0123

/* Enable reboot using Ctrl-Alt-Delete keystroke.  */
#define RB_ENABLE_CAD	0x89abcdef

/* Disable reboot using Ctrl-Alt-Delete keystroke.  */
#define RB_DISABLE_CAD	0

/* Stop system and switch power off if possible.  */
#define RB_POWER_OFF	0x4321fedc

/* Suspend system using software suspend.  */
#define RB_SW_SUSPEND	0xd000fce2

/* Reboot system into new kernel.  */
#define RB_KEXEC	0x45584543

__BEGIN_DECLS

/* Reboot or halt the system.  */
extern int reboot (int __howto) __THROW;

__END_DECLS

#endif	/* _SYS_REBOOT_H */
