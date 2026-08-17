/* Copyright (C) 1995-2020 Free Software Foundation, Inc.
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
# error "Never include <bits/shm.h> directly; use <sys/shm.h> instead."
#endif

#include <bits/types.h>
#include <bits/wordsize.h>
#include <bits/shmlba.h>
#include <bits/shm-pad.h>

/* Permission flag for shmget.  */
#define SHM_R		0400		/* or S_IRUGO from <linux/stat.h> */
#define SHM_W		0200		/* or S_IWUGO from <linux/stat.h> */

/* Flags for `shmat'.  */
#define SHM_RDONLY	010000		/* attach read-only else read-write */
#define SHM_RND		020000		/* round attach address to SHMLBA */
#define SHM_REMAP	040000		/* take-over region on attach */
#define SHM_EXEC	0100000		/* execution access */

/* Commands for `shmctl'.  */
#define SHM_LOCK	11		/* lock segment (root only) */
#define SHM_UNLOCK	12		/* unlock segment (root only) */

__BEGIN_DECLS

/* Type to count number of attaches.  */
typedef __syscall_ulong_t shmatt_t;

#if __SHM_PAD_BEFORE_TIME
# define __SHM_PAD_TIME(NAME, RES)				\
  unsigned long int __glibc_reserved ## RES; __time_t NAME
#elif __SHM_PAD_AFTER_TIME
# define __SHM_PAD_TIME(NAME, RES)				\
  __time_t NAME; unsigned long int __glibc_reserved ## RES
#else
# define __SHM_PAD_TIME(NAME, RES)		\
  __time_t NAME
#endif

/* Data structure describing a shared memory segment.  */
struct shmid_ds
  {
    struct ipc_perm shm_perm;		/* operation permission struct */
#if !__SHM_SEGSZ_AFTER_TIME
    size_t shm_segsz;			/* size of segment in bytes */
#endif
    __SHM_PAD_TIME (shm_atime, 1);	/* time of last shmat() */
    __SHM_PAD_TIME (shm_dtime, 2);	/* time of last shmdt() */
    __SHM_PAD_TIME (shm_ctime, 3);	/* time of last change by shmctl() */
#if __SHM_PAD_BETWEEN_TIME_AND_SEGSZ
    unsigned long int __glibc_reserved4;
#endif
#if __SHM_SEGSZ_AFTER_TIME
    size_t shm_segsz;			/* size of segment in bytes */
#endif
    __pid_t shm_cpid;			/* pid of creator */
    __pid_t shm_lpid;			/* pid of last shmop */
    shmatt_t shm_nattch;		/* number of current attaches */
    __syscall_ulong_t __glibc_reserved5;
    __syscall_ulong_t __glibc_reserved6;
  };

#ifdef __USE_MISC

/* ipcs ctl commands */
# define SHM_STAT 	13
# define SHM_INFO 	14
# define SHM_STAT_ANY	15

/* shm_mode upper byte flags */
# define SHM_DEST	01000	/* segment will be destroyed on last detach */
# define SHM_LOCKED	02000   /* segment will not be swapped */
# define SHM_HUGETLB	04000	/* segment is mapped via hugetlb */
# define SHM_NORESERVE	010000	/* don't check for reservations */

struct	shminfo
  {
    __syscall_ulong_t shmmax;
    __syscall_ulong_t shmmin;
    __syscall_ulong_t shmmni;
    __syscall_ulong_t shmseg;
    __syscall_ulong_t shmall;
    __syscall_ulong_t __glibc_reserved1;
    __syscall_ulong_t __glibc_reserved2;
    __syscall_ulong_t __glibc_reserved3;
    __syscall_ulong_t __glibc_reserved4;
  };

struct shm_info
  {
    int used_ids;
    __syscall_ulong_t shm_tot;	/* total allocated shm */
    __syscall_ulong_t shm_rss;	/* total resident shm */
    __syscall_ulong_t shm_swp;	/* total swapped shm */
    __syscall_ulong_t swap_attempts;
    __syscall_ulong_t swap_successes;
  };

#endif /* __USE_MISC */

__END_DECLS
