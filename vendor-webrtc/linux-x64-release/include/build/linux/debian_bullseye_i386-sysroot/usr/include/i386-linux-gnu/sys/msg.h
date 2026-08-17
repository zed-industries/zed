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
#define _SYS_MSG_H

#include <features.h>

#define __need_size_t
#include <stddef.h>

/* Get common definition of System V style IPC.  */
#include <sys/ipc.h>

/* Get system dependent definition of `struct msqid_ds' and more.  */
#include <bits/msq.h>

/* Define types required by the standard.  */
#include <bits/types/time_t.h>

#ifndef __pid_t_defined
typedef __pid_t pid_t;
# define __pid_t_defined
#endif

#ifndef __ssize_t_defined
typedef __ssize_t ssize_t;
# define __ssize_t_defined
#endif

/* The following System V style IPC functions implement a message queue
   system.  The definition is found in XPG2.  */

#ifdef __USE_GNU
/* Template for struct to be used as argument for `msgsnd' and `msgrcv'.  */
struct msgbuf
  {
    __syscall_slong_t mtype;	/* type of received/sent message */
    char mtext[1];		/* text of the message */
  };
#endif


__BEGIN_DECLS

/* Message queue control operation.  */
extern int msgctl (int __msqid, int __cmd, struct msqid_ds *__buf) __THROW;

/* Get messages queue.  */
extern int msgget (key_t __key, int __msgflg) __THROW;

/* Receive message from message queue.

   This function is a cancellation point and therefore not marked with
   __THROW.  */
extern ssize_t msgrcv (int __msqid, void *__msgp, size_t __msgsz,
		       long int __msgtyp, int __msgflg);

/* Send message to message queue.

   This function is a cancellation point and therefore not marked with
   __THROW.  */
extern int msgsnd (int __msqid, const void *__msgp, size_t __msgsz,
		   int __msgflg);

__END_DECLS

#endif /* sys/msg.h */
