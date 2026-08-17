/* Define where padding goes in struct shmid_ds.  Generic version.
   Copyright (C) 2018-2020 Free Software Foundation, Inc.
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

#ifndef _SYS_SHM_H
# error "Never use <bits/shm-pad.h> directly; include <sys/shm.h> instead."
#endif

#include <bits/timesize.h>

/* On most architectures, padding goes after time fields for 32-bit
   systems and is omitted for 64-bit systems.  Some architectures pad
   before time fields instead, or omit padding despite being 32-bit,
   or include it despite being 64-bit.  Furthermore, some
   architectures place shm_segsz after the time fields rather than
   before them, with or without padding there.  This must match the
   layout used for struct shmid64_ds in <asm/shmbuf.h>, as glibc does
   not do layout conversions for this structure.  */

#define __SHM_PAD_AFTER_TIME (__TIMESIZE == 32)
#define __SHM_PAD_BEFORE_TIME 0
#define __SHM_SEGSZ_AFTER_TIME 0
#define __SHM_PAD_BETWEEN_TIME_AND_SEGSZ 0
