
/* System call numbers for Linux that are shared across all architectures. */

/*
   This file is part of Valgrind, a dynamic binary instrumentation
   framework.

   Copyright (C) 2019 Bart Van Assche <bvanassche@acm.org>

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

   The GNU General Public License is contained in the file COPYING.
*/

#ifndef __VKI_SCNUMS_SHARED_LINUX_H
#define __VKI_SCNUMS_SHARED_LINUX_H

// Derived from linux-5.2/include/uapi/asm-generic/unistd.h

#define __NR_pidfd_send_signal	424
#define __NR_io_uring_setup	425
#define __NR_io_uring_enter	426
#define __NR_io_uring_register	427
#define __NR_open_tree		428
#define __NR_move_mount		429
#define __NR_fsopen		430
#define __NR_fsconfig		431
#define __NR_fsmount		432
#define __NR_fspick		433

#endif
