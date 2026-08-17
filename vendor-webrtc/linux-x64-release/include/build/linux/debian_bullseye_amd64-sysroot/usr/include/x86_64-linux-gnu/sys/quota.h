/* This just represents the non-kernel parts of <linux/quota.h>.
   Copyright (C) 1998-2020 Free Software Foundation, Inc.
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

/*
 * Copyright (c) 1982, 1986 Regents of the University of California.
 * All rights reserved.
 *
 * This code is derived from software contributed to Berkeley by
 * Robert Elz at The University of Melbourne.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 4. Neither the name of the University nor the names of its contributors
 *    may be used to endorse or promote products derived from this software
 *    without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE REGENTS AND CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE REGENTS OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 */

#ifndef _SYS_QUOTA_H
#define _SYS_QUOTA_H 1

#include <features.h>
#include <sys/types.h>

#include <linux/quota.h>

/*
 * Convert diskblocks to blocks and the other way around.
 * currently only to fool the BSD source. :-)
 */
#define dbtob(num) ((num) << 10)
#define btodb(num) ((num) >> 10)

/*
 * Convert count of filesystem blocks to diskquota blocks, meant for
 * filesystems where i_blksize != 1024.
 */
#define fs_to_dq_blocks(num, blksize) (((num) * (blksize)) / 1024)

/*
 * Definitions for disk quotas imposed on the average user
 * (big brother finally hits Linux).
 *
 * The following constants define the amount of time given a user
 * before the soft limits are treated as hard limits (usually resulting
 * in an allocation failure). The timer is started when the user crosses
 * their soft limit, it is reset when they go below their soft limit.
 */
#define MAX_IQ_TIME  604800	/* (7*24*60*60) 1 week */
#define MAX_DQ_TIME  604800	/* (7*24*60*60) 1 week */

#define QUOTAFILENAME "quota"
#define QUOTAGROUP "staff"

#define NR_DQHASH 43          /* Just an arbitrary number any suggestions ? */
#define NR_DQUOTS 256         /* Number of quotas active at one time */

/* Old name for struct if_dqblk.  */
struct dqblk
  {
    __uint64_t dqb_bhardlimit;	/* absolute limit on disk quota blocks alloc */
    __uint64_t dqb_bsoftlimit;	/* preferred limit on disk quota blocks */
    __uint64_t dqb_curspace;	/* current quota block count */
    __uint64_t dqb_ihardlimit;	/* maximum # allocated inodes */
    __uint64_t dqb_isoftlimit;	/* preferred inode limit */
    __uint64_t dqb_curinodes;	/* current # allocated inodes */
    __uint64_t dqb_btime;	/* time limit for excessive disk use */
    __uint64_t dqb_itime;	/* time limit for excessive files */
    __uint32_t dqb_valid;	/* bitmask of QIF_* constants */
  };

/*
 * Shorthand notation.
 */
#define	dq_bhardlimit	dq_dqb.dqb_bhardlimit
#define	dq_bsoftlimit	dq_dqb.dqb_bsoftlimit
#define dq_curspace	dq_dqb.dqb_curspace
#define dq_valid	dq_dqb.dqb_valid
#define	dq_ihardlimit	dq_dqb.dqb_ihardlimit
#define	dq_isoftlimit	dq_dqb.dqb_isoftlimit
#define	dq_curinodes	dq_dqb.dqb_curinodes
#define	dq_btime	dq_dqb.dqb_btime
#define	dq_itime	dq_dqb.dqb_itime

#define dqoff(UID)      ((__loff_t)((UID) * sizeof (struct dqblk)))

/* Old name for struct if_dqinfo.  */
struct dqinfo
  {
    __uint64_t dqi_bgrace;
    __uint64_t dqi_igrace;
    __uint32_t dqi_flags;
    __uint32_t dqi_valid;
  };

__BEGIN_DECLS

extern int quotactl (int __cmd, const char *__special, int __id,
		     __caddr_t __addr) __THROW;

__END_DECLS

#endif /* sys/quota.h */
