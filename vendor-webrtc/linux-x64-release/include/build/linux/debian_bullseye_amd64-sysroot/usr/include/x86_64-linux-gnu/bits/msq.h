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

#ifndef _SYS_MSG_H
# error "Never use <bits/msq.h> directly; include <sys/msg.h> instead."
#endif

#include <bits/types.h>
#include <bits/msq-pad.h>

/* Define options for message queue functions.  */
#define MSG_NOERROR	010000	/* no error if message is too big */
#ifdef __USE_GNU
# define MSG_EXCEPT	020000	/* recv any msg except of specified type */
# define MSG_COPY	040000	/* copy (not remove) all queue messages */
#endif

/* Types used in the structure definition.  */
typedef __syscall_ulong_t msgqnum_t;
typedef __syscall_ulong_t msglen_t;

#if __MSQ_PAD_BEFORE_TIME
# define __MSQ_PAD_TIME(NAME, RES)				\
  unsigned long int __glibc_reserved ## RES; __time_t NAME
#elif __MSQ_PAD_AFTER_TIME
# define __MSQ_PAD_TIME(NAME, RES)				\
  __time_t NAME; unsigned long int __glibc_reserved ## RES
#else
# define __MSQ_PAD_TIME(NAME, RES)		\
  __time_t NAME
#endif

/* Structure of record for one message inside the kernel.
   The type `struct msg' is opaque.  */
struct msqid_ds
{
  struct ipc_perm msg_perm;	/* structure describing operation permission */
  __MSQ_PAD_TIME (msg_stime, 1);	/* time of last msgsnd command */
  __MSQ_PAD_TIME (msg_rtime, 2);	/* time of last msgrcv command */
  __MSQ_PAD_TIME (msg_ctime, 3);	/* time of last change */
  __syscall_ulong_t __msg_cbytes; /* current number of bytes on queue */
  msgqnum_t msg_qnum;		/* number of messages currently on queue */
  msglen_t msg_qbytes;		/* max number of bytes allowed on queue */
  __pid_t msg_lspid;		/* pid of last msgsnd() */
  __pid_t msg_lrpid;		/* pid of last msgrcv() */
  __syscall_ulong_t __glibc_reserved4;
  __syscall_ulong_t __glibc_reserved5;
};

#ifdef __USE_MISC

# define msg_cbytes	__msg_cbytes

/* ipcs ctl commands */
# define MSG_STAT 11
# define MSG_INFO 12
# define MSG_STAT_ANY 13

/* buffer for msgctl calls IPC_INFO, MSG_INFO */
struct msginfo
  {
    int msgpool;
    int msgmap;
    int msgmax;
    int msgmnb;
    int msgmni;
    int msgssz;
    int msgtql;
    unsigned short int msgseg;
  };

#endif /* __USE_MISC */
